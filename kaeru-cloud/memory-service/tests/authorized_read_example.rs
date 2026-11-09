//! Real HTTP/2 boundary tests for the opt-in integration adapter.
#[path = "../examples/authorized_read/client.rs"]
mod client;
#[path = "../examples/authorized_read/config.rs"]
mod config;
#[allow(dead_code)]
#[path = "../examples/authorized_read/models.rs"]
mod models;
#[path = "../examples/authorized_read/router.rs"]
mod router;
use axum::{
    Router,
    body::{Body, Bytes},
    extract::State,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::post,
    serve::ListenerExt,
};
use contracts::{AuthorizeRequest, AuthorizeResponse, SCHEMA_VERSION};
use kaeru_core::{Layer, NodeType, Store, Tier, Visibility, upsert_node};
use kaeru_runtime::ApiConfig;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
use tower::ServiceExt;
use uuid::Uuid;

#[derive(Clone, Default)]
struct Mock {
    calls: Arc<AtomicUsize>,
    connections: Arc<AtomicUsize>,
    mode: Arc<AtomicUsize>,
}
async fn authorize(State(mock): State<Mock>, body: Bytes) -> axum::response::Response {
    mock.calls.fetch_add(1, Ordering::SeqCst);
    let request: AuthorizeRequest = ciborium::from_reader(body.as_ref()).unwrap();
    let mode = mock.mode.load(Ordering::SeqCst);
    let mut decision = AuthorizeResponse {
        schema_version: SCHEMA_VERSION,
        request_id: request.request_id,
        allowed: mode != 1,
        principal_id: Uuid::from_u128(if request.credential.expose() == "other-principal" {
            2
        } else {
            1
        }),
        checks: request.checks,
        mode: request.mode,
        source_generation: Uuid::from_u128(99),
        source_revision: 0,
        authz_revision: 0,
    };
    match mode {
        2 => decision.request_id = Uuid::now_v7(),
        3 => decision.schema_version = 0,
        4 => decision.checks.clear(),
        5 => decision.mode = contracts::AccessMode::InspectDeleted,
        9 => decision.authz_revision = 1,
        _ => {}
    }
    let mut bytes = Vec::new();
    ciborium::into_writer(&decision, &mut bytes).unwrap();
    if mode == 6 {
        bytes.push(0);
    }
    if mode == 7 {
        bytes = vec![0; config::MAX_DECISION_BYTES + 1];
    }
    if mode == 8 {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    ([("content-type", "application/cbor")], bytes).into_response()
}
async fn setup(include_local: bool) -> (Router, Mock, tokio::task::JoinHandle<()>) {
    let mock = Mock::default();
    let count = mock.connections.clone();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let app = Router::new()
        .route("/v1/authorize", post(authorize))
        .with_state(mock.clone());
    let server = tokio::spawn(async move {
        axum::serve(
            listener.tap_io(move |_| {
                count.fetch_add(1, Ordering::SeqCst);
            }),
            app,
        )
        .await
        .unwrap();
    });
    let store = Arc::new(Store::open_in_memory().unwrap());
    for (id, scope, visibility) in [
        ("visible", "alpha", Visibility::Shared),
        ("private", "alpha", Visibility::Local),
        ("foreign", "beta", Visibility::Shared),
    ] {
        upsert_node(
            &store,
            &id.into(),
            NodeType::Episode,
            Tier::Operational,
            id,
            Some(&"synthetic body ".repeat(1400)),
            &[],
            Some(scope),
            visibility,
            Layer::Warm,
            None,
        )
        .unwrap();
    }
    let mapping = models::Mapping {
        revision: Uuid::now_v7(),
        initiatives: BTreeMap::from([
            ("alpha".into(), Uuid::from_u128(10)),
            ("beta".into(), Uuid::from_u128(20)),
        ]),
    };
    let app = router::router(router::App {
        store,
        cfg: ApiConfig {
            allow: vec!["alpha".into()],
            include_local,
            ..Default::default()
        },
        mapping,
        authorizer: client::Authorizer::new(
            &format!("http://{address}"),
            contracts::SecretToken::new("synthetic-memory-service-key".into()),
        )
        .unwrap(),
    })
    .unwrap();
    (app, mock, server)
}
async fn read(app: &Router, args: Value, token: Option<&str>) -> (StatusCode, Value) {
    let mut request = Request::post("/v1/read").header("content-type", "application/json");
    if let Some(token) = token {
        request = request.header("authorization", format!("Bearer {token}"));
    }
    let response = app
        .clone()
        .oneshot(request.body(Body::from(args.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), 1024 * 1024)
        .await
        .unwrap();
    (status, serde_json::from_slice(&body).unwrap())
}
#[tokio::test]
async fn all_pages_reauthorize_and_reuse_one_http2_connection() {
    let (app, mock, server) = setup(false).await;
    let mut args = json!({"initiative":"alpha","node":"visible","budget_chars":5000});
    let mut pages = 0;
    let mut body = String::new();
    loop {
        let (status, response) = read(&app, args, Some("one-credential")).await;
        assert_eq!(status, StatusCode::OK, "{response}");
        pages += 1;
        body.push_str(response["data"]["node"]["body"].as_str().unwrap());
        match response.pointer("/continuation/next_arguments") {
            Some(next) => args = next.clone(),
            None => break,
        }
    }
    assert_eq!(body, "synthetic body ".repeat(1400));
    assert!(pages > 1);
    assert_eq!(mock.calls.load(Ordering::SeqCst), pages);
    assert_eq!(mock.connections.load(Ordering::SeqCst), 1);
    server.abort();
}
#[tokio::test]
async fn operator_scope_local_visibility_and_cross_initiative_references_stay_enforced() {
    let (app, mock, server) = setup(false).await;
    for args in [
        json!({"initiative":"beta","node":"foreign"}),
        json!({"initiative":"alpha","node":"foreign"}),
        json!({"initiative":"alpha","node":"private"}),
    ] {
        let (status, response) = read(&app, args, Some("one-credential")).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(response.pointer("/data/node").is_none());
    }
    let before = mock.calls.load(Ordering::SeqCst);
    for (args, token, expected) in [
        (
            json!({"initiative":"unknown","node":"visible"}),
            Some("one-credential"),
            StatusCode::NOT_FOUND,
        ),
        (
            json!({"node":"visible"}),
            Some("one-credential"),
            StatusCode::BAD_REQUEST,
        ),
        (
            json!({"initiative":"alpha","node":"visible"}),
            None,
            StatusCode::UNAUTHORIZED,
        ),
    ] {
        let (status, response) = read(&app, args, token).await;
        assert_eq!(status, expected);
        assert!(response.pointer("/data/node").is_none());
    }
    assert_eq!(mock.calls.load(Ordering::SeqCst), before);
    server.abort();
}
#[tokio::test]
async fn replay_by_another_credential_or_principal_and_revocation_between_pages_fail_closed() {
    let (app, mock, server) = setup(false).await;
    let args = json!({"initiative":"alpha","node":"visible","budget_chars":5000});
    let (status, response) = read(&app, args, Some("one-credential")).await;
    assert_eq!(status, StatusCode::OK);
    let next = response["continuation"]["next_arguments"].clone();
    for token in ["another-credential", "other-principal"] {
        let (status, response) = read(&app, next.clone(), Some(token)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(response["issues"][0]["code"], "CURSOR_STALE");
        assert!(response.pointer("/data/node").is_none());
    }
    mock.mode.store(9, Ordering::SeqCst);
    let (status, response) = read(&app, next.clone(), Some("one-credential")).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(response["issues"][0]["code"], "CURSOR_STALE");
    mock.mode.store(1, Ordering::SeqCst);
    let (status, response) = read(&app, next.clone(), Some("one-credential")).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert!(response.pointer("/data/node").is_none());
    mock.mode.store(0, Ordering::SeqCst);
    assert_eq!(
        read(&app, next, Some("one-credential")).await.0,
        StatusCode::OK,
        "failed probes must not consume an authorized cursor"
    );
    server.abort();
}
#[tokio::test]
async fn mismatched_oversize_unavailable_and_trailing_tokenizer_decisions_never_allow_a_read() {
    let (app, mock, server) = setup(true).await;
    for mode in 2..=8 {
        mock.mode.store(mode, Ordering::SeqCst);
        let (status, response) = read(
            &app,
            json!({"initiative":"alpha","node":"visible"}),
            Some("one-credential"),
        )
        .await;
        assert_eq!(
            status,
            StatusCode::SERVICE_UNAVAILABLE,
            "mode {mode}: {response}"
        );
        assert!(response.pointer("/data/node").is_none());
    }
    server.abort();
}
#[test]
fn mapping_rejects_aliases_wildcards_and_duplicate_ids() {
    let id = Uuid::now_v7();
    for names in [vec!["alpha*"], vec!["alpha", "beta"], vec![" alpha"]] {
        let mapping = models::Mapping {
            revision: Uuid::now_v7(),
            initiatives: names.into_iter().map(|s| (s.into(), id)).collect(),
        };
        assert!(mapping.validate().is_err());
    }
}

#[tokio::test]
async fn unavailable_transport_fails_closed() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);
    let authorizer = client::Authorizer::new(
        &format!("http://{addr}"),
        contracts::SecretToken::new("synthetic-service-key".into()),
    )
    .unwrap();
    assert_eq!(
        authorizer
            .authorize("credential", Uuid::now_v7(), Uuid::now_v7())
            .await,
        Err(client::Denial::Unavailable)
    );
}
