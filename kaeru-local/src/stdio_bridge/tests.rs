use super::*;
use std::borrow::Cow;
use std::sync::{Arc, Mutex};

use crate::session::BoundedSessionManager;
use crate::settings::MCP_PROTOCOL_VERSION;
use axum::{
    Router,
    extract::{Request, State},
    middleware::{self, Next},
    response::Response,
};
use rmcp::{
    ErrorData, RoleServer, ServerHandler,
    model::{
        ListToolsResult, PaginatedRequestParams, ProtocolVersion, ServerCapabilities, ServerConfig,
    },
    service::RequestContext,
    transport::streamable_http_server::{
        StreamableHttpServerConfig, StreamableHttpService,
        session::{SessionManager, local::LocalSessionManager},
    },
};
use serde_json::{Value, json};
use tokio_util::sync::CancellationToken;

#[test]
fn only_data_lines_survive_an_sse_frame() {
    let body = ": keep-alive\nevent: message\nid: 3\ndata: {\"jsonrpc\":\"2.0\",\"id\":1}\n\n";
    assert_eq!(parse_sse(body), vec![r#"{"jsonrpc":"2.0","id":1}"#]);
}
#[test]
fn a_stream_carrying_several_messages_relays_all_of_them() {
    assert_eq!(
        parse_sse("data: {\"id\":1}\n\ndata: {\"id\":2}\n\n"),
        vec![r#"{"id":1}"#, r#"{"id":2}"#]
    );
}
#[test]
fn a_keep_alive_only_frame_relays_nothing() {
    assert!(parse_sse(": ping\n\n").is_empty());
}

#[derive(Clone, Default)]
struct Handler {
    writes: Arc<std::sync::atomic::AtomicUsize>,
}
impl ServerHandler for Handler {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_protocol_version(MCP_PROTOCOL_VERSION)
    }
    fn supported_protocol_versions(&self) -> Cow<'static, [ProtocolVersion]> {
        Cow::Borrowed(ProtocolVersion::known_up_to(&MCP_PROTOCOL_VERSION))
    }
    async fn list_tools(
        &self,
        _: Option<PaginatedRequestParams>,
        _: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        Ok(ListToolsResult::default())
    }
    async fn call_tool(
        &self,
        _: rmcp::model::CallToolRequestParams,
        _: RequestContext<RoleServer>,
    ) -> Result<rmcp::model::CallToolResponse, ErrorData> {
        self.writes
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(rmcp::model::CallToolResult::success(vec![]).into())
    }
}

type Requests = Arc<Mutex<Vec<(String, Option<String>)>>>;
async fn record(State(requests): State<Requests>, request: Request, next: Next) -> Response {
    requests.lock().unwrap().push((
        request.method().to_string(),
        request
            .headers()
            .get(MCP_PROTOCOL_HEADER)
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned),
    ));
    next.run(request).await
}

