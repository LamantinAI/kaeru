//! Background hygiene: where the sweep actually runs.
//!
//! The pass itself lives in `kaeru_core::hygiene`; this is the part that
//! decides *when* and keeps it out of everybody's way.
//!
//! Why inside the daemon rather than a cron job or a sidecar: the substrate is
//! a single-writer RocksDB, and this process holds the lock (see the module
//! docs in `main.rs`). A second process opening the same vault loses the LOCK
//! race — the exact reason this daemon exists. An external scheduler can still
//! drive it, but only by asking the daemon, not by opening the vault itself.
//!
//! Three properties this module is responsible for:
//!
//! * **Non-blocking.** Every pass runs on the blocking pool via
//!   `spawn_blocking`, so it never occupies a reactor thread; the tool call
//!   that triggered it returns immediately. `run_pass` takes the store guard
//!   per batch, and the pause between batches happens here — with a real
//!   sleep, because `std::sync::Mutex` is not fair and a tight
//!   release→acquire loop can keep barging past a waiting client thread.
//! * **Never twice at once.** An in-flight set of initiative names guards
//!   against a double start. It is deliberately in memory rather than in the
//!   store: a persisted "running" flag would survive a crash as `true`
//!   forever, and hygiene would never run again — a worse failure than the one
//!   it fixes, and one that would need its own staleness timeout to undo.
//!   One daemon owns the vault, so a fresh process starts with an empty set by
//!   construction.
//! * **Interruptible.** The cancellation token is checked between batches, so
//!   a shutdown stops the sweep cleanly instead of tearing a batch in half.

use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use kaeru_core::hygiene::{self, HygieneReport};
use kaeru_core::{EpisodeKind, Significance, Store};
use tokio_util::sync::CancellationToken;

/// Schedules hygiene passes for a store shared by every MCP session.
#[derive(Clone)]
pub struct HygieneScheduler {
    store: Arc<Store>,
    /// Initiatives with a pass in flight. In memory on purpose — see the
    /// module docs.
    in_flight: Arc<Mutex<HashSet<String>>>,
    cancel: CancellationToken,
    /// Set from `KAERU_MCP_HYGIENE_ENABLE`; when false, every trigger is a
    /// no-op. Opt-in: a vault gets its first sweep only once its owner has
    /// asked for one, so an upgrade never re-layers a live graph unannounced.
    enabled: bool,
    /// Passes actually started since the daemon came up. Surfaced by the
    /// `hygiene` tool, and the assertion target for the double-start test.
    passes_started: Arc<AtomicUsize>,
}

