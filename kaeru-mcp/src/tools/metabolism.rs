//! Hygiene tools: `forget`, `revise`, `layer`.

use std::str::FromStr;

use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use kaeru_core::Error;
use kaeru_core::Layer;
use kaeru_core::Store;
use kaeru_core::set_layer as core_set_layer;

use crate::utils::resolve_name_or_id;
use crate::utils::text;
use crate::utils::to_mcp;
use crate::utils::with_initiative;

pub fn forget(
    store: &Store,
    name_or_id: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name_or_id)?;
        kaeru_core::forget(store, &id).map_err(to_mcp)?;
        Ok(text(&format!("forgot: {name_or_id}")))
    })
}

pub fn set_layer(
    store: &Store,
    name_or_id: &str,
    layer: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let parsed = Layer::from_str(layer).map_err(to_mcp)?;
        let id = resolve_name_or_id(store, name_or_id)?;
        core_set_layer(store, &id, parsed).map_err(to_mcp)?;
        Ok(text(&format!("layer: {name_or_id} → {}", parsed.as_str())))
    })
}

pub fn revise(
    store: &Store,
    name: &str,
    body: Option<&str>,
    rename: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name)?;
        let brief = kaeru_core::node_brief_by_id(store, &id)
            .map_err(to_mcp)?
            .ok_or_else(|| {
                to_mcp(Error::NotFound(format!("node {name:?} not found at NOW")))
            })?;
        let new_name = rename.unwrap_or(&brief.name);
        let preserved_body = if body.is_none() {
            kaeru_core::summary_view(store, &id)
                .map_err(to_mcp)?
                .root
                .body_excerpt
                .unwrap_or_default()
        } else {
            String::new()
        };
        let new_body = body.unwrap_or(&preserved_body);
        kaeru_core::improve(store, &id, new_name, new_body).map_err(to_mcp)?;
        Ok(text(&format!("revised: {name} → {new_name}")))
    })
}
