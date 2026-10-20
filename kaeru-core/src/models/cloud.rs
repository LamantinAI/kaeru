//! Shared cloud input and edge-import contracts.

use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CloudRecallInput {
    pub initiative: String,
    #[serde(default)]
    pub cloud: Option<String>,
    /// Read one full node without importing it; omit for discovery.
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub query: Option<String>,
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub offset: Option<usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CloudPullInput {
    pub id: String,
    pub initiative: String,
    #[serde(default)]
    pub cloud: Option<String>,
}

/// Outcome of a node-local edge import. Existing local decisions win.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct EdgeImportResult {
    pub linked: usize,
    pub kept: usize,
}

/// The cloud representation of an edge; unknown future edge types are skipped.
pub type CloudEdge = (String, String, String, f64);
