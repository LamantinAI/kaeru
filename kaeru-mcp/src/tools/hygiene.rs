//! The `hygiene` tool: what the sweep would do, when it last ran, and a way
//! to run it now.

use kaeru_core::Store;
use kaeru_core::hygiene::{self, HygieneAction};
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::hygiene::HygieneScheduler;
use crate::utils::{text, to_mcp};

/// Reports hygiene status for an initiative, or runs a pass when `force`.
///
/// Not scoped through `with_initiative`: a forced pass takes the store guard
/// per batch itself, and the guard is not reentrant.
pub fn hygiene(
    store: &Store,
    scheduler: &HygieneScheduler,
    initiative: &str,
    force: bool,
) -> Result<CallToolResult, McpError> {
    if force {
        let report = scheduler.run_now(initiative).map_err(to_mcp)?;
        let body = match report {
            Some(report) => {
                let mut out = report.headline();
                if report.stopped_early {
                    out.push_str("\n(stopped early — the daemon is shutting down)");
                }
                for line in &report.lines {
                    out.push_str(&format!("\n  • {line}"));
                }
                out
            }
            None => format!("nothing was due for `{initiative}`"),
        };
        return Ok(text(&body));
    }

    let state = hygiene::state(store, initiative).map_err(to_mcp)?;
    let nodes = hygiene::node_count(store, initiative).map_err(to_mcp)?;
    let core = hygiene::core_count(store, initiative).map_err(to_mcp)?;
    let due = hygiene::due(store, initiative).map_err(to_mcp)?;
    let candidates = hygiene::collect(store, initiative).map_err(to_mcp)?;

    let (archive, demote, promote) =
        candidates
            .iter()
            .fold((0, 0, 0), |(a, d, p), c| match c.action {
                HygieneAction::Archive => (a + 1, d, p),
                HygieneAction::DemoteFromCore => (a, d + 1, p),
                HygieneAction::Promote => (a, d, p + 1),
            });

    let mut out = format!("hygiene · `{initiative}`\n");
    out.push_str(&format!("  nodes: {nodes} · core: {core}\n"));
    out.push_str(&match state.last_run_at {
        at if at <= 0.0 => "  last pass: never\n".to_string(),
        at => {
            let ago = (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0) as f64
                - at)
                .max(0.0);
            format!(
                "  last pass: {:.0}h ago, at {} nodes\n",
                ago / 3600.0,
                state.nodes_at_last_run
            )
        }
    });
    out.push_str(&match &due {
        Some(reason) => format!("  due: yes — {reason}\n"),
        None => "  due: no\n".to_string(),
    });
    if scheduler.is_disabled() {
        out.push_str("  ⚠ disabled by KAERU_MCP_HYGIENE_DISABLE — nothing will run\n");
    } else {
        out.push_str(&format!(
            "  passes since this daemon started: {}\n",
            scheduler.passes_started()
        ));
    }
    out.push_str(&format!(
        "  would move: {archive} to archive · {demote} out of core · {promote} promoted\n"
    ));
    for candidate in candidates.iter().take(7) {
        out.push_str(&format!(
            "    {} {} → {}: {} ({})\n",
            candidate.action.as_str(),
            candidate.from.as_str(),
            candidate.to.as_str(),
            candidate.name,
            candidate.reason
        ));
    }
    if candidates.len() > 7 {
        out.push_str(&format!("    … +{} more\n", candidates.len() - 7));
    }
    out.push_str(
        "↳ passes run on their own when writes accumulate, when core grows, or on the sweep \
         timer. `force=true` runs one now. Every move is a layer change and reverses with \
         `layer <name> <old>`.",
    );
    Ok(text(&out))
}
