//! Hygiene — the pass that keeps an initiative's memory from silting up.
//!
//! Memory rots in a specific way: the journal of "deployed this, fixed that"
//! accumulates in the layers that get injected into context, `core` grows
//! because nothing bounds it, and nodes that everything references sit in
//! `warm` where they are easy to miss. Cleaning that by hand is a weekly
//! chore, so it doesn't happen, so the graph lies.
//!
//! This module does it automatically, and stays safe by construction:
//!
//! * **It only moves layers.** No deletion, no rewriting of names or bodies,
//!   no merging, no touching `visibility` — the cloud fate of a node stays a
//!   human decision. Every action reverses with one `layer` call.
//! * **It never demotes `core` more than one step.** A misjudged `core` node
//!   lands in `hot`, still inside the window of recent nodes, not in an
//!   archive nobody opens.
//! * **It never promotes into `core`.** Growth of the uncapped, always-injected
//!   layer stays a deliberate act; the pass can only suggest by promoting to
//!   `hot`.
//! * **It skips pinned nodes.** A pin is a louder, more explicit "keep this in
//!   view" than any layer.
//! * **It re-checks before every write.** Candidates are collected once and
//!   applied in batches; an agent may have touched a node in between, and the
//!   agent's decision wins.
//!
//! Concurrency: [`apply`] takes and releases the store guard once per batch,
//! so a concurrent tool call waits for one batch, not for the whole pass. The
//! caller supplies the scoping (see `run_pass`), and the daemon adds a short
//! pause between batches because `std::sync::Mutex` is not fair.

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};

use cozo::{DataValue, ScriptMutability};

use crate::errors::Result;
use crate::graph::temporal::validity_seconds;
use crate::graph::{Layer, NodeId};
use crate::mutate::{now_validity_seconds, set_layer_as};
use crate::store::Store;

/// Audit actor stamped on every move the pass makes.
pub const HYGIENE_ACTOR: &str = "hygiene";

/// What the pass proposes to do with a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HygieneAction {
    /// Journal entry nobody references any more → `cold`.
    Archive,
    /// Untouched, unreferenced `core` node → one step down, to `hot`.
    DemoteFromCore,
    /// Node many live nodes point at → one step up (never into `core`).
    Promote,
}

impl HygieneAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            HygieneAction::Archive => "archive",
            HygieneAction::DemoteFromCore => "demote",
            HygieneAction::Promote => "promote",
        }
    }
}

/// A node the pass intends to move, with the evidence for it.
#[derive(Debug, Clone)]
pub struct HygieneCandidate {
    pub node_id: NodeId,
    pub name: String,
    pub action: HygieneAction,
    /// The layer observed at collection time — re-checked before writing.
    pub from: Layer,
    pub to: Layer,
    /// Human-readable justification, computed from the graph.
    pub reason: String,
}

/// The outcome of one pass.
#[derive(Debug, Clone, Default)]
pub struct HygieneReport {
    pub initiative: String,
    pub trigger: String,
    pub archived: usize,
    pub demoted: usize,
    pub promoted: usize,
    /// Candidates dropped because the node changed between collection and
    /// application — the agent's own decision took precedence.
    pub skipped: usize,
    pub core_before: usize,
    pub core_after: usize,
    /// The ceiling `core_after` is measured against, so the summary can say
    /// when the layer is still over it and why the pass could not help.
    pub core_ceiling: usize,
    /// One line per applied move, for the durable episode.
    pub lines: Vec<String>,
    /// The pass was asked to stop at a batch boundary (daemon shutting down).
    /// Not an error: the next pass re-derives whatever is left.
    pub stopped_early: bool,
}

impl HygieneReport {
    /// Total moves applied.
    pub fn applied(&self) -> usize {
        self.archived + self.demoted + self.promoted
    }

    /// The one-line cue that rides along on the next tool response.
    ///
    /// Deliberately **not** a work report. The agent never asked for the pass
    /// and doesn't know the nodes it touched, so a tally of
    /// archived/demoted/promoted is accounting it can't act on. What it needs
    /// to know is that the ground moved under an open session — and what to do
    /// about it. Same `↳` shape as the other in-result hints: state the
    /// consequence, point at the next step, keep the detail one call away
    /// (`hygiene <initiative>`, plus the durable episode).
    pub fn headline(&self) -> String {
        let n = self.applied();
        let nodes = if n == 1 { "node" } else { "nodes" };
        let init = &self.initiative;
        format!(
            "↳ memory shifted under you — {n} {nodes} re-layered in `{init}`.\n  \
             Re-run `awake {init}` if you're mid-session; `hygiene {init}` for what moved."
        )
    }

    /// The tally, for whoever **asked** for the pass (`hygiene force=true`).
    ///
    /// The counterpart to [`headline`](Self::headline): same pass, different
    /// reader. Someone who ran it on purpose wants to know what it did; an
    /// agent who didn't ask wants to know only that the ground moved.
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();
        if self.archived > 0 {
            parts.push(format!("{} archived", self.archived));
        }
        if self.demoted > 0 {
            parts.push(format!("core {} → {}", self.core_before, self.core_after));
        }
        if self.promoted > 0 {
            parts.push(format!("{} promoted", self.promoted));
        }
        if self.skipped > 0 {
            parts.push(format!("{} skipped (touched meanwhile)", self.skipped));
        }
        if parts.is_empty() {
            parts.push("nothing to do".to_string());
        }
        let mut line = format!(
            "[hygiene {}] {} · trigger: {}",
            self.initiative,
            parts.join(" · "),
            self.trigger
        );
        // A standing condition the pass could not clear must be said out loud.
        // "nothing to do" beside an oversized core reads as "core is fine",
        // and the only thing that gets it down is a hand the reader doesn't
        // know to lift (#75). Every node the pass may move it has already
        // moved by this point, so what is left is pinned — name that, and the
        // one verb that undoes it.
        if self.core_ceiling > 0 && self.core_after > self.core_ceiling {
            let over = self.core_after - self.core_ceiling;
            line.push_str(&format!(
                "\n  ↳ core still holds {} (ceiling {}); the {over} over are pinned, \
                 and a pin outranks the sweep. `unpin <name>` to release one, \
                 or `layer <name> warm` to move it yourself.",
                self.core_after, self.core_ceiling
            ));
        }
        line
    }
}

