//! Initiative endpoints — discovery for cross-session / cross-user recall.
//!
//! `GET /api/v1/initiatives` lists every initiative the cloud holds, with a
//! live node count — the entry point when a client knows the cloud exists
//! but not what's in it. `GET /api/v1/initiatives/{name}/nodes` lists the
//! shared nodes the cloud holds for one initiative, as compact briefs. The
//! local daemon calls these so an agent can see what team knowledge exists,
//! then `pull` individual nodes by id.

use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, Query, State};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::api::extractors::Authenticated;
use crate::api::router::edges::EdgeView;
use crate::api::state::AppState;
use crate::errors::ApiError;
use crate::models::ErrorBody;
pub use crate::models::{
    DeleteResp, InitiativeBrief, NodeBriefView, PageQuery, RenameReq, RenameResp,
};
use kaeru_core::config::cloud::{CLOUD_DEFAULT_PAGE_SIZE, CLOUD_MAX_PAGE_SIZE};
use kaeru_core::{
    Error, Store, count_nodes_in_initiative, delete_initiative, edges_in_initiative,
    list_initiatives, nodes_in_initiative, rename_initiative,
};

pub const INITIATIVES_TAG: &str = "initiatives";

pub fn initiatives_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list_all))
        .routes(routes!(list_nodes))
        .routes(routes!(list_edges))
        .routes(routes!(rename))
        .routes(routes!(remove))
}

/// Lists every initiative the cloud holds, with live node counts. Names
/// come from the append-only `node_initiative` junction, so an initiative
/// whose nodes were all since forgotten still appears — with `nodes: 0` —
/// which is itself useful discovery signal.
#[utoipa::path(
    get,
    path = "/",
    tag = INITIATIVES_TAG,
    responses(
        (status = 200, description = "Every initiative the cloud holds, with live node counts.", body = Vec<InitiativeBrief>),
        (status = 401, description = "Missing or invalid bearer token.", body = ErrorBody),
    ),
    security(("bearer" = []))
)]
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

/// Renames an initiative across the whole shared store — team-wide.
#[utoipa::path(
    post,
    path = "/{name}/rename",
    tag = INITIATIVES_TAG,
    params(("name" = String, Path, description = "The initiative name, exactly as stored.")),
    request_body = RenameReq,
    responses(
        (status = 200, description = "Renamed team-wide.", body = RenameResp),
        (status = 400, description = "The new name is empty, identical, or already exists.", body = ErrorBody),
        (status = 401, description = "Missing or invalid bearer token.", body = ErrorBody),
    ),
    security(("bearer" = []))
)]
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
#[utoipa::path(
    delete,
    path = "/{name}",
    tag = INITIATIVES_TAG,
    params(("name" = String, Path, description = "The initiative name, exactly as stored.")),
    responses(
        (status = 200, description = "Deleted team-wide: nodes exclusive to it are forgotten, shared ones lose only this membership.", body = DeleteResp),
        (status = 401, description = "Missing or invalid bearer token.", body = ErrorBody),
    ),
    security(("bearer" = []))
)]
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

#[utoipa::path(
    get,
    path = "/{name}/nodes",
    tag = INITIATIVES_TAG,
    params(("name" = String, Path, description = "The initiative name, exactly as stored."), PageQuery),
    responses(
        (status = 200, description = "One page of the initiative's shared nodes, as briefs. `pull` one by id for the full record.", body = Vec<NodeBriefView>),
        (status = 401, description = "Missing or invalid bearer token.", body = ErrorBody),
    ),
    security(("bearer" = []))
)]
async fn list_nodes(
    _: Authenticated,
    State(store): State<Arc<Store>>,
    Path(name): Path<String>,
    Query(page): Query<PageQuery>,
) -> Result<Json<Vec<NodeBriefView>>, ApiError> {
    let limit = page
        .limit
        .unwrap_or(CLOUD_DEFAULT_PAGE_SIZE)
        .clamp(1, CLOUD_MAX_PAGE_SIZE);
    let offset = page.offset.unwrap_or(0);
    let needle = page.q.as_deref().map(str::to_lowercase);
    let briefs = nodes_in_initiative(&store, &name)?;
    let views = briefs
        .into_iter()
        .filter(|b| match &needle {
            None => true,
            Some(q) => {
                b.name.to_lowercase().contains(q)
                    || b.body_excerpt
                        .as_deref()
                        .is_some_and(|e| e.to_lowercase().contains(q))
            }
        })
        .skip(offset)
        .take(limit)
        .map(|b| NodeBriefView {
            id: b.id,
            node_type: b.node_type,
            name: b.name,
            body_excerpt: b.body_excerpt,
        })
        .collect();
    Ok(Json(views))
}

