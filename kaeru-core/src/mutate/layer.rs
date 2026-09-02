//! `set_layer` — change a node's memory layer.

use std::collections::BTreeMap;
use std::str::FromStr;

use cozo::{DataValue, ScriptMutability};

use super::rewrite_node_column_in_place;
use crate::errors::Result;
use crate::graph::audit::write_audit;
use crate::graph::{Layer, NodeId};
use crate::store::Store;

/// Changes the memory layer of an existing node.
///
/// Layers control priority during context injection:
/// - `Core` — always in context
/// - `Hot` — injected first after Core
/// - `Warm` — default, relevant
/// - `Cold` — archived, explicit recall only
/// - `Frozen` — stored but not surfaced
///
/// Changes a node's `layer`, preserving every other attribute.
///
/// Implemented as an in-place rewrite: the node's current row is read
/// together with its exact `validity` key and re-`:put` with only the
/// `layer` value changed. Because no new validity version is minted, the
/// `@ 'NOW'` travel can never resolve to two competing versions — the
/// failure that previously hid the node (while its edges survived).
/// Trade-off: the layer change itself is not separately versioned in
/// history; the node keeps the validity of whatever version it had.
///
/// Field values round-trip as Cozo parameters (`$body`, `$tags`, …)
/// rather than being string-formatted into the script — `DataValue`s
/// read out go straight back in, so bodies/lists never need escaping.
///
/// The read prefers the `@ 'NOW'` view; if the node is not visible at
/// NOW (e.g. a node left invisible by the older buggy `set_layer`), it
/// falls back to the latest historical version, so re-running this verb
/// also *recovers* such nodes.
pub fn set_layer(store: &Store, node_id: &NodeId, layer: Layer) -> Result<()> {
    set_layer_as(store, node_id, layer, "system")
}

/// [`set_layer`] with an explicit audit actor. The hygiene pass writes
/// `"hygiene"` so its moves are separable from an agent's deliberate
/// `layer` call in the audit trail — "what did the sweep touch on the 12th"
/// is a query, not a guess.
pub fn set_layer_as(store: &Store, node_id: &NodeId, layer: Layer, actor: &str) -> Result<()> {
    // The rewrite itself is shared with `set_visibility` and generated from
    // `NODE_VALUE_COLUMNS`, so a column added to the schema round-trips here
    // without this verb being taught about it — the class of silent data loss
    // that hand-written column lists caused before.
    rewrite_node_column_in_place(store, node_id, "layer", layer.as_str())?;

    write_audit(store.db_ref(), "set_layer", actor, &[node_id.clone()])?;

    Ok(())
}

/// Returns the current layer of a node.
pub fn get_layer(store: &Store, node_id: &NodeId) -> Result<Layer> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(node_id.clone().into()));
    let script = r#"
        ?[layer] := *node{id, layer @ 'NOW'}, id = $id
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let layer_str = rows
        .rows
        .first()
        .and_then(|row| row.first())
        .and_then(|v| v.get_str())
        .unwrap_or("warm");

    Layer::from_str(layer_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        EpisodeKind, Significance, jot_with_layer, write_episode, write_episode_with_layer,
    };

    #[test]
    fn set_layer_changes_node_layer() {
        let store = Store::open_in_memory().expect("open");

        let id = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "test-layer",
            "test body",
        )
        .unwrap();

        // Default layer is Warm
        let layer = get_layer(&store, &id).unwrap();
        assert_eq!(layer, Layer::Warm);

        // Change to Hot
        set_layer(&store, &id, Layer::Hot).unwrap();
        let layer = get_layer(&store, &id).unwrap();
        assert_eq!(layer, Layer::Hot);

        // Change to Core
        set_layer(&store, &id, Layer::Core).unwrap();
        let layer = get_layer(&store, &id).unwrap();
        assert_eq!(layer, Layer::Core);

        // Change to Frozen
        set_layer(&store, &id, Layer::Frozen).unwrap();
        let layer = get_layer(&store, &id).unwrap();
        assert_eq!(layer, Layer::Frozen);
    }

    #[test]
    fn set_layer_keeps_node_visible_when_changed_later() {
        // Regression: changing a layer at a whole-second *after* the node
        // was written must not make it invisible at NOW. The earlier impl
        // emitted a same-second retract that won the validity tie-break and
        // hid the node from `@ 'NOW'` reads (while edges survived).
        let store = Store::open_in_memory().expect("open");

        let id = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "later-layer",
            "body that should survive a later layer change",
        )
        .unwrap();

        // Force a distinct, later validity second for the layer change.
        std::thread::sleep(std::time::Duration::from_millis(1_100));
        set_layer(&store, &id, Layer::Core).unwrap();

        // The node must still resolve at NOW...
        let visible = crate::mutate::read_node_now(&store, &id).unwrap();
        assert!(
            visible.is_some(),
            "node went invisible at NOW after a later set_layer"
        );
        // ...and carry the new layer.
        assert_eq!(get_layer(&store, &id).unwrap(), Layer::Core);
    }

    #[test]
    fn write_episode_with_explicit_layer() {
        let store = Store::open_in_memory().expect("open");
        // A `core` write needs an initiative (#81) — this test is about the
        // layer being stored, so give it a home first.
        store.use_initiative("proj");

        let id = write_episode_with_layer(
            &store,
            EpisodeKind::Decision,
            Significance::High,
            "core-decision",
            "always remember this",
            Layer::Core,
        )
        .unwrap();

        let layer = get_layer(&store, &id).unwrap();
        assert_eq!(layer, Layer::Core);
    }

    #[test]
    fn core_write_without_an_initiative_is_refused() {
        let store = Store::open_in_memory().expect("open");
        // No initiative → a `core` node could never load in a scoped session, so
        // it is refused at the source (#81).
        let refused = write_episode_with_layer(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "orphan-core",
            "body",
            Layer::Core,
        );
        assert!(refused.is_err(), "core with no initiative is refused");

        // A non-core untagged capture is an ordinary local note — still allowed.
        let allowed = write_episode_with_layer(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "warm-note",
            "body",
            Layer::Warm,
        );
        assert!(allowed.is_ok(), "warm untagged capture stays allowed");
    }

    #[test]
    fn jot_with_layer_works() {
        let store = Store::open_in_memory().expect("open");

        let id = jot_with_layer(&store, "quick hot thought", Layer::Hot).unwrap();
        let layer = get_layer(&store, &id).unwrap();
        assert_eq!(layer, Layer::Hot);
    }
}
