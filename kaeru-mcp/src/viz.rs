//! Read-only `/graph.json` endpoint for the `kaeru-viz` visualizer.
//!
//! Serves the substrate as one JSON document via [`kaeru_core::export_graph_json`].
//! Safe by default and operator-driven — there are **no** vault-specific names
//! in this source:
//!
//! - **Opt-in:** the route is only mounted when `KAERU_MCP_VIZ_ENABLE` is set
//!   (wired in `main.rs`); a daemon never exposes a whole-graph export unasked.
//! - **Safe-empty allow:** `KAERU_MCP_VIZ_INITIATIVES` is the authoritative
//!   ceiling; empty (the default) exports nothing. A request's `?initiatives=`
//!   only *narrows within* it (intersection) — it can never widen the set.
//! - **Shared-only:** by default only `visibility = shared` nodes are exported;
//!   `local` nodes stay on the machine unless `KAERU_MCP_VIZ_INCLUDE_LOCAL=1`.
//! - **Redacted:** every node passes the public secret/credential guard.
//! - `KAERU_MCP_VIZ_DENY` is always-applied; `?deny=` adds to it.

use std::sync::Arc;

use axum::Router;
use axum::extract::{Query, State};
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::get;
use serde::Deserialize;

use kaeru_core::{ExportOpts, Store, export_graph_json};

/// Endpoint configuration, all operator-controlled (no names in source).
#[derive(Clone)]
pub struct VizConfig {
    /// Authoritative allow-list ceiling (`KAERU_MCP_VIZ_INITIATIVES`). **Empty =
    /// export nothing** — the operator must opt in. Requests can only narrow it.
    pub allow: Vec<String>,
    /// Always-applied deny-list (`KAERU_MCP_VIZ_DENY`).
    pub deny: Vec<String>,
    /// Export `local` nodes too (`KAERU_MCP_VIZ_INCLUDE_LOCAL`). Default false —
    /// only `shared` nodes leave the daemon.
    pub include_local: bool,
}

#[derive(Clone)]
struct VizState {
    store: Arc<Store>,
    cfg: VizConfig,
}

#[derive(Debug, Deserialize)]
struct GraphQuery {
    /// CSV of names / globs to **narrow within** the configured allow-list
    /// (intersection — it can never widen the exported set).
    initiatives: Option<String>,
    /// CSV of names / globs to additionally deny.
    deny: Option<String>,
    /// Include full bodies instead of excerpts (still redacted). Default false.
    bodies: Option<bool>,
}

/// Builds the viz router from operator config.
pub fn router(store: Arc<Store>, cfg: VizConfig) -> Router {
    Router::new()
        .route("/graph.json", get(graph_json))
        .with_state(VizState { store, cfg })
}

async fn graph_json(State(st): State<VizState>, Query(q): Query<GraphQuery>) -> impl IntoResponse {
    // The configured allow-list is the authoritative ceiling — ALWAYS `Some`,
    // so an empty config exports nothing. A request's `?initiatives=` becomes a
    // *narrowing* filter (intersection), never a replacement, so a caller can't
    // widen past what the operator opted into.
    let mut deny = st.cfg.deny.clone();
    if let Some(csv) = q.deny {
        deny.extend(csv_to_vec(&csv));
    }
    let opts = ExportOpts {
        allow_initiatives: Some(st.cfg.allow.clone()),
        restrict_initiatives: q.initiatives.as_deref().map(csv_to_vec),
        deny_initiatives: deny,
        shared_only: !st.cfg.include_local,
        include_bodies: q.bodies.unwrap_or(false),
        redact: true,
    };

    // The export is synchronous Cozo work — keep it off the async executor.
    let store = st.store.clone();
    let result = tokio::task::spawn_blocking(move || export_graph_json(&store, &opts)).await;

    match result {
        Ok(Ok(graph)) => match serde_json::to_string(&graph) {
            Ok(json) => (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, "application/json"),
                    (header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
                ],
                json,
            )
                .into_response(),
            Err(e) => internal(format!("serialize graph: {e}")),
        },
        Ok(Err(e)) => internal(format!("export graph: {e}")),
        Err(e) => internal(format!("export task: {e}")),
    }
}

fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

fn internal(msg: String) -> axum::response::Response {
    tracing::warn!(error = %msg, "graph.json export failed");
    (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
}
