//! Diagnostic tools: `lint`, `reflect`.

use kaeru_core::Store;
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{brief_suffix, text, to_mcp, with_initiative};

/// How many shared-node edges the reflection lists before summarising the
/// rest. A well-connected shared initiative has hundreds, and the point of
/// the section is the condition, not the enumeration.
const SHARED_EDGE_CAP: usize = 10;

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
        if !report.supersedes_conflicts.is_empty() {
            out.push_str(&format!(
                "\ncontradictory supersessions ({}) — `supersedes` runs one way, from the \
                 replacement to what it replaced, so a pair carrying both says each one replaced \
                 the other and nothing can say which is current. `unlink` the wrong direction:\n",
                report.supersedes_conflicts.len()
            ));
            for (a, b) in &report.supersedes_conflicts {
                out.push_str(&format!(
                    "  - {}{} ⇄ {}{}\n",
                    a,
                    brief_suffix(store, a),
                    b,
                    brief_suffix(store, b)
                ));
            }
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
            + r.archivable.len()
            + r.shared_edges.len()
            + r.duplicate_initiatives.len()
            + r.shared.len()
            + r.overdue_tasks.len()
            + r.contested_claims.len()
            + r.orphan_core.len();
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
            "orphan core nodes — load in no session",
            "`core` with no initiative: injected via `awake <initiative>`, so these load nowhere. \
             `attach <name> <initiative>`, or `layer <name> warm` to demote",
            &r.orphan_core,
        );
        push_section(
            &mut out,
            store,
            "overdue tasks",
            "past their `due:` date — `done` when finished, else `set_status` to move them",
            &r.overdue_tasks,
        );
        push_section(
            &mut out,
            store,
            "claims whose text already answers them",
            "the body says the verdict, the tag still says open — settle it with `confirm` / \
             `refute` / `inconclusive`",
            &r.contested_claims,
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
        // Cortex loads whole and uncapped in every session, so a section that
        // proposes to grow it has to say by how much. Printed as a count
        // alone, "86 cortex candidates" reads as a work-list to apply, and
        // applying it turned a 30-node cortex into a 116-node one (#76).
        let after = r.cortex_size + r.cortex_candidates.len();
        push_section(
            &mut out,
            store,
            "cortex candidates — review, not a batch",
            &format!(
                "settled and still referenced. `settle`/`cite` the ones that will be READ again \
                 — cortex loads whole every session, so all of them is cortex {} → {after}. \
                 Set `layer=core` only if it must always load",
                r.cortex_size
            ),
            &r.cortex_candidates,
        );
        push_section(
            &mut out,
            store,
            "delivered — nothing references them",
            "finished work the graph no longer points at. `layer <name> cold` — out of `awake`, \
             still there via `surface layers=cold`. `settle` these only if you expect to read \
             them again",
            &r.archivable,
        );
        push_section(
            &mut out,
            store,
            "shared (cloud)",
            "ASK THE USER before any re-share or edge rebalance — never touch the cloud yourself",
            &r.shared,
        );
        // Fragmentation had a cure and no diagnosis: `attach_node` is
        // documented verbatim as "the repair primitive for initiative
        // fragmentation" and was called seven times in a 6,003-call corpus,
        // while three projects sat split across two or three names each (#86).
        if !r.duplicate_initiatives.is_empty() {
            out.push_str(&format!(
                "\ninitiatives that may be one thing ({}) — memory split across names is invisible \
                 until something looks. `merge_initiative <source> <target>` re-homes every node \
                 and edge in one step, or `attach <node> <initiative>` for a few. Sub-scoping \
                 (`proj` beside `proj-api`) looks the same from here and is fine — you decide:\n",
                r.duplicate_initiatives.len()
            ));
            for (a, b, why) in &r.duplicate_initiatives {
                out.push_str(&format!("  - `{a}` · `{b}` — {why}\n"));
            }
        }
        // The section that used to advise about "edge rebalance" while
        // computing nothing about edges (#85). It still cannot be a
        // diagnosis — only the cloud knows what the cloud holds — so it says
        // what it is: the set that can be stale, and the one call that
        // reconciles it.
        if !r.shared_edges.is_empty() {
            out.push_str(&format!(
                "\nedges between shared nodes ({}) — the cloud may not hold these. Graph edits \
                 propagate as of 0.7.2; anything linked before that, or refused by the cloud, is \
                 still local only. `share <either endpoint>` re-pushes the node and its edges:\n",
                r.shared_edges.len()
            ));
            for (src, dst, edge_type) in r.shared_edges.iter().take(SHARED_EDGE_CAP) {
                out.push_str(&format!(
                    "  - {}{} -[{edge_type}]-> {}{}\n",
                    src,
                    brief_suffix(store, src),
                    dst,
                    brief_suffix(store, dst)
                ));
            }
            if r.shared_edges.len() > SHARED_EDGE_CAP {
                out.push_str(&format!(
                    "  … and {} more\n",
                    r.shared_edges.len() - SHARED_EDGE_CAP
                ));
            }
        }
        out.push_str(
            "\n↳ work it: link/relink and `reweight` where structure shifted, `rechain` stale \
             trails, promote settled facts into cortex. The last two sections are candidates to \
             judge one at a time, not a list to apply — cortex is the expensive tier. Cloud items \
             are the user's call: surface a recommendation and wait.",
        );
        Ok(text(&out))
    })
}
