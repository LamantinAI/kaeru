//! Fixed-algorithm JWT verification and bounded password work.

use std::sync::Arc;
use std::time::{Duration, Instant};

use argon2::password_hash::{
    SaltString,
    rand_core::{OsRng, RngCore},
};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use tokio::sync::Semaphore;

use super::store::AccountStore;
use crate::models::auth::SessionClaims;
use crate::settings::web::{self, WebConfig};

pub struct WebSession {
    pub account: Option<Arc<AccountStore>>,
    workers: Arc<Semaphore>,
    attempts: std::sync::Mutex<Vec<Instant>>,
}

pub fn random_secret() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn hash_password(password: &str) -> Result<String, String> {
    if password.is_empty() || password.len() > web::MAX_PASSWORD_BYTES {
        return Err("Password must contain between 1 and 1024 bytes".into());
    }
    Argon2::default()
        .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
        .map(|hash| hash.to_string())
        .map_err(|_| "Could not hash password".into())
}

impl WebSession {
    pub fn new(account: Option<AccountStore>) -> Arc<Self> {
        Arc::new(Self {
            account: account.map(Arc::new),
            workers: Arc::new(Semaphore::new(web::PASSWORD_WORKERS)),
            attempts: std::sync::Mutex::new(Vec::new()),
        })
    }

    pub fn admit_login(&self) -> bool {
        let mut attempts = self.attempts.lock().unwrap_or_else(|p| p.into_inner());
        attempts.retain(|time| time.elapsed() < Duration::from_secs(web::LOGIN_WINDOW_SECONDS));
        if attempts.len() >= web::LOGIN_ATTEMPTS {
            return false;
        }
        attempts.push(Instant::now());
        true
    }

    pub async fn check_password(&self, password: String, expected_revision: u64) -> bool {
        if password.len() > web::MAX_PASSWORD_BYTES {
            return false;
        }
        let Some(account) = &self.account else {
            return false;
        };
        let snapshot = account.snapshot();
        if snapshot.revision != expected_revision {
            return false;
        }
        let hash = snapshot.password_hash;
        let Ok(permit) = self.workers.clone().try_acquire_owned() else {
            return false;
        };
        tokio::task::spawn_blocking(move || {
            let _permit = permit;
            PasswordHash::new(&hash).is_ok_and(|parsed| {
                Argon2::default()
                    .verify_password(password.as_bytes(), &parsed)
                    .is_ok()
            })
        })
        .await
        .unwrap_or(false)
    }

    pub async fn new_hash(&self, password: String) -> Result<String, String> {
        let permit = self
            .workers
            .clone()
            .try_acquire_owned()
            .map_err(|_| "Password processing busy; retry")?;
        tokio::task::spawn_blocking(move || {
            let _permit = permit;
            hash_password(&password)
        })
        .await
        .map_err(|_| "Password processing failed")?
    }

    pub fn issue(&self, expected_revision: u64) -> Result<(String, SessionClaims, bool), String> {
        let cfg = self
            .account
            .as_ref()
            .ok_or("Account not configured")?
            .snapshot();
        if cfg.revision != expected_revision {
            return Err("Account changed during sign-in".into());
        }
        let now = jsonwebtoken::get_current_timestamp();
        let claims = SessionClaims {
            sub: cfg.username.clone(),
            exp: now + cfg.ttl_seconds,
            iat: now,
            nbf: now,
            iss: cfg.issuer.clone(),
            aud: cfg.audience.clone(),
            revision: cfg.revision,
            csrf: random_secret(),
        };
        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(cfg.jwt_secret.as_bytes()),
        )
        .map_err(|_| "Could not issue session")?;
        Ok((token, claims, cfg.secure_cookie()))
    }

    pub fn verify(&self, token: &str) -> Option<SessionClaims> {
        let cfg = self.account.as_ref()?.snapshot();
        Self::verify_with(&cfg, token)
    }

    fn verify_with(cfg: &WebConfig, token: &str) -> Option<SessionClaims> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 0;
        validation.validate_nbf = true;
        validation.set_required_spec_claims(&["exp", "sub", "iss", "aud", "nbf", "iat"]);
        validation.set_issuer(&[&cfg.issuer]);
        validation.set_audience(&[&cfg.audience]);
        let claims = decode::<SessionClaims>(
            token,
            &DecodingKey::from_secret(cfg.jwt_secret.as_bytes()),
            &validation,
        )
        .ok()?
        .claims;
        if claims.sub != cfg.username
            || claims.revision != cfg.revision
            || claims.csrf.len() != 64
            || claims.iat > jsonwebtoken::get_current_timestamp()
            || claims.exp <= claims.iat
            || claims.exp - claims.iat > web::MAX_TTL_SECONDS
        {
            return None;
        }
        Some(claims)
    }
}
