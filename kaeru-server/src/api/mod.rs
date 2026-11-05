//! Named HTTP routes over the same memory operations as MCP.

#[cfg(test)]
mod cancellation_tests;
pub mod egress;
mod handlers;
pub mod principal;

use std::sync::Arc;

use axum::Router;
use axum::routing::post;
pub use egress::ApiConfig;
use kaeru_core::Store;
use kaeru_core::api::tool_definitions;
use kaeru_core::config::api::API_OPERATION_PREFIX;

use self::handlers::invoke;
use crate::cloud_client::CloudRegistry;

#[derive(Clone)]
pub struct ApiState {
    pub store: Arc<Store>,
    pub cfg: ApiConfig,
    pub clouds: CloudRegistry,
}

#[cfg(test)]
pub fn router(store: Arc<Store>, cfg: ApiConfig) -> Router {
    router_with_clouds(store, cfg, CloudRegistry::default())
        .layer(axum::Extension(principal::Principal::Local))
}

pub fn router_with_clouds(store: Arc<Store>, cfg: ApiConfig, clouds: CloudRegistry) -> Router {
    let mut router = Router::new();
    for tool in tool_definitions() {
        router = router.route(
            &format!("{API_OPERATION_PREFIX}{}", tool.name),
            post(invoke),
        );
    }
    router.with_state(ApiState { store, cfg, clouds })
}
