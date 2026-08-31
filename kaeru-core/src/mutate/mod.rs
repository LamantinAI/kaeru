//! Active-mutation primitives: `write_episode`, `link`, `synthesise`, …
//!
//! Each primitive is a graph mutation that automatically writes an
//! `audit_event` node alongside the domain change. Submodules group
//! primitives by the shape of the mutation they perform; this `mod.rs`
//! re-exports the public surface and houses cross-submodule helpers
//! (timestamp generation, RMW reads).

use std::collections::{BTreeMap, HashMap};
use std::time::{SystemTime, UNIX_EPOCH};

use cozo::{DataValue, ScriptMutability};

use crate::errors::{Error, Result};
use crate::graph::NodeId;
use crate::store::Store;

pub mod board;
pub mod chain;
pub mod cite;
pub mod consolidate;
pub mod edge;
pub mod episode;
pub mod hypothesis;
pub mod ingest;
pub mod initiative;
pub mod layer;
pub mod metabolism;
pub mod review;
pub mod sharing;
pub mod slot;
pub mod supersedes;
pub mod synthesise;
pub mod task;

pub use board::{
    add_status, ensure_board, relabel_status, remove_status, reorder_statuses, set_status,
};
pub use chain::{ChainOutcome, RechainStats, create_chain, extend_chain, regenerate_chain};
pub use cite::{cite, cite_with_layer};
pub use consolidate::{consolidate_in, consolidate_out};
pub use edge::{link, link_remote, link_remote_to, link_with_weight, set_edge_weight, unlink};
pub use episode::{jot, jot_with_layer, write_episode, write_episode_with_layer};
pub use hypothesis::{
    formulate_hypothesis, formulate_hypothesis_with_layer, formulate_hypothesis_with_status,
    run_experiment, update_hypothesis_status,
};
pub use ingest::{upsert_edge, upsert_node};
pub use initiative::{
    AttachStats, DeleteStats, RenameStats, attach_node, delete_initiative, rename_initiative,
};
pub use layer::{get_layer, set_layer, set_layer_as};
pub use metabolism::{forget, improve};
pub use review::{mark_resolved, mark_under_review, resolve_review};
pub use sharing::{
    get_share_policy, get_visibility, initiative_clouds, permits_cloud, set_initiative_clouds,
    set_share_policy, set_visibility,
};
pub use slot::{SlotOutcome, occupy_slot, release_slot, slot_holder, slots_in};
pub use supersedes::supersedes;
pub use synthesise::synthesise;
pub use task::{complete_task, write_task, write_task_with_layer};

/// Cozo coerces `[float, bool]` to `Validity` only when the float is integer-
/// valued (whole seconds). Sub-second precision via fractional float fails
/// `eval::invalid_validity`. We therefore pin to whole-second resolution at
/// the substrate level. Tests that need distinct timestamps within the same
/// operation sequence add an explicit sleep.
pub(crate) fn now_validity_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Maximum number of `topic:<word>` tags derived from a node.
/// Keeps the tag list bounded; bumps later if needed.
const MAX_TOPIC_TOKENS: usize = 5;

/// How much harder an author-chosen name counts than a word in the body.
/// A name is the one string on a node somebody deliberately wrote to say what
/// it is about, so a single mention there outranks two in the prose.
const NAME_WEIGHT: usize = 3;

/// Extracts up to [`MAX_TOPIC_TOKENS`] topical tokens from a node —
/// lowercased, alphanumeric (Unicode-aware, so Cyrillic / CJK survive),
/// length ≥ 3, stop-words removed. Used to build `topic:<word>` tags so nodes
/// can be sliced by content via `tagged "topic:<word>"`.
///
/// Selection is by **salience, not position**. The original took the body's
/// first five content words, which selects for however the sentence happens
/// to open — so a project's actual subject, mentioned throughout but rarely
/// in the opening clause, never became a tag at all, and `tagged` answered
/// emptily on the one theme the initiative was about (#59). Tokens are scored
/// by how often they occur, ties broken by first appearance so the result
/// stays deterministic: the same node always derives the same tags,
/// regardless of what else the vault holds.
///
/// `name` is counted at [`NAME_WEIGHT`] and should be passed only when
/// somebody **chose** it. Pass `None` for auto-named nodes (`jot`, `task`),
/// whose name is itself derived from the body's first words — counting it
/// would smuggle the position bias back in through the front door.
///
/// A hyphenated or underscored compound yields its parts as well as itself:
/// `figma-макет` scores `figma-макет`, `figma` and `макет`. Tags are matched
/// exactly by `tagged`, so without this the word inside a compound is
/// unreachable — which is exactly how a Figma-centric initiative ended up
/// with no `topic:figma` on anything.
///
/// Returns `Vec<String>` of just the tokens themselves (without the
/// `topic:` prefix); call sites do that wrapping.
pub(crate) fn derive_topic_tokens(name: Option<&str>, body: &str) -> Vec<String> {
    // token -> (score, first appearance) — the tuple is the whole ranking.
    let mut scores: HashMap<String, (usize, usize)> = HashMap::new();
    let mut position = 0usize;

    let mut count = |token: String, weight: usize, position: usize| {
        let entry = scores.entry(token).or_insert((0, position));
        entry.0 += weight;
    };

    for (text, weight) in [(name.unwrap_or(""), NAME_WEIGHT), (body, 1)] {
        for raw in text.split_whitespace() {
            let cleaned: String = raw
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
                .collect::<String>()
                .to_lowercase();
            let cleaned = cleaned.trim_matches(['-', '_']).to_string();
            if cleaned.chars().count() < 3 || is_stop_word(&cleaned) {
                continue;
            }
            count(cleaned.clone(), weight, position);
            position += 1;

            // Parts of a compound are searchable in their own right.
            if cleaned.contains(['-', '_']) {
                for part in cleaned.split(['-', '_']) {
                    if part.chars().count() >= 3 && !is_stop_word(part) {
                        count(part.to_string(), weight, position);
                        position += 1;
                    }
                }
            }
        }
    }

    let mut ranked: Vec<(String, usize, usize)> = scores
        .into_iter()
        .map(|(tok, (score, first))| (tok, score, first))
        .collect();
    // Most-mentioned first; ties settled by who appeared first, so the output
    // never depends on hash iteration order.
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.2.cmp(&b.2)));
    ranked
        .into_iter()
        .take(MAX_TOPIC_TOKENS)
        .map(|(tok, _, _)| tok)
        .collect()
}

