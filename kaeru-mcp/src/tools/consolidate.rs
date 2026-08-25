//! Consolidation: `settle`, `unsettle`, `synthesise`, `supersede`.
//!
//! The tier model — operational work hardening into archival knowledge — is
//! the centre of the design and was, in practice, unused: four outcomes and
//! two summaries across 1245 nodes. The cause was not reluctance but price.
//! Every consolidating verb demanded a re-authored type, name *and* body, so
//! "this note is finished" cost three fields of writing, and the cheap
//! neighbour (demote it to a cold layer) was one. The layer temperature had
//! quietly replaced the tier.
//!
//! So `settle` and `unsettle` now promote **in place**: with no `new_*` the
//! node keeps its name and its body, and only the tier — plus a type derived
//! from what it was — actually moves. The chosen type is always printed back,
//! because a default the agent cannot see is a default it cannot correct.

use kaeru_core::{Error, NodeFull, NodeId, NodeType, Store, Visibility, get_visibility};
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{parse_tier, resolve_name, resolve_name_or_id, text, to_mcp, with_initiative};

/// Reads the node a consolidating verb is about to replace.
///
/// Through `read_node_full`, never `summary_view`: the body is carried over
/// verbatim on a promote-in-place, and an excerpt would silently truncate the
/// node it claims to be preserving.
fn source_record(store: &Store, id: &NodeId, label: &str) -> Result<NodeFull, McpError> {
    kaeru_core::read_node_full(store, id)
        .map_err(to_mcp)?
        .ok_or_else(|| to_mcp(Error::NotFound(format!("node {label:?} not found at NOW"))))
}

/// The successor's type: the caller's choice, or a default derived from the
/// source. `derive` supplies the default — `settled_form` on the way out to
/// the archival tier, identity on the way back in and for a straight
/// replacement.
///
/// Returns the type plus whether it was inferred, so the caller can say so.
fn successor_type(
    asked: Option<&str>,
    current: &str,
    derive: impl Fn(NodeType) -> NodeType,
) -> Result<(NodeType, bool), McpError> {
    match asked {
        Some(t) => Ok((t.parse::<NodeType>().map_err(to_mcp)?, false)),
        None => {
            let parsed = current.parse::<NodeType>().map_err(to_mcp)?;
            Ok((derive(parsed), true))
        }
    }
}

/// `↳ …` naming the defaults a promote-in-place applied, so nothing about the
/// successor is a surprise. Empty when the caller spelled everything out.
fn inherited_hint(
    type_inferred: bool,
    from_type: &str,
    kept_name: bool,
    kept_body: bool,
) -> String {
    let mut parts: Vec<String> = Vec::new();
    if type_inferred {
        parts.push(format!("type from `{from_type}`"));
    }
    if kept_name {
        parts.push("name".to_string());
    }
    if kept_body {
        parts.push("body".to_string());
    }
    if parts.is_empty() {
        return String::new();
    }
    format!(
        "\n↳ carried over: {} — pass `new_type` / `new_name` / `new_body` to change any of them.",
        parts.join(", ")
    )
}

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
    new_type_str: Option<&str>,
    new_name: Option<&str>,
    new_body: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let draft_id = resolve_name_or_id(store, source)?;
        let current = source_record(store, &draft_id, source)?;
        let (new_type, inferred) =
            successor_type(new_type_str, &current.node_type, |t| t.settled_form())?;
        let name = new_name.unwrap_or(&current.name);
        let body = new_body.unwrap_or(current.body.as_deref().unwrap_or_default());

        let shared = predecessor_shared_hint(store, &draft_id)?;
        let id =
            kaeru_core::consolidate_out(store, &draft_id, new_type, name, body).map_err(to_mcp)?;
        let carried = inherited_hint(
            inferred,
            &current.node_type,
            new_name.is_none(),
            new_body.is_none(),
        );
        Ok(text(&format!(
            "settled: {source} → {name} ({}) — {id}{carried}{shared}",
            new_type.as_str()
        )))
    })
}

