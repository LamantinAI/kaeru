//! Edge endpoints — the cloud's graph structure surface.
//!
//! - `POST /api/v1/edges` — ingest an edge between two shared nodes. The
//!   local daemon calls this after both endpoints are shared, so the graph
//!   structure survives `share` / `pull`, not just the nodes.
//! - `DELETE /api/v1/edges` — **retract** one edge, by `src` / `dst` /
//!   `edge_type`. Until this existed a cloud edge could only die by
//!   retracting one of its endpoint *nodes*, so a local `unlink` had nowhere
//!   to send itself and the next `pull` put the edge straight back — a
//!   retraction any pull can undo is not a retraction (#85).
//!
//! Per-initiative listing lives at
//! `GET /api/v1/initiatives/{name}/edges` (see `initiatives.rs`), the
//! counterpart a puller reads to rebuild edges locally.
//!
//! Gates with the `Authenticated` extractor and delegates straight to
//! `kaeru-core` — no business logic in between.

use std::str::FromStr;
use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::api::extractors::Authenticated;
use crate::api::state::AppState;
use crate::errors::ApiError;
use crate::models::ErrorBody;
pub use crate::models::{EdgeIngestReq, EdgeRetractReq, EdgeView};
use kaeru_core::{EdgeType, Store, unlink, upsert_edge};

pub const EDGES_TAG: &str = "edges";

pub fn edges_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(ingest_edge, retract_edge))
}

#[utoipa::path(
    post,
    path = "/",
    tag = EDGES_TAG,
    request_body = EdgeIngestReq,
    responses(
        (status = 201, description = "Stored. Re-posting the same triple is an upsert — the way to change a weight.", body = EdgeView),
        (status = 400, description = "Unknown edge_type, or an empty src / dst.", body = ErrorBody),
        (status = 401, description = "Missing or invalid bearer token.", body = ErrorBody),
    ),
    security(("bearer" = []))
)]
async fn ingest_edge(
    _: Authenticated,
    State(store): State<Arc<Store>>,
    Json(req): Json<EdgeIngestReq>,
) -> Result<(StatusCode, Json<EdgeView>), ApiError> {
    let edge_type =
        EdgeType::from_str(&req.edge_type).map_err(|e| ApiError::BadRequest(e.to_string()))?;
    if req.src.trim().is_empty() || req.dst.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "src and dst must not be empty".to_string(),
        ));
    }

    upsert_edge(&store, &req.src, &req.dst, edge_type, req.weight)?;

    Ok((
        StatusCode::CREATED,
        Json(EdgeView {
            src: req.src,
            dst: req.dst,
            edge_type: edge_type.as_str().to_string(),
            weight: req.weight.clamp(0.0, 1.0),
        }),
    ))
}

/// `DELETE /api/v1/edges` — retract one edge, bi-temporally.
///
/// The body carries the triple rather than the path, because a path segment
/// cannot hold two UUIDs and a type without inventing an encoding for them.
///
/// Retraction, not erasure, exactly like `DELETE /nodes/{id}`: the edge stops
/// resolving at NOW and leaves the initiative's edge listing, while a read at
/// a past moment still traverses it.
///
/// Idempotent — retracting an edge that is already gone answers 204, so a
/// caller retrying after a dropped connection is not told its own success was
/// a failure.
///
/// **Whole-second caveat**, as everywhere: an edge retracted inside the same
/// second it was ingested carries an assert and a retract that cannot be
/// ordered, and may still read until the next write. Retrying a second later
/// settles it.
#[utoipa::path(
    delete,
    path = "/",
    tag = EDGES_TAG,
    request_body = EdgeRetractReq,
    responses(
        (status = 204, description = "Retracted — or already gone; the call is idempotent."),
        (status = 400, description = "Unknown edge_type, or an empty src / dst.", body = ErrorBody),
        (status = 401, description = "Missing or invalid bearer token.", body = ErrorBody),
    ),
    security(("bearer" = []))
)]
async fn retract_edge(
    _: Authenticated,
    State(store): State<Arc<Store>>,
    Json(req): Json<EdgeRetractReq>,
) -> Result<StatusCode, ApiError> {
    let edge_type =
        EdgeType::from_str(&req.edge_type).map_err(|e| ApiError::BadRequest(e.to_string()))?;
    if req.src.trim().is_empty() || req.dst.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "src and dst must not be empty".to_string(),
        ));
    }

    unlink(&store, &req.src, &req.dst, edge_type)?;
    Ok(StatusCode::NO_CONTENT)
}
