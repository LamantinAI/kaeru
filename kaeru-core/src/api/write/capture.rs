use std::collections::BTreeMap;
use std::fs::read_to_string;

use cozo::{DataValue, ScriptMutability};
use serde_json::{Value, from_value, json};
use sha2::{Digest, Sha256};

use super::super::access::{check_scope, resolve};
use super::super::types::{
    Evidence, FailedStep, Issue, NodeRef, Response, ResponseData, Source, State,
};
use super::super::{ApiOptions, ApiResult, Args, issue_from_error, put_properties};
use super::{capture_slot, result};
use crate::mutate::{build_body_tags, tags_literal};
use crate::{
    EdgeType, Error, HypothesisStatus, Layer, NodeType, Store, Tier, Visibility, cite_with_layer,
    derive_auto_name, effective_statuses, formulate_hypothesis_with_status, history, link,
    list_initiatives, new_node_id, read_node_full, run_experiment, set_reminder,
    strip_tool_call_markup, update_hypothesis_status, upsert_node, write_task_with_layer,
};

pub(super) fn dispatch(
    store: &Store,
    name: &str,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    let layer = args
        .value("options")
        .get("layer")
        .and_then(Value::as_str)
        .unwrap_or("warm")
        .parse::<Layer>()
        .map_err(issue_from_error)?;
    let scope = args.required_text("initiative")?;
    let is_new = !list_initiatives(store)
        .map_err(issue_from_error)?
        .iter()
        .any(|s| s == &scope);
    let (id, missing_citation) = match name {
        "note" => {
            let body = args.required_text("body")?;
            let kind = args.text("kind").unwrap_or_else(|| "observation".into());
            let ty = match kind.as_str() {
                "constraint" => NodeType::Reference,
                "plan" => NodeType::Roadmap,
                "outcome" => NodeType::Outcome,
                _ => NodeType::Episode,
            };
            let tier = if matches!(ty, NodeType::Reference | NodeType::Outcome) {
                Tier::Archival
            } else {
                Tier::Operational
            };
            let name = args
                .text("name")
                .unwrap_or_else(|| derive_auto_name(&body, &kind));
            let mut properties = json!({"kind":kind});
            if kind == "constraint" {
                properties["constraint_state"] = json!("active");
                if args.has("applicability") {
                    let mut applicability = args.value("applicability").clone();
                    if let Some(task) = applicability.get("task") {
                        let target: NodeRef = from_value(task.clone())
                            .map_err(|e| Issue::new("INVALID_ARGUMENT", e.to_string()))?;
                        let id = resolve(store, &target, options, None, false, false)?;
                        let node = read_node_full(store, &id)
                            .map_err(issue_from_error)?
                            .unwrap();
                        if node.node_type != "task" {
                            return Err(Issue::new(
                                "WRONG_NODE_TYPE",
                                "constraint applicability requires a task",
                            ));
                        }
                        applicability.as_object_mut().unwrap().remove("task");
                        applicability["task_id"] = json!(id);
                    }
                    properties["applicability"] = applicability;
                }
            }
            let id = new_node_id();
            let kind_tag = format!("kind:{kind}");
            let tags = build_body_tags(
                &[
                    &kind_tag,
                    if kind == "observation" {
                        "sig:low"
                    } else {
                        "sig:medium"
                    },
                ],
                Some(&name),
                &body,
            );
            let body = strip_tool_call_markup(&body).0;
            upsert_node(
                store,
                &id,
                ty,
                tier,
                &name,
                Some(&body),
                &tags,
                Some(&scope),
                Visibility::Local,
                layer,
                Some(&properties),
            )
            .map_err(issue_from_error)?;
            if args.has("reminder") {
                let reminder = args.value("reminder");
                set_reminder(
                    store,
                    &id,
                    reminder["after"].as_str().unwrap(),
                    reminder["window_days"].as_i64().unwrap(),
                )
                .map_err(issue_from_error)?;
            }
            (id, false)
        }
        "reference" => {
            let name = args.required_text("name")?;
            let url = args.text("url");
            let (original, source) = if let Some(source) = args.optional::<Source>("source")? {
                source_text(&source, options)?
            } else {
                (String::new(), json!({"origin":"url_only"}))
            };
            let id = cite_with_layer(store, &name, url.as_deref(), &original, layer)
                .map_err(issue_from_error)?;
            let mut properties = json!({"source":source,"description":args.value("description")});
            if let Some(url) = url {
                properties["url"] = json!(url);
            }
            put_properties(store, &id, properties)?;
            (id, false)
        }
        "task" => {
            let statuses = effective_statuses(store, &scope).map_err(issue_from_error)?;
            if !statuses.iter().any(|s| s.key == "open" && !s.terminal) {
                return Err(Issue::new(
                    "REGISTRY_INTEGRITY",
                    "the board needs a nonterminal open column",
                ));
            }
            (
                write_task_with_layer(
                    store,
                    &args.required_text("body")?,
                    args.text("due").as_deref(),
                    layer,
                )
                .map_err(issue_from_error)?,
                false,
            )
        }
        "claim" => {
            let text = args.required_text("text")?;
            let status = args
                .text("initial_status")
                .unwrap_or_else(|| "open".into())
                .parse::<HypothesisStatus>()
                .map_err(issue_from_error)?;
            // Resolve the declared semantic target/evidence before constructing the claim.
            let about = args
                .optional::<NodeRef>("about")?
                .map(|n| resolve(store, &n, options, None, false, true))
                .transpose()?;
            let evidence = args.optional::<Evidence>("evidence")?;
            let existing = match &evidence {
                Some(Evidence::Existing(e)) => {
                    Some(resolve(store, &e.node, options, None, false, true)?)
                }
                _ => None,
            };
            let id = formulate_hypothesis_with_status(
                store,
                &derive_auto_name(&text, "claim"),
                &text,
                layer,
                status,
            )
            .map_err(issue_from_error)?;
            if let Some(about) = about {
                link(store, &id, &about, EdgeType::RefersTo).map_err(issue_from_error)?;
            }
            let check = record_evidence(store, &id, evidence, existing)?;
            if status.is_verdict() {
                update_hypothesis_status(store, &id, status, check.as_ref())
                    .map_err(issue_from_error)?;
            }
            record_decision(store, &id, status.as_str(), check.as_deref())?;
            (id, status.is_verdict() && check.is_none())
        }
        _ => unreachable!(),
    };
    let mut response = result(store, name, &id, true, options)?;
    if let ResponseData::Mutation(data) = &mut response.data {
        data.slots = capture_slot(store, args, &id, options)?;
    }
    if is_new {
        let candidates = list_initiatives(store)
            .map_err(issue_from_error)?
            .into_iter()
            .filter(|s| s != &scope && check_scope(s, options).is_ok())
            .collect::<Vec<_>>();
        response.issues.push(Issue::new(
            "NEW_INITIATIVE",
            format!(
                "created initiative `{scope}`; existing permitted initiatives: {}",
                candidates.join(", ")
            ),
        ));
    }
    if missing_citation {
        response.issues.push(Issue::new(
            "MISSING_CITATION",
            "verdict recorded without an evidence node",
        ));
    }
    if args
        .value("sharing")
        .get("visibility")
        .and_then(Value::as_str)
        == Some("shared")
    {
        if let ResponseData::Mutation(data) = &mut response.data {
            data.failed_steps.push(FailedStep {
                step: "share".into(),
                code: "REMOTE_PENDING".into(),
                message: "local capture committed; adapter must perform the requested share".into(),
            });
        }
        response.state = State::Partial;
    }
    Ok(response)
}

