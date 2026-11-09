use axum::{
    body::Bytes,
    extract::{FromRequest, Request},
    http::{
        StatusCode,
        header::{CACHE_CONTROL, CONTENT_TYPE},
    },
    response::{IntoResponse, Response},
};
use contracts::{ErrorResponse, SCHEMA_VERSION};
use serde::{Serialize, de::DeserializeOwned};

use crate::codec::{decode, encode};
use crate::config::defaults::CBOR_NESTING;
use crate::error::Error;

pub struct Cbor<T>(pub T);

impl<S: Send + Sync, T: DeserializeOwned> FromRequest<S> for Cbor<T> {
    type Rejection = Error;
    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let media = request
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|s| s.to_str().ok())
            .and_then(|s| s.split(';').next());
        if !media.is_some_and(|s| s.trim().eq_ignore_ascii_case("application/cbor")) {
            return Err(Error::UnsupportedMediaType);
        }
        let bytes = Bytes::from_request(request, state).await.map_err(|r| {
            if r.status() == StatusCode::PAYLOAD_TOO_LARGE {
                Error::PayloadTooLarge
            } else {
                Error::Invalid("invalid request body")
            }
        })?;
        Ok(Self(
            decode(&bytes, CBOR_NESTING).map_err(|_| Error::Invalid("invalid CBOR document"))?,
        ))
    }
}

impl<T: Serialize> IntoResponse for Cbor<T> {
    fn into_response(self) -> Response {
        match encode(&self.0) {
            Ok(bytes) => (
                [
                    (CONTENT_TYPE, "application/cbor"),
                    (CACHE_CONTROL, "no-store"),
                ],
                bytes,
            )
                .into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            Self::Invalid(_) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "invalid_request",
                "The request is invalid.",
            ),
            Self::UnsupportedMediaType => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "unsupported_media_type",
                "Use application/cbor.",
            ),
            Self::PayloadTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "payload_too_large",
                "The request body is too large.",
            ),
            Self::MethodNotAllowed => (
                StatusCode::METHOD_NOT_ALLOWED,
                "method_not_allowed",
                "The method is not supported.",
            ),
            Self::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "invalid_credential",
                "The credential is not accepted.",
            ),
            Self::Forbidden => (
                StatusCode::FORBIDDEN,
                "forbidden",
                "The operation is not permitted.",
            ),
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                "not_found",
                "The resource is not available.",
            ),
            Self::Conflict(_) => (
                StatusCode::CONFLICT,
                "conflict",
                "The operation conflicts with current state.",
            ),
            Self::NotReady => (
                StatusCode::SERVICE_UNAVAILABLE,
                "not_ready",
                "Authorization state is not ready.",
            ),
            _ => (
                StatusCode::SERVICE_UNAVAILABLE,
                "dependency_unavailable",
                "An authorization dependency is unavailable.",
            ),
        };
        (
            status,
            Cbor(ErrorResponse {
                schema_version: SCHEMA_VERSION,
                code: code.into(),
                message: message.into(),
            }),
        )
            .into_response()
    }
}
