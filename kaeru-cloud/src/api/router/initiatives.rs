//! Initiative endpoints — discovery for cross-session / cross-user recall.
//!
//! `GET /api/v1/initiatives` lists every initiative the cloud holds, with a
//! live node count — the entry point when a client knows the cloud exists
//! but not what's in it. `GET /api/v1/initiatives/{name}/nodes` lists the
//! shared nodes the cloud holds for one initiative, as compact briefs. The
//! local daemon calls these so an agent can see what team knowledge exists,
//! then `pull` individual nodes by id.

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use kaeru_core::{
    Error, Store, count_nodes_in_initiative, delete_initiative, edges_in_initiative,
    list_initiatives, nodes_in_initiative, rename_initiative,
};
use serde::{Deserialize, Serialize};

use crate::api::extractors::Authenticated;
use crate::api::router::edges::EdgeView;
use crate::api::state::AppState;
use crate::errors::ApiError;

pub fn initiatives_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_all))
        .route("/{name}/nodes", get(list_nodes))
        .route("/{name}/edges", get(list_edges))
        .route("/{name}/rename", post(rename))
        .route("/{name}", delete(remove))
}

/// One row of the initiative listing: the name plus a live (at NOW,
/// audit-events excluded) node count.
#[derive(Debug, Serialize)]
pub struct InitiativeBrief {
    pub name: String,
    pub nodes: usize,
}

/// Lists every initiative the cloud holds, with live node counts. Names
/// come from the append-only `node_initiative` junction, so an initiative
/// whose nodes were all since forgotten still appears — with `nodes: 0` —
/// which is itself useful discovery signal.
async fn list_all(
    _: Authenticated,
    State(store): State<Arc<Store>>,
) -> Result<Json<Vec<InitiativeBrief>>, ApiError> {
    let names = list_initiatives(&store)?;
    let mut views = Vec::with_capacity(names.len());
    for name in names {
        let nodes = count_nodes_in_initiative(&store, &name)?;
        views.push(InitiativeBrief { name, nodes });
    }
    Ok(Json(views))
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
        .map(|(src, dst, edge_type, weight)| EdgeView {
            src,
            dst,
            edge_type,
            weight,
        })
        .collect();
    Ok(Json(views))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use kaeru_core::Store;
    use tower::util::ServiceExt;

    use crate::api::router::api_router;
    use crate::api::state::AppState;

    /// Full app router over an in-memory store holding one node in each of
    /// two initiatives, with auth disabled (empty expected token).
    fn app() -> axum::Router {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("alpha");
        kaeru_core::jot(&store, "seed for alpha").expect("jot alpha");
        store.use_initiative("beta");
        kaeru_core::jot(&store, "seed for beta").expect("jot beta");
        api_router(AppState {
            api_token: Arc::from(""),
            store: Arc::new(store),
        })
    }

    #[tokio::test]
    async fn lists_initiatives_with_live_counts() {
        let resp = app()
            .oneshot(
                Request::builder()
                    .uri("/api/v1/initiatives")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let bytes = resp.into_body().collect().await.unwrap().to_bytes();
        let views: Vec<serde_json::Value> = serde_json::from_slice(&bytes).unwrap();
        let pairs: Vec<(String, u64)> = views
            .iter()
            .map(|v| {
                (
                    v["name"].as_str().unwrap().to_string(),
                    v["nodes"].as_u64().unwrap(),
                )
            })
            .collect();
        // Audit-event nodes are excluded from counts; names sort alphabetically.
        assert_eq!(
            pairs,
            vec![("alpha".to_string(), 1), ("beta".to_string(), 1)]
        );
    }

    #[tokio::test]
    async fn listing_requires_auth_when_token_configured() {
        let store = Store::open_in_memory().expect("open");
        let app = api_router(AppState {
            api_token: Arc::from("sekret"),
            store: Arc::new(store),
        });
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/v1/initiatives")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
