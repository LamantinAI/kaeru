//! Initiative discovery — `list_initiatives` returns the distinct set
//! of initiative names the substrate has seen at least one node attached
//! to. Mutations populate `node_initiative` automatically when the
//! `Store` has a `current_initiative` set.

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};

use super::{NodeBrief, parse_brief};
use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

/// Returns every initiative name that has at least one node attached
/// through the `node_initiative` junction. Sorted alphabetically.
///
/// Datalog rule-head deduplication produces distinct names; ordering is
/// applied at projection time so CLI output is stable.
pub fn list_initiatives(store: &Store) -> Result<Vec<String>> {
    let script = r#"
        ?[initiative] := *node_initiative{initiative, node_id}
        :order initiative
    "#;
    let rows = store.run_read(script)?;
    let names: Vec<String> = rows
        .rows
        .iter()
        .filter_map(|row| row.first().and_then(|v| v.get_str()).map(String::from))
        .collect();
    Ok(names)
}

/// Returns briefs for every node attached to `initiative` at NOW, with an
/// **explicit** initiative argument (not `Store::current_initiative`), so
/// it is safe to call concurrently from a multi-request server. Audit-event
/// nodes are excluded — they are operational noise, not shareable content.
pub fn nodes_in_initiative(store: &Store, initiative: &str) -> Result<Vec<NodeBrief>> {
    let excerpt_chars = store.config().body_excerpt_chars;
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));

    let script = r#"
        ?[id, type, name, body, validity] := *node_initiative{initiative, node_id: id},
                                   initiative = $init,
                                   *node{id, type, name, body, validity @ 'NOW'},
                                   type != 'audit_event'
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let briefs = rows
        .rows
        .iter()
        .map(|row| parse_brief(row.as_slice(), excerpt_chars))
        .collect();
    Ok(briefs)
}

/// Counts non-audit nodes attached to `initiative` at NOW. A cheap `COUNT`
/// (no body loads, unlike `nodes_in_initiative`) — used by the capture nudge
/// to tell whether the initiative already holds anything worth linking a
/// fresh node to.
pub fn count_nodes_in_initiative(store: &Store, initiative: &str) -> Result<usize> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));

    let script = r#"
        ?[count(id)] := *node_initiative{initiative, node_id: id},
                        initiative = $init,
                        *node{id, type @ 'NOW'},
                        type != 'audit_event'
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let count = rows
        .rows
        .first()
        .and_then(|row| row.first())
        .and_then(|v| v.get_int())
        .unwrap_or(0);
    Ok(count as usize)
}

/// Returns every `local` edge whose **both** endpoints are attached to
/// `initiative` at NOW, as `(src, dst, edge_type)`. Explicit initiative
/// argument (not `Store::current_initiative`) for concurrency safety. The
/// cloud serves this so a puller can rebuild the graph structure among
/// the nodes it materialises. Mirrors `export`'s both-endpoints scoping.
pub fn edges_in_initiative(
    store: &Store,
    initiative: &str,
) -> Result<Vec<(NodeId, NodeId, String, f64)>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));

    let script = r#"
        ?[src, dst, edge_type, weight] :=
            *edge{src, dst, edge_type, weight, dst_store @ 'NOW'},
            dst_store = 'local',
            *node_initiative{initiative, node_id: src},
            initiative = $init,
            *node_initiative{initiative: i2, node_id: dst},
            i2 = $init
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
