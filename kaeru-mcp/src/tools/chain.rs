//! Knowledge-chain tools: `chain` (save the reasoning trail between two
//! nodes), `why` (read it back — a chain's steps, or the chain a node belongs
//! to), `rechain` (refresh a trail the graph has outgrown), and `path` (preview
//! the route without saving anything).
//!
//! `why` replaces the former `chains` + `read_chain` pair — see its own docs
//! for why one polymorphic verb beat two.

use kaeru_core::Store;
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{resolve_name_or_id, text, to_mcp, ts_suffix, with_initiative};

/// Materializes the shortest weighted path `from → to` as a saved chain.
pub fn chain(
    store: &Store,
    from: &str,
    to: &str,
    name: Option<&str>,
    summary: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let from_id = resolve_name_or_id(store, from)?;
        let to_id = resolve_name_or_id(store, to)?;
        match kaeru_core::create_chain(store, &from_id, &to_id, name, summary).map_err(to_mcp)? {
            Some(outcome) => {
                let members = kaeru_core::read_chain(store, &outcome.id).map_err(to_mcp)?;
                let trail = members
                    .iter()
                    .map(|m| m.name.as_str())
                    .collect::<Vec<_>>()
                    .join(" → ");
                let verb = if outcome.reused {
                    "reused existing chain"
                } else {
                    "chain saved"
                };
                Ok(text(&format!(
                    "{verb} ({} nodes): {trail}\nid: {}",
                    members.len(),
                    outcome.id
                )))
            }
            None => Ok(text(&format!(
                "no path from `{from}` to `{to}` — nothing to chain"
            ))),
        }
    })
}

/// `why` — the read side of the chain family, one verb for both questions a
/// reasoning trail answers.
///
/// Replaces `chains` + `read_chain`, which had zero calls between them across
/// the entire usage history. Part of the reason was circular: the only place
/// `read_chain` was advertised was the output of `chains`, and nobody called
/// `chains` either. A single polymorphic verb has one entry point, and a name
/// that says what a chain is *for* — a chain is the state → reasoning →
/// decision story a fresh agent reads to understand WHY, not just WHAT.
///
/// Dispatch on what the input turns out to be:
/// - a chain → its ordered steps;
/// - a node in exactly one chain → that chain's steps directly, because
///   making the caller take a second turn to reach the only possible answer
///   is the friction that killed the pair;
/// - a node in several → the menu, triaged by name + summary.
pub fn why(
    store: &Store,
    name_or_id: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name_or_id)?;
        let is_chain = kaeru_core::node_brief_by_id(store, &id)
            .ok()
            .flatten()
            .is_some_and(|b| b.node_type == "chain");

        if is_chain {
            return Ok(text(&render_trail(store, &id, name_or_id)?));
        }

        let chains = kaeru_core::chains_of(store, &id).map_err(to_mcp)?;
        match chains.len() {
            0 => Ok(text(&format!(
                "`{name_or_id}` is in no chain yet — no saved reasoning leads here.\n\
                 ↳ once a line of work runs observation→decision, \
                 `chain from to --summary` saves that trail."
            ))),
            // One chain is the answer, not a menu.
            1 => {
                let only = &chains[0];
                let mut out = format!("`{name_or_id}` is in one chain:\n\n");
                out.push_str(&render_trail(store, &only.id, &only.name)?);
                Ok(text(&out))
            }
            n => {
                let mut out = format!("`{name_or_id}` is in {n} chains:\n");
                for ch in &chains {
                    out.push_str(&format!("  - {} — {}\n", ch.name, ch.id));
                    if let Some(s) = &ch.body_excerpt {
                        out.push_str(&format!("    {s}\n"));
                    }
                }
                out.push_str("\n↳ triage by name + summary, then `why <name>` for the full trail.");
                Ok(text(&out))
            }
        }
    })
}

/// Renders a chain's ordered members — shared by both `why` branches.
fn render_trail(store: &Store, chain_id: &str, label: &str) -> Result<String, McpError> {
    let members = kaeru_core::read_chain(store, &chain_id.to_string()).map_err(to_mcp)?;
    if members.is_empty() {
        return Ok(format!("chain `{label}` has no members"));
    }
    let mut out = format!("chain `{label}` ({} steps):\n", members.len());
    for (i, m) in members.iter().enumerate() {
        out.push_str(&format!(
            "{}. {} ({}) — {}{}\n",
            i + 1,
            m.name,
            m.node_type,
            m.id,
            ts_suffix(m.ts)
        ));
        if let Some(e) = &m.body_excerpt {
            out.push_str(&format!("   {e}\n"));
        }
    }
    Ok(out)
}

