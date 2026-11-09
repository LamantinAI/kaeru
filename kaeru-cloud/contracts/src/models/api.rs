use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Action, ProjectionChange, ProjectionSnapshot, SecretToken};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenPurpose {
    Memory,
    Control,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "project_ids",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum TokenCeiling {
    AllCurrent,
    Projects(Vec<Uuid>),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessMode {
    #[default]
    Normal,
    InspectDeleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "id",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum Resource {
    Platform,
    Principal(Uuid),
    PersonalSpace(Uuid),
    Organization(Uuid),
    Membership(Uuid),
    Project(Uuid),
    Initiative(Uuid),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccessCheck {
    pub action: Action,
    pub resource: Resource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizeRequest {
    pub schema_version: u16,
    pub request_id: Uuid,
    pub credential: SecretToken,
    pub checks: Vec<AccessCheck>,
    #[serde(default)]
    pub mode: AccessMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizeResponse {
    pub schema_version: u16,
    pub request_id: Uuid,
    pub allowed: bool,
    pub principal_id: Uuid,
    pub checks: Vec<AccessCheck>,
    pub mode: AccessMode,
    pub source_generation: Uuid,
    pub source_revision: u64,
    pub authz_revision: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BootstrapRequest {
    pub schema_version: u16,
    pub policy_version: u16,
    pub operation_id: Uuid,
    pub source_generation: Uuid,
    pub source_revision: u64,
    pub snapshot: ProjectionSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApplyRightsRequest {
    pub schema_version: u16,
    pub policy_version: u16,
    pub operation_id: Uuid,
    pub source_generation: Uuid,
    pub expected_revision: u64,
    pub source_revision: u64,
    pub changes: Vec<ProjectionChange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApplyRightsResponse {
    pub schema_version: u16,
    pub operation_id: Uuid,
    pub source_generation: Uuid,
    pub source_revision: u64,
    pub authz_revision: u64,
    pub replayed: bool,
}

/// Sent only by the trusted session gateway, never authenticated by a product token.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateTokenRequest {
    pub schema_version: u16,
    pub operation_id: Uuid,
    pub actor_id: Uuid,
    pub name: String,
    pub purpose: TokenPurpose,
    pub ceiling: TokenCeiling,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenMetadata {
    pub id: Uuid,
    pub principal_id: Uuid,
    pub name: String,
    pub purpose: TokenPurpose,
    pub ceiling: TokenCeiling,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenOperationResponse {
    pub schema_version: u16,
    pub operation_id: Uuid,
    pub token: TokenMetadata,
    /// Present only on the winning create response, never in operation storage.
    pub secret: Option<SecretToken>,
    pub replayed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RevokeTokenRequest {
    pub schema_version: u16,
    pub operation_id: Uuid,
    pub actor_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenList {
    pub schema_version: u16,
    pub tokens: Vec<TokenMetadata>,
    pub next_after: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ErrorResponse {
    pub schema_version: u16,
    pub code: String,
    pub message: String,
}
