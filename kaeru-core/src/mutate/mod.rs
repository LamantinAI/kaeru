//! Active-mutation primitives: `write_episode`, `link`, `synthesise`, …
//!
//! Each primitive is a graph mutation that automatically writes an
//! `audit_event` node alongside the domain change. Submodules group
//! primitives by the shape of the mutation they perform; this `mod.rs`
//! re-exports the public surface and houses cross-submodule helpers
//! (timestamp generation, RMW reads).

use std::collections::{BTreeMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

use cozo::{DataValue, ScriptMutability};

use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

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
pub mod supersedes;
pub mod synthesise;
pub mod task;

pub use chain::{ChainOutcome, RechainStats, create_chain, extend_chain, regenerate_chain};
pub use cite::{cite, cite_with_layer};
pub use consolidate::{consolidate_in, consolidate_out};
pub use edge::{link, link_remote, link_remote_to, link_with_weight, set_edge_weight, unlink};
pub use episode::{jot, jot_with_layer, write_episode, write_episode_with_layer};
pub use hypothesis::{
    formulate_hypothesis, formulate_hypothesis_with_layer, run_experiment, update_hypothesis_status,
};
pub use ingest::{upsert_edge, upsert_node};
pub use initiative::{
    AttachStats, DeleteStats, RenameStats, attach_node, delete_initiative, rename_initiative,
};
pub use layer::{get_layer, set_layer};
pub use metabolism::{forget, improve};
pub use review::{mark_resolved, mark_under_review, resolve_review};
pub use sharing::{get_share_policy, get_visibility, set_share_policy, set_visibility};
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

/// Maximum number of `topic:<word>` tags derived from a body.
/// Keeps the tag list bounded; bumps later if needed.
const MAX_TOPIC_TOKENS: usize = 5;

/// Extracts up to [`MAX_TOPIC_TOKENS`] significant content tokens from
/// `body` — lowercased, alphanumeric (Unicode-aware, so Cyrillic /
/// CJK survive), length ≥ 3, deduped, basic stop-words removed. Used
/// to build `topic:<word>` tags so nodes can be sliced by content via
/// `tagged "topic:<word>"`.
///
/// Returns `Vec<String>` of just the tokens themselves (without the
/// `topic:` prefix); call sites do that wrapping.
pub(crate) fn derive_topic_tokens(body: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for raw in body.split_whitespace() {
        let cleaned: String = raw
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
            .collect::<String>()
            .to_lowercase();
        if cleaned.chars().count() < 3 || is_stop_word(&cleaned) || !seen.insert(cleaned.clone()) {
            continue;
        }
        out.push(cleaned);
        if out.len() >= MAX_TOPIC_TOKENS {
            break;
        }
    }
    out
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

/// Convenience: builds the tags list for a write that has a body.
/// Combines fixed prefix tags (caller-specified) with the auto-derived
/// `lang:*` and `topic:<word>` tags.
pub(crate) fn build_body_tags(fixed: &[&str], body: &str) -> Vec<String> {
    let mut tags: Vec<String> = fixed.iter().map(|s| (*s).to_string()).collect();
    tags.push(detect_lang_tag(body));
    for token in derive_topic_tokens(body) {
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
pub(crate) fn initiatives_of_node(store: &Store, node_id: &NodeId) -> Result<Vec<String>> {
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
mod tests {
    use std::collections::BTreeMap;

    use cozo::{DataValue, ScriptMutability};

    use super::{
        NODE_VALUE_COLUMNS, ReassertRow, merge_tags, read_node_now, reassert_node_now,
        retract_node_at,
    };
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
