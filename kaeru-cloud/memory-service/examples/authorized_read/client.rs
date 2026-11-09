//! A single reused HTTP/2 connection; every page checks current authority.
use super::config::{AUTHORIZE_TIMEOUT, MAX_DECISION_BYTES};
use contracts::{
    AccessCheck, AccessMode, Action, AuthorizeRequest, AuthorizeResponse, Resource, SCHEMA_VERSION,
    SecretToken,
};
use reqwest::{Client, StatusCode, Version};
use sha2::{Digest, Sha256};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use uuid::Uuid;

#[derive(Clone)]
pub struct Authorizer {
    client: Client,
    url: String,
    memory_key: SecretToken,
    pub calls: Arc<AtomicUsize>,
    pub http2_responses: Arc<AtomicUsize>,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Denial {
    Unauthorized,
    Forbidden,
    Unavailable,
}
impl Authorizer {
    pub fn new(base: &str, memory_key: SecretToken) -> Result<Self, reqwest::Error> {
        Ok(Self {
            client: Client::builder()
                .http2_prior_knowledge()
                .timeout(AUTHORIZE_TIMEOUT)
                .redirect(reqwest::redirect::Policy::none())
                .build()?,
            url: format!("{}/v1/authorize", base.trim_end_matches('/')),
            memory_key,
            calls: Arc::default(),
            http2_responses: Arc::default(),
        })
    }
    pub async fn authorize(
        &self,
        credential: &str,
        initiative: Uuid,
        mapping: Uuid,
    ) -> Result<String, Denial> {
        let request = AuthorizeRequest {
            schema_version: SCHEMA_VERSION,
            request_id: Uuid::now_v7(),
            credential: SecretToken::new(credential.into()),
            checks: vec![AccessCheck {
                action: Action::Read,
                resource: Resource::Initiative(initiative),
            }],
            mode: AccessMode::Normal,
        };
        let mut body = Vec::new();
        ciborium::into_writer(&request, &mut body).map_err(|_| Denial::Unavailable)?;
        self.calls.fetch_add(1, Ordering::Relaxed);
        let mut response = self
            .client
            .post(&self.url)
            .bearer_auth(self.memory_key.expose())
            .header("content-type", "application/cbor")
            .body(body)
            .send()
            .await
            .map_err(|_| Denial::Unavailable)?;
        if response.version() != Version::HTTP_2 {
            return Err(Denial::Unavailable);
        }
        self.http2_responses.fetch_add(1, Ordering::Relaxed);
        let status = response.status();
        if response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            != Some("application/cbor")
            || response
                .content_length()
                .is_some_and(|n| n > MAX_DECISION_BYTES as u64)
        {
            return Err(Denial::Unavailable);
        }
        let mut body = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|_| Denial::Unavailable)? {
            if chunk.len() > MAX_DECISION_BYTES.saturating_sub(body.len()) {
                return Err(Denial::Unavailable);
            }
            body.extend_from_slice(&chunk);
        }
        if status != StatusCode::OK {
            return Err(match status {
                StatusCode::UNAUTHORIZED => Denial::Unauthorized,
                StatusCode::FORBIDDEN => Denial::Forbidden,
                _ => Denial::Unavailable,
            });
        }
        let mut cursor = std::io::Cursor::new(&body);
        let decision: AuthorizeResponse =
            ciborium::de::from_reader_with_recursion_limit(&mut cursor, 16)
                .map_err(|_| Denial::Unavailable)?;
        if cursor.position() != body.len() as u64
            || decision.schema_version != SCHEMA_VERSION
            || decision.request_id != request.request_id
            || decision.checks != request.checks
            || decision.mode != request.mode
            || decision.principal_id.is_nil()
            || decision.source_generation.is_nil()
        {
            return Err(Denial::Unavailable);
        }
        if !decision.allowed {
            return Err(Denial::Forbidden);
        }
        let fingerprint = Sha256::digest(credential.as_bytes());
        let identity = serde_json::to_vec(&(
            decision.principal_id,
            fingerprint.as_slice(),
            decision.source_generation,
            decision.source_revision,
            decision.authz_revision,
            mapping,
        ))
        .map_err(|_| Denial::Unavailable)?;
        Ok(format!("authorized-read:{:x}", Sha256::digest(identity)))
    }
}
