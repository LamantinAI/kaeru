//! Task-board tools: `board` (read the columns), `set_status` (move a task),
//! `board_status` (customize the column registry). All are initiative-scoped —
//! a board is per-initiative.

use kaeru_core::{BoardStatus, Error, Store};
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{parse_when, resolve_name_or_id, text, to_mcp, with_initiative};

fn need_initiative(initiative: Option<&str>) -> Result<&str, McpError> {
    initiative.ok_or_else(|| to_mcp(Error::Invalid("a board needs an `initiative`".to_string())))
}

fn render_statuses(statuses: &[BoardStatus]) -> String {
    let cols: Vec<String> = statuses
        .iter()
        .map(|s| format!("{} [{}]", s.label, s.key))
        .collect();
    format!("statuses: {}", cols.join(" → "))
}

/// Renders the board as columns with their tasks (in registry order, empties
/// included). `when` rewinds the whole board — columns and cards — to a past
/// moment.
pub fn board(
    store: &Store,
    initiative: Option<&str>,
    when: Option<&str>,
) -> Result<CallToolResult, McpError> {
    let init = need_initiative(initiative)?;
    let at = match when {
        Some(w) if !w.trim().is_empty() => Some(parse_when(w).map_err(to_mcp)?),
        _ => None,
    };
    let view = kaeru_core::board_view_at(store, init, at).map_err(to_mcp)?;

    let mut out = match when {
        Some(w) if !w.trim().is_empty() => format!("board `{}` [as of {w}]:\n", view.initiative),
        _ => format!("board `{}`:\n", view.initiative),
    };
    for c in &view.columns {
        out.push_str(&format!("\n{} [{}] ({})\n", c.label, c.key, c.tasks.len()));
        for t in &c.tasks {
            let due = t
                .due
                .as_deref()
                .map(|d| format!(" (due {d})"))
                .unwrap_or_default();
            out.push_str(&format!("  - {}{} — {}\n", t.name, due, t.id));
        }
    }
    Ok(text(&out))
}

/// Moves a task to `status` (its board column) — strict against the registry.
pub fn set_status(
    store: &Store,
    task: &str,
    status: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    let init = need_initiative(initiative)?;
    with_initiative(store, Some(init), || {
        let id = resolve_name_or_id(store, task)?;
        kaeru_core::set_status(store, init, &id, status).map_err(to_mcp)?;
        Ok(text(&format!("moved `{task}` → {status}")))
    })
}

/// Customizes the column registry: `add` / `remove` / `relabel` / `reorder`.
pub fn board_status(
    store: &Store,
    initiative: Option<&str>,
    action: &str,
    key: Option<&str>,
    label: Option<&str>,
    order: Option<&[String]>,
) -> Result<CallToolResult, McpError> {
    let init = need_initiative(initiative)?;
    let need_key = || key.ok_or_else(|| to_mcp(Error::Invalid("`key` is required".to_string())));

    let statuses = match action {
        "add" => {
            let key = need_key()?;
            kaeru_core::add_status(store, init, key, label.unwrap_or(key)).map_err(to_mcp)?
        }
        "remove" => kaeru_core::remove_status(store, init, need_key()?).map_err(to_mcp)?,
        "relabel" => {
            let key = need_key()?;
            let label =
                label.ok_or_else(|| to_mcp(Error::Invalid("`label` is required".to_string())))?;
            kaeru_core::relabel_status(store, init, key, label).map_err(to_mcp)?
        }
        "reorder" => {
            let order =
                order.ok_or_else(|| to_mcp(Error::Invalid("`order` is required".to_string())))?;
            kaeru_core::reorder_statuses(store, init, order).map_err(to_mcp)?
        }
        other => {
            return Err(to_mcp(Error::Invalid(format!(
                "unknown action `{other}` — use add / remove / relabel / reorder"
            ))));
        }
    };
    Ok(text(&render_statuses(&statuses)))
}
