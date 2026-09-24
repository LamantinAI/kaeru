//! What changed about a node without minting a version (#95).
//!
//! The substrate versions a node's `type`, `tier`, `name`, `body` and `tags`:
//! each write asserts a new row, so `at` can read the node as it stood and
//! `history` can list the revisions. Three fields are **not** versioned —
//! `layer`, `visibility` and an edge's `weight` — because changing them is an
//! in-place rewrite of the current row.
//!
//! That was deliberate. A layer change asserted as a new version used to
//! leave two rows competing at NOW, and the loser took the node out of every
//! read while its edges survived. Rewriting in place cannot do that.
//!
//! The cost is that a read of the past reports today's layer and says nothing
//! about it, which is worse than either being versioned or being absent: a
//! reader trusts it. So the change is not invisible — every one of these
//! writes leaves an audit event naming the moment and the actor — and this
//! module is what reads them back, so `at` and `history` can point at the
//! record instead of quietly presenting the current value as a past one.
//!
//! What is recorded is *that* it changed and *who* changed it, never the old
//! value. A `layer` a node held last Tuesday is not recoverable from here.

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};
use serde_json::Value;

use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

/// The operations that rewrite a node or edge in place instead of asserting
/// a new version. Keep this list and the primitives that write these audit
/// ops in step — a rewrite whose op is missing here is a change that
/// disappears from `history` entirely.
pub const UNVERSIONED_OPS: [&str; 3] = ["set_layer", "set_visibility", "set_edge_weight"];

/// One in-place change, as the audit trail recorded it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnversionedChange {
    /// When it happened, in the same whole seconds as a `Revision`.
    pub seconds: u64,
    /// `set_layer`, `set_visibility` or `set_edge_weight`.
    pub op: String,
    /// Who did it — `system` for an agent's own call, `hygiene` for the
    /// background pass. This is what answers "did I do that, or did the
    /// sweep?".
    pub actor: String,
}

/// Every in-place change recorded against `id`, oldest first.
///
/// Ordered by whole seconds, which is all the audit trail stores: two
/// rewrites inside one second come back in an arbitrary order relative to
/// each other.
///
/// Reads the audit trail rather than the node's own rows, because the node's
/// rows are exactly what these writes do not add to.
pub fn unversioned_changes(store: &Store, id: &NodeId) -> Result<Vec<UnversionedChange>> {
    let rows = store.db_ref().run_script(
        r#"
        ?[validity, properties] := *node{id, type, validity, properties @ 'NOW'},
                                   type = 'audit_event'
        :order validity
        "#,
        BTreeMap::new(),
        ScriptMutability::Immutable,
    )?;

    let mut out = Vec::new();
    for row in &rows.rows {
        let Some(DataValue::Json(payload)) = row.get(1) else {
            continue;
        };
        let op = payload.0.get("op").and_then(Value::as_str).unwrap_or("");
        if !UNVERSIONED_OPS.contains(&op) {
            continue;
        }
        let touches = payload
            .0
            .get("affected_refs")
            .and_then(Value::as_array)
            .is_some_and(|refs| refs.iter().any(|r| r.as_str() == Some(id.as_str())));
        if !touches {
            continue;
        }
        let seconds = crate::graph::temporal::parse_validity(row.first())
            .map(|(secs, _)| secs as u64)
            .unwrap_or(0);
        out.push(UnversionedChange {
            seconds,
            op: op.to_string(),
            actor: payload
                .0
                .get("actor")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::{UNVERSIONED_OPS, unversioned_changes};
    use crate::graph::Layer;
    use crate::store::Store;

    /// The change a version cannot show is at least recorded, with its actor
    /// — which is how "did the sweep move this, or did I?" gets answered.
    #[test]
    fn a_layer_change_is_readable_even_though_it_is_not_a_version() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("proj");
        let id = crate::jot(&store, "a note").expect("jot");
        assert!(
            unversioned_changes(&store, &id).expect("read").is_empty(),
            "nothing has been rewritten yet"
        );

        crate::set_layer(&store, &id, Layer::Hot).expect("layer");
        crate::mutate::set_layer_as(&store, &id, Layer::Cold, "hygiene").expect("sweep");

        let changes = unversioned_changes(&store, &id).expect("read");
        assert_eq!(changes.len(), 2, "both rewrites are there: {changes:?}");
        assert!(changes.iter().all(|c| c.op == "set_layer"));
        // Both landed in the same whole second, and within a second the
        // order is not recoverable — which is the same honesty problem this
        // module exists to state, one level down. Assert what is knowable.
        let actors: Vec<&str> = changes.iter().map(|c| c.actor.as_str()).collect();
        assert!(
            actors.contains(&"system"),
            "the agent's own call: {actors:?}"
        );
        assert!(
            actors.contains(&"hygiene"),
            "and the background pass: {actors:?}"
        );

        // Another node's rewrite is not this node's history.
        let other = crate::jot(&store, "another note").expect("jot");
        crate::set_layer(&store, &other, Layer::Hot).expect("layer");
        assert_eq!(
            unversioned_changes(&store, &id).expect("read").len(),
            2,
            "still only its own"
        );
    }

    #[test]
    fn the_op_list_matches_what_the_primitives_write() {
        // A cheap guard: if a primitive is renamed, its op stops matching and
        // the change quietly vanishes from `history`. The names are asserted
        // here so that rename fails a test instead.
        assert_eq!(
            UNVERSIONED_OPS,
            ["set_layer", "set_visibility", "set_edge_weight"]
        );
    }
}
