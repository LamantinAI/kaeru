//! One reusable database executor, outside Cozo's Rayon pool and async runtimes.

use std::{
    collections::BTreeMap,
    thread::{self, JoinHandle},
};

use cozo::{DataValue, DbInstance, Error, NamedRows, ScriptMutability, TransactionPayload};
use crossbeam_channel::{Receiver, Sender, bounded};

pub(super) struct TransactionJob {
    pub write: bool,
    pub commands: Receiver<TransactionPayload>,
    pub results: Sender<Result<NamedRows, Error>>,
    pub done: Sender<()>,
}

pub(super) struct ScriptJob {
    pub script: String,
    pub params: BTreeMap<String, DataValue>,
    pub mutability: ScriptMutability,
    pub result: Sender<Result<NamedRows, Error>>,
}

pub(super) enum DatabaseJob {
    Transaction(TransactionJob),
    Script(ScriptJob),
}

pub(super) struct TransactionWorker {
    jobs: Option<Sender<DatabaseJob>>,
    thread: Option<JoinHandle<()>>,
}

impl TransactionWorker {
    pub fn spawn(db: DbInstance) -> std::io::Result<Self> {
        let (jobs, incoming) = bounded::<DatabaseJob>(1);
        let thread = thread::Builder::new()
            .name("kaeru-transaction".into())
            .spawn(move || {
                while let Ok(job) = incoming.recv() {
                    match job {
                        DatabaseJob::Transaction(job) => {
                            db.run_multi_transaction(job.write, job.commands, job.results);
                            // Commit/abort replies precede full transaction cleanup.
                            // Acknowledge only after snapshots and storage locks drop.
                            let _ = job.done.send(());
                        }
                        DatabaseJob::Script(job) => {
                            let result = db.run_script(&job.script, job.params, job.mutability);
                            let _ = job.result.send(result);
                        }
                    }
                }
            })?;
        Ok(Self {
            jobs: Some(jobs),
            thread: Some(thread),
        })
    }

    pub fn is_finished(&self) -> bool {
        self.thread.as_ref().is_none_or(JoinHandle::is_finished)
    }

    pub fn submit(&self, job: DatabaseJob) -> Result<(), DatabaseJob> {
        match &self.jobs {
            Some(jobs) => jobs.send(job).map_err(|error| error.0),
            None => Err(job),
        }
    }

    pub fn shutdown(&mut self) {
        // Disconnect before joining; the idle worker may be waiting on recv.
        drop(self.jobs.take());
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }

    #[cfg(test)]
    pub fn id(&self) -> thread::ThreadId {
        self.thread.as_ref().unwrap().thread().id()
    }
}

impl Drop for TransactionWorker {
    fn drop(&mut self) {
        self.shutdown();
    }
}
