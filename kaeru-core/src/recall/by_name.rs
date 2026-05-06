//! Explicit lookup by `name`, plus a `count_by_type` helper used by tests
//! and lint diagnostics. Both are simple `*node`-anchored-at-NOW reads.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;

use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

use super::NodeBrief;
use super::parse_brief;

/// Looks up a node id by its `name` at the current moment.
/// Returns `None` if no node matches.
///
/// If the store has a `current_initiative` set (via
/// [`Store::use_initiative`]), the lookup is constrained to nodes
/// attached to that initiative through the `node_initiative` junction.
/// Otherwise the search is cross-initiative.
pub fn recall_id_by_name(store: &Store, name: &str) -> Result<Option<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("name".to_string(), DataValue::Str(name.into()));

    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
                ?[id] := *node{id, name @ 'NOW'}, name = $name,
                         *node_initiative{initiative, node_id: id},
                         initiative = $init
            "#
        }
        None => {
            r#"
                ?[id] := *node{id, name @ 'NOW'}, name = $name
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
        ?[id, type, name, body] := *node{id, type, name, body @ 'NOW'}, id = $id
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
