//! A bounded pool of persistent snapshot executors sharing one engine owner.
use super::worker::{DatabaseJob, TransactionWorker};
use crate::config::database::READ_WORKER_ACQUIRE_TIMEOUT;
use crate::errors::{Error, Result};
use cozo::DbInstance;
use std::sync::{Condvar, Mutex};
use std::time::Instant;

struct State {
    idle: Vec<TransactionWorker>,
    live: usize,
}
pub(super) struct Readers {
    state: Mutex<State>,
    available: Condvar,
    capacity: usize,
}
pub(super) struct Lease<'a> {
    pool: &'a Readers,
    worker: Option<TransactionWorker>,
}

impl Readers {
    #[cfg(test)]
    pub fn stop_idle_worker(&self) {
        self.state
            .lock()
            .unwrap()
            .idle
            .last_mut()
            .unwrap()
            .shutdown();
    }

    #[cfg(test)]
    pub fn worker_ids(&self) -> Vec<std::thread::ThreadId> {
        self.state
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .idle
            .iter()
            .map(TransactionWorker::id)
            .collect()
    }
    pub fn new(capacity: usize) -> Self {
        Self {
            state: Mutex::new(State {
                idle: Vec::new(),
                live: 0,
            }),
            available: Condvar::new(),
            capacity,
        }
    }
    pub fn take(&self, raw: &DbInstance) -> Result<Lease<'_>> {
        let deadline = Instant::now() + READ_WORKER_ACQUIRE_TIMEOUT;
        let mut state = self.state.lock().unwrap_or_else(|p| p.into_inner());
        loop {
            if let Some(worker) = state.idle.pop() {
                if worker.is_finished() {
                    state.live -= 1;
                    drop(worker);
                    continue;
                }
                return Ok(Lease {
                    pool: self,
                    worker: Some(worker),
                });
            }
            if state.live < self.capacity {
                state.live += 1;
                drop(state);
                return match TransactionWorker::spawn(raw.clone()) {
                    Ok(worker) => Ok(Lease {
                        pool: self,
                        worker: Some(worker),
                    }),
                    Err(error) => {
                        self.state.lock().unwrap_or_else(|p| p.into_inner()).live -= 1;
                        self.available.notify_all();
                        Err(error.into())
                    }
                };
            }
            let Some(remaining) = deadline.checked_duration_since(Instant::now()) else {
                return Err(Error::Invalid(
                    "read worker capacity deadline exceeded".into(),
                ));
            };
            let (next, timed) = self
                .available
                .wait_timeout(state, remaining)
                .unwrap_or_else(|p| p.into_inner());
            state = next;
            if timed.timed_out() {
                return Err(Error::Invalid(
                    "read worker capacity deadline exceeded".into(),
                ));
            }
        }
    }
}
impl Lease<'_> {
    pub fn submit(&mut self, raw: &DbInstance, job: DatabaseJob) -> Result<()> {
        if let Err(job) = self.worker.as_ref().expect("leased worker").submit(job) {
            self.retire();
            self.worker = Some(TransactionWorker::spawn(raw.clone())?);
            self.worker.as_ref().unwrap().submit(job).map_err(|_| {
                Error::Substrate("read worker stopped before accepting work".into())
            })?;
        }
        Ok(())
    }
    pub fn retire(&mut self) {
        drop(self.worker.take());
    }
}
impl Drop for Lease<'_> {
    fn drop(&mut self) {
        let mut state = self.pool.state.lock().unwrap_or_else(|p| p.into_inner());
        match self.worker.take() {
            Some(worker) if !worker.is_finished() => state.idle.push(worker),
            other => {
                drop(other);
                state.live -= 1;
            }
        }
        drop(state);
        self.pool.available.notify_one();
    }
}

#[cfg(test)]
mod tests {
    use super::Readers;
    use cozo::DbInstance;
    #[test]
    fn saturation_is_bounded_and_a_released_slot_can_be_reused() {
        let raw = DbInstance::new("mem", "", "").unwrap();
        let pool = Readers::new(1);
        let first = pool.take(&raw).unwrap();
        assert!(pool.take(&raw).is_err());
        {
            let state = pool.state.lock().unwrap();
            assert_eq!(state.live, 1);
            assert!(state.idle.is_empty());
        }
        drop(first);
        let next = pool.take(&raw).unwrap();
        drop(next);
        assert_eq!(pool.worker_ids().len(), 1);
    }
}
