use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};
use serde_json::json;

use crate::api::access::resolve;
use crate::api::read::{entity, node, relations};
use crate::api::write::capture::{record_decision, record_evidence};
use crate::api::write::{changed, result, target};
use crate::api::{ApiOptions, ApiResult, Args, issue_from_error};
use crate::models::{Evidence, EvidenceResult, Issue, NodeRef, Response, ResponseData, State};
use crate::{
    EdgeType, EpisodeKind, Error, HypothesisStatus, NodeType, Significance, Store,
    derive_auto_name, effective_statuses, link, mark_resolved, mark_under_review, read_node_full,
    set_status, unlink, update_hypothesis_status, upsert_node, write_episode,
};

pub(in crate::api) fn verdict(
    store: &Store,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    let id = target(store, args, "hypothesis", Some("hypothesis"), options)?;
    let before = node(store, &id, options, false)?.status;
    let evidence = args.optional::<Evidence>("evidence")?;
    let existing = match &evidence {
        Some(Evidence::Existing(e)) => Some(resolve(store, &e.node, options, None, false, true)?),
        _ => None,
    };
    let check = record_evidence(store, &id, evidence, existing)?;
    let action = args.action("decide");
    let after = if action == "decide" {
        let status = args
            .required_text("status")?
            .parse::<HypothesisStatus>()
            .map_err(issue_from_error)?;
        update_hypothesis_status(store, &id, status, check.as_ref()).map_err(issue_from_error)?;
        record_decision(store, &id, status.as_str(), check.as_deref())?;
        Some(status.as_str().into())
    } else {
        before.clone()
    };
    let mut r = result(store, "verdict", &id, false, options)?;
    if let ResponseData::Mutation(d) = &mut r.data {
        if before != after {
            d.changes
                .push(changed("status", json!(before), json!(after)));
        }
        if let Some(check) = check {
            d.evidence.push(EvidenceResult {
                id: check,
                relation: "targets".into(),
            });
        } else if action == "decide" {
            r.issues.push(Issue::new(
                "MISSING_CITATION",
                "verdict recorded without an evidence node",
            ));
        }
    }
    Ok(r)
}

