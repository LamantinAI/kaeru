//! `lint` — read-only graph-hygiene snapshot. Surfaces orphan nodes and
//! the unresolved-review queue in one report.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;

use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

use super::under_review::under_review_pinned;

/// Diagnostic report produced by [`lint`] — graph anomalies the agent
/// might want to address. Empty fields mean "no findings of that kind".
#[derive(Debug, Clone)]
pub struct LintReport {
    /// Nodes valid at NOW that have no incoming or outgoing edges valid
    /// at NOW. Often legitimate (a fresh episode is briefly orphaned),
    /// but persistent orphans usually want either linking or forgetting.
    pub orphans: Vec<NodeId>,
    /// Nodes with inbound `contradicts` edges valid at NOW —
    /// the open-review queue. Mirror of `under_review_pinned` surfaced
    /// here so a single `lint` call returns the agent's hygiene to-do list.
    pub unresolved_reviews: Vec<NodeId>,
}

/// Returns a diagnostic snapshot of graph hygiene at NOW. Read-only.
///
/// MVP scope covers orphans and unresolved reviews. Dangling edges
/// (edges valid at NOW pointing at retracted endpoints) are a known
/// follow-up — they require parsing each edge's endpoints against the
/// current node set, and the bi-temporal substrate already keeps the
/// historical edge intact, so the issue surfaces as a query-time
/// anomaly rather than corruption.
pub fn lint(store: &Store) -> Result<LintReport> {
    // Orphans: nodes valid at NOW that don't appear as `src` or `dst` of
    // any edge valid at NOW. When an initiative is active, the projection
    // also restricts orphans to nodes attached to that initiative.
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let orphan_script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
                connected[id] := *edge{src: id, dst, edge_type @ 'NOW'}
                connected[id] := *edge{src, dst: id, edge_type @ 'NOW'}
                ?[id] := *node{id, type @ 'NOW'},
                          type != 'audit_event',
                          not connected[id],
                          *node_initiative{initiative, node_id: id},
                          initiative = $init
            "#
        }
        None => {
            r#"
                connected[id] := *edge{src: id, dst, edge_type @ 'NOW'}
                connected[id] := *edge{src, dst: id, edge_type @ 'NOW'}
                ?[id] := *node{id, type @ 'NOW'}, type != 'audit_event', not connected[id]
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(orphan_script, params, ScriptMutability::Immutable)?;
    let orphans: Vec<NodeId> = rows
        .rows
        .iter()
        .filter_map(|r| r.first().and_then(|v| v.get_str()).map(String::from))
        .collect();

    let unresolved_reviews = under_review_pinned(store)?;

    Ok(LintReport {
        orphans,
        unresolved_reviews,
    })
}
