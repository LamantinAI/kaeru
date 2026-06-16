//! `kaeru-cloud` — the shared cloud tier of the local/cloud memory split.
//!
//! An Axum REST service wrapping `kaeru-core`: the **same** substrate
//! mechanics as a local vault (Cozo + RocksDB), but reachable over HTTP and
//! shared between users, gated by a bearer token. The local `kaeru-mcp`
//! daemon is the agent's only surface and proxies into this service for the
//! nodes an initiative has chosen to share.
//!
//! Topology and rationale: see the design vault page `local_cloud_split`.
//! Multi-tenant per-user isolation is deferred — for now the token is the
//! access gate to one shared team space, scoped by initiative through the
//! junction relation `kaeru-core` already maintains.

pub mod api;
pub mod config;
pub mod errors;

use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::signal;

use kaeru_core::Store;

use crate::api::router::api_router;
use crate::api::state::AppState;
use crate::config::KaeruCloudConfig;
use crate::errors::StartError;

/// Builds the application state, binds the listener, and serves until a
/// shutdown signal arrives. The `store` is the cloud substrate, opened by
/// the caller (usually `Store::open_with_config(KaeruConfig::from_env())`).
pub async fn run(cloud_config: KaeruCloudConfig, store: Store) -> Result<(), StartError> {
    // A token is mandatory whenever the service is reachable off-host: an
    // empty token disables auth, so an unauthenticated routable bind is an
    // open store. Loopback (dev) is the only place an empty token is allowed.
    if cloud_config.api_token.trim().is_empty() && !cloud_config.listen_address.is_loopback() {
        return Err(StartError::InsecureBind(cloud_config.listen_address));
    }

    let state = AppState {
        api_token: Arc::from(cloud_config.api_token.as_str()),
        store: Arc::new(store),
    };

    let router = api_router(state);

    let address = format!("{}:{}", cloud_config.listen_address, cloud_config.listen_port);
    let listener = TcpListener::bind(&address).await?;
    tracing::info!(%address, "kaeru-cloud listening — point the local daemon's cloud tools here");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown())
        .await?;

    tracing::info!("kaeru-cloud stopped");
    Ok(())
}

/// Resolves on Ctrl-C or SIGTERM so the service winds down cleanly under
/// systemd / launchd.
async fn shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to register Ctrl-C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => tracing::info!("Ctrl-C received"),
        _ = terminate => tracing::info!("SIGTERM received"),
    }
}
