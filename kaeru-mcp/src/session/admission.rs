use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

use axum::{
    extract::{Request, State},
    http::{Method, StatusCode, header::RETRY_AFTER},
    middleware::Next,
    response::{IntoResponse, Response},
};
use rmcp::transport::streamable_http_server::session::{
    SessionId, SessionManager, local::LocalSessionManager,
};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use super::{BoundedSessionManager, SessionError};
use crate::settings::{MCP_SESSION_HEADER, SESSION_RETRY_AFTER_SECS};

tokio::task_local! {
    static RESERVED: Reservation;
}

struct Reservation {
    permit: Mutex<Option<OwnedSemaphorePermit>>,
    created: Mutex<Option<(Arc<LocalSessionManager>, SessionId)>>,
    delivered: AtomicBool,
}

impl Drop for Reservation {
    fn drop(&mut self) {
        if self.delivered.load(Ordering::Relaxed) {
            return;
        }
        let created = self
            .created
            .get_mut()
            .unwrap_or_else(|p| p.into_inner())
            .take();
        if let Some((manager, id)) = created
            && let Ok(runtime) = tokio::runtime::Handle::try_current()
        {
            // The HTTP future failed before returning an initialized session.
            // The caller cannot know this ID, so do not leave it abandoned.
            runtime.spawn(async move {
                let _ = manager.close_session(&id).await;
            });
        }
    }
}

pub(super) async fn scoped<T>(permit: OwnedSemaphorePermit, future: impl Future<Output = T>) -> T {
    RESERVED
        .scope(
            Reservation {
                permit: Mutex::new(Some(permit)),
                created: Mutex::new(None),
                delivered: AtomicBool::new(false),
            },
            future,
        )
        .await
}

pub(super) fn created(manager: Arc<LocalSessionManager>, id: SessionId) {
    let _ = RESERVED.try_with(|reserved| {
        *reserved.created.lock().unwrap_or_else(|p| p.into_inner()) = Some((manager, id));
    });
}

pub(super) fn reservation(slots: &Arc<Semaphore>) -> Result<OwnedSemaphorePermit, SessionError> {
    match RESERVED.try_with(|reserved| {
        reserved
            .permit
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .take()
    }) {
        Ok(permit) => permit.ok_or(SessionError::Capacity),
        // Direct SDK use remains bounded as well; the HTTP layer additionally
        // maps admission failures to a transport status rather than SDK 500.
        Err(_) => slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| SessionError::Capacity),
    }
}

pub async fn admit(
    State(manager): State<Arc<BoundedSessionManager>>,
    request: Request,
    next: Next,
) -> Response {
    if matches!(*request.method(), Method::GET | Method::POST)
        && let Some(id) = request
            .headers()
            .get(MCP_SESSION_HEADER)
            .and_then(|h| h.to_str().ok())
        && let Ok(slot) = manager.slot(&Arc::<str>::from(id))
        && let Ok(activity) = slot.activity(false)
    {
        // Pin routing before the SDK's has_session check. Request extensions
        // and response streams take over protection of accepted operations.
        let response = next.run(request).await;
        if response.status().is_success() {
            activity.refresh();
        }
        return response;
    }
    if request.method() != Method::POST || request.headers().contains_key(MCP_SESSION_HEADER) {
        return next.run(request).await;
    }
    let Ok(permit) = manager.slots.clone().try_acquire_owned() else {
        tracing::warn!(maximum = manager.maximum, "MCP session admission full");
        let mut response = (
            StatusCode::SERVICE_UNAVAILABLE,
            "MCP session capacity reached; close unused clients or retry later",
        )
            .into_response();
        response
            .headers_mut()
            .insert(RETRY_AFTER, SESSION_RETRY_AFTER_SECS.into());
        return response;
    };
    // The SDK creates the session synchronously within this HTTP routing
    // future. Unused reservations also drop on parse errors/cancellation.
    scoped(permit, async {
        let response = next.run(request).await;
        RESERVED.with(|reserved| {
            let created = reserved.created.lock().unwrap_or_else(|p| p.into_inner());
            let delivered = response.status().is_success()
                && created.as_ref().is_some_and(|(_, id)| {
                    response
                        .headers()
                        .get(MCP_SESSION_HEADER)
                        .and_then(|h| h.to_str().ok())
                        == Some(id.as_ref())
                });
            reserved.delivered.store(delivered, Ordering::Relaxed);
        });
        response
    })
    .await
}
