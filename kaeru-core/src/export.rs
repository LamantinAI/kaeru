//! Obsidian-friendly markdown export — snapshot of the substrate as a
//! directory of markdown pages with YAML frontmatter and `[[wikilink]]`
//! edges, readable as a regular Obsidian vault.
//!
//! This is a derived view: the substrate stays the source of truth.
//! `Validity` is intentionally absent from the frontmatter — bi-temporal
//! history doesn't survive a flat snapshot, and pretending otherwise
//! would mislead readers.
//!
//! Scope: when `Store.current_initiative()` is `Some`, the export
//! covers only that initiative's nodes and edges (both endpoints in
//! scope). Without an active initiative, the whole substrate is dumped.

use chrono::DateTime;
use chrono::Utc;
use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::errors::Result;
use crate::graph::temporal::parse_validity;
use crate::store::Store;

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

/// Snapshots the substrate at NOW into `output_dir` as a directory tree
/// of markdown files. Existing files in `output_dir` are overwritten.
///
/// Honours `Store.current_initiative()`: a scoped export only includes
/// nodes attached to that initiative and edges whose `src` and `dst`
/// are both in the initiative.
pub fn export_vault(store: &Store, output_dir: impl AsRef<Path>) -> Result<ExportSummary> {
    let root = output_dir.as_ref().to_path_buf();
    let initiative = store.current_initiative();

    fs::create_dir_all(&root)?;

    let nodes = read_nodes(store, initiative.as_deref())?;
    let edges = read_edges(store, initiative.as_deref())?;
    let initiatives_by_node = read_initiatives_by_node(store)?;

    let id_to_name = build_unique_names(&nodes);

    // Group edges so each node sees its outgoing and incoming neighbours
    // in O(1) during render. We dedupe (src, dst, edge_type) at this
    // point — Datalog set semantics make duplicates unlikely, but
    // multiple edge rows with the same key under bi-temporal validity
    // can happen.
    let mut outgoing: HashMap<String, Vec<&EdgeRow>> = HashMap::new();
    let mut incoming: HashMap<String, Vec<&EdgeRow>> = HashMap::new();
    let mut seen: HashSet<(&str, &str, &str)> = HashSet::new();
    for edge in &edges {
        let key = (edge.src.as_str(), edge.dst.as_str(), edge.edge_type.as_str());
        if !seen.insert(key) {
            continue;
        }
        outgoing.entry(edge.src.clone()).or_default().push(edge);
        incoming.entry(edge.dst.clone()).or_default().push(edge);
    }

    let mut nodes_count = 0;
    for node in &nodes {
        let path = node_path(&root, node, &id_to_name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let inits = initiatives_by_node.get(&node.id).cloned().unwrap_or_default();
        let content = render_node(
            node,
            &inits,
            outgoing.get(&node.id).map(|v| v.as_slice()).unwrap_or(&[]),
            incoming.get(&node.id).map(|v| v.as_slice()).unwrap_or(&[]),
            &id_to_name,
        );
        fs::write(&path, content)?;
        nodes_count += 1;
    }

    // llm-wiki framing: top-level README, INDEX, and LOG. Even with an
    // empty graph these files exist so the snapshot reads as a vault,
    // not a loose pile of pages. README captures provenance; INDEX is
    // the navigation surface; LOG is the chronological audit-event
    // stream filtered to operations on visible nodes.
    let exported_ids: HashSet<&str> = id_to_name.keys().map(|s| s.as_str()).collect();
    let audit_events = read_audit_events_touching(store, &exported_ids)?;
    let edges_count = seen.len();

    fs::write(
        root.join("README.md"),
        render_readme(&initiative, nodes_count, edges_count, audit_events.len()),
    )?;
    fs::write(
        root.join("INDEX.md"),
        render_index(&nodes, &id_to_name, &outgoing, &incoming),
    )?;
    fs::write(
        root.join("LOG.md"),
        render_log(&audit_events, &id_to_name),
    )?;

    Ok(ExportSummary {
        initiative,
        root,
        nodes_exported: nodes_count,
        edges_exported: edges_count,
    })
}

#[derive(Debug)]
struct NodeRow {
    id: String,
    node_type: String,
    tier: String,
    name: String,
    body: Option<String>,
    tags: Vec<String>,
}

#[derive(Debug)]
struct EdgeRow {
    src: String,
    dst: String,
    edge_type: String,
}

#[derive(Debug)]
struct AuditEvent {
    /// Unix seconds of the audit row's validity timestamp. Used for
    /// chronological ordering.
    seconds: f64,
    op: String,
    actor: String,
    /// Node ids the operation affected. May contain ids outside the
    /// export scope; those get filtered out at render time.
    affected_refs: Vec<String>,
}

fn read_nodes(store: &Store, initiative: Option<&str>) -> Result<Vec<NodeRow>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match initiative {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.to_string().into()));
            r#"
                ?[id, type, tier, name, body, tags] :=
                    *node{id, type, tier, name, body, tags @ 'NOW'},
                    type != 'audit_event',
                    *node_initiative{initiative, node_id: id},
                    initiative = $init
            "#
        }
        None => {
            r#"
                ?[id, type, tier, name, body, tags] :=
                    *node{id, type, tier, name, body, tags @ 'NOW'},
                    type != 'audit_event'
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let nodes = rows
        .rows
        .iter()
        .filter_map(|row| {
            let id = row.first().and_then(|v| v.get_str())?.to_string();
            let node_type = row.get(1).and_then(|v| v.get_str())?.to_string();
            let tier = row.get(2).and_then(|v| v.get_str())?.to_string();
            let name = row.get(3).and_then(|v| v.get_str())?.to_string();
            let body = row.get(4).and_then(|v| v.get_str()).map(String::from);
            let tags = row
                .get(5)
                .map(|v| match v {
                    DataValue::List(items) => items
                        .iter()
                        .filter_map(|x| x.get_str().map(String::from))
                        .collect(),
                    _ => Vec::new(),
                })
                .unwrap_or_default();
            Some(NodeRow {
                id,
                node_type,
                tier,
                name,
                body,
                tags,
            })
        })
        .collect();
    Ok(nodes)
}

