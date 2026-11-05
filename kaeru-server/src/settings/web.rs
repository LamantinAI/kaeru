//! Browser-account configuration and limits; no active default credentials.

use serde::{Deserialize, Serialize};

pub const SESSION_COOKIE: &str = "kaeru_session";
pub const CSRF_HEADER: &str = "x-kaeru-csrf";
pub const SESSION_ISSUER: &str = "kaeru-mcp";
pub const SESSION_AUDIENCE: &str = "kaeru-web";
pub const DEFAULT_TTL_SECONDS: u64 = 3600;
pub const MIN_TTL_SECONDS: u64 = 300;
pub const MAX_TTL_SECONDS: u64 = 86400;
pub const MAX_PASSWORD_BYTES: usize = 1024;
pub const MAX_LOGIN_BYTES: usize = 128;
pub const LOGIN_ATTEMPTS: usize = 5;
pub const LOGIN_WINDOW_SECONDS: u64 = 60;
pub const PASSWORD_WORKERS: usize = 2;
pub const MAX_REQUEST_BYTES: usize = 8192;

#[derive(Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct WebConfig {
    pub enabled: bool,
    pub mcp_token: String,
    pub username: String,
    pub password_hash: String,
    pub jwt_secret: String,
    #[serde(default)]
    pub public_origin: String,
    pub ttl_seconds: u64,
    pub issuer: String,
    pub audience: String,
    pub revision: u64,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mcp_token: String::new(),
            username: String::new(),
            password_hash: String::new(),
            jwt_secret: String::new(),
            public_origin: String::new(),
            ttl_seconds: DEFAULT_TTL_SECONDS,
            issuer: SESSION_ISSUER.into(),
            audience: SESSION_AUDIENCE.into(),
            revision: 1,
        }
    }
}

pub struct AccessMode {
    pub enabled: bool,
    pub mcp_token: String,
}

pub fn access_mode(config: Option<&WebConfig>, token: &str) -> AccessMode {
    let enabled = config.map_or(!token.trim().is_empty(), |cfg| cfg.enabled);
    let token = if !enabled {
        ""
    } else if !token.trim().is_empty() {
        token.trim()
    } else {
        config.map_or("", |cfg| cfg.mcp_token.as_str())
    };
    AccessMode {
        enabled,
        mcp_token: token.into(),
    }
}

impl WebConfig {
    pub fn validate(&self) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }
        if self.username.trim().is_empty() || self.username.len() > MAX_LOGIN_BYTES {
            return Err("Username must be between 1 and 128 bytes".into());
        }
        if !(MIN_TTL_SECONDS..=MAX_TTL_SECONDS).contains(&self.ttl_seconds) {
            return Err("Session lifetime must be between 300 and 86400 seconds".into());
        }
        if self.jwt_secret.len() < 64 || !self.jwt_secret.bytes().all(|c| c.is_ascii_hexdigit()) {
            return Err(
                "The signing key must contain at least 32 random bytes encoded as hex".into(),
            );
        }
        let hash =
            argon2::PasswordHash::new(&self.password_hash).map_err(|_| "Invalid password hash")?;
        if hash.algorithm.as_str() != "argon2id" || hash.salt.is_none() || hash.hash.is_none() {
            return Err("An Argon2id password hash with salt is required".into());
        }
        if self.issuer.is_empty() || self.audience.is_empty() {
            return Err("JWT issuer and audience are required".into());
        }
        if !self.public_origin.is_empty() {
            let url =
                reqwest::Url::parse(&self.public_origin).map_err(|_| "Invalid public origin")?;
            if !matches!(url.scheme(), "http" | "https")
                || url.origin().ascii_serialization() != self.public_origin
            {
                return Err("Use an exact HTTP or HTTPS origin, without a path".into());
            }
        }
        Ok(())
    }

    pub fn secure_cookie(&self) -> bool {
        self.public_origin.starts_with("https://")
    }
}
