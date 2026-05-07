//! Snapshot tool: `export`.

use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use kaeru_core::Store;

use crate::utils::text;
use crate::utils::to_mcp;
use crate::utils::with_initiative;

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
