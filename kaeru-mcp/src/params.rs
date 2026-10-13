//! Input DTOs for the nine retained cloud operations.

use schemars::JsonSchema;
use serde::Deserialize;

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
