//! Whole-graph JSON export for external visualizers.
//!
//! Unlike [`crate::export`] (an Obsidian markdown snapshot, per-initiative and
//! lossy on weight/validity), this assembles the **entire** substrate into one
//! serde-serializable [`GraphExport`]: every node with its type / tier / layer /
//! visibility / tags / initiatives / creation time, every edge with weight and
//! creation time, and the materialized knowledge chains. It is the data source
//! for `kaeru-viz` (a 3D force-graph) and is reachable read-only against the
//! live daemon's `Store`.
//!
//! Two safety knobs make it fit for a **public** export: an initiative
//! allow/deny filter (`*` suffix globs supported), and a per-node redaction
//! pass via [`crate::guard::scan_public`] that drops the body and replaces the
//! name of any node whose content trips the secret/credential scanner.

use std::collections::{HashMap, HashSet};

use cozo::{DataValue, Validity};
use serde::Serialize;

use crate::errors::Result;
use crate::guard;
use crate::store::Store;

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

/// Generic auto-derived topics that carry no cross-project signal.
const TOPIC_STOP: &[&str] = &[
    "topic:fix", "topic:test", "topic:build", "topic:first", "topic:after", "topic:phase",
    "topic:root", "topic:running", "topic:merged", "topic:master", "topic:earlier",
    "topic:correction", "topic:bug", "topic:finding", "topic:the", "topic:name", "topic:local",
    "topic:new", "topic:final", "topic:initial", "topic:via", "topic:not", "topic:and", "topic:---",
];

