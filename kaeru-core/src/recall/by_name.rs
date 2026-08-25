//! Explicit lookup by `name`, plus a `count_by_type` helper used by tests
//! and lint diagnostics. Both are simple `*node`-anchored-at-NOW reads.

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};

use super::fts::fuzzy_recall;
use super::{NodeBrief, NodeFull, parse_brief};
use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

/// Looks up a node id by its `name` at the current moment.
/// Returns `None` if no node matches.
///
/// If the store has a `current_initiative` set (via
/// [`Store::use_initiative`]), the lookup is constrained to nodes
/// attached to that initiative through the `node_initiative` junction.
/// Otherwise the search is cross-initiative.
///
/// When several distinct nodes share the same name, the **newest
/// assertion wins** — `:order validity` returns newest-first because
/// Cozo wraps the timestamp in `Reverse<>`.
pub fn recall_id_by_name(store: &Store, name: &str) -> Result<Option<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("name".to_string(), DataValue::Str(name.into()));

    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
                ?[id, validity] := *node{id, validity, name @ 'NOW'},
                                    name = $name,
                                    *node_initiative{initiative, node_id: id},
                                    initiative = $init
                :order validity
                :limit 1
            "#
        }
        None => {
            r#"
                ?[id, validity] := *node{id, validity, name @ 'NOW'}, name = $name
                :order validity
                :limit 1
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let id = rows
        .rows
        .first()
        .and_then(|row| row.first())
        .and_then(|v| v.get_str())
        .map(String::from);
    Ok(id)
}

/// Like [`recall_id_by_name`] but **always cross-initiative** — it ignores the
/// store's active initiative. Needed where a lookup must stay global even under
/// a scoped store: a scoped adapter (e.g. `kaeru-rig`, whose calls run inside
/// `Store::scoped`) can't clear the scope without re-locking that guard, so it
/// resolves through here instead. Used by `attach`, which targets a node living
/// under a *different* initiative than the active one.
pub fn recall_id_by_name_global(store: &Store, name: &str) -> Result<Option<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("name".to_string(), DataValue::Str(name.into()));
    let rows = store.db_ref().run_script(
        r#"
            ?[id, validity] := *node{id, validity, name @ 'NOW'}, name = $name
            :order validity
            :limit 1
        "#,
        params,
        ScriptMutability::Immutable,
    )?;
    Ok(rows
        .rows
        .first()
        .and_then(|row| row.first())
        .and_then(|v| v.get_str())
        .map(String::from))
}

/// Resolves a name across the node's **whole history**, not just NOW — the
/// last node that ever carried it wins.
///
/// `history` is the tool for reading a node's past, and until this existed it
/// could not resolve a name *from* that past: once `supersede` or `revise`
/// renamed a node, its old name stopped resolving at NOW, so exactly the nodes
/// whose history you wanted were unreachable by the name you remembered (#51).
///
/// Deliberately unscoped in time rather than taking an `at`: the caller
/// remembers a name, not the moment it was valid.
pub fn recall_id_by_name_ever(store: &Store, name: &str) -> Result<Option<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("name".to_string(), DataValue::Str(name.into()));
    // No `@` modifier: every assertion of every version is in scope. Order by
    // validity so the most recent node to carry the name wins.
    let script = r#"
        ?[id, validity] := *node{id, name, validity}, name = $name
        :order -validity
        :limit 1
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    Ok(rows
        .rows
        .first()
        .and_then(|r| r.first())
        .and_then(|v| v.get_str())
        .map(String::from))
}

/// Like [`recall_id_by_name`] but resolves the name **as of `at_seconds`**
/// instead of NOW, so a node that existed then but was retracted since still
/// resolves. This is what makes time-travel reads (`at(name, when)`) work for a
/// since-forgotten node: resolve at the target instant, then read at it. When
/// several nodes shared the name at that instant, the newest wins (`:order
/// validity`). Initiative-scoped like [`recall_id_by_name`]; the
/// `node_initiative` junction is append-only, so a retracted node's membership
/// still scopes it.
pub fn recall_id_by_name_at(store: &Store, name: &str, at_seconds: f64) -> Result<Option<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("name".to_string(), DataValue::Str(name.into()));

    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            format!(
                r#"
                ?[id, validity] := *node{{id, validity, name @ {at_seconds}}},
                                    name = $name,
                                    *node_initiative{{initiative, node_id: id}},
                                    initiative = $init
                :order validity
                :limit 1
                "#
            )
        }
        None => format!(
            r#"
            ?[id, validity] := *node{{id, validity, name @ {at_seconds}}}, name = $name
            :order validity
            :limit 1
            "#
        ),
    };
    let rows = store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Immutable)?;
    Ok(rows
        .rows
        .first()
        .and_then(|row| row.first())
        .and_then(|v| v.get_str())
        .map(String::from))
}

