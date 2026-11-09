//! First-run workflow against an empty, isolated tokenizer installation.

use std::env;
use std::error::Error;

use contracts::{
    AccessCheck, AccessMode, Action, ActionMask, ApplyRightsRequest, ApplyRightsResponse,
    AuthorizeRequest, AuthorizeResponse, BootstrapRequest, CreateTokenRequest, Initiative,
    Membership, MembershipState, Organization, OrganizationRole, OrganizationState, POLICY_VERSION,
    PlatformRole, Principal, PrincipalState, Project, ProjectionChange, ProjectionSnapshot,
    Resource, ResourceState, RevokeTokenRequest, SCHEMA_VERSION, TokenCeiling,
    TokenOperationResponse, TokenPurpose,
};
use kaeru_tokenizer_service::codec::{decode, encode};
use reqwest::{Client, StatusCode, Version};
use serde::{Serialize, de::DeserializeOwned};
use uuid::Uuid;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

struct ClientContext {
    client: Client,
    base: String,
    control: String,
    memory: String,
}

impl ClientContext {
    async fn post<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        key: &str,
        value: &T,
    ) -> Result<R> {
        let response = self
            .client
            .post(format!("{}{}", self.base, path))
            .bearer_auth(key)
            .header("content-type", "application/cbor")
            .body(encode(value)?)
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(format!("API operation failed: {}", response.status()).into());
        }
        if response.version() != Version::HTTP_2 {
            return Err("HTTP/2 was not negotiated".into());
        }
        decode(&response.bytes().await?, 32).map_err(|_| "invalid CBOR response".into())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let context = ClientContext {
        client: Client::builder().http2_prior_knowledge().build()?,
        base: env::var("TOKENIZER_BASE_URL")?,
        control: env::var("TOKENIZER_CONTROL_KEY")?,
        memory: env::var("TOKENIZER_MEMORY_KEY")?,
    };
    let owner = Uuid::now_v7();
    let user = Uuid::now_v7();
    let org = Uuid::now_v7();
    let project = Uuid::now_v7();
    let initiative = Uuid::now_v7();
    let generation = Uuid::now_v7();
    let snapshot = ProjectionSnapshot {
        principals: vec![
            Principal {
                id: owner,
                role: PlatformRole::User,
                state: PrincipalState::Active,
            },
            Principal {
                id: user,
                role: PlatformRole::User,
                state: PrincipalState::Active,
            },
        ],
        organizations: vec![Organization {
            id: org,
            owner_id: owner,
            state: OrganizationState::Active,
            deletion_reason: None,
        }],
        memberships: vec![
            Membership {
                id: Uuid::now_v7(),
                principal_id: owner,
                organization_id: org,
                role: OrganizationRole::Admin,
                state: MembershipState::Active,
                all_projects: false,
                all_actions: ActionMask::EMPTY,
            },
            Membership {
                id: Uuid::now_v7(),
                principal_id: user,
                organization_id: org,
                role: OrganizationRole::Member,
                state: MembershipState::Active,
                all_projects: true,
                all_actions: ActionMask::of(&[Action::Read]),
            },
        ],
        projects: vec![Project {
            id: project,
            organization_id: org,
            state: ResourceState::Active,
        }],
        initiatives: vec![Initiative {
            id: initiative,
            personal_owner_id: None,
            project_id: Some(project),
            state: ResourceState::Active,
            placement_revision: 1,
        }],
        ..ProjectionSnapshot::default()
    };
    let _: ApplyRightsResponse = context
        .post(
            "/v1/rights/bootstrap",
            &context.control,
            &BootstrapRequest {
                schema_version: SCHEMA_VERSION,
                policy_version: POLICY_VERSION,
                operation_id: Uuid::now_v7(),
                source_generation: generation,
                source_revision: 0,
                snapshot,
            },
        )
        .await?;
    let created: TokenOperationResponse = context
        .post(
            "/v1/tokens",
            &context.control,
            &CreateTokenRequest {
                schema_version: SCHEMA_VERSION,
                operation_id: Uuid::now_v7(),
                actor_id: user,
                name: "First-run workflow".into(),
                purpose: TokenPurpose::Memory,
                ceiling: TokenCeiling::AllCurrent,
                expires_at: None,
            },
        )
        .await?;
    let secret = created
        .secret
        .ok_or("initial create did not return a secret")?;
    let query = AuthorizeRequest {
        schema_version: SCHEMA_VERSION,
        request_id: Uuid::now_v7(),
        credential: secret,
        checks: vec![AccessCheck {
            action: Action::Read,
            resource: Resource::Initiative(initiative),
        }],
        mode: AccessMode::Normal,
    };
    let result: AuthorizeResponse = context
        .post("/v1/authorize", &context.memory, &query)
        .await?;
    if !result.allowed {
        return Err("initial inherited read was denied".into());
    }
    let new_project = Uuid::now_v7();
    let new_initiative = Uuid::now_v7();
    let _: ApplyRightsResponse = context
        .post(
            "/v1/rights/apply",
            &context.control,
            &ApplyRightsRequest {
                schema_version: SCHEMA_VERSION,
                policy_version: POLICY_VERSION,
                operation_id: Uuid::now_v7(),
                source_generation: generation,
                expected_revision: 0,
                source_revision: 1,
                changes: vec![
                    ProjectionChange::Project(Project {
                        id: new_project,
                        organization_id: org,
                        state: ResourceState::Active,
                    }),
                    ProjectionChange::Initiative(Initiative {
                        id: new_initiative,
                        personal_owner_id: None,
                        project_id: Some(new_project),
                        state: ResourceState::Active,
                        placement_revision: 1,
                    }),
                ],
            },
        )
        .await?;
    let mut query = query;
    query.request_id = Uuid::now_v7();
    query.checks[0].resource = Resource::Initiative(new_initiative);
    let result: AuthorizeResponse = context
        .post("/v1/authorize", &context.memory, &query)
        .await?;
    if !result.allowed {
        return Err("all-projects grant did not cover new project".into());
    }
    let _: TokenOperationResponse = context
        .post(
            &format!("/v1/tokens/{}/revoke", created.token.id),
            &context.control,
            &RevokeTokenRequest {
                schema_version: SCHEMA_VERSION,
                operation_id: Uuid::now_v7(),
                actor_id: user,
            },
        )
        .await?;
    let response = context
        .client
        .post(format!("{}/v1/authorize", context.base))
        .bearer_auth(&context.memory)
        .header("content-type", "application/cbor")
        .body(encode(&query)?)
        .send()
        .await?;
    if response.status() != StatusCode::UNAUTHORIZED {
        return Err("revoked credential was not rejected".into());
    }
    println!(
        "HTTP/2 + CBOR: bootstrap, one-time issuance, inherited read, new-project access and revoke passed."
    );
    Ok(())
}
