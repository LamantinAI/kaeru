//! Read-side tools: `recall`, `drill`, `trace`, `search`, `ideas`,
//! `outcomes`, `tagged`, `between`.

use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use kaeru_core::Store;

use crate::utils::render_briefs;
use crate::utils::render_summary;
use crate::utils::resolve_name;
use crate::utils::resolve_name_or_id;
use crate::utils::text;
use crate::utils::to_mcp;
use crate::utils::with_initiative;

pub fn recall(
    store: &Store,
    name: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        match kaeru_core::recall_id_by_name(store, name).map_err(to_mcp)? {
            Some(id) => Ok(text(&id)),
            None => Ok(text("(not found)")),
        }
    })
}

pub fn drill(
    store: &Store,
    name: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name)?;
        let view = kaeru_core::summary_view(store, &id).map_err(to_mcp)?;
        Ok(text(&render_summary(&view)))
    })
}

pub fn trace(
    store: &Store,
    name: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name(store, name)?;
        let ancestors = kaeru_core::recollect_provenance(store, &id).map_err(to_mcp)?;
        if ancestors.is_empty() {
            return Ok(text("(no provenance)"));
        }
        let mut out = format!("provenance ({}):\n", ancestors.len());
        for b in &ancestors {
            out.push_str(&format!("  - {} ({}) — {}\n", b.name, b.node_type, b.id));
            if let Some(e) = &b.body_excerpt {
                out.push_str(&format!("    {e}\n"));
            }
        }
        Ok(text(&out))
    })
}

pub fn search(
    store: &Store,
    query: &str,
    limit: usize,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let hits = kaeru_core::fuzzy_recall(store, query, limit).map_err(to_mcp)?;
        if hits.is_empty() {
            return Ok(text("(no matches)"));
        }
        let mut out = format!("matches ({}):\n", hits.len());
        for b in &hits {
            out.push_str(&format!("  - {} ({}) — {}\n", b.name, b.node_type, b.id));
            if let Some(e) = &b.body_excerpt {
                out.push_str(&format!("    {e}\n"));
            }
        }
        Ok(text(&out))
    })
}

pub fn ideas(store: &Store, initiative: Option<&str>) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let briefs = kaeru_core::recollect_idea(store).map_err(to_mcp)?;
        Ok(text(&render_briefs("ideas", &briefs)))
    })
}

pub fn outcomes(store: &Store, initiative: Option<&str>) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let briefs = kaeru_core::recollect_outcome(store).map_err(to_mcp)?;
        Ok(text(&render_briefs("outcomes", &briefs)))
    })
}

pub fn tagged(
    store: &Store,
    tag: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let briefs = kaeru_core::tagged(store, tag).map_err(to_mcp)?;
        Ok(text(&render_briefs(&format!("tagged `{tag}`"), &briefs)))
    })
}

pub fn between(
    store: &Store,
    a: &str,
    b: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let a_id = resolve_name(store, a)?;
        let b_id = resolve_name(store, b)?;
        let edges = kaeru_core::between(store, &a_id, &b_id).map_err(to_mcp)?;
        if edges.is_empty() {
            return Ok(text(&format!("(no edges between {a} and {b})")));
        }
        let mut out = format!("edges ({}):\n", edges.len());
        for e in &edges {
            if e.a_to_b {
                out.push_str(&format!("  {a} —[{}]→ {b}\n", e.edge_type));
            } else {
                out.push_str(&format!("  {a} ←[{}]— {b}\n", e.edge_type));
            }
        }
        Ok(text(&out))
    })
}
