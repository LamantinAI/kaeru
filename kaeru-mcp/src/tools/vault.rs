//! Snapshot tool: `export`.

use kaeru_core::Store;
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{text, to_mcp, with_initiative};

pub fn export(
    store: &Store,
    output_dir: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let summary = kaeru_core::export_vault(store, output_dir).map_err(to_mcp)?;
        Ok(text(&format!(
            "exported {} node(s), {} edge(s) → {}",
            summary.nodes_exported,
            summary.edges_exported,
            summary.root.display()
        )))
    })
}
