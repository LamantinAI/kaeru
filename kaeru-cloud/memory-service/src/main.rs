//! TiKV-default memory service; local process management belongs to kaeru-local.

use std::{error::Error, sync::Arc};

use clap::Parser;
use kaeru_core::Store;
use kaeru_memory_service::{application, settings::MemoryServiceConfig};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[derive(Parser)]
#[command(version, about = "Kaeru memory service: MCP, HTTP and TiKV storage")]
struct Options {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Options::parse();
    let config = MemoryServiceConfig::from_env()?;
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.transport.log_level));
    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_writer(std::io::stderr)
                .with_ansi(false)
                .compact(),
        )
        .init();
    let options = config.runtime_options()?;
    let storage = MemoryServiceConfig::storage_from_env()?;
    let store = Arc::new(Store::open_with_config(storage)?);
    tracing::info!(
        backend = store.config().storage.kind(),
        "memory-service substrate ready"
    );
    application(&config, store, options).serve().await?;
    Ok(())
}
