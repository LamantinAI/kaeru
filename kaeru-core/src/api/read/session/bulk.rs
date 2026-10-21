//! Bulk loads authorize lightweight metadata before loading any record bodies.
use std::collections::{BTreeMap, BTreeSet};

use cozo::DataValue;
use serde_json::Value;

use super::fields::query;
use super::{current, current_at};
use crate::Store;
use crate::api::access::check_node;
use crate::api::{ApiOptions, ApiResult, issue_from_error};
use crate::config::api::API_READ_BATCH_SIZE;
use crate::graph::temporal::parse_validity;
use crate::recall::by_name::full_from_row;

fn params(ids: &[String]) -> BTreeMap<String, DataValue> {
    BTreeMap::from([(
        "ids".into(),
        DataValue::List(
            ids.iter()
                .map(|id| DataValue::List(vec![DataValue::Str(id.clone().into())]))
                .collect(),
        ),
    )])
}
fn text(value: &DataValue) -> String {
    value.get_str().unwrap_or_default().to_string()
}
pub(super) fn metadata(store: &Store, ids: &[String]) -> ApiResult<()> {
    let missing = ids
        .iter()
        .filter(|id| {
            !current(store, None, |s| {
                s.visible.contains_key(*id) && s.members.contains_key(*id)
            })
            .unwrap_or(false)
        })
        .cloned()
        .collect::<Vec<_>>();
    if missing.is_empty() {
        return Ok(());
    }
    let generation = store.generation();
    let input = params(&missing);
    let visible = query(
        store,
        "allowed[id] <- $ids\n?[id,visibility,validity] := allowed[id],*node{id,visibility,validity @ 'NOW'}",
        input.clone(),
    )?;
    let members = query(
        store,
        "allowed[id] <- $ids\n?[id,initiative] := allowed[id],*node_initiative{node_id:id,initiative}",
        input,
    )?;
    current_at(store, None, generation, |s| {
        for id in &missing {
            s.visible.insert(id.clone(), None);
            s.recorded.insert(id.clone(), None);
            s.members.insert(id.clone(), vec![]);
        }
        for row in visible.rows {
            let id = text(&row[0]);
            s.visible.insert(id.clone(), Some(text(&row[1])));
            s.recorded
                .insert(id, parse_validity(row.get(2)).ok().map(|(time, _)| time));
        }
        for row in members.rows {
            s.members
                .entry(text(&row[0]))
                .or_default()
                .push(text(&row[1]));
        }
    });
    Ok(())
}

pub(in crate::api) fn prefetch(
    store: &Store,
    ids: &[String],
    options: &ApiOptions,
) -> ApiResult<()> {
    if ids.is_empty() || current(store, Some(options), |_| ()).is_none() {
        return Ok(());
    }
    metadata(store, ids)?;
    let allowed = ids
        .iter()
        .filter(|id| check_node(store, id, None, options, false).is_ok())
        .cloned()
        .collect::<Vec<_>>();
    if allowed.is_empty() {
        return Ok(());
    }
    let generation = store.generation();
    let input = params(&allowed);
    let nodes = query(
        store,
        "allowed[id] <- $ids\n?[id,type,tier,name,body,tags,visibility,layer,properties] := allowed[id],*node{id,type,tier,name,body,tags,visibility,layer,properties @ 'NOW'}",
        input.clone(),
    )?;
    let histories = query(
        store,
        "allowed[id] <- $ids\n?[id,validity] := allowed[id],*node{id,validity}",
        input.clone(),
    )?;
    let pins = query(
        store,
        "allowed[id] <- $ids\n?[id,reason] := allowed[id],*session_pin{node_id:id,reason}",
        input.clone(),
    )?;
    // Keep validity intact. A non-covering temporal index join-back panics in
    // Cozo's mem backend; covering keys + ordinary exact-version reads do not.
    let edges = query(
        store,
        "allowed[id] <- $ids\nincident[src,dst,edge_type,validity] := allowed[src],*edge{src,dst,edge_type,validity @ 'NOW'}\nincident[src,dst,edge_type,validity] := allowed[dst],*edge{src,dst,edge_type,validity @ 'NOW'}\n?[src,dst,edge_type,weight,properties,dst_store] := incident[src,dst,edge_type,validity],*edge{src,dst,edge_type,validity,weight,properties,dst_store}",
        input,
    )?;
    let mut endpoints = BTreeSet::new();
    for row in &edges.rows {
        endpoints.insert(text(&row[0]));
        if row[5].get_str() == Some("local") {
            endpoints.insert(text(&row[1]));
        }
    }
    for chunk in endpoints
        .into_iter()
        .collect::<Vec<_>>()
        .chunks(API_READ_BATCH_SIZE)
    {
        metadata(store, chunk)?;
    }
    let mut created = BTreeMap::<String, f64>::new();
    for row in histories.rows {
        let (time, asserted) = parse_validity(row.get(1)).map_err(issue_from_error)?;
        if asserted {
            created
                .entry(text(&row[0]))
                .and_modify(|old| *old = old.min(time))
                .or_insert(time);
        }
    }
    current_at(store, Some(options), generation, |s| {
        // Bound heavy bodies, edge rows and assembled relations to this batch.
        // Lightweight authorization metadata can be reused across batches.
        s.raw.clear();
        s.edges.clear();
        s.relations.clear();
        for id in &allowed {
            s.raw.insert(id.clone(), None);
            s.created.insert(id.clone(), created.get(id).copied());
            s.pins.insert(id.clone(), None);
            s.edges.insert(id.clone(), vec![]);
        }
        for row in nodes.rows {
            let id = text(&row[0]);
            s.raw
                .insert(id.clone(), Some(full_from_row(&id, &row[1..])));
        }
        for row in pins.rows {
            s.pins
                .insert(text(&row[0]), Some(Value::from(row[1].clone())));
        }
        for row in edges.rows {
            let src = text(&row[0]);
            let dst = text(&row[1]);
            if let Some(rows) = s.edges.get_mut(&src) {
                rows.push(row.clone());
            }
            if dst != src
                && let Some(rows) = s.edges.get_mut(&dst)
            {
                rows.push(row);
            }
        }
    });
    Ok(())
}