pub(in crate::api) fn task_update(
    store: &Store,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    let id = target(store, args, "task", Some("task"), options)?;
    let scope = args.required_text("initiative")?;
    let old = read_node_full(store, &id)
        .map_err(issue_from_error)?
        .unwrap();
    if let Some(status) = args.text("status") {
        let known = effective_statuses(store, &scope).map_err(issue_from_error)?;
        if !known.iter().any(|s| s.key == status) {
            return Err(Issue::new(
                "UNKNOWN_STATUS",
                format!(
                    "known statuses: {}",
                    known
                        .iter()
                        .map(|s| s.key.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            )
            .field("status"));
        }
        if old.tags.iter().find_map(|t| t.strip_prefix("status:")) != Some(status.as_str()) {
            set_status(store, &scope, &id, &status).map_err(issue_from_error)?;
        }
    }
    if args.has("due")
        && old
            .tags
            .iter()
            .find_map(|t| t.strip_prefix("due:"))
            .map(str::to_owned)
            != args.text("due")
    {
        let n = read_node_full(store, &id)
            .map_err(issue_from_error)?
            .unwrap();
        let mut tags: Vec<_> = n
            .tags
            .into_iter()
            .filter(|t| !t.starts_with("due:"))
            .collect();
        if let Some(due) = args.text("due") {
            tags.push(format!("due:{due}"));
        }
        upsert_node(
            store,
            &id,
            NodeType::Task,
            n.tier.parse().map_err(issue_from_error)?,
            &n.name,
            n.body.as_deref(),
            &tags,
            Some(&scope),
            n.visibility.parse().map_err(issue_from_error)?,
            n.layer.parse().map_err(issue_from_error)?,
            n.properties.as_ref(),
        )
        .map_err(issue_from_error)?;
    }
    let new = read_node_full(store, &id)
        .map_err(issue_from_error)?
        .unwrap();
    let mut r = result(store, "task_update", &id, false, options)?;
    if let ResponseData::Mutation(d) = &mut r.data {
        for prefix in ["status:", "due:"] {
            let before = old.tags.iter().find_map(|t| t.strip_prefix(prefix));
            let after = new.tags.iter().find_map(|t| t.strip_prefix(prefix));
            if before != after {
                d.changes.push(changed(
                    prefix.trim_end_matches(':'),
                    json!(before),
                    json!(after),
                ));
            }
        }
        let status = new
            .tags
            .iter()
            .find_map(|t| t.strip_prefix("status:"))
            .unwrap_or("");
        let terminal = effective_statuses(store, &scope)
            .map_err(issue_from_error)?
            .iter()
            .any(|s| s.key == status && s.terminal);
        let before_terminal = effective_statuses(store, &scope)
            .map_err(issue_from_error)?
            .iter()
            .any(|s| s.terminal && old.tags.iter().any(|t| t == &format!("status:{}", s.key)));
        if before_terminal != terminal {
            d.changes
                .push(changed("terminal", json!(before_terminal), json!(terminal)));
        }
        if d.changes.is_empty() && old == new {
            r.state = State::Unchanged;
        }
    }
    Ok(r)
}

pub(in crate::api) fn review(
    store: &Store,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    let id = target(store, args, "target", None, options)?;
    let action = args.action("");
    match action.as_str() {
        "open" => {
            let review = mark_under_review(store, &id, &args.required_text("reason")?)
                .map_err(issue_from_error)?;
            let mut r = result(store, "review", &id, false, options)?;
            if let ResponseData::Mutation(d) = &mut r.data {
                d.created.push(entity(store, &review, options)?);
            }
            Ok(r)
        }
        "answer" => {
            let kind = node(store, &id, options, false)?.node_type;
            if matches!(kind.as_str(), "task" | "hypothesis") {
                return Err(Issue::new(
                    "WRONG_NODE_TYPE",
                    "use task_update or verdict for this target",
                ));
            }
            let answer = resolve(store, &args.parse("answer")?, options, None, false, true)?;
            mark_resolved(store, &id, &answer).map_err(issue_from_error)?;
            result(store, "review", &id, false, options)
        }
        _ => {
            let edges = relations(store, &id, options)?;
            let mut selected = vec![];
            if let Some(refs) = args.optional::<Vec<NodeRef>>("reviews")? {
                for reference in refs {
                    let reviewer = resolve(store, &reference, options, None, false, true)?;
                    let n = node(store, &reviewer, options, false)?;
                    if !n.tags.iter().any(|t| t == "role:review") {
                        return Err(Issue::new(
                            "WRONG_NODE_TYPE",
                            "selected node is not a review flag",
                        ));
                    }
                    if edges
                        .iter()
                        .any(|e| e.from == reviewer && e.to == id && e.edge_type == "contradicts")
                    {
                        selected.push(reviewer);
                    } else {
                        let rows=store.db_ref().run_script("?[src] := *edge{src,dst,edge_type}, src=$src,dst=$dst,edge_type='contradicts'",BTreeMap::from([("src".into(),DataValue::Str(reviewer.into())),("dst".into(),DataValue::Str(id.clone().into()))]),ScriptMutability::Immutable).map_err(Error::from).map_err(issue_from_error)?;
                        if rows.rows.is_empty() {
                            return Err(Issue::new(
                                "INVALID_ARGUMENT",
                                "review does not belong to this target",
                            ));
                        }
                    }
                }
            } else {
                for edge in &edges {
                    if edge.to == id
                        && edge.edge_type == "contradicts"
                        && node(store, &edge.from, options, false)?
                            .tags
                            .iter()
                            .any(|t| t == "role:review")
                    {
                        selected.push(edge.from.clone());
                    }
                }
            }
            if selected.is_empty() {
                let mut r = result(store, "review", &id, false, options)?;
                r.state = State::Unchanged;
                return Ok(r);
            }
            for reviewer in &selected {
                unlink(store, reviewer, &id, EdgeType::Contradicts).map_err(issue_from_error)?;
            }
            let mut r = result(store, "review", &id, false, options)?;
            if let ResponseData::Mutation(d) = &mut r.data {
                for reviewer in &selected {
                    d.affected.push(entity(store, reviewer, options)?);
                }
                if let Some(text) = args.text("resolution") {
                    let resolution = write_episode(
                        store,
                        EpisodeKind::Observation,
                        Significance::Medium,
                        &derive_auto_name(&text, "resolution"),
                        &text,
                    )
                    .map_err(issue_from_error)?;
                    for reviewer in &selected {
                        link(store, &resolution, reviewer, EdgeType::Supersedes)
                            .map_err(issue_from_error)?;
                    }
                    d.created.push(entity(store, &resolution, options)?);
                }
            }
            Ok(r)
        }
    }
}
