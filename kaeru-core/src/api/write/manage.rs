use serde_json::{Value, json};

use super::super::access::{check_node, check_scope, resolve};
use super::super::read::entity;
use super::super::{ApiOptions, ApiResult, Args, hash, issue_from_error};
use super::export::run as export_archive;
use super::{assign_slot, changed, check_revision, result, target};
use crate::hygiene::{collect, force_pass};
use crate::models::{
    ContextData, Entity, FailedStep, Issue, MutationData, Response, ResponseData,
    RetentionProposal, Section, State,
};
use crate::mutate::board::set_terminal_status;
use crate::{
    EpisodeKind, Layer, Significance, Store, add_status, attach_node, delete_initiative,
    effective_statuses, get_layer, get_share_policy, initiative_clouds, list_initiatives,
    merge_initiative, new_node_id, nodes_in_initiative, read_node_full, relabel_status,
    remove_status, rename_initiative, reorder_statuses, set_initiative_clouds, set_layer,
    set_share_policy, write_episode_with_layer,
};

pub(super) fn dispatch(
    store: &Store,
    name: &str,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    match name {
        "initiative_manage" => initiative_manage(store, args, options),
        "board_config" => board(store, args),
        "policy" => policy(store, args),
        "retention" => retention(store, args, options),
        "export" => export_archive(store, args, options),
        _ => unreachable!(),
    }
}

fn initiative_manage(store: &Store, args: &Args, options: &ApiOptions) -> ApiResult<Response> {
    let action = args.action("");
    let mut data = MutationData::default();
    match action.as_str() {
        "rename" => {
            let old = args.required_text("old")?;
            let new = args.required_text("new")?;
            check_scope(&old, options)?;
            check_scope(&new, options)?;
            let stats = rename_initiative(store, &old, &new).map_err(issue_from_error)?;
            data.changes.push(changed(
                "initiative",
                json!(old),
                json!({"name":new,"nodes":stats.nodes,"edges":stats.edges}),
            ));
        }
        "merge" => {
            let old = args.required_text("source")?;
            let new = args.required_text("target")?;
            check_scope(&old, options)?;
            check_scope(&new, options)?;
            if !list_initiatives(store)
                .map_err(issue_from_error)?
                .contains(&new)
            {
                return Err(Issue::new("NOT_FOUND", "merge target must already exist"));
            }
            let stats = merge_initiative(store, &old, &new).map_err(issue_from_error)?;
            data.changes.push(changed(
                "initiative",
                json!(old),
                json!({"name":new,"nodes":stats.nodes,"edges":stats.edges}),
            ));
        }
        "delete" => {
            let scope = args.required_text("name")?;
            check_scope(&scope, options)?;
            for n in nodes_in_initiative(store, &scope).map_err(issue_from_error)? {
                check_node(store, &n.id, Some(&scope), options, false)?;
            }
            let stats = delete_initiative(store, &scope).map_err(issue_from_error)?;
            data.changes.push(changed(
                "initiative",
                json!(scope),
                json!({"forgotten":stats.forgotten,"detached":stats.unscoped}),
            ));
        }
        "attach" => {
            let scope = args.required_text("to")?;
            check_scope(&scope, options)?;
            let id = resolve(store, &args.parse("node")?, options, None, false, true)?;
            let stats = attach_node(store, &id, &scope).map_err(issue_from_error)?;
            data.primary = Some(entity(store, &id, options)?);
            if stats.already_member {
                return Ok(Response {
                    state: State::Unchanged,
                    ..Response::mutation("initiative_manage", data)
                });
            }
            data.changes
                .push(changed("membership", Value::Null, json!(scope)));
        }
        "set_slot" | "clear_slot" => {
            let scope = args.required_text("initiative")?;
            check_scope(&scope, options)?;
            let slot = args.required_text("slot")?;
            let id = if action == "set_slot" {
                Some(resolve(
                    store,
                    &args.parse("node")?,
                    options,
                    None,
                    false,
                    false,
                )?)
            } else {
                None
            };
            let slot = assign_slot(
                store,
                &scope,
                &slot,
                id.as_deref(),
                args.text("expected_slot_revision").as_deref(),
                options,
            )?;
            let unchanged = slot.before == slot.after;
            data.slots.push(slot);
            return Ok(Response {
                state: if unchanged {
                    State::Unchanged
                } else {
                    State::Applied
                },
                ..Response::mutation("initiative_manage", data)
            });
        }
        _ => unreachable!(),
    }
    let mut response = Response::mutation("initiative_manage", data);
    if args.has("cloud") {
        if let ResponseData::Mutation(d) = &mut response.data {
            d.failed_steps.push(FailedStep {
                step: "namespace_cloud".into(),
                code: "REMOTE_PENDING".into(),
                message: "local change committed; adapter must update the explicitly named cloud"
                    .into(),
            });
        }
        response.state = State::Partial;
    }
    Ok(response)
}

