//! Review-flow tools: `flag`, `resolve`.

use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use kaeru_core::Store;

use crate::utils::resolve_name;
use crate::utils::text;
use crate::utils::to_mcp;
use crate::utils::with_initiative;

pub fn flag(
    store: &Store,
    target: &str,
    reason: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let target_id = resolve_name(store, target)?;
        let review_id = kaeru_core::mark_under_review(store, &target_id, reason)
            .map_err(to_mcp)?;
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
