mod codec;
mod handlers;

use std::sync::Arc;

use axum::{
    Router,
    extract::{DefaultBodyLimit, Request, State},
    http::header::AUTHORIZATION,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
};

use crate::config::defaults::REQUEST_TIMEOUT;
use crate::config::{Caller, ServiceKeys};
use crate::error::{Error, Result};
use crate::storage::Store;

#[derive(Clone)]
pub struct ApiState {
    pub store: Store,
    pub keys: ServiceKeys,
    pub max_projection_records: usize,
}

pub fn router(state: ApiState, max_body_bytes: usize) -> Router {
    let state = Arc::new(state);
    let protected = Router::new()
        .route("/v1/authorize", post(handlers::authorize))
        .route("/v1/rights/bootstrap", post(handlers::bootstrap))
        .route("/v1/rights/apply", post(handlers::apply))
        .route(
            "/v1/tokens",
            post(handlers::create_token).get(handlers::list_tokens),
        )
        .route("/v1/tokens/{id}", get(handlers::get_token))
        .route("/v1/tokens/{id}/revoke", post(handlers::revoke_token))
        .route("/v1/operations/{id}", get(handlers::operation))
        .route_layer(middleware::from_fn_with_state(state.clone(), authenticate));
    Router::new()
        .merge(protected)
        .route("/health/live", get(handlers::live))
        .route("/health/ready", get(handlers::ready))
        .fallback(|| async { Error::NotFound })
        .method_not_allowed_fallback(|| async { Error::MethodNotAllowed })
        .layer(DefaultBodyLimit::max(max_body_bytes))
        .layer(middleware::from_fn(request_timeout))
        .with_state(state)
}

async fn authenticate(
    State(state): State<Arc<ApiState>>,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let key = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(Error::Unauthorized)?;
    let caller = state.keys.identify(key).ok_or(Error::Unauthorized)?;
    request.extensions_mut().insert(caller);
    Ok(next.run(request).await)
}

pub(super) fn control(caller: Caller) -> Result<()> {
    if caller == Caller::Control {
        Ok(())
    } else {
        Err(Error::Forbidden)
    }
}

async fn request_timeout(request: Request, next: Next) -> Result<Response> {
    tokio::time::timeout(REQUEST_TIMEOUT, next.run(request))
        .await
        .map_err(|_| Error::Timeout)
}
