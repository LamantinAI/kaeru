//! Session-restoration & vault-meta tools: `awake`, `overview`,
//! `initiatives`, `recent`, `pin`, `unpin`, `config`.

use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use kaeru_core::Store;

use crate::utils::brief_suffix;
use crate::utils::parse_duration_secs;
use crate::utils::resolve_name_or_id;
use crate::utils::text;
use crate::utils::to_mcp;
use crate::utils::with_initiative;

pub fn awake(store: &Store, initiative: Option<&str>) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let ctx = kaeru_core::awake(store).map_err(to_mcp)?;
        let mut out = String::new();
        out.push_str(&format!(
            "initiative: {}\n\n",
            ctx.initiative.as_deref().unwrap_or("(none)")
        ));
        out.push_str(&format!("pinned ({}):\n", ctx.pinned.len()));
        for id in &ctx.pinned {
            out.push_str(&format!("  - {id}{}\n", brief_suffix(store, id)));
        }
        out.push('\n');
        out.push_str(&format!("recent ({}):\n", ctx.recent.len()));
        for id in &ctx.recent {
            out.push_str(&format!("  - {id}{}\n", brief_suffix(store, id)));
        }
        out.push('\n');
        out.push_str(&format!("under review ({}):\n", ctx.under_review.len()));
        for id in &ctx.under_review {
            out.push_str(&format!("  - {id}{}\n", brief_suffix(store, id)));
        }
        Ok(text(&out))
    })
}

pub fn overview(store: &Store, initiative: Option<&str>) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let report = kaeru_core::overview(store).map_err(to_mcp)?;
        Ok(text(&report))
    })
}

pub fn initiatives(store: &Store) -> Result<CallToolResult, McpError> {
    let names = kaeru_core::list_initiatives(store).map_err(to_mcp)?;
    if names.is_empty() {
        return Ok(text(
            "(no initiatives yet — pass `initiative` on a mutation to register one)",
        ));
    }
    let mut out = format!("initiatives ({}):\n", names.len());
    for n in &names {
        out.push_str(&format!("  - {n}\n"));
    }
    Ok(text(&out))
}

pub fn recent(
    store: &Store,
    since: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let window = parse_duration_secs(since).map_err(to_mcp)?;
        let ids = kaeru_core::recent_episodes(store, window).map_err(to_mcp)?;
        let mut out = format!("recent ({}):\n", ids.len());
        for id in &ids {
            out.push_str(&format!("  - {id}{}\n", brief_suffix(store, id)));
        }
        Ok(text(&out))
    })
}

pub fn pin(
    store: &Store,
    name_or_id: &str,
    reason: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name_or_id)?;
        kaeru_core::pin(store, &id, reason).map_err(to_mcp)?;
        Ok(text(&format!("pinned: {name_or_id} ({id})")))
    })
}

pub fn unpin(
    store: &Store,
    name_or_id: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name_or_id)?;
        kaeru_core::unpin(store, &id).map_err(to_mcp)?;
        Ok(text(&format!("unpinned: {name_or_id} ({id})")))
    })
}

pub fn config(store: &Store) -> Result<CallToolResult, McpError> {
    let c = store.config();
    let out = format!(
        "kaeru {}\nvault_path           = {}\nactive_window_size   = {}\nrecent_episodes_cap  = {}\nawake_window_secs    = {}\nsummary_children_cap = {}\nbody_excerpt_chars   = {}\nprovenance_max_hops  = {}\ndefault_max_hops     = {}\nmax_hops_cap         = {}\n",
        kaeru_core::version(),
        c.vault_path.display(),
        c.active_window_size,
        c.recent_episodes_cap,
        c.awake_default_window_secs,
        c.summary_view_children_cap,
        c.body_excerpt_chars,
        c.provenance_max_hops,
        c.default_max_hops,
        c.max_hops_cap,
    );
    Ok(text(&out))
}
