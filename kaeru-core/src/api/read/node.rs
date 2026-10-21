//! Bounded retrieval with currentness and stored reasoning on the ordinary path.

use serde_json::{Value, json};

use super::session::{
    created, edge_rows, memberships, pin, raw_node, recorded, relations as cached_relations,
};
use super::time_string;
use crate::Store;
use crate::api::access::{check_node, check_scope};
use crate::api::{ApiOptions, ApiResult, hash, issue_from_error};
use crate::config::api::API_EXCERPT_CHARS;
use crate::models::{ContentRange, Entity, Issue, NodeData, RelationData};

pub(in crate::api) fn node(
    store: &Store,
    id: &str,
    options: &ApiOptions,
    full: bool,
) -> ApiResult<NodeData> {
    check_node(store, id, None, options, false)?;
    let n = raw_node(store, id)
        .map_err(issue_from_error)?
        .ok_or_else(|| Issue::new("NOT_FOUND", "node is not live"))?;
    let recorded_at = recorded(store, id)?;
    let edges = relations(store, id, options)?;
    let pin = pin(store, id)?;
    let revision = hash(&json!([
        n.id,
        n.node_type,
        n.tier,
        n.name,
        n.body,
        n.tags,
        n.visibility,
        n.layer,
        n.properties,
        recorded_at,
        memberships(store, id).map_err(issue_from_error)?,
        edges,
        pin
    ]));
    let successors: Vec<_> = edges
        .iter()
        .filter_map(|e| {
            if e.edge_type == "supersedes" && e.to == id {
                Some(e.from.clone())
            } else if e.edge_type == "consolidated_to" && e.from == id {
                Some(e.to.clone())
            } else {
                None
            }
        })
        .collect();
    let status = n
        .tags
        .iter()
        .find_map(|t| t.strip_prefix("status:").map(str::to_string));
    let currentness = if !successors.is_empty() {
        "superseded"
    } else if status.as_deref() == Some("refuted") {
        "refuted"
    } else if edges
        .iter()
        .any(|e| e.to == id && matches!(e.edge_type.as_str(), "contradicts" | "falsifies"))
    {
        "contested"
    } else {
        "current"
    };
    let mut props = n.properties.clone();
    let mut body = n.body.clone();
    if let Some(source) = props.as_mut().and_then(|p| p.get_mut("source")) {
        if full && let Some(text) = source.get("original").and_then(Value::as_str) {
            body = Some(text.into());
        }
        if let Some(o) = source.as_object_mut() {
            o.remove("original");
        }
    }
    filter_references(store, &mut props, options);
    let body_range = if !full {
        body.as_ref()
            .filter(|b| b.chars().count() > API_EXCERPT_CHARS)
            .map(|b| ContentRange {
                start: 0,
                end: API_EXCERPT_CHARS,
                total: b.chars().count(),
            })
    } else {
        None
    };
    if !full {
        body = body.map(|b| b.chars().take(API_EXCERPT_CHARS).collect());
    }
    Ok(NodeData {
        initiatives: memberships(store, id)
            .map_err(issue_from_error)?
            .into_iter()
            .filter(|s| check_scope(s, options).is_ok())
            .collect(),
        id: id.into(),
        revision,
        node_type: n.node_type,
        tier: n.tier,
        name: n.name,
        body,
        tags: n.tags,
        layer: n.layer,
        visibility: n.visibility,
        properties: props,
        currentness: currentness.into(),
        successors,
        status,
        recorded_at: recorded_at.map(time_string),
        created_at: created_at(store, id)?,
        metadata_is_current: true,
        body_range,
    })
}

pub(in crate::api) fn created_at(store: &Store, id: &str) -> ApiResult<Option<String>> {
    Ok(created(store, id)?.map(time_string))
}

pub(in crate::api) fn entity(store: &Store, id: &str, options: &ApiOptions) -> ApiResult<Entity> {
    let n = node(store, id, options, false)?;
    Ok(Entity {
        id: n.id,
        revision: n.revision,
    })
}

pub(in crate::api) fn relations(
    store: &Store,
    id: &str,
    options: &ApiOptions,
) -> ApiResult<Vec<RelationData>> {
    cached_relations(store, id, options, || relations_inner(store, id, options))
}

fn relations_inner(store: &Store, id: &str, options: &ApiOptions) -> ApiResult<Vec<RelationData>> {
    let rows = edge_rows(store, id)?;
    Ok(rows
        .iter()
        .filter_map(|r| {
            let from = r[0].get_str()?.to_string();
            let to = r[1].get_str()?.to_string();
            let kind = r[2].get_str()?.to_string();
            let historical = matches!(kind.as_str(), "supersedes" | "consolidated_to");
            if check_node(store, &from, None, options, historical).is_err() {
                return None;
            }
            let dst_store = r[5].get_str().unwrap_or("local").to_string();
            if dst_store != "local" && options.allowed_initiatives.is_some() {
                return None;
            }
            if dst_store == "local" && check_node(store, &to, None, options, historical).is_err() {
                return None;
            }
            Some(RelationData {
                revision: hash(&json!([
                    from,
                    to,
                    kind,
                    r[3].get_float(),
                    Value::from(r[4].clone()),
                    dst_store
                ])),
                from,
                to,
                edge_type: kind,
                weight: r[3].get_float().unwrap_or(1.0),
                properties: Value::from(r[4].clone()),
                dst_store,
            })
        })
        .collect())
}

pub(in crate::api) fn filter_references(
    store: &Store,
    props: &mut Option<Value>,
    options: &ApiOptions,
) {
    if let Some(task) = props
        .as_ref()
        .and_then(|p| p.pointer("/applicability/task_id"))
        .and_then(Value::as_str)
        && check_node(store, task, None, options, false).is_err()
        && let Some(applicability) = props
            .as_mut()
            .and_then(|p| p.get_mut("applicability"))
            .and_then(Value::as_object_mut)
    {
        applicability.remove("task_id");
        applicability.insert("task_accessible".into(), json!(false));
    }
}
