//! The published OpenAPI document (#70), tested the way a client meets it:
//! over HTTP, as JSON.

use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt;
use kaeru_core::{EdgeType, Layer, NodeType, Store, Tier};
use serde_json::Value;
use tower::util::ServiceExt;

use crate::api::docs::BEARER;
use crate::api::router::{DOCS_PATH, OPENAPI_PATH, api_router};
use crate::api::state::AppState;

fn app(token: &str) -> axum::Router {
    api_router(AppState {
        api_token: Arc::from(token),
        store: Arc::new(Store::open_in_memory().expect("open")),
    })
}

async fn get(app: axum::Router, path: &str) -> (StatusCode, Vec<u8>) {
    let resp = app
        .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = resp.status();
    let bytes = resp
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes()
        .to_vec();
    (status, bytes)
}

async fn spec() -> Value {
    let (status, bytes) = get(app(""), OPENAPI_PATH).await;
    assert_eq!(status, StatusCode::OK);
    serde_json::from_slice(&bytes).expect("the spec is JSON")
}

/// The strings a schema's `enum` holds, `null` excluded.
fn enum_of(spec: &Value, schema: &str, field: &str) -> Vec<String> {
    spec["components"]["schemas"][schema]["properties"][field]["enum"]
        .as_array()
        .unwrap_or_else(|| panic!("{schema}.{field} carries no enum"))
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect()
}

/// A schema is only useful before a client has a working token — so it has to
/// be reachable without one, while everything else stays gated.
#[tokio::test]
async fn the_spec_is_public_while_the_api_stays_gated() {
    let (status, _) = get(app("secret"), OPENAPI_PATH).await;
    assert_eq!(status, StatusCode::OK, "no token needed for the schema");

    let (status, _) = get(app("secret"), "/api/v1/initiatives").await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "and auth is still on");
}

/// Every route the router serves is in the document, with its method. Routes
/// are registered through `routes!`, so this is really a check that nothing is
/// registered around it.
#[tokio::test]
async fn every_route_is_in_the_spec() {
    let spec = spec().await;
    let paths = &spec["paths"];
    let expected = [
        ("get", "/health"),
        ("post", "/api/v1/nodes"),
        ("get", "/api/v1/nodes/{id}"),
        ("delete", "/api/v1/nodes/{id}"),
        ("post", "/api/v1/edges"),
        ("delete", "/api/v1/edges"),
        ("get", "/api/v1/initiatives"),
        ("get", "/api/v1/initiatives/{name}/nodes"),
        ("get", "/api/v1/initiatives/{name}/edges"),
        ("post", "/api/v1/initiatives/{name}/rename"),
        ("delete", "/api/v1/initiatives/{name}"),
    ];
    for (method, path) in expected {
        assert!(
            paths[path][method].is_object(),
            "{method} {path} missing; the spec has: {:?}",
            paths.as_object().map(|p| p.keys().collect::<Vec<_>>())
        );
    }
    let operations: usize = paths
        .as_object()
        .unwrap()
        .values()
        .map(|item| item.as_object().unwrap().len())
        .sum();
    assert_eq!(
        operations,
        expected.len(),
        "and nothing undocumented beside them"
    );
}

/// The incident behind #70: an agent probing 422s concluded the node payload
/// had no `initiative` field — validation can only ever name *required* ones —
/// and posted a node nobody could find. The field is an `Option` in the type,
/// so a missing one gets an explanatory 400; the schema must still say it is
/// required, or it repeats the incident in writing.
#[tokio::test]
async fn initiative_is_published_as_required() {
    let spec = spec().await;
    let node = &spec["components"]["schemas"]["NodeIngestReq"];
    let required: Vec<&str> = node["required"]
        .as_array()
        .expect("NodeIngestReq lists required fields")
        .iter()
        .filter_map(Value::as_str)
        .collect();
    assert!(required.contains(&"initiative"), "required: {required:?}");
    assert_eq!(node["properties"]["initiative"]["type"], "string");
}

