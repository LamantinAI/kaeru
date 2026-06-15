//! Node endpoints — the cloud's core surface for the local/cloud split.
//!
//! - `POST /api/v1/nodes` — ingest a shared node. The local daemon calls
//!   this after a node passes the share gates; the **id is preserved** so a
//!   local soft link (`dst = <id>`) resolves back here.
//! - `GET /api/v1/nodes/{id}` — fetch a node by id. Resolves a soft link
//!   lazily; id is globally unique so no initiative scope is needed.
//!
//! Both gate themselves with the `Authenticated` extractor and delegate
//! straight to `kaeru-core` — there is no business logic in between.

use std::str::FromStr;
use std::sync::Arc;

use axum::Json;
use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::routing::post;
use serde::Deserialize;
use serde::Serialize;

use kaeru_core::NodeFull;
use kaeru_core::NodeType;
use kaeru_core::Store;
use kaeru_core::Tier;
use kaeru_core::Visibility;
use kaeru_core::read_node_full;
use kaeru_core::upsert_node;

use crate::api::extractors::Authenticated;
use crate::api::state::AppState;
use crate::errors::ApiError;

pub fn nodes_router() -> Router<AppState> {
    Router::new()
        .route("/", post(ingest_node))
        .route("/{id}", get(get_node))
}

/// A node being pushed up from a local vault. `id` is the local node's
/// UUIDv7, preserved verbatim so soft links resolve.
#[derive(Debug, Deserialize)]
pub struct NodeIngestReq {
    pub id: String,
    pub node_type: String,
    pub tier: String,
    pub name: String,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    /// Initiative this node belongs to — the shared scope on both sides.
    #[serde(default)]
    pub initiative: Option<String>,
}

/// Full node view returned to the caller — the **untruncated** body and
/// tier/tags, so a puller can materialise the node locally verbatim.
#[derive(Debug, Serialize)]
pub struct NodeView {
    pub id: String,
    pub node_type: String,
    pub tier: String,
    pub name: String,
    pub body: Option<String>,
    pub tags: Vec<String>,
    pub visibility: String,
}

async fn ingest_node(
    _: Authenticated,
    State(store): State<Arc<Store>>,
    Json(req): Json<NodeIngestReq>,
) -> Result<(StatusCode, Json<NodeView>), ApiError> {
    let node_type =
        NodeType::from_str(&req.node_type).map_err(|e| ApiError::BadRequest(e.to_string()))?;
    let tier = Tier::from_str(&req.tier).map_err(|e| ApiError::BadRequest(e.to_string()))?;

    if req.name.trim().is_empty() {
        return Err(ApiError::BadRequest("name must not be empty".to_string()));
    }

    // A node living in the cloud is shared by definition.
    upsert_node(
        &store,
        &req.id,
        node_type,
        tier,
        &req.name,
        req.body.as_deref(),
        &req.tags,
        req.initiative.as_deref(),
        Visibility::Shared,
    )?;

    let full = read_node_full(&store, &req.id)?.ok_or(ApiError::NotFound)?;
    Ok((StatusCode::CREATED, Json(full_to_view(full))))
}

async fn get_node(
    _: Authenticated,
    State(store): State<Arc<Store>>,
    Path(id): Path<String>,
) -> Result<Json<NodeView>, ApiError> {
    let full = read_node_full(&store, &id)?.ok_or(ApiError::NotFound)?;
    Ok(Json(full_to_view(full)))
}

fn full_to_view(full: NodeFull) -> NodeView {
    NodeView {
        id: full.id,
        node_type: full.node_type,
        tier: full.tier,
        name: full.name,
        body: full.body,
        tags: full.tags,
        visibility: full.visibility,
    }
}
