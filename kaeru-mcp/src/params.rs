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
    /// may sync), or `ask`. Omit to leave it as it is.
    #[serde(default)]
    pub policy: Option<String>,
    /// Restrict this initiative to named clouds — comma or space separated.
    /// `policy` says WHETHER an initiative may leave; this says WHERE TO. An
    /// initiative with no list may go to any configured cloud, which is how
    /// every initiative behaves until this is set. Pass an empty string to
    /// clear the restriction. Set independently of `policy`, so restricting
    /// does not re-open and re-opening does not un-restrict.
    #[serde(default)]
    pub clouds: Option<String>,
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
pub struct CloudScopeParams {
    /// Which cloud to ask. Required when several are configured.
    #[serde(default)]
    pub cloud: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UnshareParams {
    /// Node name or UUIDv7 id to withdraw from the cloud.
    pub name: String,
    /// Initiative scope — required, same as `share`.
    pub initiative: String,
    /// Which cloud to withdraw from. Required when several are configured.
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
    /// Cloud name to query in a multi-cloud setup. Required when several are
    /// configured.
    #[serde(default)]
    pub cloud: Option<String>,
    /// Search the cloud instead of listing it: a case-insensitive substring
    /// over shared node names and excerpts. Omit to list everything shared.
    #[serde(default)]
    pub query: Option<String>,
    /// Page size, default 25, ceiling 500. A shared initiative can hold
    /// hundreds of nodes — more than a context window — so this read is
    /// bounded like every other list on the surface.
    #[serde(default)]
    pub limit: Option<usize>,
    /// How many to skip, for the next page. The result says when there is one.
    #[serde(default)]
    pub offset: Option<usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LinkCloudParams {
    /// Local node name or UUIDv7 id to soft-link from.
    pub name: String,
    /// UUIDv7 id of the cloud node to link to.
    pub cloud_id: String,
    /// Edge type for the soft link — closed vocabulary, one of: `refers_to` (default), `causal`, `derived_from`, `contradicts`, `part_of`, `blocks`, `targets`, `supersedes`, `verifies`, `falsifies`, `temporal`, `consolidated_to`.
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
    /// Name of a cloud to ALSO rename this initiative in — team-wide, and it
    /// affects everyone using it. Omit for a local-only rename. Never
    /// defaulted: with several clouds configured, an unnamed cloud rename
    /// would be a guess at which team it disrupts.
    #[serde(default)]
    pub cloud: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteInitiativeParams {
    /// Initiative to delete.
    pub name: String,
    /// Name of a cloud to ALSO delete this initiative from — removes it for
    /// everyone, and the cloud has no undo. Omit for a local-only delete.
    /// Never defaulted.
    #[serde(default)]
    pub cloud: Option<String>,
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
pub struct BoardParams {
    /// Initiative whose board to show (required).
    #[serde(default)]
    pub initiative: Option<String>,
    /// Optional moment to rewind the board to (unix seconds, RFC-3339, or
    /// `5m` / `2h` ago). Omit for the board as it stands now.
    #[serde(default)]
    pub when: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetStatusParams {
    /// Task node name or id to move.
    pub task: String,
    /// Target status (board column key) — must exist in the initiative's board.
    pub status: String,
    /// Initiative whose board defines the valid statuses (required).
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BoardStatusParams {
    /// What to do: `add` / `remove` / `relabel` / `reorder`.
    pub action: String,
    /// Status key (stable id). Required for add / remove / relabel.
    #[serde(default)]
    pub key: Option<String>,
    /// Human label. Required for relabel; optional for add (defaults to key).
    #[serde(default)]
    pub label: Option<String>,
    /// Full ordered list of existing keys. Required for reorder.
    #[serde(default)]
    pub order: Option<Vec<String>>,
    /// Initiative whose board to customize (required).
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
    /// Which cloud `visibility=shared` publishes to. Only consulted when
    /// sharing. Required when several clouds are configured — the push is
    /// refused rather than sent to a default you did not name.
    #[serde(default)]
    pub cloud: Option<String>,
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
    /// Which cloud `visibility=shared` publishes to. Only consulted when
    /// sharing. Required when several clouds are configured — the push is
    /// refused rather than sent to a default you did not name.
    #[serde(default)]
    pub cloud: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LinkParams {
    /// Source node name or id — resolved in the active initiative first,
    /// then across all initiatives, so an edge may span initiatives.
    pub from: String,
    /// Destination node name or id — resolved in the active initiative
    /// first, then across all initiatives.
    pub to: String,
    /// Edge type — a CLOSED vocabulary, one of exactly these: `refers_to` (default), `causal`, `derived_from`, `contradicts`, `part_of`, `blocks`, `targets`, `supersedes`, `verifies`, `falsifies`, `temporal`, `consolidated_to`.
    /// Nothing else is accepted (`related_to` and friends are not edge types).
    /// Snake_case or kebab-case both accepted.
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
    /// Source node name or id — resolved in the active initiative first,
    /// then across all initiatives.
    pub from: String,
    /// Destination node name or id — resolved in the active initiative
    /// first, then across all initiatives.
    pub to: String,
    /// Edge type — closed vocabulary, one of: `refers_to` (default), `causal`, `derived_from`, `contradicts`, `part_of`, `blocks`, `targets`, `supersedes`, `verifies`, `falsifies`, `temporal`, `consolidated_to`.
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
pub struct WhyParams {
    /// A chain (read its steps) or any node (see the reasoning it belongs to).
    /// Name or UUIDv7 id.
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
    /// The verdict, when you ALREADY KNOW IT — the usual case, since you
    /// normally reach memory after the check has run. Omit for a genuinely
    /// open question. A CLOSED vocabulary, one of exactly these: `supported`,
    /// `refuted`, `inconclusive` (`confirmed` / `falsified` / `partial` are
    /// accepted as aliases).
    #[serde(default)]
    pub verdict: Option<String>,
    /// Evidence node (name or id) the verdict rests on — linked `verifies` /
    /// `falsifies`. Optional, but a verdict without one is a claim with no
    /// citation.
    #[serde(default)]
    pub by: Option<String>,
    /// Optional memory layer stamped at creation: `core`, `hot`, `warm`,
    /// `cold`, or `frozen`. Defaults to `warm`.
    #[serde(default)]
    pub layer: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EvidenceParams {
    /// Hypothesis name or id this evidence bears on.
    pub hypothesis: String,
    /// What you actually did and what came out of it — past tense. Creates
    /// the experiment node. Give this OR `node`.
    #[serde(default)]
    pub method: Option<String>,
    /// An existing node (name or id) to register as the evidence instead of
    /// writing a new one — an episode you already captured, say. Give this OR
    /// `method`.
    #[serde(default)]
    pub node: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct VerdictParams {
    /// Hypothesis name or id.
    pub hypothesis: String,
    /// Evidence node name or id — linked `verifies` for `confirm`,
    /// `falsifies` for `refute`. OPTIONAL: record the verdict even with
    /// nothing to point at yet, rather than leaving the claim tagged `open`
    /// with the answer buried in its prose. (`inconclusive` writes no edge at
    /// all, so it never needs one.)
    #[serde(default)]
    pub by: Option<String>,
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
pub struct CloseReviewParams {
    /// Target node name (or id) whose open review to close.
    pub target: String,
    /// Optional note on how it was settled — recorded as a resolution episode
    /// that supersedes the closed review. Omit for a bare close.
    #[serde(default)]
    pub resolution: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ConsolidateParams {
    /// Source node name (or id).
    pub source: String,
    /// Type for the successor. OPTIONAL — omit to promote in place, and the
    /// type is derived from the source (`episode`/`task`/`experiment`/
    /// `hypothesis` → `outcome`; `draft`/`scratch` → `idea`; anything already
    /// archival keeps itself). The chosen type is always printed back.
    /// A CLOSED vocabulary, one of exactly these: `episode`, `task`,
    /// `checklist`, `roadmap`, `experiment`, `hypothesis`, `scratch`, `draft`,
    /// `audit_event`, `chain`, `board`, `idea`, `outcome`, `reference`,
    /// `concept`, `entity`, `summary`.
    #[serde(default)]
    pub new_type: Option<String>,
    /// Name for the successor. OPTIONAL — omit to carry the source's name over
    /// unchanged.
    #[serde(default)]
    pub new_name: Option<String>,
    /// Body for the successor. OPTIONAL — omit to carry the source's body over
    /// unchanged (in full, not the excerpt).
    #[serde(default)]
    pub new_body: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SynthesiseParams {
    /// Seed node names.
    pub from: Vec<String>,
    /// Type of the synthesised node (defaults `summary`). A CLOSED
    /// vocabulary, one of exactly these: `episode`, `task`, `checklist`,
    /// `roadmap`, `experiment`, `hypothesis`, `scratch`, `draft`,
    /// `audit_event`, `chain`, `board`, `idea`, `outcome`, `reference`,
    /// `concept`, `entity`, `summary`.
    #[serde(default = "default_synth_type")]
    pub new_type: String,
    /// Name for the synthesised node.
    pub new_name: String,
    /// Body for the synthesised node.
    pub new_body: String,
    /// Tier override. A CLOSED vocabulary, one of exactly these:
    /// `operational`, `archival`. Defaults from the type. (Not to be confused
    /// with the memory *layer* — `core`/`hot`/`warm`/`cold`/`frozen` — which
    /// `layer` sets.)
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
    /// Type for the successor. OPTIONAL — omit to keep the old node's own
    /// type, which is what a straight replacement wants. A CLOSED vocabulary,
    /// one of exactly these: `episode`, `task`, `checklist`, `roadmap`,
    /// `experiment`, `hypothesis`, `scratch`, `draft`, `audit_event`, `chain`,
    /// `board`, `idea`, `outcome`, `reference`, `concept`, `entity`,
    /// `summary`.
    #[serde(default)]
    pub new_type: Option<String>,
    /// New node name.
    pub new_name: String,
    /// New node body.
    pub new_body: String,
    /// Tier override. A CLOSED vocabulary, one of exactly these:
    /// `operational`, `archival`. Defaults from the type. (Not the memory
    /// *layer* — that is `core`/`hot`/`warm`/`cold`/`frozen`, set by `layer`.)
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
    /// Which cloud `visibility=shared` publishes to. Only consulted when
    /// sharing. Required when several clouds are configured — the push is
    /// refused rather than sent to a default you did not name.
    #[serde(default)]
    pub cloud: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BetweenParams {
    /// First node name (also accepts a UUIDv7 id).
    pub a: String,
    /// Second node name (also accepts a UUIDv7 id).
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

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SlotParams {
    /// Initiative the role belongs to. Slots are per-initiative: the same
    /// role name in another initiative is a different slot.
    pub initiative: String,
    /// The role — `handoff`, `entrypoint`, `queue`, `prod-state`, or any
    /// name you keep to one live node.
    pub slot: String,
    /// Node to install as the holder. Name or UUIDv7 id.
    pub name: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SlotScope {
    /// Initiative whose slots to act on.
    pub initiative: String,
    /// The role name.
    pub slot: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct InitiativeOnly {
    /// Initiative to report on.
    pub initiative: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct HygieneParams {
    /// Initiative to report on, or to sweep when `force` is set.
    pub initiative: String,
    /// Run a pass now instead of reporting what one would do.
    #[serde(default)]
    pub force: bool,
}