fn registry(store: &Store, scope: &str) -> ApiResult<Value> {
    let statuses = effective_statuses(store, scope).map_err(issue_from_error)?;
    Ok(json!(
        statuses
            .into_iter()
            .map(|s| json!({"key":s.key,"label":s.label,"terminal":s.terminal}))
            .collect::<Vec<_>>()
    ))
}

fn board(store: &Store, args: &Args) -> ApiResult<Response> {
    let scope = args.required_text("initiative")?;
    let before = registry(store, &scope)?;
    check_revision(args, &hash(&before))?;
    let key = args.text("key");
    match args.action("").as_str() {
        "add" => {
            let key = key.unwrap();
            add_status(
                store,
                &scope,
                &key,
                args.text("label").as_deref().unwrap_or(&key),
            )
            .map_err(issue_from_error)?;
            if args.value("terminal").as_bool() == Some(true) {
                set_terminal_status(store, &scope, &key, true).map_err(issue_from_error)?;
            }
        }
        "remove" => {
            let key = key.unwrap();
            let statuses = effective_statuses(store, &scope).map_err(issue_from_error)?;
            if key == "open" {
                return Err(Issue::new(
                    "REGISTRY_INTEGRITY",
                    "the initial open column cannot be removed",
                ));
            }
            if statuses.iter().filter(|s| s.terminal).count() == 1
                && statuses.iter().any(|s| s.key == key && s.terminal)
            {
                return Err(Issue::new(
                    "REGISTRY_INTEGRITY",
                    "the final terminal column cannot be removed",
                ));
            }
            let occupied = nodes_in_initiative(store, &scope)
                .map_err(issue_from_error)?
                .into_iter()
                .any(|n| {
                    read_node_full(store, &n.id)
                        .ok()
                        .flatten()
                        .is_some_and(|n| {
                            n.node_type == "task" && n.tags.contains(&format!("status:{key}"))
                        })
                });
            if occupied {
                return Err(Issue::new(
                    "REGISTRY_INTEGRITY",
                    "move tasks before removing an occupied column",
                ));
            }
            remove_status(store, &scope, &key).map_err(issue_from_error)?;
        }
        "relabel" => {
            relabel_status(store, &scope, &key.unwrap(), &args.required_text("label")?)
                .map_err(issue_from_error)?;
        }
        "reorder" => {
            reorder_statuses(store, &scope, &args.parse::<Vec<String>>("order")?)
                .map_err(issue_from_error)?;
        }
        "set_terminal" => {
            set_terminal_status(
                store,
                &scope,
                &key.unwrap(),
                args.value("terminal").as_bool().unwrap(),
            )
            .map_err(issue_from_error)?;
        }
        _ => unreachable!(),
    }
    let after = registry(store, &scope)?;
    Ok(Response::mutation(
        "board_config",
        MutationData {
            primary: Some(Entity {
                id: format!("board:{scope}"),
                revision: hash(&after),
            }),
            changes: vec![changed("columns", before, after)],
            ..MutationData::default()
        },
    ))
}

pub(super) fn policy(store: &Store, args: &Args) -> ApiResult<Response> {
    let scope = args.required_text("initiative")?;
    let before = json!({"policy":get_share_policy(store,&scope).map_err(issue_from_error)?.as_str(),"clouds":initiative_clouds(store,&scope).map_err(issue_from_error)?});
    check_revision(args, &hash(&before))?;
    if let Some(policy) = args.text("policy") {
        set_share_policy(store, &scope, policy.parse().map_err(issue_from_error)?)
            .map_err(issue_from_error)?;
    }
    if let Some(clouds) = args.optional::<Vec<String>>("clouds")? {
        set_initiative_clouds(store, &scope, &clouds).map_err(issue_from_error)?;
    }
    let after = json!({"policy":get_share_policy(store,&scope).map_err(issue_from_error)?.as_str(),"clouds":initiative_clouds(store,&scope).map_err(issue_from_error)?});
    Ok(Response {
        state: if before == after {
            State::Unchanged
        } else {
            State::Applied
        },
        ..Response::mutation(
            "policy",
            MutationData {
                primary: Some(Entity {
                    id: format!("policy:{scope}"),
                    revision: hash(&after),
                }),
                changes: vec![changed("policy", before, after)],
                ..MutationData::default()
            },
        )
    })
}

