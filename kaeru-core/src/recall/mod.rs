//! Recall primitives — the read side of the curator API.
//!
//! Submodules are split by primitive group; this `mod.rs` re-exports the
//! public surface and houses the shared `NodeBrief` type plus its
//! parsing helpers, which several submodules build on.

use cozo::DataValue;

use crate::graph::NodeId;

pub mod between;
pub mod by_name;
pub mod fts;
pub mod initiatives;
pub mod lint;
pub mod overview;
pub mod recent;
pub mod recollect;
pub mod summary_view;
pub mod tagged;
pub mod under_review;
pub mod walk;

pub use between::EdgeRow;
pub use between::between;
pub use by_name::count_by_type;
pub use by_name::node_brief_by_id;
pub use by_name::recall_id_by_name;
pub use fts::FUZZY_RECALL_LIMIT_CAP;
pub use fts::fuzzy_recall;
pub use initiatives::list_initiatives;
pub use lint::LintReport;
pub use lint::lint;
pub use overview::overview;
pub use recent::recent_episodes;
pub use recollect::recollect_idea;
pub use recollect::recollect_outcome;
pub use recollect::recollect_provenance;
pub use summary_view::SummaryView;
pub use summary_view::summary_view;
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
}

/// Parses a Cozo result row of `[id, type, name, body, ...]` into a
/// `NodeBrief`, truncating `body` to `excerpt_chars` characters. Extra
/// trailing columns (e.g. `validity` used for ordering) are ignored.
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
    NodeBrief {
        id,
        node_type,
        name,
        body_excerpt,
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
