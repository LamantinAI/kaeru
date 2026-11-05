//! Public HTTP error schema.

use utoipa::ToSchema;

/// The body of every error response — 400, 401, 404 and 500 alike.
///
/// A 500 never carries internal detail: substrate failures are logged and
/// flattened to `"internal error"`.
#[derive(ToSchema)]
#[allow(dead_code)] // describes a body the handlers build with `json!`
pub struct ErrorBody {
    /// Human-readable reason. For a 400 it says what to change.
    pub error: String,
}
