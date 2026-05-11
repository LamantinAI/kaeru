//! `kaeru-mcp` — Model Context Protocol server, exposed as a long-lived
//! HTTP service. **One daemon owns the substrate**; agent sessions
//! (Claude Code, Cursor, etc.) connect concurrently as MCP clients to
//! `http://<listen_address>:<listen_port><mount_path>`.
//!
//! Why service-mode rather than stdio: kaeru's substrate is a single-
//! writer RocksDB (Cozo). With stdio transport every agent session
//! would spawn its own kaeru-mcp subprocess, the second one to start
//! losing the LOCK race and silently failing. The service model puts
//! ownership of the vault in one place and lets every session connect.
//!
//! Configuration:
//! - `KAERU_MCP_*` env vars tune the daemon itself (see `config.rs`).
//! - `KAERU_*` env vars tune the curator-API caps and vault path
//!   (see `kaeru-core::config::KaeruConfig`).

// `settings` rather than `config` — avoids a path-resolution clash
// with the external `config` crate that this module imports from.
mod params;
mod server;
mod settings;
mod tools;
mod utils;

use std::error::Error;
use std::str::FromStr;

use rmcp::transport::streamable_http_server::StreamableHttpServerConfig;
use rmcp::transport::streamable_http_server::StreamableHttpService;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use tokio::net::TcpListener;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use tracing::Level;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

use kaeru_core::KaeruConfig;
use kaeru_core::Store;

use crate::server::KaeruServer;
use crate::settings::KaeruMcpConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mcp_config = KaeruMcpConfig::new()?;

    let level = Level::from_str(&mcp_config.log_level)?;
    tracing_subscriber::registry()
        .with(EnvFilter::from(level.as_str()))
        .with(
            fmt::layer()
                .with_writer(std::io::stderr)
                .with_ansi(false)
                .compact(),
        )
        .init();

    tracing::info!(
        listen_address = %mcp_config.listen_address,
        listen_port    = mcp_config.listen_port,
        mount_path     = %mcp_config.mount_path,
        "kaeru-mcp starting"
    );

    let store_config = KaeruConfig::from_env()?;
    let vault_path = store_config.vault_path.clone();
    let store = Store::open_with_config(store_config)?;
    tracing::info!(?vault_path, "kaeru substrate ready");

    let server = KaeruServer::new(store);

    let cancel = CancellationToken::new();
    let mut session_manager = LocalSessionManager::default();
    // rmcp defaults to a 5-minute idle timeout that reaps Claude Code MCP
    // sessions during normal interactive pauses. The reaped sessions appear
    // as `kaeru · ✘ failed` in the client UI before auto-reconnect. Disable
    // the timeout — sessions live as long as the underlying connection.
    session_manager.session_config.keep_alive = None;
    let service = StreamableHttpService::new(
        // Each MCP session reuses the same KaeruServer (and therefore
        // the same Arc<Store> / RocksDB lock); cloning the server is
        // cheap and shares state across sessions.
        move || Ok(server.clone()),
        std::sync::Arc::new(session_manager),
        StreamableHttpServerConfig::default()
            .with_cancellation_token(cancel.child_token()),
    );

    let router = axum::Router::new().nest_service(&mcp_config.mount_path, service);
    let address = format!("{}:{}", mcp_config.listen_address, mcp_config.listen_port);
    let listener = TcpListener::bind(&address).await?;

    tracing::info!(
        url = %format!("http://{address}{}", mcp_config.mount_path),
        "kaeru-mcp listening — point MCP clients here"
    );

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown(cancel))
        .await?;

    tracing::info!("kaeru-mcp stopped");
    Ok(())
}

/// Waits for either Ctrl-C or SIGTERM, then cancels the rmcp service
/// token so in-flight sessions wind down cleanly. SIGTERM coverage is
/// what makes the daemon usable under systemd / launchd.
async fn shutdown(cancel: CancellationToken) {
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
    cancel.cancel();
}
