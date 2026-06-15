//! Initiative discovery — `list_initiatives` returns the distinct set
//! of initiative names the substrate has seen at least one node attached
//! to. Mutations populate `node_initiative` automatically when the
//! `Store` has a `current_initiative` set.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;

use crate::errors::Result;
use crate::store::Store;

use super::NodeBrief;
use super::parse_brief;

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
        ?[id, type, name, body] := *node_initiative{initiative, node_id: id},
                                   initiative = $init,
                                   *node{id, type, name, body @ 'NOW'},
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
