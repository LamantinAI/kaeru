//! Diagnostic tool: `lint`.

use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use kaeru_core::Store;

use crate::utils::brief_suffix;
use crate::utils::text;
use crate::utils::to_mcp;
use crate::utils::with_initiative;

pub fn lint(store: &Store, initiative: Option<&str>) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let report = kaeru_core::lint(store).map_err(to_mcp)?;
        let mut out = format!("orphans ({}):\n", report.orphans.len());
        for id in &report.orphans {
            out.push_str(&format!("  - {id}{}\n", brief_suffix(store, id)));
        }
        out.push('\n');
        out.push_str(&format!(
            "unresolved reviews ({}):\n",
            report.unresolved_reviews.len()
        ));
        for id in &report.unresolved_reviews {
            out.push_str(&format!("  - {id}{}\n", brief_suffix(store, id)));
        }
        Ok(text(&out))
    })
}
