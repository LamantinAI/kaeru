//! Snapshot transactions share one engine while keeping caller state independent.

use std::collections::{BTreeMap, HashMap};
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, ThreadId};

use cozo::{
    DataValue, DbInstance, Error as CozoError, MultiTransaction, NamedRows, ScriptMutability,
};
use crossbeam_channel::{Receiver, bounded};

#[cfg(test)]
use crate::config::database::DEFAULT_READ_WORKERS;
use crate::errors::{Error, Result};

mod gate;
mod readers;
mod worker;
use gate::Gate;
use readers::{Lease, Readers};
use worker::{DatabaseJob, ScriptJob, TransactionJob, TransactionWorker};
#[cfg(test)]
mod tests;

struct ActiveTransaction {
    handle: MultiTransaction,
    failed: bool,
    wrote: bool,
    read_only: bool,
}

/// Read workers and the writer reuse one DbInstance. No worker opens a second
/// namespace owner, and cleanup is acknowledged before a slot becomes reusable.
pub(crate) struct Database {
    raw: DbInstance,
    gate: Gate,
    active: Mutex<HashMap<ThreadId, Arc<Mutex<ActiveTransaction>>>>,
    generation: AtomicU64,
    worker: Mutex<Option<TransactionWorker>>,
    readers: Readers,
    blocking_backend: bool,
}

impl Database {
    #[cfg(test)]
    pub(crate) fn new(raw: DbInstance) -> Self {
        Self::with_read_workers(raw, DEFAULT_READ_WORKERS)
    }

    pub(crate) fn with_read_workers(raw: DbInstance, count: usize) -> Self {
        #[cfg(feature = "storage-tikv")]
        let blocking_backend = matches!(&raw, DbInstance::TiKv(_));
        #[cfg(not(feature = "storage-tikv"))]
        let blocking_backend = false;
        Self {
            raw,
            blocking_backend,
            gate: Gate::default(),
            active: Mutex::new(HashMap::new()),
            generation: AtomicU64::new(0),
            worker: Mutex::new(None),
            readers: Readers::new(count),
        }
    }

    /// Bootstrap/migration access, before this store is published to callers.
    pub(crate) fn raw(&self) -> &DbInstance {
        &self.raw
    }

    fn current(&self) -> Option<Arc<Mutex<ActiveTransaction>>> {
        self.active
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .get(&thread::current().id())
            .cloned()
    }

    pub(crate) fn write_transaction_active(&self) -> bool {
        self.current()
            .is_some_and(|tx| !tx.lock().unwrap_or_else(|p| p.into_inner()).read_only)
    }
    pub(crate) fn exclusive_operation(&self) -> bool {
        self.gate.exclusive()
    }

    /// A lease can span assembly, delivery bookkeeping and cache publication.
    pub(crate) fn operation<T>(&self, write: bool, f: impl FnOnce() -> Result<T>) -> Result<T> {
        self.gate.run(write, f)
    }

    pub(crate) fn run_script(
        &self,
        script: &str,
        params: BTreeMap<String, DataValue>,
        mutability: ScriptMutability,
    ) -> std::result::Result<NamedRows, CozoError> {
        #[cfg(test)]
        super::test_support::record(script, &params, mutability);
        if let Some(tx) = self.current() {
            let mut tx = tx.lock().unwrap_or_else(|p| p.into_inner());
            if tx.read_only && matches!(mutability, ScriptMutability::Mutable) {
                return Err(CozoError::msg("writes are forbidden during read assembly"));
            }
            // Only prepared Core statements participate in MultiTransaction.
            let result = tx.handle.run_script(script, params);
            tx.wrote |= matches!(mutability, ScriptMutability::Mutable);
            tx.failed |= !tx.read_only && result.is_err();
            return result;
        }
        let write = matches!(mutability, ScriptMutability::Mutable);
        self.operation(write, || {
            let result = self
                .run_standalone(script, params, mutability)
                .map_err(Error::from);
            if write && result.is_ok() {
                self.generation.fetch_add(1, Ordering::Release);
            }
            result
        })
        .map_err(|e| CozoError::msg(e.to_string()))
    }

    fn run_standalone(
        &self,
        script: &str,
        params: BTreeMap<String, DataValue>,
        mutability: ScriptMutability,
    ) -> std::result::Result<NamedRows, CozoError> {
        if !self.blocking_backend {
            return self.raw.run_script(script, params, mutability);
        }
        let (result, incoming) = bounded(1);
        let job = DatabaseJob::Script(ScriptJob {
            script: script.into(),
            params,
            mutability,
            result,
        });
        let mut reader = if matches!(mutability, ScriptMutability::Immutable) {
            Some(
                self.readers
                    .take(&self.raw)
                    .map_err(|e| CozoError::msg(e.to_string()))?,
            )
        } else {
            None
        };
        self.submit(job, reader.as_mut())
            .map_err(|e| CozoError::msg(e.to_string()))?;
        incoming.recv().map_err(|_| {
            CozoError::msg("database worker stopped before delivering script result")
        })?
    }

