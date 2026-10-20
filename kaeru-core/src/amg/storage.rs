//! Parameterized storage reads shared by snapshot capture and import planning.

use std::collections::{BTreeMap, BTreeSet};

use chrono::{DateTime, SecondsFormat, Utc};
use cozo::{DataValue, NamedRows, ScriptMutability};
use serde_json::Value;

use super::validate::invalid;
use crate::graph::temporal::parse_validity;
use crate::models::amg::{AmgEdge, AmgNode, AmgOrigin};
use crate::{Result, Store};

pub(super) fn query(
    store: &Store,
    script: &str,
    params: BTreeMap<String, DataValue>,
) -> Result<NamedRows> {
    Ok(store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?)
}
pub(super) fn write(
    store: &Store,
    script: &str,
    params: BTreeMap<String, DataValue>,
) -> Result<()> {
    store
        .db_ref()
        .run_script(script, params, ScriptMutability::Mutable)?;
    Ok(())
}
pub(super) fn string(value: &DataValue) -> Result<String> {
    value
        .get_str()
        .map(str::to_string)
        .ok_or_else(|| invalid("storage text column has wrong type"))
}
pub(super) fn strings(value: &DataValue) -> Result<Vec<String>> {
    match value {
        DataValue::Null => Ok(vec![]),
        DataValue::List(values) => values.iter().map(string).collect(),
        _ => Err(invalid("storage list column has wrong type")),
    }
}
pub(super) fn json(value: &DataValue) -> Value {
    Value::from(value.clone())
}
pub(super) fn time(seconds: f64) -> Result<String> {
    if !seconds.is_finite() {
        return Err(invalid("invalid source timestamp"));
    }
    DateTime::<Utc>::from_timestamp(seconds as i64, 0)
        .map(|date| date.to_rfc3339_opts(SecondsFormat::Secs, true))
        .ok_or_else(|| invalid("source timestamp out of range"))
}
pub(super) fn version(time: Option<&String>) -> Result<i64> {
    let value = time.ok_or_else(|| invalid("current storage row lacks recorded timestamp"))?;
    Ok(DateTime::parse_from_rfc3339(value)
        .map_err(|_| invalid("invalid recorded timestamp"))?
        .timestamp())
}
pub(super) fn ids_param(ids: &BTreeSet<String>) -> BTreeMap<String, DataValue> {
    BTreeMap::from([(
        "ids".into(),
        DataValue::List(
            ids.iter()
                .map(|id| DataValue::List(vec![DataValue::Str(id.clone().into())]))
                .collect(),
        ),
    )])
}
pub(super) fn memberships(store: &Store, edges: bool) -> Result<BTreeMap<String, Vec<String>>> {
    let script = if edges {
        "?[key,initiative] := *edge_initiative{edge_pk:key,initiative}"
    } else {
        "?[key,initiative] := *node_initiative{node_id:key,initiative}"
    };
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for row in query(store, script, BTreeMap::new())?.rows {
        out.entry(string(&row[0])?)
            .or_default()
            .push(string(&row[1])?);
    }
    for values in out.values_mut() {
        values.sort();
        values.dedup();
    }
    Ok(out)
}
pub(super) fn edge_key(src: &str, dst: &str, kind: &str) -> String {
    serde_json::to_string(&[src, dst, kind]).expect("string tuple serializes")
}
pub(super) fn legacy_edge_key(src: &str, dst: &str, kind: &str) -> String {
    format!("{src}|{dst}|{kind}")
}

fn origins(
    store: &Store,
    kind: &str,
    versions: &BTreeMap<String, i64>,
) -> Result<BTreeMap<(String, i64), Vec<AmgOrigin>>> {
    let mut out = BTreeMap::new();
    if versions.is_empty() {
        return Ok(out);
    }
    let wanted = DataValue::List(
        versions
            .iter()
            .map(|(key, version)| {
                DataValue::List(vec![
                    DataValue::Str(key.clone().into()),
                    DataValue::from(*version),
                ])
            })
            .collect(),
    );
    for row in query(store, "wanted[key,version] <- $versions\n ?[key,version,origins] := wanted[key,version],*amg_origin{kind,key,version,origins},kind=$kind",
        BTreeMap::from([("kind".into(), DataValue::Str(kind.into())),("versions".into(),wanted)]))?.rows {
        out.insert((string(&row[0])?, row[1].get_int().ok_or_else(|| invalid("invalid provenance version"))?),
            serde_json::from_value(json(&row[2])).map_err(|_| invalid("invalid stored AMG provenance"))?);
    }
    Ok(out)
}

