//! Memory-service: MCP, canonical HTTP and the legacy sharing adapter on one Store.

pub mod api;
pub mod config;
pub mod errors;
pub mod models;
pub mod settings;

use std::sync::Arc;

use kaeru_core::Store;
use kaeru_server::{RuntimeOptions, ServerRuntime};

use api::{router::api_router, state::AppState};
use errors::StartError;
use settings::MemoryServiceConfig;

/// Compose every transport around the exact same substrate handle.
pub fn application(
    config: &MemoryServiceConfig,
    store: Arc<Store>,
    options: RuntimeOptions,
) -> ServerRuntime {
    let compatibility = api_router(AppState {
        api_token: Arc::from(options.access.mcp_token.as_str()),
        store: store.clone(),
    });
    ServerRuntime::new(store, config.transport.clone(), options).merge(compatibility)
}

/// Compatibility for embedded callers supplying an already opened Store.
/// The deployed memory-service entrypoint resolves its own TiKV-default profile.
pub async fn run(config: config::KaeruCloudConfig, store: Store) -> Result<(), StartError> {
    let service: MemoryServiceConfig = serde_json::from_value(serde_json::json!({
        "listen_address":config.listen_address,"listen_port":config.listen_port,
        "api_token":config.api_token,"log_level":config.log_level
    }))
    .expect("legacy configuration maps to service fields");
    let options = service.runtime_options()?;
    application(&service, Arc::new(store), options)
        .serve()
        .await?;
    Ok(())
}

#[cfg(test)]
mod composition_tests;
