//! HTTP caller identity installed by the authentication boundary.

use axum::http::StatusCode;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;

/// The authenticated caller.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Principal {
    /// Anonymous access allowed by the installation authentication policy.
    Local,
    StaticBearer,
    Web {
        username: String,
    },
}

impl<S: Send + Sync> FromRequestParts<S> for Principal {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Principal>()
            .cloned()
            .ok_or((StatusCode::UNAUTHORIZED, "Authenticated principal required"))
    }
}
