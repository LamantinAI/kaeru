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
    NodeBrief, NodeId, Store, edges_of, get_visibility, initiatives_of_node, node_brief_by_id,
    read_chain, read_node_full,
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

/// Whether a node may appear in a response at all: inside the operator's
/// ceiling, and shared.
///
/// One predicate, called before a node is read, described or followed. The
/// two halves used to be applied in different places — the ceiling here, the
/// visibility nowhere — which is how a trail came to serve in full the very
/// bodies `at` refused on the same daemon.
fn may_leave(store: &Store, id: &NodeId, cfg: &ApiConfig) -> bool {
    initiatives_of_node(store, id).is_ok_and(|i| cfg.reaches_node(&i))
        && get_visibility(store, id).is_ok_and(|v| cfg.may_show(v.as_str()))
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
    // the trail node itself is a node, and answers to the same rule
    let head_full = read_node_full(store, &id).ok()??;
    if !cfg.may_show(&head_full.visibility) {
        return None;
    }
    let head = node_brief_by_id(store, &id).ok()??;
    let members = read_chain(store, &id).ok()?;

    // A step the caller may not have is dropped rather than redacted.
    // Redaction says "there is something here you may not read"; for a trail
    // the honest answer is that the operator did not share this line of work
    // at all.
    //
    // Both halves of the rule are applied here, once, and before anything
    // walks a step's edges — a step filtered later would still have had its
    // edges followed, putting ids and structure into the response for nodes
    // that never appear in it.
    let members: Vec<NodeBrief> = members
        .into_iter()
        .filter(|m| may_leave(store, &m.id, cfg))
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
                // an edge into work the operator did not share is not shown —
                // the margin cites names, and a name is disclosure enough
                if !may_leave(store, other, cfg) {
                    continue;
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
        // the full body, not the excerpt: the steps below carry untruncated text,
        // and a summary that lost its tail while they kept theirs would be a
        // silent inconsistency rather than a visible one
        summary: head_full.body,
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

#[cfg(test)]
mod tests {
    use kaeru_core::{
        EdgeType, EpisodeKind, Significance, Store, Visibility, create_chain, link, set_visibility,
        write_episode,
    };

    use super::read;
    use crate::api::ApiConfig;

    fn cfg() -> ApiConfig {
        ApiConfig {
            allow: vec!["t".into()],
            ..ApiConfig::default()
        }
    }

    /// Two episodes joined into a trail. Everything is `local` — the default —
    /// unless a test says otherwise.
    fn store_with_a_trail() -> (Store, String, String, String) {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        let a = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Medium,
            "first step",
            "MY PRIVATE PASSWORD NOTES",
        )
        .expect("a");
        let b = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Medium,
            "second step",
            "and what followed from it",
        )
        .expect("b");
        link(&store, &a, &b, EdgeType::DerivedFrom).expect("edge");
        let chain = create_chain(&store, &a, &b, Some("the trail"), Some("why it matters"))
            .expect("chain")
            .expect("connected");
        (store, chain.id, a, b)
    }

    /// The defect: the trail checked the operator's initiative ceiling and
    /// never `visibility`, so it served in full the very bodies `at` answered
    /// 404 for — on one daemon, under one config.
    #[test]
    fn a_local_trail_is_not_served_at_all() {
        let (store, chain, _, _) = store_with_a_trail();
        assert!(read(&store, &chain, &cfg()).is_none());
    }

    #[test]
    fn a_shared_trail_still_withholds_its_local_steps() {
        let (store, chain, a, b) = store_with_a_trail();
        set_visibility(&store, &chain, Visibility::Shared).expect("share chain");
        set_visibility(&store, &b, Visibility::Shared).expect("share second");
        // `a` stays local on purpose

        let out = read(&store, &chain, &cfg()).expect("served");
        let bodies: Vec<String> = out.steps.iter().filter_map(|s| s.body.clone()).collect();
        assert!(
            !bodies.iter().any(|t| t.contains("PRIVATE PASSWORD")),
            "the local step's body is nowhere in the answer: {bodies:?}"
        );
        assert!(
            !out.steps.iter().any(|s| s.id == a),
            "and neither is its id"
        );
        assert!(
            !out.edges.iter().any(|e| e.src == a || e.dst == a),
            "nor is it reachable through an edge"
        );
        assert_eq!(out.steps.len(), 1, "only the shared step remains");
    }

    /// The summary used to come from the brief, which truncates, while the
    /// steps beside it carried full bodies.
    #[test]
    fn the_summary_is_not_truncated_while_the_steps_are_not() {
        let (store, chain, _, b) = store_with_a_trail();
        set_visibility(&store, &chain, Visibility::Shared).expect("share chain");
        set_visibility(&store, &b, Visibility::Shared).expect("share second");
        let out = read(&store, &chain, &cfg()).expect("served");
        assert_eq!(out.summary.as_deref(), Some("why it matters"));
    }
}
