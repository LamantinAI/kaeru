//! Consolidation: `settle`, `reopen`, `synthesise`, `supersede`.

use kaeru_core::{Error, NodeId, NodeType, Store, Visibility, get_visibility};
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{parse_tier, resolve_name, resolve_name_or_id, text, to_mcp, with_initiative};

/// The re-share hint appended when a consolidation-family verb replaces a
/// node whose local copy was `shared`: the successor is a brand-new id the
/// cloud has never seen, so the cloud keeps serving the retracted
/// predecessor until the successor is shared explicitly.
fn predecessor_shared_hint(store: &Store, old_id: &NodeId) -> Result<&'static str, McpError> {
    Ok(
        if get_visibility(store, old_id).map_err(to_mcp)? == Visibility::Shared {
            "\n⚠ predecessor was shared — the cloud still holds the old node; run `share` on the successor to update it."
        } else {
            ""
        },
    )
}

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
        let hint = predecessor_shared_hint(store, &draft_id)?;
        let id = kaeru_core::consolidate_out(store, &draft_id, new_type, new_name, new_body)
            .map_err(to_mcp)?;
        Ok(text(&format!(
            "settled: {source} → {new_name} ({}) — {id}{hint}",
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
        let hint = predecessor_shared_hint(store, &archival_id)?;
        let id = kaeru_core::consolidate_in(store, &archival_id, new_type, new_name, new_body)
            .map_err(to_mcp)?;
        Ok(text(&format!(
            "reopened: {source} → {new_name} ({}) — {id}{hint}",
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
        let id =
            kaeru_core::synthesise(store, &seed_ids, new_type, target_tier, new_name, new_body)
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
        let hint = predecessor_shared_hint(store, &old_id)?;
        let id = kaeru_core::supersedes(store, &old_id, new_type, target_tier, new_name, new_body)
            .map_err(to_mcp)?;
        Ok(text(&format!(
            "superseded: {old} → {new_name} ({} / {}) — {id}{hint}",
            new_type.as_str(),
            target_tier.as_str()
        )))
    })
}