/// `settle`'s mirror — archival back to operational.
///
/// Named `unsettle` rather than `reopen` because `reopen` read as "reopen a
/// task or a review" and was never once called for what it does. It also
/// keeps well clear of the memory-layer vocabulary: this moves a *tier*, not
/// a temperature.
///
/// No type heuristic here — coming back into the working tier, an idea is
/// still an idea. The default is simply the node's own type.
pub fn unsettle(
    store: &Store,
    source: &str,
    new_type_str: Option<&str>,
    new_name: Option<&str>,
    new_body: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let archival_id = resolve_name_or_id(store, source)?;
        let current = source_record(store, &archival_id, source)?;
        let (new_type, inferred) = successor_type(new_type_str, &current.node_type, |t| t)?;
        let name = new_name.unwrap_or(&current.name);
        let body = new_body.unwrap_or(current.body.as_deref().unwrap_or_default());

        let shared = predecessor_shared_hint(store, &archival_id)?;
        let id = kaeru_core::consolidate_in(store, &archival_id, new_type, name, body)
            .map_err(to_mcp)?;
        let carried = inherited_hint(
            inferred,
            &current.node_type,
            new_name.is_none(),
            new_body.is_none(),
        );
        Ok(text(&format!(
            "unsettled: {source} → {name} ({}) — {id}{carried}{shared}",
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

/// A straight replacement: the successor carries new content, so `new_name`
/// and `new_body` stay required — a supersede that changed neither would only
/// churn the id. `new_type` defaults to the old node's, because replacing a
/// node rarely changes what kind of thing it is.
pub fn supersede(
    store: &Store,
    old: &str,
    new_type_str: Option<&str>,
    new_name: &str,
    new_body: &str,
    tier: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let old_id = resolve_name_or_id(store, old)?;
        let current = source_record(store, &old_id, old)?;
        let (new_type, inferred) = successor_type(new_type_str, &current.node_type, |t| t)?;
        let target_tier = match tier {
            Some(t) => parse_tier(t).map_err(to_mcp)?,
            None => new_type.default_tier(),
        };
        let shared = predecessor_shared_hint(store, &old_id)?;
        let id = kaeru_core::supersedes(store, &old_id, new_type, target_tier, new_name, new_body)
            .map_err(to_mcp)?;
        let carried = inherited_hint(inferred, &current.node_type, false, false);
        Ok(text(&format!(
            "superseded: {old} → {new_name} ({} / {}) — {id}{carried}{shared}",
            new_type.as_str(),
            target_tier.as_str()
        )))
    })
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;

    use kaeru_core::{EpisodeKind, Significance, Store};
    use rmcp::model::CallToolResult;

    use super::{settle, supersede, unsettle};

    fn text_of(r: CallToolResult) -> String {
        r.content
            .iter()
            .filter_map(|c| c.as_text().map(|t| t.text.clone()))
            .collect::<Vec<_>>()
            .join("")
    }

    fn store_t() -> Store {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        store
    }

    fn ep(store: &Store, name: &str, body: &str) -> String {
        kaeru_core::write_episode(
            store,
            EpisodeKind::Observation,
            Significance::Low,
            name,
            body,
        )
        .expect("write")
    }

    /// Validities are whole seconds, so a node created and consolidated inside
    /// the same second carries an assert and a retract that cannot be ordered.
    /// Promote-in-place makes that visible in a way the old signature could
    /// not: predecessor and successor now share a name, so an ambiguous
    /// second means a read by name can still land on the retracted one. Every
    /// test here crosses the boundary first, like the core suite does.
    fn cross_second() {
        sleep(Duration::from_millis(1100));
    }

    /// The successor's id, taken from the tool's own output — the honest handle
    /// right after a consolidation, since predecessor and successor share a
    /// name until the read settles.
    fn id_in(out: &str) -> String {
        out.split(" — ")
            .nth(1)
            .and_then(|rest| rest.split_whitespace().next())
            .map(|s| s.trim_end_matches('\n').to_string())
            .expect("output carries the new id")
    }

    /// The whole point of #55: `settle <name>` and nothing else. The successor
    /// keeps the name, keeps the body in full, and lands in the archival tier.
    #[test]
    fn settle_needs_only_a_name() {
        let store = store_t();
        let long = "the finding, ".repeat(200); // far past any excerpt cap
        ep(&store, "the-finding", &long);
        cross_second();

        let out = text_of(settle(&store, "the-finding", None, None, None, Some("t")).unwrap());
        assert!(out.contains("settled: the-finding"), "{out}");
        assert!(
            out.contains("(outcome)"),
            "an episode settles as an outcome: {out}"
        );
        assert!(
            out.contains("carried over: type from `episode`, name, body"),
            "and every default is named: {out}"
        );

        let full = kaeru_core::read_node_full(&store, &id_in(&out))
            .unwrap()
            .unwrap();
        assert_eq!(full.tier, "archival", "it actually moved tier");
        assert_eq!(full.name, "the-finding", "the name carried over");
        assert_eq!(
            full.body.as_deref(),
            Some(long.as_str()),
            "the body carried over in full — not the excerpt"
        );
    }

    /// An explicit type still wins, and then nothing is reported as inferred.
    #[test]
    fn an_explicit_type_overrides_the_heuristic() {
        let store = store_t();
        ep(&store, "note", "x");
        cross_second();
        let out =
            text_of(settle(&store, "note", Some("reference"), None, None, Some("t")).unwrap());
        assert!(out.contains("(reference)"), "{out}");
        assert!(
            !out.contains("type from"),
            "nothing was inferred, so nothing is claimed: {out}"
        );
    }

    /// Manual tags are the node's own content. A promotion moves it between
    /// tiers; it does not edit what the node says about itself.
    #[test]
    fn manual_tags_survive_the_promotion() {
        let store = store_t();
        let id =
            kaeru_core::write_task(&store, "ship the thing", Some("2030-01-01")).expect("task");
        kaeru_core::set_status(&store, "t", &id, "in-progress").expect("status");
        let name = kaeru_core::node_brief_by_id(&store, &id)
            .unwrap()
            .unwrap()
            .name;
        cross_second();

        let out = text_of(settle(&store, &name, None, None, None, Some("t")).unwrap());
        let tags = kaeru_core::read_node_full(&store, &id_in(&out))
            .unwrap()
            .unwrap()
            .tags;
        assert!(tags.iter().any(|t| t == "due:2030-01-01"), "{tags:?}");
        assert!(tags.iter().any(|t| t == "status:in-progress"), "{tags:?}");
        assert!(
            tags.iter().any(|t| t == "kind:outcome") && !tags.iter().any(|t| t == "kind:task"),
            "but the derived kind: follows the new type: {tags:?}"
        );
    }

    /// The mirror keeps the type — an outcome coming back into the working
    /// tier is still an outcome, so there is no heuristic to apply.
    #[test]
    fn unsettle_keeps_the_type_it_finds() {
        let store = store_t();
        ep(&store, "the-finding", "it held");
        cross_second();
        settle(&store, "the-finding", None, None, None, Some("t")).unwrap();
        cross_second();

        let out = text_of(unsettle(&store, "the-finding", None, None, None, Some("t")).unwrap());
        assert!(out.contains("unsettled: the-finding"), "{out}");
        assert!(
            out.contains("(outcome)"),
            "still an outcome, just back in flight: {out}"
        );
        assert_eq!(
            kaeru_core::read_node_full(&store, &id_in(&out))
                .unwrap()
                .unwrap()
                .tier,
            "operational"
        );
    }

    /// `supersede` replaces content, so name and body stay required — only the
    /// type defaults, to the old node's own.
    #[test]
    fn supersede_defaults_its_type_to_the_old_ones() {
        let store = store_t();
        ep(&store, "v1", "first");
        cross_second();
        let out = text_of(supersede(&store, "v1", None, "v2", "second", None, Some("t")).unwrap());
        assert!(out.contains("superseded: v1 → v2"), "{out}");
        assert!(out.contains("(episode"), "kept the old node's type: {out}");
    }
}
