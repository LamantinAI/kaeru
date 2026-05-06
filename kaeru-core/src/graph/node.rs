//! Node types and the value-object used by curator primitives.
//!
//! Two-tier taxonomy:
//! - operational-only: Task, Checklist, Roadmap, Experiment, Hypothesis,
//!   Scratch, Draft, Episode, AuditEvent.
//! - archival-only: Idea, Outcome, Reference.
//! - both (semantics distinguished by `tier`): Concept, Entity, Summary.

use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;
use uuid::Uuid;

use crate::errors::Error;

/// Stable identifier for a node. UUIDv7 — temporally-ordered, immutable on rename.
pub type NodeId = String;

/// Generates a new UUIDv7 as a string.
pub fn new_node_id() -> NodeId {
    Uuid::now_v7().to_string()
}

/// Tier — operational (cognitive working surface) or archival (settled recollection).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    Operational,
    Archival,
}

impl Tier {
    pub fn as_str(&self) -> &'static str {
        match self {
            Tier::Operational => "operational",
            Tier::Archival => "archival",
        }
    }
}

/// Node type — full taxonomy from the spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    // Operational
    Task,
    Checklist,
    Roadmap,
    Experiment,
    Hypothesis,
    Scratch,
    Draft,
    Episode,
    AuditEvent,
    // Archival
    Idea,
    Outcome,
    Reference,
    // Both tiers
    Concept,
    Entity,
    Summary,
}

impl NodeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            NodeType::Task => "task",
            NodeType::Checklist => "checklist",
            NodeType::Roadmap => "roadmap",
            NodeType::Experiment => "experiment",
            NodeType::Hypothesis => "hypothesis",
            NodeType::Scratch => "scratch",
            NodeType::Draft => "draft",
            NodeType::Episode => "episode",
            NodeType::AuditEvent => "audit_event",
            NodeType::Idea => "idea",
            NodeType::Outcome => "outcome",
            NodeType::Reference => "reference",
            NodeType::Concept => "concept",
            NodeType::Entity => "entity",
            NodeType::Summary => "summary",
        }
    }

    /// Default tier for a `NodeType` when the caller doesn't pin it
    /// explicitly. Operational-only types pin to `Operational`,
    /// archival-only types pin to `Archival`. Dual-tier types
    /// (`Concept`, `Entity`, `Summary`) default to `Operational` —
    /// promote with `consolidate_out` once they settle.
    pub fn default_tier(&self) -> Tier {
        match self {
            NodeType::Idea | NodeType::Outcome | NodeType::Reference => Tier::Archival,
            _ => Tier::Operational,
        }
    }
}

impl FromStr for NodeType {
    type Err = Error;

    /// Parses a `NodeType` from `as_str()` (snake_case) or kebab-case
    /// alias (`audit-event` ↔ `audit_event`).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.replace('-', "_").to_lowercase();
        match normalized.as_str() {
            "task" => Ok(NodeType::Task),
            "checklist" => Ok(NodeType::Checklist),
            "roadmap" => Ok(NodeType::Roadmap),
            "experiment" => Ok(NodeType::Experiment),
            "hypothesis" => Ok(NodeType::Hypothesis),
            "scratch" => Ok(NodeType::Scratch),
            "draft" => Ok(NodeType::Draft),
            "episode" => Ok(NodeType::Episode),
            "audit_event" => Ok(NodeType::AuditEvent),
            "idea" => Ok(NodeType::Idea),
            "outcome" => Ok(NodeType::Outcome),
            "reference" => Ok(NodeType::Reference),
            "concept" => Ok(NodeType::Concept),
            "entity" => Ok(NodeType::Entity),
            "summary" => Ok(NodeType::Summary),
            _ => Err(Error::Invalid(format!("unknown node type: {s}"))),
        }
    }
}

/// Episode kind — describes the nature of an event captured as an episode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EpisodeKind {
    Chat,
    Observation,
    Action,
    Decision,
}

impl EpisodeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            EpisodeKind::Chat => "chat",
            EpisodeKind::Observation => "observation",
            EpisodeKind::Action => "action",
            EpisodeKind::Decision => "decision",
        }
    }
}

/// Hypothesis status — open by default; updated as experiments report
/// verifies / falsifies / inconclusive verdicts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HypothesisStatus {
    Open,
    Supported,
    Refuted,
    Inconclusive,
}

impl HypothesisStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            HypothesisStatus::Open => "open",
            HypothesisStatus::Supported => "supported",
            HypothesisStatus::Refuted => "refuted",
            HypothesisStatus::Inconclusive => "inconclusive",
        }
    }
}

/// Episode significance — orthogonal to kind; how important for downstream reasoning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Significance {
    Low,
    Medium,
    High,
}

impl Significance {
    pub fn as_str(&self) -> &'static str {
        match self {
            Significance::Low => "low",
            Significance::Medium => "medium",
            Significance::High => "high",
        }
    }
}
