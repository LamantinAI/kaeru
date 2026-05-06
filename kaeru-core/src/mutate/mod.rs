//! Active-mutation primitives: `write_episode`, `link`, `synthesise`, …
//!
//! Each primitive is a graph mutation that automatically writes an
//! `audit_event` node alongside the domain change. Submodules group
//! primitives by the shape of the mutation they perform; this `mod.rs`
//! re-exports the public surface and houses cross-submodule helpers
//! (timestamp generation, RMW reads).

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

pub mod cite;
pub mod consolidate;
pub mod edge;
pub mod episode;
pub mod hypothesis;
pub mod metabolism;
pub mod review;
pub mod supersedes;
pub mod synthesise;

pub use cite::cite;
pub use consolidate::consolidate_in;
pub use consolidate::consolidate_out;
pub use edge::link;
pub use edge::unlink;
pub use episode::jot;
pub use episode::write_episode;
pub use hypothesis::formulate_hypothesis;
pub use hypothesis::run_experiment;
pub use hypothesis::update_hypothesis_status;
pub use metabolism::forget;
pub use metabolism::improve;
pub use review::mark_resolved;
pub use review::mark_under_review;
pub use supersedes::supersedes;
pub use synthesise::synthesise;

/// Cozo coerces `[float, bool]` to `Validity` only when the float is integer-
/// valued (whole seconds). Sub-second precision via fractional float fails
/// `eval::invalid_validity`. We therefore pin to whole-second resolution at
/// the substrate level. Tests that need distinct timestamps within the same
/// operation sequence add an explicit sleep.
pub(crate) fn now_validity_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Reads a node's `(name, body)` at NOW. Returns `None` if no row is valid
/// at the moment of the call. Used by primitives that need to rewrite a node
/// while preserving fields the caller did not change.
pub(crate) fn read_name_body_now(
    store: &Store,
    id: &NodeId,
) -> Result<Option<(String, Option<String>)>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));
    let script = r#"
        ?[name, body] := *node{id, name, body @ 'NOW'}, id = $id
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    let row = rows.rows.first();
    let result = row.map(|r| {
        let name = r
            .first()
            .and_then(|v| v.get_str())
            .map(String::from)
            .unwrap_or_default();
        let body = r.get(1).and_then(|v| v.get_str()).map(String::from);
        (name, body)
    });
    Ok(result)
}

/// Reads a node's `(type, tier)` strings at NOW for primitives that
/// preserve them through retract+reassert.
pub(crate) fn read_type_tier_now(
    store: &Store,
    id: &NodeId,
) -> Result<Option<(String, String)>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));
    let script = r#"
        ?[type, tier] := *node{id, type, tier @ 'NOW'}, id = $id
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    let result = rows.rows.first().and_then(|r| {
        let type_str = r.first().and_then(|v| v.get_str()).map(String::from)?;
        let tier_str = r.get(1).and_then(|v| v.get_str()).map(String::from)?;
        Some((type_str, tier_str))
    });
    Ok(result)
}

/// Returns every edge (src, dst, edge_type) connected to `node_id` at NOW
/// (inbound or outbound). Used by [`metabolism::forget`] to retract them.
pub(crate) fn read_connected_edges(
    store: &Store,
    node_id: &NodeId,
) -> Result<Vec<(String, String, String)>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("nid".to_string(), DataValue::Str(node_id.clone().into()));
    let script = r#"
        ?[src, dst, edge_type] := *edge{src, dst, edge_type @ 'NOW'}, src = $nid
        ?[src, dst, edge_type] := *edge{src, dst, edge_type @ 'NOW'}, dst = $nid
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    let edges: Vec<(String, String, String)> = rows
        .rows
        .iter()
        .filter_map(|r| {
            let src = r.first().and_then(|v| v.get_str()).map(String::from)?;
            let dst = r.get(1).and_then(|v| v.get_str()).map(String::from)?;
            let et = r.get(2).and_then(|v| v.get_str()).map(String::from)?;
            Some((src, dst, et))
        })
        .collect();
    Ok(edges)
}

/// Attaches `node_id` to the store's current initiative through the
/// `node_initiative` junction relation. No-op if no initiative is
/// active. Called by every mutation that asserts a fresh node.
pub(crate) fn attach_node_to_initiative(store: &Store, node_id: &NodeId) -> Result<()> {
    let Some(initiative) = store.current_initiative() else {
        return Ok(());
    };
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    params.insert("nid".to_string(), DataValue::Str(node_id.clone().into()));
    let script = r#"
        ?[initiative, node_id] <- [[$init, $nid]]
        :put node_initiative {initiative, node_id}
    "#;
    store
        .db_ref()
        .run_script(script, params, ScriptMutability::Mutable)?;
    Ok(())
}

/// Attaches an edge to the store's current initiative through the
/// `edge_initiative` junction relation. The edge's primary key is
/// encoded as `src|dst|edge_type` so re-attachment is idempotent. No-op
/// if no initiative is active.
pub(crate) fn attach_edge_to_initiative(
    store: &Store,
    src: &NodeId,
    dst: &NodeId,
    edge_type: &str,
) -> Result<()> {
    let Some(initiative) = store.current_initiative() else {
        return Ok(());
    };
    let edge_pk = format!("{src}|{dst}|{edge_type}");
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    params.insert("epk".to_string(), DataValue::Str(edge_pk.into()));
    let script = r#"
        ?[initiative, edge_pk] <- [[$init, $epk]]
        :put edge_initiative {initiative, edge_pk}
    "#;
    store
        .db_ref()
        .run_script(script, params, ScriptMutability::Mutable)?;
    Ok(())
}

/// Returns dst ids of `derived_from` edges where `src_id` is the source
/// at NOW. Used by [`consolidate`] to replicate provenance edges across
/// the tier boundary.
pub(crate) fn read_derived_from_targets(
    store: &Store,
    src_id: &NodeId,
) -> Result<Vec<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("src".to_string(), DataValue::Str(src_id.clone().into()));
    let script = r#"
        ?[dst] := *edge{src, dst, edge_type @ 'NOW'},
                  src = $src,
                  edge_type = 'derived_from'
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    let targets: Vec<NodeId> = rows
        .rows
        .iter()
        .filter_map(|r| r.first().and_then(|v| v.get_str()).map(String::from))
        .collect();
    Ok(targets)
}
