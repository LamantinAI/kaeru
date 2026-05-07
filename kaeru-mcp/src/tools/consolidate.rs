//! Consolidation: `settle`, `reopen`, `synthesise`, `supersede`.

use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use kaeru_core::Error;
use kaeru_core::NodeType;
use kaeru_core::Store;

use crate::utils::parse_tier;
use crate::utils::resolve_name;
use crate::utils::resolve_name_or_id;
use crate::utils::text;
use crate::utils::to_mcp;
use crate::utils::with_initiative;

pub fn settle(
    store: &Store,
    source: &str,
    new_type_str: &str,
    new_name: &str,
    new_body: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let draft_id = resolve_name(store, source)?;
        let new_type: NodeType = new_type_str.parse().map_err(to_mcp)?;
        let id = kaeru_core::consolidate_out(store, &draft_id, new_type, new_name, new_body)
            .map_err(to_mcp)?;
        Ok(text(&format!(
            "settled: {source} → {new_name} ({}) — {id}",
            new_type.as_str()
        )))
    })
}

pub fn reopen(
    store: &Store,
    source: &str,
    new_type_str: &str,
    new_name: &str,
    new_body: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let archival_id = resolve_name(store, source)?;
        let new_type: NodeType = new_type_str.parse().map_err(to_mcp)?;
        let id = kaeru_core::consolidate_in(store, &archival_id, new_type, new_name, new_body)
            .map_err(to_mcp)?;
        Ok(text(&format!(
            "reopened: {source} → {new_name} ({}) — {id}",
            new_type.as_str()
        )))
    })
}

pub fn synthesise(
    store: &Store,
    from: &[String],
    new_type_str: &str,
    new_name: &str,
    new_body: &str,
    tier: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        if from.is_empty() {
            return Err(to_mcp(Error::Invalid(
                "from must list at least one seed".to_string(),
            )));
        }
        let new_type: NodeType = new_type_str.parse().map_err(to_mcp)?;
        let target_tier = match tier {
            Some(t) => parse_tier(t).map_err(to_mcp)?,
            None => new_type.default_tier(),
        };
        let mut seed_ids = Vec::with_capacity(from.len());
        for n in from {
            seed_ids.push(resolve_name(store, n)?);
        }
        let id = kaeru_core::synthesise(
            store,
            &seed_ids,
            new_type,
            target_tier,
            new_name,
            new_body,
        )
        .map_err(to_mcp)?;
        Ok(text(&format!(
            "synthesised: {new_name} ({} / {}) — {id}",
            new_type.as_str(),
            target_tier.as_str()
        )))
    })
}

pub fn supersede(
    store: &Store,
    old: &str,
    new_type_str: &str,
    new_name: &str,
    new_body: &str,
    tier: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let old_id = resolve_name_or_id(store, old)?;
        let new_type: NodeType = new_type_str.parse().map_err(to_mcp)?;
        let target_tier = match tier {
            Some(t) => parse_tier(t).map_err(to_mcp)?,
            None => new_type.default_tier(),
        };
        let id = kaeru_core::supersedes(
            store,
            &old_id,
            new_type,
            target_tier,
            new_name,
            new_body,
        )
        .map_err(to_mcp)?;
        Ok(text(&format!(
            "superseded: {old} → {new_name} ({} / {}) — {id}",
            new_type.as_str(),
            target_tier.as_str()
        )))
    })
}
