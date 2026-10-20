//! Cloud initiatives exchange contracts.

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// One row of the initiative listing: the name plus a live (at NOW,
/// audit-events excluded) node count.
#[derive(Debug, Serialize, ToSchema)]
pub struct InitiativeBrief {
    pub name: String,
    pub nodes: usize,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RenameReq {
    pub new: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RenameResp {
    pub nodes: usize,
    pub edges: usize,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DeleteResp {
    pub unscoped: usize,
    pub forgotten: usize,
}

/// Compact node view for discovery listings.
#[derive(Debug, Serialize, ToSchema)]
pub struct NodeBriefView {
    pub id: String,
    pub node_type: String,
    pub name: String,
    pub body_excerpt: Option<String>,
}

/// `?limit=&offset=` for a node listing. Both optional; out-of-range values
/// clamp rather than error, since a caller asking for more than the ceiling
/// wants as much as it can have, not a rejection.
#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct PageQuery {
    /// Page size. Default 50, ceiling 500; out-of-range values clamp.
    #[serde(default)]
    pub(crate) limit: Option<usize>,
    /// How many to skip, for the next page.
    #[serde(default)]
    pub(crate) offset: Option<usize>,
    /// Case-insensitive substring over name and excerpt.
    ///
    /// The cloud had listing and get-by-id and no way to *look* for anything,
    /// so an agent whose whole reflex is `search` could not reach the second
    /// tier with it at all — it had to page through an initiative or already
    /// know an id (#57).
    #[serde(default)]
    pub(crate) q: Option<String>,
}