impl HygieneScheduler {
    pub fn new(store: Arc<Store>, cancel: CancellationToken, enabled: bool) -> Self {
        Self {
            store,
            in_flight: Arc::new(Mutex::new(HashSet::new())),
            cancel,
            enabled,
            passes_started: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Considers a pass for `initiative` and returns immediately. The decision
    /// (`hygiene::due`) and the pass itself both happen on the blocking pool,
    /// so the caller — a tool handler — is never held up.
    ///
    /// Safe to call on every read and every write: `due` is a couple of
    /// indexed counts, and a pass already in flight makes this a no-op.
    pub fn consider(&self, initiative: Option<&str>) {
        if !self.enabled || self.cancel.is_cancelled() {
            return;
        }
        let Some(initiative) = initiative.map(str::to_string) else {
            // Cross-initiative calls have no scope to sweep; the sweep timer
            // covers those initiatives on its own schedule.
            return;
        };

        let store = Arc::clone(&self.store);
        let in_flight = Arc::clone(&self.in_flight);
        let cancel = self.cancel.clone();
        let passes_started = Arc::clone(&self.passes_started);
        tokio::task::spawn_blocking(move || {
            match hygiene::due(&store, &initiative) {
                Ok(Some(_)) => {}
                Ok(None) => return,
                Err(e) => {
                    tracing::debug!(initiative = %initiative, error = %e, "hygiene: due-check failed");
                    return;
                }
            }

            // Claim the initiative, or bail if another pass owns it.
            {
                let mut guard = in_flight
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                if !guard.insert(initiative.clone()) {
                    return;
                }
            }
            passes_started.fetch_add(1, Ordering::Relaxed);

            let outcome = run_and_record(&store, &initiative, &cancel, false);

            in_flight
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .remove(&initiative);

            match outcome {
                Ok(Some(report)) => tracing::info!(
                    initiative = %initiative,
                    archived = report.archived,
                    demoted = report.demoted,
                    promoted = report.promoted,
                    skipped = report.skipped,
                    stopped_early = report.stopped_early,
                    "hygiene pass finished"
                ),
                Ok(None) => {}
                Err(e) => {
                    tracing::warn!(initiative = %initiative, error = %e, "hygiene pass failed")
                }
            }
        });
    }

    /// Starts the periodic sweep. Without it the "N days since the last pass"
    /// trigger is dead for any initiative nobody opens: that condition is only
    /// ever evaluated when something touches the initiative.
    pub fn spawn_sweeper(&self) {
        if !self.enabled {
            tracing::info!("hygiene off (set KAERU_MCP_HYGIENE_ENABLE=1 to turn it on)");
            return;
        }
        let interval_secs = self.store.config().hygiene_sweep_interval_secs;
        let scheduler = self.clone();
        let cancel = self.cancel.clone();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(interval_secs.max(60)));
            // The first tick fires immediately; skip it so startup stays quiet.
            ticker.tick().await;
            loop {
                tokio::select! {
                    _ = cancel.cancelled() => break,
                    _ = ticker.tick() => {
                        let store = Arc::clone(&scheduler.store);
                        let names = tokio::task::spawn_blocking(move || {
                            kaeru_core::list_initiatives(&store).unwrap_or_default()
                        })
                        .await
                        .unwrap_or_default();
                        for name in names {
                            scheduler.consider(Some(&name));
                        }
                    }
                }
            }
        });
        tracing::info!(
            interval_secs,
            "hygiene sweeper started (also triggered by writes and by awake)"
        );
    }

    /// Runs a pass now, ignoring the due-check — the `hygiene` tool's
    /// `force` path. Blocking; the caller is already on the blocking pool.
    pub fn run_now(&self, initiative: &str) -> kaeru_core::Result<Option<HygieneReport>> {
        run_and_record(&self.store, initiative, &self.cancel, true)
    }

    pub fn is_disabled(&self) -> bool {
        !self.enabled
    }

    /// Passes started since this daemon came up.
    pub fn passes_started(&self) -> usize {
        self.passes_started.load(Ordering::Relaxed)
    }
}

