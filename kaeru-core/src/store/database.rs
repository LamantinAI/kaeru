//! Transaction-aware execution shared by the existing curator primitives.

use std::collections::BTreeMap;
use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::{self, ThreadId};

use cozo::{
    DataValue, DbInstance, Error as CozoError, MultiTransaction, NamedRows, ScriptMutability,
};
use crossbeam_channel::{Receiver, bounded};

use crate::errors::{Error, Result};

mod worker;
use worker::{DatabaseJob, ScriptJob, TransactionJob, TransactionWorker};
#[cfg(test)]
mod tests;

struct ActiveTransaction {
    owner: ThreadId,
    handle: MultiTransaction,
    failed: bool,
    wrote: bool,
    read_only: bool,
}

/// Embedded standalone scripts retain direct execution; TiKV scripts also use
/// the blocking worker. During an atomic command or read assembly, its thread
/// shares one Cozo transaction and temporal
/// NOW. Read assemblies reject writes; atomic commands commit their primitives
/// together. Separate transactions reuse a per-database worker, retaining
/// RocksDB's thread-local state without occupying Cozo's Rayon compute pool.
pub(crate) struct Database {
    raw: DbInstance,
    writers: Mutex<()>,
    active: Mutex<Option<ActiveTransaction>>,
    generation: AtomicU64,
    worker: Mutex<Option<TransactionWorker>>,
    blocking_backend: bool,
}

impl Database {
    pub(crate) fn new(raw: DbInstance) -> Self {
        #[cfg(feature = "storage-tikv")]
        let blocking_backend = matches!(&raw, DbInstance::TiKv(_));
        #[cfg(not(feature = "storage-tikv"))]
        let blocking_backend = false;
        Self {
            blocking_backend,
            raw,
            writers: Mutex::new(()),
            active: Mutex::new(None),
            generation: AtomicU64::new(0),
            worker: Mutex::new(None),
        }
    }

    /// Bootstrap/migration access, before this store becomes available to callers.
    pub(crate) fn raw(&self) -> &DbInstance {
        &self.raw
    }

    pub(crate) fn run_script(
        &self,
        script: &str,
        params: BTreeMap<String, DataValue>,
        mutability: ScriptMutability,
    ) -> std::result::Result<NamedRows, cozo::Error> {
        #[cfg(test)]
        super::test_support::record(script, &params, mutability);
        {
            let mut active = self.active.lock().unwrap_or_else(|p| p.into_inner());
            if let Some(tx) = active.as_mut()
                && tx.owner == thread::current().id()
            {
                if tx.read_only && matches!(mutability, ScriptMutability::Mutable) {
                    return Err(CozoError::msg("writes are forbidden during read assembly"));
                }
                // Only prepared core scripts participate in a transaction. Cozo's
                // multi-transaction query API does not take a mutability flag.
                let result = tx.handle.run_script(script, params);
                tx.wrote |= matches!(mutability, ScriptMutability::Mutable);
                tx.failed |= !tx.read_only && result.is_err();
                return result;
            }
        }
        // Do not keep the active-state mutex across a foreign query: a storage
        // reader/writer may wait for commit, which must still obtain that mutex.
        if matches!(mutability, ScriptMutability::Mutable) {
            let _writer = self.writers.lock().unwrap_or_else(|p| p.into_inner());
            let result = self.run_standalone(script, params, mutability);
            if result.is_ok() {
                self.generation.fetch_add(1, Ordering::Release);
            }
            result
        } else {
            self.run_standalone(script, params, mutability)
        }
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
        self.submit_job(DatabaseJob::Script(ScriptJob {
            script: script.into(),
            params,
            mutability,
            result,
        }))
        .map_err(|error| CozoError::msg(error.to_string()))?;
        incoming.recv().map_err(|_| {
            CozoError::msg("database worker stopped before delivering script result")
        })?
    }

    pub(crate) fn atomic<T>(&self, f: impl FnOnce() -> Result<T>) -> Result<T> {
        self.transaction(true, f)
    }

    /// All queries in one read assembly share the database snapshot and Cozo's
    /// single temporal NOW. End this span before delivery bookkeeping writes.
    pub(crate) fn read_snapshot<T>(&self, f: impl FnOnce() -> Result<T>) -> Result<T> {
        self.transaction(false, f)
    }

    fn transaction<T>(&self, is_write: bool, f: impl FnOnce() -> Result<T>) -> Result<T> {
        {
            let active = self.active.lock().unwrap_or_else(|p| p.into_inner());
            if active
                .as_ref()
                .is_some_and(|tx| tx.owner == thread::current().id())
            {
                if is_write && active.as_ref().is_some_and(|tx| tx.read_only) {
                    return Err(Error::Invalid(
                        "write transaction inside read assembly".into(),
                    ));
                }
                drop(active);
                let result = f();
                if result.is_err()
                    && let Some(tx) = self
                        .active
                        .lock()
                        .unwrap_or_else(|p| p.into_inner())
                        .as_mut()
                {
                    tx.failed |= !tx.read_only;
                }
                return result;
            }
        }

        let _writer = self.writers.lock().unwrap_or_else(|p| p.into_inner());
        let (sender, commands) = bounded(1);
        let (results, receiver) = bounded(1);
        let (done, completed) = bounded(1);
        self.submit_transaction(TransactionJob {
            write: is_write,
            commands,
            results,
            done,
        })?;
        *self.active.lock().unwrap_or_else(|p| p.into_inner()) = Some(ActiveTransaction {
            owner: thread::current().id(),
            handle: MultiTransaction { sender, receiver },
            failed: false,
            wrote: false,
            read_only: !is_write,
        });

        let outcome = catch_unwind(AssertUnwindSafe(f));
        let tx = self
            .active
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .take()
            .expect("the transaction owner retains its active handle");
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
                self.complete_transaction(completed);
                resume_unwind(panic)
            }
        };
        drop(tx);
        self.complete_transaction(completed);
        result
    }

    fn submit_transaction(&self, job: TransactionJob) -> Result<()> {
        self.submit_job(DatabaseJob::Transaction(job))
    }

    fn submit_job(&self, job: DatabaseJob) -> Result<()> {
        let mut worker = self.worker.lock().unwrap_or_else(|p| p.into_inner());
        if worker.as_ref().is_some_and(TransactionWorker::is_finished) {
            drop(worker.take());
        }
        if worker.is_none() {
            *worker = Some(TransactionWorker::spawn(self.raw.clone())?);
        }
        if let Err(job) = worker.as_ref().unwrap().submit(job) {
            // The worker stopped before accepting this job. It has executed
            // neither a script nor caller code. Never retry accepted work.
            drop(worker.take());
            *worker = Some(TransactionWorker::spawn(self.raw.clone())?);
            worker.as_ref().unwrap().submit(job).map_err(|_| {
                Error::Substrate("transaction worker stopped before accepting work".into())
            })?;
        }
        Ok(())
    }

    fn complete_transaction(&self, completed: Receiver<()>) {
        if completed.recv().is_err() {
            // Join any worker unwind before releasing the writer guard. A
            // confirmed commit remains successful even if later cleanup failed.
            drop(self.worker.lock().unwrap_or_else(|p| p.into_inner()).take());
        }
    }

    pub(crate) fn generation(&self) -> u64 {
        self.generation.load(Ordering::Acquire)
    }
}
