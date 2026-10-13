//! Bounded retrieval with currentness and stored reasoning on the ordinary path.

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};
use serde_json::{Value, json};

use crate::api::access::check_node;
use crate::api::types::{ContentRange, Issue, TrailData};
use crate::api::{ApiOptions, ApiResult, hash, issue_from_error};
use crate::{Error, Store, read_node_full};

pub(in crate::api) fn content_hash(store: &Store, id: &str) -> ApiResult<String> {
    let n = read_node_full(store, &id.into())
        .map_err(issue_from_error)?
        .ok_or_else(|| Issue::new("NOT_FOUND", "trail member not live"))?;
    Ok(hash(&json!([n.name, n.body])))
}

pub(in crate::api) fn trails(
    store: &Store,
    id: &str,
    options: &ApiOptions,
    expanded: bool,
) -> ApiResult<Vec<TrailData>> {
    let root = read_node_full(store, &id.into())
        .map_err(issue_from_error)?
        .ok_or_else(|| Issue::new("NOT_FOUND", "record is not live"))?;
    let mut chain_ids = store
        .db_ref()
        .run_script(
            "?[chain_id] := *chain_member{chain_id,node_id},node_id=$id",
            BTreeMap::from([("id".into(), DataValue::Str(id.into()))]),
            ScriptMutability::Immutable,
        )
        .map_err(Error::from)
        .map_err(issue_from_error)?
        .rows
        .into_iter()
        .filter_map(|r| r.first().and_then(DataValue::get_str).map(str::to_string))
        .collect::<Vec<_>>();
    if root.node_type == "chain" {
        chain_ids.insert(0, id.into());
    }
    let mut out = vec![];
    for chain_id in chain_ids {
        if check_node(store, &chain_id, None, options, false).is_err() {
            continue;
        }
        let n = read_node_full(store, &chain_id)
            .map_err(issue_from_error)?
            .unwrap();
        let rows=store.db_ref().run_script("?[node_id,position] := *chain_member{chain_id,position,node_id},chain_id=$id :order position",BTreeMap::from([("id".into(),DataValue::Str(chain_id.clone().into()))]),ScriptMutability::Immutable).map_err(Error::from).map_err(issue_from_error)?;
        let all_members = rows
            .rows
            .into_iter()
            .filter_map(|r| r.first().and_then(DataValue::get_str).map(str::to_string))
            .collect::<Vec<_>>();
        let members = all_members
            .iter()
            .filter(|id| check_node(store, id, None, options, false).is_ok())
            .cloned()
            .collect::<Vec<_>>();
        let withheld = members.len() != all_members.len();
        let snapshot=store.db_ref().run_script("?[fingerprints, stale] := *api_trail_summary{chain_id, fingerprints, stale}, chain_id = $id",BTreeMap::from([("id".into(),DataValue::Str(n.id.clone().into()))]),ScriptMutability::Immutable).map_err(Error::from).map_err(issue_from_error)?;
        let stale = if withheld {
            None
        } else if let Some(row) = snapshot.rows.first() {
            let fingerprints = Value::from(row[0].clone());
            Some(
                row[1].get_bool().unwrap_or(true)
                    || members.iter().any(|id| {
                        content_hash(store, id).ok().as_deref()
                            != fingerprints.get(id).and_then(Value::as_str)
                    })
                    || fingerprints
                        .as_object()
                        .is_some_and(|f| f.len() != members.len()),
            )
        } else {
            None
        };
        let summary_range = n
            .body
            .as_ref()
            .filter(|b| b.chars().count() > 400 && chain_id != id)
            .map(|b| ContentRange {
                start: 0,
                end: 400,
                total: b.chars().count(),
            });
        out.push(TrailData {
            id: n.id,
            name: n.name,
            summary: if chain_id == id {
                None
            } else {
                n.body.map(|b| b.chars().take(400).collect())
            },
            stale,
            members: if expanded { members } else { vec![] },
            summary_range,
        });
    }
    Ok(out)
}
