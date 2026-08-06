//! Background hygiene for an embedded vault.
//!
//! The pass itself lives in `kaeru_core::hygiene` and takes only a `&Store`,
//! so it is adapter-agnostic; this module is the *when*, ported from the
//! daemon's scheduler (`kaeru-mcp/src/hygiene.rs`) with the properties that
//! matter kept intact:
//!
//! * **Non-blocking** — every pass runs on the blocking pool, so the tool call
//!   that triggered it returns immediately and no reactor thread is held.
//! * **Never twice at once** — an in-memory in-flight set guards the double
//!   start. Deliberately not persisted: a "running" flag surviving a crash as
//!   `true` would stop hygiene forever, a worse failure than the one it fixes.
//! * **Interruptible** — cancellation is checked between batches, so a
//!   shutdown stops at a boundary rather than tearing a batch in half.
//!
//! Two things differ, because this is a **library inside someone else's
//! process** rather than a daemon that owns its lifetime:
//!
//! 1. **The host owns the config.** No env var — hygiene is off until the host
//!    asks for it via [`KaeruMemory::with_hygiene`](crate::KaeruMemory::with_hygiene).
//!    kaeru never sweeps a vault its embedder didn't opt in.
//! 2. **The host owns the lifetime.** The sweeper runs on the host's tokio
//!    runtime and stops on [`KaeruMemory::shutdown_hygiene`](crate::KaeruMemory::shutdown_hygiene)
//!    (or when the process ends). A `KaeruMemory` is `Clone` and shared, so a
//!    dropped clone must *not* stop it — hence an explicit call rather than
//!    `Drop`.
//!
//! Single-writer caveat, inherited from the substrate: one `Arc<Store>` is one
//! RocksDB writer. If the same vault is also open in a `kaeru-mcp` daemon, that
//! daemon holds the lock and this process never opened the vault at all — so
//! there is no question of two schedulers fighting over one graph.

use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use kaeru_core::hygiene::{self, HygieneReport};
use kaeru_core::{EpisodeKind, Layer, Significance, Store};
use tokio_util::sync::CancellationToken;

/// Schedules hygiene passes for an embedded store. Cheap to clone — every
/// clone shares one in-flight set and one cancellation token.
#[derive(Clone)]
pub(crate) struct HygieneScheduler {
    store: Arc<Store>,
    /// Initiatives with a pass in flight. In memory on purpose — see module docs.
    in_flight: Arc<Mutex<HashSet<String>>>,
    cancel: CancellationToken,
    /// Off unless the host opted in. A library does not sweep uninvited.
    enabled: bool,
    /// Passes started since this memory was built — the assertion target for
    /// the double-start test, and surfaced by the `hygiene` tool.
    passes_started: Arc<AtomicUsize>,
}

impl HygieneScheduler {
    pub(crate) fn new(store: Arc<Store>, enabled: bool) -> Self {
        Self {
            store,
            in_flight: Arc::new(Mutex::new(HashSet::new())),
            cancel: CancellationToken::new(),
            enabled,
            passes_started: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub(crate) fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Considers a pass for `initiative` and returns immediately. Both the
    /// due-check and the pass run on the blocking pool, so a tool handler is
    /// never held up. Safe on every read and every write: `due` is a couple of
    /// indexed counts, and a pass already in flight makes this a no-op.
    pub(crate) fn consider(&self, initiative: Option<&str>) {
        if !self.enabled || self.cancel.is_cancelled() {
            return;
        }
        let Some(initiative) = initiative.map(str::to_string) else {
            // Nothing to scope a sweep to; the sweeper covers those on its own.
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
                    "hygiene pass finished"
                ),
                Ok(None) => {}
                Err(e) => {
                    tracing::warn!(initiative = %initiative, error = %e, "hygiene pass failed")
                }
            }
        });
    }

    /// Starts the periodic sweep on the host's runtime. Without it the "N days
    /// since the last pass" trigger is dead for any initiative the agent never
    /// touches — that condition is only evaluated when something touches it.
    ///
    /// Requires a tokio reactor; call it from inside the host's runtime.
    pub(crate) fn spawn_sweeper(&self) {
        if !self.enabled {
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
            "kaeru hygiene sweeper started (also triggered by writes and by awake)"
        );
    }

    /// Stops the sweeper and makes every further trigger a no-op. A pass
    /// already running stops at its next batch boundary.
    pub(crate) fn shutdown(&self) {
        self.cancel.cancel();
    }

    /// Runs a pass now, ignoring the due-check — the `hygiene force=true` path.
    /// Blocking; the caller must already be on the blocking pool.
    pub(crate) fn run_now(&self, initiative: &str) -> kaeru_core::Result<Option<HygieneReport>> {
        run_and_record(&self.store, initiative, &self.cancel, true)
    }

    pub(crate) fn passes_started(&self) -> usize {
        self.passes_started.load(Ordering::Relaxed)
    }
}

