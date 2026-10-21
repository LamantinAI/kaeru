//! Bounded retrieval with currentness and stored reasoning on the ordinary path.

use std::collections::BTreeMap;

use chrono::DateTime;
use cozo::{DataValue, ScriptMutability};
use serde_json::{Value, json};

use super::session::{memberships, raw_node};
use super::{created_at, filter_references, node, relations, time_string, trails};
use crate::api::access::{check_node, check_scope, resolve};
use crate::api::graph::{path, provenance};
use crate::api::{ApiOptions, ApiResult, Args, hash, issue_from_error};
use crate::graph::temporal::parse_validity;
use crate::models::{Issue, NodeData, NodeRef, Omission, RecordData, Response, ResponseData};
use crate::{Error, Store, at, history as core_history};

pub(in crate::api) fn historical_node(
    store: &Store,
    id: &str,
    when: f64,
    options: &ApiOptions,
) -> ApiResult<NodeData> {
    check_node(store, id, None, options, true)?;
    let snap = at(store, &id.into(), when)
        .map_err(issue_from_error)?
        .ok_or_else(|| Issue::new("NOT_FOUND_AT_TIME", "node had no assertion at that instant"))?;
    let metadata=store.db_ref().run_script("?[validity,properties,layer,visibility] := *node{id:$id,validity,properties,layer,visibility}",BTreeMap::from([("id".into(),DataValue::Str(id.into()))]),ScriptMutability::Immutable).map_err(Error::from).map_err(issue_from_error)?;
    let latest = metadata
        .rows
        .iter()
        .filter_map(|r| {
            parse_validity(r.first())
                .ok()
                .filter(|(_, asserted)| *asserted)
                .map(|(time, _)| (time, r))
        })
        .max_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, r)| r);
    let mut props = latest.map(|r| Value::from(r[1].clone()));
    let mut body = snap.body;
    if let Some(source) = props.as_mut().and_then(|p| p.get_mut("source")) {
        if let Some(original) = source.get("original").and_then(Value::as_str) {
            body = Some(original.into());
        }
        if let Some(p) = source.as_object_mut() {
            p.remove("original");
        }
    }
    filter_references(store, &mut props, options);
    let status = snap
        .tags
        .iter()
        .find_map(|t| t.strip_prefix("status:").map(str::to_string));
    Ok(NodeData {
        initiatives: memberships(store, &id.to_string())
            .map_err(issue_from_error)?
            .into_iter()
            .filter(|s| check_scope(s, options).is_ok())
            .collect(),
        id: id.into(),
        revision: hash(&json!([id, when, snap.name, body])),
        node_type: snap.node_type,
        tier: snap.tier,
        name: snap.name,
        body,
        tags: snap.tags,
        layer: latest
            .and_then(|r| r[2].get_str())
            .unwrap_or(&snap.layer)
            .into(),
        visibility: latest
            .and_then(|r| r[3].get_str())
            .unwrap_or(&snap.visibility)
            .into(),
        properties: props,
        currentness: "historical".into(),
        successors: vec![],
        status,
        recorded_at: snap.ts.map(time_string),
        created_at: created_at(store, id)?,
        metadata_is_current: true,
        body_range: None,
    })
}