/// Tiny EN+RU stop-word list — drops the most common low-content tokens
/// so they don't burn slots in the topic-tag set. Not exhaustive on
/// purpose; the goal is "not pure noise", not perfect linguistics.
fn is_stop_word(w: &str) -> bool {
    matches!(
        w,
        // English
        "the" | "and" | "for" | "are" | "but" | "not" | "you" | "all" | "any"
        | "can" | "had" | "her" | "was" | "one" | "our" | "out" | "have"
        | "this" | "with" | "they" | "from" | "what" | "been" | "were"
        | "than" | "them" | "then" | "into" | "some" | "more" | "just"
        | "that" | "will" | "your"
        // Russian (basic high-frequency forms)
        | "что" | "это" | "как" | "так" | "вот" | "уже" | "был" | "была"
        | "было" | "были" | "она" | "они" | "его" | "ему" | "тех" | "там"
        | "тут" | "под" | "над" | "при" | "для" | "или" | "между" | "если"
        | "когда" | "потом" | "тоже" | "после"
    )
}

/// Detects the predominant script of `body` and returns a tag string
/// (`lang:ru` / `lang:en` / `lang:mixed` / `lang:other`). Heuristic
/// only — counts Cyrillic vs Latin alphabetic chars, ignores
/// punctuation and digits. Multilingual-by-design: doesn't enforce a
/// language, just gives a hint for downstream agents.
pub(crate) fn detect_lang_tag(body: &str) -> String {
    let mut cyrillic: usize = 0;
    let mut latin: usize = 0;
    for c in body.chars() {
        if !c.is_alphabetic() {
            continue;
        }
        let cp = c as u32;
        // Cyrillic + Cyrillic Supplement Unicode blocks.
        if (0x0400..=0x04FF).contains(&cp) || (0x0500..=0x052F).contains(&cp) {
            cyrillic += 1;
        } else if c.is_ascii_alphabetic() {
            latin += 1;
        }
    }
    let total = cyrillic + latin;
    if total == 0 {
        return "lang:other".to_string();
    }
    let cyr_ratio = cyrillic as f64 / total as f64;
    if cyr_ratio > 0.7 {
        "lang:ru".to_string()
    } else if cyr_ratio < 0.3 {
        "lang:en".to_string()
    } else {
        "lang:mixed".to_string()
    }
}

/// Builds a Cozo list literal of single-quoted strings, suitable for
/// inlining into a `<-` rule. Tokens that came through `derive_topic_tokens`
/// are already alphanumeric, so quote escaping is unnecessary; we still
/// double single-quotes defensively for fixed prefix tags
/// (`kind:`, `sig:`, `lang:`, …) that might one day include them.
pub(crate) fn tags_literal(tags: &[String]) -> String {
    if tags.is_empty() {
        return "null".to_string();
    }
    let inner = tags
        .iter()
        .map(|t| format!("'{}'", t.replace('\'', "''")))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{inner}]")
}

