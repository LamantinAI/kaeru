//! Diagnostic tools: `lint`, `reflect`.

use kaeru_core::Store;
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{brief_suffix, text, to_mcp, with_initiative};

/// Appends one labelled section (title, how-to, then the ids with names) to
/// the reflection output. No-op when `ids` is empty.
fn push_section(out: &mut String, store: &Store, title: &str, how: &str, ids: &[String]) {
    if ids.is_empty() {
        return;
    }
    out.push_str(&format!("\n{title} ({}) — {how}:\n", ids.len()));
    for id in ids {
        out.push_str(&format!("  - {id}{}\n", brief_suffix(store, id)));
    }
}

pub fn lint(store: &Store, initiative: Option<&str>) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let report = kaeru_core::lint(store).map_err(to_mcp)?;
        let mut out = format!("orphans ({}):\n", report.orphans.len());
        for id in &report.orphans {
            out.push_str(&format!("  - {id}{}\n", brief_suffix(store, id)));
        }
        out.push('\n');
        out.push_str(&format!(
            "unresolved reviews ({}):\n",
            report.unresolved_reviews.len()
        ));
        for id in &report.unresolved_reviews {
            out.push_str(&format!("  - {id}{}\n", brief_suffix(store, id)));
        }
        out.push('\n');
        out.push_str(&format!(
            "dangling edges ({}) — an endpoint was retracted; re-point at its successor or unlink:\n",
            report.dangling_edges.len()
        ));
        for (src, dst, edge_type) in &report.dangling_edges {
            out.push_str(&format!("  - {src} -[{edge_type}]-> {dst}\n"));
        }
        Ok(text(&out))
    })
}

/// The reflection pass: a computed maintenance work-list paired with how to
/// act on each part. The store works out *what* needs tending; the lines here
/// say *how* — including that cloud changes are escalated to the user, never
/// done automatically.
pub fn reflect(store: &Store, initiative: Option<&str>) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let r = kaeru_core::reflect(store).map_err(to_mcp)?;
        let total = r.orphans.len()
            + r.open_reviews.len()
            + r.stale_chains.len()
            + r.cortex_candidates.len()
            + r.shared.len();
        if total == 0 {
            return Ok(text(
                "reflection: store is tidy — nothing to tend right now.",
            ));
        }

        let mut out = format!("reflection — {total} item(s) to tend:\n");
        push_section(
            &mut out,
            store,
            "orphans",
            "`search` for relatives and `link`, else `forget`",
            &r.orphans,
        );
        push_section(
            &mut out,
            store,
            "open reviews",
            "`resolve` or `refute` the contradiction",
            &r.open_reviews,
        );
        push_section(
            &mut out,
            store,
            "stale chains",
            "`rechain` to recompute the trail the graph outgrew",
            &r.stale_chains,
        );
        push_section(
            &mut out,
            store,
            "cortex candidates",
            "settled — `settle`/`cite` into cortex; set `layer=core` only if it must always load",
            &r.cortex_candidates,
        );
        push_section(
            &mut out,
            store,
            "shared (cloud)",
            "ASK THE USER before any re-share or edge rebalance — never touch the cloud yourself",
            &r.shared,
        );
        out.push_str(
            "\n↳ work it: link/relink and `reweight` where structure shifted, `rechain` stale \
             trails, promote settled facts into cortex. Cloud items are the user's call — surface \
             a recommendation and wait.",
        );
        Ok(text(&out))
    })
}
