//! Stable operation identity, replay and safe durable results.
use crate::codec::{decode, encode};
use crate::error::{Error, Result};
use contracts::{SCHEMA_VERSION, TokenMetadata, TokenOperationResponse};
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub(super) enum OperationKind {
    Create,
    Revoke,
}
impl OperationKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Revoke => "revoke",
        }
    }
}
pub(super) struct OperationIdentity<'a> {
    pub service: &'a str,
    pub actor: Uuid,
    pub id: Uuid,
    pub digest: Vec<u8>,
    pub kind: OperationKind,
}

pub(super) fn digest<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    Ok(Sha256::digest(encode(value)?).to_vec())
}

pub(super) async fn replay(
    tx: &mut Transaction<'_, Postgres>,
    operation: &OperationIdentity<'_>,
) -> Result<Option<TokenOperationResponse>> {
    let row:Option<(Vec<u8>,Vec<u8>,String)>=sqlx::query_as("SELECT digest,safe_result,kind FROM token_operations WHERE service_id=$1 AND actor_id=$2 AND operation_id=$3")
        .bind(operation.service).bind(operation.actor).bind(operation.id).fetch_optional(&mut **tx).await?;
    if let Some((stored, bytes, stored_kind)) = row {
        if stored != operation.digest || stored_kind != operation.kind.as_str() {
            return Err(Error::Conflict("operation ID has different content"));
        }
        let mut result: TokenOperationResponse = decode(&bytes, 32)?;
        if result.secret.is_some() {
            return Err(Error::Corrupt);
        }
        result.replayed = true;
        Ok(Some(result))
    } else {
        Ok(None)
    }
}
pub(super) async fn save_operation(
    tx: &mut Transaction<'_, Postgres>,
    operation: &OperationIdentity<'_>,
    token: TokenMetadata,
) -> Result<TokenOperationResponse> {
    let result = TokenOperationResponse {
        schema_version: SCHEMA_VERSION,
        operation_id: operation.id,
        token,
        secret: None,
        replayed: false,
    };
    sqlx::query("INSERT INTO token_operations(service_id,actor_id,operation_id,kind,digest,token_id,safe_result) VALUES($1,$2,$3,$4,$5,$6,$7)")
        .bind(operation.service).bind(operation.actor).bind(operation.id).bind(operation.kind.as_str()).bind(&operation.digest).bind(result.token.id).bind(encode(&result)?).execute(&mut **tx).await?;
    Ok(result)
}
