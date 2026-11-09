use std::borrow::Cow;
use std::sync::Arc;
use std::time::Duration;

use axum::{Router, middleware};
use reqwest::{Client, Response, StatusCode};
use rmcp::{
    ErrorData, RoleServer, ServerHandler,
    model::{
        CallToolRequestParams, CallToolResponse, CallToolResult, ProtocolVersion,
        ServerCapabilities, ServerConfig,
    },
    service::RequestContext,
    transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService},
};
use serde_json::{Value, json};
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use super::*;
use crate::settings::MCP_PROTOCOL_VERSION;

#[derive(Clone, Default)]
struct Handler {
    started: Arc<Notify>,
    cancelled: Arc<Notify>,
    release: Arc<Notify>,
    finished: Arc<Notify>,
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
        _request: Option<rmcp::model::PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<rmcp::model::ListToolsResult, ErrorData> {
        let _session = require_lease(&context.extensions)?;
        Ok(rmcp::model::ListToolsResult::default())
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        assert!(
            context.extensions.get::<Arc<SessionSlot>>().is_some(),
            "SDK lost session lease extension"
        );
        // Mirror KaeruServer's partial context move; the remaining extensions
        // must live through the future, not merely through HTTP headers.
        let cancellation = context.ct;
        if request.name == "hold" {
            self.started.notify_one();
            cancellation.cancelled().await;
            self.cancelled.notify_one();
            self.release.notified().await;
            self.finished.notify_one();
        } else if request.name == "wait" {
            self.started.notify_one();
            self.release.notified().await;
            self.finished.notify_one();
        }
        Ok(CallToolResult::success(vec![]).into())
    }
}

struct Fixture {
    manager: Arc<BoundedSessionManager>,
    handler: Handler,
    cancel: CancellationToken,
    server: tokio::task::JoinHandle<()>,
    url: String,
    client: Client,
}

