//! Edge endpoints — the cloud's graph structure surface.
//!
//! - `POST /api/v1/edges` — ingest an edge between two shared nodes. The
//!   local daemon calls this after both endpoints are shared, so the graph
//!   structure survives `share` / `pull`, not just the nodes.
//!
//! Per-initiative listing lives at
//! `GET /api/v1/initiatives/{name}/edges` (see `initiatives.rs`), the
//! counterpart a puller reads to rebuild edges locally.
//!
//! Gates with the `Authenticated` extractor and delegates straight to
//! `kaeru-core` — no business logic in between.

use std::str::FromStr;
use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use serde::{Deserialize, Serialize};

use kaeru_core::{EdgeType, Store, upsert_edge};

use crate::api::extractors::Authenticated;
use crate::api::state::AppState;
use crate::errors::ApiError;

pub fn edges_router() -> Router<AppState> {
    Router::new().route("/", post(ingest_edge))
}

/// An edge pushed up from a local vault. `src` / `dst` are the (preserved)
/// node UUIDv7s — both must already be shared so the edge resolves.
#[derive(Debug, Deserialize)]
pub struct EdgeIngestReq {
    pub src: String,
    pub dst: String,
    pub edge_type: String,
}

#[derive(Debug, Serialize)]
pub struct EdgeView {
    pub src: String,
    pub dst: String,
    pub edge_type: String,
}

async fn ingest_edge(
    _: Authenticated,
    State(store): State<Arc<Store>>,
    Json(req): Json<EdgeIngestReq>,
) -> Result<(StatusCode, Json<EdgeView>), ApiError> {
    let edge_type =
        EdgeType::from_str(&req.edge_type).map_err(|e| ApiError::BadRequest(e.to_string()))?;
    if req.src.trim().is_empty() || req.dst.trim().is_empty() {
        return Err(ApiError::BadRequest("src and dst must not be empty".to_string()));
    }

    upsert_edge(&store, &req.src, &req.dst, edge_type)?;

    Ok((
        StatusCode::CREATED,
        Json(EdgeView {
            src: req.src,
            dst: req.dst,
            edge_type: edge_type.as_str().to_string(),
        }),
    ))
}
