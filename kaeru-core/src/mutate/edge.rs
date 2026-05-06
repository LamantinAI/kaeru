//! Edge-level mutations: `link` (assert) and `unlink` (retract).

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;

use crate::errors::Result;
use crate::graph::EdgeType;
use crate::graph::NodeId;
use crate::graph::audit::write_audit;
use crate::store::Store;

use super::attach_edge_to_initiative;
use super::now_validity_seconds;

/// Creates a typed directed edge and writes an audit_event.
pub fn link(
    store: &Store,
    src: &NodeId,
    dst: &NodeId,
    edge_type: EdgeType,
) -> Result<()> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("src".to_string(), DataValue::Str(src.clone().into()));
    params.insert("dst".to_string(), DataValue::Str(dst.clone().into()));
    params.insert(
        "edge_type".to_string(),
        DataValue::Str(edge_type.as_str().into()),
    );

    let now_secs = now_validity_seconds();
    let script = format!(
        r#"
        ?[src, dst, edge_type, validity, weight, properties] <-
            [[$src, $dst, $edge_type, [{now_secs}.0, true], 1.0, null]]
        :put edge {{src, dst, edge_type, validity => weight, properties}}
        "#
    );
    store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Mutable)?;

    attach_edge_to_initiative(store, src, dst, edge_type.as_str())?;
    write_audit(
        store.db_ref(),
        "link",
        "system",
        &[src.clone(), dst.clone()],
    )?;
    Ok(())
}

/// Retracts a previously-asserted edge through the bi-temporal substrate.
/// The historical assertion stays in the graph (so `history`-style queries
/// at earlier timestamps still see the edge); only reads at NOW or after
/// the retraction skip it.
///
/// No-op-safe: retracting an edge that was never asserted is harmless —
/// the substrate just records a retraction with no effect on reads.
pub fn unlink(
    store: &Store,
    src: &NodeId,
    dst: &NodeId,
    edge_type: EdgeType,
) -> Result<()> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("src".to_string(), DataValue::Str(src.clone().into()));
    params.insert("dst".to_string(), DataValue::Str(dst.clone().into()));
    params.insert(
        "edge_type".to_string(),
        DataValue::Str(edge_type.as_str().into()),
    );

    let now_secs = now_validity_seconds();
    let script = format!(
        r#"
        ?[src, dst, edge_type, validity, weight, properties] <-
            [[$src, $dst, $edge_type, [{now_secs}.0, false], 1.0, null]]
        :put edge {{src, dst, edge_type, validity => weight, properties}}
        "#
    );
    store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Mutable)?;

    write_audit(
        store.db_ref(),
        "unlink",
        "system",
        &[src.clone(), dst.clone()],
    )?;
    Ok(())
}
