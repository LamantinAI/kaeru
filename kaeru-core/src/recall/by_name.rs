//! Explicit lookup by `name`, plus a `count_by_type` helper used by tests
//! and lint diagnostics. Both are simple `*node`-anchored-at-NOW reads.

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};

use super::{NodeBrief, NodeFull, parse_brief};
use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

/// Looks up a node id by its `name` at the current moment.
/// Returns `None` if no node matches.
///
/// If the store has a `current_initiative` set (via
/// [`Store::use_initiative`]), the lookup is constrained to nodes
/// attached to that initiative through the `node_initiative` junction.
/// Otherwise the search is cross-initiative.
///
/// When several distinct nodes share the same name, the **newest
/// assertion wins** — `:order validity` returns newest-first because
/// Cozo wraps the timestamp in `Reverse<>`.
pub fn recall_id_by_name(store: &Store, name: &str) -> Result<Option<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("name".to_string(), DataValue::Str(name.into()));

    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
                ?[id, validity] := *node{id, validity, name @ 'NOW'},
                                    name = $name,
                                    *node_initiative{initiative, node_id: id},
                                    initiative = $init
                :order validity
                :limit 1
            "#
        }
        None => {
            r#"
                ?[id, validity] := *node{id, validity, name @ 'NOW'}, name = $name
                :order validity
                :limit 1
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let id = rows
        .rows
        .first()
        .and_then(|row| row.first())
        .and_then(|v| v.get_str())
        .map(String::from);
    Ok(id)
}

/// Returns a [`NodeBrief`] for `id` at NOW, or `None` if the node is
/// not currently asserted. Useful for CLI / display code that holds an
/// id and needs the human-readable name + excerpt.
pub fn node_brief_by_id(store: &Store, id: &NodeId) -> Result<Option<NodeBrief>> {
    let excerpt_chars = store.config().body_excerpt_chars;
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));

    let script = r#"
        ?[id, type, name, body, validity] := *node{id, type, name, body, validity @ 'NOW'}, id = $id
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let brief = rows
        .rows
        .first()
        .map(|row| parse_brief(row.as_slice(), excerpt_chars));
    Ok(brief)
}

/// Reads the **full** node record for `id` at NOW (untruncated body,
/// tier, tags, visibility), or `None` if not currently asserted. Used by
/// the cloud adapter, which needs every field to push a shared node and to
/// materialise one on pull — `node_brief_by_id` truncates the body and
/// omits tier/tags.
pub fn read_node_full(store: &Store, id: &NodeId) -> Result<Option<NodeFull>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));

    let script = r#"
        ?[type, tier, name, body, tags, visibility, layer] :=
            *node{id, type, tier, name, body, tags, visibility, layer @ 'NOW'}, id = $id
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let Some(row) = rows.rows.first() else {
        return Ok(None);
    };
    let node_type = row
        .first()
        .and_then(|v| v.get_str())
        .map(String::from)
        .unwrap_or_default();
    let tier = row
        .get(1)
        .and_then(|v| v.get_str())
        .map(String::from)
        .unwrap_or_default();
    let name = row
        .get(2)
        .and_then(|v| v.get_str())
        .map(String::from)
        .unwrap_or_default();
    let body = row.get(3).and_then(|v| v.get_str()).map(String::from);
    let tags = row.get(4).map(extract_string_list).unwrap_or_default();
    let visibility = row
        .get(5)
        .and_then(|v| v.get_str())
        .map(String::from)
        .unwrap_or_else(|| "local".to_string());
    let layer = row
        .get(6)
        .and_then(|v| v.get_str())
        .map(String::from)
        .unwrap_or_else(|| "warm".to_string());

    Ok(Some(NodeFull {
        id: id.clone(),
        node_type,
        tier,
        name,
        body,
        tags,
        visibility,
        layer,
    }))
}

/// Returns the **full** records of every `visibility = local` node in
/// `initiative` at NOW (explicit initiative, audit nodes excluded). This is
/// the sync-review work-list: `local` is exactly "not yet shared", so it
/// doubles as the since-last-sync marker — no separate watermark needed.
pub fn local_nodes_for_review(store: &Store, initiative: &str) -> Result<Vec<NodeFull>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));

    let script = r#"
        ?[id, type, tier, name, body, tags, visibility, layer] :=
            *node_initiative{initiative, node_id: id}, initiative = $init,
            *node{id, type, tier, name, body, tags, visibility, layer @ 'NOW'},
            visibility = 'local', type != 'audit_event'
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let nodes = rows
        .rows
        .iter()
        .map(|row| {
            let id = row
                .first()
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_default();
            let node_type = row
                .get(1)
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_default();
            let tier = row
                .get(2)
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_default();
            let name = row
                .get(3)
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_default();
            let body = row.get(4).and_then(|v| v.get_str()).map(String::from);
            let tags = row.get(5).map(extract_string_list).unwrap_or_default();
            let visibility = row
                .get(6)
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_else(|| "local".to_string());
            let layer = row
                .get(7)
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_else(|| "warm".to_string());
            NodeFull {
                id,
                node_type,
                tier,
                name,
                body,
                tags,
                visibility,
                layer,
            }
        })
        .collect();
    Ok(nodes)
}

/// Extracts a `Vec<String>` from a Cozo list column value; non-list
/// (e.g. `null`) yields an empty vec.
fn extract_string_list(v: &DataValue) -> Vec<String> {
    match v {
        DataValue::List(items) => items
            .iter()
            .filter_map(|x| x.get_str().map(String::from))
            .collect(),
        _ => Vec::new(),
    }
}

/// Counts nodes of a given type at the current moment.
/// Useful for tests and lint diagnostics.
pub fn count_by_type(store: &Store, node_type: &str) -> Result<usize> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("nt".to_string(), DataValue::Str(node_type.into()));

    let script = r#"
        ?[count(id)] := *node{id, type @ 'NOW'}, type = $nt
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
