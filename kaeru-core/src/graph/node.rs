//! Node types and the value-object used by curator primitives.
//!
//! Two-tier taxonomy:
//! - operational-only: Task, Checklist, Roadmap, Experiment, Hypothesis,
//!   Scratch, Draft, Episode, AuditEvent.
//! - archival-only: Idea, Outcome, Reference.
//! - both (semantics distinguished by `tier`): Concept, Entity, Summary.

use std::str::FromStr;

use serde::{Deserialize, Serialize};
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

impl FromStr for Tier {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "operational" => Ok(Tier::Operational),
            "archival" => Ok(Tier::Archival),
            _ => Err(Error::Invalid(format!("unknown tier: {s}"))),
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
    /// A materialized "knowledge chain" — an ordered reasoning path saved as
    /// a node; members live in the `chain_member` relation.
    Chain,
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
            NodeType::Chain => "chain",
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
            "chain" => Ok(NodeType::Chain),
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

/// Memory layer — controls priority during context injection.
///
/// When an agent recalls, nodes are returned grouped by layer in order:
/// `Core` → `Hot` → `Warm` → `Cold` → `Frozen`. This lets the agent
/// (or a host application) fill the context window starting with the
/// most critical memories.
///
/// Layers are orthogonal to `Tier` (operational/archival) and
/// `Significance` (low/medium/high). A `Core` layer node might be an
/// archival `Idea` that the user marked as always-relevant; a `Hot`
/// layer might be a recent `Episode` the agent auto-promoted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Layer {
    /// Always injected into context — system-level knowledge,
    /// user preferences, standing instructions.
    Core,
    /// Frequently needed — injected first after Core.
    Hot,
    /// Relevant, default for most nodes.
    Warm,
    /// Archived — only retrieved on explicit recall.
    Cold,
    /// Forgotten but retrievable — stored, not surfaced by default.
    Frozen,
}

impl Layer {
    pub fn as_str(&self) -> &'static str {
        match self {
            Layer::Core => "core",
            Layer::Hot => "hot",
            Layer::Warm => "warm",
            Layer::Cold => "cold",
            Layer::Frozen => "frozen",
        }
    }
}

impl Default for Layer {
    fn default() -> Self {
        Layer::Warm
    }
}

impl FromStr for Layer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "core" => Ok(Layer::Core),
            "hot" => Ok(Layer::Hot),
            "warm" => Ok(Layer::Warm),
            "cold" => Ok(Layer::Cold),
            "frozen" => Ok(Layer::Frozen),
            _ => Err(Error::Invalid(format!("unknown layer: {s}"))),
        }
    }
}

/// Visibility — controls whether a node may leave the local store for the
/// shared cloud. Orthogonal to `Tier`, `Layer`, and `Significance`.
///
/// `Local` is the default and a hard floor: a `Local` node never syncs,
/// regardless of its initiative's `SharePolicy`. Promotion `Local →
/// Shared` is meant to be an explicit human act, never an automatic agent
/// decision. Same-id rewrites (`improve`, `complete_task`, status
/// transitions) preserve an existing `Shared` — the human already made
/// that call, and losing the flag silently orphaned nodes from the cloud —
/// while brand-new ids (successors from `supersedes` / `consolidate`)
/// still start `Local`, so nothing reaches the cloud without an explicit
/// `share`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    /// Never leaves the local store. Default. Personal configs, scratch,
    /// secrets — anything not deliberately shared.
    Local,
    /// Eligible for the shared cloud — still gated by the initiative's
    /// `SharePolicy` and the pre-share guard before it actually syncs.
    Shared,
}

impl Visibility {
    pub fn as_str(&self) -> &'static str {
        match self {
            Visibility::Local => "local",
            Visibility::Shared => "shared",
        }
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Local
    }
}

impl FromStr for Visibility {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Visibility::Local),
            "shared" => Ok(Visibility::Shared),
            _ => Err(Error::Invalid(format!("unknown visibility: {s}"))),
        }
    }
}