/// Inverse-frequency-weighted project affinity over exported nodes' `topic:`
/// tags; top 70 pairs, weight normalized to the strongest.
fn project_affinity(nodes: &[NodeExport]) -> Vec<ProjectLink> {
    let stop: HashSet<&str> = TOPIC_STOP.iter().copied().collect();
    let mut topic_inits: HashMap<&str, HashSet<&str>> = HashMap::new();
    for n in nodes {
        let Some(init) = n.initiatives.first() else { continue };
        for t in &n.tags {
            if t.starts_with("topic:") && !stop.contains(t.as_str()) {
                topic_inits.entry(t).or_default().insert(init);
            }
        }
    }
    let mut aff: HashMap<(&str, &str), f64> = HashMap::new();
    for set in topic_inits.values() {
        let span = set.len();
        if !(2..=12).contains(&span) {
            continue;
        }
        let w = 1.0 / span as f64;
        let mut v: Vec<&str> = set.iter().copied().collect();
        v.sort_unstable();
        for i in 0..v.len() {
            for j in (i + 1)..v.len() {
                *aff.entry((v[i], v[j])).or_insert(0.0) += w;
            }
        }
    }
    let mut pairs: Vec<((&str, &str), f64)> = aff.into_iter().collect();
    pairs.sort_by(|a, b| b.1.total_cmp(&a.1));
    pairs.truncate(70);
    let maxw = pairs.first().map(|p| p.1).unwrap_or(1.0);
    pairs
        .into_iter()
        .map(|((a, b), w)| ProjectLink {
            a: a.to_string(),
            b: b.to_string(),
            weight: w / maxw,
        })
        .collect()
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

/// Assembles the whole (scoped) graph into [`GraphExport`].
///
/// Read-only: issues only immutable `run_read` queries, so it is safe to call
/// against the live daemon. `current_initiative` does not affect the result —
/// every query is global and joins `node_initiative` explicitly for scoping.
///
/// Returns the whole result in one document — there is no pagination. That is
/// fine for a visualizer / demo; a very large vault may want a streaming or
/// paged variant before this is used as a general bulk-export API.
pub fn export_graph_json(store: &Store, opts: &ExportOpts) -> Result<GraphExport> {
    let excerpt = store.config().body_excerpt_chars;

    // node_id -> [initiative]
    let mut node_inits: HashMap<String, Vec<String>> = HashMap::new();
    for row in store
        .run_read("?[node_id, initiative] := *node_initiative{initiative, node_id}")?
        .rows
    {
        if let (Some(nid), Some(init)) = (str_at(&row, 0), str_at(&row, 1)) {
            node_inits.entry(nid).or_default().push(init);
        }
    }

    // node_id -> earliest asserted second (creation).
    let mut node_created: HashMap<String, f64> = HashMap::new();
    for row in store.run_read("?[id, validity] := *node{id, validity}")?.rows {
        if let (Some(id), Some((secs, true))) = (str_at(&row, 0), validity_at(&row, 1)) {
            node_created
                .entry(id)
                .and_modify(|m| *m = m.min(secs))
                .or_insert(secs);
        }
    }

    // edge "src|dst|type" -> earliest asserted second.
    let mut edge_created: HashMap<String, f64> = HashMap::new();
    for row in store
        .run_read("?[src, dst, edge_type, validity] := *edge{src, dst, edge_type, validity}")?
        .rows
    {
        if let (Some(s), Some(d), Some(t), Some((secs, true))) = (
            str_at(&row, 0),
            str_at(&row, 1),
            str_at(&row, 2),
            validity_at(&row, 3),
        ) {
            let key = format!("{s}|{d}|{t}");
            edge_created
                .entry(key)
                .and_modify(|m| *m = m.min(secs))
                .or_insert(secs);
        }
    }

    // Current nodes (at NOW), audit events excluded. Stash raw rows, decide
    // inclusion by the initiative filter.
    struct Raw {
        id: String,
        node_type: String,
        tier: String,
        name: String,
        body: Option<String>,
        tags: Vec<String>,
        visibility: String,
        layer: String,
        inits: Vec<String>,
    }
    let mut raws: Vec<Raw> = Vec::new();
    let mut included: HashSet<String> = HashSet::new();
    for row in store
        .run_read(
            "?[id, type, tier, name, body, tags, visibility, layer] := \
             *node{id, type, tier, name, body, tags, visibility, layer @ 'NOW'}, \
             type != 'audit_event'",
        )?
        .rows
    {
        let Some(id) = str_at(&row, 0) else { continue };
        let visibility = str_at(&row, 6).unwrap_or_else(|| "local".into());
        // Honour the local/shared contract: by default `local` nodes (which
        // "never leave the machine") are not exported.
        if opts.shared_only && visibility != "shared" {
            continue;
        }
        let inits = allowed_inits(node_inits.get(&id), opts);
        if inits.is_empty() {
            continue;
        }
        included.insert(id.clone());
        raws.push(Raw {
            node_type: str_at(&row, 1).unwrap_or_default(),
            tier: str_at(&row, 2).unwrap_or_default(),
            name: str_at(&row, 3).unwrap_or_default(),
            body: str_at(&row, 4),
            tags: list_at(&row, 5),
            visibility,
            layer: str_at(&row, 7).unwrap_or_else(|| "warm".into()),
            id,
            inits,
        });
    }

    // Edges: both endpoints must be exported.
    let mut edges: Vec<EdgeExport> = Vec::new();
    let mut degree: HashMap<String, usize> = HashMap::new();
    for row in store
        .run_read(
            "?[src, dst, edge_type, weight, dst_store] := \
             *edge{src, dst, edge_type, weight, dst_store @ 'NOW'}",
        )?
        .rows
    {
        let (Some(src), Some(dst), Some(edge_type)) =
            (str_at(&row, 0), str_at(&row, 1), str_at(&row, 2))
        else {
            continue;
        };
        if !included.contains(&src) || !included.contains(&dst) {
            continue;
        }
        *degree.entry(src.clone()).or_default() += 1;
        *degree.entry(dst.clone()).or_default() += 1;
        let key = format!("{src}|{dst}|{edge_type}");
        edges.push(EdgeExport {
            weight: float_at(&row, 3).unwrap_or(1.0),
            dst_store: str_at(&row, 4).unwrap_or_else(|| "local".into()),
            created_secs: edge_created.get(&key).copied(),
            src,
            dst,
            edge_type,
        });
    }

    // Build node exports with redaction.
    let mut redacted_count = 0usize;
    let mut init_counts: HashMap<String, usize> = HashMap::new();
    let nodes: Vec<NodeExport> = raws
        .into_iter()
        .map(|r| {
            for i in &r.inits {
                *init_counts.entry(i.clone()).or_default() += 1;
            }
            // Redact body-only when the body trips the public guard — names are
            // the visualizer's labels and stay readable. Only when the *name*
            // itself is sensitive do we genericize it too.
            let name_hit = opts.redact && !guard::scan_public(&r.name).is_empty();
            let body_hit = opts.redact
                && r.body
                    .as_deref()
                    .is_some_and(|b| !guard::scan_public(b).is_empty());
            let flagged = name_hit || body_hit;
            if flagged {
                redacted_count += 1;
            }
            let name = if name_hit {
                format!("\u{27e8}redacted {}\u{27e9}", r.node_type)
            } else {
                r.name
            };
            let body = if flagged {
                None
            } else {
                r.body.map(|b| {
                    if opts.include_bodies {
                        b
                    } else {
                        truncate(&b, excerpt)
                    }
                })
            };
            NodeExport {
                degree: degree.get(&r.id).copied().unwrap_or(0),
                created_secs: node_created.get(&r.id).copied(),
                id: r.id,
                name,
                node_type: r.node_type,
                tier: r.tier,
                layer: r.layer,
                visibility: r.visibility,
                tags: r.tags,
                body,
                initiatives: r.inits,
                redacted: flagged,
            }
        })
        .collect();

    // Chains: each `chain` node, its members filtered to exported nodes.
    let mut chains: Vec<ChainExport> = Vec::new();
    for n in nodes.iter().filter(|n| n.node_type == "chain") {
        let members: Vec<String> = crate::recall::read_chain(store, &n.id)?
            .into_iter()
            .map(|b| b.id)
            .filter(|id| included.contains(id))
            .collect();
        if members.len() >= 2 {
            chains.push(ChainExport {
                id: n.id.clone(),
                name: n.name.clone(),
                members,
            });
        }
    }

    let mut initiatives: Vec<InitiativeStat> = init_counts
        .into_iter()
        .map(|(name, node_count)| InitiativeStat { name, node_count })
        .collect();
    initiatives.sort_by(|a, b| b.node_count.cmp(&a.node_count).then(a.name.cmp(&b.name)));

    let earliest_secs = nodes.iter().filter_map(|n| n.created_secs).reduce(f64::min);
    let latest_secs = nodes.iter().filter_map(|n| n.created_secs).reduce(f64::max);

    let meta = GraphMeta {
        node_count: nodes.len(),
        edge_count: edges.len(),
        initiative_count: initiatives.len(),
        chain_count: chains.len(),
        redacted_count,
        earliest_secs,
        latest_secs,
    };

    let project_links = project_affinity(&nodes);

    Ok(GraphExport {
        meta,
        initiatives,
        nodes,
        edges,
        chains,
        project_links,
    })
}

// ── helpers ─────────────────────────────────────────────────────────────────

/// Filters a node's initiatives by the allow/deny options (`*`-suffix globs).
fn allowed_inits(inits: Option<&Vec<String>>, opts: &ExportOpts) -> Vec<String> {
    let Some(inits) = inits else {
        return Vec::new();
    };
    inits
        .iter()
        .filter(|i| {
            let allowed = match &opts.allow_initiatives {
                Some(pats) => pats.iter().any(|p| pat_match(i, p)),
                None => true,
            };
            // `restrict` only narrows — a node must match it too (if present).
            let within_restrict = match &opts.restrict_initiatives {
                Some(pats) => pats.iter().any(|p| pat_match(i, p)),
                None => true,
            };
            allowed && within_restrict && !opts.deny_initiatives.iter().any(|p| pat_match(i, p))
        })
        .cloned()
        .collect()
}

/// Exact match, or prefix match when `pattern` ends with `*`.
fn pat_match(name: &str, pattern: &str) -> bool {
    match pattern.strip_suffix('*') {
        Some(prefix) => name.starts_with(prefix),
        None => name == pattern,
    }
}

fn str_at(row: &[DataValue], i: usize) -> Option<String> {
    row.get(i).and_then(|v| v.get_str()).map(String::from)
}

fn float_at(row: &[DataValue], i: usize) -> Option<f64> {
    row.get(i).and_then(|v| v.get_float())
}

fn list_at(row: &[DataValue], i: usize) -> Vec<String> {
    match row.get(i) {
        Some(DataValue::List(items)) => items
            .iter()
            .filter_map(|x| x.get_str().map(String::from))
            .collect(),
        _ => Vec::new(),
    }
}

/// `(seconds, asserted)` from a `Validity` column, mirroring
/// `graph::temporal::parse_validity` but total (no error).
fn validity_at(row: &[DataValue], i: usize) -> Option<(f64, bool)> {
    match row.get(i) {
        Some(DataValue::Validity(Validity {
            timestamp,
            is_assert,
        })) => Some((timestamp.0.0 as f64, is_assert.0)),
        _ => None,
    }
}

/// Char-bounded excerpt with an ellipsis.
fn truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        return s.to_string();
    }
    let head: String = s.chars().take(max_chars).collect();
    format!("{head}…")
}