fn read_edges(store: &Store, initiative: Option<&str>) -> Result<Vec<EdgeRow>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    // Filter both endpoints when scoped: prevents wikilinks pointing to
    // nodes outside the export.
    let script = match initiative {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.to_string().into()));
            r#"
                ?[src, dst, edge_type] := *edge{src, dst, edge_type @ 'NOW'},
                                           *node_initiative{initiative, node_id: src},
                                           initiative = $init,
                                           *node_initiative{initiative: i2, node_id: dst},
                                           i2 = $init
            "#
        }
        None => {
            r#"
                ?[src, dst, edge_type] := *edge{src, dst, edge_type @ 'NOW'}
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let edges = rows
        .rows
        .iter()
        .filter_map(|row| {
            let src = row.first().and_then(|v| v.get_str())?.to_string();
            let dst = row.get(1).and_then(|v| v.get_str())?.to_string();
            let edge_type = row.get(2).and_then(|v| v.get_str())?.to_string();
            Some(EdgeRow { src, dst, edge_type })
        })
        .collect();
    Ok(edges)
}

/// Reads audit-event rows whose `affected_refs` overlap with the export
/// scope (`exported_ids`). Cross-initiative by definition — audit events
/// are not currently attached to the initiative junction, so we filter
/// after the fact by intersection with the visible node set.
fn read_audit_events_touching(
    store: &Store,
    exported_ids: &HashSet<&str>,
) -> Result<Vec<AuditEvent>> {
    let script = r#"
        ?[validity, properties] := *node{id, type, validity, properties @ 'NOW'},
                                    type = 'audit_event'
        :order validity
    "#;
    let rows = store.run_read(script)?;

    let mut events: Vec<AuditEvent> = Vec::new();
    for row in &rows.rows {
        let (seconds, asserted) = parse_validity(row.first())?;
        if !asserted {
            continue;
        }
        let Some(DataValue::Json(jd)) = row.get(1) else {
            continue;
        };
        let json = &jd.0;
        let op = json
            .get("op")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let actor = json
            .get("actor")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let affected_refs: Vec<String> = json
            .get("affected_refs")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|x| x.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        // Keep only events whose affected_refs intersect the export.
        let touches_scope = affected_refs
            .iter()
            .any(|id| exported_ids.contains(id.as_str()));
        if !touches_scope {
            continue;
        }
        events.push(AuditEvent {
            seconds,
            op,
            actor,
            affected_refs,
        });
    }
    // `:order validity` puts newest-first because of the `Reverse<>`
    // wrapping; flip to chronological for a log.
    events.reverse();
    Ok(events)
}

