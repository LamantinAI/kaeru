//! `GET /v1/at` — one node, in full.
//!
//! The `at` verb: the whole body rather than the excerpt the export carries,
//! and optionally the node as it stood at a past moment.
//!
//! This is what lets a room stop paying the galaxy's price for one node's
//! text. The export truncates every body to keep the whole-graph document
//! survivable, so a room that wanted the real text had only one option — ask
//! for every body in the vault. On this vault that is 3.7 MB to fill a panel
//! showing one card.

use axum::extract::{Query, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use kaeru_core::{
    Store, at as node_at, get_visibility, initiatives_of_node, node_brief_by_id, read_node_full,
};
use serde::{Deserialize, Serialize};

use crate::api::egress::{redact_excerpt, redact_name};
use crate::api::principal::Principal;
use crate::api::{ApiConfig, ApiState};

#[derive(Debug, Deserialize)]
pub struct AtQuery {
    /// The node's id.
    id: String,
    /// Unix seconds to read the node as of. Omit for now.
    when: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct NodeOut {
    id: String,
    #[serde(rename = "type")]
    node_type: String,
    tier: String,
    layer: String,
    name: String,
    body: Option<String>,
    tags: Vec<String>,
    initiatives: Vec<String>,
    /// Latest assertion time, when the read carried one.
    ts: Option<f64>,
    redacted: bool,
}

/// Reads the node and decides whether it may leave.
///
/// Both happen here rather than in the handler so the order cannot be got
/// wrong: permission is judged against what was actually read, never against
/// what was asked for.
///
/// Visibility is judged on the record as it stands **now**, even for a
/// historical read. A node made `local` today should not leak through a
/// question about last week — visibility is policy, and policy is not
/// something the past gets a vote on.
///
/// It is read on its own rather than taken off the full record, which would
/// have a second effect the rule does not intend: the full record only exists
/// at NOW, so gating on it would gate *existence* on the present and answer
/// 404 for a node since retracted — for the one verb whose whole purpose is
/// to look at moments that have passed.
fn read(store: &Store, id: &str, when: Option<f64>, cfg: &ApiConfig) -> Option<NodeOut> {
    let id = id.to_string();
    let inits = initiatives_of_node(store, &id).ok()?;
    if !cfg.reaches_node(&inits) {
        return None;
    }
    if !cfg.may_show(get_visibility(store, &id).ok()?.as_str()) {
        return None;
    }

    // With a moment, answer from history; without one, the full record — the
    // only read at NOW that carries the body untruncated. The brief supplies
    // the assertion time the full record does not, and every step the reader
    // draws is dated.
    let (node_type, tier, layer, name, body, tags, ts) = match when {
        Some(secs) => {
            let snap = node_at(store, &id, secs).ok()??;
            let ts = snap.ts;
            (
                snap.node_type,
                snap.tier,
                snap.layer,
                snap.name,
                snap.body,
                snap.tags,
                ts,
            )
        }
        None => {
            let full = read_node_full(store, &id).ok()??;
            let ts = node_brief_by_id(store, &id)
                .ok()
                .flatten()
                .and_then(|b| b.ts);
            (
                full.node_type,
                full.tier,
                full.layer,
                full.name,
                full.body,
                full.tags,
                ts,
            )
        }
    };

    let (name, name_hit) = redact_name(&name, &node_type);
    let (body, body_hit) = redact_excerpt(body.as_deref());
    Some(NodeOut {
        id,
        node_type,
        tier,
        layer,
        name,
        body,
        tags,
        // an initiative the caller cannot reach is not named back at them
        initiatives: cfg.visible_initiatives(inits),
        ts,
        redacted: name_hit || body_hit,
    })
}

pub async fn at(_who: Principal, State(st): State<ApiState>, Query(q): Query<AtQuery>) -> Response {
    let store = st.store.clone();
    let cfg = st.cfg.clone();
    let id = q.id.clone();
    let when = q.when;
    let found = tokio::task::spawn_blocking(move || read(&store, &id, when, &cfg)).await;

    let mut resp = match found {
        Ok(Some(node)) => match serde_json::to_string(&node) {
            Ok(json) => (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "application/json")],
                json,
            )
                .into_response(),
            Err(e) => internal(format!("serialize node: {e}")),
        },
        // Outside the ceiling and simply absent are the same answer: saying
        // "exists, but not for you" is saying it exists.
        Ok(None) => (StatusCode::NOT_FOUND, "no such node").into_response(),
        Err(e) => internal(format!("node task: {e}")),
    };
    st.cfg.finish(&mut resp);
    resp
}

fn internal(msg: String) -> Response {
    tracing::warn!(error = %msg, "node read failed");
    (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
}

#[cfg(test)]
mod tests {
    use kaeru_core::{Store, Visibility, attach_node, set_visibility, write_task};

    use super::read;
    use crate::api::ApiConfig;

    fn cfg() -> ApiConfig {
        ApiConfig {
            allow: vec!["t".into()],
            ..ApiConfig::default()
        }
    }

    #[test]
    fn a_local_node_is_not_served() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        let id = write_task(&store, "a private chore", None).expect("task");
        assert!(read(&store, &id, None, &cfg()).is_none());

        set_visibility(&store, &id, Visibility::Shared).expect("share");
        assert!(
            read(&store, &id, None, &cfg()).is_some(),
            "shared is served"
        );
    }

    /// An initiative name is often the most sensitive string in a record — a
    /// client codename, an unannounced project. Naming one back at a caller
    /// who cannot reach it is the disclosure this surface avoids by answering
    /// 404 rather than 403.
    #[test]
    fn initiatives_outside_the_ceiling_are_not_named_back() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        let id = write_task(&store, "a shared chore", None).expect("task");
        set_visibility(&store, &id, Visibility::Shared).expect("share");
        attach_node(&store, &id, "codename-thunderbolt").expect("attach");

        let out = read(&store, &id, None, &cfg()).expect("served");
        assert_eq!(out.initiatives, vec!["t".to_string()]);
    }
}