/// Moves a node's `chain_member` rows onto its successor when a verb replaces
/// one identity with another.
///
/// A chain stores its steps by id, so a `settle` / `supersede` used to drop the
/// node out of every trail it belonged to — silently, and always the step worth
/// having, because the node worth promoting is usually the outcome the trail
/// was built to explain (#71). Nothing reported the loss: `rechain` counted the
/// retracted member and called the chain current, and `why` on the successor
/// said no trail led to it.
///
/// This is the same commitment `consolidate` already makes for `derived_from`:
/// an identity change is bookkeeping, and the graph's statements about a node
/// should survive it.
pub(crate) fn carry_chain_membership(
    store: &Store,
    old_id: &NodeId,
    new_id: &NodeId,
) -> Result<usize> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("old".to_string(), DataValue::Str(old_id.clone().into()));
    let rows = store.db_ref().run_script(
        "?[chain_id, position] := *chain_member{chain_id, position, node_id}, node_id = $old",
        params,
        ScriptMutability::Immutable,
    )?;

    let mut moved = 0usize;
    for r in &rows.rows {
        let (Some(cid), Some(pos)) = (
            r.first().and_then(|v| v.get_str()),
            r.get(1).and_then(|v| v.get_int()),
        ) else {
            continue;
        };
        let mut p: BTreeMap<String, DataValue> = BTreeMap::new();
        p.insert("cid".to_string(), DataValue::Str(cid.into()));
        p.insert("nid".to_string(), DataValue::Str(new_id.clone().into()));
        // `chain_member` is keyed by {chain_id, position}, so writing the same
        // key with the successor's id replaces the step in place — the trail
        // keeps its order and its length.
        let script = format!(
            r#"
            ?[chain_id, position, node_id] <- [[$cid, {pos}, $nid]]
            :put chain_member {{chain_id, position => node_id}}
            "#
        );
        store
            .db_ref()
            .run_script(&script, p, ScriptMutability::Mutable)?;
        moved += 1;
    }
    Ok(moved)
}

/// Convenience: builds the tags list for a write that has a body.
/// Combines fixed prefix tags (caller-specified) with the auto-derived
/// `lang:*` and `topic:<word>` tags.
///
/// Pass `name` only when it was **chosen** — see [`derive_topic_tokens`]. The
/// auto-naming verbs (`jot`, `task`) pass `None`, because their name is the
/// body's own opening words and counting it would just re-weight position.
pub(crate) fn build_body_tags(fixed: &[&str], name: Option<&str>, body: &str) -> Vec<String> {
    let mut tags: Vec<String> = fixed.iter().map(|s| (*s).to_string()).collect();
    tags.push(detect_lang_tag(body));
    for token in derive_topic_tokens(name, body) {
        tags.push(format!("topic:{token}"));
    }
    tags
}

/// Value (non-key) columns of the `node` relation, in schema order — the
/// single source of truth for RMW rewrites. [`reassert_node_now`] builds its
/// `:put` from this list, and the schema-lock test compares it against
/// `::columns node`, so adding a column to the schema fails the suite until
/// every rewrite path handles the new column explicitly.
pub(crate) const NODE_VALUE_COLUMNS: [&str; 9] = [
    "type",
    "tier",
    "name",
    "body",
    "tags",
    "initiatives",
    "properties",
    "visibility",
    "layer",
];

/// A node's value columns as read at NOW — everything an RMW rewrite must
/// decide about, minus the opaque `initiatives` / `properties`, which
/// [`reassert_node_now`] copies forward inside the substrate.
pub(crate) struct NodeNow {
    pub type_: String,
    pub tier: String,
    pub name: String,
    pub body: Option<String>,
    pub tags: Vec<String>,
    pub visibility: String,
    pub layer: String,
}

/// Reads a node's value columns at NOW. Returns `None` if no row is valid
/// at the moment of the call. Used by primitives that rewrite a node while
/// preserving the fields the caller did not change.
pub(crate) fn read_node_now(store: &Store, id: &NodeId) -> Result<Option<NodeNow>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));
    let script = r#"
        ?[type, tier, name, body, tags, visibility, layer] :=
            *node{id, type, tier, name, body, tags, visibility, layer @ 'NOW'}, id = $id
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    let result = rows.rows.first().map(|r| {
        let s = |i: usize| {
            r.get(i)
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_default()
        };
        NodeNow {
            type_: s(0),
            tier: s(1),
            name: s(2),
            body: r.get(3).and_then(|v| v.get_str()).map(String::from),
            tags: match r.get(4) {
                Some(DataValue::List(items)) => items
                    .iter()
                    .filter_map(|x| x.get_str().map(String::from))
                    .collect(),
                _ => Vec::new(),
            },
            visibility: r
                .get(5)
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_else(|| "local".to_string()),
            layer: r
                .get(6)
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_else(|| "warm".to_string()),
        }
    });
    Ok(result)
}

/// Tag merge for RMW rewrites: keeps the current tags (so manual tags
/// survive a rewrite) minus the `drop_prefixes` families the caller is
/// re-deriving (`status:`, `lang:`, …), then unions in `add`. Order is
/// stable — survivors first, new tags after — with exact-string dedup.
pub(crate) fn merge_tags(
    current: &[String],
    drop_prefixes: &[&str],
    add: Vec<String>,
) -> Vec<String> {
    let mut out: Vec<String> = current
        .iter()
        .filter(|t| !drop_prefixes.iter().any(|p| t.starts_with(p)))
        .cloned()
        .collect();
    for tag in add {
        if !out.contains(&tag) {
            out.push(tag);
        }
    }
    out
}

/// The fully-decided value columns for an RMW re-assert. `initiatives` and
/// `properties` are deliberately absent: they are opaque to every rewrite
/// verb and get copied forward from the current row inside the substrate.
pub(crate) struct ReassertRow<'a> {
    pub secs: u64,
    pub type_: &'a str,
    pub tier: &'a str,
    pub name: &'a str,
    pub body: Option<&'a str>,
    pub tags: Vec<String>,
    pub visibility: &'a str,
    pub layer: &'a str,
}

