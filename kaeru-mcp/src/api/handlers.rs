use axum::Json;
use axum::extract::State;
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use serde_json::{Value, json};

use crate::api::ApiState;
use crate::api::principal::Principal;
use crate::canonical::{call_with_cancellation, options};
use kaeru_core::config::api::API_OPERATION_PREFIX;
use kaeru_core::models::ApiOptions;
use tokio_util::sync::CancellationToken;

pub async fn invoke(
    _principal: Principal,
    State(state): State<ApiState>,
    uri: Uri,
    Json(arguments): Json<Value>,
) -> Response {
    let name = uri.path().strip_prefix(API_OPERATION_PREFIX).unwrap_or("");
    let options = ApiOptions {
        allowed_initiatives: Some(state.cfg.allow.clone()),
        denied_initiatives: state.cfg.deny.clone(),
        include_local: state.cfg.include_local,
        caller: "http-local-operator".into(),
        ..options()
    };
    let cancel = CancellationToken::new();
    let _cancel_on_drop = cancel.clone().drop_guard();
    let outcome =
        call_with_cancellation(state.store, &state.clouds, name, arguments, options, cancel).await;
    let (status, mut value) = match outcome {
        Ok(outcome) => {
            let value = outcome
                .structured_content
                .unwrap_or_else(|| json!({"content":outcome.content,"is_error":outcome.is_error}));
            let status = if outcome.is_error == Some(true) {
                match value.pointer("/issues/0/code").and_then(Value::as_str) {
                    Some("NOT_FOUND" | "NOT_FOUND_AT_TIME") => StatusCode::NOT_FOUND,
                    Some("CONFLICT") => StatusCode::CONFLICT,
                    _ => StatusCode::BAD_REQUEST,
                }
            } else {
                StatusCode::OK
            };
            (status, value)
        }
        Err(error) => (
            StatusCode::BAD_REQUEST,
            json!({"error":error.message,"details":error.data}),
        ),
    };
    state.cfg.redact_value(&mut value);
    let mut response = (status, Json(value)).into_response();
    state.cfg.finish(&mut response);
    response
}

#[cfg(test)]
mod tests {
    use crate::api::{ApiConfig, router};
    use axum::{body::Body, http::Request};
    use kaeru_core::{Store, api};
    use serde_json::json;
    use std::sync::Arc;
    use tower::ServiceExt;

