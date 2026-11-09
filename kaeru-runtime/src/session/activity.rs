//! Idle expiry follows application work, not socket lifetime or SSE heartbeats.

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use super::{BoundedSessionManager, SessionError, SessionSlot};
use crate::settings::SESSION_REAP_INTERVAL;
use rmcp::transport::streamable_http_server::session::SessionManager;

pub(super) struct Lifecycle {
    last_activity: Instant,
    active: usize,
    initializing: bool,
    closing: bool,
}

impl Default for Lifecycle {
    fn default() -> Self {
        Self {
            last_activity: Instant::now(),
            active: 0,
            initializing: true,
            closing: false,
        }
    }
}

pub(super) struct ActivityLease {
    slot: Arc<SessionSlot>,
    refresh: AtomicBool,
}

impl ActivityLease {
    pub(super) fn refresh(&self) {
        self.refresh.store(true, Ordering::Relaxed);
    }
}

impl Drop for ActivityLease {
    fn drop(&mut self) {
        let mut state = self
            .slot
            .lifecycle
            .lock()
            .unwrap_or_else(|p| p.into_inner());
        state.active -= 1;
        if self.refresh.load(Ordering::Relaxed) {
            state.last_activity = Instant::now();
        }
    }
}

impl SessionSlot {
    pub(super) fn activity(
        self: &Arc<Self>,
        refresh: bool,
    ) -> Result<Arc<ActivityLease>, SessionError> {
        let mut state = self.lifecycle.lock().unwrap_or_else(|p| p.into_inner());
        if state.closing {
            return Err(SessionError::Unavailable);
        }
        state.active += 1;
        Ok(Arc::new(ActivityLease {
            slot: self.clone(),
            refresh: AtomicBool::new(refresh),
        }))
    }

    pub(super) fn is_open(&self) -> bool {
        !self
            .lifecycle
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .closing
    }

    pub(super) fn initialized(&self) {
        let mut state = self.lifecycle.lock().unwrap_or_else(|p| p.into_inner());
        state.initializing = false;
        state.last_activity = Instant::now();
    }

    pub(super) fn mark_closing(&self) {
        self.lifecycle
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .closing = true;
    }

    fn expire(&self, now: Instant, timeout: Duration) -> bool {
        let mut state = self.lifecycle.lock().unwrap_or_else(|p| p.into_inner());
        if timeout.is_zero()
            || state.initializing
            || state.closing
            || state.active != 0
            || now.saturating_duration_since(state.last_activity) < timeout
        {
            return false;
        }
        state.closing = true;
        true
    }
}

impl BoundedSessionManager {
    pub fn spawn_idle_reaper(self: &Arc<Self>, timeout: Duration, cancel: CancellationToken) {
        if timeout.is_zero() {
            return;
        }
        let manager = Arc::downgrade(self);
        tokio::spawn(async move {
            let mut ticks = tokio::time::interval(timeout.min(SESSION_REAP_INTERVAL));
            ticks.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            loop {
                tokio::select! {
                    biased;
                    _ = cancel.cancelled() => break,
                    _ = ticks.tick() => {
                        let Some(manager) = manager.upgrade() else { break };
                        tokio::select! {
                            _ = cancel.cancelled() => break,
                            _ = manager.reap_idle_at(Instant::now(), timeout) => {}
                        }
                    }
                }
            }
        });
    }

    pub(super) async fn reap_idle_at(&self, now: Instant, timeout: Duration) -> usize {
        let slots: Vec<_> = self
            .registry
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .values()
            .filter_map(std::sync::Weak::upgrade)
            .collect();
        let mut closing = tokio::task::JoinSet::new();
        let mut expired = 0;
        for slot in slots {
            if slot.expire(now, timeout) {
                expired += 1;
                let inner = self.inner.clone();
                closing.spawn(async move {
                    // Remove the SDK lookup entry as well as stopping the worker.
                    // The transport retains the capacity permit through shutdown.
                    if let Err(error) = inner.close_session(&slot.id).await {
                        tracing::warn!(%error, "MCP idle session close failed");
                    }
                });
            }
        }
        while let Some(result) = closing.join_next().await {
            if let Err(error) = result {
                tracing::warn!(%error, "MCP idle session cleanup task failed");
            }
        }
        if expired != 0 {
            tracing::info!(expired, "MCP idle sessions closed");
        }
        expired
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, Weak};
    use tokio::sync::Semaphore;

    fn slot() -> Arc<SessionSlot> {
        let slot = Arc::new(SessionSlot {
            id: Arc::from("activity-test"),
            _permit: Arc::new(Semaphore::new(1)).try_acquire_owned().unwrap(),
            registry: Weak::new(),
            lifecycle: Mutex::default(),
        });
        slot.initialized();
        slot.lifecycle.lock().unwrap().last_activity = Instant::now() - Duration::from_secs(10);
        slot
    }

    #[test]
    fn lease_clones_hold_one_activity_until_final_completion() {
        let slot = slot();
        let activity = slot.activity(true).unwrap();
        let copy = activity.clone();
        let old = slot.lifecycle.lock().unwrap().last_activity;
        drop(activity);
        assert_eq!(slot.lifecycle.lock().unwrap().active, 1);
        assert_eq!(slot.lifecycle.lock().unwrap().last_activity, old);
        assert!(!slot.expire(
            Instant::now() + Duration::from_secs(60),
            Duration::from_secs(1)
        ));
        let finished = Instant::now();
        drop(copy);
        let state = slot.lifecycle.lock().unwrap();
        assert_eq!(state.active, 0);
        assert!(state.last_activity >= finished);
        drop(state);
        assert!(!slot.expire(Instant::now(), Duration::from_secs(1)));
    }

    #[test]
    fn failed_routing_does_not_extend_idle_deadline() {
        let slot = slot();
        let old = slot.lifecycle.lock().unwrap().last_activity;
        drop(slot.activity(false).unwrap());
        assert_eq!(slot.lifecycle.lock().unwrap().last_activity, old);
        assert!(slot.expire(Instant::now(), Duration::from_secs(5)));
        assert!(slot.activity(true).is_err());
    }
}
