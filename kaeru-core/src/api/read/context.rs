//! Bounded retrieval with currentness and stored reasoning on the ordinary path.

use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use cozo::{DataValue, ScriptMutability};
use serde_json::{Value, json};

use super::session::prefetch;
use super::{historical_node, node};
use crate::api::access::{check_node, check_scope};
use crate::api::control::checkpoint;
use crate::api::write::slot_revision;
use crate::api::{API_VERSION, ApiOptions, ApiResult, Args, issue_from_error};
use crate::config::api::{API_EXCERPT_CHARS, API_READ_BATCH_SIZE};
use crate::models::{
    ContentRange, ContextData, Issue, NodeData, Omission, Response, ResponseData, Section,
    SlotChange,
};
use crate::recall::open_work::reminder_schedule;
use crate::{
    BoardStatus, Error, NodeType, Store, board_node_id, effective_statuses_at, read_node_full,
    slots_in, under_review_pinned, version,
};

pub(in crate::api) fn scope_nodes(
    store: &Store,
    scope: &str,
    options: &ApiOptions,
) -> ApiResult<Vec<NodeData>> {
    check_scope(scope, options)?;
    let rows = store
        .db_ref()
        .run_script(
            "?[id] := initiative=$scope,*node_initiative{initiative,node_id:id}",
            BTreeMap::from([("scope".into(), DataValue::Str(scope.into()))]),
            ScriptMutability::Immutable,
        )
        .map_err(Error::from)
        .map_err(issue_from_error)?;
    let ids = rows
        .rows
        .iter()
        .filter_map(|r| r.first().and_then(DataValue::get_str).map(str::to_string))
        .collect::<Vec<_>>();
    let mut nodes = vec![];
    for batch in ids.chunks(API_READ_BATCH_SIZE) {
        checkpoint()?;
        prefetch(store, batch, options)?;
        nodes.extend(
            batch
                .iter()
                .filter_map(|id| node(store, id, options, false).ok()),
        );
        checkpoint()?;
    }
    nodes.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(nodes)
}