/// Returns a [`NodeBrief`] for `id` at NOW, or `None` if the node is
/// not currently asserted. Useful for CLI / display code that holds an
/// id and needs the human-readable name + excerpt.
pub fn node_brief_by_id(store: &Store, id: &NodeId) -> Result<Option<NodeBrief>> {
    let excerpt_chars = store.config().body_excerpt_chars;
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));

    let script = r#"
        ?[id, type, name, body, validity] := *node{id, type, name, body, validity @ 'NOW'}, id = $id
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let brief = rows
        .rows
        .first()
        .map(|row| parse_brief(row.as_slice(), excerpt_chars));
    Ok(brief)
}

/// Reads the **full** node record for `id` at NOW (untruncated body,
/// tier, tags, visibility), or `None` if not currently asserted. Used by
/// the cloud adapter, which needs every field to push a shared node and to
/// materialise one on pull — `node_brief_by_id` truncates the body and
/// omits tier/tags.
pub fn read_node_full(store: &Store, id: &NodeId) -> Result<Option<NodeFull>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));

    let script = r#"
        ?[type, tier, name, body, tags, visibility, layer] :=
            *node{id, type, tier, name, body, tags, visibility, layer @ 'NOW'}, id = $id
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let Some(row) = rows.rows.first() else {
        return Ok(None);
    };
    let node_type = row
        .first()
        .and_then(|v| v.get_str())
        .map(String::from)
        .unwrap_or_default();
    let tier = row
        .get(1)
        .and_then(|v| v.get_str())
        .map(String::from)
        .unwrap_or_default();
    let name = row
        .get(2)
        .and_then(|v| v.get_str())
        .map(String::from)
        .unwrap_or_default();
    let body = row.get(3).and_then(|v| v.get_str()).map(String::from);
    let tags = row.get(4).map(extract_string_list).unwrap_or_default();
    let visibility = row
        .get(5)
        .and_then(|v| v.get_str())
        .map(String::from)
        .unwrap_or_else(|| "local".to_string());
    let layer = row
        .get(6)
        .and_then(|v| v.get_str())
        .map(String::from)
        .unwrap_or_else(|| "warm".to_string());

    Ok(Some(NodeFull {
        id: id.clone(),
        node_type,
        tier,
        name,
        body,
        tags,
        visibility,
        layer,
    }))
}

/// Returns the **full** records of every `visibility = local` node in
/// `initiative` at NOW (explicit initiative, audit nodes excluded). This is
/// the sync-review work-list: `local` is exactly "not yet shared", so it
/// doubles as the since-last-sync marker — no separate watermark needed.
pub fn local_nodes_for_review(store: &Store, initiative: &str) -> Result<Vec<NodeFull>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));

    let script = r#"
        ?[id, type, tier, name, body, tags, visibility, layer] :=
            *node_initiative{initiative, node_id: id}, initiative = $init,
            *node{id, type, tier, name, body, tags, visibility, layer @ 'NOW'},
            visibility = 'local', type != 'audit_event'
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let nodes = rows
        .rows
        .iter()
        .map(|row| {
            let id = row
                .first()
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_default();
            let node_type = row
                .get(1)
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_default();
            let tier = row
                .get(2)
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_default();
            let name = row
                .get(3)
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_default();
            let body = row.get(4).and_then(|v| v.get_str()).map(String::from);
            let tags = row.get(5).map(extract_string_list).unwrap_or_default();
            let visibility = row
                .get(6)
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_else(|| "local".to_string());
            let layer = row
                .get(7)
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_else(|| "warm".to_string());
            NodeFull {
                id,
                node_type,
                tier,
                name,
                body,
                tags,
                visibility,
                layer,
            }
        })
        .collect();
    Ok(nodes)
}

/// Extracts a `Vec<String>` from a Cozo list column value; non-list
/// (e.g. `null`) yields an empty vec.
fn extract_string_list(v: &DataValue) -> Vec<String> {
    match v {
        DataValue::List(items) => items
            .iter()
            .filter_map(|x| x.get_str().map(String::from))
            .collect(),
        _ => Vec::new(),
    }
}

/// Counts nodes of a given type at the current moment.
/// Useful for tests and lint diagnostics.
pub fn count_by_type(store: &Store, node_type: &str) -> Result<usize> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("nt".to_string(), DataValue::Str(node_type.into()));

    let script = r#"
        ?[count(id)] := *node{id, type @ 'NOW'}, type = $nt
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let count = rows
        .rows
        .first()
        .and_then(|row| row.first())
        .and_then(|v| v.get_int())
        .unwrap_or(0);
    Ok(count as usize)
}

