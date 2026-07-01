//! Recall primitives — the read side of the curator API.
//!
//! Submodules are split by primitive group; this `mod.rs` re-exports the
//! public surface and houses the shared `NodeBrief` type plus its
//! parsing helpers, which several submodules build on.

use cozo::DataValue;

use crate::graph::NodeId;
use crate::graph::temporal::validity_seconds;

pub mod between;
pub mod by_name;
pub mod fts;
pub mod initiatives;
pub mod layered;
pub mod lint;
pub mod overview;
pub mod path;
pub mod recent;
pub mod recollect;
pub mod reflect;
pub mod summary_view;
pub mod tagged;
pub mod under_review;
pub mod walk;

pub use between::{EdgeRow, between, cloud_links, edges_of};
pub use by_name::{
    count_by_type, local_nodes_for_review, node_brief_by_id, read_node_full, recall_id_by_name,
    recall_id_by_name_global,
};
pub use fts::{FUZZY_RECALL_LIMIT_CAP, fuzzy_recall};
pub use initiatives::{
    count_nodes_in_initiative, edges_in_initiative, list_initiatives, nodes_in_initiative,
};
pub use layered::{LayerBucket, recall_by_layer, recall_by_layer_in_tier};
pub use lint::{LintReport, lint};
pub use overview::overview;
pub use path::{chains_of, read_chain, shortest_path};
pub use recent::recent_episodes;
pub use recollect::{recollect_idea, recollect_outcome, recollect_provenance};
pub use reflect::{ReflectionReport, reflect};
pub use summary_view::{SummaryView, summary_view};
pub use tagged::tagged;
pub use under_review::under_review_pinned;
pub use walk::walk;

/// Compact handle on a node — id, type, name, and a truncated body
/// excerpt. Sized to be cheap for an LLM to read and decide whether to
/// drill down. Returned by `summary_view` and the `recollect_*` family.
#[derive(Debug, Clone, PartialEq)]
pub struct NodeBrief {
    pub id: NodeId,
    pub node_type: String,
    pub name: String,
    pub body_excerpt: Option<String>,
    /// Unix seconds of the node's latest assertion (created / last revised),
    /// for chronological display. `Some` when the producing query selects
    /// `validity` as its **last** column; `None` otherwise.
    pub ts: Option<f64>,
}

/// Full node record at NOW — every field a sharing / ingest path needs,
/// with the body **untruncated** (unlike `NodeBrief`'s excerpt). Returned
/// by `read_node_full`; the cloud adapter serialises this to push a shared
/// node and to materialise one on pull.
#[derive(Debug, Clone, PartialEq)]
pub struct NodeFull {
    pub id: NodeId,
    pub node_type: String,
    pub tier: String,
    pub name: String,
    pub body: Option<String>,
    pub tags: Vec<String>,
    pub visibility: String,
    /// Memory layer (`core`/`hot`/`warm`/`cold`/`frozen`) — carried through
    /// share/pull so a node keeps its recall priority across the cloud.
    pub layer: String,
}

/// Parses a Cozo result row of `[id, type, name, body, …, validity]` into a
/// `NodeBrief`, truncating `body` to `excerpt_chars` characters. The node's
/// `ts` is read from the row's **last** column when it carries a `validity`
/// (every brief query binds it there for ordering); rows without one yield
/// `ts: None`. Any other trailing columns are ignored.
pub(crate) fn parse_brief(row: &[DataValue], excerpt_chars: usize) -> NodeBrief {
    let id = row
        .first()
        .and_then(|v| v.get_str())
        .map(String::from)
        .unwrap_or_default();
    let node_type = row
        .get(1)
        .and_then(|v| v.get_str())
        .map(String::from)
        .unwrap_or_default();
    let name = row
        .get(2)
        .and_then(|v| v.get_str())
        .map(String::from)
        .unwrap_or_default();
    let body_excerpt = row
        .get(3)
        .and_then(|v| v.get_str())
        .map(|s| truncate_excerpt(s, excerpt_chars));
    // Brief queries bind `validity` as the last column (for `:order`); the
    // body lives at index 3, so a 4-column row has no validity and yields
    // `None` here. Reading `last()` keeps this agnostic to the column count
    // (fts adds a `score` column before validity, others don't).
    let ts = validity_seconds(row.last());
    NodeBrief {
        id,
        node_type,
        name,
        body_excerpt,
        ts,
    }
}

pub(crate) fn truncate_excerpt(s: &str, max_chars: usize) -> String {
    let count = s.chars().count();
    if count <= max_chars {
        s.to_string()
    } else {
        let head: String = s.chars().take(max_chars).collect();
        format!("{head}…")
    }
}
