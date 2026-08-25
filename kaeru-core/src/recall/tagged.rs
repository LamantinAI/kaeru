//! `tagged` — list nodes whose `tags` array contains the given tag at
//! NOW. Slices the graph by tag (`kind:observation`, `sig:high`,
//! `role:review`, …).

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};

use super::{NodeBrief, parse_brief};
use crate::errors::Result;
use crate::store::Store;

/// Returns briefs for nodes whose `tags` list contains `tag`, valid at
/// NOW. Initiative-scoped when the store has a current initiative.
pub fn tagged(store: &Store, tag: &str) -> Result<Vec<NodeBrief>> {
    let excerpt_chars = store.config().body_excerpt_chars;
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("tag".to_string(), DataValue::Str(tag.into()));

    // `is_in` fails when `tags` is null; skip null rows first.
    // `:order validity` puts newest-first because Cozo wraps the
    // timestamp in `Reverse<>`.
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
                ?[id, type, name, body, validity] :=
                    *node{id, type, name, body, tags, validity @ 'NOW'},
                    !is_null(tags),
                    is_in($tag, tags),
                    *node_initiative{initiative, node_id: id},
                    initiative = $init
                :order validity
            "#
        }
        None => {
            r#"
                ?[id, type, name, body, validity] :=
                    *node{id, type, name, body, tags, validity @ 'NOW'},
                    !is_null(tags),
                    is_in($tag, tags)
                :order validity
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    let briefs = rows
        .rows
        .iter()
        .map(|r| parse_brief(r.as_slice(), excerpt_chars))
        .collect();
    Ok(briefs)
}
/// Distinct tags in scope that *contain* `fragment`, with how many nodes carry
/// each — most-used first, capped.
///
/// The answer to an empty `tagged`. Exact-match is the right semantics for a
/// tag (a slice is a slice), but it makes every near-miss indistinguishable
/// from an empty vault, and the first empty answer is what taught agents to
/// stop reaching for the verb at all (#59). This turns the miss into a menu of
/// what actually exists — `topic:figma` finding `topic:figma-макет` rather
/// than silence.
pub fn tags_like(store: &Store, fragment: &str) -> Result<Vec<(String, usize)>> {
    const SUGGESTION_CAP: usize = 6;
    let needle = fragment.to_lowercase();
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
                ?[tags] := *node{id, tags @ 'NOW'},
                           !is_null(tags),
                           *node_initiative{initiative, node_id: id},
                           initiative = $init
            "#
        }
        None => r#"?[tags] := *node{id, tags @ 'NOW'}, !is_null(tags)"#,
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for row in &rows.rows {
        if let Some(DataValue::List(items)) = row.first() {
            for tag in items.iter().filter_map(|v| v.get_str()) {
                if is_near(tag, &needle) {
                    *counts.entry(tag.to_string()).or_insert(0) += 1;
                }
            }
        }
    }
    let mut ranked: Vec<(String, usize)> = counts.into_iter().collect();
    // Most-carried first; alphabetical within a count keeps it deterministic.
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    ranked.truncate(SUGGESTION_CAP);
    Ok(ranked)
}

/// Whether `tag` is close enough to `needle` to be worth offering.
///
/// Compared on the tag's **value**, not the whole string: someone asking for
/// `topic:figma` is asking about figma, and matching the `topic:` prefix would
/// offer every topic tag in the vault.
///
/// Near in three ways, because a miss happens in three directions — the query
/// is shorter than what exists (`figma` vs `figma-макет`), longer than it
/// (`figma-file` vs `figma`), or a sibling spelling that shares a stem. The
/// prefix rule needs 3 characters, the same floor a topic token itself has.
fn is_near(tag: &str, needle: &str) -> bool {
    const MIN_SHARED_PREFIX: usize = 3;
    let value = tag.split_once(':').map(|(_, v)| v).unwrap_or(tag);
    let value = value.to_lowercase();
    if value.contains(needle) || needle.contains(&value) {
        return true;
    }
    let shared = value
        .chars()
        .zip(needle.chars())
        .take_while(|(a, b)| a == b)
        .count();
    shared >= MIN_SHARED_PREFIX
}
