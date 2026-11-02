//! Opt-in bounded diagnostics for HTTP handlers and logical MCP sessions.
//! Response timing ends at response headers, not at the end of an SSE stream.

use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use std::time::{Duration, Instant};

use crate::session::BoundedSessionManager;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use tokio_util::sync::CancellationToken;

#[derive(Default)]
struct Counters {
    started: AtomicU64,
    returned_headers: AtomicU64,
    abandoned_handlers: AtomicU64,
    in_flight_handlers: AtomicU64,
    sessions_issued: AtomicU64,
}

struct HandlerGuard {
    counters: Arc<Counters>,
    returned: bool,
}

impl HandlerGuard {
    fn begin(counters: Arc<Counters>) -> Self {
        counters.started.fetch_add(1, Ordering::Relaxed);
        counters.in_flight_handlers.fetch_add(1, Ordering::Relaxed);
        Self {
            counters,
            returned: false,
        }
    }

    fn returned(&mut self) {
        self.counters
            .returned_headers
            .fetch_add(1, Ordering::Relaxed);
        self.returned = true;
    }
}

impl Drop for HandlerGuard {
    fn drop(&mut self) {
        self.counters
            .in_flight_handlers
            .fetch_sub(1, Ordering::Relaxed);
        if !self.returned {
            self.counters
                .abandoned_handlers
                .fetch_add(1, Ordering::Relaxed);
        }
    }
}

fn fingerprint(headers: &axum::http::HeaderMap) -> Option<u64> {
    let session = headers.get("mcp-session-id")?;
    let mut hasher = DefaultHasher::new();
    session.as_bytes().hash(&mut hasher);
    Some(hasher.finish())
}

async fn observe(State(counters): State<Arc<Counters>>, request: Request, next: Next) -> Response {
    let mut guard = HandlerGuard::begin(counters.clone());
    let started = Instant::now();
    let method = request.method().clone();
    // Only fixed labels: arbitrary paths/query strings can contain user data.
    let route = match request.uri().path() {
        "/mcp" => "mcp",
        "/sse" => "sse",
        "/messages" => "messages",
        path if path.starts_with("/v1/") => "memory_api",
        path if path.starts_with("/ui/") => "ui",
        _ => "other",
    };
    let request_session = fingerprint(request.headers());
    let response = next.run(request).await;
    let response_session = fingerprint(response.headers());
    if request_session.is_none() && response_session.is_some() {
        counters.sessions_issued.fetch_add(1, Ordering::Relaxed);
    }
    guard.returned();
    tracing::info!(
        target: "kaeru_mcp::diagnostics",
        method = %method,
        route,
        status = response.status().as_u16(),
        response_headers_ms = started.elapsed().as_secs_f64() * 1000.0,
        session_fingerprint = response_session.or(request_session),
        "HTTP handler returned"
    );
    response
}

pub fn attach(
    router: axum::Router,
    sessions: Arc<BoundedSessionManager>,
    interval: Duration,
    cancel: CancellationToken,
) -> axum::Router {
    let counters = Arc::new(Counters::default());
    let samples = counters.clone();
    let local = sessions.local();
    tokio::spawn(async move {
        let mut timer = tokio::time::interval(interval);
        timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        loop {
            tokio::select! {
                () = cancel.cancelled() => break,
                _ = timer.tick() => {
                    // Sampling must not wait behind a slow session close.
                    let session_count = local.sessions.try_read().ok().map(|map| map.len());
                    tracing::info!(
                        target: "kaeru_mcp::diagnostics",
                        logical_sessions = session_count,
                        session_slots_in_use = sessions.slots_in_use(),
                        session_capacity = sessions.maximum(),
                        http_started = samples.started.load(Ordering::Relaxed),
                        http_returned_headers = samples.returned_headers.load(Ordering::Relaxed),
                        http_abandoned_handlers = samples.abandoned_handlers.load(Ordering::Relaxed),
                        in_flight_http_handlers = samples.in_flight_handlers.load(Ordering::Relaxed),
                        sessions_issued = samples.sessions_issued.load(Ordering::Relaxed),
                        "MCP diagnostic sample"
                    );
                }
            }
        }
    });
    router.layer(axum::middleware::from_fn_with_state(counters, observe))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn cancellation_releases_in_flight_handler_count() {
        let counters = Arc::new(Counters::default());
        let guard = HandlerGuard::begin(counters.clone());
        let (ready, started) = tokio::sync::oneshot::channel();
        let task = tokio::spawn(async move {
            let _guard = guard;
            ready.send(()).unwrap();
            std::future::pending::<()>().await;
        });
        started.await.unwrap();
        assert_eq!(counters.in_flight_handlers.load(Ordering::Relaxed), 1);
        task.abort();
        assert!(task.await.unwrap_err().is_cancelled());
        assert_eq!(counters.in_flight_handlers.load(Ordering::Relaxed), 0);
        assert_eq!(counters.abandoned_handlers.load(Ordering::Relaxed), 1);
        assert_eq!(counters.returned_headers.load(Ordering::Relaxed), 0);
    }
}