/// Re-asserts node `id` at `row.secs` with **every** value column of the
/// schema spelled out — nothing silently falls back to a schema default
/// (which is how rewrites used to reset `layer` to `warm` and `visibility`
/// to `local`). The opaque columns (`initiatives`, `properties`) are copied
/// forward from the row valid at NOW by the substrate itself.
///
/// ORDERING INVARIANT: call this **before** retracting the old row. The
/// copy-forward reads `@ 'NOW'`; once the retract lands, the read resolves
/// nothing, the `:put` writes zero rows, and the node simply vanishes.
/// Callers therefore re-assert first and retract second, passing the SAME
/// whole-second timestamp to both writes — at equal timestamps the
/// substrate resolves the assertion, so write order within the second
/// doesn't matter.
pub(crate) fn reassert_node_now(store: &Store, id: &NodeId, row: ReassertRow<'_>) -> Result<()> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));
    params.insert("name".to_string(), DataValue::Str(row.name.into()));
    params.insert(
        "body".to_string(),
        match row.body {
            Some(b) => DataValue::Str(b.into()),
            None => DataValue::Null,
        },
    );
    // Tags and the Validity literal must be inlined — cozo needs concrete
    // values for List / Validity columns (same constraint as `upsert_node`).
    // type / tier / visibility / layer are enum-derived strings, never
    // attacker-controlled, so inlining their quoted form is safe.
    let tags_lit = tags_literal(&row.tags);
    let cols = NODE_VALUE_COLUMNS.join(", ");
    let script = format!(
        r#"
        ?[id, validity, {cols}] :=
            *node{{id, initiatives, properties @ 'NOW'}}, id = $id,
            validity = [{secs}.0, true],
            type = '{ty}', tier = '{tier}',
            name = $name, body = $body,
            tags = {tags_lit},
            visibility = '{vis}', layer = '{layer}'
        :put node {{id, validity => {cols}}}
        "#,
        secs = row.secs,
        ty = row.type_,
        tier = row.tier,
        vis = row.visibility,
        layer = row.layer,
    );
    store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Mutable)?;
    Ok(())
}

/// In-place rewrite of exactly ONE value column, with every other column
/// round-tripped from the row it read. Both the read and the `:put` are
/// generated from [`NODE_VALUE_COLUMNS`], so a column added to the schema is
/// carried through automatically instead of silently resetting to its default
/// — the failure that used to reset `layer` to `warm` and `visibility` to
/// `local` on every `set_layer` / `set_visibility` call, because each verb
/// spelled the column list out by hand and read the row positionally.
///
/// No new validity is minted: the SAME `(id, validity)` primary key is
/// overwritten, so an `@ 'NOW'` read can never resolve two competing versions.
/// The read prefers the `@ 'NOW'` view and falls back to the latest historical
/// version, so re-running the verb also *recovers* a node left invisible by an
/// older buggy rewrite.
///
/// Values round-trip as Cozo parameters (`$body`, `$tags`, …) — `DataValue`s
/// read out go straight back in, so bodies, lists and JSON never need escaping.
pub(crate) fn rewrite_node_column_in_place(
    store: &Store,
    id: &NodeId,
    column: &str,
    value: &str,
) -> Result<()> {
    if !NODE_VALUE_COLUMNS.contains(&column) {
        return Err(Error::Invalid(format!(
            "unknown node column `{column}` (known: {})",
            NODE_VALUE_COLUMNS.join(", ")
        )));
    }
    let cols = NODE_VALUE_COLUMNS.join(", ");

    let mut read_params: BTreeMap<String, DataValue> = BTreeMap::new();
    read_params.insert("id".to_string(), DataValue::Str(id.clone().into()));

    let now_script = format!(
        r#"
        ?[validity, {cols}] :=
            *node{{id, validity, {cols} @ 'NOW'}}, id = $id
        "#
    );
    let mut current = store.db_ref().run_script(
        &now_script,
        read_params.clone(),
        ScriptMutability::Immutable,
    )?;

    if current.rows.is_empty() {
        let hist_script = format!(
            r#"
            ?[validity, {cols}] :=
                *node{{id, validity, {cols}}}, id = $id
            :order -validity
            :limit 1
            "#
        );
        current =
            store
                .db_ref()
                .run_script(&hist_script, read_params, ScriptMutability::Immutable)?;
    }

    let row = current
        .rows
        .first()
        .ok_or_else(|| Error::NotFound(format!("node not found: {id}")))?;

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));
    params.insert(
        "validity".to_string(),
        row.first()
            .cloned()
            .ok_or_else(|| Error::Invalid(format!("node {id}: row has no validity")))?,
    );
    // Column i of the read sits at row[i + 1] — `validity` occupies row[0].
    for (index, col) in NODE_VALUE_COLUMNS.iter().enumerate() {
        let carried = row
            .get(index + 1)
            .cloned()
            .ok_or_else(|| Error::Invalid(format!("node {id}: row has no column `{col}`")))?;
        let next = if *col == column {
            DataValue::Str(value.into())
        } else {
            carried
        };
        params.insert((*col).to_string(), next);
    }

    let placeholders = NODE_VALUE_COLUMNS
        .iter()
        .map(|c| format!("${c}"))
        .collect::<Vec<_>>()
        .join(", ");
    let put_script = format!(
        r#"
        ?[id, validity, {cols}] <- [[$id, $validity, {placeholders}]]
        :put node {{id, validity => {cols}}}
        "#
    );
    store
        .db_ref()
        .run_script(&put_script, params, ScriptMutability::Mutable)?;
    Ok(())
}

