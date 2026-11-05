//! Bounded stateful MCP sessions; application memory remains in Store.

mod activity;
mod admission;
#[cfg(test)]
mod tests;
mod transport;

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex, Weak};

use rmcp::model::{ClientJsonRpcMessage, GetExtensions, JsonRpcMessage, ServerJsonRpcMessage};
use rmcp::transport::streamable_http_server::session::{
    ServerSseMessage, SessionId, SessionManager,
    local::{LocalSessionManager, LocalSessionManagerError},
};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tokio_stream::Stream;
use tokio_stream::StreamExt;

pub use admission::admit;
use transport::SessionTransport;

pub(crate) fn require_lease(
    extensions: &rmcp::model::Extensions,
) -> Result<Arc<SessionSlot>, rmcp::ErrorData> {
    extensions
        .get::<Arc<SessionSlot>>()
        .cloned()
        .ok_or_else(|| {
            rmcp::ErrorData::invalid_request(
                "Stateful MCP initialization required; inline lifecycle is unsupported",
                None,
            )
        })
}

type Registry = Mutex<HashMap<SessionId, Weak<SessionSlot>>>;

pub(super) struct SessionSlot {
    id: SessionId,
    _permit: OwnedSemaphorePermit,
    registry: Weak<Registry>,
    lifecycle: Mutex<activity::Lifecycle>,
}

impl SessionSlot {
    fn unregister(&self) {
        if let Some(registry) = self.registry.upgrade() {
            let mut entries = registry.lock().unwrap_or_else(|p| p.into_inner());
            if entries
                .get(&self.id)
                .is_some_and(|entry| std::ptr::eq(entry.as_ptr(), self))
            {
                entries.remove(&self.id);
            }
        }
    }
}

#[derive(Debug)]
pub enum SessionError {
    Local(LocalSessionManagerError),
    Capacity,
    Unavailable,
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Local(error) => error.fmt(f),
            Self::Capacity => f.write_str("MCP session capacity reached"),
            Self::Unavailable => f.write_str("MCP session unavailable"),
        }
    }
}
impl std::error::Error for SessionError {}
impl From<LocalSessionManagerError> for SessionError {
    fn from(error: LocalSessionManagerError) -> Self {
        Self::Local(error)
    }
}

pub struct BoundedSessionManager {
    inner: Arc<LocalSessionManager>,
    slots: Arc<Semaphore>,
    maximum: usize,
    registry: Arc<Registry>,
}

impl BoundedSessionManager {
    pub fn new(inner: LocalSessionManager, maximum: usize) -> Self {
        assert!(maximum > 0);
        Self {
            inner: Arc::new(inner),
            slots: Arc::new(Semaphore::new(maximum)),
            maximum,
            registry: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn local(&self) -> Arc<LocalSessionManager> {
        self.inner.clone()
    }
    pub fn maximum(&self) -> usize {
        self.maximum
    }
    pub fn slots_in_use(&self) -> usize {
        self.maximum - self.slots.available_permits()
    }

    fn slot(&self, id: &SessionId) -> Result<Arc<SessionSlot>, SessionError> {
        let slot = self
            .registry
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .get(id)
            .and_then(Weak::upgrade)
            .ok_or(SessionError::Unavailable)?;
        if slot.is_open() {
            Ok(slot)
        } else {
            Err(SessionError::Unavailable)
        }
    }

    fn retain_slot(
        &self,
        id: &SessionId,
        message: &mut ClientJsonRpcMessage,
    ) -> Result<(), SessionError> {
        let slot = self.slot(id)?;
        let activity = slot.activity(true)?;
        // Extensions are moved into RequestContext by rmcp and remain alive
        // through Kaeru's async handler, including cooperative cancellation.
        match message {
            JsonRpcMessage::Request(request) => {
                request.request.extensions_mut().insert(slot);
                request.request.extensions_mut().insert(activity);
            }
            JsonRpcMessage::Notification(notification) => {
                notification.notification.extensions_mut().insert(slot);
                notification.notification.extensions_mut().insert(activity);
            }
            _ => {}
        }
        Ok(())
    }
}

impl SessionManager for BoundedSessionManager {
    type Error = SessionError;
    type Transport = SessionTransport<<LocalSessionManager as SessionManager>::Transport>;

    async fn create_session(&self) -> Result<(SessionId, Self::Transport), Self::Error> {
        let permit = admission::reservation(&self.slots)?;
        let (id, inner) = self.inner.create_session().await?;
        let slot = Arc::new(SessionSlot {
            id: id.clone(),
            _permit: permit,
            registry: Arc::downgrade(&self.registry),
            lifecycle: Mutex::default(),
        });
        self.registry
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .insert(id.clone(), Arc::downgrade(&slot));
        admission::created(self.inner.clone(), id.clone());
        Ok((id, SessionTransport::new(inner, slot)))
    }

    async fn initialize_session(
        &self,
        id: &SessionId,
        mut message: ClientJsonRpcMessage,
    ) -> Result<ServerJsonRpcMessage, Self::Error> {
        let slot = self.slot(id)?;
        self.retain_slot(id, &mut message)?;
        match self.inner.initialize_session(id, message).await {
            Ok(response) => {
                slot.initialized();
                Ok(response)
            }
            Err(error) => {
                slot.mark_closing();
                let _ = self.inner.close_session(id).await;
                Err(error.into())
            }
        }
    }

    async fn has_session(&self, id: &SessionId) -> Result<bool, Self::Error> {
        Ok(self.slot(id).is_ok() && self.inner.has_session(id).await?)
    }

    async fn close_session(&self, id: &SessionId) -> Result<(), Self::Error> {
        // The slot is released by transport shutdown, not by removing the ID
        // from the SDK lookup map. Surviving handlers retain their Arc too.
        if let Ok(slot) = self.slot(id) {
            slot.mark_closing();
        }
        self.inner.close_session(id).await.map_err(Into::into)
    }

    async fn create_stream(
        &self,
        id: &SessionId,
        mut message: ClientJsonRpcMessage,
    ) -> Result<impl Stream<Item = ServerSseMessage> + Send + Sync + 'static, Self::Error> {
        self.retain_slot(id, &mut message)?;
        let activity = self.slot(id)?.activity(true)?;
        let stream = self.inner.create_stream(id, message).await?;
        Ok(stream.map(move |event| {
            let _ = &activity;
            event
        }))
    }

    async fn accept_message(
        &self,
        id: &SessionId,
        mut message: ClientJsonRpcMessage,
    ) -> Result<(), Self::Error> {
        let _slot = self.slot(id)?;
        self.retain_slot(id, &mut message)?;
        self.inner
            .accept_message(id, message)
            .await
            .map_err(Into::into)
    }

    async fn create_standalone_stream(
        &self,
        id: &SessionId,
    ) -> Result<impl Stream<Item = ServerSseMessage> + Send + Sync + 'static, Self::Error> {
        let _activity = self.slot(id)?.activity(true)?;
        self.inner
            .create_standalone_stream(id)
            .await
            .map_err(Into::into)
    }

    async fn resume(
        &self,
        id: &SessionId,
        last_event_id: String,
    ) -> Result<impl Stream<Item = ServerSseMessage> + Send + Sync + 'static, Self::Error> {
        let _activity = self.slot(id)?.activity(true)?;
        self.inner
            .resume(id, last_event_id)
            .await
            .map_err(Into::into)
    }
}