fn read_initiatives_by_node(store: &Store) -> Result<HashMap<String, Vec<String>>> {
    let script = r#"
        ?[node_id, initiative] := *node_initiative{initiative, node_id}
    "#;
    let rows = store.run_read(script)?;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for row in &rows.rows {
        let Some(node_id) = row.first().and_then(|v| v.get_str()) else {
            continue;
        };
        let Some(initiative) = row.get(1).and_then(|v| v.get_str()) else {
            continue;
        };
        map.entry(node_id.to_string())
            .or_default()
            .push(initiative.to_string());
    }
    for inits in map.values_mut() {
        inits.sort();
        inits.dedup();
    }
    Ok(map)
}

/// Sanitises a node name into a filename-safe slug. Unicode letters
/// stay (lowercased), runs of unsafe characters collapse to `-`, and
/// the empty case falls back to `"unnamed"`.
fn sanitize(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut last_was_dash = false;
    for c in name.chars() {
        if c.is_alphanumeric() || c == '_' || c == '-' {
            out.extend(c.to_lowercase());
            last_was_dash = false;
        } else if !last_was_dash && !out.is_empty() {
            out.push('-');
            last_was_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.is_empty() {
        out.push_str("unnamed");
    }
    out
}

/// Builds an `id → unique sanitized name` map. Collisions get suffixes
/// `-2`, `-3`, … in id-sorted order so the assignment is deterministic
/// across exports.
fn build_unique_names(nodes: &[NodeRow]) -> HashMap<String, String> {
    let mut sorted: Vec<&NodeRow> = nodes.iter().collect();
    sorted.sort_by(|a, b| a.id.cmp(&b.id));

    let mut taken: HashSet<(String, String, String)> = HashSet::new();
    let mut id_to_name: HashMap<String, String> = HashMap::new();
    for node in sorted {
        let base = sanitize(&node.name);
        let mut candidate = base.clone();
        let mut suffix = 2;
        while !taken.insert((node.tier.clone(), node.node_type.clone(), candidate.clone())) {
            candidate = format!("{base}-{suffix}");
            suffix += 1;
        }
        id_to_name.insert(node.id.clone(), candidate);
    }
    id_to_name
}

fn node_path(root: &Path, node: &NodeRow, id_to_name: &HashMap<String, String>) -> PathBuf {
    let leaf = id_to_name
        .get(&node.id)
        .cloned()
        .unwrap_or_else(|| sanitize(&node.name));
    root.join(&node.tier)
        .join(&node.node_type)
        .join(format!("{leaf}.md"))
}

fn render_node(
    node: &NodeRow,
    initiatives: &[String],
    outgoing: &[&EdgeRow],
    incoming: &[&EdgeRow],
    id_to_name: &HashMap<String, String>,
) -> String {
    let mut out = String::new();

    // Frontmatter — minimal, deterministic key order.
    out.push_str("---\n");
    out.push_str(&format!("id: {}\n", node.id));
    out.push_str(&format!("type: {}\n", node.node_type));
    out.push_str(&format!("tier: {}\n", node.tier));
    if !initiatives.is_empty() {
        out.push_str("initiatives:\n");
        for init in initiatives {
            out.push_str(&format!("  - {init}\n"));
        }
    }
    if !node.tags.is_empty() {
        out.push_str("tags:\n");
        for tag in &node.tags {
            out.push_str(&format!("  - {tag}\n"));
        }
    }
    out.push_str("---\n\n");

    // Body.
    out.push_str(&format!("# {}\n\n", node.name));
    if let Some(body) = &node.body {
        out.push_str(body);
        if !body.ends_with('\n') {
            out.push('\n');
        }
        out.push('\n');
    }

    // Outgoing / Incoming, grouped by edge_type. Both peer maps are
    // resolved through `id_to_name`; an edge whose endpoint is missing
    // (shouldn't happen given the read filters, but defensive) is
    // skipped rather than rendered as a dangling link.
    if !outgoing.is_empty() {
        out.push_str("## Outgoing\n\n");
        render_edge_section(&mut out, outgoing, |e| &e.dst, id_to_name);
    }
    if !incoming.is_empty() {
        out.push_str("## Incoming\n\n");
        render_edge_section(&mut out, incoming, |e| &e.src, id_to_name);
    }

    out
}

fn render_edge_section<F>(
    out: &mut String,
    edges: &[&EdgeRow],
    peer: F,
    id_to_name: &HashMap<String, String>,
) where
    F: Fn(&EdgeRow) -> &str,
{
    let mut by_type: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for edge in edges {
        let peer_id = peer(edge);
        let Some(peer_name) = id_to_name.get(peer_id) else {
            continue;
        };
        by_type
            .entry(edge.edge_type.as_str())
            .or_default()
            .push(peer_name.as_str());
    }
    for (edge_type, mut peers) in by_type {
        peers.sort();
        peers.dedup();
        out.push_str(&format!("### {edge_type}\n\n"));
        for peer_name in peers {
            out.push_str(&format!("- [[{peer_name}]]\n"));
        }
        out.push('\n');
    }
}

/// Top-level vault metadata: scope, counts, layout description. Always
/// written, even when the substrate is empty — gives the snapshot a
/// recognisable llm-wiki shape.
fn render_readme(
    initiative: &Option<String>,
    nodes_count: usize,
    edges_count: usize,
    audit_count: usize,
) -> String {
    let scope = initiative
        .as_deref()
        .map(|s| format!("initiative `{s}`"))
        .unwrap_or_else(|| "all initiatives".to_string());
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");

    format!(
        r#"# kaeru vault

Exported: {now}
Scope: {scope}
Nodes: {nodes_count}
Edges: {edges_count}
Audit events: {audit_count}

This is a derived snapshot of a kaeru substrate. The substrate stays
authoritative — re-run `kaeru export` to refresh.

Layout:
- Pages live under `<tier>/<type>/<sanitized-name>.md`.
- Each page has YAML frontmatter (id, type, tier, initiatives, tags),
  a body, and optional `## Outgoing` / `## Incoming` sections of
  `[[wikilink]]` references grouped by edge type.

See [[INDEX]] for navigation across every exported page, and [[LOG]]
for the chronological audit-event stream of operations on visible
nodes.
"#
    )
}

/// Hierarchical navigation file: every exported node grouped by tier
/// and type, sorted alphabetically by sanitised filename within each
/// group, rendered as `[[wikilink]]`. Followed by graph-structure
/// sections — provenance forests, open questions, edge statistics —
/// so a re-entering reader sees both the catalogue and the topology.
fn render_index(
    nodes: &[NodeRow],
    id_to_name: &HashMap<String, String>,
    outgoing: &HashMap<String, Vec<&EdgeRow>>,
    incoming: &HashMap<String, Vec<&EdgeRow>>,
) -> String {
    // Group: tier → type → Vec<(sanitised name, original name)>.
    let mut tree: BTreeMap<&str, BTreeMap<&str, Vec<(&str, &str)>>> = BTreeMap::new();
    for node in nodes {
        let Some(slug) = id_to_name.get(&node.id) else {
            continue;
        };
        tree.entry(node.tier.as_str())
            .or_default()
            .entry(node.node_type.as_str())
            .or_default()
            .push((slug.as_str(), node.name.as_str()));
    }

    let mut out = String::from("# Index\n\n");
    if tree.is_empty() {
        out.push_str("(no exported nodes yet)\n");
        return out;
    }
    for (tier, types) in &tree {
        out.push_str(&format!("## {tier}\n\n"));
        for (node_type, items) in types {
            out.push_str(&format!("### {node_type}\n\n"));
            let mut sorted = items.clone();
            sorted.sort_by_key(|(slug, _)| *slug);
            for (slug, original) in sorted {
                if slug == original {
                    out.push_str(&format!("- [[{slug}]]\n"));
                } else {
                    // Show the original name when the slug differs (e.g.
                    // sanitisation collapsed punctuation) so the human
                    // reader still sees the canonical label.
                    out.push_str(&format!("- [[{slug}]] — {original}\n"));
                }
            }
            out.push('\n');
        }
    }

    render_provenance_forests(&mut out, nodes, id_to_name, outgoing);
    render_open_questions(&mut out, id_to_name, incoming);
    render_edge_stats(&mut out, outgoing);

    out
}

/// Renders archival nodes as roots of nested `derived_from` trees so
/// the reader sees "this conclusion came from these things". Cycle-safe
/// via a `seen` set; nodes already rendered higher up appear without
/// further expansion.
fn render_provenance_forests(
    out: &mut String,
    nodes: &[NodeRow],
    id_to_name: &HashMap<String, String>,
    outgoing: &HashMap<String, Vec<&EdgeRow>>,
) {
    let mut roots: Vec<&NodeRow> = nodes.iter().filter(|n| n.tier == "archival").collect();
    if roots.is_empty() {
        return;
    }
    roots.sort_by_key(|n| id_to_name.get(&n.id).map(|s| s.as_str()).unwrap_or(""));

    out.push_str("## Provenance forests\n\n");
    for root in &roots {
        let Some(slug) = id_to_name.get(&root.id) else {
            continue;
        };
        out.push_str(&format!("- [[{slug}]] ({})\n", root.node_type));
        let mut seen: HashSet<String> = HashSet::new();
        seen.insert(root.id.clone());
        render_derived_from_subtree(out, &root.id, outgoing, id_to_name, &mut seen, 1);
    }
    out.push('\n');
}

fn render_derived_from_subtree(
    out: &mut String,
    node_id: &str,
    outgoing: &HashMap<String, Vec<&EdgeRow>>,
    id_to_name: &HashMap<String, String>,
    seen: &mut HashSet<String>,
    depth: usize,
) {
    let Some(edges) = outgoing.get(node_id) else {
        return;
    };
    let indent = "  ".repeat(depth);
    let mut targets: Vec<&str> = edges
        .iter()
        .filter(|e| e.edge_type == "derived_from")
        .map(|e| e.dst.as_str())
        .collect();
    targets.sort_by_key(|id| id_to_name.get(*id).map(|s| s.as_str()).unwrap_or(""));
    for target in targets {
        let Some(slug) = id_to_name.get(target) else {
            continue;
        };
        if !seen.insert(target.to_string()) {
            out.push_str(&format!("{indent}- [[{slug}]] (already shown)\n"));
            continue;
        }
        out.push_str(&format!("{indent}- [[{slug}]]\n"));
        render_derived_from_subtree(out, target, outgoing, id_to_name, seen, depth + 1);
    }
}

/// Lists nodes with at least one inbound `contradicts` edge — the
/// open-review queue surfaced by `mark_under_review`. Empty section
/// is skipped (a healthy graph has nothing here).
fn render_open_questions(
    out: &mut String,
    id_to_name: &HashMap<String, String>,
    incoming: &HashMap<String, Vec<&EdgeRow>>,
) {
    let mut open: Vec<&str> = incoming
        .iter()
        .filter(|(_, edges)| edges.iter().any(|e| e.edge_type == "contradicts"))
        .map(|(id, _)| id.as_str())
        .collect();
    if open.is_empty() {
        return;
    }
    open.sort_by_key(|id| id_to_name.get(*id).map(|s| s.as_str()).unwrap_or(""));

    out.push_str("## Open questions\n\n");
    for id in open {
        if let Some(slug) = id_to_name.get(id) {
            out.push_str(&format!("- [[{slug}]]\n"));
        }
    }
    out.push('\n');
}

/// Texture summary — count of edges by type, for a quick read on the
/// graph's character ("mostly causal? heavy on contradicts?").
fn render_edge_stats(out: &mut String, outgoing: &HashMap<String, Vec<&EdgeRow>>) {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for edges in outgoing.values() {
        for edge in edges {
            *counts.entry(edge.edge_type.as_str()).or_insert(0) += 1;
        }
    }
    if counts.is_empty() {
        return;
    }
    out.push_str("## Edge stats\n\n");
    for (et, count) in &counts {
        out.push_str(&format!("- `{et}`: {count}\n"));
    }
    out.push('\n');
}

/// Chronological log of curator operations that touched at least one
/// exported node. Affected refs that resolve to an exported page are
/// rendered as wikilinks; refs outside the scope (e.g. an unrelated
/// audit row that incidentally shares an op name) become bare ids.
fn render_log(events: &[AuditEvent], id_to_name: &HashMap<String, String>) -> String {
    let mut out = String::from(
        "# Log\n\n\
         Operations on visible nodes, in chronological order.\n\n",
    );
    if events.is_empty() {
        out.push_str("(no audit events touch this scope)\n");
        return out;
    }
    for event in events {
        let when = DateTime::<Utc>::from_timestamp(event.seconds as i64, 0)
            .map(|d| d.format("%Y-%m-%d %H:%M:%S UTC").to_string())
            .unwrap_or_else(|| format!("t={}", event.seconds));
        let actor_suffix = if event.actor.is_empty() {
            String::new()
        } else {
            format!(" ({})", event.actor)
        };
        out.push_str(&format!(
            "## {when} — {op}{actor_suffix}\n\n",
            op = event.op
        ));
        for id in &event.affected_refs {
            match id_to_name.get(id) {
                Some(slug) => out.push_str(&format!("- [[{slug}]]\n")),
                None => out.push_str(&format!("- `{id}` (outside export scope)\n")),
            }
        }
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::sanitize;

    #[test]
    fn sanitize_basic() {
        assert_eq!(sanitize("hello world"), "hello-world");
        assert_eq!(sanitize("Hello World!"), "hello-world");
        assert_eq!(sanitize("with-dashes_underscores"), "with-dashes_underscores");
        assert_eq!(sanitize("   "), "unnamed");
        assert_eq!(sanitize("///"), "unnamed");
        assert_eq!(sanitize("trailing!!!"), "trailing");
        assert_eq!(sanitize("multiple   spaces"), "multiple-spaces");
    }

    #[test]
    fn sanitize_unicode() {
        // Cyrillic stays — files on modern Linux/macOS handle UTF-8.
        assert_eq!(sanitize("первая мысль"), "первая-мысль");
        // Mixed.
        assert_eq!(sanitize("alpha-первая"), "alpha-первая");
    }
}