/// Mutates an existing chain so it survives graph changes: with no `to`, it
/// regenerates (recomputes the shortest path between its current endpoints);
/// with `to`, it extends the trail out to that node. The chain keeps its id,
/// name, and summary.
pub fn rechain(
    store: &Store,
    chain: &str,
    to: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let chain_id = resolve_name_or_id(store, chain)?;
        let (action, stats) = match to {
            Some(target) => {
                let to_id = resolve_name_or_id(store, target)?;
                let s = kaeru_core::extend_chain(store, &chain_id, &to_id).map_err(to_mcp)?;
                ("extended", s)
            }
            None => {
                let s = kaeru_core::regenerate_chain(store, &chain_id).map_err(to_mcp)?;
                ("regenerated", s)
            }
        };
        let Some(stats) = stats else {
            return Ok(text(&format!(
                "`{chain}` left unchanged — endpoint unreachable now (no path)"
            )));
        };
        let members = kaeru_core::read_chain(store, &chain_id).map_err(to_mcp)?;
        let trail = members
            .iter()
            .map(|m| m.name.as_str())
            .collect::<Vec<_>>()
            .join(" → ");
        let note = if stats.changed {
            action
        } else {
            "already current"
        };
        Ok(text(&format!(
            "{note} ({} nodes): {trail}\nid: {chain_id}",
            stats.members
        )))
    })
}

/// Computes the shortest weighted path `from → to` without saving it.
pub fn path(
    store: &Store,
    from: &str,
    to: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let from_id = resolve_name_or_id(store, from)?;
        let to_id = resolve_name_or_id(store, to)?;
        let ids = kaeru_core::shortest_path(store, &from_id, &to_id).map_err(to_mcp)?;
        if ids.is_empty() {
            return Ok(text(&format!("no path from `{from}` to `{to}`")));
        }
        let names: Vec<String> = ids
            .iter()
            .map(|id| {
                kaeru_core::node_brief_by_id(store, id)
                    .ok()
                    .flatten()
                    .map(|b| b.name)
                    .unwrap_or_else(|| id.clone())
            })
            .collect();
        Ok(text(&format!(
            "path ({} nodes): {}\nUse `chain {from} {to}` to save it.",
            ids.len(),
            names.join(" → ")
        )))
    })
}

#[cfg(test)]
mod tests {
    use kaeru_core::{EdgeType, EpisodeKind, Significance, Store};
    use rmcp::model::CallToolResult;

    use super::why;

    fn text_of(r: CallToolResult) -> String {
        r.content
            .iter()
            .filter_map(|c| c.as_text().map(|t| t.text.clone()))
            .collect::<Vec<_>>()
            .join("")
    }

    /// Three nodes wired a→b→c, with a saved trail across them.
    fn store_with_a_trail() -> (Store, String) {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        let mk = |n: &str| {
            kaeru_core::write_episode(&store, EpisodeKind::Observation, Significance::Low, n, n)
                .expect("write")
        };
        let (a, b, c) = (mk("start"), mk("middle"), mk("decision"));
        for (x, y) in [(&a, &b), (&b, &c)] {
            kaeru_core::link_with_weight(&store, x, y, EdgeType::RefersTo, 0.9).expect("link");
        }
        let outcome = kaeru_core::create_chain(&store, &a, &c, Some("the-trail"), None)
            .expect("chain")
            .expect("path exists");
        (store, outcome.id)
    }

    /// Given a chain, `why` reads its ordered steps — the old `read_chain`.
    #[test]
    fn a_chain_reads_as_its_steps() {
        let (store, chain_id) = store_with_a_trail();
        let out = text_of(why(&store, &chain_id, Some("t")).unwrap());
        assert!(out.contains("steps"), "renders a trail: {out}");
        assert!(
            out.contains("start") && out.contains("decision"),
            "in order: {out}"
        );
    }

    /// Given a node that sits in exactly one chain, `why` reads that chain
    /// directly rather than handing back a one-item menu — the second call was
    /// the friction that killed the old pair.
    #[test]
    fn a_node_in_one_chain_gets_the_trail_not_a_menu() {
        let (store, _) = store_with_a_trail();
        let out = text_of(why(&store, "middle", Some("t")).unwrap());
        assert!(out.contains("is in one chain"), "says what it found: {out}");
        assert!(out.contains("steps"), "and reads it straight away: {out}");
        assert!(out.contains("start"), "showing the actual trail: {out}");
    }

    /// A node in no chain says so, and names the verb that would create one.
    #[test]
    fn a_node_in_no_chain_points_at_how_to_make_one() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        kaeru_core::write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "lonely",
            "x",
        )
        .expect("write");
        let out = text_of(why(&store, "lonely", Some("t")).unwrap());
        assert!(out.contains("no chain"), "states it: {out}");
        assert!(out.contains("`chain from to"), "names the fix: {out}");
    }
}