    pub(crate) fn atomic<T>(&self, f: impl FnOnce() -> Result<T>) -> Result<T> {
        self.transaction(true, f)
    }
    pub(crate) fn read_snapshot<T>(&self, f: impl FnOnce() -> Result<T>) -> Result<T> {
        self.transaction(false, f)
    }

    fn transaction<T>(&self, write: bool, f: impl FnOnce() -> Result<T>) -> Result<T> {
        if let Some(tx) = self.current() {
            if write && tx.lock().unwrap_or_else(|p| p.into_inner()).read_only {
                return Err(Error::Invalid(
                    "write transaction inside read assembly".into(),
                ));
            }
            let result = f();
            if result.is_err() {
                let mut tx = tx.lock().unwrap_or_else(|p| p.into_inner());
                tx.failed |= !tx.read_only;
            }
            return result;
        }
        self.operation(write, || self.start_transaction(write, f))
    }

    fn start_transaction<T>(&self, write: bool, f: impl FnOnce() -> Result<T>) -> Result<T> {
        let mut reader = if write {
            None
        } else {
            Some(self.readers.take(&self.raw)?)
        };
        let (sender, commands) = bounded(1);
        let (results, receiver) = bounded(1);
        let (done, completed) = bounded(1);
        self.submit(
            DatabaseJob::Transaction(TransactionJob {
                write,
                commands,
                results,
                done,
            }),
            reader.as_mut(),
        )?;
        let owner = thread::current().id();
        self.active
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .insert(
                owner,
                Arc::new(Mutex::new(ActiveTransaction {
                    handle: MultiTransaction { sender, receiver },
                    failed: false,
                    wrote: false,
                    read_only: !write,
                })),
            );
        let outcome = catch_unwind(AssertUnwindSafe(f));
        let tx = self
            .active
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .remove(&owner)
            .expect("transaction owner retains its handle");
        let tx = Arc::try_unwrap(tx)
            .unwrap_or_else(|_| panic!("active transaction handle escaped its caller"));
        let tx = tx.into_inner().unwrap_or_else(|p| p.into_inner());
        let result = match outcome {
            Ok(Ok(value)) if !tx.failed => tx
                .handle
                .commit()
                .map(|()| {
                    if tx.wrote {
                        self.generation.fetch_add(1, Ordering::Release);
                    }
                    value
                })
                .map_err(Error::from),
            Ok(Ok(_)) => {
                let _ = tx.handle.abort();
                Err(Error::Invalid(
                    "atomic command contains a failed nested operation".into(),
                ))
            }
            Ok(Err(error)) => {
                let _ = tx.handle.abort();
                Err(error)
            }
            Err(panic) => {
                let _ = tx.handle.abort();
                drop(tx);
                self.complete_transaction(completed, reader.as_mut());
                resume_unwind(panic)
            }
        };
        drop(tx);
        self.complete_transaction(completed, reader.as_mut());
        result
    }

    fn submit(&self, job: DatabaseJob, reader: Option<&mut Lease<'_>>) -> Result<()> {
        if let Some(reader) = reader {
            return reader.submit(&self.raw, job);
        }
        let mut worker = self.worker.lock().unwrap_or_else(|p| p.into_inner());
        if worker.as_ref().is_some_and(TransactionWorker::is_finished) {
            drop(worker.take());
        }
        if worker.is_none() {
            *worker = Some(TransactionWorker::spawn(self.raw.clone())?);
        }
        if let Err(job) = worker.as_ref().unwrap().submit(job) {
            // Retry only unaccepted work; an accepted operation is never replayed.
            drop(worker.take());
            *worker = Some(TransactionWorker::spawn(self.raw.clone())?);
            worker
                .as_ref()
                .unwrap()
                .submit(job)
                .map_err(|_| Error::Substrate("writer stopped before accepting work".into()))?;
        }
        Ok(())
    }

    fn complete_transaction(&self, completed: Receiver<()>, reader: Option<&mut Lease<'_>>) {
        if completed.recv().is_err() {
            // Join unwind/cleanup before releasing the gate or leasing the worker.
            if let Some(reader) = reader {
                reader.retire();
            } else {
                drop(self.worker.lock().unwrap_or_else(|p| p.into_inner()).take());
            }
        }
    }

    pub(crate) fn generation(&self) -> u64 {
        self.generation.load(Ordering::Acquire)
    }
}
