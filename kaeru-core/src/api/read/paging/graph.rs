//! Page borrowed compact items and retain one shared immutable snapshot.
use std::sync::Arc;
use std::time::Instant;

use serde_json::{json, to_string, to_vec};

use super::{Snapshot, arguments_hash, policy_hash};
use crate::api::control::checkpoint;
use crate::api::read::graph::revalidate;
use crate::api::{ApiOptions, ApiResult, Args, issue_from_error};
use crate::config::api::{
    API_DEFAULT_CONTEXT_BUDGET_CHARS, API_MAX_CACHED_SNAPSHOTS, API_MAX_SNAPSHOT_BYTES,
    API_SNAPSHOT_TTL,
};
use crate::models::{
    Continuation, GraphData, GraphSnapshot, Issue, Omission, Response, ResponseData,
};
use crate::{Error, Store};

pub(super) fn page(
    store: &Store,
    full: Arc<Response>,
    args: &Args,
    options: &ApiOptions,
    offset: usize,
    token: Option<String>,
) -> ApiResult<Response> {
    let ResponseData::Graph(source) = &full.data else {
        unreachable!()
    };
    let graph = &source.graph;
    let valid = store
        .db_ref()
        .read_snapshot(|| {
            revalidate(store, graph, options).map_err(|e| Error::Invalid(to_string(&e).unwrap()))
        })
        .map_err(issue_from_error)?;
    if !valid {
        return Err(Issue::new(
            "CURSOR_STALE",
            "graph visibility changed; restart the graph read",
        ));
    }
    checkpoint()?;
    if offset == 0 && to_vec(full.as_ref()).unwrap().len() > API_MAX_SNAPSHOT_BYTES {
        return Err(Issue::new(
            "SNAPSHOT_TOO_LARGE",
            "select fewer initiatives for this graph",
        ));
    }
    let total = graph.nodes.len() + graph.edges.len() + graph.members.len();
    if offset > total {
        return Err(Issue::new("CURSOR_STALE", "invalid graph offset"));
    }
    let budget = args
        .value("budget_chars")
        .as_u64()
        .unwrap_or(API_DEFAULT_CONTEXT_BUDGET_CHARS as u64) as usize;
    if offset == 0 && to_string(full.as_ref()).unwrap().chars().count() <= budget {
        return Ok(Arc::unwrap_or_clone(full));
    }
    let token = token.unwrap_or_else(|| graph.snapshot_id.clone());
    let mut next_arguments = args.0.clone();
    next_arguments["cursor"] = json!(format!("{token}:{total}"));
    let mut page = Response::read(
        "context",
        ResponseData::Graph(GraphData {
            graph: GraphSnapshot {
                snapshot_id: graph.snapshot_id.clone(),
                nodes: vec![],
                edges: vec![],
                members: vec![],
                counts: graph.counts.clone(),
            },
        }),
        full.coverage.as_ref().unwrap().initiatives.clone(),
    );
    let coverage = page.coverage.as_mut().unwrap();
    coverage.complete = false;
    coverage.rules_complete = false;
    coverage.omitted.push(Omission {
        section: "continuation".into(),
        count: total.saturating_sub(offset),
        reason: "budget_or_limit".into(),
    });
    page.continuation = Some(Continuation {
        cursor: format!("{token}:{total}"),
        next_arguments,
    });
    let mut complete_page = page.clone();
    complete_page.continuation = None;
    complete_page.coverage.as_mut().unwrap().complete = true;
    complete_page.coverage.as_mut().unwrap().omitted.clear();
    let complete_savings = to_string(&page).unwrap().chars().count()
        - to_string(&complete_page).unwrap().chars().count();
    let mut used = to_string(&page).unwrap().chars().count();
    let mut end = offset;
    // Each item is serialized once for sizing, without cloning the full graph
    // or rebuilding a vector of every item on each continuation page.
    while end < total {
        checkpoint()?;
        let cost = if end < graph.nodes.len() {
            to_string(&graph.nodes[end]).unwrap().chars().count()
        } else if end < graph.nodes.len() + graph.edges.len() {
            to_string(&graph.edges[end - graph.nodes.len()])
                .unwrap()
                .chars()
                .count()
        } else {
            to_string(&graph.members[end - graph.nodes.len() - graph.edges.len()])
                .unwrap()
                .chars()
                .count()
        };
        let savings = if end + 1 == total {
            complete_savings
        } else {
            0
        };
        if used + cost + 1 - savings > budget {
            break;
        }
        let ResponseData::Graph(data) = &mut page.data else {
            unreachable!()
        };
        if end < graph.nodes.len() {
            data.graph.nodes.push(graph.nodes[end].clone());
        } else if end < graph.nodes.len() + graph.edges.len() {
            data.graph
                .edges
                .push(graph.edges[end - graph.nodes.len()].clone());
        } else {
            data.graph
                .members
                .push(graph.members[end - graph.nodes.len() - graph.edges.len()].clone());
        }
        used += cost + 1;
        end += 1;
    }
    if end == offset && end < total {
        return Err(Issue::new(
            "BUDGET_TOO_SMALL",
            "increase graph budget_chars",
        ));
    }
    if end == total {
        page.coverage.as_mut().unwrap().complete = true;
        page.coverage.as_mut().unwrap().omitted.clear();
        page.continuation = None;
    } else {
        page.coverage.as_mut().unwrap().omitted[0].count = total - end;
        let continuation = page.continuation.as_mut().unwrap();
        continuation.cursor = format!("{token}:{end}");
        continuation.next_arguments["cursor"] = json!(continuation.cursor);
        let mut snapshots = store
            .api_snapshots
            .lock()
            .unwrap_or_else(|p| p.into_inner());
        snapshots.retain(|_, s| s.expires > Instant::now());
        if snapshots.len() >= API_MAX_CACHED_SNAPSHOTS {
            snapshots.clear();
        }
        snapshots.insert(
            token,
            Snapshot {
                caller: options.caller.clone(),
                arguments_hash: arguments_hash("context", args),
                policy_hash: policy_hash(options),
                generation: store.generation(),
                expires: Instant::now() + API_SNAPSHOT_TTL,
                response: full,
            },
        );
    }
    if to_string(&page).unwrap().chars().count() > budget {
        return Err(Issue::new(
            "BUDGET_TOO_SMALL",
            "increase graph budget_chars",
        ));
    }
    checkpoint()?;
    Ok(page)
}