/// Builds the inline value tuple for a whole-row `:put node`, in
/// [`NODE_VALUE_COLUMNS`] order. Every column must have an entry in `values`
/// — either a Cozo literal (`'episode'`, `null`, `['a','b']`) or a parameter
/// reference (`$name`). A column added to the schema without a value here
/// fails loudly at the first call instead of silently taking its default.
pub(crate) fn node_row_values(values: &BTreeMap<&str, String>) -> Result<String> {
    let mut out = Vec::with_capacity(NODE_VALUE_COLUMNS.len());
    for col in NODE_VALUE_COLUMNS {
        let value = values.get(col).ok_or_else(|| {
            Error::Invalid(format!(
                "node column `{col}` has no value in this write path — teach it \
                 about the column before changing the schema"
            ))
        })?;
        out.push(value.clone());
    }
    Ok(out.join(", "))
}

/// Writes the bi-temporal retraction row for `id` at `secs`. The
/// placeholder values in the value columns are never observable — the row
/// is a retraction and does not resolve at NOW.
pub(crate) fn retract_node_at(store: &Store, id: &NodeId, secs: u64) -> Result<()> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));
    let script = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{secs}.0, false], 'placeholder', 'operational', '', null, null, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Mutable)?;
    Ok(())
}

/// Returns every edge (src, dst, edge_type) connected to `node_id` at NOW
/// (inbound or outbound). Used by [`metabolism::forget`] to retract them.
pub(crate) fn read_connected_edges(
    store: &Store,
    node_id: &NodeId,
) -> Result<Vec<(String, String, String)>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("nid".to_string(), DataValue::Str(node_id.clone().into()));
    let script = r#"
        ?[src, dst, edge_type] := *edge{src, dst, edge_type @ 'NOW'}, src = $nid
        ?[src, dst, edge_type] := *edge{src, dst, edge_type @ 'NOW'}, dst = $nid
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    let edges: Vec<(String, String, String)> = rows
        .rows
        .iter()
        .filter_map(|r| {
            let src = r.first().and_then(|v| v.get_str()).map(String::from)?;
            let dst = r.get(1).and_then(|v| v.get_str()).map(String::from)?;
            let et = r.get(2).and_then(|v| v.get_str()).map(String::from)?;
            Some((src, dst, et))
        })
        .collect();
    Ok(edges)
}

/// Attaches `node_id` to the store's current initiative through the
/// `node_initiative` junction relation. No-op if no initiative is
/// active. Called by every mutation that asserts a fresh node.
pub(crate) fn attach_node_to_initiative(store: &Store, node_id: &NodeId) -> Result<()> {
    let Some(initiative) = store.current_initiative() else {
        return Ok(());
    };
    attach_node_to_initiative_named(store, node_id, &initiative)
}

/// Attaches `node_id` to an **explicitly named** initiative — the junction
/// write behind [`attach_node_to_initiative`], usable when the membership
/// comes from somewhere other than the store's current scope (e.g.
/// consolidation inheriting the source node's initiatives). Idempotent.
pub(crate) fn attach_node_to_initiative_named(
    store: &Store,
    node_id: &NodeId,
    initiative: &str,
) -> Result<()> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    params.insert("nid".to_string(), DataValue::Str(node_id.clone().into()));
    let script = r#"
        ?[initiative, node_id] <- [[$init, $nid]]
        :put node_initiative {initiative, node_id}
    "#;
    store
        .db_ref()
        .run_script(script, params, ScriptMutability::Mutable)?;
    Ok(())
}

/// Returns every initiative `node_id` is attached to through the
/// `node_initiative` junction. The junction is append-only, so this also
/// answers for retracted nodes — which is exactly what consolidation needs
/// when it inherits memberships from a node it just retracted.
/// Public because two different reads need it. A not-found has to answer
/// "does it exist somewhere else?" before it can say anything useful, and a
/// node-addressed read on the HTTP surface has to answer "may this leave" —
/// and both answers are written in initiatives. It lives in `mutate` because
/// consolidation needed it first; nothing about it writes.
pub fn initiatives_of_node(store: &Store, node_id: &NodeId) -> Result<Vec<String>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("nid".to_string(), DataValue::Str(node_id.clone().into()));
    let script = r#"
        ?[initiative] := *node_initiative{initiative, node_id}, node_id = $nid
        :order initiative
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    let names = rows
        .rows
        .iter()
        .filter_map(|r| r.first().and_then(|v| v.get_str()).map(String::from))
        .collect();
    Ok(names)
}

