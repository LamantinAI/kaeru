//! Read-only graph scanning and hygiene candidate selection.
use crate::graph::temporal::validity_seconds;
use crate::graph::{Layer, NodeId};
use crate::mutate::now_validity_seconds;
use crate::recall::verdicts::{CANCELLING, Verdict, verdicts_against};
use crate::{Result, Store};
use cozo::{DataValue, ScriptMutability};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};

/// What the pass proposes to do with a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HygieneAction {
    /// Journal entry nobody references any more → `cold`.
    Archive,
    /// `core` node the pass unloads by one step, to `hot`: either untouched
    /// and unreferenced, or cancelled by a verdict — see [`Verdict`].
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

/// One row of the node scan: everything the rules need, read once.
struct Scanned {
    id: NodeId,
    name: String,
    node_type: String,
    layer: Layer,
    ts: f64,
    /// Inbound edges from live nodes that **support** this one — cancelling
    /// types excluded, see [`CANCELLING`].
    in_degree: usize,
    /// The verdict a live node has passed on this one, if any.
    cancelled_by: Option<Verdict>,
    /// When the pass last unloaded this node from `core`, if it did (#94).
    demoted_at: Option<f64>,
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
    // Counted per type, because a cancelling edge is not support (#92).
    let inbound = store.db_ref().run_script(
        r#"
        live[src] := *node{id: src @ 'NOW'}
        ?[dst, edge_type, count(src)] := *edge{src, dst, edge_type @ 'NOW'}, live[src]
        "#,
        BTreeMap::new(),
        ScriptMutability::Immutable,
    )?;
    let mut in_degree: BTreeMap<String, usize> = BTreeMap::new();
    for r in &inbound.rows {
        let (Some(dst), Some(edge_type), Some(n)) = (
            r.first().and_then(|v| v.get_str()),
            r.get(1).and_then(|v| v.get_str()),
            r.get(2).and_then(|v| v.get_int()),
        ) else {
            continue;
        };
        if CANCELLING.contains(&edge_type) {
            continue;
        }
        *in_degree.entry(dst.to_string()).or_insert(0) += n.max(0) as usize;
    }

    // What the graph has already ruled on. Verdicts only: a `contradicts`
    // is an open doubt, and stops at not protecting its target.
    let cancelled_by = verdicts_against(store)?;

    // When the pass last unloaded a node from `core`, for the grace period
    // the archive rule owes it (#94).
    let demotions = store.db_ref().run_script(
        "?[node_id, at] := *hygiene_demotion{node_id, at}",
        BTreeMap::new(),
        ScriptMutability::Immutable,
    )?;
    let mut demoted_at: BTreeMap<String, f64> = BTreeMap::new();
    for r in &demotions.rows {
        if let (Some(id), Some(at)) = (
            r.first().and_then(|v| v.get_str()),
            r.get(1).and_then(|v| v.get_float()),
        ) {
            demoted_at.insert(id.to_string(), at);
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
            cancelled_by: cancelled_by.get(&id).cloned(),
            demoted_at: demoted_at.get(&id).copied(),
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
            // Demote what nothing references and nobody has touched — and
            // what the graph itself says is obsolete, however recent and
            // however well connected (#92). A superseded fact injected into
            // every session is worse than a stale one: it is read as current
            // and planned from, which is how this was found.
            Layer::Core => {
                if let Some(verdict) = &node.cancelled_by {
                    out.push(HygieneCandidate {
                        node_id: node.id.clone(),
                        name: node.name.clone(),
                        action: HygieneAction::DemoteFromCore,
                        from: Layer::Core,
                        to: Layer::Hot,
                        reason: verdict.phrase(),
                    });
                } else if node.ts < core_cutoff && node.in_degree == 0 {
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
            // written. Archive once they are old AND unreferenced — unless
            // the pass itself put the node here (#94). A layer move is an
            // in-place rewrite that does not refresh the node's timestamp,
            // so a node demoted out of `core` is instantly an old `hot`
            // episode and the next pass would archive it. That would make
            // "never more than one step" true per pass and false across
            // two, which is the promise a user actually relies on: a
            // misjudged `core` node lands in the working set, where it can
            // be seen and put back, not in an archive nobody opens.
            Layer::Hot | Layer::Warm => {
                let in_grace = node
                    .demoted_at
                    .is_some_and(|at| now - at < cfg.hygiene_journal_age_secs as f64);
                if node.node_type == "episode"
                    && node.ts < journal_cutoff
                    && node.in_degree == 0
                    && !in_grace
                {
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
                    "core over its ceiling ({core_total} of {}) — least referenced \
                     ({} inbound), untouched {age_days}d",
                    cfg.hygiene_core_trigger, node.in_degree
                ),
            });
        }
    }

    Ok(out)
}