pub(super) fn retention(store: &Store, args: &Args, options: &ApiOptions) -> ApiResult<Response> {
    let scope = args.required_text("initiative")?;
    match args.action("").as_str() {
        "set_layer" => {
            let id = target(store, args, "node", None, options)?;
            let layer = args
                .required_text("layer")?
                .parse()
                .map_err(issue_from_error)?;
            let before = get_layer(store, &id).map_err(issue_from_error)?;
            set_layer(store, &id, layer).map_err(issue_from_error)?;
            let mut r = result(store, "retention", &id, false, options)?;
            if let ResponseData::Mutation(d) = &mut r.data {
                d.changes.push(changed(
                    "layer",
                    json!(before.as_str()),
                    json!(layer.as_str()),
                ));
            }
            Ok(r)
        }
        "preview_hygiene" => {
            let candidates = collect(store, &scope)
                .map_err(issue_from_error)?
                .into_iter()
                .filter(|c| check_node(store, &c.node_id, Some(&scope), options, false).is_ok())
                .collect::<Vec<_>>();
            Ok(Response::read(
                "retention",
                ResponseData::Context(ContextData {
                    proposals: candidates
                        .iter()
                        .map(|c| RetentionProposal {
                            id: c.node_id.clone(),
                            name: c.name.clone(),
                            action: c.action.as_str().into(),
                            from: c.from.as_str().into(),
                            to: c.to.as_str().into(),
                            reason: c.reason.clone(),
                        })
                        .collect(),
                    sections: vec![Section {
                        name: "hygiene".into(),
                        records: vec![],
                        details: json!({"candidate_count":candidates.len()}),
                    }],
                    slots: vec![],
                }),
                vec![scope],
            ))
        }
        "run_hygiene" => Err(Issue::new(
            "INVALID_ARGUMENT",
            "run_hygiene owns its scope guard and must enter through the API dispatcher",
        )),
        _ => unreachable!(),
    }
}

pub(super) fn run_hygiene(store: &Store, args: &Args, options: &ApiOptions) -> ApiResult<Response> {
    let scope = args.required_text("initiative")?;
    check_scope(&scope, options)?;
    if !options.include_local {
        return Err(Issue::new(
            "READ_ONLY_SCOPE",
            "metadata hygiene requires local operator access",
        ));
    }
    let report = match force_pass(store, &scope, || true) {
        Ok(report) => report,
        Err(error) => {
            let issue = issue_from_error(error);
            let mut response = Response::mutation(
                "retention",
                MutationData {
                    failed_steps: vec![FailedStep {
                        step: "run_hygiene".into(),
                        code: issue.code.clone(),
                        message: issue.message.clone(),
                    }],
                    ..MutationData::default()
                },
            );
            response.state = State::Partial;
            response.issues.push(issue);
            return Ok(response);
        }
    };
    let mut response=Response::mutation("retention",MutationData{changes:vec![changed("hygiene",Value::Null,report.as_ref().map(|r|json!({"archived":r.archived,"demoted":r.demoted,"promoted":r.promoted,"skipped":r.skipped,"stopped_early":r.stopped_early})).unwrap_or(Value::Null))],..MutationData::default()});
    if let Some(report) = report
        && report.applied() > 0
    {
        let body = format!("{}\n\n{}", report.summary(), report.lines.join("\n"));
        let record = store.scoped(Some(&scope), |s| {
            write_episode_with_layer(
                s,
                EpisodeKind::Action,
                Significance::Low,
                &format!("hygiene-{scope}-{}", new_node_id()),
                &body,
                Layer::Cold,
            )
            .map_err(issue_from_error)
            .and_then(|id| entity(s, &id, options))
        });
        if let ResponseData::Mutation(data) = &mut response.data {
            match record {
                Ok(record) => data.created.push(record),
                Err(issue) => {
                    data.failed_steps.push(FailedStep {
                        step: "record_hygiene_log".into(),
                        code: issue.code.clone(),
                        message: issue.message.clone(),
                    });
                    response.state = State::Partial;
                    response.issues.push(issue);
                }
            }
        }
    }
    Ok(response)
}
