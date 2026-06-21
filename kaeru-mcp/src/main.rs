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
mod auth;
mod cloud_client;
mod params;
mod server;
mod settings;
mod sse;
mod tools;
mod utils;

use std::collections::HashMap;
use std::error::Error;
use std::future::IntoFuture;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use kaeru_core::{KaeruConfig, Store};
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};
use tokio::net::TcpListener;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use tracing::Level;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{EnvFilter, fmt};

use crate::cloud_client::{CloudClient, CloudRegistry};
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

    // Bridge(s) to the shared kaeru-cloud service. Named clouds come from the
    // clouds TOML; the legacy single `cloud_url`/`cloud_token` pair, if set,
    // is folded in as an extra named cloud. Empty registry → the cloud tools
    // (share / pull / cloud_recall) report the cloud is not configured.
    let mut clients: HashMap<String, CloudClient> = mcp_config
        .clouds
        .iter()
        .filter(|(_, ep)| !ep.url.trim().is_empty())
        .map(|(name, ep)| {
            (
                name.clone(),
                CloudClient::new(ep.url.clone(), ep.token.clone()),
            )
        })
        .collect();

    if !mcp_config.cloud_url.trim().is_empty() {
        // Fold the legacy single-cloud config in under the default name (or
        // "default" when none is set), without clobbering a same-named TOML
        // entry.
        let legacy_name = if mcp_config.default_cloud.trim().is_empty() {
            "default".to_string()
        } else {
            mcp_config.default_cloud.clone()
        };
        clients.entry(legacy_name).or_insert_with(|| {
            CloudClient::new(mcp_config.cloud_url.clone(), mcp_config.cloud_token.clone())
        });
    }

    let default_name =
        (!mcp_config.default_cloud.trim().is_empty()).then(|| mcp_config.default_cloud.clone());
    let clouds = CloudRegistry::new(clients, default_name);

    if clouds.is_empty() {
        tracing::info!("cloud sharing disabled (no clouds configured)");
    } else {
        tracing::info!(
            clouds = ?clouds.names(),
            default = ?clouds.default_name(),
            "cloud sharing enabled"
        );
    }

    let server = KaeruServer::new(store, clouds);

    let cancel = CancellationToken::new();

    let mut session_manager = LocalSessionManager::default();
    // rmcp defaults to a 5-minute idle timeout that reaps Claude Code MCP
    // sessions during normal interactive pauses, surfacing as
    // `kaeru · ✘ failed` until auto-reconnect. Default is `0` = disabled
    // (sessions live as long as the underlying connection); set
    // `KAERU_MCP_KEEP_ALIVE_SECS` to a non-zero value for proxy-style
    // deployments where hung clients should free server-side state.
    session_manager.session_config.keep_alive = match mcp_config.keep_alive_secs {
        0 => None,
        secs => Some(Duration::from_secs(secs)),
    };

    let sse_router = sse::router(
        server.clone(),
        cancel.child_token(),
        &mcp_config.sse_path,
        &mcp_config.messages_path,
    );
    // rmcp's Streamable HTTP transport rejects any request whose `Host`
    // header isn't in `allowed_hosts` (default: loopback only) as a
    // DNS-rebinding guard — answering `403 Forbidden: Host header is not
    // allowed`, which clients like Claude Code mislabel as "Needs
    // authentication". When exposed on a routable address the operator
    // must whitelist the host(s) clients use; we keep the loopback
    // defaults so localhost sessions keep working regardless.
    let default_config = StreamableHttpServerConfig::default();
    let mut allowed_hosts = default_config.allowed_hosts.clone();
    allowed_hosts.extend(
        mcp_config
            .allowed_hosts
            .split(',')
            .map(str::trim)
            .filter(|h| !h.is_empty())
            .map(str::to_string),
    );
    tracing::info!(?allowed_hosts, "host allow-list for inbound MCP requests");

    let service = StreamableHttpService::new(
        // Each MCP session reuses the same KaeruServer (and therefore
        // the same Arc<Store> / RocksDB lock); cloning the server is
        // cheap and shares state across sessions.
        move || Ok(server.clone()),
        Arc::new(session_manager),
        default_config
            .with_allowed_hosts(allowed_hosts)
            .with_cancellation_token(cancel.child_token()),
    );

    let router = axum::Router::new()
        .nest_service(&mcp_config.mount_path, service)
        .merge(sse_router);

    // Bearer-token gate, layered over the whole router so it covers the
    // streamable HTTP and legacy SSE transports alike. Off when no token
    // is configured — fine for the loopback default, loudly flagged when
    // the daemon is reachable off-host without one.
    let router = match mcp_config.auth_token.trim() {
        "" => {
            if mcp_config.listen_address.is_loopback() {
                tracing::info!(
                    "bearer-token auth disabled (no KAERU_MCP_AUTH_TOKEN); loopback bind"
                );
            } else {
                tracing::warn!(
                    listen_address = %mcp_config.listen_address,
                    "kaeru-mcp is bound to a non-loopback address with NO auth token — \
                     anyone who can reach this port has full curator access. Set \
                     KAERU_MCP_AUTH_TOKEN to require `Authorization: Bearer <token>`."
                );
            }
            router
        }
        token => {
            tracing::info!("bearer-token auth enabled for all inbound MCP requests");
            let expected: Arc<str> = Arc::from(token);
            router.layer(axum::middleware::from_fn_with_state(
                expected,
                auth::require_bearer,
            ))
        }
    };

    let address = format!("{}:{}", mcp_config.listen_address, mcp_config.listen_port);
    let listener = TcpListener::bind(&address).await?;

    tracing::info!(
        streamable_http = %format!("http://{address}{}", mcp_config.mount_path),
        sse             = %format!("http://{address}{}", mcp_config.sse_path),
        messages        = %format!("http://{address}{}", mcp_config.messages_path),
        "kaeru-mcp listening — point MCP clients here"
    );

    // Serve with graceful shutdown, but bound the drain: long-lived MCP
    // streams (the streamable-HTTP GET channel, legacy SSE) don't always
    // close the instant their sessions are cancelled, so an unbounded
    // graceful shutdown can block until systemd's `TimeoutStopSec` (90s by
    // default) SIGKILLs the daemon. Once shutdown is requested we give
    // in-flight work a short grace window, then force the process to exit.
    let graceful = axum::serve(listener, router)
        .with_graceful_shutdown(shutdown(cancel.clone()))
        .into_future();
    tokio::pin!(graceful);
    tokio::select! {
        res = &mut graceful => {
            res?;
            tracing::info!("kaeru-mcp stopped");
        }
        () = grace_deadline(cancel, SHUTDOWN_GRACE) => {
            tracing::warn!(
                grace_secs = SHUTDOWN_GRACE.as_secs(),
                "shutdown grace window elapsed with connections still open; forcing exit"
            );
        }
    }
    Ok(())
}

/// How long graceful shutdown is allowed to drain in-flight connections
/// after a stop is requested before the daemon forces exit. Kept well under
/// systemd's default `TimeoutStopSec` (90s) so a clean stop never escalates
/// to SIGKILL.
const SHUTDOWN_GRACE: Duration = Duration::from_secs(3);

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

/// Resolves once shutdown has been requested (`cancel` fired) **and** the
/// grace window has elapsed — the backstop that bounds how long the daemon
/// waits for stubborn long-lived connections to drain.
async fn grace_deadline(cancel: CancellationToken, grace: Duration) {
    cancel.cancelled().await;
    tokio::time::sleep(grace).await;
}