/// Attaches an edge to the store's current initiative through the
/// `edge_initiative` junction relation. The edge's primary key is
/// encoded as `src|dst|edge_type` so re-attachment is idempotent. No-op
/// if no initiative is active.
pub(crate) fn attach_edge_to_initiative(
    store: &Store,
    src: &NodeId,
    dst: &NodeId,
    edge_type: &str,
) -> Result<()> {
    let Some(initiative) = store.current_initiative() else {
        return Ok(());
    };
    let edge_pk = format!("{src}|{dst}|{edge_type}");
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    params.insert("epk".to_string(), DataValue::Str(edge_pk.into()));
    let script = r#"
        ?[initiative, edge_pk] <- [[$init, $epk]]
        :put edge_initiative {initiative, edge_pk}
    "#;
    store
        .db_ref()
        .run_script(script, params, ScriptMutability::Mutable)?;
    Ok(())
}

/// Returns dst ids of `derived_from` edges where `src_id` is the source
/// at NOW. Used by [`consolidate`] to replicate provenance edges across
/// the tier boundary.
pub(crate) fn read_derived_from_targets(store: &Store, src_id: &NodeId) -> Result<Vec<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("src".to_string(), DataValue::Str(src_id.clone().into()));
    let script = r#"
        ?[dst] := *edge{src, dst, edge_type @ 'NOW'},
                  src = $src,
                  edge_type = 'derived_from'
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    let targets: Vec<NodeId> = rows
        .rows
        .iter()
        .filter_map(|r| r.first().and_then(|v| v.get_str()).map(String::from))
        .collect();
    Ok(targets)
}

#[cfg(test)]
mod topic_tests {
    use super::derive_topic_tokens;

    /// The bug #59 is named after: the theme runs through the whole note but
    /// never opens it, so a first-five-words rule never tagged it.
    #[test]
    fn the_repeated_subject_wins_over_the_opening_words() {
        let tokens = derive_topic_tokens(
            None,
            "Yesterday morning during standup we agreed the figma export keeps \
             dropping layer names; figma support confirmed it, so figma is the \
             blocker until they ship a fix",
        );
        assert!(
            tokens.contains(&"figma".to_string()),
            "the subject is tagged: {tokens:?}"
        );
    }

    /// A chosen name is the one string somebody wrote to say what the node is
    /// about, so one mention there beats two in the prose.
    #[test]
    fn a_chosen_name_outweighs_the_body() {
        let tokens = derive_topic_tokens(
            Some("figma-export-broken"),
            "the pipeline drops values while writing the manifest, twice today",
        );
        assert!(tokens.contains(&"figma".to_string()), "{tokens:?}");
        assert!(tokens.contains(&"export".to_string()), "{tokens:?}");
    }

    /// Tags are matched exactly, so a word buried in a compound has to be
    /// reachable on its own — this is the literal `topic:figma` miss.
    #[test]
    fn a_compound_is_findable_by_its_parts() {
        let tokens = derive_topic_tokens(None, "правим figma-макет и ещё раз figma-макет");
        assert!(tokens.contains(&"figma-макет".to_string()), "{tokens:?}");
        assert!(
            tokens.contains(&"figma".to_string()),
            "and by its parts: {tokens:?}"
        );
        assert!(tokens.contains(&"макет".to_string()), "{tokens:?}");
    }

    /// An auto-named node passes `None`, so its name — the body's own opening
    /// words — cannot smuggle the position bias back in.
    #[test]
    fn an_auto_name_is_not_counted_twice() {
        let body = "alpha alpha beta beta beta";
        let auto = derive_topic_tokens(None, body);
        assert_eq!(auto.first(), Some(&"beta".to_string()), "{auto:?}");
    }

    /// Same input, same tags — always. Ranking must not depend on hash order,
    /// or a node's tags would differ between runs.
    #[test]
    fn derivation_is_deterministic() {
        let body = "cache cache index index storage retry retry retry";
        let first = derive_topic_tokens(Some("storage-cache"), body);
        for _ in 0..20 {
            assert_eq!(derive_topic_tokens(Some("storage-cache"), body), first);
        }
    }

