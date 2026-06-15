//! Liveness probe. Unauthenticated, no substrate access — just confirms the
//! service is up and reports the build version.

use axum::Json;
use axum::Router;
use axum::routing::get;
use serde_json::Value;
use serde_json::json;

use crate::api::state::AppState;

pub fn health_router() -> Router<AppState> {
    Router::new().route("/", get(health))
}

async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "kaeru-cloud",
        "core_version": kaeru_core::version(),
    }))
}
