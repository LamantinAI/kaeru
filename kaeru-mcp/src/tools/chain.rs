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
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let from_id = resolve_name_or_id(store, from)?;
        let to_id = resolve_name_or_id(store, to)?;
        match kaeru_core::create_chain(store, &from_id, &to_id, name).map_err(to_mcp)? {
            Some(cid) => {
                let members = kaeru_core::read_chain(store, &cid).map_err(to_mcp)?;
                let trail = members
                    .iter()
                    .map(|m| m.name.as_str())
                    .collect::<Vec<_>>()
                    .join(" → ");
                Ok(text(&format!(
                    "chain saved ({} nodes): {trail}\nid: {cid}",
                    members.len()
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
        }
        out.push_str("\nUse `read_chain <name|id>` to read a trail in full.");
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
