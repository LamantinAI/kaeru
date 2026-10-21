//! Demand reads share the same values as the bulk loader and ordinary primitives.
use std::collections::{BTreeMap, BTreeSet};

use cozo::{DataValue, NamedRows, ScriptMutability};
use serde_json::Value;

use super::{current, current_at, memo};
use crate::api::control::checkpoint;
use crate::api::{ApiResult, issue_from_error};
use crate::config::api::API_READ_BATCH_SIZE;
use crate::graph::temporal::parse_validity;
use crate::{Error, NodeFull, Store, initiatives_of_node, read_node_full};

pub(super) fn query(
    store: &Store,
    sql: &str,
    params: BTreeMap<String, DataValue>,
) -> ApiResult<NamedRows> {
    checkpoint()?;
    let rows = store
        .db_ref()
        .run_script(sql, params, ScriptMutability::Immutable)
        .map_err(Error::from)
        .map_err(issue_from_error)?;
    checkpoint()?;
    Ok(rows)
}
fn key(id: &str) -> BTreeMap<String, DataValue> {
    BTreeMap::from([("id".into(), DataValue::Str(id.into()))])
}
pub(in crate::api) fn memberships(store: &Store, id: &str) -> crate::Result<Vec<String>> {
    memo(
        store,
        None,
        |s| s.members.get(id).cloned(),
        |s, v| {
            s.members.insert(id.into(), v);
        },
        || initiatives_of_node(store, &id.into()),
    )
}
pub(in crate::api) fn raw_node(store: &Store, id: &str) -> crate::Result<Option<NodeFull>> {
    memo(
        store,
        None,
        |s| s.raw.get(id).cloned(),
        |s, v| {
            if s.raw.len() >= API_READ_BATCH_SIZE * 2 {
                s.raw.clear();
            }
            s.raw.insert(id.into(), v);
        },
        || read_node_full(store, &id.into()),
    )
}
pub(in crate::api) fn visibility(
    store: &Store,
    id: &str,
    historical: bool,
) -> ApiResult<Option<String>> {
    if historical {
        return memo(
            store,
            None,
            |s| s.historical_visible.get(id).cloned(),
            |s, v| {
                s.historical_visible.insert(id.into(), v);
            },
            || {
                let rows = query(
                    store,
                    "?[visibility] := *node{id:$id,visibility,validity} :limit 1",
                    key(id),
                )?;
                Ok(rows
                    .rows
                    .first()
                    .and_then(|r| r.first()?.get_str().map(str::to_string)))
            },
        );
    }
    memo(
        store,
        None,
        |s| s.visible.get(id).cloned(),
        |s, v| {
            s.visible.insert(id.into(), v);
        },
        || {
            let generation = store.generation();
            let rows = query(
                store,
                "?[visibility,validity] := *node{id:$id,visibility,validity @ 'NOW'}",
                key(id),
            )?;
            let recorded = rows
                .rows
                .first()
                .map(|r| parse_validity(r.get(1)).map(|(seconds, _)| seconds))
                .transpose()
                .map_err(issue_from_error)?;
            current_at(store, None, generation, |s| {
                s.recorded.insert(id.into(), recorded);
            });
            Ok(rows
                .rows
                .first()
                .and_then(|r| r.first()?.get_str().map(str::to_string)))
        },
    )
}
pub(in crate::api) fn recorded(store: &Store, id: &str) -> ApiResult<Option<f64>> {
    if let Some(Some(value)) = current(store, None, |s| s.recorded.get(id).cloned()) {
        return Ok(value);
    }
    if current(store, None, |_| ()).is_some() {
        visibility(store, id, false)?;
    }
    if let Some(Some(value)) = current(store, None, |s| s.recorded.get(id).cloned()) {
        return Ok(value);
    }
    let rows = query(
        store,
        "?[validity] := *node{id:$id,validity @ 'NOW'}",
        key(id),
    )?;
    rows.rows
        .first()
        .map(|r| parse_validity(r.first()).map(|(time, _)| time))
        .transpose()
        .map_err(issue_from_error)
}
pub(in crate::api) fn created(store: &Store, id: &str) -> ApiResult<Option<f64>> {
    memo(
        store,
        None,
        |s| s.created.get(id).cloned(),
        |s, v| {
            s.created.insert(id.into(), v);
        },
        || {
            let rows = query(store, "?[validity] := *node{id:$id,validity}", key(id))?;
            Ok(rows
                .rows
                .iter()
                .filter_map(|row| parse_validity(row.first()).ok())
                .filter(|(_, asserted)| *asserted)
                .map(|(time, _)| time)
                .min_by(f64::total_cmp))
        },
    )
}
pub(in crate::api) fn pin(store: &Store, id: &str) -> ApiResult<Option<Value>> {
    memo(
        store,
        None,
        |s| s.pins.get(id).cloned(),
        |s, v| {
            s.pins.insert(id.into(), v);
        },
        || {
            let rows = query(
                store,
                "?[reason] := *session_pin{node_id:$id,reason}",
                key(id),
            )?;
            Ok(rows
                .rows
                .first()
                .and_then(|r| r.first())
                .map(|v| Value::from(v.clone())))
        },
    )
}
pub(in crate::api) fn edge_rows(store: &Store, id: &str) -> ApiResult<Vec<Vec<DataValue>>> {
    memo(
        store,
        None,
        |s| s.edges.get(id).cloned(),
        |s, v| {
            if s.edges.len() >= API_READ_BATCH_SIZE * 2 {
                s.edges.clear();
            }
            s.edges.insert(id.into(), v);
        },
        || {
            let rows = query(
                store,
                "incident[src,dst,edge_type,validity] := src=$id,*edge{src,dst,edge_type,validity @ 'NOW'}\nincident[src,dst,edge_type,validity] := dst=$id,*edge{src,dst,edge_type,validity @ 'NOW'}\n?[src,dst,edge_type,weight,properties,dst_store] := incident[src,dst,edge_type,validity],*edge{src,dst,edge_type,validity,weight,properties,dst_store}",
                key(id),
            )?;
            if current(store, None, |_| ()).is_some() {
                let mut ids = BTreeSet::new();
                for row in &rows.rows {
                    if let Some(id) = row[0].get_str() {
                        ids.insert(id.to_string());
                    }
                    if row[5].get_str() == Some("local")
                        && let Some(id) = row[1].get_str()
                    {
                        ids.insert(id.to_string());
                    }
                }
                for batch in ids
                    .into_iter()
                    .collect::<Vec<_>>()
                    .chunks(API_READ_BATCH_SIZE)
                {
                    super::bulk::metadata(store, batch)?;
                }
            }
            Ok(rows.rows)
        },
    )
}
