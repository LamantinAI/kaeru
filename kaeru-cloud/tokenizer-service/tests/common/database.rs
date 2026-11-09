use std::env;
use std::time::Duration;

use contracts::{
    AccessCheck, AccessMode, ApplyRightsRequest, ApplyRightsResponse, AuthorizeRequest,
    AuthorizeResponse, BootstrapRequest, CreateTokenRequest, POLICY_VERSION, ProjectionChange,
    SCHEMA_VERSION, SecretToken, TokenCeiling, TokenPurpose,
};
use kaeru_tokenizer_service::{cache::Cache, error::Result, storage::Store};
use reqwest::Url;
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

use super::{USER, snapshot};

pub struct Database {
    pub store: Store,
    pub url: String,
    pub namespace: String,
    admin: PgPool,
    name: String,
}

impl Database {
    pub async fn new() -> Self {
        let base=env::var("TOKENIZER_TEST_DATABASE_URL").expect("set TOKENIZER_TEST_DATABASE_URL to an isolated PostgreSQL with CREATE DATABASE permission");
        let admin = PgPoolOptions::new()
            .max_connections(2)
            .connect(&base)
            .await
            .unwrap();
        let name = format!("tokenizer_test_{}", Uuid::now_v7().simple());
        sqlx::query(&format!("CREATE DATABASE \"{name}\""))
            .execute(&admin)
            .await
            .unwrap();
        let mut url = Url::parse(&base).unwrap();
        url.set_path(&format!("/{name}"));
        let namespace = format!("tokenizer:test:{}", Uuid::now_v7().simple());
        let cache = if let Ok(value) = env::var("TOKENIZER_TEST_VALKEY_URL") {
            Cache::connect(
                &value,
                Duration::from_secs(30),
                Duration::from_millis(100),
                2 * 1024 * 1024,
            )
            .await
        } else {
            None
        };
        let store = Store::connect(url.as_str(), 8, cache, namespace.clone())
            .await
            .unwrap();
        Self {
            store,
            url: url.into(),
            namespace,
            admin,
            name,
        }
    }

    pub async fn cleanup(self) {
        self.store.close().await;
        sqlx::query(&format!("DROP DATABASE \"{}\" WITH (FORCE)", self.name))
            .execute(&self.admin)
            .await
            .unwrap();
        self.admin.close().await;
    }

    pub async fn bootstrap(&self) -> ApplyRightsResponse {
        self.store
            .bootstrap(BootstrapRequest {
                schema_version: SCHEMA_VERSION,
                policy_version: POLICY_VERSION,
                operation_id: Uuid::now_v7(),
                source_generation: Uuid::now_v7(),
                source_revision: 0,
                snapshot: snapshot(),
            })
            .await
            .unwrap()
    }

    pub async fn apply(&self, changes: Vec<ProjectionChange>) -> Result<ApplyRightsResponse> {
        let state = self.store.state().await?;
        self.store
            .apply(ApplyRightsRequest {
                schema_version: SCHEMA_VERSION,
                policy_version: POLICY_VERSION,
                operation_id: Uuid::now_v7(),
                source_generation: state.source_generation.unwrap(),
                expected_revision: state.source_revision,
                source_revision: state.source_revision + 1,
                changes,
            })
            .await
    }
}

pub fn create() -> CreateTokenRequest {
    CreateTokenRequest {
        schema_version: SCHEMA_VERSION,
        operation_id: Uuid::now_v7(),
        actor_id: USER,
        name: "Synthetic integration credential".into(),
        purpose: TokenPurpose::Memory,
        ceiling: TokenCeiling::AllCurrent,
        expires_at: None,
    }
}

pub async fn authorize(
    store: &Store,
    token: &SecretToken,
    checks: Vec<AccessCheck>,
) -> Result<AuthorizeResponse> {
    store
        .authorize(AuthorizeRequest {
            schema_version: SCHEMA_VERSION,
            request_id: Uuid::now_v7(),
            credential: token.clone(),
            checks,
            mode: AccessMode::Normal,
        })
        .await
}
