//! Initiative endpoints — discovery for cross-session / cross-user recall.
//!
//! `GET /api/v1/initiatives/{name}/nodes` lists the shared nodes the cloud
//! holds for an initiative, as compact briefs. The local daemon calls this
//! so an agent can see what team knowledge exists, then `pull` individual
//! nodes by id.

use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::routing::{delete, get, post};
use serde::{Deserialize, Serialize};

use kaeru_core::{
    Error, Store, delete_initiative, edges_in_initiative, nodes_in_initiative, rename_initiative,
};

use crate::api::extractors::Authenticated;
use crate::api::router::edges::EdgeView;
use crate::api::state::AppState;
use crate::errors::ApiError;

pub fn initiatives_router() -> Router<AppState> {
    Router::new()
        .route("/{name}/nodes", get(list_nodes))
        .route("/{name}/edges", get(list_edges))
        .route("/{name}/rename", post(rename))
        .route("/{name}", delete(remove))
}

#[derive(Debug, Deserialize)]
pub struct RenameReq {
    pub new: String,
}

#[derive(Debug, Serialize)]
pub struct RenameResp {
    pub nodes: usize,
    pub edges: usize,
}

#[derive(Debug, Serialize)]
pub struct DeleteResp {
    pub unscoped: usize,
    pub forgotten: usize,
}

/// Renames an initiative across the whole shared store — team-wide.
async fn rename(
    _: Authenticated,
    State(store): State<Arc<Store>>,
    Path(name): Path<String>,
    Json(req): Json<RenameReq>,
) -> Result<Json<RenameResp>, ApiError> {
    let stats = rename_initiative(&store, &name, &req.new).map_err(|e| match e {
        Error::Invalid(m) => ApiError::BadRequest(m),
        other => ApiError::Core(other),
    })?;
    Ok(Json(RenameResp {
        nodes: stats.nodes,
        edges: stats.edges,
    }))
}

/// Deletes an initiative from the whole shared store — team-wide.
async fn remove(
    _: Authenticated,
    State(store): State<Arc<Store>>,
    Path(name): Path<String>,
) -> Result<Json<DeleteResp>, ApiError> {
    let stats = delete_initiative(&store, &name)?;
    Ok(Json(DeleteResp {
        unscoped: stats.unscoped,
        forgotten: stats.forgotten,
    }))
}

/// Compact node view for discovery listings.
#[derive(Debug, Serialize)]
pub struct NodeBriefView {
    pub id: String,
    pub node_type: String,
    pub name: String,
    pub body_excerpt: Option<String>,
}

async fn list_nodes(
    _: Authenticated,
    State(store): State<Arc<Store>>,
    Path(name): Path<String>,
) -> Result<Json<Vec<NodeBriefView>>, ApiError> {
    let briefs = nodes_in_initiative(&store, &name)?;
    let views = briefs
        .into_iter()
        .map(|b| NodeBriefView {
            id: b.id,
            node_type: b.node_type,
            name: b.name,
            body_excerpt: b.body_excerpt,
        })
        .collect();
    Ok(Json(views))
}

async fn list_edges(
    _: Authenticated,
    State(store): State<Arc<Store>>,
    Path(name): Path<String>,
) -> Result<Json<Vec<EdgeView>>, ApiError> {
    let edges = edges_in_initiative(&store, &name)?;
    let views = edges
        .into_iter()
        .map(|(src, dst, edge_type)| EdgeView { src, dst, edge_type })
        .collect();
    Ok(Json(views))
}
