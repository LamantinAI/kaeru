use serde_json::{Value, json};

use crate::api::access::resolve;
use crate::api::graph::supersession_reaches;
use crate::api::read::{entity, relations};
use crate::api::types::{Entity, Issue, MutationData, Response, State};
use crate::api::write::{assign_slot, changed, check_revision, result, target};
use crate::api::{ApiOptions, ApiResult, Args, hash, issue_from_error};
use crate::{
    EdgeType, Store, forget as core_forget, link_with_weight, pin as core_pin, read_node_full,
    set_edge_weight, slots_in, unlink, unpin,
};

pub(in crate::api) fn relate(
    store: &Store,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    let from = resolve(store, &args.parse("from")?, options, None, false, true)?;
    let to = resolve(store, &args.parse("to")?, options, None, false, true)?;
    let kind = args
        .required_text("type")?
        .parse::<EdgeType>()
        .map_err(issue_from_error)?;
    let action = args.action("");
    let previous = relations(store, &from, options)?
        .into_iter()
        .find(|e| e.from == from && e.to == to && e.edge_type == kind.as_str());
    let revision = previous
        .as_ref()
        .map(|edge| edge.revision.clone())
        .unwrap_or_else(|| hash(&Value::Null));
    check_revision(args, &revision)?;
    if action == "connect"
        && kind == EdgeType::Supersedes
        && (from == to || supersession_reaches(store, &to, &from)?)
    {
        return Err(Issue::new(
            "INVALID_ARGUMENT",
            "supersession cannot point to itself or form a cycle",
        ));
    }
    let weight = args.value("weight").as_f64();
    let no_change = action == "disconnect" && previous.is_none()
        || action != "disconnect" && previous.as_ref().is_some_and(|e| Some(e.weight) == weight);
    if !no_change {
        match action.as_str() {
            "disconnect" => unlink(store, &from, &to, kind),
            "set_weight" => set_edge_weight(store, &from, &to, kind, weight.unwrap()),
            _ => link_with_weight(store, &from, &to, kind, weight.unwrap()),
        }
        .map_err(issue_from_error)?;
    }
    let current = relations(store, &from, options)?
        .into_iter()
        .find(|e| e.from == from && e.to == to && e.edge_type == kind.as_str());
    let mut r = Response::mutation(
        "relate",
        MutationData {
            primary: Some(Entity {
                id: format!("{from}|{to}|{}", kind.as_str()),
                revision: current
                    .as_ref()
                    .map(|edge| edge.revision.clone())
                    .unwrap_or_else(|| hash(&Value::Null)),
            }),
            changes: vec![changed("edge", json!(previous), json!(current))],
            ..MutationData::default()
        },
    );
    if no_change {
        r.state = State::Unchanged;
    }
    Ok(r)
}

pub(in crate::api) fn pin(store: &Store, args: &Args, options: &ApiOptions) -> ApiResult<Response> {
    let id = target(store, args, "node", None, options)?;
    let enabled = args.value("enabled").as_bool().unwrap();
    if enabled {
        core_pin(store, &id, &args.required_text("reason")?)
    } else {
        unpin(store, &id)
    }
    .map_err(issue_from_error)?;
    result(store, "pin", &id, false, options)
}

pub(in crate::api) fn forget(
    store: &Store,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    let id = resolve(store, &args.parse("node")?, options, None, true, false)?;
    if read_node_full(store, &id)
        .map_err(issue_from_error)?
        .is_none()
    {
        return Ok(Response {
            state: State::Unchanged,
            ..Response::mutation("forget", MutationData::default())
        });
    }
    check_revision(args, &entity(store, &id, options)?.revision)?;
    unpin(store, &id).map_err(issue_from_error)?;
    let scope = args.required_text("initiative")?;
    for (slot, holder) in slots_in(store, &scope).map_err(issue_from_error)? {
        if holder == id {
            assign_slot(store, &scope, &slot, None, None, options)?;
        }
    }
    core_forget(store, &id).map_err(issue_from_error)?;
    Ok(Response::mutation(
        "forget",
        MutationData {
            changes: vec![changed("retracted", json!(false), json!(true))],
            ..MutationData::default()
        },
    ))
}
