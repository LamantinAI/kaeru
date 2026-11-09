//! Input models used by the cloud operation adapter.

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
    /// Target cloud name in a multi-cloud setup. Omit only when one cloud is configured.
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
pub struct LinkCloudParams {
    /// Local node name or UUIDv7 id to soft-link from.
    pub name: String,
    /// UUIDv7 id of the cloud node to link to.
    pub cloud_id: String,
    /// Edge type for the soft link — closed vocabulary, one of: `refers_to` (default), `causal`, `derived_from`, `contradicts`, `part_of`, `blocks`, `targets`, `supersedes`, `verifies`, `falsifies`, `temporal`, `consolidated_to`.
    #[serde(default)]
    pub edge_type: Option<String>,
    /// Cloud the dst lives in (multi-cloud). Omit when one cloud is configured; the
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

pub use kaeru_core::models::{CloudPullInput as PullParams, CloudRecallInput as CloudRecallParams};
