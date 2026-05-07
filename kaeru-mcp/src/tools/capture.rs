//! Write-side tools: `episode`, `jot`, `link`, `unlink`, `cite`.

use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use kaeru_core::EdgeType;
use kaeru_core::EpisodeKind;
use kaeru_core::Significance;
use kaeru_core::Store;

use crate::utils::resolve_name;
use crate::utils::text;
use crate::utils::to_mcp;
use crate::utils::with_initiative;

pub fn episode(
    store: &Store,
    name: &str,
    body: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = kaeru_core::write_episode(
            store,
            EpisodeKind::Observation,
            Significance::Medium,
            name,
            body,
        )
        .map_err(to_mcp)?;
        Ok(text(&format!("wrote episode: {id}")))
    })
}

pub fn jot(store: &Store, body: &str, initiative: Option<&str>) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = kaeru_core::jot(store, body).map_err(to_mcp)?;
        let name = kaeru_core::node_brief_by_id(store, &id)
            .ok()
            .flatten()
            .map(|b| b.name)
            .unwrap_or_default();
        Ok(text(&format!("jotted: {name} — {id}")))
    })
}

pub fn link(
    store: &Store,
    from: &str,
    to: &str,
    edge_type_str: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let edge: EdgeType = edge_type_str.parse().map_err(to_mcp)?;
        let from_id = resolve_name(store, from)?;
        let to_id = resolve_name(store, to)?;
        kaeru_core::link(store, &from_id, &to_id, edge).map_err(to_mcp)?;
        Ok(text(&format!(
            "linked: {from} -[{}]-> {to}",
            edge.as_str()
        )))
    })
}

pub fn unlink(
    store: &Store,
    from: &str,
    to: &str,
    edge_type_str: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let edge: EdgeType = edge_type_str.parse().map_err(to_mcp)?;
        let from_id = resolve_name(store, from)?;
        let to_id = resolve_name(store, to)?;
        kaeru_core::unlink(store, &from_id, &to_id, edge).map_err(to_mcp)?;
        Ok(text(&format!(
            "unlinked: {from} -[{}]-> {to}",
            edge.as_str()
        )))
    })
}

pub fn cite(
    store: &Store,
    name: &str,
    url: Option<&str>,
    body: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = kaeru_core::cite(store, name, url, body).map_err(to_mcp)?;
        let label = match url {
            Some(u) => format!("cited: {name} ({u}) — {id}"),
            None => format!("cited: {name} — {id}"),
        };
        Ok(text(&label))
    })
}