pub(in crate::api) fn context(
    store: &Store,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    let view = args.text("view").unwrap_or_else(|| "work".into());
    if view == "diagnostics" {
        let section = Section {
            name: "diagnostics".into(),
            records: vec![],
            details: json!({"version":version(),"api_version":API_VERSION,"tools":35,"temporal_axis":"recorded_time","import":"Capture scoped records by their meaning; link important relationships. No bulk byte importer."}),
        };
        return Ok(Response::read(
            "context",
            ResponseData::Context(ContextData {
                proposals: vec![],
                sections: vec![section],
                slots: vec![],
            }),
            store.current_initiative().into_iter().collect(),
        ));
    }
    let scope = args.required_text("initiative")?;
    if view == "board" {
        let at = args
            .text("recorded_at")
            .map(|t| {
                DateTime::parse_from_rfc3339(&t)
                    .map(|d| d.timestamp() as f64)
                    .map_err(|e| Issue::new("INVALID_ARGUMENT", e.to_string()))
            })
            .transpose()?;
        let statuses = accessible_statuses(store, &scope, at, options)?
            .ok_or_else(|| Issue::new("NOT_FOUND", "board is not accessible"))?;
        let columns = statuses
            .iter()
            .map(|s| json!({"key":s.key,"label":s.label,"terminal":s.terminal}))
            .collect::<Vec<_>>();
        let rows = store
            .db_ref()
            .run_script(
                "?[id] := initiative=$scope,*node_initiative{initiative,node_id:id}",
                BTreeMap::from([("scope".into(), DataValue::Str(scope.clone().into()))]),
                ScriptMutability::Immutable,
            )
            .map_err(Error::from)
            .map_err(issue_from_error)?;
        let mut records = vec![];
        for id in rows
            .rows
            .iter()
            .filter_map(|r| r.first().and_then(DataValue::get_str))
        {
            if check_node(store, id, Some(&scope), options, at.is_some()).is_err() {
                continue;
            }
            let record = if let Some(at) = at {
                historical_node(store, id, at, options)
            } else {
                node(store, id, options, false)
            };
            if let Ok(mut record) = record
                && record.node_type == "task"
            {
                if at.is_some()
                    && let Some(body) = record.body.as_mut()
                    && body.chars().count() > API_EXCERPT_CHARS
                {
                    let total = body.chars().count();
                    *body = body.chars().take(API_EXCERPT_CHARS).collect();
                    record.body_range = Some(ContentRange {
                        start: 0,
                        end: API_EXCERPT_CHARS,
                        total,
                    });
                }
                records.push(record);
            }
        }
        return Ok(Response::read(
            "context",
            ResponseData::Context(ContextData {
                proposals: vec![],
                sections: vec![Section {
                    name: "board".into(),
                    records,
                    details: json!({"columns":columns,"recorded_at":args.text("recorded_at")}),
                }],
                slots: vec![],
            }),
            vec![scope],
        ));
    }
    let nodes = scope_nodes(store, &scope, options)?;
    if view == "map" {
        let mut sections = vec![];
        for kind in NodeType::VALID {
            let records = nodes
                .iter()
                .filter(|n| n.node_type == kind)
                .cloned()
                .collect();
            sections.push(Section {
                name: kind.into(),
                records,
                details: Value::Null,
            });
        }
        return Ok(Response::read(
            "context",
            ResponseData::Context(ContextData {
                proposals: vec![],
                sections,
                slots: vec![],
            }),
            vec![scope],
        ));
    }
    let mut sections = vec![];
    let rules = nodes
        .iter()
        .filter(|n| {
            n.layer == "core"
                || n.properties
                    .as_ref()
                    .and_then(|p| p.get("kind"))
                    .and_then(Value::as_str)
                    == Some("constraint")
        })
        .filter(|n| n.currentness != "superseded")
        .filter(|n| constraint_active(store, n, options))
        .cloned()
        .collect();
    sections.push(Section {
        name: "rules".into(),
        records: rules,
        details: Value::Null,
    });
    let registry = accessible_statuses(store, &scope, None, options)?;
    let completion_known = registry.is_some();
    let statuses = registry.unwrap_or_default();
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let mut tasks = nodes
        .iter()
        .filter(|n| {
            n.node_type == "task"
                && !statuses
                    .iter()
                    .any(|s| s.terminal && n.status.as_deref() == Some(s.key.as_str()))
        })
        .cloned()
        .collect::<Vec<_>>();
    tasks.sort_by_key(|n| {
        (
            n.tags
                .iter()
                .find_map(|t| t.strip_prefix("due:"))
                .unwrap_or("9999-12-31")
                .to_string(),
            n.id.clone(),
        )
    });
    let overdue = tasks
        .iter()
        .filter(|n| {
            n.tags
                .iter()
                .find_map(|t| t.strip_prefix("due:"))
                .is_some_and(|d| d < today.as_str())
        })
        .map(|n| n.id.clone())
        .collect::<Vec<_>>();
    sections.push(Section {
        name: "open_tasks".into(),
        records: tasks,
        details: json!({"overdue":overdue,"completion_known":completion_known}),
    });
    sections.push(Section {
        name: "open_claims".into(),
        records: nodes
            .iter()
            .filter(|n| n.node_type == "hypothesis" && n.status.as_deref() == Some("open"))
            .cloned()
            .collect(),
        details: Value::Null,
    });
    let reviews = under_review_pinned(store).map_err(issue_from_error)?;
    sections.push(Section {
        name: "reviews".into(),
        records: reviews
            .into_iter()
            .filter_map(|id| node(store, &id, options, false).ok())
            .collect(),
        details: Value::Null,
    });
    let reminders = nodes
        .iter()
        .filter(|n| {
            reminder_schedule(
                &n.tags.iter().map(String::as_str).collect::<Vec<_>>(),
                &today,
            )
            .is_some()
        })
        .cloned()
        .collect();
    sections.push(Section {
        name: "reminders".into(),
        records: reminders,
        details: Value::Null,
    });
    let pins=store.db_ref().run_script("?[id,pinned_at] := initiative=$scope,*node_initiative{initiative,node_id:id},*session_pin{node_id:id,pinned_at} :order -pinned_at",BTreeMap::from([("scope".into(),DataValue::Str(scope.clone().into()))]),ScriptMutability::Immutable).map_err(Error::from).map_err(issue_from_error)?;
    sections.push(Section {
        name: "pinned".into(),
        records: pins
            .rows
            .iter()
            .filter_map(|r| r.first().and_then(DataValue::get_str))
            .filter_map(|id| nodes.iter().find(|n| n.id == id).cloned())
            .collect(),
        details: Value::Null,
    });
    sections.push(Section {
        name: "ongoing_work".into(),
        records: nodes
            .iter()
            .filter(|n| {
                n.tier == "operational"
                    && matches!(
                        n.node_type.as_str(),
                        "roadmap" | "checklist" | "draft" | "scratch"
                    )
                    && n.currentness != "superseded"
                    && !matches!(n.layer.as_str(), "cold" | "frozen")
            })
            .cloned()
            .collect(),
        details: Value::Null,
    });
    sections.push(Section {
        name: "knowledge".into(),
        records: nodes
            .iter()
            .filter(|n| {
                n.tier == "archival"
                    && n.currentness != "superseded"
                    && !matches!(n.layer.as_str(), "cold" | "frozen")
            })
            .cloned()
            .collect(),
        details: Value::Null,
    });
    let since = args
        .text("since")
        .map(|t| {
            DateTime::parse_from_rfc3339(&t)
                .map(|d| d.timestamp() as f64)
                .map_err(|e| Issue::new("INVALID_ARGUMENT", e.to_string()))
        })
        .transpose()?
        .unwrap_or(Utc::now().timestamp() as f64 - 86400.0);
    let cue = args.text("query").map(|s| s.to_lowercase());
    let recent = nodes
        .iter()
        .filter(|n| {
            n.recorded_at
                .as_ref()
                .and_then(|t| DateTime::parse_from_rfc3339(t).ok())
                .is_some_and(|d| d.timestamp() as f64 >= since)
        })
        .filter(|n| !matches!(n.layer.as_str(), "cold" | "frozen"))
        .filter(|n| {
            cue.as_ref().is_none_or(|q| {
                n.name.to_lowercase().contains(q)
                    || n.body.as_deref().unwrap_or("").to_lowercase().contains(q)
            })
        })
        .cloned()
        .collect();
    sections.push(Section {
        name: "recent".into(),
        records: recent,
        details: Value::Null,
    });
    sections.push(Section {
        name: "reasoning".into(),
        records: nodes
            .iter()
            .filter(|n| n.node_type == "chain")
            .cloned()
            .collect(),
        details: Value::Null,
    });
    let slots = slots_in(store, &scope)
        .map_err(issue_from_error)?
        .into_iter()
        .filter_map(|(slot, id)| {
            if check_node(store, &id, Some(&scope), options, false).is_err() {
                return None;
            }
            Some(SlotChange {
                revision: slot_revision(store, &scope, &slot).ok()?,
                slot,
                before: None,
                after: Some(id),
            })
        })
        .collect();
    let pending = store
        .db_ref()
        .run_script(
            "?[pending_report] := *initiative_hygiene{initiative:$scope,pending_report}",
            BTreeMap::from([("scope".into(), DataValue::Str(scope.clone().into()))]),
            ScriptMutability::Immutable,
        )
        .map_err(Error::from)
        .map_err(issue_from_error)?;
    if let Some(report) = pending
        .rows
        .first()
        .and_then(|r| r.first())
        .and_then(DataValue::get_str)
    {
        sections.push(Section {
            name: "maintenance".into(),
            records: vec![],
            details: json!({"memory_shifted":report}),
        });
    }
    let omitted = sections
        .iter()
        .filter(|s| matches!(s.name.as_str(), "rules" | "reminders"))
        .filter_map(|s| {
            let count = s.records.iter().filter(|n| n.body_range.is_some()).count();
            (count > 0).then(|| Omission {
                section: s.name.clone(),
                count,
                reason: "body_excerpt; read returned IDs for full content".into(),
            })
        })
        .collect::<Vec<_>>();
    let mut response = Response::read(
        "context",
        ResponseData::Context(ContextData {
            proposals: vec![],
            sections,
            slots,
        }),
        vec![scope],
    );
    if !completion_known {
        let c = response.coverage.as_mut().unwrap();
        c.complete = false;
        c.omitted.push(Omission {
            section: "open_tasks".into(),
            count: 0,
            reason: "completion_vocabulary_unavailable".into(),
        });
    }
    if !omitted.is_empty() {
        let c = response.coverage.as_mut().unwrap();
        c.complete = false;
        c.rules_complete = !omitted.iter().any(|o| o.section == "rules");
        c.omitted.extend(omitted);
    }
    Ok(response)
}

