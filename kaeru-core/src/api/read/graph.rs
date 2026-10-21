//! Authorized bulk structure projection; never hydrates NodeData or bodies.
use std::collections::{BTreeMap, BTreeSet};

use cozo::DataValue;

use super::session::query;
use super::time_string;
use crate::api::access::check_scope;
use crate::api::control::checkpoint;
use crate::api::{ApiOptions, ApiResult, Args, issue_from_error};
use crate::graph::temporal::parse_validity;
use crate::models::{
    GraphCounts, GraphData, GraphEdge, GraphMember, GraphNode, GraphSnapshot, Response,
    ResponseData,
};
use crate::{Store, list_initiatives, new_node_id};

fn text(value: &DataValue) -> String {
    value.get_str().unwrap_or_default().into()
}
fn ids(values: impl IntoIterator<Item = String>) -> DataValue {
    DataValue::List(
        values
            .into_iter()
            .map(|s| DataValue::List(vec![DataValue::Str(s.into())]))
            .collect(),
    )
}

pub(super) fn graph(store: &Store, args: &Args, options: &ApiOptions) -> ApiResult<Response> {
    let scopes: Vec<String> = if args.has("initiatives") {
        args.parse("initiatives")?
    } else if let Some(scope) = args.text("initiative") {
        vec![scope]
    } else {
        list_initiatives(store)
            .map_err(issue_from_error)?
            .into_iter()
            .filter(|s| check_scope(s, options).is_ok())
            .collect()
    };
    for scope in &scopes {
        check_scope(scope, options)?;
    }
    let memberships = query(
        store,
        "scopes[initiative] <- $scopes\n?[id,initiative,visibility] := scopes[initiative],*node_initiative{initiative,node_id:id},*node{id,visibility @ 'NOW'}",
        BTreeMap::from([("scopes".into(), ids(scopes.iter().cloned()))]),
    )?;
    let mut allowed = BTreeMap::<String, Vec<String>>::new();
    for row in memberships.rows {
        if options.include_local || row[2].get_str() == Some("shared") {
            allowed
                .entry(text(&row[0]))
                .or_default()
                .push(text(&row[1]));
        }
    }
    let input = BTreeMap::from([("ids".into(), ids(allowed.keys().cloned()))]);
    let rows = query(
        store,
        "allowed[id] <- $ids\n?[id,name,type,tier,layer,tags] := allowed[id],*node{id,name,type,tier,layer,tags @ 'NOW'}",
        input.clone(),
    )?;
    let mut nodes = rows
        .rows
        .into_iter()
        .map(|r| GraphNode {
            id: text(&r[0]),
            name: text(&r[1]),
            node_type: text(&r[2]),
            tier: text(&r[3]),
            layer: text(&r[4]),
            tags: match &r[5] {
                DataValue::List(tags) => tags
                    .iter()
                    .filter_map(|s| s.get_str().map(str::to_string))
                    .collect(),
                _ => vec![],
            },
            initiatives: allowed.get(&text(&r[0])).cloned().unwrap_or_default(),
            created_at: None,
        })
        .collect::<Vec<_>>();
    if args
        .optional::<Vec<String>>("include")?
        .is_some_and(|v| v.iter().any(|s| s == "created_at"))
    {
        let rows = query(
            store,
            "allowed[id] <- $ids\n?[id,validity] := allowed[id],*node{id,validity}",
            input.clone(),
        )?;
        let mut created = BTreeMap::<String, f64>::new();
        for row in rows.rows {
            let (time, asserted) = parse_validity(row.get(1)).map_err(issue_from_error)?;
            if asserted {
                created
                    .entry(text(&row[0]))
                    .and_modify(|old| *old = old.min(time))
                    .or_insert(time);
            }
        }
        for node in &mut nodes {
            node.created_at = created.get(&node.id).copied().map(time_string);
        }
    }
    let edges=query(store,"allowed[id] <- $ids\nincident[src,dst,edge_type,validity] := allowed[src],*edge{src,dst,edge_type,validity @ 'NOW'},allowed[dst]\n?[src,dst,edge_type,weight] := incident[src,dst,edge_type,validity],*edge{src,dst,edge_type,validity,weight,dst_store},dst_store='local'",input.clone())?.rows.into_iter().map(|r|GraphEdge{from:text(&r[0]),to:text(&r[1]),edge_type:text(&r[2]),weight:r[3].get_float().unwrap_or(1.0)}).collect::<Vec<_>>();
    let chain_ids = nodes
        .iter()
        .filter(|n| n.node_type == "chain")
        .map(|n| n.id.clone())
        .collect::<BTreeSet<_>>();
    let rows = query(
        store,
        "allowed[id] <- $ids\n?[chain_id,node_id,position] := allowed[chain_id],*chain_member{chain_id,node_id,position} :order chain_id,position",
        input,
    )?;
    let mut chains = BTreeMap::<String, Vec<Vec<DataValue>>>::new();
    for row in rows.rows {
        if chain_ids.contains(&text(&row[0])) {
            chains.entry(text(&row[0])).or_default().push(row);
        }
    }
    // Incomplete trails remain ordinary readable nodes, not invented paths
    // across omitted private or out-of-selection members.
    chains.retain(|_, rows| rows.iter().all(|r| allowed.contains_key(&text(&r[1]))));
    let chain_count = chains.len();
    let members = chains
        .into_values()
        .flatten()
        .map(|r| GraphMember {
            chain_id: text(&r[0]),
            node_id: text(&r[1]),
            position: r[2].get_int().unwrap_or(0).max(0) as u64,
        })
        .collect();
    checkpoint()?;
    let counts = GraphCounts {
        nodes: nodes.len(),
        edges: edges.len(),
        chains: chain_count,
    };
    let mut response = Response::read(
        "context",
        ResponseData::Graph(GraphData {
            graph: GraphSnapshot {
                snapshot_id: new_node_id(),
                nodes,
                edges,
                members,
                counts,
            },
        }),
        scopes,
    );
    response.coverage.as_mut().unwrap().rules_complete = false;
    Ok(response)
}

/// Cached structure must not outlive a scheduled visibility/retraction change.
pub(super) fn revalidate(
    store: &Store,
    graph: &GraphSnapshot,
    options: &ApiOptions,
) -> ApiResult<bool> {
    let input = BTreeMap::from([("ids".into(), ids(graph.nodes.iter().map(|n| n.id.clone())))]);
    let rows = query(
        store,
        "allowed[id] <- $ids\n?[id,visibility] := allowed[id],*node{id,visibility @ 'NOW'}",
        input,
    )?;
    Ok(rows.rows.len() == graph.nodes.len()
        && rows
            .rows
            .iter()
            .all(|r| options.include_local || r[1].get_str() == Some("shared")))
}
