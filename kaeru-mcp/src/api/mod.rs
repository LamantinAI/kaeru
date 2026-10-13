//! Named HTTP routes over the same memory operations as MCP.

pub mod egress;
pub mod principal;
mod v1;

use crate::cloud_client::CloudRegistry;
use axum::Router;
use axum::routing::post;
pub use egress::ApiConfig;
use kaeru_core::Store;
use kaeru_core::api::tool_definitions;
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiState {
    pub store: Arc<Store>,
    pub cfg: ApiConfig,
    pub clouds: CloudRegistry,
}

#[cfg(test)]
pub fn router(store: Arc<Store>, cfg: ApiConfig) -> Router {
    router_with_clouds(store, cfg, CloudRegistry::default())
}

pub fn router_with_clouds(store: Arc<Store>, cfg: ApiConfig, clouds: CloudRegistry) -> Router {
    let mut router = Router::new();
    for tool in tool_definitions() {
        router = router.route(&format!("/v1/{}", tool.name), post(v1::invoke));
    }
    router.with_state(ApiState { store, cfg, clouds })
}
