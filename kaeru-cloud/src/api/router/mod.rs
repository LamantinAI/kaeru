//! Router assembly. Health lives at the root (unauthenticated); the
//! versioned API lives under [`API_PREFIX`] and its handlers gate
//! themselves with the `Authenticated` extractor. State is attached once,
//! at the top.

pub mod edges;
pub mod health;
pub mod initiatives;
pub mod nodes;

use axum::Router;

use crate::api::state::AppState;

/// Prefix for the versioned API surface.
pub const API_PREFIX: &str = "/api/v1";

/// Builds the full application router with state attached.
pub fn api_router(state: AppState) -> Router {
    let v1 = Router::new()
        .nest("/nodes", nodes::nodes_router())
        .nest("/edges", edges::edges_router())
        .nest("/initiatives", initiatives::initiatives_router());

    Router::new()
        .nest("/health", health::health_router())
        .nest(API_PREFIX, v1)
        .with_state(state)
}