/// Bookkeeping row for an initiative: when the pass last ran and how many
/// nodes existed then.
#[derive(Debug, Clone, Default)]
pub struct HygieneState {
    pub last_run_at: f64,
    pub nodes_at_last_run: usize,
}

/// Reads the bookkeeping row, defaulting to "never ran".
pub fn state(store: &Store, initiative: &str) -> Result<HygieneState> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    let script = r#"
        ?[last_run_at, nodes_at_last_run] :=
            *initiative_hygiene{initiative, last_run_at, nodes_at_last_run},
            initiative = $init
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    Ok(rows
        .rows
        .first()
        .map(|r| HygieneState {
            last_run_at: r.first().and_then(|v| v.get_float()).unwrap_or(0.0),
            nodes_at_last_run: r.get(1).and_then(|v| v.get_int()).unwrap_or(0).max(0) as usize,
        })
        .unwrap_or_default())
}

/// Counts nodes belonging to `initiative`, excluding audit events (they are
/// bookkeeping, not knowledge, and would swamp the write-count trigger).
pub fn node_count(store: &Store, initiative: &str) -> Result<usize> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    let script = r#"
        ?[id] := *node{id, type @ 'NOW'}, type != 'audit_event',
                 *node_initiative{initiative, node_id: id}, initiative = $init
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    Ok(rows.rows.len())
}

/// Counts live `core` nodes in `initiative`.
pub fn core_count(store: &Store, initiative: &str) -> Result<usize> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    let script = r#"
        ?[id] := *node{id, layer, type @ 'NOW'}, layer = 'core', type != 'audit_event',
                 *node_initiative{initiative, node_id: id}, initiative = $init
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    Ok(rows.rows.len())
}

/// Decides whether a pass is due, and says why. Any one of three conditions
/// fires it: enough writes since the last pass, a `core` layer grown past its
/// threshold, or simply enough time.
///
/// Read-only — safe to call on every `awake` and on every write.
pub fn due(store: &Store, initiative: &str) -> Result<Option<String>> {
    let cfg = store.config();
    let state = state(store, initiative)?;
    let nodes = node_count(store, initiative)?;

    let written = nodes.saturating_sub(state.nodes_at_last_run);
    if written >= cfg.hygiene_writes_trigger {
        return Ok(Some(format!("{written} writes since the last pass")));
    }

    // `>` rather than `>=`: the threshold is the ceiling the pass brings core
    // back to, so a core sitting exactly on it is at its target, not over it.
    // With `>=` a satisfied pass still met its own trigger and re-fired on
    // every write forever (#75).
    let core = core_count(store, initiative)?;
    if core > cfg.hygiene_core_trigger {
        return Ok(Some(format!(
            "core holds {core} nodes (threshold {})",
            cfg.hygiene_core_trigger
        )));
    }

    // A never-run initiative with too little in it to trip either threshold
    // is not "stale" — leave it alone until it has something to clean.
    if state.last_run_at > 0.0 {
        let age = now_validity_seconds() as f64 - state.last_run_at;
        if age >= cfg.hygiene_stale_after_secs as f64 {
            return Ok(Some(format!(
                "{} days since the last pass",
                (age / 86_400.0) as u64
            )));
        }
    }

    Ok(None)
}

/// One row of the node scan: everything the rules need, read once.
struct Scanned {
    id: NodeId,
    name: String,
    node_type: String,
    layer: Layer,
    ts: f64,
    in_degree: usize,
    pinned: bool,
}

/// Reads every non-audit node of `initiative` with its layer, last assertion
/// timestamp, inbound-edge count and pin state.
///
/// Three plain queries joined in Rust rather than one clever rule: CozoScript
/// has no way to fold "count if any, else zero" into a single scan, and the
/// graphs this runs on are thousands of nodes, not millions.
fn scan(store: &Store, initiative: &str) -> Result<Vec<Scanned>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    let nodes = store.db_ref().run_script(
        r#"
        ?[id, name, type, layer, validity] :=
            *node{id, name, type, layer, validity @ 'NOW'},
            type != 'audit_event',
            *node_initiative{initiative, node_id: id}, initiative = $init
        "#,
        params,
        ScriptMutability::Immutable,
    )?;

    // Inbound edges from nodes that are themselves live at NOW: a reference
    // held by a retracted node is not evidence that anyone still needs this.
    let inbound = store.db_ref().run_script(
        r#"
        live[src] := *node{id: src @ 'NOW'}
        ?[dst, count(src)] := *edge{src, dst, edge_type @ 'NOW'}, live[src]
        "#,
        BTreeMap::new(),
        ScriptMutability::Immutable,
    )?;
    let mut in_degree: BTreeMap<String, usize> = BTreeMap::new();
    for r in &inbound.rows {
        if let (Some(dst), Some(n)) = (
            r.first().and_then(|v| v.get_str()),
            r.get(1).and_then(|v| v.get_int()),
        ) {
            in_degree.insert(dst.to_string(), n.max(0) as usize);
        }
    }

    let pins = store.db_ref().run_script(
        "?[node_id] := *session_pin{node_id}",
        BTreeMap::new(),
        ScriptMutability::Immutable,
    )?;
    let pinned: std::collections::HashSet<String> = pins
        .rows
        .iter()
        .filter_map(|r| r.first().and_then(|v| v.get_str()).map(String::from))
        .collect();

    let mut out = Vec::with_capacity(nodes.rows.len());
    for r in &nodes.rows {
        let id = match r.first().and_then(|v| v.get_str()) {
            Some(s) => s.to_string(),
            None => continue,
        };
        let layer = r
            .get(3)
            .and_then(|v| v.get_str())
            .and_then(|s| s.parse::<Layer>().ok())
            .unwrap_or(Layer::Warm);
        out.push(Scanned {
            name: r
                .get(1)
                .and_then(|v| v.get_str())
                .unwrap_or_default()
                .to_string(),
            node_type: r
                .get(2)
                .and_then(|v| v.get_str())
                .unwrap_or_default()
                .to_string(),
            layer,
            ts: validity_seconds(r.get(4)).unwrap_or(0.0),
            in_degree: in_degree.get(&id).copied().unwrap_or(0),
            pinned: pinned.contains(&id),
            id,
        });
    }
    Ok(out)
}

