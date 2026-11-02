use std::fmt;
use std::sync::Arc;

use rmcp::{
    RoleServer,
    service::{RxJsonRpcMessage, TxJsonRpcMessage},
    transport::Transport,
};

use super::SessionSlot;

pub struct SessionTransport<T: Transport<RoleServer> + 'static> {
    inner: Option<T>,
    slot: Arc<SessionSlot>,
}

impl<T: Transport<RoleServer> + 'static> SessionTransport<T> {
    pub(super) fn new(inner: T, slot: Arc<SessionSlot>) -> Self {
        Self {
            inner: Some(inner),
            slot,
        }
    }
}

#[derive(Debug)]
pub enum TransportError<E> {
    Inner(E),
    Join(tokio::task::JoinError),
    Closed,
}
impl<E: fmt::Display> fmt::Display for TransportError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inner(error) => error.fmt(f),
            Self::Join(error) => error.fmt(f),
            Self::Closed => f.write_str("MCP session transport is closing"),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for TransportError<E> {}

impl<T: Transport<RoleServer> + 'static> Transport<RoleServer> for SessionTransport<T> {
    type Error = TransportError<T::Error>;

    fn send(
        &mut self,
        item: TxJsonRpcMessage<RoleServer>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send + 'static {
        let future = self.inner.as_mut().map(|inner| inner.send(item));
        async move {
            match future {
                Some(future) => future.await.map_err(TransportError::Inner),
                None => Err(TransportError::Closed),
            }
        }
    }

    async fn receive(&mut self) -> Option<RxJsonRpcMessage<RoleServer>> {
        match self.inner.as_mut() {
            Some(inner) => inner.receive().await,
            None => None,
        }
    }

    async fn close(&mut self) -> Result<(), Self::Error> {
        let Some(mut inner) = self.inner.take() else {
            return Ok(());
        };
        let cleanup = Cleanup(self.slot.clone());
        // WorkerTransport removes its JoinHandle before awaiting it. Run
        // that close in an owned task: cancelling this caller must not lose
        // the worker join or release the admission slot prematurely.
        tokio::spawn(async move {
            let _cleanup = cleanup;
            inner.close().await
        })
        .await
        .map_err(TransportError::Join)?
        .map_err(TransportError::Inner)
    }
}

struct Cleanup(Arc<SessionSlot>);
impl Drop for Cleanup {
    fn drop(&mut self) {
        self.0.unregister();
    }
}

impl<T: Transport<RoleServer> + 'static> Drop for SessionTransport<T> {
    fn drop(&mut self) {
        let Some(mut inner) = self.inner.take() else {
            return;
        };
        let cleanup = Cleanup(self.slot.clone());
        if let Ok(runtime) = tokio::runtime::Handle::try_current() {
            // Construct the cleanup guard before spawning: it must also run
            // if shutdown drops this future before its first poll.
            runtime.spawn(async move {
                let _cleanup = cleanup;
                let _ = inner.close().await;
            });
        } else {
            drop(inner);
            drop(cleanup);
        }
    }
}
