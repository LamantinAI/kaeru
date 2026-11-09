use std::sync::Arc;

use axum::response::{IntoResponse, Response};
use axum::{
    Extension,
    extract::{Path, Query, State, rejection::QueryRejection},
};
use contracts::{
    ApplyRightsRequest, ApplyRightsResponse, AuthorizeRequest, AuthorizeResponse, BootstrapRequest,
    CreateTokenRequest, RevokeTokenRequest, TokenList, TokenMetadata, TokenOperationResponse,
};
use uuid::Uuid;

use crate::config::Caller;
use crate::config::defaults::DEFAULT_PAGE_SIZE;
use crate::error::{Error, Result};
use crate::models::{Health, OperationQuery, TokenQuery};

use super::{ApiState, codec::Cbor, control};

pub async fn authorize(
    State(state): State<Arc<ApiState>>,
    Cbor(request): Cbor<AuthorizeRequest>,
) -> Result<Cbor<AuthorizeResponse>> {
    Ok(Cbor(state.store.authorize(request).await?))
}

pub async fn bootstrap(
    State(state): State<Arc<ApiState>>,
    Extension(caller): Extension<Caller>,
    Cbor(request): Cbor<BootstrapRequest>,
) -> Result<Cbor<ApplyRightsResponse>> {
    control(caller)?;
    if request.snapshot.len() > state.max_projection_records {
        return Err(Error::Invalid("projection is too large"));
    }
    Ok(Cbor(state.store.bootstrap(request).await?))
}

pub async fn apply(
    State(state): State<Arc<ApiState>>,
    Extension(caller): Extension<Caller>,
    Cbor(request): Cbor<ApplyRightsRequest>,
) -> Result<Cbor<ApplyRightsResponse>> {
    control(caller)?;
    if request.changes.len() > state.max_projection_records {
        return Err(Error::Invalid("projection batch is too large"));
    }
    Ok(Cbor(state.store.apply(request).await?))
}

pub async fn create_token(
    State(state): State<Arc<ApiState>>,
    Extension(caller): Extension<Caller>,
    Cbor(request): Cbor<CreateTokenRequest>,
) -> Result<Cbor<TokenOperationResponse>> {
    control(caller)?;
    Ok(Cbor(state.store.create_token("control", request).await?))
}

fn uuid(value: &str) -> Result<Uuid> {
    Uuid::parse_str(value).map_err(|_| Error::Invalid("invalid resource ID"))
}
fn query<T>(value: std::result::Result<Query<T>, QueryRejection>) -> Result<T> {
    value
        .map(|q| q.0)
        .map_err(|_| Error::Invalid("invalid query"))
}

pub async fn get_token(
    State(state): State<Arc<ApiState>>,
    Extension(caller): Extension<Caller>,
    Path(id): Path<String>,
    args: std::result::Result<Query<TokenQuery>, QueryRejection>,
) -> Result<Cbor<TokenMetadata>> {
    control(caller)?;
    let args = query(args)?;
    Ok(Cbor(
        state.store.get_token(args.actor_id, uuid(&id)?).await?,
    ))
}

pub async fn list_tokens(
    State(state): State<Arc<ApiState>>,
    Extension(caller): Extension<Caller>,
    args: std::result::Result<Query<TokenQuery>, QueryRejection>,
) -> Result<Cbor<TokenList>> {
    control(caller)?;
    let args = query(args)?;
    Ok(Cbor(
        state
            .store
            .list_tokens(
                args.actor_id,
                args.after,
                args.limit.unwrap_or(DEFAULT_PAGE_SIZE),
            )
            .await?,
    ))
}

pub async fn revoke_token(
    State(state): State<Arc<ApiState>>,
    Extension(caller): Extension<Caller>,
    Path(id): Path<String>,
    Cbor(request): Cbor<RevokeTokenRequest>,
) -> Result<Cbor<TokenOperationResponse>> {
    control(caller)?;
    Ok(Cbor(
        state
            .store
            .revoke_token("control", uuid(&id)?, request)
            .await?,
    ))
}

pub async fn operation(
    State(state): State<Arc<ApiState>>,
    Extension(caller): Extension<Caller>,
    Path(id): Path<String>,
    args: std::result::Result<Query<OperationQuery>, QueryRejection>,
) -> Result<Response> {
    control(caller)?;
    let args = query(args)?;
    let id = uuid(&id)?;
    match (args.actor_id, args.source_generation) {
        (Some(actor), None) => {
            Ok(Cbor(state.store.token_operation("control", actor, id).await?).into_response())
        }
        (None, Some(generation)) => {
            Ok(Cbor(state.store.projection_operation(generation, id).await?).into_response())
        }
        _ => Err(Error::Invalid("select one operation domain")),
    }
}

pub async fn live() -> Cbor<Health> {
    Cbor(Health { ready: true })
}
pub async fn ready(State(state): State<Arc<ApiState>>) -> Result<Cbor<Health>> {
    if !state.store.state().await?.ready {
        return Err(Error::NotReady);
    }
    Ok(Cbor(Health { ready: true }))
}