/// Classifies the initiative's nodes into the moves the pass would make.
/// Pure with respect to the graph — collects, never writes.
pub fn collect(store: &Store, initiative: &str) -> Result<Vec<HygieneCandidate>> {
    let cfg = store.config();
    let now = now_validity_seconds() as f64;
    let journal_cutoff = now - cfg.hygiene_journal_age_secs as f64;
    let core_cutoff = now - cfg.hygiene_core_review_age_secs as f64;

    let nodes = scan(store, initiative)?;
    let core_total = nodes.iter().filter(|n| n.layer == Layer::Core).count();

    let mut out = Vec::new();
    for node in &nodes {
        // A pin outranks every rule below: the user put it in the window on
        // purpose.
        if node.pinned {
            continue;
        }
        let age_days = ((now - node.ts) / 86_400.0).max(0.0) as u64;

        match node.layer {
            // `core` is the expensive layer: injected whole, every session.
            // Demote only what nothing references and nobody has touched.
            Layer::Core => {
                if node.ts < core_cutoff && node.in_degree == 0 {
                    out.push(HygieneCandidate {
                        node_id: node.id.clone(),
                        name: node.name.clone(),
                        action: HygieneAction::DemoteFromCore,
                        from: Layer::Core,
                        to: Layer::Hot,
                        reason: format!("untouched {age_days}d, no inbound references"),
                    });
                }
            }
            // The journal: episodes that were interesting the day they were
            // written. Archive once they are old AND unreferenced.
            Layer::Hot | Layer::Warm => {
                if node.node_type == "episode" && node.ts < journal_cutoff && node.in_degree == 0 {
                    out.push(HygieneCandidate {
                        node_id: node.id.clone(),
                        name: node.name.clone(),
                        action: HygieneAction::Archive,
                        from: node.layer,
                        to: Layer::Cold,
                        reason: format!("journal entry, {age_days}d old, no inbound references"),
                    });
                } else if node.layer == Layer::Warm
                    && node.in_degree >= cfg.hygiene_promote_in_degree
                {
                    // Referenced from everywhere but sitting in the default
                    // layer — raise it one step. Never into `core`.
                    out.push(HygieneCandidate {
                        node_id: node.id.clone(),
                        name: node.name.clone(),
                        action: HygieneAction::Promote,
                        from: Layer::Warm,
                        to: Layer::Hot,
                        reason: format!("{} live nodes reference it", node.in_degree),
                    });
                }
            }
            // Already archived: leave alone. Recovering something from `cold`
            // is a judgement call, not a rule.
            Layer::Cold | Layer::Frozen => {}
        }
    }

    // The overflow rule. The age-and-unreferenced rule above keeps core tidy,
    // but it cannot keep it *small*: a core of recent, well-referenced nodes
    // satisfies neither half of it, so the layer grew past its threshold and
    // stayed there while every pass reported "nothing to do" (#75). The
    // threshold started a pass that had no rule able to change the condition
    // that started it.
    //
    // So when core is over the ceiling, demote back down to it — least
    // referenced first, oldest first among equals. Every safety property of
    // the rule above still holds: one step down (to `hot`, which is still the
    // working set), never into the archive, pins untouchable, and the move is
    // reversible with a single `layer <name> core`.
    let already: HashSet<NodeId> = out
        .iter()
        .filter(|c| c.action == HygieneAction::DemoteFromCore)
        .map(|c| c.node_id.clone())
        .collect();
    let over = core_total.saturating_sub(cfg.hygiene_core_trigger);
    if over > already.len() {
        let mut spare: Vec<&Scanned> = nodes
            .iter()
            .filter(|n| n.layer == Layer::Core && !n.pinned && !already.contains(&n.id))
            .collect();
        // Least referenced first; among equals, the one touched longest ago.
        spare.sort_by(|a, b| {
            a.in_degree
                .cmp(&b.in_degree)
                .then(a.ts.partial_cmp(&b.ts).unwrap_or(Ordering::Equal))
        });
        for node in spare.into_iter().take(over - already.len()) {
            let age_days = ((now - node.ts) / 86_400.0).max(0.0) as u64;
            out.push(HygieneCandidate {
                node_id: node.id.clone(),
                name: node.name.clone(),
                action: HygieneAction::DemoteFromCore,
                from: Layer::Core,
                to: Layer::Hot,
                reason: format!(
                    "core over its ceiling ({core_total} of {}) — least referenced ({} inbound),                      untouched {age_days}d",
                    cfg.hygiene_core_trigger, node.in_degree
                ),
            });
        }
    }

    Ok(out)
}

