//! Shared input value objects and result DTOs for the canonical API.

use std::path::PathBuf;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::API_VERSION;
use crate::EdgeType;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NodeRef {
    NameOrId(String),
    Id(IdRef),
    Scoped(ScopedRef),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IdRef {
    pub id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScopedRef {
    pub name: String,
    pub initiative: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Evidence {
    Existing(ExistingEvidence),
    Check(CheckEvidence),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExistingEvidence {
    pub node: NodeRef,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckEvidence {
    pub method: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Source {
    Text(TextSource),
    Artifact(ArtifactSource),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextSource {
    pub text: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactSource {
    pub artifact: String,
}

/// Host-owned capabilities, never accepted as model arguments.
#[derive(Clone, Debug, Default)]
pub struct ApiOptions {
    /// Reads and exports of files must stay within this configured root.
    pub artifact_root: Option<PathBuf>,
    /// None means the trusted local owner. HTTP adapters supply their ceiling.
    pub allowed_initiatives: Option<Vec<String>>,
    pub denied_initiatives: Vec<String>,
    pub include_local: bool,
    /// Identity used for persistent replay records, supplied by the adapter.
    pub caller: String,
}

impl ApiOptions {
    pub fn local() -> Self {
        Self {
            include_local: true,
            caller: "local-owner".into(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolAnnotations {
    pub read_only_hint: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
    pub annotations: ToolAnnotations,
    pub priority: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Issue {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    pub retryable: bool,
}

impl Issue {
    pub fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            field: None,
            retryable: false,
        }
    }
    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Ok,
    Applied,
    Unchanged,
    Partial,
    Rejected,
}

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

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ResponseData {
    Mutation(MutationData),
    Record(Box<RecordData>),
    Search(SearchData),
    Context(ContextData),
    History(HistoryData),
    Initiatives(InitiativesData),
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Omission {
    pub section: String,
    pub count: usize,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Coverage {
    pub initiatives: Vec<String>,
    pub complete: bool,
    pub rules_complete: bool,
    pub omitted: Vec<Omission>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Continuation {
    pub cursor: String,
    pub next_arguments: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Response {
    pub api_version: u32,
    pub operation: String,
    pub state: State,
    pub request_id: Option<String>,
    pub replayed: bool,
    pub data: ResponseData,
    pub issues: Vec<Issue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<Coverage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<Continuation>,
}

impl Response {
    pub fn mutation(operation: &str, data: MutationData) -> Self {
        Self {
            api_version: API_VERSION,
            operation: operation.into(),
            state: State::Applied,
            request_id: None,
            replayed: false,
            data: ResponseData::Mutation(data),
            issues: vec![],
            coverage: None,
            continuation: None,
        }
    }
    pub fn read(operation: &str, data: ResponseData, scopes: Vec<String>) -> Self {
        Self {
            api_version: API_VERSION,
            operation: operation.into(),
            state: State::Ok,
            request_id: None,
            replayed: false,
            data,
            issues: vec![],
            coverage: Some(Coverage {
                initiatives: scopes,
                complete: true,
                rules_complete: true,
                omitted: vec![],
            }),
            continuation: None,
        }
    }
    pub fn rejected(operation: &str, issue: Issue) -> Self {
        let mut result = Self::mutation(operation, MutationData::default());
        result.state = State::Rejected;
        result.issues.push(issue);
        result
    }
    pub fn is_error(&self) -> bool {
        matches!(self.state, State::Rejected | State::Partial)
    }
}

/// A network adapter mirrors this committed local edit after the transaction.
#[derive(Clone, Copy, Debug)]
pub enum EdgeChange {
    Upsert(f64),
    Retract,
}
#[derive(Clone, Debug)]
pub struct EdgeMirror {
    pub from: String,
    pub to: String,
    pub edge_type: EdgeType,
    pub change: EdgeChange,
}
