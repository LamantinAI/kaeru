//! Intrinsic local commands. Supplements run only after the local transaction commits.

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};
use serde_json::{Value, from_value, json};

use self::capture::dispatch as capture_dispatch;
use self::manage::{
    dispatch as manage_dispatch, policy, retention, run_hygiene as manage_run_hygiene,
};
use self::state::dispatch as state_dispatch;
use super::access::{check_node, resolve};
use super::read::{entity, node};
use super::types::{
    Change, FailedStep, Issue, MutationData, NodeRef, Response, ResponseData, SlotChange, State,
};
use super::{ApiOptions, ApiResult, Args, hash, issue_from_error};
use crate::graph::audit::write_audit_details;
use crate::{EdgeType, Error, Store, link_with_weight, new_node_id, release_slot, slot_holder};

mod capture;
mod manage;
mod state;

pub(super) fn dispatch(
    store: &Store,
    name: &str,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    match name {
        "note" | "reference" | "task" | "claim" => capture_dispatch(store, name, args, options),
        "verdict" | "task_update" | "revise" | "replace" | "lifecycle" | "relate" | "review"
        | "trail" | "pin" | "forget" | "synthesise" => state_dispatch(store, name, args, options),
        "initiative_manage" | "board_config" | "retention" | "policy" | "export" => {
            manage_dispatch(store, name, args, options)
        }
        _ => Err(Issue::new(
            "UNKNOWN_TOOL",
            "operation is not a canonical local mutation",
        )),
    }
}

fn target(
    store: &Store,
    args: &Args,
    key: &str,
    kind: Option<&str>,
    options: &ApiOptions,
) -> ApiResult<String> {
    let id = resolve(
        store,
        &args.parse::<NodeRef>(key)?,
        options,
        None,
        false,
        false,
    )?;
    if let Some(kind) = kind {
        let n = node(store, &id, options, false)?;
        if n.node_type != kind {
            return Err(Issue::new(
                "WRONG_NODE_TYPE",
                format!("expected {kind}, got {}", n.node_type),
            )
            .field(key));
        }
    }
    check_revision(args, &entity(store, &id, options)?.revision)?;
    Ok(id)
}

fn check_revision(args: &Args, revision: &str) -> ApiResult<()> {
    if args
        .text("expected_revision")
        .is_some_and(|v| v != revision)
    {
        return Err(
            Issue::new("CONFLICT", "the target changed; read its current revision")
                .field("expected_revision"),
        );
    }
    Ok(())
}

fn changed(field: &str, before: Value, after: Value) -> Change {
    Change {
        field: field.into(),
        before,
        after,
    }
}

fn result(
    store: &Store,
    name: &str,
    id: &str,
    created: bool,
    options: &ApiOptions,
) -> ApiResult<Response> {
    let e = entity(store, id, options)?;
    let mut data = MutationData {
        primary: Some(e.clone()),
        ..MutationData::default()
    };
    if created {
        data.created.push(e);
    }
    Ok(Response::mutation(name, data))
}

pub(super) fn slot_revision(store: &Store, scope: &str, slot: &str) -> ApiResult<String> {
    let rows=store.db_ref().run_script("?[revision] := *api_slot_revision{initiative, slot, revision}, initiative = $scope, slot = $slot",BTreeMap::from([("scope".into(),DataValue::Str(scope.into())),("slot".into(),DataValue::Str(slot.into()))]),ScriptMutability::Immutable).map_err(Error::from).map_err(issue_from_error)?;
    let epoch = rows
        .rows
        .first()
        .and_then(|r| r.first())
        .and_then(DataValue::get_str);
    let holder=store.db_ref().run_script("?[node_id,set_at] := *slot_occupant{initiative,slot,node_id,set_at},initiative=$scope,slot=$slot",BTreeMap::from([("scope".into(),DataValue::Str(scope.into())),("slot".into(),DataValue::Str(slot.into()))]),ScriptMutability::Immutable).map_err(Error::from).map_err(issue_from_error)?;
    let holder = holder
        .rows
        .first()
        .map(|r| json!([Value::from(r[0].clone()), Value::from(r[1].clone())]));
    Ok(hash(&json!([scope, slot, holder, epoch])))
}

