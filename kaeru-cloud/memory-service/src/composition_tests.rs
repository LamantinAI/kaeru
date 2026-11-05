use std::sync::Arc;

use crate::{application, settings::MemoryServiceConfig};
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use kaeru_core::{Store, read_node_full};
use serde_json::{Value, json};
use tower::ServiceExt;

async fn call(
    router: &Router,
    method: &str,
    path: &str,
    token: Option<&str>,
    value: Value,
) -> (StatusCode, Value) {
    let mut request = Request::builder()
        .method(method)
        .uri(path)
        .header("host", "localhost")
        .header("content-type", "application/json");
    if let Some(token) = token {
        request = request.header("authorization", format!("Bearer {token}"));
    }
    let response = router
        .clone()
        .oneshot(request.body(Body::from(value.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), 1_000_000).await.unwrap();
    (status, serde_json::from_slice(&body).unwrap_or(Value::Null))
}

#[tokio::test]
async fn legacy_and_canonical_routes_share_store_without_sharing_auth_bypasses() {
    let config:MemoryServiceConfig=serde_json::from_value(json!({"api_token":"fixture-token","api_allow":"alpha","api_include_local":true,"hygiene_enable":false})).unwrap();
    let store = Arc::new(Store::open_in_memory().unwrap());
    let runtime = application(&config, store.clone(), config.runtime_options().unwrap());
    assert!(Arc::ptr_eq(&store, &runtime.store()));
    let app = runtime.router();
    for (method, path) in [
        ("POST", "/mcp"),
        ("POST", "/v1/context"),
        ("GET", "/api/v1/initiatives"),
    ] {
        assert_eq!(
            call(&app, method, path, None, json!({})).await.0,
            StatusCode::UNAUTHORIZED,
            "{path}"
        );
    }
    let id = "00000000-0000-7000-8000-000000000001";
    let(status,_)=call(&app,"POST","/api/v1/nodes",Some("fixture-token"),json!({"id":id,"node_type":"episode","tier":"operational","name":"From compatibility API","body":"legacy fixture body","initiative":"alpha"})).await;
    assert!(status.is_success(), "{status}");
    assert!(read_node_full(&store, &id.into()).unwrap().is_some());
    let (status, value) = call(
        &app,
        "POST",
        "/v1/read",
        Some("fixture-token"),
        json!({"initiative":"alpha","node":id}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{value}");
    assert!(value.to_string().contains("legacy fixture body"));
    let (status, value) = call(
        &app,
        "POST",
        "/v1/note",
        Some("fixture-token"),
        json!({"initiative":"alpha","name":"Canonical","body":"canonical fixture body"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{value}");
    let id = value["data"]["primary"]["id"].as_str().unwrap();
    let (status, value) = call(
        &app,
        "GET",
        &format!("/api/v1/nodes/{id}"),
        Some("fixture-token"),
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(value["body"], "canonical fixture body");
    runtime.cancellation_token().cancel();
}

#[tokio::test]
async fn explicit_instance_ui_cookie_does_not_authorize_legacy_sharing_or_mcp() {
    use kaeru_server::{
        auth::{
            session::{WebSession, hash_password, random_secret},
            store::AccountStore,
        },
        settings::web::WebConfig,
    };
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("instance.toml");
    AccountStore::initialize(
        path.clone(),
        WebConfig {
            username: "owner".into(),
            password_hash: hash_password("fixture password").unwrap(),
            jwt_secret: random_secret(),
            mcp_token: "saved-token".into(),
            ..Default::default()
        },
    )
    .unwrap();
    let session = WebSession::new(AccountStore::open(path.clone()).unwrap());
    let (jwt, _, _) = session.issue(1).unwrap();
    assert!(session.verify(&jwt).is_some());
    drop(session);
    let config: MemoryServiceConfig = serde_json::from_value(
        json!({"api_token":"explicit-token","web_config_file":path,"hygiene_enable":false}),
    )
    .unwrap();
    let options = config.runtime_options().unwrap();
    assert!(options.ui_enabled);
    assert_eq!(options.access.mcp_token, "explicit-token");
    let runtime = application(&config, Arc::new(Store::open_in_memory().unwrap()), options);
    let app = runtime.router();
    let ui = app
        .clone()
        .oneshot(Request::get("/ui/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(ui.status(), StatusCode::OK);
    let view = app
        .clone()
        .oneshot(
            Request::get("/auth/session")
                .header("cookie", format!("kaeru_session={jwt}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(view.status(), StatusCode::OK);
    let value: Value =
        serde_json::from_slice(&to_bytes(view.into_body(), 65536).await.unwrap()).unwrap();
    assert_eq!(value["authenticated"], true);
    for path in ["/api/v1/initiatives", "/mcp"] {
        let response = app
            .clone()
            .oneshot(
                Request::get(path)
                    .header("cookie", format!("kaeru_session={jwt}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED, "{path}");
    }
    runtime.cancellation_token().cancel();
}