#[cfg(test)]
mod tests {
    use super::{ExportOpts, export_graph_json};
    use crate::store::Store;
    use crate::{EdgeType, EpisodeKind, NodeType, Significance, Visibility};
    use crate::{create_chain, link_with_weight, set_visibility, write_episode};

    #[test]
    fn export_filters_initiatives_and_redacts_secrets() {
        let store = Store::open_in_memory().expect("open");

        // alpha: two linked episodes + a chain.
        store.use_initiative("alpha");
        let a = write_episode(&store, EpisodeKind::Observation, Significance::Low, "a-node", "A")
            .unwrap();
        let b = write_episode(&store, EpisodeKind::Observation, Significance::Low, "b-node", "B")
            .unwrap();
        link_with_weight(&store, &a, &b, EdgeType::RefersTo, 1.0).unwrap();
        let chain = create_chain(&store, &a, &b, Some("a-to-b"))
            .unwrap()
            .expect("chain created");

        // alpha: a node that trips the guard → must be redacted.
        let secret = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "lab-default-creds",
            "the password=hunter2longvalue for the box",
        )
        .unwrap();

        // secretproj: a whole initiative excluded by the allow list.
        store.use_initiative("secretproj");
        write_episode(&store, EpisodeKind::Observation, Significance::Low, "hidden", "nope")
            .unwrap();

