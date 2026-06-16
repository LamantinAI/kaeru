//! `kaeru-cloud` binary — thin entrypoint. Resolves config, initialises
//! tracing, opens the cloud substrate, then hands off to
//! [`kaeru_cloud::run`]. All wiring lives in the library.
//!
//! Configuration:
//! - `KAERU_CLOUD_*` env vars tune the service (listen address/port, bearer
//!   token, log level) — see [`kaeru_cloud::config`].
//! - `KAERU_*` env vars (esp. `KAERU_VAULT_PATH`) tune the substrate — see
//!   `kaeru_core::config::KaeruConfig`. Point this at the *cloud* vault,
//!   distinct from any local vault.

use std::error::Error;

use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use kaeru_core::{KaeruConfig, Store};

use kaeru_cloud::{config::KaeruCloudConfig, run};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cloud_config = KaeruCloudConfig::from_env()?;

    // Prefer the standard `RUST_LOG` env (so the compose `RUST_LOG` knob and
    // per-target filters work); fall back to `KAERU_CLOUD_LOG_LEVEL`.
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&cloud_config.log_level));
    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_writer(std::io::stderr)
                .with_ansi(false)
                .compact(),
        )
        .init();

    tracing::info!(
        listen_address = %cloud_config.listen_address,
        listen_port    = cloud_config.listen_port,
        "kaeru-cloud starting"
    );

    let store_config = KaeruConfig::from_env()?;
    let vault_path = store_config.vault_path.clone();
    let store = Store::open_with_config(store_config)?;
    tracing::info!(?vault_path, "kaeru-cloud substrate ready");

    if cloud_config.api_token.trim().is_empty() {
        tracing::warn!(
            "KAERU_CLOUD_API_TOKEN is empty — bearer auth is DISABLED. \
             Anyone who can reach this port has full access to the shared \
             store. Set a token before exposing the service off-host."
        );
    } else {
        tracing::info!("bearer-token auth enabled");
    }

    run(cloud_config, store).await?;
    Ok(())
}
