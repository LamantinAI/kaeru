//! `kaeru-mcp` — Model Context Protocol server exposing the curator
//! API as native MCP tools.
//!
//! Drop-in for any MCP-aware agent runtime: stdio transport, no
//! network. Logs go to **stderr only** because stdout is the
//! JSON-RPC channel. Configuration mirrors `kaeru-cli` —
//! `KAERU_VAULT_PATH` selects the vault, `KAERU_*` overrides every
//! cap.

mod server;

use std::error::Error;

use rmcp::ServiceExt;
use rmcp::transport::stdio;
use tracing_subscriber::EnvFilter;

use kaeru_core::KaeruConfig;
use kaeru_core::Store;

use server::KaeruServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Logs MUST go to stderr — stdout is reserved for the MCP protocol.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("kaeru-mcp starting");

    let config = KaeruConfig::from_env()?;
    let vault_path = config.vault_path.clone();
    let store = Store::open_with_config(config)?;
    tracing::info!(?vault_path, "kaeru substrate ready");

    let server = KaeruServer::new(store);
    let service = server.serve(stdio()).await.inspect_err(|e| {
        tracing::error!(error = ?e, "service init failed");
    })?;
    service.waiting().await?;
    Ok(())
}
