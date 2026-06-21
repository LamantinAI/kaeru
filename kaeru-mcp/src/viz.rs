//! Read-only `/graph.json` endpoint for the `kaeru-viz` visualizer.
//!
//! Serves the whole substrate as one JSON document via
//! [`kaeru_core::export_graph_json`]. Every node is run through the public
//! secret guard (redaction), and the initiative allow/deny scope is driven
//! entirely by configuration — there are **no** vault-specific names in this
//! source. An operator exposing this for a public audience sets
//! `KAERU_MCP_VIZ_INITIATIVES` (allow) and `KAERU_MCP_VIZ_DENY` (deny) to a
//! curated list; per-request `?initiatives=a,b&deny=c` overrides them.

use std::sync::Arc;

use axum::Router;
use axum::extract::{Query, State};
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use axum::routing::get;
use serde::Deserialize;

use kaeru_core::{ExportOpts, Store, export_graph_json};

#[derive(Clone)]
struct VizState {
    store: Arc<Store>,
    /// Default allow-list (from config). Empty = every initiative is exported
    /// (still redacted); curate it for a public deployment.
    default_allow: Vec<String>,
    /// Default deny-list (from config), always applied.
    default_deny: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct GraphQuery {
    /// CSV of initiative names / globs to allow (overrides the configured set).
    initiatives: Option<String>,
    /// CSV of initiative names / globs to deny (added to the configured set).
    deny: Option<String>,
    /// Include full bodies instead of excerpts (still redacted). Default false.
    bodies: Option<bool>,
}

/// Builds the viz router. `allow` / `deny` come from configuration
/// (`KAERU_MCP_VIZ_INITIATIVES` / `KAERU_MCP_VIZ_DENY`); both may be empty.
pub fn router(store: Arc<Store>, allow: Vec<String>, deny: Vec<String>) -> Router {
    Router::new()
        .route("/graph.json", get(graph_json))
        .with_state(VizState {
            store,
            default_allow: allow,
            default_deny: deny,
        })
}

async fn graph_json(State(st): State<VizState>, Query(q): Query<GraphQuery>) -> impl IntoResponse {
    // Allow-list: request override, else configured default, else None (all).
    let allow = match q.initiatives {
        Some(csv) => Some(csv_to_vec(&csv)),
        None if st.default_allow.is_empty() => None,
        None => Some(st.default_allow.clone()),
    };
    let mut deny = st.default_deny.clone();
    if let Some(csv) = q.deny {
        deny.extend(csv_to_vec(&csv));
    }
    let opts = ExportOpts {
        allow_initiatives: allow,
        deny_initiatives: deny,
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
