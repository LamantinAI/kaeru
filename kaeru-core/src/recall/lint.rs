//! `lint` — read-only graph-hygiene snapshot. Surfaces orphan nodes,
//! the unresolved-review queue, and dangling edges in one report.

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};

use super::under_review::under_review_pinned;
use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

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
    /// Edges valid at NOW where at least one endpoint is no longer a
    /// live node — `(src, dst, edge_type)`. These arise when a node is
    /// retracted without its edges (e.g. `consolidate` retracts the
    /// draft but leaves its old edges asserted). Not corruption — the
    /// bi-temporal substrate keeps the history intact — but a traversal
    /// at NOW walks into nothing, so they usually want re-pointing at
    /// the successor node or retracting.
    pub dangling_edges: Vec<(NodeId, NodeId, String)>,
}

/// Returns a diagnostic snapshot of graph hygiene at NOW. Read-only.
///
/// Covers orphans, unresolved reviews, and dangling edges.
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

    // Dangling edges: edges valid at NOW whose src and/or dst is not a
    // live node at NOW. When an initiative is active, an edge is in scope
    // if either endpoint is attached to it — the junction is append-only,
    // so a retracted endpoint still carries its membership row.
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let dangling_script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
                alive[id] := *node{id, type @ 'NOW'}
                member[id] := *node_initiative{initiative, node_id: id}, initiative = $init
                scoped[src, dst, edge_type] := *edge{src, dst, edge_type @ 'NOW'}, member[src]
                scoped[src, dst, edge_type] := *edge{src, dst, edge_type @ 'NOW'}, member[dst]
                ?[src, dst, edge_type] := scoped[src, dst, edge_type], not alive[src]
                ?[src, dst, edge_type] := scoped[src, dst, edge_type], not alive[dst]
            "#
        }
        None => {
            r#"
                alive[id] := *node{id, type @ 'NOW'}
                ?[src, dst, edge_type] := *edge{src, dst, edge_type @ 'NOW'}, not alive[src]
                ?[src, dst, edge_type] := *edge{src, dst, edge_type @ 'NOW'}, not alive[dst]
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(dangling_script, params, ScriptMutability::Immutable)?;
    let dangling_edges: Vec<(NodeId, NodeId, String)> = rows
        .rows
        .iter()
        .filter_map(|r| {
            let src = r.first().and_then(|v| v.get_str())?;
            let dst = r.get(1).and_then(|v| v.get_str())?;
            let edge_type = r.get(2).and_then(|v| v.get_str())?;
            Some((src.to_string(), dst.to_string(), edge_type.to_string()))
        })
        .collect();

    Ok(LintReport {
        orphans,
        unresolved_reviews,
        dangling_edges,
    })
}

#[cfg(test)]
mod tests {
    use crate::graph::{EdgeType, NodeType};
    use crate::store::Store;

    /// `consolidate_out` retracts the draft node but leaves its old edges
    /// asserted — the canonical dangling-edge factory. `lint` must report
    /// them; the live replacement edge set must stay clean.
    #[test]
    fn lint_reports_edges_left_behind_by_consolidation() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("demo");

        let a = crate::jot(&store, "draft under test").expect("jot a");
        let b = crate::jot(&store, "stable neighbour").expect("jot b");
        crate::link(&store, &a, &b, EdgeType::RefersTo).expect("link a->b");

        let before = crate::lint(&store).expect("lint before");
        assert!(
            before.dangling_edges.is_empty(),
            "no dangling edges while both endpoints are alive"
        );

        // Validity has whole-second resolution; a retraction in the same
        // second as the assertion is ambiguous at `@ 'NOW'`. Sleep across
        // the second boundary, as the rest of the test suite does.
        std::thread::sleep(std::time::Duration::from_millis(1100));
        let new_id = crate::consolidate_out(&store, &a, NodeType::Outcome, "a-settled", "body")
            .expect("consolidate");
        std::thread::sleep(std::time::Duration::from_millis(1100));

        let after = crate::lint(&store).expect("lint after");
        let refers = (a.clone(), b.clone(), "refers_to".to_string());
        let consolidated = (a.clone(), new_id.clone(), "consolidated_to".to_string());
        assert!(
            after.dangling_edges.contains(&refers),
            "old refers_to edge from the retracted draft is dangling: {:?}",
            after.dangling_edges
        );
        assert!(
            after.dangling_edges.contains(&consolidated),
            "consolidated_to edge hangs off the retracted draft: {:?}",
            after.dangling_edges
        );
        assert!(
            !after
                .dangling_edges
                .iter()
                .any(|(s, d, _)| s == &new_id && d == &b),
            "edges of the live replacement are not dangling"
        );
    }
}
