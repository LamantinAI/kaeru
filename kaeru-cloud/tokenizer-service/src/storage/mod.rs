mod authorization;
mod credentials;
mod operations;
mod projection;
mod rows;

use std::str::FromStr;

use contracts::{ApplyRightsResponse, ProjectionSnapshot};
use sqlx::{
    ConnectOptions, PgPool, Postgres, Transaction,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use uuid::Uuid;

use crate::cache::Cache;
use crate::codec::decode;
use crate::config::defaults::{DB_ACQUIRE_TIMEOUT, DB_STATEMENT_TIMEOUT};
use crate::error::{Error, Result};
use crate::models::ProjectionState;

#[derive(Clone)]
pub struct Store {
    pool: PgPool,
    cache: Option<Cache>,
    namespace: String,
}

impl Store {
    pub async fn connect(
        database_url: &str,
        max_connections: u32,
        cache: Option<Cache>,
        namespace: String,
    ) -> Result<Self> {
        let options = PgConnectOptions::from_str(database_url)
            .map_err(|_| Error::Config("database_url"))?
            .application_name("kaeru-tokenizer")
            .disable_statement_logging();
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .acquire_timeout(DB_ACQUIRE_TIMEOUT)
            .after_connect(|connection, _| {
                Box::pin(async move {
                    sqlx::query("SET synchronous_commit = on")
                        .execute(&mut *connection)
                        .await?;
                    sqlx::query("SELECT set_config('statement_timeout',$1,false)")
                        .bind(DB_STATEMENT_TIMEOUT)
                        .execute(&mut *connection)
                        .await?;
                    Ok(())
                })
            })
            .connect_with(options)
            .await?;
        let fsync: String = sqlx::query_scalar("SHOW fsync").fetch_one(&pool).await?;
        if fsync != "on" {
            pool.close().await;
            return Err(Error::Config("postgres_fsync"));
        }
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self {
            pool,
            cache,
            namespace,
        })
    }

    pub async fn close(&self) {
        self.pool.close().await;
    }

    pub async fn state(&self) -> Result<ProjectionState> {
        rows::state(&mut self.pool.begin().await?, false).await
    }

    async fn read_transaction(&self) -> Result<Transaction<'_, Postgres>> {
        Ok(self
            .pool
            .begin_with("BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY")
            .await?)
    }

    pub async fn projection_snapshot(&self) -> Result<ProjectionSnapshot> {
        let mut tx = self.read_transaction().await?;
        let result = rows::snapshot(&mut tx).await?;
        tx.commit().await?;
        Ok(result)
    }

    pub async fn projection_operation(
        &self,
        generation: Uuid,
        id: Uuid,
    ) -> Result<ApplyRightsResponse> {
        let bytes: Option<Vec<u8>> = sqlx::query_scalar("SELECT safe_result FROM projection_operations WHERE source_generation=$1 AND operation_id=$2")
            .bind(generation).bind(id).fetch_optional(&self.pool).await?;
        let mut result: ApplyRightsResponse = decode(&bytes.ok_or(Error::NotFound)?, 32)?;
        result.replayed = true;
        Ok(result)
    }
}
