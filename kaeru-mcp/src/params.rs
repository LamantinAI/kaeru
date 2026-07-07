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
pub struct PolicyParams {
    /// Initiative whose cloud sharing policy to read or set.
    pub initiative: String,
    /// New policy: `private` (default, never leaves), `team` (shared nodes
    /// may sync), or `ask`. Omit to read the current policy.
    #[serde(default)]
    pub policy: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ShareParams {
    /// Node name or UUIDv7 id to share to the team cloud.
    pub name: String,
    /// Initiative scope — required; sharing is gated by its `share_policy`.
    pub initiative: String,
    /// Override the pre-share secret guard when it flags content. Default false.
    #[serde(default)]
    pub force: bool,
    /// Target cloud name in a multi-cloud setup. Omit for the default cloud.
    #[serde(default)]
    pub cloud: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PullParams {
    /// UUIDv7 id of the cloud node to materialise into the local vault.
    pub id: String,
    /// Initiative to attach the pulled node to locally.
    pub initiative: String,
    /// Source cloud name in a multi-cloud setup. Omit for the default cloud.
    #[serde(default)]
    pub cloud: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CloudRecallParams {
    /// Initiative to list shared cloud nodes for.
    pub initiative: String,
    /// Cloud name to query in a multi-cloud setup. Omit for the default cloud.
    #[serde(default)]
    pub cloud: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LinkCloudParams {
    /// Local node name or UUIDv7 id to soft-link from.
    pub name: String,
    /// UUIDv7 id of the cloud node to link to.
    pub cloud_id: String,
    /// Edge type for the soft link. Defaults to `refers_to`.
    #[serde(default)]
    pub edge_type: Option<String>,
    /// Cloud the dst lives in (multi-cloud). Omit for the default cloud — the
    /// soft link records the name so resolution routes to the right endpoint.
    #[serde(default)]
    pub cloud: Option<String>,
    /// Initiative scope (both sides share the same initiative name).
    pub initiative: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CloudLinksParams {
    /// Local node name or UUIDv7 id whose cloud soft links to resolve.
    pub name: String,
    pub initiative: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SyncReviewParams {
    /// Team initiative to review still-local nodes for.
    pub initiative: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RenameInitiativeParams {
    /// Current initiative name.
    pub old: String,
    /// New initiative name (must not already exist).
    pub new: String,
    /// Also rename it in the shared cloud — team-wide, affects everyone.
    /// Default false (local only).
    #[serde(default)]
    pub cloud: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteInitiativeParams {
    /// Initiative to delete.
    pub name: String,
    /// Also delete it from the shared cloud — team-wide, removes it for
    /// everyone. Default false (local only).
    #[serde(default)]
    pub cloud: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AttachParams {
    /// Node name or UUIDv7 id to attach. Resolved across all initiatives, so
    /// the node may currently live under a different one.
    pub node: String,
    /// Target initiative to add the node to. Additive — the node keeps every
    /// initiative it already belongs to.
    pub to: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScopeOnly {
    /// Optional initiative to scope the operation to. When omitted,
    /// reads are cross-initiative; mutations end up un-tagged.
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SurfaceParams {
    /// Comma/space-separated memory layers to surface, e.g. `cold,frozen`
    /// or `cold`. Defaults to `cold,frozen` when omitted.
    #[serde(default)]
    pub layers: Option<String>,
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
pub struct LayerParams {
    /// Node name or UUIDv7 id.
    pub name: String,
    /// Target memory layer: `core`, `hot`, `warm`, `cold`, or `frozen`.
    pub layer: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EpisodeParams {
    /// Short, recallable name.
    pub name: String,
    /// Free-form body.
    pub body: String,
    /// Optional memory layer stamped at creation: `core`, `hot`, `warm`,
    /// `cold`, or `frozen`. Defaults to `warm`.
    #[serde(default)]
    pub layer: Option<String>,
    /// Optional visibility. `shared` marks team knowledge and — in a `team`
    /// initiative with the secret guard clear — pushes it to the cloud in
    /// this one call. Defaults to `local` (stays private).
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct JotParams {
    /// Free-form body. Name is auto-derived from first words + id suffix.
    pub body: String,
    /// Optional memory layer stamped at creation: `core`, `hot`, `warm`,
    /// `cold`, or `frozen`. Defaults to `warm`.
    #[serde(default)]
    pub layer: Option<String>,
    /// Optional visibility. `shared` marks team knowledge and — in a `team`
    /// initiative with the secret guard clear — pushes it to the cloud in
    /// this one call. Defaults to `local` (stays private).
    #[serde(default)]
    pub visibility: Option<String>,
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
    /// Connection strength `0..1` — drives knowledge-chain shortest-paths.
    /// Omit for a neutral link (0.5); `strong=true` makes it 1.0.
    #[serde(default)]
    pub weight: Option<f64>,
    /// Mark this as a key reasoning link (weight 1.0). Overridden by an
    /// explicit `weight`.
    #[serde(default)]
    pub strong: bool,
    #[serde(default)]
    pub initiative: Option<String>,
}

fn default_edge_type() -> String {
    "refers_to".to_string()
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReweightParams {
    /// Source node name or id.
    pub from: String,
    /// Destination node name or id.
    pub to: String,
    /// Edge type (default `refers_to`).
    #[serde(default = "default_edge_type")]
    pub edge_type: String,
    /// New connection strength in `0..1` (1 = strong → shorter chain paths).
    pub weight: f64,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ChainParams {
    /// Start node name or UUIDv7 id.
    pub from: String,
    /// End node name or UUIDv7 id.
    pub to: String,
    /// Optional name for the saved chain (auto-derived from endpoints if omitted).
    #[serde(default)]
    pub name: Option<String>,
    /// Optional one-line summary of why this trail matters — having traced the
    /// path, say what it captures. Becomes the chain's body so `chains` can be
    /// triaged by name + summary without reading every trail. Auto-derived if
    /// omitted.
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RechainParams {
    /// Chain name or UUIDv7 id to refresh.
    pub chain: String,
    /// Omit to regenerate (recompute the shortest path between the chain's
    /// current endpoints). Provide a node name/id to instead extend the trail
    /// out to it.
    #[serde(default)]
    pub to: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ChainsParams {
    /// Node name or id whose chains to list.
    pub name: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadChainParams {
    /// Chain name or UUIDv7 id.
    pub name: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PathParams {
    /// Start node name or id.
    pub from: String,
    /// End node name or id.
    pub to: String,
    #[serde(default)]
    pub initiative: Option<String>,
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
    /// Node name or UUIDv7 id. With `when` set, it resolves as of that moment,
    /// so a node retracted since then is still reachable — by id, or by the
    /// name it carried at that time.
    pub name: String,
    /// Optional moment to time-travel to — Unix seconds, RFC-3339
    /// (`2026-05-06T12:00:00Z`), or duration suffix (`5m`, `2h`, `3d` =
    /// "ago"). Omit to read the node as it is NOW.
    #[serde(default)]
    pub when: Option<String>,
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
    /// Optional memory layer stamped at creation: `core`, `hot`, `warm`,
    /// `cold`, or `frozen`. Defaults to `warm`.
    #[serde(default)]
    pub layer: Option<String>,
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
    /// Optional memory layer stamped at creation: `core`, `hot`, `warm`,
    /// `cold`, or `frozen`. Defaults to `warm`.
    #[serde(default)]
    pub layer: Option<String>,
    /// Optional visibility. `shared` marks team knowledge and — in a `team`
    /// initiative with the secret guard clear — pushes it to the cloud in
    /// this one call. Defaults to `local` (stays private).
    #[serde(default)]
    pub visibility: Option<String>,
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
    /// Optional memory layer stamped at creation: `core`, `hot`, `warm`,
    /// `cold`, or `frozen`. Defaults to `warm`.
    #[serde(default)]
    pub layer: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}
