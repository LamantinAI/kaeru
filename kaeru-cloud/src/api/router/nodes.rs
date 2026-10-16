//! Node endpoints — the cloud's core surface for the local/cloud split.
//!
//! - `POST /api/v1/nodes` — ingest a shared node. The local daemon calls
//!   this after a node passes the share gates; the **id is preserved** so a
//!   local soft link (`dst = <id>`) resolves back here.
//! - `GET /api/v1/nodes/{id}` — fetch a node by id. Resolves a soft link
//!   lazily; id is globally unique so no initiative scope is needed.
//! - `DELETE /api/v1/nodes/{id}` — **retract** a node. Bi-temporal, not a
//!   hard delete: the node stops resolving at NOW and drops out of the
//!   initiative listings, while `at(<past>)` still reads it. That is the
//!   house model — kaeru does not delete, it marks — and the cloud simply
//!   never inherited it (#66).
//!
//! Note there is no `PUT`. `POST` is an upsert: re-posting the same id
//! asserts a new version under it, so a correction is a re-`share`, not a
//! second node alongside the first.
//!
//! Both gate themselves with the `Authenticated` extractor and delegate
//! straight to `kaeru-core` — there is no business logic in between.

use std::str::FromStr;
use std::sync::Arc;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::api::extractors::Authenticated;
use crate::api::state::AppState;
use crate::errors::ApiError;
use crate::models::ErrorBody;
pub use crate::models::{NodeIngestReq, NodeView};
use kaeru_core::{
    Layer, NodeFull, NodeType, Store, Tier, Visibility, forget, read_node_full, upsert_node,
};

pub const NODES_TAG: &str = "nodes";

pub fn nodes_router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(ingest_node))
        .routes(routes!(get_node, retract_node))
}

#[utoipa::path(
    post,
    path = "/",
    tag = NODES_TAG,
    request_body = NodeIngestReq,
    responses(
        (status = 201, description = "Stored — the node as the cloud now holds it.", body = NodeView),
        (status = 400, description = "Unknown node_type / tier / layer, an empty name, or no initiative.", body = ErrorBody),
        (status = 401, description = "Missing or invalid bearer token.", body = ErrorBody),
    ),
    security(("bearer" = []))
)]
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

    // An initiative-less node is accepted by the substrate and then invisible
    // to everything that walks initiatives — `cloud_recall`, the listings,
    // the counters. It used to answer 201 for a node nobody could ever find,
    // and with no DELETE it could not be swept up either. Refuse instead:
    // the field is optional in the type only because the struct predates the
    // rule (#66).
    let initiative = req.initiative.as_deref().map(str::trim).unwrap_or("");
    if initiative.is_empty() {
        return Err(ApiError::BadRequest(
            "initiative is required — a node without one is invisible to `cloud_recall` and to \
             the initiative listings, which is never what a share intends"
                .to_string(),
        ));
    }

    let layer = match req.layer.as_deref() {
        Some(s) if !s.trim().is_empty() => {
            Layer::from_str(s.trim()).map_err(|e| ApiError::BadRequest(e.to_string()))?
        }
        _ => Layer::default(),
    };

    // A node living in the cloud is shared by definition.
    upsert_node(
        &store,
        &req.id,
        node_type,
        tier,
        &req.name,
        req.body.as_deref(),
        &req.tags,
        Some(initiative),
        Visibility::Shared,
        layer,
        req.properties.as_ref(),
    )?;

    let full = read_node_full(&store, &req.id)?.ok_or(ApiError::NotFound)?;
    Ok((StatusCode::CREATED, Json(full_to_view(full))))
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = NODES_TAG,
    params(("id" = String, Path, description = "The node's UUIDv7, preserved from the local vault it was shared from.")),
    responses(
        (status = 200, description = "The node at NOW, untruncated.", body = NodeView),
        (status = 401, description = "Missing or invalid bearer token.", body = ErrorBody),
        (status = 404, description = "No node with that id at NOW — never shared, or retracted.", body = ErrorBody),
    ),
    security(("bearer" = []))
)]
async fn get_node(
    _: Authenticated,
    State(store): State<Arc<Store>>,
    Path(id): Path<String>,
) -> Result<Json<NodeView>, ApiError> {
    let full = read_node_full(&store, &id)?.ok_or(ApiError::NotFound)?;
    Ok(Json(full_to_view(full)))
}