/// Applies `candidates` in batches, re-checking each node against the layer
/// it had at collection time. A node whose layer changed meanwhile is skipped
/// — an agent touched it, and an agent's decision outranks the sweep.
///
/// The caller owns batching policy: this function performs one batch per call
/// so the daemon can release the store guard and pause in between. Returns
/// how many candidates were consumed.
pub fn apply_batch(
    store: &Store,
    batch: &[HygieneCandidate],
    report: &mut HygieneReport,
) -> Result<()> {
    for candidate in batch {
        let current = match crate::mutate::read_node_now(store, &candidate.node_id)? {
            Some(node) => node,
            // Retracted between collection and application — skip rather than
            // resurrect it through `set_layer`'s historical fallback.
            None => {
                report.skipped += 1;
                continue;
            }
        };
        let current_layer = current.layer.parse::<Layer>().unwrap_or(Layer::Warm);
        if current_layer != candidate.from {
            report.skipped += 1;
            continue;
        }

        set_layer_as(store, &candidate.node_id, candidate.to, HYGIENE_ACTOR)?;
        match candidate.action {
            HygieneAction::Archive => report.archived += 1,
            HygieneAction::DemoteFromCore => report.demoted += 1,
            HygieneAction::Promote => report.promoted += 1,
        }
        report.lines.push(format!(
            "{} {} → {}: {} ({})",
            candidate.action.as_str(),
            candidate.from.as_str(),
            candidate.to.as_str(),
            candidate.name,
            candidate.reason
        ));
    }
    Ok(())
}

/// Records that a pass ran: stamps the time, the node count it saw, and the
/// headline waiting for delivery.
pub fn record_run(
    store: &Store,
    initiative: &str,
    nodes_seen: usize,
    pending_report: Option<&str>,
) -> Result<()> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    params.insert(
        "at".to_string(),
        DataValue::from(now_validity_seconds() as f64),
    );
    params.insert("nodes".to_string(), DataValue::from(nodes_seen as i64));
    params.insert(
        "report".to_string(),
        match pending_report {
            Some(text) => DataValue::Str(text.into()),
            None => DataValue::Null,
        },
    );
    let script = r#"
        ?[initiative, last_run_at, nodes_at_last_run, pending_report] <-
            [[$init, $at, $nodes, $report]]
        :put initiative_hygiene {initiative => last_run_at, nodes_at_last_run, pending_report}
    "#;
    store
        .db_ref()
        .run_script(script, params, ScriptMutability::Mutable)?;
    Ok(())
}

/// Reads the pending cue **without** clearing it, so an empty pass can carry an
/// undelivered one forward instead of overwriting it.
fn read_pending_report(store: &Store, initiative: &str) -> Result<Option<String>> {
    let mut read = BTreeMap::new();
    read.insert("init".to_string(), DataValue::Str(initiative.into()));
    let script = r#"
        ?[pending_report] := *initiative_hygiene{initiative, pending_report},
            initiative = $init
    "#;
    let rows = store
        .db_ref()
        .run_script(script, read, ScriptMutability::Immutable)?;
    Ok(rows
        .rows
        .first()
        .and_then(|r| r.first())
        .and_then(|v| v.get_str())
        .map(String::from))
}

/// Takes the pending headline for `initiative`, clearing it. Called on the
/// next tool response — the one delivery channel that reliably reaches both
/// Claude Code and Codex (MCP notifications are received but not surfaced by
/// either client).
pub fn take_pending_report(store: &Store, initiative: &str) -> Result<Option<String>> {
    let pending = read_pending_report(store, initiative)?;

    if pending.is_some() {
        // Clear ONLY the report. Reading it is not a pass: rewriting
        // `last_run_at` here would push the "N days since the last pass"
        // trigger forward every time an agent picked up a headline.
        let state = state(store, initiative)?;
        let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
        params.insert("init".to_string(), DataValue::Str(initiative.into()));
        params.insert("at".to_string(), DataValue::from(state.last_run_at));
        params.insert(
            "nodes".to_string(),
            DataValue::from(state.nodes_at_last_run as i64),
        );
        let clear = r#"
            ?[initiative, last_run_at, nodes_at_last_run, pending_report] <-
                [[$init, $at, $nodes, null]]
            :put initiative_hygiene {initiative => last_run_at, nodes_at_last_run, pending_report}
        "#;
        store
            .db_ref()
            .run_script(clear, params, ScriptMutability::Mutable)?;
    }
    Ok(pending)
}

/// Runs a full pass for `initiative` when one is [`due`]: collect, apply in
/// batches of `hygiene_batch_size`, record the result.
///
/// ⚠️ **Call this OUTSIDE `Store::scoped`.** The pass takes the store guard
/// itself — once for the collection phase, then once per batch — so that a
/// concurrent tool call waits for one batch rather than for the whole sweep.
/// The guard is a plain `Mutex` and not reentrant: calling this from inside
/// an already-scoped closure (the MCP adapter's `with_initiative`) would
/// deadlock. The daemon's background task is the intended caller.
///
/// `pause` runs between batches, outside the guard, and decides whether to
/// continue: returning `false` stops the pass cleanly at a batch boundary
/// (the daemon wires it to its cancellation token, so a shutdown never tears
/// a batch in half). The daemon also sleeps there, because
/// `std::sync::Mutex` is not fair and a tight release→acquire loop can keep
/// barging ahead of a waiting client thread. Tests pass `|| true`.
///
/// A pass stopped early is not a problem: candidates are predicates over the
/// current graph, not a stored to-do list, so the next pass simply re-derives
/// whatever is left.
///
/// Returns `None` when no pass was due.
pub fn run_pass(
    store: &Store,
    initiative: &str,
    pause: impl FnMut() -> bool,
) -> Result<Option<HygieneReport>> {
    run_pass_inner(store, initiative, pause, false)
}

