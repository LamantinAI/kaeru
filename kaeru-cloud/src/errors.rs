//! Error types. `ApiError` is the domain/request error returned by
//! handlers; its `IntoResponse` lives in `api/errors.rs` (status mapping is
//! an HTTP concern, kept next to the other HTTP glue). `StartError` covers
//! process startup. No `anyhow` — explicit variants, `#[from]` for upstream
//! errors.

use thiserror::Error;

/// A request-handling error. Maps to an HTTP status in `api::errors`.
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("not found")]
    NotFound,

    #[error("invalid request: {0}")]
    BadRequest(String),

    /// A failure from the underlying `kaeru-core` substrate / primitives.
    /// Surfaced as 500; details are logged, never sent to the client.
    #[error(transparent)]
    Core(#[from] kaeru_core::Error),
}

/// A process-startup error. Returned by `run` and bubbled out of `main`.
#[derive(Error, Debug)]
pub enum StartError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Config(#[from] config::ConfigError),

    #[error(transparent)]
    Core(#[from] kaeru_core::Error),
}
