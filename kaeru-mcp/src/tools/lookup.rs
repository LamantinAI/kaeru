//! Read-side tools: `recall`, `drill`, `trace`, `search`, `ideas`,
//! `outcomes`, `tagged`, `between`.

use kaeru_core::Store;
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{
    AT_FULLTEXT_HINT_MANY, at_fulltext_hint, body_truncated, history_hint, recall_read_hint,
    render_briefs, render_summary, resolve_name, resolve_name_or_id, text, to_mcp, was_revised,
    with_initiative,
};

pub fn recall(
    store: &Store,
    name: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        match kaeru_core::recall_id_by_name(store, name).map_err(to_mcp)? {
            // Just an opaque id lands here — point the agent at how to read it.
            Some(id) => Ok(text(&format!("{id}{}", recall_read_hint(name)))),
            None => Ok(text("(not found)")),
        }
    })
}

pub fn drill(
    store: &Store,
    name: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name)?;
        let view = kaeru_core::summary_view(store, &id).map_err(to_mcp)?;
        let mut out = render_summary(&view);
        // Deepen-lane edges: only where they teach.
        let truncated = body_truncated(view.root.body_excerpt.as_deref())
            || view
                .children
                .iter()
                .any(|c| body_truncated(c.body_excerpt.as_deref()));
        if truncated {
            out.push_str(&at_fulltext_hint(&view.root.name));
        }
        if was_revised(store, &id) {
            out.push_str(&history_hint(&view.root.name));
        }
        Ok(text(&out))
    })
}

pub fn trace(
    store: &Store,
    name: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name(store, name)?;
        let ancestors = kaeru_core::recollect_provenance(store, &id).map_err(to_mcp)?;
        if ancestors.is_empty() {
            return Ok(text("(no provenance)"));
        }
        let mut out = format!("provenance ({}):\n", ancestors.len());
        for b in &ancestors {
            out.push_str(&format!("  - {} ({}) — {}\n", b.name, b.node_type, b.id));
            if let Some(e) = &b.body_excerpt {
                out.push_str(&format!("    {e}\n"));
            }
        }
        Ok(text(&out))
    })
}

pub fn search(
    store: &Store,
    query: &str,
    limit: usize,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let hits = kaeru_core::fuzzy_recall(store, query, limit).map_err(to_mcp)?;
        if hits.is_empty() {
            return Ok(text("(no matches)"));
        }
        let mut out = format!("matches ({}):\n", hits.len());
        let mut any_truncated = false;
        for b in &hits {
            out.push_str(&format!("  - {} ({}) — {}\n", b.name, b.node_type, b.id));
            if let Some(e) = &b.body_excerpt {
                out.push_str(&format!("    {e}\n"));
                any_truncated |= body_truncated(Some(e));
            }
        }
        if any_truncated {
            out.push_str(AT_FULLTEXT_HINT_MANY);
        }
        Ok(text(&out))
    })
}

pub fn ideas(store: &Store, initiative: Option<&str>) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let briefs = kaeru_core::recollect_idea(store).map_err(to_mcp)?;
        Ok(text(&render_briefs("ideas", &briefs)))
    })
}

pub fn outcomes(store: &Store, initiative: Option<&str>) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let briefs = kaeru_core::recollect_outcome(store).map_err(to_mcp)?;
        Ok(text(&render_briefs("outcomes", &briefs)))
    })
}

pub fn tagged(
    store: &Store,
    tag: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let briefs = kaeru_core::tagged(store, tag).map_err(to_mcp)?;
        Ok(text(&render_briefs(&format!("tagged `{tag}`"), &briefs)))
    })
}

pub fn between(
    store: &Store,
    a: &str,
    b: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let a_id = resolve_name(store, a)?;
        let b_id = resolve_name(store, b)?;
        let edges = kaeru_core::between(store, &a_id, &b_id).map_err(to_mcp)?;
        if edges.is_empty() {
            return Ok(text(&format!("(no edges between {a} and {b})")));
        }
        let mut out = format!("edges ({}):\n", edges.len());
        for e in &edges {
            if e.a_to_b {
                out.push_str(&format!("  {a} —[{}]→ {b}\n", e.edge_type));
            } else {
                out.push_str(&format!("  {a} ←[{}]— {b}\n", e.edge_type));
            }
        }
        Ok(text(&out))
    })
}

#[cfg(test)]
mod tests {
    use kaeru_core::{EpisodeKind, Significance, Store};
    use rmcp::model::CallToolResult;

    use super::{drill, recall, search};

    fn store_t() -> Store {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        store
    }

    fn text_of(r: CallToolResult) -> String {
        r.content
            .iter()
            .filter_map(|c| c.as_text().map(|t| t.text.clone()))
            .collect::<Vec<_>>()
            .join("")
    }

    fn write(store: &Store, name: &str, body: &str) -> String {
        kaeru_core::write_episode(
            store,
            EpisodeKind::Observation,
            Significance::Low,
            name,
            body,
        )
        .expect("write")
    }

    #[test]
    fn drill_hints_at_when_body_is_truncated() {
        let store = store_t();
        write(&store, "long-note", &"word ".repeat(400)); // beyond any excerpt cap
        let out = text_of(drill(&store, "long-note", Some("t")).unwrap());
        assert!(
            out.contains("full text: `at long-note`"),
            "truncated drill points at `at`:\n{out}"
        );
    }

    #[test]
    fn drill_stays_quiet_on_a_short_untruncated_body() {
        let store = store_t();
        write(&store, "short-note", "hi");
        let out = text_of(drill(&store, "short-note", Some("t")).unwrap());
        assert!(
            !out.contains("full text: `at"),
            "no at-hint when nothing was cut:\n{out}"
        );
        assert!(
            !out.contains("timeline: `history"),
            "no history-hint for a never-revised node:\n{out}"
        );
    }

    #[test]
    fn drill_hints_history_after_a_revision() {
        let store = store_t();
        let id = write(&store, "v1", "first");
        std::thread::sleep(std::time::Duration::from_millis(1100)); // cross validity second
        kaeru_core::improve(&store, &id, "v2", "second").unwrap();
        let out = text_of(drill(&store, "v2", Some("t")).unwrap());
        assert!(
            out.contains("timeline: `history v2`"),
            "revised drill points at `history`:\n{out}"
        );
    }

    #[test]
    fn recall_points_at_how_to_read_the_id() {
        let store = store_t();
        write(&store, "findme", "x");
        let found = text_of(recall(&store, "findme", Some("t")).unwrap());
        assert!(
            found.contains("that's the id"),
            "recall teaches at/drill:\n{found}"
        );
        let missing = text_of(recall(&store, "nope", Some("t")).unwrap());
        assert_eq!(missing, "(not found)", "no hint on a miss");
    }

    #[test]
    fn search_hints_at_when_excerpts_are_truncated() {
        let store = store_t();
        write(
            &store,
            "hit",
            &format!("alphaquery {}", "word ".repeat(400)),
        );
        let out = text_of(search(&store, "alphaquery", 10, Some("t")).unwrap());
        assert!(
            out.contains("read one in full"),
            "truncated search points at `at`:\n{out}"
        );
    }
}