/// [`run_pass`] without the due-check — the `hygiene force=true` path. Used
/// when a human asks for a sweep now rather than waiting for a trigger.
/// Everything else is identical, including the batching and the guard
/// discipline, so this must also be called outside `Store::scoped`.
pub fn force_pass(
    store: &Store,
    initiative: &str,
    pause: impl FnMut() -> bool,
) -> Result<Option<HygieneReport>> {
    run_pass_inner(store, initiative, pause, true)
}

fn run_pass_inner(
    store: &Store,
    initiative: &str,
    mut pause: impl FnMut() -> bool,
    force: bool,
) -> Result<Option<HygieneReport>> {
    let Some((trigger, candidates, core_before)) =
        store.scoped(Some(initiative), |s| -> Result<_> {
            let trigger = match due(s, initiative)? {
                Some(reason) => reason,
                None if force => "forced".to_string(),
                None => return Ok(None),
            };
            Ok(Some((
                trigger,
                collect(s, initiative)?,
                core_count(s, initiative)?,
            )))
        })?
    else {
        return Ok(None);
    };

    let mut report = HygieneReport {
        initiative: initiative.to_string(),
        trigger,
        core_before,
        core_ceiling: store.config().hygiene_core_trigger,
        ..Default::default()
    };

    let batch_size = store.config().hygiene_batch_size.max(1);
    for (index, batch) in candidates.chunks(batch_size).enumerate() {
        if index > 0 && !pause() {
            report.stopped_early = true;
            break;
        }
        // One short critical section per batch: this is the whole reason the
        // pass is batched at all.
        store.scoped(Some(initiative), |s| apply_batch(s, batch, &mut report))?;
    }

    store.scoped(Some(initiative), |s| -> Result<()> {
        report.core_after = core_count(s, initiative)?;
        let nodes_seen = node_count(s, initiative)?;
        // A pass that moved nothing has nothing to announce — and must not
        // clear a cue an earlier pass left that the agent hasn't collected yet.
        let cue = if report.applied() > 0 {
            Some(report.headline())
        } else {
            read_pending_report(s, initiative)?
        };
        record_run(s, initiative, nodes_seen, cue.as_deref())
    })?;
    Ok(Some(report))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{EdgeType, Significance};
    use crate::mutate::{get_layer, link, set_layer};
    use crate::{EpisodeKind, attach_node, write_episode};

    /// Writes an episode into `init` and returns its id.
    fn episode_in(store: &Store, init: &str, name: &str) -> NodeId {
        let id = write_episode(
            store,
            EpisodeKind::Observation,
            Significance::Medium,
            name,
            "body of the note",
        )
        .expect("write");
        attach_node(store, &id, init).expect("attach");
        id
    }

    /// Backdates a node so the age rules can be tested without sleeping for
    /// two weeks. Moves the row to an older validity key: asserting an older
    /// version alongside the current one would change nothing, since `@ 'NOW'`
    /// resolves the newest assertion.
    fn backdate(store: &Store, id: &NodeId, days: u64) {
        let secs = now_validity_seconds() - days * 86_400;
        let mut read: BTreeMap<String, DataValue> = BTreeMap::new();
        read.insert("id".to_string(), DataValue::Str(id.clone().into()));
        let current = store
            .db_ref()
            .run_script(
                r#"
                ?[validity, type, tier, name, body, tags, initiatives, properties, visibility, layer] :=
                    *node{id, validity, type, tier, name, body, tags, initiatives, properties, visibility, layer @ 'NOW'},
                    id = $id
                "#,
                read,
                ScriptMutability::Immutable,
            )
            .expect("read current row");
        let row = current.rows.first().expect("node exists").clone();

        let mut rm: BTreeMap<String, DataValue> = BTreeMap::new();
        rm.insert("id".to_string(), DataValue::Str(id.clone().into()));
        rm.insert("v".to_string(), row[0].clone());
        store
            .db_ref()
            .run_script(
                "?[id, validity] <- [[$id, $v]] :rm node {id, validity}",
                rm,
                ScriptMutability::Mutable,
            )
            .expect("drop the current key");

        let mut put: BTreeMap<String, DataValue> = BTreeMap::new();
        put.insert("id".to_string(), DataValue::Str(id.clone().into()));
        put.insert(
            "validity".to_string(),
            DataValue::List(vec![DataValue::from(secs as f64), DataValue::Bool(true)]),
        );
        for (index, column) in crate::mutate::NODE_VALUE_COLUMNS.iter().enumerate() {
            put.insert((*column).to_string(), row[index + 1].clone());
        }
        let cols = crate::mutate::NODE_VALUE_COLUMNS.join(", ");
        let placeholders = crate::mutate::NODE_VALUE_COLUMNS
            .iter()
            .map(|c| format!("${c}"))
            .collect::<Vec<_>>()
            .join(", ");
        store
            .db_ref()
            .run_script(
                &format!(
                    r#"
                    ?[id, validity, {cols}] <- [[$id, $validity, {placeholders}]]
                    :put node {{id, validity => {cols}}}
                    "#
                ),
                put,
                ScriptMutability::Mutable,
            )
            .expect("reinsert at the older validity");
    }

    /// A configuration that fires on small fixtures.
    fn eager_store() -> Store {
        let mut cfg = crate::config::KaeruConfig::defaults();
        cfg.hygiene_writes_trigger = 2;
        cfg.hygiene_core_trigger = 3;
        cfg.hygiene_promote_in_degree = 2;
        Store::open_in_memory_with(cfg).expect("open")
    }

    #[test]
    fn archives_old_unreferenced_journal_entries() {
        let store = eager_store();
        let old = episode_in(&store, "proj", "deployed-something");
        let fresh = episode_in(&store, "proj", "deployed-today");
        backdate(&store, &old, 30);

        let report = run_pass(&store, "proj", || true)
            .expect("pass")
            .expect("pass was due");

        assert_eq!(report.archived, 1, "only the old entry was archived");
        assert_eq!(get_layer(&store, &old).expect("layer"), Layer::Cold);
        assert_eq!(
            get_layer(&store, &fresh).expect("layer"),
            Layer::Warm,
            "today's entry stays where it is"
        );
    }

    #[test]
    fn referenced_journal_entries_are_left_alone() {
        let store = eager_store();
        let referenced = episode_in(&store, "proj", "the-lesson");
        let pointer = episode_in(&store, "proj", "entrypoint");
        backdate(&store, &referenced, 30);
        link(&store, &pointer, &referenced, EdgeType::RefersTo).expect("link");

        let report = run_pass(&store, "proj", || true)
            .expect("pass")
            .expect("due");

        assert_eq!(report.archived, 0, "something references it");
        assert_eq!(get_layer(&store, &referenced).expect("layer"), Layer::Warm);
    }

    #[test]
    fn core_is_demoted_one_step_never_archived() {
        let store = eager_store();
        let stale = episode_in(&store, "proj", "stale-core-note");
        set_layer(&store, &stale, Layer::Core).expect("core");
        backdate(&store, &stale, 60);
        // Two more nodes so the write trigger fires.
        episode_in(&store, "proj", "filler-a");
        episode_in(&store, "proj", "filler-b");

        let report = run_pass(&store, "proj", || true)
            .expect("pass")
            .expect("due");

        assert_eq!(report.demoted, 1);
        assert_eq!(
            get_layer(&store, &stale).expect("layer"),
            Layer::Hot,
            "core drops one step, to hot — never straight to the archive"
        );
    }

    #[test]
    fn heavily_referenced_nodes_are_promoted_but_never_into_core() {
        let store = eager_store();
        let hub = episode_in(&store, "proj", "the-map");
        for i in 0..3 {
            let referrer = episode_in(&store, "proj", &format!("note-{i}"));
            link(&store, &referrer, &hub, EdgeType::RefersTo).expect("link");
        }

        let report = run_pass(&store, "proj", || true)
            .expect("pass")
            .expect("due");

        assert_eq!(report.promoted, 1);
        assert_eq!(
            get_layer(&store, &hub).expect("layer"),
            Layer::Hot,
            "promotion stops at hot; growing core stays a deliberate act"
        );
    }

    #[test]
    fn pinned_nodes_are_never_touched() {
        let store = eager_store();
        let pinned = episode_in(&store, "proj", "pinned-old-note");
        backdate(&store, &pinned, 30);
        crate::session::pin(&store, &pinned, "keep in view").expect("pin");
        episode_in(&store, "proj", "filler");

        let report = run_pass(&store, "proj", || true)
            .expect("pass")
            .expect("due");

        assert_eq!(report.archived, 0);
        assert_eq!(
            get_layer(&store, &pinned).expect("layer"),
            Layer::Warm,
            "a pin outranks every hygiene rule"
        );
    }

    /// The race the design is built around: a node is collected as a
    /// candidate, an agent moves it, and the batch must then leave it alone.
    #[test]
    fn a_node_touched_after_collection_is_skipped() {
        let store = eager_store();
        let node = episode_in(&store, "proj", "about-to-be-rescued");
        backdate(&store, &node, 30);
        episode_in(&store, "proj", "filler");

        let candidates = collect(&store, "proj").expect("collect");
        assert_eq!(candidates.len(), 1, "the old note is a candidate");

        // The agent decides it matters, between collection and application.
        set_layer(&store, &node, Layer::Core).expect("agent promotes it");

        let mut report = HygieneReport::default();
        apply_batch(&store, &candidates, &mut report).expect("apply");

        assert_eq!(report.archived, 0);
        assert_eq!(report.skipped, 1, "the sweep stood down");
        assert_eq!(
            get_layer(&store, &node).expect("layer"),
            Layer::Core,
            "the agent's decision survived the sweep"
        );
    }

    #[test]
    fn a_second_pass_over_a_clean_graph_does_nothing() {
        let store = eager_store();
        let old = episode_in(&store, "proj", "old-note");
        backdate(&store, &old, 30);
        episode_in(&store, "proj", "filler");

        let first = run_pass(&store, "proj", || true)
            .expect("pass")
            .expect("due");
        assert_eq!(first.archived, 1);

        // Nothing new was written, so the pass is not due again...
        assert!(due(&store, "proj").expect("due").is_none());

        // ...and even when forced, it finds nothing to move.
        let candidates = collect(&store, "proj").expect("collect");
        assert!(
            candidates.is_empty(),
            "an already-swept graph yields no candidates: {candidates:?}"
        );
    }

    #[test]
    fn trigger_fires_on_writes_then_resets() {
        let store = eager_store();
        assert!(
            due(&store, "proj").expect("due").is_none(),
            "an empty initiative is not due"
        );

        episode_in(&store, "proj", "one");
        episode_in(&store, "proj", "two");
        let reason = due(&store, "proj")
            .expect("due")
            .expect("two writes fire it");
        assert!(
            reason.contains("writes"),
            "reason names the trigger: {reason}"
        );

        run_pass(&store, "proj", || true)
            .expect("pass")
            .expect("due");
        assert!(
            due(&store, "proj").expect("due").is_none(),
            "the pass reset the counter"
        );
    }

    #[test]
    fn report_is_delivered_once_then_cleared() {
        let store = eager_store();
        let old = episode_in(&store, "proj", "old-note");
        backdate(&store, &old, 30);
        episode_in(&store, "proj", "filler");
        run_pass(&store, "proj", || true)
            .expect("pass")
            .expect("due");

        let first = take_pending_report(&store, "proj").expect("take");
        let cue = first.as_deref().expect("a cue is waiting");
        // A re-orientation cue, not a work report: it states the consequence
        // and points at the next step. A tally of archived/demoted/promoted is
        // accounting the agent never asked for and can't act on.
        assert!(cue.contains("shifted"), "states the consequence: {cue}");
        assert!(cue.contains("awake proj"), "points at the next step: {cue}");
        assert!(
            !cue.contains("archived") && !cue.contains("demoted"),
            "no bookkeeping in the cue: {cue}"
        );
        assert_eq!(
            take_pending_report(&store, "proj").expect("take again"),
            None,
            "delivered once, then cleared"
        );
    }

    /// A pass that moves nothing must not clear a cue an earlier pass left —
    /// the sweep timer fires every few hours, and would otherwise swallow the
    /// notice before the agent ever came back to collect it.
    #[test]
    fn an_empty_pass_does_not_swallow_an_undelivered_cue() {
        let store = eager_store();
        let old = episode_in(&store, "proj", "old-note");
        backdate(&store, &old, 30);
        episode_in(&store, "proj", "filler");
        run_pass(&store, "proj", || true)
            .expect("pass")
            .expect("due");

        // A second pass over the now-clean graph — forced, since the trigger
        // reset, which is exactly what the sweep timer or `hygiene force=true`
        // does. It finds nothing to move.
        let second = force_pass(&store, "proj", || true)
            .expect("pass")
            .expect("forced");
        assert_eq!(second.applied(), 0, "nothing left to do");

        let cue = take_pending_report(&store, "proj").expect("take");
        // Specifically the FIRST pass's cue (it moved one node) — not the empty
        // pass's own "0 nodes", which would mean it overwrote the notice.
        assert!(
            cue.as_deref().is_some_and(|s| s.contains("1 node ")),
            "the first pass's cue survived the empty one: {cue:?}"
        );
    }

    /// The promise the batching exists for: a sweep in progress must not stall
    /// an agent writing to the same store. The writer thread runs for the
    /// whole duration of the pass and must get through several writes — if the
    /// pass held the guard end to end, it would land zero until the sweep
    /// finished.
    #[test]
    fn a_pass_in_progress_does_not_stall_concurrent_writers() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        use std::sync::{Arc, Barrier};

        let mut cfg = crate::config::KaeruConfig::defaults();
        cfg.hygiene_writes_trigger = 2;
        cfg.hygiene_batch_size = 5; // 40 candidates → 8 batches
        let store = Arc::new(Store::open_in_memory_with(cfg).expect("open"));

        for i in 0..40 {
            let id = episode_in(&store, "proj", &format!("old-note-{i}"));
            backdate(&store, &id, 30);
        }

        let writes = Arc::new(AtomicUsize::new(0));
        let barrier = Arc::new(Barrier::new(2));
        let sweeper = {
            let store = Arc::clone(&store);
            let barrier = Arc::clone(&barrier);
            std::thread::spawn(move || {
                barrier.wait();
                run_pass(&store, "proj", || {
                    std::thread::sleep(std::time::Duration::from_millis(4));
                    true
                })
                .expect("pass")
                .expect("due")
            })
        };

        barrier.wait();

        // Write from this thread while the sweep runs in the other one.
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(500);
        while std::time::Instant::now() < deadline {
            store.scoped(Some("proj"), |s| {
                crate::jot(s, "concurrent write").expect("jot");
            });
            writes.fetch_add(1, Ordering::Relaxed);
            if sweeper.is_finished() {
                break;
            }
        }
        let report = sweeper.join().expect("sweeper finished");

        assert_eq!(report.archived, 40, "the sweep did its work");
        assert!(
            writes.load(Ordering::Relaxed) >= 3,
            "the writer got through {} writes while the sweep ran — the guard \
             is released between batches",
            writes.load(Ordering::Relaxed)
        );
    }

    /// `force` means force. Found by driving the daemon over MCP: the tool
    /// advertised "runs one now", but the forced path went through the same
    /// due-check and answered "nothing was due".
    #[test]
    fn a_forced_pass_runs_even_when_nothing_is_due() {
        let store = eager_store();
        let old = episode_in(&store, "proj", "old-note");
        backdate(&store, &old, 30);
        episode_in(&store, "proj", "filler");

        run_pass(&store, "proj", || true)
            .expect("pass")
            .expect("due");
        assert!(due(&store, "proj").expect("due").is_none());

        let another = episode_in(&store, "proj", "another-old-note");
        backdate(&store, &another, 30);
        assert!(
            due(&store, "proj").expect("due").is_none(),
            "one write is below the trigger"
        );

        let report = force_pass(&store, "proj", || true)
            .expect("forced")
            .expect("a forced pass always runs");
        assert_eq!(report.trigger, "forced");
        assert_eq!(report.archived, 1);
        assert_eq!(get_layer(&store, &another).expect("layer"), Layer::Cold);
    }

    /// Reading the headline is not a pass. An earlier version stamped
    /// `last_run_at` while clearing the report, which pushed the "N days
    /// since the last pass" trigger forward every time an agent picked one up.
    #[test]
    fn taking_the_report_does_not_move_the_last_run_stamp() {
        let store = eager_store();
        let old = episode_in(&store, "proj", "old-note");
        backdate(&store, &old, 30);
        episode_in(&store, "proj", "filler");
        run_pass(&store, "proj", || true)
            .expect("pass")
            .expect("due");

        let before = state(&store, "proj").expect("state");
        assert!(before.last_run_at > 0.0);

        take_pending_report(&store, "proj")
            .expect("take")
            .expect("a headline was waiting");

        let after = state(&store, "proj").expect("state");
        assert_eq!(
            after.last_run_at, before.last_run_at,
            "the stamp survived the read"
        );
        assert_eq!(after.nodes_at_last_run, before.nodes_at_last_run);
    }

    #[test]
    fn hygiene_moves_are_attributed_in_the_audit_trail() {
        let store = eager_store();
        let old = episode_in(&store, "proj", "old-note");
        backdate(&store, &old, 30);
        episode_in(&store, "proj", "filler");
        run_pass(&store, "proj", || true)
            .expect("pass")
            .expect("due");

        let rows = store
            .db_ref()
            .run_script(
                r#"?[id, properties] := *node{id, type, properties @ 'NOW'}, type = 'audit_event'"#,
                BTreeMap::new(),
                ScriptMutability::Immutable,
            )
            .expect("audit");
        let has_hygiene_actor = rows
            .rows
            .iter()
            .any(|r| format!("{:?}", r.get(1)).contains(HYGIENE_ACTOR));
        assert!(
            has_hygiene_actor,
            "the sweep's moves are separable from an agent's own layer calls"
        );
    }

    /// The reported case: a core of recent, well-referenced nodes. Neither
    /// half of the age-and-unreferenced rule applies to any of them, so before
    /// the overflow rule the pass reported "nothing to do · trigger: core
    /// holds N nodes" — naming an oversized core as its reason and then doing
    /// nothing about it, forever, on every write.
    #[test]
    fn an_oversized_core_of_fresh_referenced_nodes_is_brought_down() {
        let store = eager_store(); // ceiling 3
        let mut ids = Vec::new();
        for i in 0..6 {
            let id = episode_in(&store, "proj", &format!("core-note-{i}"));
            set_layer(&store, &id, Layer::Core).expect("core");
            ids.push(id);
        }
        // Every one of them referenced, so the existing rule cannot fire.
        let anchor = episode_in(&store, "proj", "the-thing-they-describe");
        for id in &ids {
            link(&store, &anchor, id, EdgeType::DerivedFrom).expect("link");
        }

        let report = run_pass(&store, "proj", || true)
            .expect("pass")
            .expect("pass was due");

        assert_eq!(report.core_after, 3, "core came down to its ceiling");
        assert_eq!(report.demoted, 3);
        assert!(
            !report.summary().contains("nothing to do"),
            "the pass reports what it did: {}",
            report.summary()
        );
        // And a second pass is no longer due — the trigger it satisfied does
        // not re-fire on every write from here on.
        assert!(
            due(&store, "proj").expect("due").is_none()
                || !due(&store, "proj")
                    .expect("due")
                    .unwrap()
                    .contains("core holds"),
            "the core trigger is satisfied"
        );
    }

    /// A demoted node lands in `hot`, still in the working set, and one
    /// `layer <name> core` away from where it was.
    #[test]
    fn overflow_demotion_is_one_reversible_step() {
        let store = eager_store();
        let mut ids = Vec::new();
        for i in 0..5 {
            let id = episode_in(&store, "proj", &format!("core-note-{i}"));
            set_layer(&store, &id, Layer::Core).expect("core");
            ids.push(id);
        }

        run_pass(&store, "proj", || true)
            .expect("pass")
            .expect("due");

        let demoted: Vec<_> = ids
            .iter()
            .filter(|id| get_layer(&store, id).expect("layer") == Layer::Hot)
            .collect();
        assert_eq!(demoted.len(), 2, "exactly the overflow, and only to hot");
        for id in &ids {
            let l = get_layer(&store, id).expect("layer");
            assert!(
                l == Layer::Core || l == Layer::Hot,
                "never further than one step: {l:?}"
            );
        }
    }

    /// A pin outranks the sweep — including the overflow rule. What the pass
    /// must not do is stay silent about the core it therefore cannot shrink.
    #[test]
    fn a_core_held_up_by_pins_is_reported_not_hidden() {
        let store = eager_store();
        for i in 0..5 {
            let id = episode_in(&store, "proj", &format!("pinned-core-{i}"));
            set_layer(&store, &id, Layer::Core).expect("core");
            crate::session::pin(&store, &id, "held open on purpose").expect("pin");
        }

        let report = run_pass(&store, "proj", || true)
            .expect("pass")
            .expect("pass was due");

        assert_eq!(report.demoted, 0, "pins are untouchable");
        assert_eq!(report.core_after, 5);
        let summary = report.summary();
        assert!(
            summary.contains("core still holds 5") && summary.contains("pinned"),
            "the standing condition is named, not hidden behind 'nothing to do': {summary}"
        );
        assert!(
            summary.contains("unpin") || summary.contains("layer"),
            "and the summary says what to do about it: {summary}"
        );
    }

    /// Least referenced first, oldest first among equals — the pass gives up
    /// the least useful core node it can, not an arbitrary one.
    #[test]
    fn overflow_demotes_the_least_referenced_first() {
        let store = eager_store();
        let keep = episode_in(&store, "proj", "the-load-bearing-one");
        let drop = episode_in(&store, "proj", "the-orphan");
        let mid = episode_in(&store, "proj", "the-middle");
        let spare = episode_in(&store, "proj", "the-other-middle");
        for id in [&keep, &drop, &mid, &spare] {
            set_layer(&store, id, Layer::Core).expect("core");
        }
        let a = episode_in(&store, "proj", "ref-a");
        let b = episode_in(&store, "proj", "ref-b");
        link(&store, &a, &keep, EdgeType::DerivedFrom).expect("link");
        link(&store, &b, &keep, EdgeType::DerivedFrom).expect("link");
        link(&store, &a, &mid, EdgeType::DerivedFrom).expect("link");
        link(&store, &a, &spare, EdgeType::DerivedFrom).expect("link");

        run_pass(&store, "proj", || true)
            .expect("pass")
            .expect("due");

        assert_eq!(
            get_layer(&store, &drop).expect("layer"),
            Layer::Hot,
            "the unreferenced one goes first"
        );
        assert_eq!(
            get_layer(&store, &keep).expect("layer"),
            Layer::Core,
            "the most referenced one stays"
        );
    }
}
