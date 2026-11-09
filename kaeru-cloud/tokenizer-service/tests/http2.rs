mod common;

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::time::Duration;

use axum::serve::ListenerExt;
use contracts::{
    AccessCheck, AccessMode, Action, AuthorizeRequest, AuthorizeResponse, BootstrapRequest,
    ErrorResponse, POLICY_VERSION, Resource, SCHEMA_VERSION, SecretToken, TokenOperationResponse,
};
use kaeru_tokenizer_service::{
    api::{self, ApiState},
    codec::{decode, encode},
    config::{ServiceKeys, defaults::MAX_BODY_BYTES},
};
use reqwest::{Client, StatusCode, Version};
use tokio::{net::TcpListener, sync::oneshot};
use uuid::Uuid;

use common::database::{Database, create};
use common::{I1, USER, snapshot};

const CONTROL_KEY: &str = "synthetic-control-key-for-tests-only-0001";
const MEMORY_KEY: &str = "synthetic-memory-key-for-tests-only-00002";

#[tokio::test]
#[ignore = "requires isolated PostgreSQL; see service README"]
async fn real_http2_cbor_reuses_connections_and_never_replays_a_secret() {
    let db = Database::new().await;
    let state = ApiState {
        store: db.store.clone(),
        keys: ServiceKeys::new(CONTROL_KEY, MEMORY_KEY).unwrap(),
        max_projection_records: 10_000,
    };
    let connections = Arc::new(AtomicUsize::new(0));
    let count = connections.clone();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let listener = listener.tap_io(move |_| {
        count.fetch_add(1, Ordering::SeqCst);
    });
    let (stop, stopped) = oneshot::channel();
    let server = tokio::spawn(async move {
        axum::serve(listener, api::router(state, MAX_BODY_BYTES))
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    let base = format!("http://{address}");
    let client = Client::builder()
        .http2_prior_knowledge()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();
    let response = client
        .get(format!("{base}/health/ready"))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let _ = response.bytes().await.unwrap();
    let request = BootstrapRequest {
        schema_version: SCHEMA_VERSION,
        policy_version: POLICY_VERSION,
        operation_id: Uuid::now_v7(),
        source_generation: Uuid::now_v7(),
        source_revision: 0,
        snapshot: snapshot(),
    };
    let response = client
        .post(format!("{base}/v1/rights/bootstrap"))
        .bearer_auth(CONTROL_KEY)
        .header("content-type", "application/cbor")
        .body(encode(&request).unwrap())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.version(), Version::HTTP_2);
    let _ = response.bytes().await.unwrap();
    let request = create();
    let bytes = encode(&request).unwrap();
    let response = client
        .post(format!("{base}/v1/tokens"))
        .bearer_auth(MEMORY_KEY)
        .header("content-type", "application/cbor")
        .body(bytes.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let _ = response.bytes().await.unwrap();
    let response = client
        .post(format!("{base}/v1/tokens"))
        .bearer_auth(CONTROL_KEY)
        .header("content-type", "application/cbor")
        .body(bytes.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["cache-control"], "no-store");
    let first: TokenOperationResponse = decode(&response.bytes().await.unwrap(), 32).unwrap();
    let secret = first.secret.unwrap();
    let response = client
        .post(format!("{base}/v1/tokens"))
        .bearer_auth(CONTROL_KEY)
        .header("content-type", "application/cbor")
        .body(bytes)
        .send()
        .await
        .unwrap();
    let replay: TokenOperationResponse = decode(&response.bytes().await.unwrap(), 32).unwrap();
    assert!(replay.secret.is_none());
    assert!(replay.replayed);
    let response = client
        .get(format!(
            "{base}/v1/tokens/{}?actor_id={USER}",
            first.token.id
        ))
        .bearer_auth(CONTROL_KEY)
        .send()
        .await
        .unwrap();
    let metadata = response.bytes().await.unwrap();
    assert!(
        !metadata
            .windows(secret.expose().len())
            .any(|w| w == secret.expose().as_bytes())
    );
    let check = AuthorizeRequest {
        schema_version: SCHEMA_VERSION,
        request_id: Uuid::now_v7(),
        credential: secret.clone(),
        checks: vec![AccessCheck {
            action: Action::Read,
            resource: Resource::Initiative(I1),
        }],
        mode: AccessMode::Normal,
    };
    let response = client
        .post(format!("{base}/v1/authorize"))
        .bearer_auth(MEMORY_KEY)
        .header("content-type", "application/cbor")
        .body(encode(&check).unwrap())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.version(), Version::HTTP_2);
    let decision: AuthorizeResponse = decode(&response.bytes().await.unwrap(), 32).unwrap();
    assert!(decision.allowed);
    assert_eq!(decision.principal_id, USER);
    let response = client
        .post(format!("{base}/v1/tokens"))
        .bearer_auth(secret.expose())
        .header("content-type", "application/cbor")
        .body(encode(&request).unwrap())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let _ = response.bytes().await.unwrap();
    let response = client
        .post(format!("{base}/v1/authorize"))
        .bearer_auth(MEMORY_KEY)
        .header("content-type", "application/json")
        .body("{}")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    let _ = response.bytes().await.unwrap();
    let mut trailing = encode(&check).unwrap();
    trailing.push(0);
    let response = client
        .post(format!("{base}/v1/authorize"))
        .bearer_auth(MEMORY_KEY)
        .header("content-type", "application/cbor")
        .body(trailing)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let _ = response.bytes().await.unwrap();
    let response = client
        .post(format!("{base}/v1/authorize"))
        .bearer_auth(MEMORY_KEY)
        .header("content-type", "application/cbor")
        .body(vec![0; MAX_BODY_BYTES + 1])
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let _ = response.bytes().await.unwrap();
    let bad = AuthorizeRequest {
        credential: SecretToken::new("invalid".into()),
        ..check
    };
    let response = client
        .post(format!("{base}/v1/authorize"))
        .bearer_auth(MEMORY_KEY)
        .header("content-type", "application/cbor")
        .body(encode(&bad).unwrap())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = response.bytes().await.unwrap();
    let error: ErrorResponse = decode(&body, 32).unwrap();
    assert_eq!(error.code, "invalid_credential");
    assert!(
        !body
            .windows(secret.expose().len())
            .any(|w| w == secret.expose().as_bytes())
    );
    assert_eq!(
        connections.load(Ordering::SeqCst),
        1,
        "one shared HTTP/2 connection should serve all requests"
    );
    db.store.close().await;
    let response = client
        .get(format!("{base}/health/ready"))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    let _ = response.bytes().await.unwrap();
    drop(client);
    let _ = stop.send(());
    tokio::time::timeout(Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap();
    db.cleanup().await;
}