impl Fixture {
    async fn new(maximum: usize, protected: bool) -> Self {
        let mut local = LocalSessionManager::default();
        local.session_config.keep_alive = None;
        let manager = Arc::new(BoundedSessionManager::new(local, maximum));
        let cancel = CancellationToken::new();
        let handler = Handler::default();
        let factory = handler.clone();
        let service = StreamableHttpService::new(
            move || Ok(factory.clone()),
            manager.clone(),
            StreamableHttpServerConfig::default().with_cancellation_token(cancel.child_token()),
        );
        let mut router = Router::new()
            .nest_service("/mcp", service)
            .layer(middleware::from_fn_with_state(manager.clone(), admit));
        if protected {
            router = router.layer(middleware::from_fn_with_state(
                Arc::<str>::from("test-token"),
                crate::auth::require_bearer,
            ));
        }
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}/mcp", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });
        Self {
            manager,
            handler,
            cancel,
            server,
            url,
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
        }
    }

    async fn initialize(&self, index: usize, token: bool) -> Response {
        let mut request = self
            .client
            .post(&self.url)
            .header("accept", "application/json, text/event-stream")
            .json(
                &json!({"jsonrpc":"2.0","id":index,"method":"initialize","params":{
                "protocolVersion":MCP_PROTOCOL_VERSION,"capabilities":{},
                "clientInfo":{"name":"session-limit-test","version":"1"}}}),
            );
        if token {
            request = request.bearer_auth("test-token");
        }
        request.send().await.unwrap()
    }

    async fn rpc(&self, session: &str, payload: Value) -> Response {
        self.client
            .post(&self.url)
            .header("accept", "application/json, text/event-stream")
            .header("mcp-session-id", session)
            .header("mcp-protocol-version", MCP_PROTOCOL_VERSION.to_string())
            .json(&payload)
            .send()
            .await
            .unwrap()
    }

    async fn delete(&self, session: &str) {
        let result = self
            .client
            .delete(&self.url)
            .header("mcp-session-id", session)
            .send()
            .await
            .unwrap();
        assert!(result.status().is_success());
    }

    async fn until(&self, predicate: impl Fn() -> bool) {
        tokio::time::timeout(Duration::from_secs(8), async {
            while !predicate() {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .unwrap();
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        self.handler.release.notify_waiters();
        self.cancel.cancel();
        self.server.abort();
    }
}

async fn session(response: Response) -> String {
    assert_eq!(response.status(), StatusCode::OK);
    let id = response.headers()["mcp-session-id"]
        .to_str()
        .unwrap()
        .to_owned();
    let _ = response.text().await.unwrap();
    id
}

#[tokio::test]
async fn idle_reaper_returns_404_releases_capacity_and_allows_reinitialization() {
    let f = Fixture::new(1, false).await;
    f.manager
        .spawn_idle_reaper(Duration::from_millis(40), f.cancel.child_token());
    let id = session(f.initialize(1, false).await).await;
    f.until(|| f.manager.slots_in_use() == 0).await;
    assert!(f.manager.local().sessions.read().await.is_empty());
    let expired = f
        .rpc(&id, json!({"jsonrpc":"2.0","id":2,"method":"ping"}))
        .await;
    assert_eq!(expired.status(), StatusCode::NOT_FOUND);
    let replacement = session(f.initialize(3, false).await).await;
    assert_ne!(replacement, id);
    f.delete(&replacement).await;
    f.until(|| f.manager.slots_in_use() == 0).await;
}

#[tokio::test]
async fn disabled_expiry_and_initialization_are_not_reaped() {
    let f = Fixture::new(1, false).await;
    let id = session(f.initialize(1, false).await).await;
    assert_eq!(
        f.manager
            .reap_idle_at(
                tokio::time::Instant::now() + Duration::from_secs(3600),
                Duration::ZERO
            )
            .await,
        0
    );
    assert_eq!(f.manager.slots_in_use(), 1);
    f.delete(&id).await;
    f.until(|| f.manager.slots_in_use() == 0).await;

    let (id, transport) = f.manager.create_session().await.unwrap();
    assert_eq!(
        f.manager
            .reap_idle_at(
                tokio::time::Instant::now() + Duration::from_secs(3600),
                Duration::from_secs(1)
            )
            .await,
        0
    );
    drop(transport);
    f.manager.close_session(&id).await.unwrap();
    f.until(|| f.manager.slots_in_use() == 0).await;
}

#[tokio::test]
async fn slow_handler_survives_idle_timeout_and_gets_a_full_interval_after_completion() {
    let f = Fixture::new(1, false).await;
    let id = session(f.initialize(1, false).await).await;
    let response = f
        .rpc(
            &id,
            json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
        )
        .await;
    assert!(response.status().is_success());
    let client = f.client.clone();
    let url = f.url.clone();
    let sid = id.clone();
    let call = tokio::spawn(async move {
        client.post(url).header("accept","application/json, text/event-stream")
            .header("mcp-session-id",sid).header("mcp-protocol-version",MCP_PROTOCOL_VERSION.to_string())
            .json(&json!({"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"wait","arguments":{}}}))
            .send().await.unwrap().text().await.unwrap()
    });
    tokio::time::timeout(Duration::from_secs(2), f.handler.started.notified())
        .await
        .unwrap();
    let timeout = Duration::from_millis(40);
    f.manager.spawn_idle_reaper(timeout, f.cancel.child_token());
    tokio::time::sleep(timeout * 3).await;
    assert!(!call.is_finished(), "idle timeout cancelled active handler");
    assert!(
        f.manager
            .has_session(&Arc::from(id.as_str()))
            .await
            .unwrap()
    );
    assert_eq!(
        f.manager
            .reap_idle_at(
                tokio::time::Instant::now() + Duration::from_secs(60),
                timeout
            )
            .await,
        0
    );
    f.handler.release.notify_one();
    let body = call.await.unwrap();
    assert!(body.contains("result"), "{body}");
    assert_eq!(
        f.manager
            .reap_idle_at(tokio::time::Instant::now(), timeout)
            .await,
        0
    );
    f.until(|| f.manager.slots_in_use() == 0).await;
}

#[tokio::test]
async fn routing_lease_and_reaper_are_atomic_and_failed_routing_does_not_refresh_idle() {
    let f = Fixture::new(1, false).await;
    let id = session(f.initialize(1, false).await).await;
    let slot = f.manager.slot(&Arc::from(id.as_str())).unwrap();
    let routing = slot.activity(false).unwrap();
    let future = tokio::time::Instant::now() + Duration::from_secs(60);
    assert_eq!(
        f.manager.reap_idle_at(future, Duration::from_secs(1)).await,
        0
    );
    drop(routing);
    // Dropping a non-refreshing routing lease must not move the deadline.
    assert_eq!(
        f.manager.reap_idle_at(future, Duration::from_secs(1)).await,
        1
    );
    assert!(slot.activity(true).is_err());
    assert!(
        !f.manager
            .has_session(&Arc::from(id.as_str()))
            .await
            .unwrap()
    );
    drop(slot);
    f.until(|| f.manager.slots_in_use() == 0).await;
}

#[tokio::test]
async fn standalone_sse_does_not_keep_an_idle_session_alive() {
    let f = Fixture::new(1, false).await;
    let id = session(f.initialize(1, false).await).await;
    let stream = f
        .manager
        .create_standalone_stream(&Arc::from(id.as_str()))
        .await
        .unwrap();
    assert_eq!(
        f.manager
            .reap_idle_at(
                tokio::time::Instant::now() + Duration::from_secs(60),
                Duration::from_secs(1)
            )
            .await,
        1
    );
    assert!(
        !f.manager
            .has_session(&Arc::from(id.as_str()))
            .await
            .unwrap()
    );
    drop(stream);
    f.until(|| f.manager.slots_in_use() == 0).await;
}

#[tokio::test]
async fn unread_request_response_stream_is_protected_until_dropped() {
    let f = Fixture::new(1, false).await;
    let id = session(f.initialize(1, false).await).await;
    let id: SessionId = Arc::from(id.as_str());
    let message = serde_json::from_value(json!({"jsonrpc":"2.0","id":2,"method":"ping"})).unwrap();
    let stream = f.manager.create_stream(&id, message).await.unwrap();
    let future = tokio::time::Instant::now() + Duration::from_secs(60);
    assert_eq!(
        f.manager.reap_idle_at(future, Duration::from_secs(1)).await,
        0
    );
    drop(stream);
    tokio::time::timeout(Duration::from_secs(3), async {
        while f.manager.reap_idle_at(future, Duration::from_secs(1)).await == 0 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    f.until(|| f.manager.slots_in_use() == 0).await;
}

#[tokio::test]
async fn concurrent_http_routing_and_expiry_returns_success_or_404_never_500() {
    let f = Fixture::new(1, false).await;
    for index in 1..=20 {
        let id = session(f.initialize(index, false).await).await;
        let future = tokio::time::Instant::now() + Duration::from_secs(60);
        let (response, _) = tokio::join!(
            f.rpc(&id, json!({"jsonrpc":"2.0","id":100+index,"method":"ping"})),
            f.manager.reap_idle_at(future, Duration::from_secs(1)),
        );
        assert!(
            matches!(response.status(), StatusCode::OK | StatusCode::NOT_FOUND),
            "{}",
            response.status()
        );
        let _ = response.text().await.unwrap();
        f.delete(&id).await;
        f.until(|| f.manager.slots_in_use() == 0).await;
    }
}

#[tokio::test]
async fn simultaneous_initializes_are_bounded_and_existing_sessions_keep_working() {
    let f = Fixture::new(2, false).await;
    let mut jobs = tokio::task::JoinSet::new();
    for index in 1..=20 {
        let client = f.client.clone();
        let url = f.url.clone();
        jobs.spawn(async move {
            client
                .post(url)
                .header("accept", "application/json, text/event-stream")
                .json(
                    &json!({"jsonrpc":"2.0","id":index,"method":"initialize","params":{
                "protocolVersion":MCP_PROTOCOL_VERSION,"capabilities":{},
                "clientInfo":{"name":"burst","version":"1"}}}),
                )
                .send()
                .await
                .unwrap()
        });
    }
    let mut accepted = vec![];
    let mut rejected = 0;
    while let Some(result) = jobs.join_next().await {
        let response = result.unwrap();
        assert!(f.manager.slots_in_use() <= 2);
        if response.status() == StatusCode::OK {
            accepted.push(session(response).await);
        } else {
            assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
            assert_eq!(response.headers()["retry-after"], "60");
            rejected += 1;
        }
    }
    assert_eq!(accepted.len(), 2);
    assert_eq!(rejected, 18);
    let result = f
        .rpc(
            &accepted[0],
            json!({"jsonrpc":"2.0","id":40,"method":"ping"}),
        )
        .await;
    assert_eq!(result.status(), StatusCode::OK);
    let _ = result.text().await.unwrap();
    f.delete(&accepted[0]).await;
    f.until(|| f.manager.slots_in_use() == 1).await;
    let replacement = session(f.initialize(100, false).await).await;
    f.delete(&accepted[1]).await;
    f.delete(&replacement).await;
    f.until(|| f.manager.slots_in_use() == 0).await;
}

#[tokio::test]
async fn invalid_requests_do_not_leak_reservations() {
    let f = Fixture::new(1, false).await;
    for _ in 0..10 {
        let r = f
            .client
            .post(&f.url)
            .header("accept", "application/json, text/event-stream")
            .header("content-type", "application/json")
            .body("not-json")
            .send()
            .await
            .unwrap();
        assert!(r.status().is_client_error());
        f.until(|| f.manager.slots_in_use() == 0).await;
    }
    let id = session(f.initialize(1, false).await).await;
    f.delete(&id).await;
    f.until(|| f.manager.slots_in_use() == 0).await;
}

#[tokio::test]
async fn authentication_runs_before_admission() {
    let f = Fixture::new(1, true).await;
    assert_eq!(
        f.initialize(1, false).await.status(),
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(f.manager.slots_in_use(), 0);
    let _id = session(f.initialize(2, true).await).await;
    assert_eq!(
        f.initialize(3, false).await.status(),
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        f.initialize(4, true).await.status(),
        StatusCode::SERVICE_UNAVAILABLE
    );
}

#[tokio::test]
async fn cancelled_handler_keeps_slot_until_cooperative_completion_after_delete() {
    let f = Fixture::new(1, false).await;
    let id = session(f.initialize(1, false).await).await;
    let r = f
        .rpc(
            &id,
            json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
        )
        .await;
    assert!(r.status().is_success());
    let client = f.client.clone();
    let url = f.url.clone();
    let sid = id.clone();
    let call = tokio::spawn(async move {
        let response = client.post(url).header("accept", "application/json, text/event-stream")
            .header("mcp-session-id", sid).header("mcp-protocol-version", MCP_PROTOCOL_VERSION.to_string())
            .json(&json!({"jsonrpc":"2.0","id":42,"method":"tools/call","params":{"name":"hold","arguments":{}}}))
            .send().await.unwrap();
        let _ = response.text().await;
    });
    tokio::time::timeout(Duration::from_secs(3), f.handler.started.notified())
        .await
        .unwrap();
    let response = f.rpc(&id, json!({"jsonrpc":"2.0","method":"notifications/cancelled","params":{"requestId":42,"reason":"test"}})).await;
    assert!(response.status().is_success());
    tokio::time::timeout(Duration::from_secs(3), f.handler.cancelled.notified())
        .await
        .unwrap();
    f.delete(&id).await;
    f.until(|| f.manager.registry.lock().unwrap().is_empty())
        .await;
    assert_eq!(
        f.manager.slots_in_use(),
        1,
        "slot freed while cancelled handler still runs"
    );
    assert_eq!(
        f.initialize(50, false).await.status(),
        StatusCode::SERVICE_UNAVAILABLE
    );
    f.handler.release.notify_one();
    f.until(|| f.manager.slots_in_use() == 0).await;
    call.await.unwrap();
    let replacement = session(f.initialize(60, false).await).await;
    f.delete(&replacement).await;
    f.until(|| f.manager.slots_in_use() == 0).await;
}

#[tokio::test]
async fn cancelling_close_does_not_release_capacity_before_worker_finishes() {
    use rmcp::{
        service::{RxJsonRpcMessage, TxJsonRpcMessage},
        transport::Transport,
    };
    struct JoiningTransport {
        worker: Option<tokio::task::JoinHandle<()>>,
        started: Arc<Notify>,
    }
    impl Transport<RoleServer> for JoiningTransport {
        type Error = std::io::Error;
        fn send(
            &mut self,
            _: TxJsonRpcMessage<RoleServer>,
        ) -> impl Future<Output = Result<(), Self::Error>> + Send + 'static {
            std::future::ready(Ok(()))
        }
        async fn receive(&mut self) -> Option<RxJsonRpcMessage<RoleServer>> {
            std::future::pending().await
        }
        async fn close(&mut self) -> Result<(), Self::Error> {
            // Match WorkerTransport: the JoinHandle leaves self before await.
            if let Some(worker) = self.worker.take() {
                self.started.notify_one();
                worker.await.map_err(std::io::Error::other)?;
            }
            Ok(())
        }
    }
    let permits = Arc::new(tokio::sync::Semaphore::new(1));
    let registry = Arc::new(Mutex::new(HashMap::new()));
    let slot = Arc::new(SessionSlot {
        id: Arc::from("close-race"),
        _permit: permits.clone().try_acquire_owned().unwrap(),
        registry: Arc::downgrade(&registry),
        lifecycle: Mutex::default(),
    });
    registry
        .lock()
        .unwrap()
        .insert(slot.id.clone(), Arc::downgrade(&slot));
    let started = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    let worker_release = release.clone();
    let worker = tokio::spawn(async move {
        worker_release.notified().await;
    });
    let mut transport = transport::SessionTransport::new(
        JoiningTransport {
            worker: Some(worker),
            started: started.clone(),
        },
        slot,
    );
    let closing = tokio::spawn(async move {
        transport.close().await.unwrap();
    });
    started.notified().await;
    closing.abort();
    let _ = closing.await;
    tokio::time::sleep(Duration::from_millis(30)).await;
    assert_eq!(
        permits.available_permits(),
        0,
        "closing worker released capacity early"
    );
    release.notify_one();
    tokio::time::timeout(Duration::from_secs(2), async {
        while permits.available_permits() == 0 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    assert!(registry.lock().unwrap().is_empty());
}

#[tokio::test]
async fn aborted_initialize_scope_closes_unclaimed_session() {
    use rmcp::ServiceExt;
    let mut inner = LocalSessionManager::default();
    inner.session_config.keep_alive = None;
    let manager = Arc::new(BoundedSessionManager::new(inner, 1));
    let permit = manager.slots.clone().try_acquire_owned().unwrap();
    let (created, transport) = tokio::sync::oneshot::channel();
    let creator = manager.clone();
    let request = tokio::spawn(admission::scoped(permit, async move {
        let (_, session) = creator.create_session().await.unwrap();
        assert!(created.send(session).is_ok());
        std::future::pending::<()>().await;
    }));
    let transport = transport.await.unwrap();
    let service = tokio::spawn(async move {
        let _ = Handler::default().serve(transport).await;
    });
    assert_eq!(manager.slots_in_use(), 1);
    request.abort();
    let _ = request.await;
    tokio::time::timeout(Duration::from_secs(3), async {
        while manager.slots_in_use() != 0 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    tokio::time::timeout(Duration::from_secs(3), service)
        .await
        .unwrap()
        .unwrap();
    assert!(manager.local().sessions.read().await.is_empty());
    assert!(manager.registry.lock().unwrap().is_empty());
}

#[tokio::test]
async fn sdk_upgrade_does_not_enable_unbounded_stateless_protocol_requests() {
    let f = Fixture::new(1, false).await;
    let response = f
        .client
        .post(&f.url)
        .header("accept", "application/json, text/event-stream")
        .header("mcp-protocol-version", "2026-07-28")
        .json(
            &json!({"jsonrpc":"2.0","id":1,"method":"tools/list","params":{"_meta":{
            "io.modelcontextprotocol/protocolVersion":"2026-07-28",
            "io.modelcontextprotocol/clientCapabilities":{}}}}),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    f.until(|| f.manager.slots_in_use() == 0).await;
}

#[tokio::test]
async fn inline_metadata_cannot_bypass_stateful_sessions_with_a_supported_version() {
    for maximum in [1, 2] {
        let f = Fixture::new(maximum, false).await;
        let id = session(f.initialize(1, false).await).await;
        for supplied_id in [None, Some("forged-session"), Some(id.as_str())] {
            let mut request = f
                .client
                .post(&f.url)
                .header("accept", "application/json, text/event-stream")
                .header("mcp-protocol-version", "2025-11-25")
                .json(
                    &json!({"jsonrpc":"2.0","id":2,"method":"tools/list","params":{"_meta":{
                    "io.modelcontextprotocol/protocolVersion":"2025-11-25",
                    "io.modelcontextprotocol/clientCapabilities":{}}}}),
                );
            if let Some(id) = supplied_id {
                request = request.header("mcp-session-id", id);
            }
            let response = request.send().await.unwrap();
            if supplied_id.is_none() && maximum == 1 {
                assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
            } else {
                let text = response.text().await.unwrap();
                let value = serde_json::from_str::<Value>(&text)
                    .ok()
                    .or_else(|| {
                        text.lines()
                            .filter_map(|line| line.strip_prefix("data:"))
                            .find_map(|line| serde_json::from_str::<Value>(line.trim()).ok())
                    })
                    .unwrap();
                assert!(
                    value.get("result").is_none(),
                    "inline lifecycle reached a handler: {value}"
                );
                assert_eq!(value["error"]["code"], -32600);
            }
            assert_eq!(f.manager.slots_in_use(), 1);
        }
        let ping = f
            .rpc(&id, json!({"jsonrpc":"2.0","id":3,"method":"ping"}))
            .await;
        assert_eq!(ping.status(), StatusCode::OK);
        let _ = ping.text().await.unwrap();
        f.delete(&id).await;
        f.until(|| f.manager.slots_in_use() == 0).await;
    }
}

#[tokio::test]
async fn ordinary_and_nested_metadata_stays_compatible_in_a_managed_session() {
    let f = Fixture::new(1, false).await;
    let id = session(f.initialize(1, false).await).await;
    for meta in [
        json!({"progressToken":"progress-1","example/context":"kept"}),
        json!({"io.modelcontextprotocol/protocolVersion":"2025-11-25"}),
    ] {
        let response = f
            .rpc(
                &id,
                json!({"jsonrpc":"2.0","id":2,"method":"tools/list","params":{"_meta":meta}}),
            )
            .await;
        assert_eq!(response.status(), StatusCode::OK);
        let text = response.text().await.unwrap();
        assert!(text.contains("\"result\""), "{text}");
    }
    let response = f.rpc(&id,json!({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"ordinary","arguments":{"_meta":{
        "io.modelcontextprotocol/protocolVersion":"2025-11-25","io.modelcontextprotocol/clientCapabilities":{}}}}})).await;
    assert_eq!(response.status(), StatusCode::OK);
    let text = response.text().await.unwrap();
    assert!(text.contains("\"result\""), "{text}");
    f.delete(&id).await;
    f.until(|| f.manager.slots_in_use() == 0).await;
}