/// Node names close to `requested` — the did-you-mean for a name that did not
/// resolve.
///
/// A not-found that only says "no node named X" leaves the agent guessing, and
/// the audit caught the same misremembered name being tried across three
/// separate sessions: a stable false memory, re-entered because nothing ever
/// corrected it.
///
/// Candidates come from the FTS index rather than a full scan of every name —
/// this runs on a miss, and a miss should not cost a table scan. The query is
/// built from the requested name's **tokens**, each as a prefix: the input is
/// wrong by assumption, so feeding it to FTS whole finds nothing (a name with
/// punctuation parses as a quoted phrase, which then has to match exactly —
/// precisely what already failed). `auth-token-leaks` asks for `auth*`,
/// `token*`, `leaks*` and gets `auth-token-leak` back from the first.
///
/// Ranking then mirrors the initiative did-you-mean (#28): containment either
/// way, then edit distance within a length-scaled tolerance. Scoped to the
/// active initiative, like every other recall.
pub fn suggest_node_name(store: &Store, requested: &str) -> Result<Vec<String>> {
    const PER_TOKEN: usize = 6;
    const MAX_TOKENS: usize = 4;
    const SUGGESTIONS: usize = 3;

    let requested = requested.trim();
    if requested.is_empty() {
        return Ok(Vec::new());
    }
    let req = requested.to_lowercase();

    let tokens: Vec<String> = req
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| t.chars().count() >= 3)
        .take(MAX_TOKENS)
        .map(String::from)
        .collect();

    // Any FTS failure yields no suggestions rather than an error: this is
    // already the unhappy path, and a miss must never become a crash.
    let mut names: Vec<String> = Vec::new();
    for token in &tokens {
        if let Ok(hits) = fuzzy_recall(store, &format!("{token}*"), PER_TOKEN) {
            names.extend(hits.into_iter().map(|b| b.name));
        }
    }
    names.sort();
    names.dedup();

    let mut ranked: Vec<(usize, String)> = names
        .into_iter()
        .filter(|n| n.to_lowercase() != req)
        .filter_map(|n| {
            let nl = n.to_lowercase();
            let score = if nl.contains(&req) || req.contains(&nl) {
                0
            } else {
                let d = levenshtein(&req, &nl);
                if d > (nl.chars().count() / 3).max(2) {
                    return None;
                }
                d
            };
            Some((score, n))
        })
        .collect();
    ranked.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    Ok(ranked
        .into_iter()
        .take(SUGGESTIONS)
        .map(|(_, n)| n)
        .collect())
}

/// Levenshtein edit distance (two-row DP). Deliberately a second copy of the
/// one in `initiatives.rs`: sharing it would mean a `pub(crate)` helper in a
/// module neither owns, for twelve lines that will never diverge.
fn levenshtein(a: &str, b: &str) -> usize {
    let b_chars: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b_chars.len()).collect();
    let mut curr = vec![0usize; b_chars.len() + 1];
    for (i, ca) in a.chars().enumerate() {
        curr[0] = i + 1;
        for (j, &cb) in b_chars.iter().enumerate() {
            let cost = usize::from(ca != cb);
            curr[j + 1] = (prev[j] + cost).min(prev[j + 1] + 1).min(curr[j] + 1);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[b_chars.len()]
}

#[cfg(test)]
mod tests {
    use super::{
        node_brief_by_id, recall_id_by_name, recall_id_by_name_at, recall_id_by_name_global,
    };
    use crate::store::Store;
    use crate::{EpisodeKind, Significance, forget, write_episode};

    /// The global resolver finds a node by name regardless of the active
    /// initiative — the mechanism behind `attach` working across scopes even
    /// under a scoped store (e.g. kaeru-rig, whose calls run inside
    /// `Store::scoped`).
    #[test]
    fn global_resolve_ignores_active_initiative() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("alpha");
        let id = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "alpha-fact",
            "x",
        )
        .unwrap();

        // A different initiative is now active.
        store.use_initiative("beta");
        // Scoped resolution can't see the alpha node...
        assert!(recall_id_by_name(&store, "alpha-fact").unwrap().is_none());
        // ...but the global resolver does, ignoring the active scope.
        assert_eq!(
            recall_id_by_name_global(&store, "alpha-fact").unwrap(),
            Some(id)
        );
    }

    /// Resolving a name **as of a past moment** finds a node that has since
    /// been retracted — the mechanism behind `at(name, when)` reading the
    /// historical snapshot of a forgotten node (issue #27).
    #[test]
    fn at_resolver_finds_a_since_forgotten_node() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("p");
        let id = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "gone-node",
            "body",
        )
        .unwrap();
        // The moment it existed (its assertion second).
        let when = node_brief_by_id(&store, &id).unwrap().unwrap().ts.unwrap();

        // Cross the whole-second boundary so the retraction lands strictly after.
        std::thread::sleep(std::time::Duration::from_millis(1100));
        forget(&store, &id).unwrap();

        // At NOW the node is gone...
        assert!(recall_id_by_name(&store, "gone-node").unwrap().is_none());
        // ...but resolving as of when it existed still finds it.
        assert_eq!(
            recall_id_by_name_at(&store, "gone-node", when).unwrap(),
            Some(id)
        );
    }
}