#[utoipa::path(
    get,
    path = "/{name}/edges",
    tag = INITIATIVES_TAG,
    params(("name" = String, Path, description = "The initiative name, exactly as stored.")),
    responses(
        (status = 200, description = "Every edge whose both endpoints are in the initiative.", body = Vec<EdgeView>),
        (status = 401, description = "Missing or invalid bearer token.", body = ErrorBody),
    ),
    security(("bearer" = []))
)]
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

    /// The listing was unbounded: a real initiative returned 532 nodes in one
    /// response that did not fit in the caller's context and had to be spilled
    /// to a file before any of it could be read.
    #[tokio::test]
    async fn the_node_listing_is_paged() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("many");
        for i in 0..8 {
            kaeru_core::jot(&store, &format!("node number {i}")).expect("jot");
        }
        let app = api_router(AppState {
            api_token: Arc::from(""),
            store: Arc::new(store),
        });

        let page = |q: &str| {
            Request::builder()
                .uri(format!("/api/v1/initiatives/many/nodes{q}"))
                .body(Body::empty())
                .unwrap()
        };
        let count = |resp: axum::response::Response| async {
            let bytes = resp.into_body().collect().await.unwrap().to_bytes();
            serde_json::from_slice::<Vec<serde_json::Value>>(&bytes)
                .unwrap()
                .len()
        };

        assert_eq!(
            count(app.clone().oneshot(page("?limit=3")).await.unwrap()).await,
            3,
            "limit bounds the page"
        );
        assert_eq!(
            count(
                app.clone()
                    .oneshot(page("?limit=3&offset=6"))
                    .await
                    .unwrap()
            )
            .await,
            2,
            "offset walks to the tail"
        );
        assert_eq!(
            count(
                app.clone()
                    .oneshot(page("?limit=3&offset=99"))
                    .await
                    .unwrap()
            )
            .await,
            0,
            "past the end is empty, not an error"
        );
        // An out-of-range limit clamps rather than erroring: a caller asking
        // for more than the ceiling wants everything it can have.
        assert_eq!(
            count(app.oneshot(page("?limit=100000")).await.unwrap()).await,
            8
        );
    }

    /// The cloud had listing and get-by-id and no way to look for anything, so
    /// an agent whose reflex is `search` could not reach the second tier with
    /// it at all — it had to page an initiative or already know an id.
    #[tokio::test]
    async fn the_node_listing_can_be_searched() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("many");
        kaeru_core::jot(&store, "the proxy drops idle connections").expect("jot");
        kaeru_core::jot(&store, "unrelated note about billing").expect("jot");
        let app = api_router(AppState {
            api_token: Arc::from(""),
            store: Arc::new(store),
        });

        let hits = |q: &str| {
            Request::builder()
                .uri(format!("/api/v1/initiatives/many/nodes?q={q}"))
                .body(Body::empty())
                .unwrap()
        };
        let names = |resp: axum::response::Response| async {
            let bytes = resp.into_body().collect().await.unwrap().to_bytes();
            serde_json::from_slice::<Vec<serde_json::Value>>(&bytes)
                .unwrap()
                .iter()
                .map(|v| v["name"].as_str().unwrap_or("").to_string())
                .collect::<Vec<_>>()
        };

        let found = names(app.clone().oneshot(hits("proxy")).await.unwrap()).await;
        assert_eq!(found.len(), 1, "matched on the body excerpt: {found:?}");

        // Case-insensitive, and a miss is an empty list rather than an error.
        assert_eq!(
            names(app.clone().oneshot(hits("PROXY")).await.unwrap())
                .await
                .len(),
            1
        );
        assert_eq!(
            names(app.oneshot(hits("zzznope")).await.unwrap())
                .await
                .len(),
            0
        );
    }
}
