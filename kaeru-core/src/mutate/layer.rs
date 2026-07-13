//! `set_layer` — change a node's memory layer.

use std::collections::BTreeMap;
use std::str::FromStr;

use cozo::{DataValue, ScriptMutability};

use crate::errors::{Error, Result};
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
    let mut read_params: BTreeMap<String, DataValue> = BTreeMap::new();
    read_params.insert("id".to_string(), DataValue::Str(node_id.clone().into()));

    // Read the *current* row together with its exact `validity` key, so
    // the rewrite below can overwrite that same row in place rather than
    // asserting a new validity version. The `@ 'NOW'` view is preferred;
    // if the node is not valid at NOW (e.g. left invisible by the older
    // buggy `set_layer`), fall back to the most recent historical version
    // — re-running this verb on such a node restores it to NOW.
    let now_script = r#"
        ?[validity, type, tier, name, body, tags, initiatives, properties, visibility] :=
            *node{id, validity, type, tier, name, body, tags, initiatives, properties, visibility @ 'NOW'},
            id = $id
    "#;
    let mut current =
        store
            .db_ref()
            .run_script(now_script, read_params.clone(), ScriptMutability::Immutable)?;

    if current.rows.is_empty() {
        let hist_script = r#"
            ?[validity, type, tier, name, body, tags, initiatives, properties, visibility] :=
                *node{id, validity, type, tier, name, body, tags, initiatives, properties, visibility},
                id = $id
            :order -validity
            :limit 1
        "#;
        current =
            store
                .db_ref()
                .run_script(hist_script, read_params, ScriptMutability::Immutable)?;
    }

    let row = current
        .rows
        .first()
        .ok_or_else(|| Error::NotFound(format!("node not found: {node_id}")))?;

    // In-place rewrite: re-`:put` the SAME (id, validity) primary key with
    // only the `layer` value changed. No new validity is minted, so the
    // `@ 'NOW'` travel can never resolve to two competing versions — the
    // failure mode that previously hid nodes (their edges survived) while
    // a fresh-assertion approach left a stray duplicate row on RocksDB.
    // Values round-trip as Cozo parameters (`$validity`, `$body`, …), so
    // the Validity key and any lists/JSON are preserved byte-for-byte.
    let mut p: BTreeMap<String, DataValue> = BTreeMap::new();
    p.insert("id".to_string(), DataValue::Str(node_id.clone().into()));
    p.insert("validity".to_string(), row[0].clone());
    p.insert("type".to_string(), row[1].clone());
    p.insert("tier".to_string(), row[2].clone());
    p.insert("name".to_string(), row[3].clone());
    p.insert("body".to_string(), row[4].clone());
    p.insert("tags".to_string(), row[5].clone());
    p.insert("initiatives".to_string(), row[6].clone());
    p.insert("properties".to_string(), row[7].clone());
    p.insert("visibility".to_string(), row[8].clone());
    p.insert("layer".to_string(), DataValue::Str(layer.as_str().into()));
    let put_script = r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties, visibility, layer] <-
            [[$id, $validity, $type, $tier, $name, $body, $tags, $initiatives, $properties, $visibility, $layer]]
        :put node {id, validity => type, tier, name, body, tags, initiatives, properties, visibility, layer}
    "#;
    store
        .db_ref()
        .run_script(put_script, p, ScriptMutability::Mutable)?;

    write_audit(store.db_ref(), "set_layer", "system", &[node_id.clone()])?;

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
    fn jot_with_layer_works() {
        let store = Store::open_in_memory().expect("open");

        let id = jot_with_layer(&store, "quick hot thought", Layer::Hot).unwrap();
        let layer = get_layer(&store, &id).unwrap();
        assert_eq!(layer, Layer::Hot);
    }
}
