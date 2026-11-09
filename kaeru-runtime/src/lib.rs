//! Shared MCP, HTTP and instance UI runtime. Hosts resolve policy explicitly.

mod api;
pub mod auth;
mod canonical;
mod cloud;
mod cloud_client;
mod diagnostics;
mod exports;
mod hygiene;
mod models;
#[cfg(test)]
mod parity;
mod runtime;
mod server;
mod session;
pub mod settings;
mod sse;
mod ui;
mod update;
mod utils;

pub use api::{ApiConfig, authorized_read};
pub use models::AuthorizedRead;
pub use runtime::{ServerRuntime, shutdown as shutdown_signal};
pub use settings::RuntimeOptions;

/// Shared transport admission and authentication primitives for host integrations.
pub mod transport {
    pub use crate::auth::require_bearer;
    pub use crate::session::{BoundedSessionManager, admit};
}

#[cfg(test)]
mod read_trace_tests;