struct Fixture {
    manager: Arc<BoundedSessionManager>,
    requests: Requests,
    url: String,
    port: u16,
    cancel: CancellationToken,
    server: tokio::task::JoinHandle<()>,
    handler: Handler,
}
impl Fixture {
    async fn new() -> Self {
        let mut local = LocalSessionManager::default();
        local.session_config.keep_alive = None;
        let manager = Arc::new(BoundedSessionManager::new(local, 1));
        let requests: Requests = Arc::default();
        let cancel = CancellationToken::new();
        let handler = Handler::default();
        let factory = handler.clone();
        let service = StreamableHttpService::new(
            move || Ok(factory.clone()),
            manager.clone(),
            StreamableHttpServerConfig::default().with_cancellation_token(cancel.child_token()),
        );
        let router = Router::new()
            .nest_service("/mcp", service)
            .layer(middleware::from_fn_with_state(
                manager.clone(),
                crate::session::admit,
            ))
            .layer(middleware::from_fn_with_state(
                Arc::<str>::from("bridge-test-token"),
                crate::auth::require_bearer,
            ))
            .layer(middleware::from_fn_with_state(requests.clone(), record));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });
        Self {
            manager,
            requests,
            url: format!("http://127.0.0.1:{port}/mcp"),
            port,
            cancel,
            server,
            handler,
        }
    }
    fn bridge(&self) -> Bridge {
        Bridge::new(
            self.url.clone(),
            self.port,
            Some("bridge-test-token".into()),
        )
    }
    async fn released(&self) {
        tokio::time::timeout(Duration::from_secs(3), async {
            while self.manager.slots_in_use() != 0 {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .unwrap();
    }
}

#[tokio::test]
async fn expired_session_is_reinitialized_once_and_only_the_rejected_write_is_retried() {
    let f = Fixture::new().await;
    let reaper = CancellationToken::new();
    f.manager
        .spawn_idle_reaper(Duration::from_millis(80), reaper.clone());
    let mut bridge = f.bridge();
    bridge.forward(&initialize(json!(1))).await.unwrap();
    bridge
        .forward(&json!({"jsonrpc":"2.0","method":"notifications/initialized"}).to_string())
        .await
        .unwrap();
    let old = bridge.session.clone().unwrap();
    f.released().await;
    reaper.cancel();
    let frames=bridge.forward(&json!({"jsonrpc":"2.0","id":"original-write","method":"tools/call","params":{"name":"write","arguments":{}}}).to_string()).await.unwrap();
    assert_eq!(
        frames.len(),
        1,
        "internal handshake frames leaked to client"
    );
    let result: Value = serde_json::from_str(&frames[0]).unwrap();
    assert_eq!(result["id"], "original-write");
    assert!(result["result"].is_object(), "{result}");
    assert_ne!(bridge.session.as_deref(), Some(old.as_str()));
    assert_eq!(
        f.handler.writes.load(std::sync::atomic::Ordering::Relaxed),
        1
    );
    bridge.close_session().await;
    f.released().await;
}

#[tokio::test]
async fn cancellation_for_an_expired_session_does_not_restart_or_replay_into_a_new_session() {
    let f = Fixture::new().await;
    let mut bridge = f.bridge();
    bridge.forward(&initialize(json!(1))).await.unwrap();
    f.manager
        .close_session(&Arc::from(bridge.session.as_deref().unwrap()))
        .await
        .unwrap();
    f.released().await;
    let frames = bridge
        .forward(
            &json!({"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":2}})
                .to_string(),
        )
        .await
        .unwrap();
    assert!(frames.is_empty());
    assert_eq!(f.manager.slots_in_use(), 0);
    assert_eq!(
        f.handler.writes.load(std::sync::atomic::Ordering::Relaxed),
        0
    );
}

#[tokio::test]
async fn unrelated_http_404_is_not_reinitialized_or_retried() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let calls = Arc::new(AtomicUsize::new(0));
    let recorded = calls.clone();
    let router = Router::new().route(
        "/mcp",
        axum::routing::post(move || {
            recorded.fetch_add(1, Ordering::Relaxed);
            async { (axum::http::StatusCode::NOT_FOUND, "resource not found") }
        }),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
    let mut bridge = Bridge::new(format!("http://127.0.0.1:{port}/mcp"), port, None);
    bridge.session = Some("existing".into());
    bridge.initialize_request = Some(initialize(json!(1)));
    let frames = bridge
        .forward(
            &json!({"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"write"}})
                .to_string(),
        )
        .await
        .unwrap();
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(
        serde_json::from_str::<Value>(&frames[0]).unwrap()["error"]["data"]["http_status"],
        404
    );
    server.abort();
}

#[tokio::test]
async fn backend_failure_or_timeout_never_replays_a_potentially_committed_write() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    for timeout in [false, true] {
        let calls = Arc::new(AtomicUsize::new(0));
        let recorded = calls.clone();
        let router = Router::new().route(
            "/mcp",
            axum::routing::post(move || {
                recorded.fetch_add(1, Ordering::Relaxed);
                async move {
                    if timeout {
                        tokio::time::sleep(Duration::from_millis(200)).await;
                    }
                    (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        "response lost after commit",
                    )
                }
            }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });
        let mut bridge = Bridge::new(format!("http://127.0.0.1:{port}/mcp"), port, None);
        bridge.http = Client::builder()
            .timeout(Duration::from_millis(80))
            .build()
            .unwrap();
        bridge.session = Some("existing".into());
        bridge.initialize_request = Some(initialize(json!(1)));
        let response = bridge
            .forward(
                &json!({"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"write"}})
                    .to_string(),
            )
            .await;
        if timeout {
            assert!(response.unwrap_err().is_timeout());
        } else {
            assert_eq!(
                serde_json::from_str::<Value>(&response.unwrap()[0]).unwrap()["error"]["data"]["http_status"],
                500
            );
        }
        assert_eq!(
            calls.load(Ordering::Relaxed),
            1,
            "ambiguous write was replayed"
        );
        server.abort();
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        self.cancel.cancel();
        self.server.abort();
    }
}

fn initialize(id: Value) -> String {
    json!({"jsonrpc":"2.0","id":id,"method":"initialize","params":{
        "protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"bridge-test","version":"1"}}}).to_string()
}

#[tokio::test]
async fn eof_deletes_owned_session_and_echoes_negotiated_protocol() {
    let f = Fixture::new().await;
    let input = format!(
        "{}\n{}\n{}\n",
        initialize(json!(1)),
        json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
        json!({"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}})
    );
    f.bridge()
        .run_io(
            BufReader::new(input.as_bytes()),
            tokio::io::sink(),
            std::future::pending(),
        )
        .await
        .unwrap();
    f.released().await;
    let requests = f.requests.lock().unwrap();
    assert_eq!(requests.len(), 4);
    assert_eq!(requests[0], ("POST".into(), None));
    assert!(
        requests[1..]
            .iter()
            .all(|(_, version)| version.as_deref() == Some("2025-06-18"))
    );
    assert_eq!(requests[3].0, "DELETE");
}

#[tokio::test]
async fn stopping_relay_closes_session_even_with_stdin_open() {
    let f = Fixture::new().await;
    let (mut input, reader) = tokio::io::duplex(4096);
    let (writer, output) = tokio::io::duplex(4096);
    let (stop, stopped) = tokio::sync::oneshot::channel();
    let mut bridge = f.bridge();
    let task = tokio::spawn(async move {
        bridge
            .run_io(BufReader::new(reader), writer, async {
                let _ = stopped.await;
            })
            .await
    });
    input
        .write_all((initialize(json!(1)) + "\n").as_bytes())
        .await
        .unwrap();
    let mut line = String::new();
    tokio::time::timeout(
        Duration::from_secs(3),
        BufReader::new(output).read_line(&mut line),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(serde_json::from_str::<Value>(&line).unwrap()["result"].is_object());
    assert_eq!(f.manager.slots_in_use(), 1);
    stop.send(()).unwrap();
    task.await.unwrap().unwrap();
    f.released().await;
    assert!(
        f.requests
            .lock()
            .unwrap()
            .iter()
            .any(|(method, _)| method == "DELETE")
    );
}

#[tokio::test]
async fn quota_errors_are_correlated_rpc_errors_and_notifications_have_no_reply() {
    let f = Fixture::new().await;
    let mut first = f.bridge();
    first.forward(&initialize(json!(1))).await.unwrap();
    let mut second = f.bridge();
    let frames = second.forward(&initialize(json!("waiting"))).await.unwrap();
    assert_eq!(frames.len(), 1);
    let error: Value = serde_json::from_str(&frames[0]).unwrap();
    assert_eq!(error["id"], "waiting");
    assert_eq!(error["error"]["code"], -32603);
    assert_eq!(error["error"]["data"]["http_status"], 503);
    assert_eq!(error["error"]["data"]["retry_after_seconds"], 60);
    assert!(
        second
            .forward(&json!({"jsonrpc":"2.0","method":"notifications/initialized"}).to_string())
            .await
            .unwrap()
            .is_empty()
    );
    first.close_session().await;
    f.released().await;
    second.forward(&initialize(json!(3))).await.unwrap();
    second.close_session().await;
    f.released().await;
}

#[tokio::test]
async fn backend_rpc_errors_are_not_replaced_by_generic_http_errors() {
    let expected = json!({"jsonrpc":"2.0","id":7,"error":{"code":-32602,"message":"specific parameter error"}});
    let payload = expected.clone();
    let router = Router::new().route(
        "/mcp",
        axum::routing::post(move || {
            let body = payload.clone();
            async move { (axum::http::StatusCode::BAD_REQUEST, axum::Json(body)) }
        }),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });
    let mut bridge = Bridge::new(format!("http://127.0.0.1:{port}/mcp"), port, None);
    let frames = bridge
        .forward(&json!({"jsonrpc":"2.0","id":7,"method":"tools/list"}).to_string())
        .await
        .unwrap();
    assert_eq!(serde_json::from_str::<Value>(&frames[0]).unwrap(), expected);
    server.abort();
}
