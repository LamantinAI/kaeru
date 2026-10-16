use std::collections::BTreeMap;
use std::slice::from_ref;

use cozo::{DataValue, JsonData, ScriptMutability};
use serde_json::{Map, Value, from_value, json};

use crate::api::access::{check_node, resolve};
use crate::api::graph::path;
use crate::api::read::{content_hash, node, relations};
use crate::api::write::{result, target};
use crate::api::{ApiOptions, ApiResult, Args, issue_from_error};
use crate::graph::audit::write_audit;
use crate::models::{Issue, MutationData, NodeRef, Response, State};
use crate::mutate::chain::create_chain_from_members;
use crate::{Error, Store, improve, read_chain};

pub(in crate::api) fn trail(
    store: &Store,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    let action = args.action("save");
    let (id, created, route_changed) = if action == "save" {
        let route = args.value("route");
        let members = if let Some(members) = route.get("members") {
            let mut ids = vec![];
            for member in members.as_array().unwrap() {
                ids.push(resolve(
                    store,
                    &from_value::<NodeRef>(member.clone()).unwrap(),
                    options,
                    None,
                    false,
                    false,
                )?);
            }
            for pair in ids.windows(2) {
                if !relations(store, &pair[0], options)?
                    .iter()
                    .any(|e| e.from == pair[0] && e.to == pair[1] && e.dst_store == "local")
                {
                    return Err(Issue::new(
                        "INVALID_ARGUMENT",
                        "explicit trail members need directed adjacent edges",
                    ));
                }
            }
            ids
        } else {
            let from = resolve(
                store,
                &from_value::<NodeRef>(route["from"].clone()).unwrap(),
                options,
                None,
                false,
                false,
            )?;
            let to = resolve(
                store,
                &from_value::<NodeRef>(route["to"].clone()).unwrap(),
                options,
                None,
                false,
                false,
            )?;
            path(store, &from, &to, options)?
        };
        let Some(outcome) = create_chain_from_members(
            store,
            &members,
            args.text("name").as_deref(),
            args.text("summary").as_deref(),
        )
        .map_err(issue_from_error)?
        else {
            let mut r = Response::mutation("trail", MutationData::default());
            r.state = State::Unchanged;
            r.issues.push(Issue::new(
                "NO_PATH",
                "no accessible directed route between these endpoints",
            ));
            return Ok(r);
        };
        (outcome.id, !outcome.reused, true)
    } else {
        let id = target(store, args, "trail", Some("chain"), options)?;
        let rows=store.db_ref().run_script("?[position,node_id] := *chain_member{chain_id,position,node_id},chain_id=$id :order position",BTreeMap::from([("id".into(),DataValue::Str(id.clone().into()))]),ScriptMutability::Immutable).map_err(Error::from).map_err(issue_from_error)?;
        let mut old = vec![];
        for row in rows.rows {
            let member = row[1].get_str().unwrap();
            check_node(store, member, None, options, false)?;
            old.push(member.to_string());
        }
        if old.len() < 2 {
            return Err(Issue::new(
                "INVALID_ARGUMENT",
                "trail needs two live endpoints",
            ));
        }
        let members = if action == "extend" {
            let to = resolve(store, &args.parse("to")?, options, None, false, false)?;
            let tail = path(store, old.last().unwrap(), &to, options)?;
            if tail.is_empty() {
                vec![]
            } else {
                old.iter()
                    .cloned()
                    .chain(tail.into_iter().skip(1))
                    .collect()
            }
        } else {
            path(store, &old[0], old.last().unwrap(), options)?
        };
        if members.len() < 2 {
            let mut r = result(store, "trail", &id, false, options)?;
            r.state = State::Unchanged;
            r.issues.push(Issue::new(
                "NO_PATH",
                "existing trail left intact: no accessible directed route",
            ));
            return Ok(r);
        }
        if members.len().saturating_sub(1) > store.config().chain_max_hops {
            return Err(Issue::new(
                "TRAVERSAL_LIMIT",
                "extended trail exceeds the configured hop cap",
            ));
        }
        let route_changed = members != old;
        if route_changed {
            replace_members(store, &id, &members)?;
            write_audit(store.db_ref(), "update_trail", "system", from_ref(&id))
                .map_err(issue_from_error)?;
        }
        if let Some(summary) = args.text("summary") {
            let n = node(store, &id, options, false)?;
            improve(store, &id, &n.name, &summary).map_err(issue_from_error)?;
        }
        (id, false, route_changed)
    };
    if args.has("summary") {
        fingerprint_trail(store, &id)?;
    } else if route_changed {
        set_trail_stale(store, &id)?;
    }
    let mut response = result(store, "trail", &id, created, options)?;
    if !route_changed && !args.has("summary") {
        response.state = State::Unchanged;
    }
    Ok(response)
}

pub(in crate::api) fn replace_members(
    store: &Store,
    id: &str,
    members: &[String],
) -> ApiResult<()> {
    let rows=store.db_ref().run_script("?[chain_id,position] := *chain_member{chain_id,position}, chain_id=$id :rm chain_member {chain_id,position}",BTreeMap::from([("id".into(),DataValue::Str(id.into()))]),ScriptMutability::Mutable).map_err(Error::from).map_err(issue_from_error)?;
    drop(rows);
    for (position, member) in members.iter().enumerate() {
        store.db_ref().run_script(&format!("?[chain_id,position,node_id] <- [[$id,{position},$member]] :put chain_member {{chain_id,position=>node_id}}"),BTreeMap::from([("id".into(),DataValue::Str(id.into())),("member".into(),DataValue::Str(member.clone().into()))]),ScriptMutability::Mutable).map_err(Error::from).map_err(issue_from_error)?;
    }
    Ok(())
}

pub(in crate::api) fn fingerprint_trail(store: &Store, id: &str) -> ApiResult<()> {
    let members = read_chain(store, &id.into()).map_err(issue_from_error)?;
    let mut fp = Map::new();
    for n in members {
        fp.insert(n.id.clone(), json!(content_hash(store, &n.id)?));
    }
    store.db_ref().run_script("?[chain_id,fingerprints,stale] <- [[$id,$fingerprints,false]] :put api_trail_summary {chain_id=>fingerprints,stale}",BTreeMap::from([("id".into(),DataValue::Str(id.into())),("fingerprints".into(),DataValue::Json(JsonData(Value::Object(fp))))]),ScriptMutability::Mutable).map_err(Error::from).map_err(issue_from_error)?;
    Ok(())
}

pub(in crate::api) fn set_trail_stale(store: &Store, id: &str) -> ApiResult<()> {
    store.db_ref().run_script("?[chain_id,fingerprints,stale] := *api_trail_summary{chain_id,fingerprints},chain_id=$id,stale=true :put api_trail_summary {chain_id=>fingerprints,stale}",BTreeMap::from([("id".into(),DataValue::Str(id.into()))]),ScriptMutability::Mutable).map_err(Error::from).map_err(issue_from_error)?;
    Ok(())
}