pub(in crate::api) fn read(
    store: &Store,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    let when = args
        .text("recorded_at")
        .map(|t| {
            DateTime::parse_from_rfc3339(&t)
                .map(|d| d.timestamp() as f64)
                .map_err(|e| Issue::new("INVALID_ARGUMENT", e.to_string()))
        })
        .transpose()?;
    let id = resolve(
        store,
        &args.parse::<NodeRef>("node")?,
        options,
        when,
        true,
        false,
    )?;
    let scope = store.current_initiative().into_iter().collect();
    if let Some(when) = when {
        let n = historical_node(store, &id, when, options)?;
        return Ok(Response::read(
            "read",
            ResponseData::Record(
                RecordData {
                    node: n,
                    conflicts: vec![],
                    reasoning: vec![],
                    relations: vec![],
                    provenance: vec![],
                    path: vec![],
                }
                .into(),
            ),
            scope,
        ));
    }
    if raw_node(store, &id).map_err(issue_from_error)?.is_none() {
        let latest = core_history(store, &id)
            .map_err(issue_from_error)?
            .into_iter()
            .filter(|r| r.asserted)
            .map(|r| r.seconds)
            .max_by(f64::total_cmp)
            .ok_or_else(|| Issue::new("NOT_FOUND", "no recorded assertion exists"))?;
        let mut n = historical_node(store, &id, latest, options)?;
        let edges = relations(store, &id, options)?;
        n.successors = edges
            .iter()
            .filter_map(|e| {
                if e.to == id && e.edge_type == "supersedes" {
                    Some(e.from.clone())
                } else if e.from == id && e.edge_type == "consolidated_to" {
                    Some(e.to.clone())
                } else {
                    None
                }
            })
            .collect();
        n.currentness = if n.successors.is_empty() {
            "retired"
        } else {
            "superseded"
        }
        .into();
        let mut response = Response::read(
            "read",
            ResponseData::Record(
                RecordData {
                    node: n,
                    conflicts: vec![],
                    reasoning: vec![],
                    relations: edges,
                    provenance: vec![],
                    path: vec![],
                }
                .into(),
            ),
            scope,
        );
        response.issues.push(Issue::new(
            "RETIRED_RECORD",
            "content is the last recorded assertion; inspect accessible successors and history",
        ));
        return Ok(response);
    }
    let include: Vec<String> = args.optional("include")?.unwrap_or_default();
    let edges = relations(store, &id, options)?;
    let conflicts = edges
        .iter()
        .filter(|e| e.to == id && matches!(e.edge_type.as_str(), "contradicts" | "falsifies"))
        .cloned()
        .collect();
    let to = args
        .optional::<NodeRef>("to")?
        .map(|n| resolve(store, &n, options, None, false, true))
        .transpose()?;
    let filter: Vec<String> = args.optional("edge_types")?.unwrap_or_default();
    let selected = if include.iter().any(|i| i == "relations") {
        edges
            .into_iter()
            .filter(|e| to.as_ref().is_none_or(|to| e.from == *to || e.to == *to))
            .filter(|e| filter.is_empty() || filter.contains(&e.edge_type))
            .collect()
    } else {
        vec![]
    };
    let (provenance, provenance_truncated) = if include.iter().any(|i| i == "provenance") {
        let (ids, truncated) = provenance(store, &id, options)?;
        (
            ids.into_iter()
                .filter_map(|id| node(store, &id, options, false).ok())
                .collect(),
            truncated,
        )
    } else {
        (vec![], false)
    };
    let path = if include.iter().any(|i| i == "path") {
        path(store, &id, &to.unwrap(), options)?
    } else {
        vec![]
    };
    let expanded = include.iter().any(|i| i == "reasoning");
    let mut reasoning = trails(store, &id, options, expanded)?;
    let omitted = reasoning
        .len()
        .saturating_sub(if expanded { 50 } else { 3 });
    reasoning.truncate(if expanded { 50 } else { 3 });
    let mut response = Response::read(
        "read",
        ResponseData::Record(
            RecordData {
                node: node(store, &id, options, true)?,
                conflicts,
                reasoning,
                relations: selected,
                provenance,
                path,
            }
            .into(),
        ),
        scope,
    );
    if provenance_truncated {
        let c = response.coverage.as_mut().unwrap();
        c.complete = false;
        c.omitted.push(Omission {
            section: "provenance".into(),
            count: 1,
            reason: "configured_hop_cap".into(),
        });
    }
    if include.iter().any(|i| i == "path")
        && matches!(&response.data,ResponseData::Record(d) if d.path.is_empty())
    {
        response.issues.push(Issue::new(
            "NO_PATH",
            "no accessible directed path within the configured hop cap",
        ));
    }
    if omitted > 0 {
        let c = response.coverage.as_mut().unwrap();
        c.complete = false;
        c.omitted.push(Omission {
            section: "reasoning".into(),
            count: omitted,
            reason: "saved_trail_cap; read returned trails individually".into(),
        });
    }
    Ok(response)
}
