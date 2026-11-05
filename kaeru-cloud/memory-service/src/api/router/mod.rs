//! Router assembly. Health lives at the root (unauthenticated); the
//! versioned API lives under [`API_PREFIX`] and its handlers gate
//! themselves with the `Authenticated` extractor. State is attached once,
//! at the top.

pub mod edges;
pub mod health;
pub mod initiatives;
pub mod nodes;

use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::docs::{CloudApiDoc, finalise};
use crate::api::state::AppState;

/// Prefix for the versioned API surface.
pub const API_PREFIX: &str = "/api/v1";

/// Where the OpenAPI document is served (#70).
pub const OPENAPI_PATH: &str = "/openapi.json";
/// Where the interactive Swagger UI is served, reading [`OPENAPI_PATH`].
pub const DOCS_PATH: &str = "/docs";

/// Builds the full application router with state attached, and the OpenAPI
/// document alongside it.
///
/// Routes are registered through `utoipa_axum`, so the router and the document
/// are built from the same `#[utoipa::path]` attributes in one pass — a route
/// cannot exist without appearing in the spec, and the spec cannot describe a
/// path the router does not serve.
///
/// The document and the UI are always mounted, unauthenticated: there is
/// nothing to hide in the shape of the API, and a caller needs the schema
/// before it has a working token, not after.
pub fn api_router(state: AppState) -> Router {
    let v1 = OpenApiRouter::new()
        .nest("/nodes", nodes::nodes_router())
        .nest("/edges", edges::edges_router())
        .nest("/initiatives", initiatives::initiatives_router());

    let (router, mut spec) = OpenApiRouter::with_openapi(CloudApiDoc::openapi())
        .nest("/health", health::health_router())
        .nest(API_PREFIX, v1)
        .with_state(state)
        .split_for_parts();
    finalise(&mut spec);

    router.merge(SwaggerUi::new(DOCS_PATH).url(OPENAPI_PATH, spec))
}
