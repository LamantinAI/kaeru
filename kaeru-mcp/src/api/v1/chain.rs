//! `GET /v1/chain` — a saved reasoning trail, with its margin.
//!
//! The `read_chain` verb, plus the context that makes a trail readable rather
//! than a list of names. Three parts:
//!
//! - **steps** — the members in order, bodies untruncated. The excerpt the
//!   export carries is enough to recognise a node in the galaxy and not enough
//!   to read it.
//! - **edges** — every local edge touching a step. The reader lays a trail out
//!   as a DAG, so it needs the real edges: a fork has to look like a fork, and
//!   inventing a spine where the graph has none would draw a continuity that
//!   is not there.
//! - **nearby** — the other end of any edge that leaves the trail. This is the
//!   margin: what a step supersedes, refers to, is grounded in. Names only —
//!   the margin cites, it does not quote.
//!
//! Together that is a few kilobytes. Deriving the same thing in the browser
//! meant the whole graph and every body in the vault: on this vault, 5 MB to
//! read three steps.

use std::collections::{BTreeMap, BTreeSet};

use axum::extract::{Query, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use kaeru_core::{
    NodeBrief, Store, edges_of, initiatives_of_node, node_brief_by_id, read_chain, read_node_full,
};
use serde::{Deserialize, Serialize};

use crate::api::egress::{redact_excerpt, redact_name};
use crate::api::principal::Principal;
use crate::api::{ApiConfig, ApiState};

#[derive(Debug, Deserialize)]
pub struct ChainQuery {
    /// The chain node's id.
    id: String,
}

#[derive(Debug, Serialize)]
pub struct ChainOut {
    id: String,
    name: String,
    /// The chain node's own body — why the trail was worth saving.
    summary: Option<String>,
    steps: Vec<StepOut>,
    edges: Vec<EdgeOut>,
    nearby: Vec<NearOut>,
}

#[derive(Debug, Serialize)]
struct StepOut {
    id: String,
    #[serde(rename = "type")]
    node_type: String,
    tier: String,
    layer: String,
    name: String,
    body: Option<String>,
    tags: Vec<String>,
    ts: Option<f64>,
    redacted: bool,
}

#[derive(Debug, Serialize)]
struct EdgeOut {
    src: String,
    dst: String,
    #[serde(rename = "type")]
    edge_type: String,
}

#[derive(Debug, Serialize)]
struct NearOut {
    id: String,
    #[serde(rename = "type")]
    node_type: String,
    name: String,
}

fn step(store: &Store, brief: &NodeBrief) -> StepOut {
    // The brief carries the assertion time and a truncated body; the full
    // record carries the text. Neither alone is a step.
    let full = read_node_full(store, &brief.id).ok().flatten();
    let node_type = full
        .as_ref()
        .map(|f| f.node_type.clone())
        .unwrap_or_else(|| brief.node_type.clone());
    let (name, name_hit) = redact_name(&brief.name, &node_type);
    let (body, body_hit) = redact_excerpt(
        full.as_ref()
            .and_then(|f| f.body.as_deref())
            .or(brief.body_excerpt.as_deref()),
    );
    StepOut {
        id: brief.id.clone(),
        node_type,
        tier: full.as_ref().map(|f| f.tier.clone()).unwrap_or_default(),
        layer: full.as_ref().map(|f| f.layer.clone()).unwrap_or_default(),
        name,
        body,
        tags: full.map(|f| f.tags).unwrap_or_default(),
        ts: brief.ts,
        redacted: name_hit || body_hit,
    }
}

fn read(store: &Store, id: &str, cfg: &ApiConfig) -> Option<ChainOut> {
    let id = id.to_string();
    if !cfg.reaches_node(&initiatives_of_node(store, &id).ok()?) {
        return None;
    }
    let head = node_brief_by_id(store, &id).ok()??;
    let members = read_chain(store, &id).ok()?;

    // A step outside the ceiling is dropped rather than redacted. Redaction
    // says "there is something here you may not read"; for a trail the honest
    // answer is that the operator did not share this line of work at all.
    let members: Vec<NodeBrief> = members
        .into_iter()
        .filter(|m| {
            initiatives_of_node(store, &m.id)
                .map(|i| cfg.reaches_node(&i))
                .unwrap_or(false)
        })
        .collect();

    let on_trail: BTreeSet<String> = members.iter().map(|m| m.id.clone()).collect();
    let mut edges = Vec::new();
    let mut seen_edge = BTreeSet::new();
    let mut nearby: BTreeMap<String, NearOut> = BTreeMap::new();

    for m in &members {
        for (src, dst, edge_type, _weight) in edges_of(store, &m.id).ok()?.into_iter() {
            if !seen_edge.insert((src.clone(), dst.clone(), edge_type.clone())) {
                continue;
            }
            // the far end of an edge that leaves the trail — the margin
            let other = if on_trail.contains(&src) { &dst } else { &src };
            if !on_trail.contains(other) {
                if nearby.contains_key(other) {
                    edges.push(EdgeOut {
                        src,
                        dst,
                        edge_type,
                    });
                    continue;
                }
                let Ok(inits) = initiatives_of_node(store, other) else {
                    continue;
                };
                if !cfg.reaches_node(&inits) {
                    continue; // an edge into work the operator did not share is not shown
                }
                let Ok(Some(b)) = node_brief_by_id(store, other) else {
                    continue;
                };
                let (name, _) = redact_name(&b.name, &b.node_type);
                nearby.insert(
                    other.clone(),
                    NearOut {
                        id: other.clone(),
                        node_type: b.node_type,
                        name,
                    },
                );
            }
            edges.push(EdgeOut {
                src,
                dst,
                edge_type,
            });
        }
    }

    Some(ChainOut {
        id,
        name: head.name,
        summary: head.body_excerpt,
        steps: members.iter().map(|m| step(store, m)).collect(),
        edges,
        nearby: nearby.into_values().collect(),
    })
}

pub async fn chain(
    _who: Principal,
    State(st): State<ApiState>,
    Query(q): Query<ChainQuery>,
) -> Response {
    let store = st.store.clone();
    let cfg = st.cfg.clone();
    let id = q.id.clone();
    let found = tokio::task::spawn_blocking(move || read(&store, &id, &cfg)).await;

    let mut resp = match found {
        Ok(Some(chain)) => match serde_json::to_string(&chain) {
            Ok(json) => (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "application/json")],
                json,
            )
                .into_response(),
            Err(e) => internal(format!("serialize chain: {e}")),
        },
        Ok(None) => (StatusCode::NOT_FOUND, "no such chain").into_response(),
        Err(e) => internal(format!("chain task: {e}")),
    };
    st.cfg.finish(&mut resp);
    resp
}

fn internal(msg: String) -> Response {
    tracing::warn!(error = %msg, "chain read failed");
    (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
}
