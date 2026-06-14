//! `set_layer` — change a node's memory layer.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;
use std::str::FromStr;

use crate::errors::Result;
use crate::graph::Layer;
use crate::graph::NodeId;
use crate::graph::audit::write_audit;
use crate::store::Store;

use super::now_validity_seconds;

/// Changes the memory layer of an existing node.
///
/// Layers control priority during context injection:
/// - `Core` — always in context
/// - `Hot` — injected first after Core
/// - `Warm` — default, relevant
/// - `Cold` — archived, explicit recall only
/// - `Frozen` — stored but not surfaced
///
/// This performs a bi-temporal retract+reassert on the `layer` column
/// only, preserving all other node attributes. An audit event is
/// written for the operation.
pub fn set_layer(store: &Store, node_id: &NodeId, layer: Layer) -> Result<()> {
    // Read current node state to preserve all fields
    let read_script = format!(
        r#"
        ?[type, tier, name, body, tags, initiatives, properties, layer] :=
            *node{{type, tier, name, body, tags, initiatives, properties, layer}}, id = '{node_id}'
        "#
    );
    let current = store.run_read(&read_script)?;

    let row = current.rows.first().ok_or_else(|| {
        crate::errors::Error::NotFound(format!("node not found: {node_id}"))
    })?;

    let node_type = row[0].get_str().unwrap_or("episode");
    let tier = row[1].get_str().unwrap_or("operational");
    let name = row[2].get_str().unwrap_or("");
    let body_raw = format!("{:?}", row[3]);
    let tags_raw = format!("{:?}", row[4]);
    let initiatives_raw = format!("{:?}", row[5]);
    let properties_raw = format!("{:?}", row[6]);
    let old_layer = row[7].get_str().unwrap_or("warm");

    let now_secs = now_validity_seconds();
    let new_layer_str = layer.as_str();

    // Retract current assertion — use the old layer value (non-null column)
    let retract_script = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties, layer] <-
            [['{node_id}', [{now_secs}.0, false], '{node_type}', '{tier}', '{name}', {body_raw}, {tags_raw}, {initiatives_raw}, {properties_raw}, '{old_layer}']]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties, layer}}
        "#
    );
    store.run(&retract_script)?;

    // Re-assert with new layer
    let reassert_script = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties, layer] <-
            [['{node_id}', [{now_secs}.0, true], '{node_type}', '{tier}', '{name}', {body_raw}, {tags_raw}, {initiatives_raw}, {properties_raw}, '{new_layer_str}']]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties, layer}}
        "#
    );
    store.run(&reassert_script)?;

    write_audit(
        store.db_ref(),
        "set_layer",
        "system",
        &[node_id.clone()],
    )?;

    Ok(())
}

/// Returns the current layer of a node.
pub fn get_layer(store: &Store, node_id: &NodeId) -> Result<Layer> {
    let script = format!(
        r#"
        ?[layer] := *node{{layer @ 'NOW'}}, id = '{node_id}'
        "#
    );
    let rows = store.run_read(&script)?;

    let layer_str = rows.rows.first()
        .and_then(|row| row.first())
        .and_then(|v| v.get_str())
        .unwrap_or("warm");

    Layer::from_str(layer_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EpisodeKind;
    use crate::Significance;
    use crate::write_episode;
    use crate::write_episode_with_layer;
    use crate::jot_with_layer;

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
