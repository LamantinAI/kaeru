//! Read-side tools: `recall`, `drill`, `neighbours`, `trace`, `search`,
//! `ideas`, `outcomes`, `tagged`, `between`.

use kaeru_core::Store;
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{
    AT_FULLTEXT_HINT_MANY, at_fulltext_hint, body_truncated, chain_membership_hint, history_hint,
    name_not_found_message, parse_edge_types, recall_read_hint, render_briefs, render_neighbours,
    render_summary, resolve_name_or_id, search_deepen_hint, search_empty_hint, text, to_mcp,
    was_revised, with_initiative,
};

pub fn recall(
    store: &Store,
    name: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        match kaeru_core::recall_id_by_name(store, name).map_err(to_mcp)? {
            // Just an opaque id lands here — point the agent at how to read it.
            Some(id) => {
                let hint = chain_membership_hint(store, &id);
                Ok(text(&format!("{id}{}{hint}", recall_read_hint(name))))
            }
            // A miss carries the same four-branch recovery as `resolve_name`'s
            // error, so the agent switches strategy instead of guessing another
            // name (the recall retry-loop in the #84 audit).
            None => Ok(text(&name_not_found_message(store, name))),
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
        out.push_str(&chain_membership_hint(store, &id));
        Ok(text(&out))
    })
}

pub fn neighbours(
    store: &Store,
    name: &str,
    edge_type: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name)?;
        let types = parse_edge_types(edge_type)?;
        let ns = kaeru_core::neighbours(store, &id, &types).map_err(to_mcp)?;
        Ok(text(&render_neighbours(name, &ns)))
    })
}

pub fn trace(
    store: &Store,
    name: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name)?;
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
            return Ok(text(&format!("(no matches){}", search_empty_hint(query))));
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
        if let Some(top) = hits.first() {
            out.push_str(&search_deepen_hint(&top.name));
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
        let mut out = render_briefs(&format!("tagged `{tag}`"), &briefs);
        // An exact-match miss reads exactly like an empty vault, and the first
        // one is what taught agents to stop reaching for this verb. Turn it
        // into a menu of what the scope actually carries.
        if briefs.is_empty() {
            out.push_str(&tagged_miss_hint(store, tag));
        }
        Ok(text(&out))
    })
}

/// `↳ …` for a `tagged` that matched nothing: the near tags that do exist,
/// or — when nothing is close — the verb that searches text instead of tags.
fn tagged_miss_hint(store: &Store, tag: &str) -> String {
    // Match on the value, not the family: someone asking for `topic:figma`
    // wants `topic:figma-макет`, and searching the whole string would only
    // ever find the `topic:` prefix it already knows about.
    let fragment = tag.split_once(':').map(|(_, v)| v).unwrap_or(tag);
    let near = kaeru_core::tags_like(store, fragment).unwrap_or_default();
    let near: Vec<&(String, usize)> = near.iter().filter(|(t, _)| t != tag).collect();
    if near.is_empty() {
        return format!(
            "\n↳ no tag like that in scope — search the text instead: `search {fragment}*`."
        );
    }
    let listed = near
        .iter()
        .map(|(t, n)| format!("`{t}` ({n})"))
        .collect::<Vec<_>>()
        .join(" · ");
    format!("\n↳ nothing carries that exact tag. In scope: {listed}.")
}

