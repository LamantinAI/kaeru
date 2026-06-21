//! Edge-level mutations: `link` (assert) and `unlink` (retract).

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};

use super::{attach_edge_to_initiative, now_validity_seconds};
use crate::errors::{Error, Result};
use crate::graph::audit::write_audit;
use crate::graph::{EdgeType, NodeId};
use crate::store::Store;

/// Creates a typed directed edge at full strength (`weight = 1.0`) and
/// writes an audit_event.
pub fn link(store: &Store, src: &NodeId, dst: &NodeId, edge_type: EdgeType) -> Result<()> {
    link_with_weight(store, src, dst, edge_type, 1.0)
}

/// Creates a typed directed edge carrying an explicit `weight` — the
/// agent's judgment of the connection's strength, in `[0, 1]` (1 = strong).
/// `weight` is the signal for semantic shortest-path / knowledge chains:
/// traversal cost is `1 − weight`, so stronger edges make shorter paths.
/// Out-of-range values are clamped.
pub fn link_with_weight(
    store: &Store,
    src: &NodeId,
    dst: &NodeId,
    edge_type: EdgeType,
    weight: f64,
) -> Result<()> {
    let w = weight.clamp(0.0, 1.0);

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("src".to_string(), DataValue::Str(src.clone().into()));
    params.insert("dst".to_string(), DataValue::Str(dst.clone().into()));
    params.insert(
        "edge_type".to_string(),
        DataValue::Str(edge_type.as_str().into()),
    );

    let now_secs = now_validity_seconds();
    // `{w:.6}` keeps a decimal point so Cozo reads it as a Float, not an Int.
    let script = format!(
        r#"
        ?[src, dst, edge_type, validity, weight, properties] <-
            [[$src, $dst, $edge_type, [{now_secs}.0, true], {w:.6}, null]]
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

/// Updates the `weight` of an existing edge **in place** at NOW, leaving
/// every other field (validity key, properties, dst_store) untouched. The
/// edit handle behind the `reweight` tool and weight rebalancing.
///
/// In-place — the current row is re-`:put` under its own `validity` key with
/// only `weight` changed — so it never mints a new bi-temporal version. This
/// deliberately mirrors `set_layer`: a fresh assertion at a later whole-second
/// races the `@ 'NOW'` validity tie-break and can resolve to the wrong
/// version, so we overwrite the current row instead. Returns `NotFound` if
/// the edge is not valid at NOW. `weight` is clamped to `[0, 1]`.
pub fn set_edge_weight(
    store: &Store,
    src: &NodeId,
    dst: &NodeId,
    edge_type: EdgeType,
    weight: f64,
) -> Result<()> {
    let w = weight.clamp(0.0, 1.0);

    let mut rp: BTreeMap<String, DataValue> = BTreeMap::new();
    rp.insert("src".to_string(), DataValue::Str(src.clone().into()));
    rp.insert("dst".to_string(), DataValue::Str(dst.clone().into()));
    rp.insert("et".to_string(), DataValue::Str(edge_type.as_str().into()));

    let read = r#"
        ?[validity, properties, dst_store] :=
            *edge{src, dst, edge_type, validity, properties, dst_store @ 'NOW'},
            src = $src, dst = $dst, edge_type = $et
    "#;
    let rows = store
        .db_ref()
        .run_script(read, rp.clone(), ScriptMutability::Immutable)?;
    let row = rows.rows.first().ok_or_else(|| {
        Error::NotFound(format!(
            "edge {src} -[{}]-> {dst} not found at NOW",
            edge_type.as_str()
        ))
    })?;

    let mut p: BTreeMap<String, DataValue> = BTreeMap::new();
    p.insert("src".to_string(), DataValue::Str(src.clone().into()));
    p.insert("dst".to_string(), DataValue::Str(dst.clone().into()));
    p.insert("et".to_string(), DataValue::Str(edge_type.as_str().into()));
    p.insert("validity".to_string(), row[0].clone());
    p.insert("properties".to_string(), row[1].clone());
    p.insert("dst_store".to_string(), row[2].clone());

    let put = format!(
        r#"
        ?[src, dst, edge_type, validity, weight, properties, dst_store] <-
            [[$src, $dst, $et, $validity, {w:.6}, $properties, $dst_store]]
        :put edge {{src, dst, edge_type, validity => weight, properties, dst_store}}
        "#
    );
    store
        .db_ref()
        .run_script(&put, p, ScriptMutability::Mutable)?;

    write_audit(
        store.db_ref(),
        "set_edge_weight",
        "system",
        &[src.clone(), dst.clone()],
    )?;
    Ok(())
}

/// Creates a **soft link** from a local node to a node in the *default*
/// shared cloud (`dst_store = 'cloud'`). Thin wrapper over
/// [`link_remote_to`] with no named cloud — kept for back-compat and the
/// single-cloud case.
pub fn link_remote(
    store: &Store,
    src: &NodeId,
    dst_cloud_id: &NodeId,
    edge_type: EdgeType,
) -> Result<()> {
    link_remote_to(store, src, dst_cloud_id, edge_type, None)
}

/// Creates a **soft link** from a local node to a node in a shared cloud.
/// A normal edge but with `dst_store` set to `cloud` (the default cloud)
/// or `cloud:<name>` when `cloud_name` names a specific cloud — that suffix
/// is how a multi-cloud daemon routes lazy resolution back to the right
/// endpoint. `dst` is the cloud node's UUIDv7 — it need not exist locally;
/// the edge is resolved lazily through the cloud API at read time. Only
/// `local → cloud` soft links exist (the cloud never sees local ids, so it
/// can't link back).
///
/// Because the cloud `dst` is not attached to any local `node_initiative`,
/// a local `walk` naturally never traverses into it — soft links are
/// followed only by the explicit cloud-resolution path (`cloud_links`).
pub fn link_remote_to(
    store: &Store,
    src: &NodeId,
    dst_cloud_id: &NodeId,
    edge_type: EdgeType,
    cloud_name: Option<&str>,
) -> Result<()> {
    let dst_store = match cloud_name {
        None => "cloud".to_string(),
        Some(n) => format!("cloud:{n}"),
    };

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("src".to_string(), DataValue::Str(src.clone().into()));
    params.insert(
        "dst".to_string(),
        DataValue::Str(dst_cloud_id.clone().into()),
    );
    params.insert(
        "edge_type".to_string(),
        DataValue::Str(edge_type.as_str().into()),
    );
    params.insert("dst_store".to_string(), DataValue::Str(dst_store.into()));

    let now_secs = now_validity_seconds();
    let script = format!(
        r#"
        ?[src, dst, edge_type, validity, weight, properties, dst_store] <-
            [[$src, $dst, $edge_type, [{now_secs}.0, true], 1.0, null, $dst_store]]
        :put edge {{src, dst, edge_type, validity => weight, properties, dst_store}}
        "#
    );
    store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Mutable)?;

    attach_edge_to_initiative(store, src, dst_cloud_id, edge_type.as_str())?;
    write_audit(
        store.db_ref(),
        "link_remote",
        "system",
        &[src.clone(), dst_cloud_id.clone()],
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
pub fn unlink(store: &Store, src: &NodeId, dst: &NodeId, edge_type: EdgeType) -> Result<()> {
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
