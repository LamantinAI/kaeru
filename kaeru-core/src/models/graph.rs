//! Compact structural reads. Record bodies and mutation revisions stay in read.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct GraphNode {
    pub id: String,
    pub name: String,
    pub node_type: String,
    pub tier: String,
    pub layer: String,
    pub tags: Vec<String>,
    pub initiatives: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub edge_type: String,
    pub weight: f64,
}

/// Positions remain original: omitted private members never splice a trail.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct GraphMember {
    pub chain_id: String,
    pub node_id: String,
    pub position: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct GraphCounts {
    pub nodes: usize,
    pub edges: usize,
    pub chains: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct GraphSnapshot {
    pub snapshot_id: String,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub members: Vec<GraphMember>,
    pub counts: GraphCounts,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct GraphData {
    pub graph: GraphSnapshot,
}