/// `DELETE /api/v1/nodes/{id}` — retract a shared node.
///
/// Bi-temporal, deliberately: the node stops resolving at NOW and leaves the
/// initiative listings and `cloud_recall`, while a read at a past moment still
/// returns it. kaeru's whole model is that knowledge is superseded rather than
/// erased, and the cloud tier is the one place that had no way to say "this
/// should not have gone out" at all — so a node written by mistake, sent to
/// the wrong cloud, or carrying something the pre-share guard missed had no
/// answer short of deleting the entire initiative around it.
///
/// Idempotent: retracting a node that is already gone answers 204 rather than
/// 404, so a caller retrying after a dropped connection is not told its own
/// success was a failure.
///
/// **Whole-second caveat.** Validities are whole seconds, so a node retracted
/// inside the same second it was ingested carries an assert and a retract that
/// cannot be ordered, and may still read at NOW until the next write moves it.
/// This is the substrate's granularity rather than anything this endpoint
/// introduces — every mutation in kaeru shares it — but it surfaces here more
/// than elsewhere, because "share it, then immediately think better of it" is
/// a real sequence. Retrying the retraction a second later settles it.
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = NODES_TAG,
    params(("id" = String, Path, description = "The node's UUIDv7.")),
    responses(
        (status = 204, description = "Retracted — or already gone; the call is idempotent."),
        (status = 401, description = "Missing or invalid bearer token.", body = ErrorBody),
    ),
    security(("bearer" = []))
)]
async fn retract_node(
    _: Authenticated,
    State(store): State<Arc<Store>>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    if read_node_full(&store, &id)?.is_some() {
        forget(&store, &id)?;
    }
    Ok(StatusCode::NO_CONTENT)
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
        layer: full.layer,
        properties: full.properties,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::body::Body;
    use axum::http::{Request, StatusCode, header};
    use http_body_util::BodyExt;
    use kaeru_core::Store;
    use tower::util::ServiceExt;

    use crate::api::router::api_router;
    use crate::api::state::AppState;

    fn app() -> axum::Router {
        api_router(AppState {
            api_token: Arc::from(""),
            store: Arc::new(Store::open_in_memory().expect("open")),
        })
    }

    fn post(body: serde_json::Value) -> Request<Body> {
        Request::builder()
            .method("POST")
            .uri("/api/v1/nodes")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_string()))
            .unwrap()
    }

    fn node(id: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "node_type": "episode",
            "tier": "operational",
            "name": "a-shared-note",
            "body": "the body",
            "initiative": "t",
        })
    }

    const ID: &str = "01a03900-0000-7000-8000-000000000abc";

    /// A share had no inverse: once a node reached the cloud, the only removal
    /// on offer was deleting the whole initiative around it. Retraction is
    /// bi-temporal — the node leaves every read at NOW, its history stays.
    #[tokio::test]
    async fn a_node_can_be_retracted_and_then_reads_as_gone() {
        let app = app();

        let created = app.clone().oneshot(post(node(ID))).await.unwrap();
        assert_eq!(created.status(), StatusCode::CREATED);

        // Validities are whole seconds: an assert and a retract inside one
        // second cannot be ordered. See the note on `retract_node`.
        tokio::time::sleep(std::time::Duration::from_millis(1100)).await;

        let found = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(format!("/api/v1/nodes/{ID}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(found.status(), StatusCode::OK, "present before");

        let gone = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri(format!("/api/v1/nodes/{ID}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(gone.status(), StatusCode::NO_CONTENT);

        let after = app
            .oneshot(
                Request::builder()
                    .uri(format!("/api/v1/nodes/{ID}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(after.status(), StatusCode::NOT_FOUND, "absent after");
    }

    /// Retracting something already gone is a success, not a failure — a
    /// caller retrying after a dropped connection must not be told its own
    /// success was an error.
    #[tokio::test]
    async fn retraction_is_idempotent() {
        let app = app();
        let del = || {
            Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/nodes/{ID}"))
                .body(Body::empty())
                .unwrap()
        };
        assert_eq!(
            app.clone().oneshot(del()).await.unwrap().status(),
            StatusCode::NO_CONTENT,
            "never existed"
        );
        app.clone().oneshot(post(node(ID))).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(1100)).await;
        assert_eq!(
            app.clone().oneshot(del()).await.unwrap().status(),
            StatusCode::NO_CONTENT
        );
        assert_eq!(
            app.oneshot(del()).await.unwrap().status(),
            StatusCode::NO_CONTENT,
            "and again"
        );
    }

    /// A node with no initiative is invisible to everything that walks
    /// initiatives, so 201 was a success answer for a node nobody could find.
    #[tokio::test]
    async fn a_node_without_an_initiative_is_refused() {
        let mut payload = node(ID);
        payload.as_object_mut().unwrap().remove("initiative");
        let resp = app().oneshot(post(payload)).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

        let bytes = resp.into_body().collect().await.unwrap().to_bytes();
        let text = String::from_utf8_lossy(&bytes);
        assert!(text.contains("initiative is required"), "{text}");
        assert!(text.contains("cloud_recall"), "and says why: {text}");
    }

    /// There is no PUT because POST is an upsert: correcting a shared node is
    /// a re-share under the same id, not a second node beside the first.
    #[tokio::test]
    async fn re_posting_the_same_id_updates_in_place() {
        let app = app();
        app.clone().oneshot(post(node(ID))).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(1100)).await;

        let mut fixed = node(ID);
        fixed["body"] = serde_json::json!("the corrected body");
        assert_eq!(
            app.clone().oneshot(post(fixed)).await.unwrap().status(),
            StatusCode::CREATED
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .uri(format!("/api/v1/nodes/{ID}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let bytes = resp.into_body().collect().await.unwrap().to_bytes();
        let view: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(view["body"], "the corrected body");
    }

    /// A `cite`'s URL lives in `properties`, and the field was absent from
    /// both the ingest request and the view — so of 78 references held in a
    /// live cloud, not one carried the link it was cited for (#85).
    #[tokio::test]
    async fn a_citations_url_survives_the_round_trip() {
        let app = app();
        let mut payload = node(ID);
        payload["node_type"] = serde_json::json!("reference");
        payload["tier"] = serde_json::json!("archival");
        payload["properties"] = serde_json::json!({ "url": "https://example.org/paper" });
        assert_eq!(
            app.clone().oneshot(post(payload)).await.unwrap().status(),
            StatusCode::CREATED
        );

        let resp = app
            .oneshot(
                Request::builder()
                    .uri(format!("/api/v1/nodes/{ID}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let bytes = resp.into_body().collect().await.unwrap().to_bytes();
        let view: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(
            view["properties"]["url"], "https://example.org/paper",
            "the field the node exists for came back: {view}"
        );
    }

    /// An older client sends no `properties` at all. That must read as "I have
    /// nothing to say about them", not as "clear them" — the destructive case
    /// was re-posting a reference and losing its URL.
    #[tokio::test]
    async fn an_omitted_properties_field_does_not_erase_what_is_stored() {
        let app = app();
        let mut first = node(ID);
        first["properties"] = serde_json::json!({ "url": "https://example.org/paper" });
        app.clone().oneshot(post(first)).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(1100)).await;

        // The same node from a client that predates the field.
        app.clone().oneshot(post(node(ID))).await.unwrap();

        let resp = app
            .oneshot(
                Request::builder()
                    .uri(format!("/api/v1/nodes/{ID}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let bytes = resp.into_body().collect().await.unwrap().to_bytes();
        let view: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(
            view["properties"]["url"], "https://example.org/paper",
            "the URL survived a push that never mentioned it: {view}"
        );
    }

    /// Until #85 an edge could only die by retracting one of its endpoint
    /// nodes, so a local `unlink` had nowhere to send itself and the next
    /// `pull` put the edge straight back.
    #[tokio::test]
    async fn an_edge_can_be_retracted_without_destroying_its_endpoints() {
        const OTHER: &str = "01a03900-0000-7000-8000-000000000def";
        let app = app();
        app.clone().oneshot(post(node(ID))).await.unwrap();
        let mut second = node(OTHER);
        second["name"] = serde_json::json!("the-other-note");
        app.clone().oneshot(post(second)).await.unwrap();

        let edge = |method: &str| {
            Request::builder()
                .method(method)
                .uri("/api/v1/edges")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "src": ID, "dst": OTHER, "edge_type": "causal", "weight": 0.9
                    })
                    .to_string(),
                ))
                .unwrap()
        };

        assert_eq!(
            app.clone().oneshot(edge("POST")).await.unwrap().status(),
            StatusCode::CREATED
        );
        let listed = |app: axum::Router| async move {
            let resp = app
                .oneshot(
                    Request::builder()
                        .uri("/api/v1/initiatives/t/edges")
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();
            let bytes = resp.into_body().collect().await.unwrap().to_bytes();
            serde_json::from_slice::<serde_json::Value>(&bytes)
                .unwrap()
                .as_array()
                .map(|a| a.len())
                .unwrap_or(0)
        };
        assert_eq!(listed(app.clone()).await, 1, "the edge is there");

        tokio::time::sleep(std::time::Duration::from_millis(1100)).await;
        assert_eq!(
            app.clone().oneshot(edge("DELETE")).await.unwrap().status(),
            StatusCode::NO_CONTENT
        );
        assert_eq!(listed(app.clone()).await, 0, "and gone, without the nodes");

        // Both endpoints still read — the point of having a DELETE for edges.
        let resp = app
            .oneshot(
                Request::builder()
                    .uri(format!("/api/v1/nodes/{ID}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK, "the node outlives its edge");
    }

    /// Retracting an edge that is already gone answers 204, so a client
    /// retrying after a dropped connection is not told its success failed.
    #[tokio::test]
    async fn edge_retraction_is_idempotent() {
        let app = app();
        let req = Request::builder()
            .method("DELETE")
            .uri("/api/v1/edges")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(
                serde_json::json!({
                    "src": ID, "dst": ID, "edge_type": "refers_to"
                })
                .to_string(),
            ))
            .unwrap();
        assert_eq!(
            app.oneshot(req).await.unwrap().status(),
            StatusCode::NO_CONTENT
        );
    }
}
