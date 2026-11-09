use chrono::{DateTime, Months, SubsecRound, Utc};
use contracts::{
    CreateTokenRequest, PrincipalState, RevokeTokenRequest, SCHEMA_VERSION, TokenCeiling,
    TokenList, TokenMetadata, TokenOperationResponse, TokenPurpose,
};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::codec::decode;
use crate::config::defaults::{DEFAULT_TOKEN_MONTHS, MAX_BATCH, MAX_TOKEN_NAME};
use crate::credentials::mint;
use crate::error::{Error, Result};

use super::{
    Store,
    operations::{OperationIdentity, OperationKind, digest, replay, save_operation},
    rows::{self, PrincipalRow, TokenRow},
};

pub fn default_expiry(now: DateTime<Utc>) -> Result<DateTime<Utc>> {
    now.checked_add_months(Months::new(DEFAULT_TOKEN_MONTHS))
        .ok_or(Error::Invalid("expiry exceeds supported range"))
}

pub(super) async fn active_actor(tx: &mut Transaction<'_, Postgres>, actor: Uuid) -> Result<()> {
    let state = rows::state(tx, false).await?;
    if !state.ready {
        return Err(Error::NotReady);
    }
    let principal = sqlx::query_as::<_, PrincipalRow>("SELECT * FROM principals WHERE id=$1")
        .bind(actor)
        .fetch_optional(&mut **tx)
        .await?
        .ok_or(Error::Unauthorized)?
        .wire()?;
    if principal.state != PrincipalState::Active {
        return Err(Error::Unauthorized);
    }
    Ok(())
}

impl Store {
    /// The caller is a trusted session gateway. Product bearer keys cannot call it.
    pub async fn create_token(
        &self,
        service: &str,
        mut request: CreateTokenRequest,
    ) -> Result<TokenOperationResponse> {
        if request.schema_version != SCHEMA_VERSION
            || request.name.trim().is_empty()
            || request.name.chars().count() > MAX_TOKEN_NAME
        {
            return Err(Error::Invalid("invalid token creation request"));
        }
        if let TokenCeiling::Projects(ids) = &mut request.ceiling {
            ids.sort();
            if ids.is_empty() || ids.len() > MAX_BATCH || ids.windows(2).any(|w| w[0] == w[1]) {
                return Err(Error::Invalid("invalid project ceiling"));
            }
        }
        request.expires_at = request.expires_at.map(|t| t.trunc_subsecs(6));
        let operation = OperationIdentity {
            service,
            actor: request.actor_id,
            id: request.operation_id,
            digest: digest(&request)?,
            kind: OperationKind::Create,
        };
        let mut tx = self.pool.begin().await?;
        rows::state(&mut tx, true).await?;
        active_actor(&mut tx, request.actor_id).await?;
        if let Some(result) = replay(&mut tx, &operation).await? {
            tx.commit().await?;
            return Ok(result);
        }
        let now = Utc::now().trunc_subsecs(6);
        let expires = request.expires_at.unwrap_or(default_expiry(now)?);
        if expires <= now {
            return Err(Error::Invalid("expiry must be in the future"));
        }
        if let TokenCeiling::Projects(ids) = &request.ceiling {
            let count: i64 = sqlx::query_scalar("SELECT count(*) FROM projects WHERE id=ANY($1)")
                .bind(ids)
                .fetch_one(&mut *tx)
                .await?;
            if count != ids.len() as i64 {
                return Err(Error::Invalid("project ceiling is unavailable"));
            }
        }
        let id = Uuid::now_v7();
        let (secret, verifier) = mint(id)?;
        let purpose = match request.purpose {
            TokenPurpose::Memory => "memory",
            TokenPurpose::Control => "control",
        };
        let ceiling_mode = match request.ceiling {
            TokenCeiling::AllCurrent => "all_current",
            TokenCeiling::Projects(_) => "projects",
        };
        sqlx::query("INSERT INTO tokens(id,principal_id,verifier,format_version,purpose,ceiling_mode,name,created_at,expires_at) VALUES($1,$2,$3,1,$4,$5,$6,$7,$8)")
            .bind(id).bind(request.actor_id).bind(verifier.as_slice()).bind(purpose).bind(ceiling_mode).bind(&request.name).bind(now).bind(expires).execute(&mut *tx).await?;
        if let TokenCeiling::Projects(ids) = &request.ceiling {
            sqlx::query(
                "INSERT INTO token_projects(token_id,project_id) SELECT $1,unnest($2::uuid[])",
            )
            .bind(id)
            .bind(ids)
            .execute(&mut *tx)
            .await?;
        }
        sqlx::query(
            "UPDATE projection_state SET authz_revision=authz_revision+1 WHERE singleton=true",
        )
        .execute(&mut *tx)
        .await?;
        let token = TokenMetadata {
            id,
            principal_id: request.actor_id,
            name: request.name,
            purpose: request.purpose,
            ceiling: request.ceiling,
            created_at: now,
            expires_at: expires,
            revoked_at: None,
        };
        let mut result = save_operation(&mut tx, &operation, token).await?;
        tx.commit().await?;
        result.secret = Some(secret);
        Ok(result)
    }

