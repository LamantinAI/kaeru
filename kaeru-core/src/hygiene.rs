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

use std::collections::BTreeMap;

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
    /// One line per applied move, for the durable episode.
    pub lines: Vec<String>,
}

impl HygieneReport {
    /// Total moves applied.
    pub fn applied(&self) -> usize {
        self.archived + self.demoted + self.promoted
    }

    /// The one-line summary that rides along on the next tool response.
    pub fn headline(&self) -> String {
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
        format!(
            "[hygiene {}] {} · trigger: {}",
            self.initiative,
            parts.join(" · "),
            self.trigger
        )
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

    let core = core_count(store, initiative)?;
    if core >= cfg.hygiene_core_trigger {
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

    let mut out = Vec::new();
    for node in scan(store, initiative)? {
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
                        node_id: node.id,
                        name: node.name,
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
                        node_id: node.id,
                        name: node.name,
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
                        node_id: node.id,
                        name: node.name,
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

/// Takes the pending headline for `initiative`, clearing it. Called on the
/// next tool response — the one delivery channel that reliably reaches both
/// Claude Code and Codex (MCP notifications are received but not surfaced by
/// either client).
pub fn take_pending_report(store: &Store, initiative: &str) -> Result<Option<String>> {
    let mut read = BTreeMap::new();
    read.insert("init".to_string(), DataValue::Str(initiative.into()));
    let script = r#"
        ?[pending_report] := *initiative_hygiene{initiative, pending_report},
            initiative = $init
    "#;
    let rows = store
        .db_ref()
        .run_script(script, read, ScriptMutability::Immutable)?;
    let pending = rows
        .rows
        .first()
        .and_then(|r| r.first())
        .and_then(|v| v.get_str())
        .map(String::from);

    if pending.is_some() {
        let state = state(store, initiative)?;
        record_run(store, initiative, state.nodes_at_last_run, None)?;
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
/// `pause` runs between batches, outside the guard. The daemon passes a real
/// sleep there: `std::sync::Mutex` is not fair, so a tight release→acquire
/// loop can keep barging ahead of a waiting client thread. Tests pass a no-op.
///
/// Returns `None` when no pass was due.
pub fn run_pass(
    store: &Store,
    initiative: &str,
    mut pause: impl FnMut(),
) -> Result<Option<HygieneReport>> {
    let Some((trigger, candidates, core_before)) =
        store.scoped(Some(initiative), |s| -> Result<_> {
            let Some(trigger) = due(s, initiative)? else {
                return Ok(None);
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
        ..Default::default()
    };

    let batch_size = store.config().hygiene_batch_size.max(1);
    for (index, batch) in candidates.chunks(batch_size).enumerate() {
        if index > 0 {
            pause();
        }
        // One short critical section per batch: this is the whole reason the
        // pass is batched at all.
        store.scoped(Some(initiative), |s| apply_batch(s, batch, &mut report))?;
    }

    store.scoped(Some(initiative), |s| -> Result<()> {
        report.core_after = core_count(s, initiative)?;
        let nodes_seen = node_count(s, initiative)?;
        record_run(s, initiative, nodes_seen, Some(&report.headline()))
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

        let report = run_pass(&store, "proj", || {})
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

        let report = run_pass(&store, "proj", || {}).expect("pass").expect("due");

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

        let report = run_pass(&store, "proj", || {}).expect("pass").expect("due");

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

        let report = run_pass(&store, "proj", || {}).expect("pass").expect("due");

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

        let report = run_pass(&store, "proj", || {}).expect("pass").expect("due");

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

        let first = run_pass(&store, "proj", || {}).expect("pass").expect("due");
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

        run_pass(&store, "proj", || {}).expect("pass").expect("due");
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
        run_pass(&store, "proj", || {}).expect("pass").expect("due");

        let first = take_pending_report(&store, "proj").expect("take");
        assert!(
            first.as_deref().is_some_and(|s| s.contains("hygiene")),
            "the headline is waiting: {first:?}"
        );
        assert_eq!(
            take_pending_report(&store, "proj").expect("take again"),
            None,
            "delivered once, then cleared"
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

    #[test]
    fn hygiene_moves_are_attributed_in_the_audit_trail() {
        let store = eager_store();
        let old = episode_in(&store, "proj", "old-note");
        backdate(&store, &old, 30);
        episode_in(&store, "proj", "filler");
        run_pass(&store, "proj", || {}).expect("pass").expect("due");

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
}
