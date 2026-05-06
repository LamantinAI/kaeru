//! Crate-wide error type and result alias.
//!
//! Every primitive returns `Result<T>`. Substrate errors are wrapped through
//! `Error::Substrate` (see the `From<cozo::Error>` impl). Domain errors get
//! their own variants.

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// Substrate (CozoDB) returned an error during script execution or
    /// schema management.
    #[error("substrate error: {0}")]
    Substrate(String),

    /// Schema bootstrap failed at a particular statement.
    #[error("schema bootstrap failed: {0}")]
    SchemaBootstrap(String),

    /// Caller supplied an argument that does not match the schema or the
    /// declared invariants of the primitive.
    #[error("invalid argument: {0}")]
    Invalid(String),

    /// Looked-up entity is missing where presence was required.
    #[error("not found: {0}")]
    NotFound(String),

    /// I/O failure, typically from the embedded substrate's persistent
    /// backend or filesystem operations adjacent to it.
    #[error("io error: {0}")]
    Io(#[from] io::Error),

    /// Configuration could not be assembled from defaults + environment.
    /// Typically a malformed `KAERU_*` env var (e.g. non-numeric for a
    /// numeric field).
    #[error("config error: {0}")]
    Config(#[from] config::ConfigError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<cozo::Error> for Error {
    fn from(e: cozo::Error) -> Self {
        Error::Substrate(format!("{e:?}"))
    }
}
