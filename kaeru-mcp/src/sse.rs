//! Legacy MCP HTTP+SSE transport adapter.
//!
//! rmcp 1.x ships only the new Streamable HTTP server transport. Some
//! MCP clients (notably Opencode 1.15.x — see opencode-ai/opencode#8058,
//! #6242) still speak the older HTTP+SSE wire format from the 2024-11-05
//! MCP spec. This module bridges the gap: GET `/sse` opens a long-lived
//! SSE stream whose first event tells the client which URL to POST
//! requests at (`/messages?session_id=<uuid>`); each POST is fed into a
//! per-session rmcp service running against the same `KaeruServer` the
//! streamable HTTP path uses. Responses flow back out as
//! `event: message` SSE frames.
//!
//! Single-writer RocksDB ownership is preserved — only the daemon
//! process holds the lock; SSE sessions are just additional in-process
//! mcp-protocol consumers of the same `KaeruServer`.
//!
//! When opencode (or any other client) gets a Streamable HTTP
//! implementation, the wiring switches back to `/mcp` and this module
//! can be retired.

use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::routing::{get, post};
use axum::{Json, Router};
use rmcp::ServiceExt;
use rmcp::service::{RoleServer, RxJsonRpcMessage, TxJsonRpcMessage};
use rmcp::transport::Transport;
use serde::Deserialize;
use tokio::sync::{Mutex, mpsc};
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::{Stream, StreamExt};
use uuid::Uuid;

use crate::server::KaeruServer;

/// Per-session channel capacity. Both directions are bounded; backpressure
/// shows up as a slow tool call rather than an OOM if a client stalls.
const CHANNEL_BUFFER: usize = 32;

/// SSE keep-alive comment cadence. Without this, idle sessions get
/// reaped by reverse proxies (and some clients) after their own idle
/// timeout.
const SSE_KEEP_ALIVE: Duration = Duration::from_secs(15);

#[derive(Clone)]
struct SseState {
    server: KaeruServer,
    sessions: Arc<Mutex<HashMap<Uuid, mpsc::Sender<RxJsonRpcMessage<RoleServer>>>>>,
    messages_path: Arc<str>,
}

/// Build an axum router exposing the legacy HTTP+SSE MCP transport.
///
/// Two routes are added:
/// - `GET  <sse_path>`     — opens the SSE stream, emits the endpoint event.
/// - `POST <messages_path>` — JSON-RPC request entry-point.
pub fn router(server: KaeruServer, sse_path: &str, messages_path: &str) -> Router {
    let state = SseState {
        server,
        sessions: Arc::new(Mutex::new(HashMap::new())),
        messages_path: Arc::from(messages_path),
    };
    Router::new()
        .route(sse_path, get(open_stream))
        .route(messages_path, post(receive_message))
        .with_state(state)
}

async fn open_stream(
    State(state): State<SseState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let session_id = Uuid::new_v4();
    let (to_sse_tx, to_sse_rx) = mpsc::channel::<TxJsonRpcMessage<RoleServer>>(CHANNEL_BUFFER);
    let (from_post_tx, from_post_rx) =
        mpsc::channel::<RxJsonRpcMessage<RoleServer>>(CHANNEL_BUFFER);

    state.sessions.lock().await.insert(session_id, from_post_tx);

    let transport = SseTransport {
        outbound: to_sse_tx,
        inbound: from_post_rx,
    };

    let server = state.server.clone();
    let sessions = state.sessions.clone();
    tokio::spawn(async move {
        match server.serve(transport).await {
            Ok(running) => match running.waiting().await {
                Ok(reason) => {
                    tracing::debug!(%session_id, ?reason, "sse session finished");
                }
                Err(e) => {
                    tracing::warn!(%session_id, error = %e, "sse session ended with error");
                }
            },
            Err(e) => {
                tracing::warn!(%session_id, error = %e, "sse session failed to initialize");
            }
        }
        sessions.lock().await.remove(&session_id);
    });

    let endpoint_uri = format!("{}?session_id={}", state.messages_path, session_id);
    let endpoint_event = Event::default().event("endpoint").data(endpoint_uri);

    let messages =
        ReceiverStream::new(to_sse_rx).filter_map(|msg| match serde_json::to_string(&msg) {
            Ok(json) => Some(Ok::<_, Infallible>(
                Event::default().event("message").data(json),
            )),
            Err(e) => {
                tracing::warn!(error = %e, "dropping unserializable SSE message");
                None
            }
        });
    let stream = tokio_stream::once(Ok::<_, Infallible>(endpoint_event)).chain(messages);

    Sse::new(stream).keep_alive(KeepAlive::new().interval(SSE_KEEP_ALIVE))
}

#[derive(Deserialize)]
struct SessionQuery {
    session_id: Uuid,
}

async fn receive_message(
    State(state): State<SseState>,
    Query(SessionQuery { session_id }): Query<SessionQuery>,
    Json(msg): Json<RxJsonRpcMessage<RoleServer>>,
) -> impl IntoResponse {
    let sessions = state.sessions.lock().await;
    match sessions.get(&session_id) {
        Some(tx) => match tx.send(msg).await {
            Ok(()) => StatusCode::ACCEPTED.into_response(),
            Err(_) => (StatusCode::GONE, "sse session closed").into_response(),
        },
        None => (StatusCode::NOT_FOUND, "unknown session").into_response(),
    }
}

/// Bare-bones implementation of `rmcp::transport::Transport` over a pair
/// of tokio mpsc channels. The `outbound` side carries server-to-client
/// messages that get serialized into SSE `event: message` frames; the
/// `inbound` side carries client-to-server messages POSTed to
/// `/messages?session_id=<uuid>`.
struct SseTransport {
    outbound: mpsc::Sender<TxJsonRpcMessage<RoleServer>>,
    inbound: mpsc::Receiver<RxJsonRpcMessage<RoleServer>>,
}

impl Transport<RoleServer> for SseTransport {
    type Error = std::io::Error;

    fn send(
        &mut self,
        item: TxJsonRpcMessage<RoleServer>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send + 'static {
        // Transport::send's returned future must be 'static — clone the
        // Sender (cheap on tokio mpsc) so the move-block doesn't borrow self.
        let outbound = self.outbound.clone();
        async move {
            outbound.send(item).await.map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::BrokenPipe, "sse stream closed")
            })
        }
    }

    fn receive(&mut self) -> impl Future<Output = Option<RxJsonRpcMessage<RoleServer>>> + Send {
        self.inbound.recv()
    }

    async fn close(&mut self) -> Result<(), Self::Error> {
        self.inbound.close();
        Ok(())
    }
}
