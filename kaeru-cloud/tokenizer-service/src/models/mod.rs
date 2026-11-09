use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectionState {
    pub instance_id: Uuid,
    pub source_generation: Option<Uuid>,
    pub source_revision: u64,
    pub authz_revision: u64,
    pub policy_version: u16,
    pub ready: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenQuery {
    pub actor_id: Uuid,
    pub after: Option<Uuid>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationQuery {
    pub actor_id: Option<Uuid>,
    pub source_generation: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Health {
    pub ready: bool,
}
