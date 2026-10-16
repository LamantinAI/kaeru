//! Cloud edges exchange contracts.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// An edge pushed up from a local vault. `src` / `dst` are the (preserved)
/// node UUIDv7s — both must already be shared so the edge resolves.
#[derive(Debug, Deserialize, ToSchema)]
pub struct EdgeIngestReq {
    pub src: String,
    pub dst: String,
    pub edge_type: String,
    /// Connection strength in `[0, 1]`. Carried so the cloud preserves it
    /// across share / pull; re-posting an edge with a new weight is also the
    /// cloud-side edit handle. Defaults to `1.0` when omitted.
    #[serde(default = "default_weight")]
    #[schema(default = 1.0, minimum = 0.0, maximum = 1.0)]
    pub weight: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EdgeView {
    pub src: String,
    pub dst: String,
    pub edge_type: String,
    pub weight: f64,
}

/// Which edge to retract. The triple is the edge's identity — the `edge`
/// relation is keyed by `{src, dst, edge_type, validity}`, so nothing else is
/// needed to name one.
#[derive(Debug, Deserialize, ToSchema)]
pub struct EdgeRetractReq {
    pub src: String,
    pub dst: String,
    pub edge_type: String,
}

fn default_weight() -> f64 {
    1.0
}
