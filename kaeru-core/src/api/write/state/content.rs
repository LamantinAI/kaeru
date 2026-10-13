use serde_json::{Value, from_value, json};

use crate::api::access::resolve;
use crate::api::read::{entity, relations};
use crate::api::types::{Issue, NodeRef, Response, ResponseData, Source, State};
use crate::api::write::capture::source_text;
use crate::api::write::{changed, result, target};
use crate::api::{ApiOptions, ApiResult, Args, issue_from_error, properties, put_properties};
use crate::{
    NodeType, Store, Tier, consolidate_in, consolidate_out, improve, read_node_full, set_reminder,
    supersedes, synthesise as core_synthesise, upsert_node,
};

pub(in crate::api) fn revise(
    store: &Store,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    let id = target(store, args, "node", None, options)?;
    let original = read_node_full(store, &id)
        .map_err(issue_from_error)?
        .unwrap();
    let patch = args.value("patch");
    let has_source = original
        .properties
        .as_ref()
        .and_then(|p| p.pointer("/source/original"))
        .is_some();
    if has_source && patch.get("body").is_some() {
        return Err(Issue::new(
            "INVALID_ARGUMENT",
            "immutable source must be changed through replace",
        )
        .field("patch.body"));
    }
    let kind = original
        .properties
        .as_ref()
        .and_then(|p| p.get("kind"))
        .and_then(Value::as_str);
    if (patch.get("constraint_state").is_some() || patch.get("applicability").is_some())
        && kind != Some("constraint")
    {
        return Err(Issue::new(
            "WRONG_NODE_TYPE",
            "constraint fields require a constraint record",
        ));
    }
    if patch.get("description").is_some() && original.node_type != "reference" {
        return Err(Issue::new(
            "WRONG_NODE_TYPE",
            "description belongs to a reference",
        ));
    }
    if patch.get("name").is_some() || patch.get("body").is_some() {
        improve(
            store,
            &id,
            patch
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or(&original.name),
            patch
                .get("body")
                .and_then(Value::as_str)
                .or(original.body.as_deref())
                .unwrap_or(""),
        )
        .map_err(issue_from_error)?;
    }
    let mut p = properties(store, &id)?;
    for key in ["description", "constraint_state", "applicability"] {
        if let Some(value) = patch.get(key) {
            let mut value = value.clone();
            if key == "applicability"
                && !value.is_null()
                && let Some(task) = value.get("task")
            {
                let reference: NodeRef = from_value(task.clone())
                    .map_err(|e| Issue::new("INVALID_ARGUMENT", e.to_string()))?;
                let target = resolve(store, &reference, options, None, false, false)?;
                if read_node_full(store, &target)
                    .map_err(issue_from_error)?
                    .unwrap()
                    .node_type
                    != "task"
                {
                    return Err(Issue::new(
                        "WRONG_NODE_TYPE",
                        "applicability requires a task",
                    ));
                }
                value.as_object_mut().unwrap().remove("task");
                value["task_id"] = json!(target);
            }
            if value.is_null() {
                p.as_object_mut().unwrap().remove(key);
            } else {
                p[key] = value;
            }
        }
    }
    if p != original.properties.clone().unwrap_or_else(|| json!({})) {
        put_properties(store, &id, p)?;
    }
    if let Some(reminder) = patch.get("reminder") {
        if reminder.is_null() {
            let n = read_node_full(store, &id)
                .map_err(issue_from_error)?
                .unwrap();
            let tags = n
                .tags
                .iter()
                .filter(|t| {
                    !t.starts_with("after:") && !t.starts_with("for:") && !t.starts_with("seen:")
                })
                .cloned()
                .collect::<Vec<_>>();
            upsert_node(
                store,
                &id,
                n.node_type.parse().map_err(issue_from_error)?,
                n.tier.parse().map_err(issue_from_error)?,
                &n.name,
                n.body.as_deref(),
                &tags,
                Some(&args.required_text("initiative")?),
                n.visibility.parse().map_err(issue_from_error)?,
                n.layer.parse().map_err(issue_from_error)?,
                n.properties.as_ref(),
            )
            .map_err(issue_from_error)?;
        } else {
            set_reminder(
                store,
                &id,
                reminder["after"].as_str().unwrap(),
                reminder["window_days"].as_i64().unwrap(),
            )
            .map_err(issue_from_error)?;
        }
    }
    let actual = read_node_full(store, &id)
        .map_err(issue_from_error)?
        .unwrap();
    let mut r = result(store, "revise", &id, false, options)?;
    if let ResponseData::Mutation(d) = &mut r.data {
        if original.name != actual.name {
            d.changes
                .push(changed("name", json!(original.name), json!(actual.name)));
        }
        if original.body != actual.body {
            d.changes
                .push(changed("body", json!(original.body), json!(actual.body)));
        }
        if original.properties != actual.properties {
            d.changes.push(changed(
                "properties",
                json!(original.properties),
                json!(actual.properties),
            ));
        }
        if original.tags != actual.tags {
            d.changes
                .push(changed("tags", json!(original.tags), json!(actual.tags)));
        }
        if d.changes.is_empty() {
            r.state = State::Unchanged;
        }
    }
    Ok(r)
}