/// Runs one pass and writes the durable record of it: the detail lines go
/// into a `cold` episode so "what did the sweep do on the 12th" survives long
/// after the one-line headline has been delivered.
fn run_and_record(
    store: &Arc<Store>,
    initiative: &str,
    cancel: &CancellationToken,
    force: bool,
) -> kaeru_core::Result<Option<HygieneReport>> {
    let pause_ms = store.config().hygiene_batch_pause_ms;
    let cancel = cancel.clone();
    let pause = move || {
        if cancel.is_cancelled() {
            return false;
        }
        // Yield the (unfair) store guard to whoever is waiting for it.
        std::thread::sleep(Duration::from_millis(pause_ms));
        true
    };
    let report = if force {
        hygiene::force_pass(store, initiative, pause)?
    } else {
        hygiene::run_pass(store, initiative, pause)?
    };

    let Some(report) = report else {
        return Ok(None);
    };
    if report.applied() == 0 {
        return Ok(Some(report));
    }

    // The episode is written inside a scope so it lands in the right
    // initiative; `run_pass` has already released the guard by now.
    let body = format!(
        "{}\n\n{}",
        report.summary(),
        report
            .lines
            .iter()
            .map(|l| format!("• {l}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let name = format!("hygiene-{initiative}-{stamp}");
    let store_for_write = Arc::clone(store);
    store_for_write.scoped(Some(initiative), |s| -> kaeru_core::Result<()> {
        let id = kaeru_core::write_episode_with_layer(
            s,
            EpisodeKind::Action,
            Significance::Low,
            &name,
            &body,
            kaeru_core::Layer::Cold,
        )?;
        kaeru_core::attach_node(s, &id, initiative)?;
        Ok(())
    })?;

    Ok(Some(report))
}

#[cfg(test)]
mod tests {
    use kaeru_core::{EpisodeKind, KaeruConfig, Significance, Store, write_episode};

    use super::*;

    /// A store whose thresholds fire on a handful of nodes.
    fn eager_store() -> Arc<Store> {
        let mut cfg = KaeruConfig::defaults();
        cfg.hygiene_writes_trigger = 2;
        cfg.hygiene_batch_pause_ms = 0;
        Arc::new(Store::open_in_memory_with(cfg).expect("open"))
    }

    fn seed(store: &Store, initiative: &str, count: usize) {
        for i in 0..count {
            let id = write_episode(
                store,
                EpisodeKind::Observation,
                Significance::Low,
                &format!("note-{i}"),
                "body",
            )
            .expect("write");
            kaeru_core::attach_node(store, &id, initiative).expect("attach");
        }
    }

    /// Counts the pass-record episodes the scheduler leaves behind.
    fn pass_records(store: &Store) -> usize {
        kaeru_core::nodes_in_initiative(store, "proj")
            .map(|nodes| {
                nodes
                    .iter()
                    .filter(|b| b.name.starts_with("hygiene-"))
                    .count()
            })
            .unwrap_or(0)
    }

    /// Several triggers arriving at once must produce ONE pass. The in-flight
    /// set is the only thing standing between "every write nudges the
    /// scheduler" and a pile-up of concurrent sweeps over the same graph.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn concurrent_triggers_start_a_single_pass() {
        let store = eager_store();
        seed(&store, "proj", 6);

        let scheduler = HygieneScheduler::new(Arc::clone(&store), CancellationToken::new(), true);
        for _ in 0..8 {
            scheduler.consider(Some("proj"));
        }

        // Let the blocking pool drain.
        tokio::time::sleep(Duration::from_millis(400)).await;

        assert_eq!(
            scheduler.passes_started(),
            1,
            "eight simultaneous triggers must start exactly one pass: the \
             in-flight set claims the initiative, and the pass that wins \
             resets the trigger for the rest"
        );
        assert!(
            pass_records(&store) <= 1,
            "at most one pass record was written"
        );
    }

    /// The disable switch is absolute: no pass, no record, whatever happens.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn hygiene_off_by_default_never_runs() {
        let store = eager_store();
        seed(&store, "proj", 6);

        let scheduler = HygieneScheduler::new(Arc::clone(&store), CancellationToken::new(), false);
        scheduler.consider(Some("proj"));
        tokio::time::sleep(Duration::from_millis(200)).await;

        assert_eq!(
            scheduler.passes_started(),
            0,
            "an opt-out scheduler stayed idle"
        );
        assert_eq!(pass_records(&store), 0);
        assert!(scheduler.is_disabled());
    }

    /// A cross-initiative call has no scope to sweep and must not panic or
    /// spawn anything.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn an_unscoped_call_is_ignored() {
        let store = eager_store();
        seed(&store, "proj", 6);

        let scheduler = HygieneScheduler::new(Arc::clone(&store), CancellationToken::new(), true);
        scheduler.consider(None);
        tokio::time::sleep(Duration::from_millis(200)).await;

        assert_eq!(scheduler.passes_started(), 0);
        assert_eq!(pass_records(&store), 0);
    }
}
