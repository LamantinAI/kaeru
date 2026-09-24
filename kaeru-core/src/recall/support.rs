//! How many live nodes **support** a node — the ordering key re-entry needs
//! (#97).
//!
//! The re-entry window has always been "the newest fifteen by write time",
//! which is a FIFO over writes and not over need: the last fifteen, never the
//! needed fifteen. A node the whole project points at drops out of the window
//! the moment fifteen newer things are written, while a note nobody ever
//! referred to sits in it because it is recent.
//!
//! Inbound edges from live nodes are the graph's own statement of what
//! matters, and #92 made that number mean something by excluding the edges
//! that *cancel* a node rather than support it. This module is that count,
//! read once and shared, so the window can be sorted by it.

use std::collections::BTreeMap;

use cozo::ScriptMutability;

use super::verdicts::CANCELLING;
use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

/// `node id → how many live nodes point at it in support`.
///
/// Cancelling types (`supersedes`, `contradicts`, `falsifies`) are excluded:
/// the edge that says a node is obsolete is not a reason to load it first.
/// A reference held by a retracted node does not count either — the same
/// rule the hygiene pass uses, for the same reason.
///
/// One query for the whole graph: the alternative is a query per node in a
/// listing, and the listing is on the re-entry path.
pub fn support_counts(store: &Store) -> Result<BTreeMap<NodeId, usize>> {
    let rows = store.db_ref().run_script(
        r#"
        live[src] := *node{id: src @ 'NOW'}
        ?[dst, edge_type, count(src)] := *edge{src, dst, edge_type @ 'NOW'}, live[src]
        "#,
        BTreeMap::new(),
        ScriptMutability::Immutable,
    )?;

    let mut out: BTreeMap<NodeId, usize> = BTreeMap::new();
    for row in &rows.rows {
        let (Some(dst), Some(edge_type), Some(n)) = (
            row.first().and_then(|v| v.get_str()),
            row.get(1).and_then(|v| v.get_str()),
            row.get(2).and_then(|v| v.get_int()),
        ) else {
            continue;
        };
        if CANCELLING.contains(&edge_type) {
            continue;
        }
        *out.entry(dst.to_string()).or_insert(0) += n.max(0) as usize;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::support_counts;
    use crate::graph::EdgeType;
    use crate::store::Store;

    #[test]
    fn support_counts_references_and_ignores_cancellations() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("proj");
        let hub = crate::jot(&store, "the fact everything points at").expect("jot");
        let quiet = crate::jot(&store, "a note nobody refers to").expect("jot");

        for i in 0..2 {
            let referrer = crate::jot(&store, &format!("note {i}")).expect("jot");
            crate::link(&store, &referrer, &hub, EdgeType::RefersTo).expect("link");
        }
        // A cancellation is not support — that was the whole of #92.
        let successor = crate::jot(&store, "what replaced it").expect("jot");
        crate::link(&store, &successor, &hub, EdgeType::Supersedes).expect("link");

        let counts = support_counts(&store).expect("counts");
        assert_eq!(counts.get(&hub), Some(&2), "two referrers, not three");
        assert_eq!(counts.get(&quiet), None, "nothing points at it");
    }
}