fn assign_slot(
    store: &Store,
    scope: &str,
    slot: &str,
    id: Option<&str>,
    expected: Option<&str>,
    options: &ApiOptions,
) -> ApiResult<SlotChange> {
    let revision = slot_revision(store, scope, slot)?;
    if expected.is_some_and(|r| r != revision) {
        return Err(Issue::new("CONFLICT", "the current slot holder changed")
            .field("expected_slot_revision"));
    }
    if let Some(id) = id {
        check_node(store, id, Some(scope), options, false)?;
    }
    let before = slot_holder(store, scope, slot).map_err(issue_from_error)?;
    let after = id.map(str::to_string);
    if before == after {
        return Ok(SlotChange {
            slot: slot.into(),
            before,
            after,
            revision,
        });
    }
    if let Some(id) = id {
        store.db_ref().run_script("?[initiative, slot, node_id] <- [[$scope, $slot, $id]] :put slot_occupant {initiative, slot => node_id}",BTreeMap::from([("scope".into(),DataValue::Str(scope.into())),("slot".into(),DataValue::Str(slot.into())),("id".into(),DataValue::Str(id.into()))]),ScriptMutability::Mutable).map_err(Error::from).map_err(issue_from_error)?;
    } else {
        release_slot(store, scope, slot).map_err(issue_from_error)?;
    }
    let revision = new_node_id();
    store.db_ref().run_script("?[initiative, slot, revision] <- [[$scope,$slot,$revision]] :put api_slot_revision {initiative, slot => revision}",BTreeMap::from([("scope".into(),DataValue::Str(scope.into())),("slot".into(),DataValue::Str(slot.into())),("revision".into(),DataValue::Str(revision.clone().into()))]),ScriptMutability::Mutable).map_err(Error::from).map_err(issue_from_error)?;
    let revision = slot_revision(store, scope, slot)?;
    write_audit_details(
        store.db_ref(),
        "set_slot_pointer",
        "system",
        &after
            .clone()
            .into_iter()
            .chain(before.clone())
            .collect::<Vec<_>>(),
        json!({"initiative":scope,"slot":slot,"before":before,"after":after,"revision":revision}),
    )
    .map_err(issue_from_error)?;
    Ok(SlotChange {
        slot: slot.into(),
        before,
        after,
        revision,
    })
}

fn capture_slot(
    store: &Store,
    args: &Args,
    id: &str,
    options: &ApiOptions,
) -> ApiResult<Vec<SlotChange>> {
    let config = args.value("options");
    if let Some(slot) = config.get("slot").and_then(Value::as_str) {
        Ok(vec![assign_slot(
            store,
            &args.required_text("initiative")?,
            slot,
            Some(id),
            config.get("expected_slot_revision").and_then(Value::as_str),
            options,
        )?])
    } else {
        Ok(vec![])
    }
}

pub(super) fn supplements(
    store: &Store,
    name: &str,
    args: &Args,
    options: &ApiOptions,
    mut response: Response,
) -> ApiResult<Response> {
    if !matches!(name, "note" | "reference" | "task") {
        return Ok(response);
    }
    if let Some(related) = args.0.get("related") {
        let primary = if let ResponseData::Mutation(d) = &response.data {
            d.primary.as_ref().unwrap().id.clone()
        } else {
            unreachable!()
        };
        let supplement = (|| {
            let to: NodeRef = from_value(related["to"].clone())
                .map_err(|e| Issue::new("INVALID_ARGUMENT", e.to_string()))?;
            let id = resolve(store, &to, options, None, false, true)?;
            let kind = related["type"]
                .as_str()
                .unwrap()
                .parse::<EdgeType>()
                .map_err(issue_from_error)?;
            link_with_weight(
                store,
                &primary,
                &id,
                kind,
                related["weight"].as_f64().unwrap(),
            )
            .map_err(issue_from_error)
        })();
        if let Err(issue) = supplement {
            if let ResponseData::Mutation(data) = &mut response.data {
                data.failed_steps.push(FailedStep {
                    step: "related".into(),
                    code: issue.code.clone(),
                    message: issue.message.clone(),
                });
            }
            response.state = State::Partial;
            response.issues.push(issue);
        }
    }
    Ok(response)
}

pub(super) fn policy_read(store: &Store, args: &Args) -> ApiResult<Response> {
    policy(store, args)
}
pub(super) fn retention_preview(
    store: &Store,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    retention(store, args, options)
}
pub(super) fn run_hygiene(store: &Store, args: &Args, options: &ApiOptions) -> ApiResult<Response> {
    manage_run_hygiene(store, args, options)
}
