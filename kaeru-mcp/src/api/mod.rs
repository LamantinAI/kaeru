//! The daemon's HTTP surface.
//!
//! kaeru-mcp has spoken HTTP since it grew a streamable transport; what it has
//! not had is an *API*. The MCP transports carry JSON-RPC, which only a client
//! that speaks MCP can use — so the visualizer was handed a side door,
//! `/graph.json`, that corresponds to no verb and grew its own config, its own
//! env vars and its own CORS rule.
//!
//! This module replaces the side door with the front one. The rule is that a
//! **route is a verb**: `/v1/export` is the `export` verb, and nothing here
//! invents a vocabulary that the curator API does not already have. The MCP
//! tools and these routes are two transports over one implementation, not two
//! implementations that have to be kept in agreement.
//!
//! Three things are fixed in the shape rather than left to each handler:
//!
//! - **`/v1/` on everything.** Free to add now, a breaking change to add later.
//! - **[`Principal`](principal::Principal) on every handler**, so a caller
//!   identity can grow a variant instead of a signature.
//! - **[`egress`] on the way out**, so redaction and scope live in one
//!   auditable place.
//!
//! The whole surface is opt-in: `main.rs` mounts it only when the operator sets
//! `KAERU_MCP_VIZ_ENABLE`, and an unconfigured allow-list exports nothing.

pub mod egress;
pub mod principal;
pub mod v1;

use std::sync::Arc;

use axum::Router;
use axum::routing::get;
use kaeru_core::Store;

pub use egress::ApiConfig;

/// Shared state for every API handler.
#[derive(Clone)]
pub struct ApiState {
    pub store: Arc<Store>,
    pub cfg: ApiConfig,
}

/// The path the visualizer used before the API existed.
///
/// Kept as an alias so a daemon that updates ahead of its visualizer keeps
/// working. It is the only route on the surface that is not named after a
/// verb, which is exactly why it is deprecated: it answers `export`, so
/// `/v1/export` is what it should always have been called.
const LEGACY_GRAPH_PATH: &str = "/graph.json";

/// Builds the API router from operator config.
pub fn router(store: Arc<Store>, cfg: ApiConfig) -> Router {
    Router::new()
        .route("/v1/at", get(v1::at::at))
        .route("/v1/board", get(v1::board::board))
        .route("/v1/chain", get(v1::chain::chain))
        .route("/v1/export", get(v1::export::export))
        .route(LEGACY_GRAPH_PATH, get(v1::export::export))
        .with_state(ApiState { store, cfg })
}