/// Runs one pass and writes the durable record of it: the detail lines go into
/// a `cold` episode, so "what did the sweep do on the 12th" survives long after
/// the one-line cue has been delivered.
fn run_and_record(
    store: &Arc<Store>,
    initiative: &str,
    cancel: &CancellationToken,
    force: bool,
) -> kaeru_core::Result<Option<HygieneReport>> {
    let pause_ms = store.config().hygiene_batch_pause_ms;
    let cancel_for_pause = cancel.clone();
    let pause = move || {
        if cancel_for_pause.is_cancelled() {
            return false;
        }
        // A real sleep between batches: `std::sync::Mutex` is not fair, and a
        // tight release→acquire loop can keep barging past a waiting caller.
        if pause_ms > 0 {
            std::thread::sleep(Duration::from_millis(pause_ms));
        }
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
            Layer::Cold,
        )?;
        kaeru_core::attach_node(s, &id, initiative)?;
        Ok(())
    })?;

    Ok(Some(report))
}

// ── the tool ────────────────────────────────────────────────────────────────

/// Arguments for `kaeru_hygiene`.
#[derive(Debug, serde::Deserialize)]
pub struct HygieneArgs {
    #[serde(default)]
    pub initiative: Option<String>,
    /// Run a pass now instead of reporting what one would do.
    #[serde(default)]
    pub force: Option<bool>,
}

async fn do_hygiene(mem: &crate::KaeruMemory, a: HygieneArgs) -> serde_json::Value {
    let Some(init) = a
        .initiative
        .clone()
        .or_else(|| mem.initiative().map(String::from))
    else {
        return serde_json::json!({ "error": "no initiative — scope the memory or pass `initiative`" });
    };

    if a.force.unwrap_or(false) {
        let scheduler = mem.hygiene_scheduler().clone();
        let for_run = init.clone();
        // `run_now` is blocking and takes the store guard per batch, so it must
        // not run on the reactor.
        let outcome = tokio::task::spawn_blocking(move || scheduler.run_now(&for_run))
            .await
            .unwrap_or_else(|e| Err(kaeru_core::Error::Invalid(format!("pass panicked: {e}"))));
        return match outcome {
            Ok(Some(report)) => serde_json::json!({
                "initiative": init,
                "ran": true,
                "summary": report.summary(),
                "archived": report.archived,
                "demoted": report.demoted,
                "promoted": report.promoted,
                "skipped": report.skipped,
                "moves": report.lines,
            }),
            Ok(None) => serde_json::json!({ "initiative": init, "ran": false }),
            Err(e) => serde_json::json!({ "error": e.to_string() }),
        };
    }

    let enabled = mem.hygiene_scheduler().is_enabled();
    let started = mem.hygiene_scheduler().passes_started();
    mem.blocking(move |s| {
        let state = match hygiene::state(s, &init) {
            Ok(v) => v,
            Err(e) => return serde_json::json!({ "error": e.to_string() }),
        };
        let nodes = hygiene::node_count(s, &init).unwrap_or(0);
        let core = hygiene::core_count(s, &init).unwrap_or(0);
        let due = hygiene::due(s, &init).ok().flatten();
        let candidates = hygiene::collect(s, &init).unwrap_or_default();
        serde_json::json!({
            "initiative": init,
            "enabled": enabled,
            "nodes": nodes,
            "core": core,
            "last_run_at": state.last_run_at,
            "nodes_at_last_run": state.nodes_at_last_run,
            "due": due,
            "passes_started": started,
            // What a pass WOULD move — read-only, nothing is applied here.
            "would_move": candidates.iter().map(|c| serde_json::json!({
                "action": c.action.as_str(),
                "name": c.name,
                "from": c.from.as_str(),
                "to": c.to.as_str(),
                "reason": c.reason,
            })).collect::<Vec<_>>(),
        })
    })
    .await
}

