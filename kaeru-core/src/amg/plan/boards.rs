//! Validate the merged board vocabulary across every affected membership.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use super::super::{storage as db, validate::invalid};
use super::Plan;
use crate::DEFAULT_STATUSES;
use crate::models::amg::AmgNode;
use crate::{Result, Store};

fn board_keys(node: &AmgNode) -> Result<BTreeSet<String>> {
    let statuses = node
        .properties
        .get("statuses")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid("board lacks status vocabulary"))?;
    let mut keys = BTreeSet::new();
    for status in statuses {
        let key = status
            .get("key")
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| invalid("invalid board status"))?;
        if !keys.insert(key.into())
            || status
                .get("label")
                .and_then(Value::as_str)
                .is_none_or(str::is_empty)
        {
            return Err(invalid("invalid or duplicate board status"));
        }
    }
    if keys.is_empty() {
        return Err(invalid("empty board vocabulary"));
    }
    Ok(keys)
}
pub(super) fn validate_board_merge(store: &Store, plan: &Plan) -> Result<()> {
    let memberships = db::memberships(store, false)?;
    let affected = plan
        .nodes
        .iter()
        .flat_map(|n| n.value.initiatives.iter().cloned())
        .collect::<BTreeSet<_>>();
    let mut ids = BTreeSet::new();
    for row in db::query(
        store,
        "?[id] := *node{id,type @ 'NOW'},type in ['board','task']",
        BTreeMap::new(),
    )?
    .rows
    {
        let id = db::string(&row[0])?;
        if memberships
            .get(&id)
            .is_some_and(|names| names.iter().any(|n| affected.contains(n)))
        {
            ids.insert(id);
        }
    }
    let mut nodes = db::nodes(store, &ids)?;
    for change in &plan.nodes {
        nodes.insert(change.value.id.clone(), change.value.clone());
    }
    let mut boards = BTreeMap::new();
    for node in nodes.values().filter(|n| n.node_type == "board") {
        let keys = board_keys(node)?;
        for name in &node.initiatives {
            if !affected.contains(name) {
                continue;
            }
            if boards
                .insert(name.clone(), (node.id.clone(), keys.clone()))
                .is_some_and(|(id, _)| id != node.id)
            {
                return Err(invalid(format!(
                    "board identity conflicts in initiative {name}"
                )));
            }
        }
    }
    for node in nodes.values().filter(|n| n.node_type == "task") {
        let statuses = node
            .tags
            .iter()
            .filter_map(|tag| tag.strip_prefix("status:"))
            .collect::<Vec<_>>();
        if statuses.len() > 1 {
            return Err(invalid("task has multiple current statuses"));
        }
        if let Some(status) = statuses.first() {
            for name in &node.initiatives {
                if !affected.contains(name) {
                    continue;
                }
                let allowed = boards.get(name).map_or_else(
                    || DEFAULT_STATUSES.iter().any(|(key, _)| key == status),
                    |(_, keys)| keys.contains(*status),
                );
                if !allowed {
                    return Err(invalid(format!(
                        "task status is incompatible with destination board in {name}"
                    )));
                }
            }
        }
    }
    Ok(())
}
