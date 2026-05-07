//! `overview` — terminal-readable map of a subgraph: counts by
//! tier/type, provenance forests rooted at archival nodes, the
//! open-review queue, and edge statistics.
//!
//! Same surface as `INDEX.md` from the export, but rendered as plain
//! text (no `[[wikilink]]` syntax) so an agent can `kaeru overview`
//! and read the shape without round-tripping through the filesystem.
//! Honours `Store.current_initiative()`.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::errors::Result;
use crate::store::Store;

/// Returns a multi-line text overview of the substrate at NOW —
/// ready to print straight to stdout. Initiative-scoped when the
/// store has a current initiative set.
pub fn overview(store: &Store) -> Result<String> {
    let initiative = store.current_initiative();
    let nodes = read_overview_nodes(store, initiative.as_deref())?;
    let edges = read_overview_edges(store, initiative.as_deref())?;

    // Group nodes by tier → type → list of names.
    let mut by_tier_type: BTreeMap<&str, BTreeMap<&str, Vec<&str>>> = BTreeMap::new();
    for node in &nodes {
        by_tier_type
            .entry(node.tier.as_str())
            .or_default()
            .entry(node.node_type.as_str())
            .or_default()
            .push(node.name.as_str());
    }

    // Outgoing edges grouped by src for provenance-tree traversal,
    // incoming edges grouped by dst for "open questions" detection.
    let mut outgoing_derived_from: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut incoming_contradicts: HashSet<&str> = HashSet::new();
    let mut edge_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for edge in &edges {
        *edge_counts.entry(edge.edge_type.as_str()).or_insert(0) += 1;
        if edge.edge_type == "derived_from" {
            outgoing_derived_from
                .entry(edge.src.as_str())
                .or_default()
                .push(edge.dst.as_str());
        }
        if edge.edge_type == "contradicts" {
            incoming_contradicts.insert(edge.dst.as_str());
        }
    }

    // id → name lookup used by the tree renderer; entries outside this
    // set (e.g. nodes hidden from the current initiative scope) become
    // `(unknown)` placeholders so dangling references still render.
    let id_to_name: HashMap<&str, &str> = nodes
        .iter()
        .map(|n| (n.id.as_str(), n.name.as_str()))
        .collect();

    let mut out = String::new();

    // Header.
    let scope = initiative
        .as_deref()
        .map(|s| format!("initiative `{s}`"))
        .unwrap_or_else(|| "all initiatives".to_string());
    out.push_str(&format!(
        "overview of {scope} — {} node(s), {} edge(s)\n\n",
        nodes.len(),
        edges.len()
    ));

    // Categorical breakdown: tier → type → names.
    if by_tier_type.is_empty() {
        out.push_str("(no nodes)\n");
        return Ok(out);
    }
    for (tier, types) in &by_tier_type {
        out.push_str(&format!("{tier}:\n"));
        for (node_type, names) in types {
            out.push_str(&format!("  {node_type} ({}):\n", names.len()));
            let mut sorted = names.clone();
            sorted.sort();
            for name in sorted {
                out.push_str(&format!("    - {name}\n"));
            }
        }
        // Blank line between tiers so visual blocks read separately.
        out.push('\n');
    }

    // Provenance forests — archival nodes as roots of derived_from
    // trees. Cycle-safe via `seen` set.
    let archival_roots: Vec<&NodeRow> = nodes.iter().filter(|n| n.tier == "archival").collect();
    if !archival_roots.is_empty() {
        out.push_str("provenance:\n");
        let mut sorted_roots = archival_roots.clone();
        sorted_roots.sort_by_key(|n| n.name.as_str());
        for root in sorted_roots {
            out.push_str(&format!("  {} ({})\n", root.name, root.node_type));
            let mut seen: HashSet<&str> = HashSet::new();
            seen.insert(root.id.as_str());
            render_subtree(&mut out, &root.id, &outgoing_derived_from, &id_to_name, &mut seen, 2);
        }
        out.push('\n');
    }

    // Open questions — nodes with inbound contradicts edges.
    if !incoming_contradicts.is_empty() {
        out.push_str("open questions:\n");
        let mut open_names: Vec<&str> = incoming_contradicts
            .iter()
            .filter_map(|id| id_to_name.get(id).copied())
            .collect();
        open_names.sort();
        for name in open_names {
            out.push_str(&format!("  - {name}\n"));
        }
        out.push('\n');
    }

    // Edge stats — texture summary.
    if !edge_counts.is_empty() {
        out.push_str("edge stats:\n");
        for (et, count) in &edge_counts {
            out.push_str(&format!("  {et}: {count}\n"));
        }
    }

    Ok(out)
}

#[derive(Debug)]
struct NodeRow {
    id: String,
    node_type: String,
    tier: String,
    name: String,
}

#[derive(Debug)]
struct EdgeRow {
    src: String,
    dst: String,
    edge_type: String,
}

fn read_overview_nodes(store: &Store, initiative: Option<&str>) -> Result<Vec<NodeRow>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match initiative {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.to_string().into()));
            r#"
                ?[id, type, tier, name] :=
                    *node{id, type, tier, name @ 'NOW'},
                    type != 'audit_event',
                    *node_initiative{initiative, node_id: id},
                    initiative = $init
            "#
        }
        None => {
            r#"
                ?[id, type, tier, name] :=
                    *node{id, type, tier, name @ 'NOW'},
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
            Some(NodeRow {
                id: row.first().and_then(|v| v.get_str())?.to_string(),
                node_type: row.get(1).and_then(|v| v.get_str())?.to_string(),
                tier: row.get(2).and_then(|v| v.get_str())?.to_string(),
                name: row.get(3).and_then(|v| v.get_str())?.to_string(),
            })
        })
        .collect();
    Ok(nodes)
}

fn read_overview_edges(store: &Store, initiative: Option<&str>) -> Result<Vec<EdgeRow>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match initiative {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.to_string().into()));
            r#"
                ?[src, dst, edge_type] :=
                    *edge{src, dst, edge_type @ 'NOW'},
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
            Some(EdgeRow {
                src: row.first().and_then(|v| v.get_str())?.to_string(),
                dst: row.get(1).and_then(|v| v.get_str())?.to_string(),
                edge_type: row.get(2).and_then(|v| v.get_str())?.to_string(),
            })
        })
        .collect();
    Ok(edges)
}

fn render_subtree<'a>(
    out: &mut String,
    node_id: &'a str,
    outgoing: &HashMap<&'a str, Vec<&'a str>>,
    id_to_name: &HashMap<&'a str, &'a str>,
    seen: &mut HashSet<&'a str>,
    depth: usize,
) {
    let indent = "  ".repeat(depth);
    let Some(targets) = outgoing.get(node_id) else {
        return;
    };
    let mut sorted_targets: Vec<&'a str> = targets.iter().copied().collect();
    sorted_targets.sort_by_key(|id| id_to_name.get(id).copied().unwrap_or("(unknown)"));
    for target in sorted_targets {
        let name = id_to_name.get(target).copied().unwrap_or("(unknown)");
        if !seen.insert(target) {
            out.push_str(&format!("{indent}- {name} (already shown)\n"));
            continue;
        }
        out.push_str(&format!("{indent}- {name}\n"));
        render_subtree(out, target, outgoing, id_to_name, seen, depth + 1);
    }
}
