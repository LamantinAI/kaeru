//! HTTP status mapping. `ApiError` (defined in `crate::errors`) and the
//! extractor-local `AuthError` both turn into a uniform
//! `{"error": "..."}` JSON body. Internal substrate failures are logged and
//! flattened to a generic 500 so details never leak to the client.

use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use serde_json::json;

use crate::errors::ApiError;

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match &self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Core(e) => {
                tracing::error!("substrate error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        // Never surface internal error text to the client.
        let message = if status == StatusCode::INTERNAL_SERVER_ERROR {
            "internal error".to_string()
        } else {
            self.to_string()
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

/// Bearer-auth rejection. Lives here next to its `IntoResponse`; all
/// variants map to 401 with a short, non-leaky message.
#[derive(Debug)]
pub enum AuthError {
    /// No `Authorization` header present.
    Missing,
    /// Header present but not a `Bearer <token>`.
    Malformed,
    /// Token did not match the configured one.
    Invalid,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let message = match self {
            AuthError::Missing => "missing Authorization header",
            AuthError::Malformed => "malformed bearer token",
            AuthError::Invalid => "invalid token",
        };
        (StatusCode::UNAUTHORIZED, Json(json!({ "error": message }))).into_response()
    }
}
