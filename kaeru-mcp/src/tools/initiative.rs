//! Initiative-level tools: `rename_initiative`, `delete_initiative`.
//!
//! Both act on the **local** vault by default. Passing `cloud: true` also
//! propagates the change to the shared cloud — a team-wide operation, so it
//! is opt-in (explicit confirmation), never the default. A local-only
//! rename/delete of a *shared* initiative diverges from the cloud until you
//! repeat it there; the tool says which side it touched.

use kaeru_core::Store;
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::cloud_client::CloudClient;
use crate::utils::{text, to_mcp};

pub async fn rename_initiative(
    store: &Store,
    cloud: Option<&CloudClient>,
    old: &str,
    new: &str,
    also_cloud: bool,
) -> Result<CallToolResult, McpError> {
    // Local first — if it fails (e.g. name collision) the cloud is untouched.
    let stats = kaeru_core::rename_initiative(store, old, new).map_err(to_mcp)?;
    let mut msg = format!(
        "renamed `{old}` → `{new}` locally ({} node(s), {} edge(s))",
        stats.nodes, stats.edges
    );

    if also_cloud {
        match cloud {
            Some(c) => {
                let (code, resp) = c.rename_initiative(old, new).await.map_err(|e| {
                    McpError::internal_error(format!("cloud rename failed: {e}"), None)
                })?;
                if (200..300).contains(&code) {
                    msg.push_str("\nalso renamed in the shared cloud (team-wide).");
                } else {
                    msg.push_str(&format!(
                        "\ncloud rename FAILED ({code}): {resp} — local and cloud now diverge for this initiative."
                    ));
                }
            }
            None => msg.push_str("\n(cloud requested but not configured — local only)"),
        }
    } else {
        msg.push_str(
            "\n(local only; pass cloud=true to also rename it in the shared cloud — affects the whole team)",
        );
    }
    Ok(text(&msg))
}

pub async fn delete_initiative(
    store: &Store,
    cloud: Option<&CloudClient>,
    name: &str,
    also_cloud: bool,
) -> Result<CallToolResult, McpError> {
    let stats = kaeru_core::delete_initiative(store, name).map_err(to_mcp)?;
    let mut msg = format!(
        "deleted `{name}` locally ({} forgotten, {} kept in other initiatives)",
        stats.forgotten, stats.unscoped
    );

    if also_cloud {
        match cloud {
            Some(c) => {
                let (code, resp) = c.delete_initiative(name).await.map_err(|e| {
                    McpError::internal_error(format!("cloud delete failed: {e}"), None)
                })?;
                if (200..300).contains(&code) {
                    msg.push_str("\nalso deleted from the shared cloud (team-wide).");
                } else {
                    msg.push_str(&format!("\ncloud delete FAILED ({code}): {resp}"));
                }
            }
            None => msg.push_str("\n(cloud requested but not configured — local only)"),
        }
    } else {
        msg.push_str(
            "\n(local only; pass cloud=true to delete it from the shared cloud too — removes it for the whole team)",
        );
    }
    Ok(text(&msg))
}
