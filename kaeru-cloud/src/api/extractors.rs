//! Request extractors. `Authenticated` is a zero-size proof that the
//! request carried a valid `Authorization: Bearer <token>` header; adding it
//! to a handler's arguments gates that route. It pulls the expected token
//! out of any state that can yield `Arc<str>` via `FromRef`, so it isn't
//! tied to a concrete state type.

use std::sync::Arc;

use axum::extract::FromRef;
use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;

use crate::api::errors::AuthError;

/// Zero-size proof that the request was authenticated.
pub struct Authenticated;

impl<S> FromRequestParts<S> for Authenticated
where
    Arc<str>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let expected = Arc::<str>::from_ref(state);

        // Empty configured token → auth disabled (loopback / dev). The
        // operator is warned about this at startup.
        if expected.is_empty() {
            return Ok(Authenticated);
        }

        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(AuthError::Missing)?;

        let token = header
            .strip_prefix("Bearer ")
            .or_else(|| header.strip_prefix("bearer "))
            .ok_or(AuthError::Malformed)?
            .trim();

        if constant_time_eq(token.as_bytes(), expected.as_bytes()) {
            Ok(Authenticated)
        } else {
            Err(AuthError::Invalid)
        }
    }
}

/// Constant-time byte comparison — avoids leaking how many leading bytes of
/// a guessed token matched via early-exit timing. A length mismatch returns
/// `false` immediately; that only reveals the token length, not its content.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff: u8 = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}