pub(in crate::api) fn constraint_active(store: &Store, n: &NodeData, options: &ApiOptions) -> bool {
    let Some(p) = n.properties.as_ref() else {
        return true;
    };
    if p.get("constraint_state").and_then(Value::as_str) == Some("released") {
        return false;
    }
    if let Some(through) = p.pointer("/applicability/through").and_then(Value::as_str)
        && through < Utc::now().format("%Y-%m-%d").to_string().as_str()
    {
        return false;
    }
    if let Some(task) = p.pointer("/applicability/task_id").and_then(Value::as_str) {
        if check_node(store, task, None, options, false).is_err() {
            return true;
        }
        if let Ok(Some(t)) = read_node_full(store, &task.into()) {
            let terminal = store
                .current_initiative()
                .and_then(|s| accessible_statuses(store, &s, None, options).ok().flatten())
                .unwrap_or_default();
            if terminal
                .iter()
                .any(|s| s.terminal && t.tags.iter().any(|t| t == &format!("status:{}", s.key)))
            {
                return false;
            }
        }
    }
    true
}

fn accessible_statuses(
    store: &Store,
    scope: &str,
    at: Option<f64>,
    options: &ApiOptions,
) -> ApiResult<Option<Vec<BoardStatus>>> {
    if let Some(id) = board_node_id(store, scope, at).map_err(issue_from_error)?
        && check_node(store, &id, Some(scope), options, at.is_some()).is_err()
    {
        return Ok(None);
    }
    effective_statuses_at(store, scope, at)
        .map(Some)
        .map_err(issue_from_error)
}