/// Optional fields are the ones a 422 can never reveal — they have to be
/// visible here. `properties` is where a citation's URL travels (#85).
#[tokio::test]
async fn optional_fields_are_visible() {
    let spec = spec().await;
    let props = &spec["components"]["schemas"]["NodeIngestReq"]["properties"];
    for field in ["body", "tags", "layer", "properties"] {
        assert!(props[field].is_object(), "{field} is not published");
    }
    assert_eq!(
        spec["components"]["schemas"]["EdgeIngestReq"]["properties"]["weight"]["default"], 1.0,
        "the weight an omitted field falls back to is stated"
    );
}

/// Closed vocabularies are published from the core's own lists, not copied
/// into attributes — so the schema cannot drift from what the server parses.
#[tokio::test]
async fn closed_vocabularies_come_from_the_core() {
    let spec = spec().await;
    assert_eq!(
        enum_of(&spec, "NodeIngestReq", "node_type"),
        NodeType::VALID
    );
    assert_eq!(enum_of(&spec, "NodeIngestReq", "tier"), Tier::VALID);
    assert_eq!(enum_of(&spec, "NodeIngestReq", "layer"), Layer::VALID);
    assert_eq!(
        enum_of(&spec, "EdgeIngestReq", "edge_type"),
        EdgeType::VALID
    );
    assert_eq!(
        enum_of(&spec, "EdgeRetractReq", "edge_type"),
        EdgeType::VALID
    );
}

/// The handlers name the scheme with a string literal the macro requires; the
/// document registers it under `BEARER`. Those two are kept equal by hand, so
/// this is what keeps them equal — and only `/health` is public.
#[tokio::test]
async fn gated_operations_point_at_the_bearer_scheme() {
    let spec = spec().await;
    assert_eq!(
        spec["components"]["securitySchemes"][BEARER]["scheme"], "bearer",
        "the scheme is registered under the name the operations use"
    );
    for (path, item) in spec["paths"].as_object().unwrap() {
        for (method, op) in item.as_object().unwrap() {
            let gated = op["security"]
                .as_array()
                .is_some_and(|s| s.iter().any(|req| req.get(BEARER).is_some()));
            assert_eq!(gated, path != "/health", "{method} {path}");
        }
    }
}

/// Every error response carries `{"error": "…"}` — documented once.
#[tokio::test]
async fn the_error_body_is_documented() {
    let spec = spec().await;
    assert!(
        spec["components"]["schemas"]["ErrorBody"]["properties"]["error"].is_object(),
        "ErrorBody is registered"
    );
}

/// The listing's paging is a query a client cannot guess.
#[tokio::test]
async fn the_node_listing_documents_its_paging() {
    let spec = spec().await;
    let names: Vec<&str> = spec["paths"]["/api/v1/initiatives/{name}/nodes"]["get"]["parameters"]
        .as_array()
        .expect("parameters")
        .iter()
        .filter_map(|p| p["name"].as_str())
        .collect();
    for p in ["name", "limit", "offset", "q"] {
        assert!(names.contains(&p), "{p} missing from {names:?}");
    }
}

/// The version a client reads here is the version the service runs.
#[tokio::test]
async fn the_spec_carries_the_running_version() {
    assert_eq!(spec().await["info"]["version"], kaeru_core::version());
}

/// A person gets a browsable UI over the same document.
#[tokio::test]
async fn swagger_ui_is_served() {
    let (status, _) = get(app("secret"), DOCS_PATH).await;
    assert!(
        status == StatusCode::OK || status.is_redirection(),
        "{DOCS_PATH} answered {status}"
    );
    let resp = app("secret")
        .oneshot(
            Request::builder()
                .uri(format!("{DOCS_PATH}/"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let ct = resp
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_string();
    assert!(ct.starts_with("text/html"), "content-type: {ct}");
}
