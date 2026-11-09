//! Opt-in synthetic integration route, using the server's normal read egress.
use super::{
    client::{Authorizer, Denial},
    config::{MAX_CREDENTIAL_BYTES, MAX_READ_REQUEST_BYTES},
    models::Mapping,
};
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
};
use kaeru_runtime::{ApiConfig, AuthorizedRead, authorized_read};
use serde_json::{Value, json};

#[derive(Clone)]
pub struct App {
    pub store: std::sync::Arc<kaeru_core::Store>,
    pub cfg: ApiConfig,
    pub authorizer: Authorizer,
    pub mapping: Mapping,
}
pub fn router(state: App) -> Result<Router, &'static str> {
    state.mapping.validate()?;
    Ok(Router::new()
        .route("/v1/read", post(read))
        .layer(DefaultBodyLimit::max(MAX_READ_REQUEST_BYTES))
        .with_state(state))
}
async fn read(
    State(state): State<App>,
    headers: HeaderMap,
    Json(arguments): Json<Value>,
) -> Response {
    let Some(credential) = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .filter(|v| !v.is_empty() && v.len() <= MAX_CREDENTIAL_BYTES)
    else {
        return rejected(&state, StatusCode::UNAUTHORIZED);
    };
    let Some(name) = arguments.get("initiative").and_then(Value::as_str) else {
        return rejected(&state, StatusCode::BAD_REQUEST);
    };
    let Some(&id) = state.mapping.initiatives.get(name) else {
        return rejected(&state, StatusCode::NOT_FOUND);
    };
    let caller = match state
        .authorizer
        .authorize(credential, id, state.mapping.revision)
        .await
    {
        Ok(caller) => caller,
        Err(denial) => {
            return rejected(
                &state,
                match denial {
                    Denial::Unauthorized => StatusCode::UNAUTHORIZED,
                    Denial::Forbidden => StatusCode::NOT_FOUND,
                    Denial::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
                },
            );
        }
    };
    let grant = AuthorizedRead {
        initiative: name.into(),
        caller,
    };
    authorized_read(state.store, state.cfg, arguments, grant).await
}
fn rejected(state: &App, status: StatusCode) -> Response {
    let mut response = (
        status,
        Json(json!({"error":"read authorization unavailable or denied"})),
    )
        .into_response();
    state.cfg.finish(&mut response);
    response
}