    pub async fn revoke_token(
        &self,
        service: &str,
        id: Uuid,
        request: RevokeTokenRequest,
    ) -> Result<TokenOperationResponse> {
        if request.schema_version != SCHEMA_VERSION {
            return Err(Error::Invalid("unsupported schema version"));
        }
        let operation = OperationIdentity {
            service,
            actor: request.actor_id,
            id: request.operation_id,
            digest: digest(&(id, &request))?,
            kind: OperationKind::Revoke,
        };
        let mut tx = self.pool.begin().await?;
        rows::state(&mut tx, true).await?;
        active_actor(&mut tx, request.actor_id).await?;
        if let Some(result) = replay(&mut tx, &operation).await? {
            tx.commit().await?;
            return Ok(result);
        }
        let row = sqlx::query_as::<_, TokenRow>(
            "SELECT * FROM tokens WHERE id=$1 AND principal_id=$2 FOR UPDATE",
        )
        .bind(id)
        .bind(request.actor_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(Error::NotFound)?;
        let mut token = rows::metadata(&mut tx, &row).await?;
        if token.revoked_at.is_none() {
            let now = Utc::now().trunc_subsecs(6);
            sqlx::query("UPDATE tokens SET revoked_at=$2 WHERE id=$1")
                .bind(id)
                .bind(now)
                .execute(&mut *tx)
                .await?;
            sqlx::query(
                "UPDATE projection_state SET authz_revision=authz_revision+1 WHERE singleton=true",
            )
            .execute(&mut *tx)
            .await?;
            token.revoked_at = Some(now);
        }
        let result = save_operation(&mut tx, &operation, token).await?;
        tx.commit().await?;
        Ok(result)
    }

    pub async fn get_token(&self, actor: Uuid, id: Uuid) -> Result<TokenMetadata> {
        let mut tx = self.read_transaction().await?;
        active_actor(&mut tx, actor).await?;
        let row =
            sqlx::query_as::<_, TokenRow>("SELECT * FROM tokens WHERE id=$1 AND principal_id=$2")
                .bind(id)
                .bind(actor)
                .fetch_optional(&mut *tx)
                .await?
                .ok_or(Error::NotFound)?;
        let result = rows::metadata(&mut tx, &row).await?;
        tx.commit().await?;
        Ok(result)
    }

    pub async fn list_tokens(
        &self,
        actor: Uuid,
        after: Option<Uuid>,
        limit: usize,
    ) -> Result<TokenList> {
        if limit == 0 || limit > MAX_BATCH {
            return Err(Error::Invalid("invalid page size"));
        }
        let mut tx = self.read_transaction().await?;
        active_actor(&mut tx, actor).await?;
        let mut rows=sqlx::query_as::<_,TokenRow>("SELECT * FROM tokens WHERE principal_id=$1 AND ($2::uuid IS NULL OR id>$2) ORDER BY id LIMIT $3")
            .bind(actor).bind(after).bind((limit+1) as i64).fetch_all(&mut *tx).await?;
        let has_more = rows.len() > limit;
        rows.truncate(limit);
        let next_after = if has_more {
            rows.last().map(|r| r.id)
        } else {
            None
        };
        let mut tokens = Vec::with_capacity(rows.len());
        for row in rows {
            tokens.push(rows::metadata(&mut tx, &row).await?);
        }
        tx.commit().await?;
        Ok(TokenList {
            schema_version: SCHEMA_VERSION,
            tokens,
            next_after,
        })
    }

    pub async fn token_operation(
        &self,
        service: &str,
        actor: Uuid,
        id: Uuid,
    ) -> Result<TokenOperationResponse> {
        let mut tx = self.read_transaction().await?;
        active_actor(&mut tx, actor).await?;
        let bytes:Option<Vec<u8>>=sqlx::query_scalar("SELECT safe_result FROM token_operations WHERE service_id=$1 AND actor_id=$2 AND operation_id=$3")
            .bind(service).bind(actor).bind(id).fetch_optional(&mut *tx).await?;
        let mut result: TokenOperationResponse = decode(&bytes.ok_or(Error::NotFound)?, 32)?;
        if result.secret.is_some() {
            return Err(Error::Corrupt);
        }
        result.replayed = true;
        tx.commit().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::default_expiry;
    use chrono::{TimeZone, Utc};

    #[test]
    fn calendar_month_expiry_clamps_without_changing_time() {
        let now = Utc.with_ymd_and_hms(2023, 8, 31, 12, 34, 56).unwrap();
        assert_eq!(
            default_expiry(now).unwrap(),
            Utc.with_ymd_and_hms(2024, 2, 29, 12, 34, 56).unwrap()
        );
        let now = Utc.with_ymd_and_hms(2024, 8, 31, 12, 34, 56).unwrap();
        assert_eq!(
            default_expiry(now).unwrap(),
            Utc.with_ymd_and_hms(2025, 2, 28, 12, 34, 56).unwrap()
        );
    }
}
