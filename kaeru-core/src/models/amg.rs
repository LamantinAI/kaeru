//! Portable current-knowledge contracts; no substrate-specific rows or credentials.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AmgScope {
    All,
    Initiatives,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AmgSelection {
    All,
    Initiatives(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AmgSoftware {
    pub name: String,
    pub version: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AmgOmission {
    pub reason: String,
    pub count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AmgMetadata {
    pub schema_version: u32,
    pub exported_at: String,
    pub software: AmgSoftware,
    pub profile: String,
    pub scope: AmgScope,
    pub initiatives: Vec<String>,
    pub omissions: Vec<AmgOmission>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AmgOrigin {
    pub document_sha256: String,
    pub exported_at: String,
    pub software: AmgSoftware,
    pub recorded_at: Option<String>,
    pub created_at: Option<String>,
    #[serde(default)]
    pub source_visibility: Option<String>,
    #[serde(default)]
    pub source_origin: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AmgNode {
    pub id: String,
    pub node_type: String,
    pub tier: String,
    pub name: String,
    pub body: Option<String>,
    pub tags: Vec<String>,
    pub properties: Value,
    pub layer: String,
    pub visibility: String,
    pub initiatives: Vec<String>,
    pub recorded_at: Option<String>,
    pub created_at: Option<String>,
    pub origins: Vec<AmgOrigin>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AmgEdge {
    pub src: String,
    pub dst: String,
    pub edge_type: String,
    pub weight: f64,
    pub properties: Value,
    pub initiatives: Vec<String>,
    pub recorded_at: Option<String>,
    pub created_at: Option<String>,
    pub origins: Vec<AmgOrigin>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AmgTrail {
    pub id: String,
    pub members: Vec<String>,
    pub fingerprints: Option<BTreeMap<String, String>>,
    pub stale: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AmgSlot {
    pub initiative: String,
    pub slot: String,
    pub node: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AmgDecision {
    pub node: String,
    pub status: String,
    pub evidence: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AmgGraph {
    pub nodes: Vec<AmgNode>,
    pub edges: Vec<AmgEdge>,
    pub trails: Vec<AmgTrail>,
    pub slots: Vec<AmgSlot>,
    pub decisions: Vec<AmgDecision>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AmgDocument {
    pub metadata: AmgMetadata,
    pub data: AmgGraph,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct AmgImportReport {
    pub created_nodes: usize,
    pub updated_nodes: usize,
    pub unchanged_nodes: usize,
    pub created_edges: usize,
    pub updated_edges: usize,
    pub unchanged_edges: usize,
    pub updated_metadata: usize,
    pub dry_run: bool,
    pub force: bool,
    pub document_sha256: String,
}
