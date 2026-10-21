//! Host-owned cooperative read cancellation. Never kills a shared Cozo query.

use std::cell::RefCell;
use std::sync::Arc;

use super::{ApiResult, Issue};

/// A cheap, thread-safe cancellation probe supplied by the host adapter.
/// A running substrate statement finishes before the next checkpoint. Once a
/// mutation or delivery bookkeeping starts, it runs to completion.
#[derive(Clone, Default)]
pub struct ExecutionControl {
    probe: Option<Arc<dyn Fn() -> bool + Send + Sync>>,
}

impl ExecutionControl {
    pub fn new(probe: impl Fn() -> bool + Send + Sync + 'static) -> Self {
        Self {
            probe: Some(Arc::new(probe)),
        }
    }

    pub fn is_cancelled(&self) -> bool {
        self.probe.as_ref().is_some_and(|probe| probe())
    }
}

thread_local! {
    static CONTROLS: RefCell<Vec<ExecutionControl>> = const { RefCell::new(Vec::new()) };
}

pub(super) struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        CONTROLS.with_borrow_mut(|controls| {
            controls.pop();
        });
    }
}

pub(super) fn enter(control: ExecutionControl) -> Guard {
    CONTROLS.with_borrow_mut(|controls| controls.push(control));
    Guard
}

pub(super) fn suspend() -> Guard {
    enter(ExecutionControl::default())
}

pub(super) fn checkpoint() -> ApiResult<()> {
    // Do not call a host callback while the TLS stack is borrowed.
    let control = CONTROLS.with_borrow(|controls| controls.last().cloned());
    if control.is_some_and(|control| control.is_cancelled()) {
        Err(Issue::new("CANCELLED", "request was cancelled"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests;
