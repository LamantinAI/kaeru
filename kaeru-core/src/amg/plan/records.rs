//! Effective record fields and identity checks.

use std::collections::{BTreeMap, BTreeSet};

use cozo::DataValue;
use serde_json::Value;

use super::super::{storage as db, validate::invalid};
use crate::graph::temporal::parse_validity;
use crate::models::amg::{AmgEdge, AmgNode};
use crate::{Error, Result, Store};

pub(super) fn conflict(kind: &str, id: &str) -> Error {
    invalid(format!(
        "import conflict for {kind} {id}; use force to accept the supplied current version"
    ))
}
fn source_equal(a: &AmgNode, b: &AmgNode) -> bool {
    a.properties.get("source") == b.properties.get("source")
}
pub(super) fn effective(
    node: &AmgNode,
    before: Option<&AmgNode>,
    memberships: &[String],
) -> AmgNode {
    let mut out = node.clone();
    out.visibility = before.map_or("local", |n| n.visibility.as_str()).into();
    out.initiatives.extend(memberships.iter().cloned());
    out.initiatives.sort();
    out.initiatives.dedup();
    if node.node_type == "reference"
        && out
            .properties
            .pointer("/source/origin")
            .and_then(Value::as_str)
            == Some("verified_artifact")
        && before.is_none_or(|old| !source_equal(node, old))
    {
        out.properties["source"]["origin"] = Value::String("imported_artifact".into());
    }
    if let Some(old) = before {
        let definition = |n: &AmgNode| {
            n.tags
                .iter()
                .filter(|t| t.starts_with("after:") || t.starts_with("for:"))
                .cloned()
                .collect::<BTreeSet<_>>()
        };
        if definition(old) == definition(node) {
            for tag in old.tags.iter().filter(|t| t.starts_with("seen:")) {
                if !out.tags.contains(tag) {
                    out.tags.push(tag.clone());
                }
            }
        }
    }
    out
}
pub(super) fn node_equal(a: &AmgNode, b: &AmgNode) -> bool {
    a.node_type == b.node_type
        && a.tier == b.tier
        && a.name == b.name
        && a.body == b.body
        && a.tags.iter().collect::<BTreeSet<_>>() == b.tags.iter().collect::<BTreeSet<_>>()
        && a.properties == b.properties
        && a.layer == b.layer
        && a.visibility == b.visibility
}
pub(super) fn edge_equal(a: &AmgEdge, b: &AmgEdge) -> bool {
    a.weight == b.weight && a.properties == b.properties
}
pub(super) fn history_type(store: &Store, id: &str) -> Result<Option<String>> {
    let mut latest = None;
    for row in db::query(
        store,
        "?[type,validity] := *node{id,type,validity},id=$id",
        BTreeMap::from([("id".into(), DataValue::Str(id.into()))]),
    )?
    .rows
    {
        let (time, asserted) = parse_validity(row.get(1))?;
        if asserted && latest.as_ref().is_none_or(|(old, _)| time > *old) {
            latest = Some((time, db::string(&row[0])?));
        }
    }
    Ok(latest.map(|(_, kind)| kind))
}