pub(super) fn nodes(store: &Store, ids: &BTreeSet<String>) -> Result<BTreeMap<String, AmgNode>> {
    if ids.is_empty() {
        return Ok(BTreeMap::new());
    }
    let memberships = memberships(store, false)?;
    let mut created = BTreeMap::<String, f64>::new();
    for row in query(
        store,
        "allowed[id] <- $ids\n ?[id,validity] := allowed[id],*node{id,validity}",
        ids_param(ids),
    )?
    .rows
    {
        let (seconds, asserted) = parse_validity(row.get(1))?;
        if asserted {
            created
                .entry(string(&row[0])?)
                .and_modify(|old| *old = old.min(seconds))
                .or_insert(seconds);
        }
    }
    let mut out = BTreeMap::new();
    for row in query(store, "allowed[id] <- $ids\n ?[id,type,tier,name,body,tags,properties,visibility,layer,validity] := allowed[id],*node{id,type,tier,name,body,tags,properties,visibility,layer,validity @ 'NOW'}", ids_param(ids))?.rows {
        let id = string(&row[0])?; let (seconds, _) = parse_validity(row.get(9))?;
        out.insert(id.clone(), AmgNode { id: id.clone(), node_type: string(&row[1])?, tier: string(&row[2])?, name: string(&row[3])?,
            body: row[4].get_str().map(str::to_string), tags: strings(&row[5])?, properties: json(&row[6]),
            visibility: string(&row[7])?, layer: string(&row[8])?, initiatives: memberships.get(&id).cloned().unwrap_or_default(),
            recorded_at: Some(time(seconds)?), created_at: created.get(&id).map(|s| time(*s)).transpose()?,
            origins: vec![] });
    }
    let versions = out
        .iter()
        .map(|(id, n)| Ok((id.clone(), version(n.recorded_at.as_ref())?)))
        .collect::<Result<BTreeMap<_, _>>>()?;
    let mut origins = origins(store, "node", &versions)?;
    for (id, node) in out.iter_mut() {
        node.origins = origins
            .remove(&(id.clone(), versions[id]))
            .unwrap_or_default();
    }
    Ok(out)
}

pub(super) fn edges(store: &Store, ids: &BTreeSet<String>) -> Result<BTreeMap<String, AmgEdge>> {
    if ids.is_empty() {
        return Ok(BTreeMap::new());
    }
    let memberships = memberships(store, true)?;
    let mut created = BTreeMap::<String, f64>::new();
    for row in query(store, "allowed[id] <- $ids\n ?[src,dst,edge_type,validity] := allowed[src],allowed[dst],*edge{src,dst,edge_type,validity}", ids_param(ids))?.rows {
        let (seconds, asserted) = parse_validity(row.get(3))?;
        if asserted {
            let key = edge_key(&string(&row[0])?, &string(&row[1])?, &string(&row[2])?);
            created.entry(key).and_modify(|old| *old=old.min(seconds)).or_insert(seconds);
        }
    }
    let mut out = BTreeMap::new();
    for row in query(store, "allowed[id] <- $ids\n ?[src,dst,edge_type,weight,properties,validity] := allowed[src],allowed[dst],*edge{src,dst,edge_type,weight,properties,dst_store:'local',validity @ 'NOW'}", ids_param(ids))?.rows {
        let src=string(&row[0])?; let dst=string(&row[1])?; let kind=string(&row[2])?;
        let key=edge_key(&src,&dst,&kind); let legacy=legacy_edge_key(&src,&dst,&kind); let (seconds, _) = parse_validity(row.get(5))?;
        out.insert(key.clone(), AmgEdge { src,dst,edge_type:kind, weight: row[3].get_float().ok_or_else(|| invalid("invalid edge weight"))?,
            properties: json(&row[4]), initiatives: memberships.get(&legacy).cloned().unwrap_or_default(),
            recorded_at: Some(time(seconds)?), created_at: created.get(&key).map(|s| time(*s)).transpose()?,
            origins: vec![] });
    }
    let versions = out
        .iter()
        .map(|(id, e)| Ok((id.clone(), version(e.recorded_at.as_ref())?)))
        .collect::<Result<BTreeMap<_, _>>>()?;
    let mut origins = origins(store, "edge", &versions)?;
    for (id, edge) in out.iter_mut() {
        edge.origins = origins
            .remove(&(id.clone(), versions[id]))
            .unwrap_or_default();
    }
    Ok(out)
}

pub(super) fn check_legacy_keys(store: &Store, edges: &[AmgEdge]) -> Result<()> {
    let mut wanted = BTreeMap::new();
    for edge in edges {
        wanted.insert(
            legacy_edge_key(&edge.src, &edge.dst, &edge.edge_type),
            (edge.src.clone(), edge.dst.clone(), edge.edge_type.clone()),
        );
    }
    if wanted.is_empty() {
        return Ok(());
    }
    for row in query(
        store,
        "?[src,dst,edge_type] := *edge{src,dst,edge_type}",
        BTreeMap::new(),
    )?
    .rows
    {
        let triple = (string(&row[0])?, string(&row[1])?, string(&row[2])?);
        if wanted
            .get(&legacy_edge_key(&triple.0, &triple.1, &triple.2))
            .is_some_and(|expected| expected != &triple)
        {
            return Err(invalid(
                "ambiguous legacy edge membership key in destination",
            ));
        }
    }
    Ok(())
}