pub fn between(
    store: &Store,
    a: &str,
    b: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let a_id = resolve_name_or_id(store, a)?;
        let b_id = resolve_name_or_id(store, b)?;
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

    use super::{drill, neighbours, recall, search, tagged};
    use kaeru_core::EdgeType;

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
    fn neighbours_shows_an_incoming_contradiction_that_drill_misses() {
        // The #84 reproduction: a node reached only by an INCOMING contradicts.
        // `drill` reports it attached to nothing; `neighbours` shows the edge,
        // labelled and directed.
        let store = store_t();
        let a = write(&store, "finding-a", "the first conclusion");
        let b = write(&store, "finding-b", "the later one that overturns it");
        kaeru_core::link(&store, &b, &a, EdgeType::Contradicts).expect("link");

        let out = text_of(neighbours(&store, "finding-a", None, Some("t")).unwrap());
        assert!(out.contains("finding-b"), "the neighbour is named:\n{out}");
        assert!(
            out.contains("←[contradicts]—"),
            "the incoming contradicts is labelled and directed:\n{out}"
        );
        assert_ne!(a, b);

        // drill, over the very same node, still sees nothing.
        let drilled = text_of(drill(&store, "finding-a", Some("t")).unwrap());
        assert!(
            drilled.contains("no drill-down children"),
            "drill is blind to the contradiction it cannot follow:\n{drilled}"
        );
    }

    #[test]
    fn neighbours_type_filter_rejects_an_unknown_type() {
        let store = store_t();
        write(&store, "solo", "body");
        // An invalid filter surfaces the closed vocabulary rather than 0 rows.
        let err = neighbours(&store, "solo", Some("related_to"), Some("t"));
        assert!(err.is_err(), "unknown edge type is an error, not empty");
    }

    #[test]
    fn a_scoped_miss_on_an_untagged_node_names_the_trap() {
        // #81: a node written with no initiative exists but loads in no session.
        // A scoped read must say THAT, not a bare "not found" that sends the
        // agent hunting for something it wrote.
        let store = Store::open_in_memory().expect("open");
        kaeru_core::write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "orphan-core",
            "body",
        )
        .expect("write"); // no initiative set → attached to none
        store.use_initiative("t");
        let err = drill(&store, "orphan-core", Some("t")).unwrap_err();
        let msg = format!("{err:?}");
        assert!(
            msg.contains("belongs to no initiative"),
            "the miss names the real problem:\n{msg}"
        );
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
        assert!(
            missing.contains("anywhere") && missing.contains("search"),
            "a miss now carries recovery, not a bare (not found):\n{missing}"
        );
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

    /// A miss is exactly where an agent concludes the memory is empty and
    /// stops. It has to leave holding the ways to widen the query.
    #[test]
    fn a_search_miss_hands_back_the_widenings() {
        let store = store_t();
        write(&store, "unrelated", "nothing to do with it");
        let out = text_of(search(&store, "zzzznotthere", 10, Some("t")).unwrap());
        assert!(out.starts_with("(no matches)"), "still says so: {out}");
        assert!(
            out.contains("`search \"zzzznotthere*\"`"),
            "offers the prefix form of this very query: {out}"
        );
        assert!(out.contains("tagged topic:"), "and browsing by tag: {out}");
    }

    /// `search` returns excerpts; the two verbs that go further had no channel
    /// anywhere in the agent's path, so the hit list names them.
    #[test]
    fn a_search_hit_points_at_drill_and_trace() {
        let store = store_t();
        write(&store, "alphahit", "alphaquery matters here");
        let out = text_of(search(&store, "alphaquery", 10, Some("t")).unwrap());
        assert!(
            out.contains("`drill alphahit`") && out.contains("`trace alphahit`"),
            "both deepen verbs, named on the top hit: {out}"
        );
    }

    /// A node inside a saved trail has to say so — otherwise the trail is only
    /// reachable by an agent that already thought to ask for it.
    #[test]
    fn a_node_inside_a_trail_says_so_on_drill() {
        let store = store_t();
        let a = write(&store, "start", "start");
        let b = write(&store, "decision", "decision");
        kaeru_core::link_with_weight(&store, &a, &b, EdgeType::RefersTo, 0.9).expect("link");
        kaeru_core::create_chain(&store, &a, &b, Some("the-trail"), None)
            .expect("chain")
            .expect("path exists");

        let out = text_of(drill(&store, "start", Some("t")).unwrap());
        assert!(
            out.contains("part of a saved trail: `the-trail` (step 1/2)"),
            "names the trail and the step: {out}"
        );
        assert!(out.contains("`why the-trail`"), "and how to read it: {out}");
    }

    /// The hint only appears where it teaches — a node in no chain stays quiet.
    #[test]
    fn a_node_in_no_trail_stays_quiet() {
        let store = store_t();
        write(&store, "lonely", "x");
        let out = text_of(drill(&store, "lonely", Some("t")).unwrap());
        assert!(!out.contains("saved trail"), "no trail, no hint: {out}");
    }

    /// The failure that killed the verb: an exact-match miss reads exactly
    /// like an empty vault. It has to say what the scope does carry.
    #[test]
    fn a_tag_miss_offers_the_near_tags_that_exist() {
        let store = store_t();
        write(
            &store,
            "the-note",
            "правим figma-макет и ещё раз figma-макет",
        );
        let out = text_of(tagged(&store, "topic:figma-макет", Some("t")).unwrap());
        assert!(out.contains("(1)"), "the exact tag works: {out}");

        // A near miss on the same word — the literal #59 report.
        let miss = text_of(tagged(&store, "topic:figma-file", Some("t")).unwrap());
        assert!(miss.contains("(0)"), "still an honest zero: {miss}");
        assert!(
            miss.contains("topic:figma-макет"),
            "and names what is there: {miss}"
        );
    }

    /// Nothing close means nothing to offer — point at the verb that reads
    /// text rather than inventing a suggestion.
    #[test]
    fn a_tag_miss_with_nothing_close_points_at_search() {
        let store = store_t();
        write(&store, "the-note", "unrelated content entirely");
        let out = text_of(tagged(&store, "topic:zzzznope", Some("t")).unwrap());
        assert!(out.contains("`search zzzznope*`"), "{out}");
    }

    /// The compound fix, end to end through a real write: the word inside a
    /// compound is reachable as a tag of its own.
    #[test]
    fn a_word_inside_a_compound_is_taggable() {
        let store = store_t();
        write(
            &store,
            "the-note",
            "правим figma-макет и ещё раз figma-макет",
        );
        let out = text_of(tagged(&store, "topic:figma", Some("t")).unwrap());
        assert!(out.contains("(1)"), "topic:figma now resolves: {out}");
    }

    /// Case 1 — the agent had the name right; only the scope was wrong. This
    /// is the dominant organic failure: `link` filtering resolution by
    /// initiative and reporting "does not exist".
    #[test]
    fn a_name_that_lives_elsewhere_says_where() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("other");
        write(&store, "the-decision", "made over there");
        store.use_initiative("t");

        let err = drill(&store, "the-decision", Some("t"))
            .unwrap_err()
            .to_string();
        assert!(err.contains("it lives in `other`"), "{err}");
        assert!(
            err.contains("initiative=other"),
            "and how to get there: {err}"
        );
    }

    /// Case 2 — a misremembered name. The audit found the same nonexistent
    /// name re-tried across three sessions, because nothing ever corrected it.
    #[test]
    fn a_misremembered_name_gets_a_did_you_mean() {
        let store = store_t();
        write(
            &store,
            "auth-token-leak",
            "the token leaked through the proxy",
        );
        let err = drill(&store, "auth-token-leaks", Some("t"))
            .unwrap_err()
            .to_string();
        assert!(err.contains("did you mean"), "{err}");
        assert!(err.contains("auth-token-leak"), "{err}");
    }

    /// Case 3 — genuinely absent. Nothing to suggest, so point at the verb
    /// that searches text rather than names.
    #[test]
    fn a_name_that_exists_nowhere_says_so_and_offers_search() {
        let store = store_t();
        write(&store, "unrelated", "nothing alike");
        let err = drill(&store, "zzzznotathing", Some("t"))
            .unwrap_err()
            .to_string();
        assert!(err.contains("anywhere at NOW"), "{err}");
        assert!(err.contains("`search zzzznotathing*`"), "{err}");
    }

    /// #74 end to end: a name of exactly 36 UTF-8 bytes with a hyphen at
    /// character 8 used to be read as a UUID and passed into the query
    /// verbatim, so `drill` / `at` / `history` denied a node that `recall` and
    /// `search` found. Three verbs, three answers, one live node.
    #[test]
    fn a_36_byte_non_ascii_name_still_resolves() {
        let store = store_t();
        let name = "änderung-prüfung-fehleranalyse2026";
        assert_eq!(name.len(), 36, "the shape that used to break");
        write(&store, name, "control node");

        let out = text_of(drill(&store, name, Some("t")).unwrap());
        assert!(out.contains(name), "drill reads it by name: {out}");

        let by_recall = text_of(recall(&store, name, Some("t")).unwrap());
        assert!(
            !by_recall.contains("(not found)"),
            "and recall agrees with it: {by_recall}"
        );
    }
}
