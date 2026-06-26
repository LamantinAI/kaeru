//! Knowledge-chain tools: `chain` (materialize), `chains` (which chains is a
//! node in), `read_chain` (read the ordered trail), and `path` (compute the
//! shortest weighted path without saving). Chains let recall return a
//! connected reasoning trail instead of an isolated, context-poor node.

use kaeru_core::Store;
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{resolve_name_or_id, text, to_mcp, ts_suffix, with_initiative};

/// Materializes the shortest weighted path `from → to` as a saved chain.
pub fn chain(
    store: &Store,
    from: &str,
    to: &str,
    name: Option<&str>,
    summary: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let from_id = resolve_name_or_id(store, from)?;
        let to_id = resolve_name_or_id(store, to)?;
        match kaeru_core::create_chain(store, &from_id, &to_id, name, summary).map_err(to_mcp)? {
            Some(outcome) => {
                let members = kaeru_core::read_chain(store, &outcome.id).map_err(to_mcp)?;
                let trail = members
                    .iter()
                    .map(|m| m.name.as_str())
                    .collect::<Vec<_>>()
                    .join(" → ");
                let verb = if outcome.reused {
                    "reused existing chain"
                } else {
                    "chain saved"
                };
                Ok(text(&format!(
                    "{verb} ({} nodes): {trail}\nid: {}",
                    members.len(),
                    outcome.id
                )))
            }
            None => Ok(text(&format!(
                "no path from `{from}` to `{to}` — nothing to chain"
            ))),
        }
    })
}

/// Lists the chains a node belongs to.
pub fn chains(
    store: &Store,
    name: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name)?;
        let chains = kaeru_core::chains_of(store, &id).map_err(to_mcp)?;
        if chains.is_empty() {
            return Ok(text(&format!("`{name}` is in no chains")));
        }
        let mut out = format!("chains containing `{name}` ({}):\n", chains.len());
        for ch in &chains {
            out.push_str(&format!("  - {} — {}\n", ch.name, ch.id));
            if let Some(s) = &ch.body_excerpt {
                out.push_str(&format!("    {s}\n"));
            }
        }
        out.push_str("\nTriage by name + summary, then `read_chain <name|id>` for the full trail.");
        Ok(text(&out))
    })
}

/// Reads a chain's ordered members — the reasoning trail.
pub fn read_chain(
    store: &Store,
    name_or_id: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name_or_id)?;
        let members = kaeru_core::read_chain(store, &id).map_err(to_mcp)?;
        if members.is_empty() {
            return Ok(text(&format!(
                "`{name_or_id}` is not a chain, or has no members"
            )));
        }
        let mut out = format!("chain `{name_or_id}` ({} nodes):\n", members.len());
        for (i, m) in members.iter().enumerate() {
            out.push_str(&format!(
                "{}. {} ({}) — {}{}\n",
                i + 1,
                m.name,
                m.node_type,
                m.id,
                ts_suffix(m.ts)
            ));
            if let Some(e) = &m.body_excerpt {
                out.push_str(&format!("   {e}\n"));
            }
        }
        Ok(text(&out))
    })
}

/// Mutates an existing chain so it survives graph changes: with no `to`, it
/// regenerates (recomputes the shortest path between its current endpoints);
/// with `to`, it extends the trail out to that node. The chain keeps its id,
/// name, and summary.
pub fn rechain(
    store: &Store,
    chain: &str,
    to: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let chain_id = resolve_name_or_id(store, chain)?;
        let (action, stats) = match to {
            Some(target) => {
                let to_id = resolve_name_or_id(store, target)?;
                let s = kaeru_core::extend_chain(store, &chain_id, &to_id).map_err(to_mcp)?;
                ("extended", s)
            }
            None => {
                let s = kaeru_core::regenerate_chain(store, &chain_id).map_err(to_mcp)?;
                ("regenerated", s)
            }
        };
        let Some(stats) = stats else {
            return Ok(text(&format!(
                "`{chain}` left unchanged — endpoint unreachable now (no path)"
            )));
        };
        let members = kaeru_core::read_chain(store, &chain_id).map_err(to_mcp)?;
        let trail = members
            .iter()
            .map(|m| m.name.as_str())
            .collect::<Vec<_>>()
            .join(" → ");
        let note = if stats.changed {
            action
        } else {
            "already current"
        };
        Ok(text(&format!(
            "{note} ({} nodes): {trail}\nid: {chain_id}",
            stats.members
        )))
    })
}

/// Computes the shortest weighted path `from → to` without saving it.
pub fn path(
    store: &Store,
    from: &str,
    to: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let from_id = resolve_name_or_id(store, from)?;
        let to_id = resolve_name_or_id(store, to)?;
        let ids = kaeru_core::shortest_path(store, &from_id, &to_id).map_err(to_mcp)?;
        if ids.is_empty() {
            return Ok(text(&format!("no path from `{from}` to `{to}`")));
        }
        let names: Vec<String> = ids
            .iter()
            .map(|id| {
                kaeru_core::node_brief_by_id(store, id)
                    .ok()
                    .flatten()
                    .map(|b| b.name)
                    .unwrap_or_else(|| id.clone())
            })
            .collect();
        Ok(text(&format!(
            "path ({} nodes): {}\nUse `chain {from} {to}` to save it.",
            ids.len(),
            names.join(" → ")
        )))
    })
}
