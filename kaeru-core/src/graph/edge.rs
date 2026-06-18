//! Edge types — typed edges with operational semantics.
//!
//! Each edge type is something the curator API responds to. `derived_from`
//! powers provenance and explainability; `contradicts` triggers a non-destructive
//! `under_review` flow; `supersedes` retracts the previous version through the
//! bi-temporal substrate; etc. Edges are not just associations.

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::errors::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeType {
    DerivedFrom,
    RefersTo,
    Supersedes,
    Causal,
    Temporal,
    Contradicts,
    PartOf,
    Blocks,
    Targets,
    Verifies,
    Falsifies,
    ConsolidatedTo,
}

impl EdgeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EdgeType::DerivedFrom => "derived_from",
            EdgeType::RefersTo => "refers_to",
            EdgeType::Supersedes => "supersedes",
            EdgeType::Causal => "causal",
            EdgeType::Temporal => "temporal",
            EdgeType::Contradicts => "contradicts",
            EdgeType::PartOf => "part_of",
            EdgeType::Blocks => "blocks",
            EdgeType::Targets => "targets",
            EdgeType::Verifies => "verifies",
            EdgeType::Falsifies => "falsifies",
            EdgeType::ConsolidatedTo => "consolidated_to",
        }
    }
}

impl FromStr for EdgeType {
    type Err = Error;

    /// Parses an `EdgeType` from its `as_str()` form (snake_case) or
    /// the kebab-case alias (`derived-from`, `refers-to`, …) — both
    /// forms feel natural at a CLI flag.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.replace('-', "_").to_lowercase();
        match normalized.as_str() {
            "derived_from" => Ok(EdgeType::DerivedFrom),
            "refers_to" => Ok(EdgeType::RefersTo),
            "supersedes" => Ok(EdgeType::Supersedes),
            "causal" => Ok(EdgeType::Causal),
            "temporal" => Ok(EdgeType::Temporal),
            "contradicts" => Ok(EdgeType::Contradicts),
            "part_of" => Ok(EdgeType::PartOf),
            "blocks" => Ok(EdgeType::Blocks),
            "targets" => Ok(EdgeType::Targets),
            "verifies" => Ok(EdgeType::Verifies),
            "falsifies" => Ok(EdgeType::Falsifies),
            "consolidated_to" => Ok(EdgeType::ConsolidatedTo),
            _ => Err(Error::Invalid(format!("unknown edge type: {s}"))),
        }
    }
}

/// Destination store of an edge — where the `dst` node lives.
///
/// `Local` (default) is a normal intra-store edge: `dst` is a node in the
/// same store. `Cloud` marks a **soft link** to a node in the shared cloud
/// store within the same initiative; `dst` is still a plain UUIDv7
/// (globally unique → resolves unambiguously in the cloud), looked up
/// lazily through the cloud API. An enum rather than a bool leaves room to
/// name *which* cloud once there is more than one, and indexes cleanly so
/// a local `walk` can prune `Cloud` edges and stay fully local.
///
/// Soft links are one-directional: only `local → cloud` exists. The cloud
/// never sees local ids, so it cannot reference them back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DstStore {
    Local,
    Cloud,
}

impl DstStore {
    pub fn as_str(&self) -> &'static str {
        match self {
            DstStore::Local => "local",
            DstStore::Cloud => "cloud",
        }
    }
}

impl Default for DstStore {
    fn default() -> Self {
        DstStore::Local
    }
}

impl FromStr for DstStore {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "local" => Ok(DstStore::Local),
            "cloud" => Ok(DstStore::Cloud),
            _ => Err(Error::Invalid(format!("unknown dst_store: {s}"))),
        }
    }
}
