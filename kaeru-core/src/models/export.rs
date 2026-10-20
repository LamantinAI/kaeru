//! Graph snapshot and Markdown export contracts.

use std::path::PathBuf;

use serde::Serialize;

/// Options controlling scope and sanitization of the export.
///
/// The allow-list is the authoritative ceiling. `restrict_initiatives` can only
/// **narrow within** it (intersection) — it can never widen the set — so an
/// untrusted request param cannot bypass the operator's configured scope.
#[derive(Debug, Clone, Default)]
pub struct ExportOpts {
    /// When `Some`, only nodes attached to a matching initiative are exported
    /// (`*` suffix = prefix glob, e.g. `"hi3516*"`). `Some(empty)` exports
    /// nothing; `None` = every initiative (trusted callers only).
    pub allow_initiatives: Option<Vec<String>>,
    /// Optional *additional* filter ANDed with `allow_initiatives` — a node's
    /// initiative must match this too. Used to narrow within the allow ceiling
    /// (e.g. a per-request `?initiatives=`); it can never expand the set.
    pub restrict_initiatives: Option<Vec<String>>,
    /// Initiatives to exclude even if they match the allow list.
    pub deny_initiatives: Vec<String>,
    /// Export only `visibility = shared` nodes (the local/shared contract:
    /// `local` nodes never leave the machine). Off = include `local` too.
    pub shared_only: bool,
    /// Include the **full** body (else a short excerpt). Redaction still applies.
    pub include_bodies: bool,
    /// Run the public secret/credential guard and redact flagged nodes.
    pub redact: bool,
}

/// The full graph, ready to `serde_json::to_string`.
#[derive(Debug, Clone, Serialize)]
pub struct GraphExport {
    pub meta: GraphMeta,
    pub initiatives: Vec<InitiativeStat>,
    pub nodes: Vec<NodeExport>,
    pub edges: Vec<EdgeExport>,
    pub chains: Vec<ChainExport>,
    /// Derived project-to-project affinity from shared `topic:` tags (the
    /// relationships exist in content but were never captured as edges).
    pub project_links: Vec<ProjectLink>,
}

/// A derived relatedness link between two initiatives, weight normalized to
/// `0..1` (strongest = 1). Computed from shared `topic:` tags, inverse-frequency
/// weighted so specific topics dominate over generic words.
#[derive(Debug, Clone, Serialize)]
pub struct ProjectLink {
    pub a: String,
    pub b: String,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphMeta {
    pub node_count: usize,
    pub edge_count: usize,
    pub initiative_count: usize,
    pub chain_count: usize,
    pub redacted_count: usize,
    /// Earliest / latest node creation second (for the time-lapse axis).
    pub earliest_secs: Option<f64>,
    pub latest_secs: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InitiativeStat {
    pub name: String,
    pub node_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct NodeExport {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub tier: String,
    pub layer: String,
    pub visibility: String,
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    pub initiatives: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_secs: Option<f64>,
    pub degree: usize,
    pub redacted: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct EdgeExport {
    pub src: String,
    pub dst: String,
    #[serde(rename = "type")]
    pub edge_type: String,
    pub weight: f64,
    pub dst_store: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_secs: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChainExport {
    pub id: String,
    pub name: String,
    /// Member node ids in trail order (filtered to exported nodes).
    pub members: Vec<String>,
}

/// Summary of a successful [`export_vault`] run.
#[derive(Debug, Clone)]
pub struct ExportSummary {
    /// The initiative the export was scoped to, or `None` for a
    /// cross-initiative dump.
    pub initiative: Option<String>,
    /// Root directory the markdown files were written under.
    pub root: PathBuf,
    /// Number of node markdown files written.
    pub nodes_exported: usize,
    /// Number of edges materialised as wikilinks (counted from the
    /// outgoing side; incoming refs use the same edge rows).
    pub edges_exported: usize,
}
