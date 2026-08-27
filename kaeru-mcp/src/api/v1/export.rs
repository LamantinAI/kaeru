//! `GET /v1/export` — the whole graph as one JSON document.
//!
//! This is the `export` verb over plain HTTP, and it is the one read that
//! honestly wants everything: the galaxy draws every node, so a narrower
//! answer would not help it. Rooms that need less should ask for less through
//! their own verb rather than filtering this in the browser.

use axum::extract::{Query, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use kaeru_core::export_graph_json;
use serde::Deserialize;

use crate::api::ApiState;
use crate::api::egress::Narrowing;
use crate::api::principal::Principal;

#[derive(Debug, Deserialize)]
pub struct ExportQuery {
    /// CSV of names / globs to **narrow within** the configured allow-list.
    initiatives: Option<String>,
    /// CSV of names / globs to additionally deny.
    deny: Option<String>,
    /// Include full bodies instead of excerpts (still redacted). Default false.
    bodies: Option<bool>,
}

pub async fn export(
    _who: Principal,
    State(st): State<ApiState>,
    Query(q): Query<ExportQuery>,
) -> Response {
    let opts = st.cfg.export_opts(
        &Narrowing {
            initiatives: q.initiatives,
            deny: q.deny,
        },
        q.bodies.unwrap_or(false),
    );

    // The export is synchronous Cozo work — keep it off the async executor.
    let store = st.store.clone();
    let result = tokio::task::spawn_blocking(move || export_graph_json(&store, &opts)).await;

    let mut resp = match result {
        Ok(Ok(graph)) => match serde_json::to_string(&graph) {
            Ok(json) => (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "application/json")],
                json,
            )
                .into_response(),
            Err(e) => internal(format!("serialize graph: {e}")),
        },
        Ok(Err(e)) => internal(format!("export graph: {e}")),
        Err(e) => internal(format!("export task: {e}")),
    };
    st.cfg.finish(&mut resp);
    resp
}

fn internal(msg: String) -> Response {
    tracing::warn!(error = %msg, "graph export failed");
    (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
}
