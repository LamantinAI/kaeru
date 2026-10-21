//! Record and mutation payloads shared by all memory transports.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::ArchiveFormat;
use super::amg::AmgOmission;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Entity {
    pub id: String,
    pub revision: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Change {
    pub field: String,
    pub before: Value,
    pub after: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct FailedStep {
    pub step: String,
    pub code: String,
    pub message: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct MutationData {
    pub primary: Option<Entity>,
    pub created: Vec<Entity>,
    pub affected: Vec<Entity>,
    pub changes: Vec<Change>,
    pub evidence: Vec<EvidenceResult>,
    pub failed_steps: Vec<FailedStep>,
    pub slots: Vec<SlotChange>,
    pub artifacts: Vec<Artifact>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct EvidenceResult {
    pub id: String,
    pub relation: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct SlotChange {
    pub slot: String,
    pub before: Option<String>,
    pub after: Option<String>,
    pub revision: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Artifact {
    pub path: String,
    pub format: ArchiveFormat,
    pub bytes: usize,
    pub sha256: String,
    pub initiatives: Vec<String>,
    pub omissions: Vec<AmgOmission>,
    pub nodes: usize,
    pub edges: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodeData {
    pub initiatives: Vec<String>,
    pub id: String,
    pub revision: String,
    pub node_type: String,
    pub tier: String,
    pub name: String,
    pub body: Option<String>,
    pub tags: Vec<String>,
    pub layer: String,
    pub visibility: String,
    pub properties: Option<Value>,
    pub currentness: String,
    pub successors: Vec<String>,
    pub status: Option<String>,
    pub recorded_at: Option<String>,
    pub created_at: Option<String>,
    pub metadata_is_current: bool,
    pub body_range: Option<ContentRange>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ContentRange {
    pub start: usize,
    pub end: usize,
    pub total: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct RelationData {
    pub revision: String,
    pub from: String,
    pub to: String,
    pub edge_type: String,
    pub weight: f64,
    pub properties: Value,
    pub dst_store: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct TrailData {
    pub id: String,
    pub name: String,
    pub summary: Option<String>,
    pub stale: Option<bool>,
    pub members: Vec<String>,
    pub summary_range: Option<ContentRange>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct RecordData {
    pub node: NodeData,
    pub conflicts: Vec<RelationData>,
    pub reasoning: Vec<TrailData>,
    pub relations: Vec<RelationData>,
    pub provenance: Vec<NodeData>,
    pub path: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchData {
    pub hits: Vec<NodeData>,
    pub total: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Section {
    pub name: String,
    pub records: Vec<NodeData>,
    pub details: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct RetentionProposal {
    pub id: String,
    pub name: String,
    pub action: String,
    pub from: String,
    pub to: String,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ContextData {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub proposals: Vec<RetentionProposal>,
    pub sections: Vec<Section>,
    pub slots: Vec<SlotChange>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct HistoryEvent {
    pub recorded_at: String,
    pub asserted: bool,
    pub name: String,
    pub body: Option<String>,
    pub body_range: Option<ContentRange>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct HistoryData {
    pub id: String,
    pub versions: Vec<HistoryEvent>,
    pub metadata_events: Vec<Value>,
    pub lineage: Vec<RelationData>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct InitiativesData {
    pub initiatives: Vec<String>,
}
