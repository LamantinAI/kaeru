//! Personal-life capture tools: `task`, `done`.

use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use kaeru_core::Store;

use crate::utils::{
    parse_due_to_iso, parse_layer, resolve_name_or_id, text, to_mcp, with_initiative,
};

pub fn task(
    store: &Store,
    body: &str,
    due: Option<&str>,
    layer: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let due_iso = match due {
            Some(d) => Some(parse_due_to_iso(d)?),
            None => None,
        };
        let layer = parse_layer(layer)?;
        let id = kaeru_core::write_task_with_layer(store, body, due_iso.as_deref(), layer)
            .map_err(to_mcp)?;
        let name = kaeru_core::node_brief_by_id(store, &id)
            .ok()
            .flatten()
            .map(|b| b.name)
            .unwrap_or_default();
        let label = match due_iso.as_deref() {
            Some(d) => format!("task: {name} (due {d}) — {id}"),
            None => format!("task: {name} — {id}"),
        };
        Ok(text(&label))
    })
}

pub fn done(
    store: &Store,
    name_or_id: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name_or_id)?;
        kaeru_core::complete_task(store, &id).map_err(to_mcp)?;
        Ok(text(&format!("done: {name_or_id}")))
    })
}
