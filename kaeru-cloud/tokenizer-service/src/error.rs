use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid request: {0}")]
    Invalid(&'static str),
    #[error("application/cbor content type is required")]
    UnsupportedMediaType,
    #[error("request body exceeds the configured limit")]
    PayloadTooLarge,
    #[error("method is not supported")]
    MethodNotAllowed,
    #[error("credential is not accepted")]
    Unauthorized,
    #[error("operation is not permitted")]
    Forbidden,
    #[error("resource is not available")]
    NotFound,
    #[error("conflicting operation: {0}")]
    Conflict(&'static str),
    #[error("authorization state is not ready")]
    NotReady,
    #[error("authorization request deadline exceeded")]
    Timeout,
    #[error("database operation failed")]
    Database(#[from] sqlx::Error),
    #[error("database migration failed")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("stored authorization state is invalid")]
    Corrupt,
    #[error("cryptographic entropy is unavailable")]
    Entropy,
    #[error("configuration field is invalid: {0}")]
    Config(&'static str),
    #[error("serialization failed")]
    Codec,
    #[error("listener failed")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
