//! Review-flow tools: `flag`, `resolve`, `close_review`.

use kaeru_core::Store;
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{resolve_name, text, to_mcp, with_initiative};

pub fn flag(
    store: &Store,
    target: &str,
    reason: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let target_id = resolve_name(store, target)?;
        let review_id = kaeru_core::mark_under_review(store, &target_id, reason).map_err(to_mcp)?;
        Ok(text(&format!("flagged: {target} (review id: {review_id})")))
    })
}

pub fn resolve(
    store: &Store,
    question: &str,
    by: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let question_id = resolve_name(store, question)?;
        let by_id = resolve_name(store, by)?;
        kaeru_core::mark_resolved(store, &question_id, &by_id).map_err(to_mcp)?;
        Ok(text(&format!("resolved: {question} ← {by}")))
    })
}

pub fn close_review(
    store: &Store,
    target: &str,
    resolution: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let target_id = resolve_name(store, target)?;
        let closed = kaeru_core::resolve_review(store, &target_id, resolution).map_err(to_mcp)?;
        if closed.is_empty() {
            return Ok(text(&format!(
                "no open review on {target} — nothing to close"
            )));
        }
        let note = if resolution.is_some() {
            " (resolution recorded)"
        } else {
            ""
        };
        Ok(text(&format!(
            "closed {} review(s) on {target}{note}",
            closed.len()
        )))
    })
}
