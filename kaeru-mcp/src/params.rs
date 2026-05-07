//! All `Parameters<T>` structs that the `#[tool]` methods deserialize.
//! Pulled out of `server.rs` so that file stays focused on tool
//! registration and dispatch.
//!
//! Reused shapes are deliberately reused (e.g. `NameScope`,
//! `ScopeOnly`); per-tool shapes get distinct names so the schemas an
//! agent's MCP client reads are self-explanatory.

use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScopeOnly {
    /// Optional initiative to scope the operation to. When omitted,
    /// reads are cross-initiative; mutations end up un-tagged.
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NameScope {
    /// Node name (also accepts a UUIDv7 id where the verb supports
    /// polymorphic resolution).
    pub name: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EpisodeParams {
    /// Short, recallable name.
    pub name: String,
    /// Free-form body.
    pub body: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct JotParams {
    /// Free-form body. Name is auto-derived from first words + id suffix.
    pub body: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LinkParams {
    /// Source node name.
    pub from: String,
    /// Destination node name.
    pub to: String,
    /// Edge type. Common values: `refers_to` (default), `causal`,
    /// `derived_from`, `contradicts`, `part_of`, `blocks`, `targets`,
    /// `supersedes`, `verifies`, `falsifies`, `temporal`,
    /// `consolidated_to`. Snake_case or kebab-case both accepted.
    #[serde(default = "default_edge_type")]
    pub edge_type: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

fn default_edge_type() -> String {
    "refers_to".to_string()
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PinParams {
    /// Node name or UUIDv7 id.
    pub name: String,
    /// Why the node deserves a place in the active window.
    pub reason: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RecentParams {
    /// Time window (e.g. `30m`, `3h`, `2d`, raw seconds). Defaults to 24h.
    #[serde(default = "default_recent_window")]
    pub since: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

fn default_recent_window() -> String {
    "24h".to_string()
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchParams {
    /// Cozo FTS query (`AND`/`OR`/`NOT`, `"phrase"`, `prefix*`).
    pub query: String,
    /// Maximum results. Capped at 50 internally.
    #[serde(default = "default_search_limit")]
    pub limit: usize,
    #[serde(default)]
    pub initiative: Option<String>,
}

fn default_search_limit() -> usize {
    10
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AtParams {
    /// Node name.
    pub name: String,
    /// Moment to query — Unix seconds, RFC-3339 (`2026-05-06T12:00:00Z`),
    /// or duration suffix (`5m`, `2h`, `3d` = "ago").
    pub when: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ClaimParams {
    /// The hypothesis text. Auto-named from first words + id suffix.
    pub text: String,
    /// Optional existing node this claim is about (refers_to edge).
    #[serde(default)]
    pub about: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TestParams {
    /// Hypothesis name.
    pub hypothesis: String,
    /// How the experiment was conducted.
    pub method: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct VerdictParams {
    /// Hypothesis name.
    pub hypothesis: String,
    /// Evidence node name (verifying for `confirm`, falsifying for `refute`).
    pub by: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FlagParams {
    /// Target node name to flag.
    pub target: String,
    /// Reason / description of the concern.
    pub reason: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ResolveParams {
    /// Question node name.
    pub question: String,
    /// Answer / resolution node name.
    pub by: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ConsolidateParams {
    /// Source node name.
    pub source: String,
    /// New node type (`idea`, `outcome`, `summary`, `draft`, …).
    pub new_type: String,
    /// New node name.
    pub new_name: String,
    /// New node body.
    pub new_body: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SynthesiseParams {
    /// Seed node names.
    pub from: Vec<String>,
    /// Type of the synthesised node (defaults `summary`).
    #[serde(default = "default_synth_type")]
    pub new_type: String,
    /// Name for the synthesised node.
    pub new_name: String,
    /// Body for the synthesised node.
    pub new_body: String,
    /// Tier override (`operational` / `archival`). Defaults from type.
    #[serde(default)]
    pub tier: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

fn default_synth_type() -> String {
    "summary".to_string()
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SupersedeParams {
    /// Old node name (or id).
    pub old: String,
    /// New node type.
    pub new_type: String,
    /// New node name.
    pub new_name: String,
    /// New node body.
    pub new_body: String,
    /// Tier override (defaults from new_type).
    #[serde(default)]
    pub tier: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReviseParams {
    /// Node name.
    pub name: String,
    /// New body. If omitted, keeps current.
    #[serde(default)]
    pub body: Option<String>,
    /// New name. If omitted, keeps current.
    #[serde(default)]
    pub rename: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CiteParams {
    /// Short, recallable name.
    pub name: String,
    /// Optional URL of the source. Skip for persona / entity records
    /// (a person, place, book without a link).
    #[serde(default)]
    pub url: Option<String>,
    /// One-paragraph summary — what's at the link, or who this entity is.
    pub body: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BetweenParams {
    /// First node name.
    pub a: String,
    /// Second node name.
    pub b: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TaggedParams {
    /// Tag value (case-sensitive).
    pub tag: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExportParams {
    /// Output directory.
    pub output_dir: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TaskParams {
    /// Free-form task description.
    pub body: String,
    /// Optional deadline. Accepts an ISO date (`2026-05-15`), an
    /// RFC-3339 datetime, or a future duration (`3d`, `2w`). Omit
    /// for tasks without a deadline.
    #[serde(default)]
    pub due: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}
