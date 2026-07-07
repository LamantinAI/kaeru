//! Liveness probe. Unauthenticated, no substrate access — just confirms the
//! service is up and reports the build version.

use axum::routing::get;
use axum::{Json, Router};
use serde_json::{Value, json};

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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use kaeru_core::Store;
    use tower::util::ServiceExt;

    use crate::api::router::api_router;
    use crate::api::state::AppState;

    /// `/health` reports the running kaeru-core version — the contract a daemon
    /// reads to warn on a mcp <-> cloud version skew (issue #30).
    #[tokio::test]
    async fn health_reports_core_version() {
        let app = api_router(AppState {
            api_token: Arc::from(""),
            store: Arc::new(Store::open_in_memory().expect("open")),
        });
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let bytes = resp.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["core_version"].as_str(), Some(kaeru_core::version()));
    }
}