pub(super) fn source_text(source: &Source, options: &ApiOptions) -> ApiResult<(String, Value)> {
    let (text, origin, artifact) = match source {
        Source::Text(s) => (s.text.clone(), "provided_text", None),
        Source::Artifact(s) => {
            let root = options.artifact_root.as_ref().ok_or_else(|| {
                Issue::new(
                    "UNSUPPORTED_SOURCE",
                    "this host has no authorized artifact root",
                )
            })?;
            let root = root
                .canonicalize()
                .map_err(|e| Issue::new("UNSUPPORTED_SOURCE", e.to_string()))?;
            let path = root
                .join(&s.artifact)
                .canonicalize()
                .map_err(|e| Issue::new("UNSUPPORTED_SOURCE", e.to_string()))?;
            if !path.starts_with(&root) {
                return Err(Issue::new(
                    "UNSUPPORTED_SOURCE",
                    "artifact is outside the authorized root",
                ));
            }
            (
                read_to_string(path)
                    .map_err(|e| Issue::new("UNSUPPORTED_SOURCE", e.to_string()))?,
                "verified_artifact",
                Some(s.artifact.clone()),
            )
        }
    };
    let digest = format!("{:x}", Sha256::digest(text.as_bytes()));
    let metadata = json!({"origin":origin,"sha256":digest,"original":text,"artifact":artifact});
    Ok((text, metadata))
}

pub(super) fn record_evidence(
    store: &Store,
    hypothesis: &str,
    evidence: Option<Evidence>,
    existing: Option<String>,
) -> ApiResult<Option<String>> {
    let id = match evidence {
        Some(Evidence::Existing(_)) => existing,
        Some(Evidence::Check(check)) => Some(
            run_experiment(
                store,
                &hypothesis.into(),
                &derive_auto_name(&check.method, "evidence"),
                &check.method,
            )
            .map_err(issue_from_error)?,
        ),
        None => None,
    };
    if let Some(id) = id.as_ref() {
        link(store, id, &hypothesis.into(), EdgeType::Targets).map_err(issue_from_error)?;
    }
    Ok(id)
}

pub(super) fn record_decision(
    store: &Store,
    hypothesis: &str,
    status: &str,
    evidence: Option<&str>,
) -> ApiResult<()> {
    let version = history(store, &hypothesis.into())
        .map_err(issue_from_error)?
        .into_iter()
        .filter(|r| r.asserted)
        .map(|r| r.seconds as i64)
        .max()
        .unwrap_or(0);
    let refs = evidence.into_iter().map(str::to_string).collect::<Vec<_>>();
    let literal = tags_literal(&refs);
    let literal = if literal == "null" { "[]" } else { &literal };
    store.db_ref().run_script(&format!("?[hypothesis, version, status, evidence] <- [[$id, {version}, $status, {literal}]] :put api_hypothesis_decision {{hypothesis, version => status, evidence}}"),BTreeMap::from([("id".into(),DataValue::Str(hypothesis.into())),("status".into(),DataValue::Str(status.into()))]),ScriptMutability::Mutable).map_err(Error::from).map_err(issue_from_error)?;
    Ok(())
}
