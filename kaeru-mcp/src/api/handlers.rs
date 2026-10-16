use axum::Json;
use axum::extract::State;
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use serde_json::{Value, json};

use crate::api::ApiState;
use crate::api::principal::Principal;
use crate::canonical::{call_with_options, options};
use kaeru_core::config::api::API_OPERATION_PREFIX;
use kaeru_core::models::ApiOptions;

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
    let outcome = call_with_options(state.store, &state.clouds, name, arguments, options).await;
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
        Err(error) => (StatusCode::BAD_REQUEST, json!({"error":error.message})),
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
    async fn named_routes_enforce_scope_and_current_visibility() {
        let store = Arc::new(Store::open_in_memory().unwrap());
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
            store,
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
