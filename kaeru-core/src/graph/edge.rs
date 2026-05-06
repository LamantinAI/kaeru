//! Edge types — typed edges with operational semantics.
//!
//! Each edge type is something the curator API responds to. `derived_from`
//! powers provenance and explainability; `contradicts` triggers a non-destructive
//! `under_review` flow; `supersedes` retracts the previous version through the
//! bi-temporal substrate; etc. Edges are not just associations.

use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;

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