pub(in crate::api) fn replacement(
    store: &Store,
    name: &str,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    let id = target(
        store,
        args,
        if name == "replace" { "old" } else { "node" },
        None,
        options,
    )?;
    let old = read_node_full(store, &id)
        .map_err(issue_from_error)?
        .unwrap();
    let successor = if name == "lifecycle" {
        let tier = args
            .required_text("target_tier")?
            .parse::<Tier>()
            .map_err(issue_from_error)?;
        let ty = if let Some(ty) = args.text("target_type") {
            ty.parse::<NodeType>().map_err(issue_from_error)?
        } else if tier == Tier::Archival {
            old.node_type
                .parse::<NodeType>()
                .map_err(issue_from_error)?
                .settled_form()
        } else {
            old.node_type.parse().map_err(issue_from_error)?
        };
        if tier.as_str() == old.tier && ty.as_str() == old.node_type {
            let mut r = result(store, name, &id, false, options)?;
            r.state = State::Unchanged;
            return Ok(r);
        }
        if tier == Tier::Archival {
            consolidate_out(store, &id, ty, &old.name, old.body.as_deref().unwrap_or(""))
        } else {
            consolidate_in(store, &id, ty, &old.name, old.body.as_deref().unwrap_or(""))
        }
        .map_err(issue_from_error)?
    } else {
        let s = args.value("successor");
        let ty = s
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or(&old.node_type)
            .parse::<NodeType>()
            .map_err(issue_from_error)?;
        let tier = s
            .get("tier")
            .and_then(Value::as_str)
            .unwrap_or(&old.tier)
            .parse::<Tier>()
            .map_err(issue_from_error)?;
        if matches!(
            ty,
            NodeType::Board | NodeType::Chain | NodeType::AuditEvent | NodeType::Experiment
        ) {
            return Err(Issue::new(
                "WRONG_NODE_TYPE",
                "system records use their own commands",
            ));
        }
        let source = s
            .get("source")
            .map(|s| {
                from_value::<Source>(s.clone())
                    .map_err(|e| Issue::new("INVALID_ARGUMENT", e.to_string()))
            })
            .transpose()?;
        if old
            .properties
            .as_ref()
            .and_then(|p| p.pointer("/source/original"))
            .is_some()
            && source.is_none()
        {
            return Err(Issue::new(
                "INVALID_ARGUMENT",
                "replacement of an exact source needs a new source",
            ));
        }
        let (body, source_meta) = if let Some(source) = source {
            let (b, p) = source_text(&source, options)?;
            (b, Some(p))
        } else {
            (s["body"].as_str().unwrap().into(), None)
        };
        let successor = supersedes(store, &id, ty, tier, s["name"].as_str().unwrap(), &body)
            .map_err(issue_from_error)?;
        if let Some(source) = source_meta {
            let mut p = properties(store, &successor)?;
            p["source"] = source;
            put_properties(store, &successor, p)?;
        }
        successor
    };
    let mut r = result(store, name, &successor, true, options)?;
    if let ResponseData::Mutation(d) = &mut r.data {
        d.changes
            .push(changed("identity", json!(id), json!(successor)));
        for edge in relations(store, &successor, options)? {
            if matches!(edge.edge_type.as_str(), "derived_from" | "refers_to")
                && edge.to == successor
                && let Ok(e) = entity(store, &edge.from, options)
            {
                d.affected.push(e);
            }
        }
    }
    if old.visibility == "shared" {
        r.issues.push(Issue::new(
            "REMOTE_STALE",
            "predecessor was shared; explicitly share the successor",
        ));
    }
    Ok(r)
}

pub(in crate::api) fn synthesise(
    store: &Store,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    let refs: Vec<NodeRef> = args.parse("sources")?;
    let mut sources = vec![];
    for reference in refs {
        sources.push(resolve(store, &reference, options, None, false, true)?);
    }
    let r = args.value("result");
    let ty = r
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or("summary")
        .parse::<NodeType>()
        .map_err(issue_from_error)?;
    let tier = r
        .get("tier")
        .and_then(Value::as_str)
        .unwrap_or("archival")
        .parse::<Tier>()
        .map_err(issue_from_error)?;
    if matches!(ty, NodeType::Idea | NodeType::Outcome) && tier != Tier::Archival {
        return Err(Issue::new(
            "INVALID_ARGUMENT",
            "idea and outcome synthesis must be archival",
        ));
    }
    let id = core_synthesise(
        store,
        &sources,
        ty,
        tier,
        r["name"].as_str().unwrap(),
        r["body"].as_str().unwrap(),
    )
    .map_err(issue_from_error)?;
    result(store, "synthesise", &id, true, options)
}