    #[tokio::test]
    async fn cloud_full_reads_preserve_operator_gate_redaction_and_trace() {
        use crate::api::router_with_clouds;
        use crate::cloud_client::{CloudClient, CloudRegistry};
        use axum::{Json, Router, routing::get};
        use std::collections::HashMap;
        use std::sync::atomic::{AtomicUsize, Ordering};

        let requests = Arc::new(AtomicUsize::new(0));
        let seen = requests.clone();
        let secret = format!("sk-{}", "a".repeat(40));
        let remote_secret = secret.clone();
        let remote = Router::new()
            .route(
                "/api/v1/nodes/node-a",
                get(move || {
                    seen.fetch_add(1, Ordering::SeqCst);
                    let secret = remote_secret.clone();
                    async move {
                        Json(json!({"id":"node-a","name":"full-note","body":secret,
                        "node_type":"reference","tier":"archival","layer":"warm","tags":[]}))
                    }
                }),
            )
            .route(
                "/api/v1/initiatives/alpha/nodes",
                get(|| async { Json(json!([{"id":"node-a"}])) }),
            )
            .route(
                "/api/v1/initiatives/beta/nodes",
                get(|| async { Json(json!([])) }),
            )
            .route(
                "/api/v1/initiatives/alpha/edges",
                get(|| async { axum::http::StatusCode::SERVICE_UNAVAILABLE }),
            );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move { axum::serve(listener, remote).await.unwrap() });
        let clouds = CloudRegistry::new(
            HashMap::from([(
                "test".into(),
                CloudClient::new("test".into(), format!("http://{address}"), String::new()),
            )]),
            None,
        );
        let directory =
            std::env::temp_dir().join(format!("kaeru-cloud-read-{}", kaeru_core::new_node_id()));
        let store = Arc::new(
            Store::open_in_memory_with(kaeru_core::KaeruConfig {
                read_trace_enabled: true,
                read_trace_dir: directory.join("trace"),
                vault_path: directory.join("vault"),
                ..kaeru_core::KaeruConfig::defaults()
            })
            .unwrap(),
        );
        for (allow, include_local, initiative, expected_status, expected_requests) in [
            (
                vec!["alpha".into()],
                true,
                "alpha",
                axum::http::StatusCode::NOT_FOUND,
                0,
            ),
            (
                vec!["*".into()],
                true,
                "beta",
                axum::http::StatusCode::OK,
                1,
            ),
            (
                vec!["*".into()],
                true,
                "alpha",
                axum::http::StatusCode::OK,
                2,
            ),
        ] {
            let app = router_with_clouds(
                store.clone(),
                ApiConfig {
                    allow,
                    include_local,
                    ..ApiConfig::default()
                },
                clouds.clone(),
            );
            let response = app
                .oneshot(
                    Request::post("/v1/cloud_recall")
                        .header("content-type", "application/json")
                        .body(Body::from(
                            json!({"initiative":initiative,"id":"node-a"}).to_string(),
                        ))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), expected_status);
            let body = axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .unwrap();
            let body = String::from_utf8(body.to_vec()).unwrap();
            assert!(!body.contains(&secret));
            if initiative == "alpha" && expected_requests == 2 {
                assert!(body.contains("[redacted]"));
            }
            assert_eq!(requests.load(Ordering::SeqCst), expected_requests);
        }
        assert!(
            kaeru_core::read_node_full(&store, &"node-a".into())
                .unwrap()
                .is_none()
        );
        let cancelled = tokio_util::sync::CancellationToken::new();
        cancelled.cancel();
        assert!(
            crate::canonical::call_with_cancellation(
                store.clone(),
                &clouds,
                "pull",
                json!({"initiative":"alpha","id":"node-a"}),
                kaeru_core::api::ApiOptions::local(),
                cancelled
            )
            .await
            .is_err()
        );
        assert_eq!(requests.load(Ordering::SeqCst), 2);
        let response = router_with_clouds(
            store.clone(),
            ApiConfig {
                allow: vec!["*".into()],
                include_local: true,
                ..ApiConfig::default()
            },
            clouds,
        )
        .oneshot(
            Request::post("/v1/pull")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"initiative":"alpha","id":"node-a"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(body["details"]["node_imported"], true);
        assert!(
            kaeru_core::read_node_full(&store, &"node-a".into())
                .unwrap()
                .is_some()
        );
        assert!(store.shutdown_read_trace(std::time::Duration::from_secs(3)));
        let events: Vec<kaeru_core::models::ReadTraceEvent> =
            std::fs::read_dir(directory.join("trace"))
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| path.extension().is_some_and(|e| e == "jsonl"))
                .flat_map(|path| {
                    std::fs::read_to_string(path)
                        .unwrap()
                        .lines()
                        .map(|line| serde_json::from_str(line).unwrap())
                        .collect::<Vec<_>>()
                })
                .collect();
        assert_eq!(
            events.len(),
            1,
            "denied membership and cancelled pulls do not emit read deliveries"
        );
        assert_eq!(events[0].operation, "cloud_recall");
        assert_eq!(events[0].node_ids, vec!["node-a"]);
        assert_eq!(events[0].initiatives, vec!["alpha"]);
        server.abort();
        std::fs::remove_dir_all(directory).unwrap();
    }

    #[tokio::test]
    async fn named_routes_enforce_scope_and_current_visibility() {
        let directory =
            std::env::temp_dir().join(format!("kaeru-http-trace-{}", kaeru_core::new_node_id()));
        let cfg = kaeru_core::KaeruConfig {
            read_trace_enabled: true,
            read_trace_dir: directory.join("trace"),
            vault_path: directory.join("vault"),
            ..kaeru_core::KaeruConfig::defaults()
        };
        let store = Arc::new(Store::open_in_memory_with(cfg).unwrap());
        let captured = api::execute(
            &store,
            "note",
            json!({"initiative":"visible","body":"Local body must stay private."}),
            &api::ApiOptions::local(),
        );
        let id = match captured.data {
            api::ResponseData::Mutation(d) => d.primary.unwrap().id,
            _ => panic!(),
        };
        let app = router(
            store.clone(),
            ApiConfig {
                allow: vec!["visible".into()],
                ..ApiConfig::default()
            },
        );
        let response = app
            .oneshot(
                Request::post("/v1/read")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        json!({"initiative":"visible","node":id}).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
        let app = router(
            store.clone(),
            ApiConfig {
                allow: vec!["visible".into()],
                include_local: true,
                ..ApiConfig::default()
            },
        );
        let response = app
            .oneshot(
                Request::post("/v1/read")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        json!({"initiative":"visible","node":id}).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert!(store.shutdown_read_trace(std::time::Duration::from_secs(3)));
        let mut events = Vec::<kaeru_core::models::ReadTraceEvent>::new();
        for entry in std::fs::read_dir(directory.join("trace")).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "jsonl") {
                for line in std::fs::read_to_string(path).unwrap().lines() {
                    events.push(serde_json::from_str(line).unwrap());
                }
            }
        }
        assert_eq!(
            events.len(),
            1,
            "denied HTTP reads are not logged; successful reads are not duplicated"
        );
        assert_eq!(events[0].node_ids, vec![id]);
        std::fs::remove_dir_all(directory).unwrap();
    }
    #[tokio::test]
    async fn removed_http_paths_have_no_aliases() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        for path in ["/v1/at", "/v1/board", "/v1/chain", "/graph.json"] {
            let response = router(store.clone(), ApiConfig::default())
                .oneshot(Request::get(path).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
        }
    }
}