    /// Bounded, and stop-words still don't burn slots.
    #[test]
    fn the_tag_set_stays_bounded_and_content_bearing() {
        let tokens = derive_topic_tokens(None, &"that will your from with ".repeat(50));
        assert!(
            tokens.is_empty(),
            "pure stop-words yield nothing: {tokens:?}"
        );
        let many = derive_topic_tokens(None, "one two three four five six seven eight nine");
        assert!(many.len() <= 5, "{many:?}");
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use cozo::{DataValue, ScriptMutability};

    use super::{
        NODE_VALUE_COLUMNS, ReassertRow, merge_tags, read_node_now, reassert_node_now,
        retract_node_at,
    };
    use crate::graph::NodeId;
    use crate::store::Store;

    /// The `node` schema and [`NODE_VALUE_COLUMNS`] must agree exactly.
    /// A new schema column that the RMW rewrite paths don't know about
    /// would silently reset to its default on every rewrite — this test
    /// makes that a loud failure instead.
    #[test]
    fn schema_lock_node_value_columns() {
        let store = Store::open_in_memory().expect("open");
        let rows = store
            .db_ref()
            .run_script(
                "::columns node",
                BTreeMap::new(),
                ScriptMutability::Immutable,
            )
            .expect("::columns node");
        let names: Vec<String> = rows
            .rows
            .iter()
            .filter_map(|r| r.first().and_then(|v| v.get_str()).map(String::from))
            .collect();
        let mut expected = vec!["id".to_string(), "validity".to_string()];
        expected.extend(NODE_VALUE_COLUMNS.iter().map(|s| s.to_string()));
        assert_eq!(
            names, expected,
            "node schema drifted from NODE_VALUE_COLUMNS — teach the RMW \
             rewrite paths (reassert_node_now and its callers) about the new \
             column before changing the schema"
        );
    }

    /// Round-trip through the RMW helper: every column the caller didn't
    /// override survives — including the opaque `initiatives` / `properties`
    /// (copied forward inside the substrate) and `visibility` / `layer`
    /// (spelled out instead of falling back to schema defaults).
    #[test]
    fn reassert_preserves_untouched_columns() {
        let store = Store::open_in_memory().expect("open");

        // Seed a node carrying a value in EVERY column.
        let mut p: BTreeMap<String, DataValue> = BTreeMap::new();
        p.insert(
            "props".to_string(),
            DataValue::Json(cozo::JsonData(serde_json::json!({"a": 1}))),
        );
        let seed = r#"
            ?[id, validity, type, tier, name, body, tags, initiatives, properties, visibility, layer] <-
                [['n1', [1000.0, true], 'episode', 'operational', 'old-name', 'old body',
                  ['custom:x'], ['team-init'], $props, 'shared', 'core']]
            :put node {id, validity => type, tier, name, body, tags, initiatives, properties, visibility, layer}
        "#;
        store
            .db_ref()
            .run_script(seed, p, ScriptMutability::Mutable)
            .expect("seed");

        let id = "n1".to_string();
        let now = read_node_now(&store, &id).expect("read").expect("present");
        assert_eq!(now.visibility, "shared");
        assert_eq!(now.layer, "core");

        // Rewrite name/body, preserve the rest — re-assert BEFORE retract,
        // same timestamp for both (the helper's ordering invariant).
        let secs = super::now_validity_seconds();
        reassert_node_now(
            &store,
            &id,
            ReassertRow {
                secs,
                type_: &now.type_,
                tier: &now.tier,
                name: "new-name",
                body: Some("new body"),
                tags: merge_tags(&now.tags, &["lang:"], vec!["role:revised".to_string()]),
                visibility: &now.visibility,
                layer: &now.layer,
            },
        )
        .expect("reassert");
        retract_node_at(&store, &id, secs).expect("retract");

        let check = r#"
            ?[name, tags, initiatives, properties, visibility, layer] :=
                *node{id, name, tags, initiatives, properties, visibility, layer @ 'NOW'}, id = 'n1'
        "#;
        let rows = store
            .db_ref()
            .run_script(check, BTreeMap::new(), ScriptMutability::Immutable)
            .expect("read back");
        assert_eq!(rows.rows.len(), 1, "node resolves at NOW: {rows:?}");
        let row = &rows.rows[0];
        assert_eq!(row[0].get_str(), Some("new-name"));
        let tags_dbg = format!("{:?}", row[1]);
        assert!(
            tags_dbg.contains("custom:x"),
            "manual tag survives: {tags_dbg}"
        );
        assert!(
            tags_dbg.contains("role:revised"),
            "new tag merged: {tags_dbg}"
        );
        assert!(
            format!("{:?}", row[2]).contains("team-init"),
            "initiatives column copied forward: {:?}",
            row[2]
        );
        assert!(
            format!("{:?}", row[3]).contains('1'),
            "properties copied forward: {:?}",
            row[3]
        );
        assert_eq!(row[4].get_str(), Some("shared"), "visibility preserved");
        assert_eq!(row[5].get_str(), Some("core"), "layer preserved");
    }

    /// Seeds `n1` with a non-default value in EVERY value column, so any
    /// column a rewrite path forgets shows up as a reset to its default.
    fn seed_full_node(store: &Store) -> NodeId {
        let mut p: BTreeMap<String, DataValue> = BTreeMap::new();
        p.insert(
            "props".to_string(),
            DataValue::Json(cozo::JsonData(serde_json::json!({"a": 1}))),
        );
        let seed = r#"
            ?[id, validity, type, tier, name, body, tags, initiatives, properties, visibility, layer] <-
                [['n1', [1000.0, true], 'episode', 'operational', 'seeded-name', 'seeded body',
                  ['custom:x'], ['team-init'], $props, 'shared', 'core']]
            :put node {id, validity => type, tier, name, body, tags, initiatives, properties, visibility, layer}
        "#;
        store
            .db_ref()
            .run_script(seed, p, ScriptMutability::Mutable)
            .expect("seed");
        "n1".to_string()
    }

    /// Reads every value column of `n1` at NOW, in schema order.
    fn read_full_row(store: &Store) -> Vec<DataValue> {
        let cols = super::NODE_VALUE_COLUMNS.join(", ");
        let script = format!(
            r#"
            ?[{cols}] := *node{{id, {cols} @ 'NOW'}}, id = 'n1'
            "#
        );
        let rows = store
            .db_ref()
            .run_script(&script, BTreeMap::new(), ScriptMutability::Immutable)
            .expect("read back");
        assert_eq!(rows.rows.len(), 1, "node resolves at NOW: {rows:?}");
        rows.rows[0].clone()
    }

    /// Regression for the 2026-07-08 incident, generalised: `set_layer` used
    /// to spell the column list out by hand and read the row positionally, so
    /// a column it didn't know about silently reset to its schema default.
    /// It now shares the generated rewrite — this test fails loudly if that
    /// ever regresses, for ANY column, not just the two that were lost then.
    #[test]
    fn set_layer_preserves_every_other_column() {
        let store = Store::open_in_memory().expect("open");
        let id = seed_full_node(&store);

        crate::mutate::set_layer(&store, &id, crate::graph::Layer::Frozen).expect("set_layer");

        let row = read_full_row(&store);
        assert_eq!(row[0].get_str(), Some("episode"), "type preserved");
        assert_eq!(row[1].get_str(), Some("operational"), "tier preserved");
        assert_eq!(row[2].get_str(), Some("seeded-name"), "name preserved");
        assert_eq!(row[3].get_str(), Some("seeded body"), "body preserved");
        assert!(
            format!("{:?}", row[4]).contains("custom:x"),
            "tags preserved: {:?}",
            row[4]
        );
        assert!(
            format!("{:?}", row[5]).contains("team-init"),
            "initiatives preserved: {:?}",
            row[5]
        );
        assert!(
            format!("{:?}", row[6]).contains('1'),
            "properties preserved: {:?}",
            row[6]
        );
        assert_eq!(
            row[7].get_str(),
            Some("shared"),
            "visibility preserved — this is the column the incident lost"
        );
        assert_eq!(row[8].get_str(), Some("frozen"), "layer actually changed");
    }

    /// Mirror of the above for `set_visibility`: the column it must not lose
    /// is `layer`, which the same incident reset to `warm`.
    #[test]
    fn set_visibility_preserves_every_other_column() {
        let store = Store::open_in_memory().expect("open");
        let id = seed_full_node(&store);

        crate::mutate::set_visibility(&store, &id, crate::graph::Visibility::Local)
            .expect("set_visibility");

        let row = read_full_row(&store);
        assert_eq!(row[2].get_str(), Some("seeded-name"), "name preserved");
        assert!(
            format!("{:?}", row[4]).contains("custom:x"),
            "tags preserved: {:?}",
            row[4]
        );
        assert!(
            format!("{:?}", row[6]).contains('1'),
            "properties preserved: {:?}",
            row[6]
        );
        assert_eq!(
            row[7].get_str(),
            Some("local"),
            "visibility actually changed"
        );
        assert_eq!(
            row[8].get_str(),
            Some("core"),
            "layer preserved — this is the column the incident lost"
        );
    }

    /// The in-place rewrite refuses a column that isn't part of the schema,
    /// rather than writing a row Cozo would reject with a cryptic message.
    #[test]
    fn rewrite_rejects_a_column_outside_the_schema() {
        let store = Store::open_in_memory().expect("open");
        let id = seed_full_node(&store);

        let err = super::rewrite_node_column_in_place(&store, &id, "not_a_column", "x")
            .expect_err("unknown column must be rejected");
        assert!(
            format!("{err}").contains("not_a_column"),
            "error names the offending column: {err}"
        );
    }

    /// Whole-row writes (cloud ingest) must supply a value for every column.
    /// A column added to the schema and to `NODE_VALUE_COLUMNS` but not to a
    /// write path fails here instead of landing on its default unnoticed.
    #[test]
    fn node_row_values_demands_a_value_for_every_column() {
        let mut partial: BTreeMap<&str, String> = BTreeMap::new();
        partial.insert("type", "'episode'".to_string());
        let err = super::node_row_values(&partial).expect_err("missing columns must be rejected");
        assert!(
            format!("{err}").contains("tier"),
            "error names a missing column: {err}"
        );

        let mut complete: BTreeMap<&str, String> = BTreeMap::new();
        for col in super::NODE_VALUE_COLUMNS {
            complete.insert(col, "null".to_string());
        }
        let rendered = super::node_row_values(&complete).expect("complete map renders");
        assert_eq!(
            rendered.split(", ").count(),
            super::NODE_VALUE_COLUMNS.len(),
            "one value per column, in schema order"
        );
    }

    /// `merge_tags` keeps foreign tags, drops the re-derived families, and
    /// dedups exact matches while preserving order.
    #[test]
    fn merge_tags_drops_families_and_dedups() {
        let current = vec![
            "custom:x".to_string(),
            "status:open".to_string(),
            "lang:en".to_string(),
            "topic:auth".to_string(),
        ];
        let merged = merge_tags(
            &current,
            &["status:", "lang:"],
            vec![
                "status:done".to_string(),
                "lang:en".to_string(),
                "topic:auth".to_string(),
            ],
        );
        assert_eq!(
            merged,
            vec![
                "custom:x".to_string(),
                "topic:auth".to_string(),
                "status:done".to_string(),
                "lang:en".to_string(),
            ]
        );
    }
}
