//! Bi-temporal handle: `at`, `history`.

use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use kaeru_core::Store;

use crate::utils::parse_when;
use crate::utils::resolve_name;
use crate::utils::text;
use crate::utils::to_mcp;
use crate::utils::with_initiative;

pub fn at(
    store: &Store,
    name: &str,
    when: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name(store, name)?;
        let secs = parse_when(when).map_err(to_mcp)?;
        match kaeru_core::at(store, &id, secs).map_err(to_mcp)? {
            Some(snap) => {
                let body = snap.body.unwrap_or_else(|| "(no body)".to_string());
                Ok(text(&format!("{}\n\n{}", snap.name, body)))
            }
            None => Ok(text("(no row valid at that moment)")),
        }
    })
}

pub fn history(
    store: &Store,
    name: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name(store, name)?;
        let revs = kaeru_core::history(store, &id).map_err(to_mcp)?;
        if revs.is_empty() {
            return Ok(text("(no history)"));
        }
        let mut out = format!("history ({}):\n", revs.len());
        for r in &revs {
            let mark = if r.asserted { "+" } else { "-" };
            out.push_str(&format!("  [{mark}] t={:.0}  {}\n", r.seconds, r.name));
        }
        Ok(text(&out))
    })
}
