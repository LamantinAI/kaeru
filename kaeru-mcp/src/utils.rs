//! Result rendering and scoped resolution for network operations.

use kaeru_core::api::{ApiOptions, NodeRef, resolve_node};
use kaeru_core::{Error, NodeId, Store};
use rmcp::ErrorData as McpError;
use rmcp::model::{CallToolResult, Content};

pub fn text(message: &str) -> CallToolResult {
    CallToolResult::success(vec![Content::text(message.to_string())])
}
pub fn to_mcp(error: Error) -> McpError {
    McpError::internal_error(error.to_string(), None)
}
pub fn with_initiative<T>(
    store: &Store,
    initiative: Option<&str>,
    f: impl FnOnce() -> Result<T, McpError>,
) -> Result<T, McpError> {
    store.scoped(initiative, |_| f())
}
pub fn resolve_name_or_id(store: &Store, reference: &str) -> Result<NodeId, McpError> {
    resolve_node(
        store,
        &NodeRef::NameOrId(reference.into()),
        &ApiOptions::local(),
    )
    .map_err(|e| McpError::invalid_params(e.message, None))
}