        let opts = ExportOpts {
            allow_initiatives: Some(vec!["alpha".into()]),
            redact: true,
            ..Default::default()
        };
        let g = export_graph_json(&store, &opts).expect("export");

        let names: Vec<&str> = g.nodes.iter().map(|n| n.name.as_str()).collect();
        assert!(names.contains(&"a-node") && names.contains(&"b-node"));
        assert!(!names.iter().any(|n| *n == "hidden"), "excluded initiative");

        // The secret node is present but redacted (body dropped, name replaced).
        let red = g.nodes.iter().find(|n| n.id == secret).expect("secret node present");
        assert!(red.redacted && red.body.is_none() && red.name.contains("redacted"));
        assert_eq!(g.meta.redacted_count, 1);

        // Edge a→b present with weight + creation time; chain present, ordered.
        assert!(g.edges.iter().any(|e| e.src == a && e.dst == b && e.weight == 1.0));
        assert!(g.nodes.iter().all(|n| n.created_secs.is_some()));
        let c = g.chains.iter().find(|c| c.id == chain).expect("chain exported");
        assert_eq!(c.members.first(), Some(&a));
        assert_eq!(c.members.last(), Some(&b));

        // Only alpha shows up in the initiative stats.
        assert_eq!(g.initiatives.len(), 1);
        assert_eq!(g.initiatives[0].name, "alpha");
        let _ = NodeType::Idea; // silence unused import if test trimmed
    }

    #[test]
    fn export_shared_only_and_restrict_narrow_scope() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("alpha");
        let _a = write_episode(&store, EpisodeKind::Observation, Significance::Low, "a", "A").unwrap();
        let b = write_episode(&store, EpisodeKind::Observation, Significance::Low, "b", "B").unwrap();
        set_visibility(&store, &b, Visibility::Shared).unwrap();
        store.use_initiative("beta");
        let _c = write_episode(&store, EpisodeKind::Observation, Significance::Low, "c", "C").unwrap();

        // shared_only → only the shared node `b`.
        let g = export_graph_json(&store, &ExportOpts { shared_only: true, ..Default::default() }).unwrap();
        let names: Vec<&str> = g.nodes.iter().map(|n| n.name.as_str()).collect();
        assert_eq!(names, vec!["b"], "shared_only excludes local nodes");

        // restrict narrows WITHIN allow: allow=[alpha,beta], restrict=[alpha] → alpha only.
        let g = export_graph_json(&store, &ExportOpts {
            allow_initiatives: Some(vec!["alpha".into(), "beta".into()]),
            restrict_initiatives: Some(vec!["alpha".into()]),
            ..Default::default()
        }).unwrap();
        let inits: std::collections::HashSet<&str> =
            g.nodes.iter().flat_map(|n| n.initiatives.iter().map(String::as_str)).collect();
        assert!(inits.contains("alpha") && !inits.contains("beta"));

        // restrict can NEVER widen past allow: allow=[alpha], restrict=[beta] → empty.
        let g = export_graph_json(&store, &ExportOpts {
            allow_initiatives: Some(vec!["alpha".into()]),
            restrict_initiatives: Some(vec!["beta".into()]),
            ..Default::default()
        }).unwrap();
        assert!(g.nodes.is_empty(), "restrict cannot expand the allow set");

        // Safe-empty: Some(empty) allow exports nothing.
        let g = export_graph_json(&store, &ExportOpts {
            allow_initiatives: Some(vec![]),
            ..Default::default()
        }).unwrap();
        assert!(g.nodes.is_empty(), "empty allow-list = safe-empty default");
    }
}
