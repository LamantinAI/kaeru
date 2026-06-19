//! `between` — list every edge that connects two named nodes (in
//! either direction) at NOW.
//!
//! Answers the agent question "why are A and B connected?" — neither
//! `walk` (typed traversal) nor `summary_view` (1-hop drill-down)
//! cover this directly because they don't enumerate edges between a
//! specific pair.

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};

use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

/// One edge that connects the pair queried by [`between`].
///
/// `direction` is `true` when the edge points from the first argument
/// (`a`) to the second (`b`), `false` for the reverse direction.
#[derive(Debug, Clone, PartialEq)]
pub struct EdgeRow {
    pub edge_type: String,
    pub a_to_b: bool,
}

/// Returns every edge valid at NOW between `a` and `b`, in either
/// direction. Initiative-scoped via the store's `current_initiative`
/// (each endpoint must be in scope, mirroring `read_edges` in
/// `export`). Empty result means the pair is unconnected at NOW.
pub fn between(store: &Store, a: &NodeId, b: &NodeId) -> Result<Vec<EdgeRow>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("a".to_string(), DataValue::Str(a.clone().into()));
    params.insert("b".to_string(), DataValue::Str(b.clone().into()));

    // Two rules unioned: a → b and b → a. Initiative scope, when set,
    // requires both endpoints to be attached.
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
                forward[edge_type] := *edge{src, dst, edge_type @ 'NOW'},
                                       src = $a, dst = $b,
                                       *node_initiative{initiative, node_id: src},
                                       initiative = $init,
                                       *node_initiative{initiative: i2, node_id: dst},
                                       i2 = $init
                reverse[edge_type] := *edge{src, dst, edge_type @ 'NOW'},
                                       src = $b, dst = $a,
                                       *node_initiative{initiative, node_id: src},
                                       initiative = $init,
                                       *node_initiative{initiative: i2, node_id: dst},
                                       i2 = $init
                ?[edge_type, a_to_b] := forward[edge_type], a_to_b = true
                ?[edge_type, a_to_b] := reverse[edge_type], a_to_b = false
            "#
        }
        None => {
            r#"
                forward[edge_type] := *edge{src, dst, edge_type @ 'NOW'},
                                       src = $a, dst = $b
                reverse[edge_type] := *edge{src, dst, edge_type @ 'NOW'},
                                       src = $b, dst = $a
                ?[edge_type, a_to_b] := forward[edge_type], a_to_b = true
                ?[edge_type, a_to_b] := reverse[edge_type], a_to_b = false
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let edges = rows
        .rows
        .iter()
        .filter_map(|row| {
            let edge_type = row.first().and_then(|v| v.get_str())?.to_string();
            let a_to_b = row.get(1).and_then(|v| v.get_bool())?;
            Some(EdgeRow { edge_type, a_to_b })
        })
        .collect();
    Ok(edges)
}

/// Returns every `local` edge connected to `node_id` at NOW, in either
/// direction, as `(src, dst, edge_type, weight)`. Soft links
/// (`dst_store = cloud`) are excluded — those point at cloud ids and are not
/// real local edges. Used by the sharing path to find edges whose other
/// endpoint is also shared, so they (and their weight) can be pushed to the
/// cloud alongside the nodes.
pub fn edges_of(store: &Store, node_id: &NodeId) -> Result<Vec<(NodeId, NodeId, String, f64)>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("nid".to_string(), DataValue::Str(node_id.clone().into()));

    let script = r#"
        ?[src, dst, edge_type, weight] :=
            *edge{src, dst, edge_type, weight, dst_store @ 'NOW'},
            src = $nid, dst_store = 'local'
        ?[src, dst, edge_type, weight] :=
            *edge{src, dst, edge_type, weight, dst_store @ 'NOW'},
            dst = $nid, dst_store = 'local'
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let edges = rows
        .rows
        .iter()
        .filter_map(|row| {
            let src = row.first().and_then(|v| v.get_str())?.to_string();
            let dst = row.get(1).and_then(|v| v.get_str())?.to_string();
            let edge_type = row.get(2).and_then(|v| v.get_str())?.to_string();
            let weight = row.get(3).and_then(|v| v.get_float()).unwrap_or(1.0);
            Some((src, dst, edge_type, weight))
        })
        .collect();
    Ok(edges)
}

/// Returns a node's **soft links** — outgoing edges pointing into a shared
/// cloud (`dst_store` is `cloud` or `cloud:<name>`) valid at NOW. Each entry
/// is `(edge_type, cloud_name, cloud_dst_id)`, where `cloud_name` is `None`
/// for the default cloud (bare `cloud`) and `Some(name)` for a named one —
/// a multi-cloud daemon uses it to resolve the dst against the right
/// endpoint. Empty when the node has no soft links.
pub fn cloud_links(
    store: &Store,
    node_id: &NodeId,
) -> Result<Vec<(String, Option<String>, NodeId)>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("nid".to_string(), DataValue::Str(node_id.clone().into()));

    // Any non-local dst_store is a soft link; the exact cloud is parsed from
    // the value below (`cloud` → default, `cloud:<name>` → named).
    let script = r#"
        ?[edge_type, dst_store, dst] := *edge{src, dst, edge_type, dst_store @ 'NOW'},
                                        src = $nid, dst_store != 'local'
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let links = rows
        .rows
        .iter()
        .filter_map(|row| {
            let edge_type = row.first().and_then(|v| v.get_str())?.to_string();
            let dst_store = row.get(1).and_then(|v| v.get_str())?;
            let dst = row.get(2).and_then(|v| v.get_str())?.to_string();
            let cloud_name = dst_store
                .strip_prefix("cloud:")
                .map(|n| n.to_string())
                .filter(|n| !n.is_empty());
            Some((edge_type, cloud_name, dst))
        })
        .collect();
    Ok(links)
}