crate::mem_tool_cloud!(
    /// `kaeru_hygiene` — what the next hygiene pass would move, or run one now.
    Hygiene,
    "kaeru_hygiene",
    "Hygiene status for an initiative: node and core counts, when the last pass ran, whether one \
     is due, and exactly what the next pass would move — read-only, nothing is applied. Passes \
     only ever change a node's layer, reversibly. `force=true` runs one now.",
    HygieneArgs,
    { "type": "object", "properties": {
        "initiative": { "type": "string", "description": "initiative (default: the memory's own)" },
        "force": { "type": "boolean", "description": "run a pass now instead of only reporting" }
    } },
    |mem, a| do_hygiene(mem, a).await
);

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use kaeru_core::{EpisodeKind, KaeruConfig, Significance, Store};
    use rig::tool::Tool;

    use crate::KaeruMemory;

    fn args<T: serde::de::DeserializeOwned>(v: serde_json::Value) -> T {
        serde_json::from_value(v).expect("args")
    }

    /// A store whose triggers fire on a handful of writes, so a test doesn't
    /// have to manufacture 25 of them.
    fn eager_store() -> Arc<Store> {
        let mut cfg = KaeruConfig::defaults();
        cfg.hygiene_writes_trigger = 2;
        cfg.hygiene_core_trigger = 3;
        cfg.hygiene_promote_in_degree = 2;
        cfg.hygiene_batch_pause_ms = 0;
        Arc::new(Store::open_in_memory_with(cfg).expect("open"))
    }

    /// Gives the pass something real to move without any time travel: a node
    /// two others point at is promoted `warm` → `hot` once its in-degree
    /// reaches the threshold (2 in this config). The archive/demote rules need
    /// a node to be days old; this one fires immediately.
    fn promotable(store: &Store, init: &str) {
        store.scoped(Some(init), |s| {
            let mk = |name: &str| {
                kaeru_core::write_episode(
                    s,
                    EpisodeKind::Observation,
                    Significance::Low,
                    name,
                    name,
                )
                .expect("write")
            };
            let (a, b, hub) = (mk("ref-a"), mk("ref-b"), mk("hub"));
            for src in [&a, &b] {
                kaeru_core::link(s, src, &hub, kaeru_core::EdgeType::RefersTo).expect("link");
            }
        });
    }

    /// Hygiene is off unless the host opts in: the tool reports it, and no
    /// pass ever starts however much gets written.
    #[tokio::test]
    async fn hygiene_is_off_until_the_host_asks() {
        let store = eager_store();
        let mem = KaeruMemory::with_initiative(Arc::clone(&store), "proj");
        promotable(&store, "proj");

        // Several writes through the tool surface — enough to trip the trigger.
        for i in 0..3 {
            mem.remember()
                .call(args(
                    serde_json::json!({ "name": format!("n{i}"), "body": "x" }),
                ))
                .await
                .unwrap();
        }
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        let status = mem
            .hygiene()
            .call(args(serde_json::json!({})))
            .await
            .unwrap();
        assert_eq!(status["enabled"], false, "off by default; got {status}");
        assert_eq!(
            mem.hygiene_scheduler().passes_started(),
            0,
            "no pass ran uninvited"
        );
        // …but the dry run still reports what a pass *would* do.
        assert!(
            !status["would_move"].as_array().unwrap().is_empty(),
            "dry run still works with hygiene off: {status}"
        );
    }

    /// With hygiene on, writes trigger a pass and the cue reaches the agent on
    /// a later tool response — the only channel an embedded adapter has.
    #[tokio::test]
    async fn a_pass_delivers_its_cue_on_a_later_tool_call() {
        let store = eager_store();
        let mem = KaeruMemory::with_initiative(Arc::clone(&store), "proj").with_hygiene();
        promotable(&store, "proj");

        // The cue rides on whichever tool response comes first once the pass
        // has landed — including the very writes that triggered it, since those
        // go through the same hook.
        let mut cue = None;
        for i in 0..40 {
            let out = mem
                .remember()
                .call(args(
                    serde_json::json!({ "name": format!("n{i}"), "body": "x" }),
                ))
                .await
                .unwrap();
            if let Some(c) = out.get("memory_shifted").and_then(|v| v.as_str()) {
                cue = Some(c.to_string());
                break;
            }
            // The pass runs on the blocking pool; let it get there.
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
        let cue = cue.expect("the cue rode along on a tool response");
        assert!(cue.contains("shifted"), "a re-orientation cue: {cue}");
        assert!(cue.contains("awake proj"), "points at the next step: {cue}");
        assert!(mem.hygiene_scheduler().passes_started() >= 1);

        // Delivered once: the very next call does not repeat that same cue.
        let again = mem.awake().call(args(serde_json::json!({}))).await.unwrap();
        assert_ne!(
            again.get("memory_shifted").and_then(|v| v.as_str()),
            Some(cue.as_str()),
            "the cue is not redelivered: {again}"
        );

        mem.shutdown_hygiene();
    }

    /// `shutdown_hygiene` stops further passes — a host winding down doesn't
    /// leave a sweeper running against a store it's about to drop.
    #[tokio::test]
    async fn shutdown_stops_further_passes() {
        let store = eager_store();
        let mem = KaeruMemory::with_initiative(Arc::clone(&store), "proj").with_hygiene();
        mem.shutdown_hygiene();

        promotable(&store, "proj");
        for i in 0..3 {
            mem.remember()
                .call(args(
                    serde_json::json!({ "name": format!("n{i}"), "body": "x" }),
                ))
                .await
                .unwrap();
        }
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        assert_eq!(
            mem.hygiene_scheduler().passes_started(),
            0,
            "no pass after shutdown"
        );
    }
}
