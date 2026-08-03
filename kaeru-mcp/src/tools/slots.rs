//! Slot tools: fill a role with exactly one live node, list the roles.

use kaeru_core::Store;
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{resolve_name_or_id, text, to_mcp, with_initiative};

/// Makes a node the live holder of a role, archiving whoever held it before.
pub fn slot(
    store: &Store,
    initiative: &str,
    slot: &str,
    name: &str,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, Some(initiative), || {
        let node_id = resolve_name_or_id(store, name)?;
        let outcome = kaeru_core::occupy_slot(store, initiative, slot, &node_id).map_err(to_mcp)?;
        let body = match outcome.previous {
            Some(prev) => {
                let prev_name = kaeru_core::node_brief_by_id(store, &prev)
                    .ok()
                    .flatten()
                    .map(|b| b.name)
                    .unwrap_or(prev);
                format!(
                    "slot `{slot}` in `{initiative}` → {name}\n\
                     ↳ previous holder `{prev_name}` archived to cold and linked by `supersedes` \
                     — still readable via `at` / `surface layers=cold`, just out of the window."
                )
            }
            None => format!("slot `{slot}` in `{initiative}` → {name} (the slot was empty)"),
        };
        Ok(text(&body))
    })
}

/// Lists the filled roles of an initiative.
pub fn slots(store: &Store, initiative: &str) -> Result<CallToolResult, McpError> {
    with_initiative(store, Some(initiative), || {
        let filled = kaeru_core::slots_in(store, initiative).map_err(to_mcp)?;
        if filled.is_empty() {
            return Ok(text(&format!(
                "no slots filled in `{initiative}`.\n\
                 ↳ a slot is a role held by exactly one live node — `handoff`, `entrypoint`, \
                 `queue`. Fill one with `slot`, and each new member archives its predecessor."
            )));
        }
        let mut out = format!("slots in `{initiative}` ({}):\n", filled.len());
        for (role, node_id) in filled {
            let name = kaeru_core::node_brief_by_id(store, &node_id)
                .ok()
                .flatten()
                .map(|b| b.name)
                .unwrap_or_else(|| node_id.clone());
            out.push_str(&format!("  {role} → {name}\n"));
        }
        Ok(text(&out))
    })
}

/// Frees a role without touching the node that held it.
pub fn unslot(store: &Store, initiative: &str, slot: &str) -> Result<CallToolResult, McpError> {
    with_initiative(store, Some(initiative), || match kaeru_core::release_slot(
        store, initiative, slot,
    )
    .map_err(to_mcp)?
    {
        Some(prev) => {
            let name = kaeru_core::node_brief_by_id(store, &prev)
                .ok()
                .flatten()
                .map(|b| b.name)
                .unwrap_or(prev);
            Ok(text(&format!(
                "slot `{slot}` in `{initiative}` released (was `{name}`; its layer is unchanged)"
            )))
        }
        None => Ok(text(&format!(
            "slot `{slot}` in `{initiative}` was already empty"
        ))),
    })
}
