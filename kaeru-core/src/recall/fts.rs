//! Full-text search across `name` and `body` via Cozo's BM25-ish FTS
//! indexes (`node:fts_name`, `node:fts_body`). Fallback for cold queries
//! where the agent doesn't remember an exact name.
//!
//! Hits are unioned across both indexes, anchored at NOW (so retracted
//! rows don't surface), filtered by `current_initiative` when set, and
//! deduplicated by node id. The score that wins for a duplicate id is
//! the larger of the per-index scores.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::errors::Result;
use crate::store::Store;

use super::NodeBrief;
use super::parse_brief;

/// Maximum results [`fuzzy_recall`] may return per call. Mirrors the
/// pattern used elsewhere — bound the working set so the agent's
/// attention budget stays small.
pub const FUZZY_RECALL_LIMIT_CAP: usize = 50;

/// Searches the substrate for nodes whose `name` or `body` matches
/// `query`. Returns at most `limit` briefs ordered by descending FTS
/// score. `limit` is clamped to [`FUZZY_RECALL_LIMIT_CAP`].
///
/// `query` is the Cozo FTS expression — single tokens, `AND` / `OR` /
/// `NOT`, or quoted phrases. See Cozo docs for the full grammar.
pub fn fuzzy_recall(store: &Store, query: &str, limit: usize) -> Result<Vec<NodeBrief>> {
    let limit = limit.min(FUZZY_RECALL_LIMIT_CAP);
    if limit == 0 {
        return Ok(Vec::new());
    }
    let excerpt_chars = store.config().body_excerpt_chars;

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("q".to_string(), DataValue::Str(query.into()));

    // The FTS atom requires a literal `k` and `query` parameters; we
    // inline `k` into the script and pass `q` as a Datalog parameter so
    // user input never reaches the script source.
    //
    // Initiative-scoped: an extra `*node_initiative` join trims hits.
    // Cross-initiative: just a NOW anchor on the base relation.
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            format!(
                r#"
                hits[id, score] := ~node:fts_name{{id | query: $q, k: {limit}, bind_score: score}}
                hits[id, score] := ~node:fts_body{{id | query: $q, k: {limit}, bind_score: score}}

                ?[id, type, name, body, score, validity] :=
                    hits[id, score],
                    *node{{id, type, name, body, validity @ 'NOW'}},
                    *node_initiative{{initiative, node_id: id}},
                    initiative = $init
                :order -score, validity
                "#
            )
        }
        None => format!(
            r#"
            hits[id, score] := ~node:fts_name{{id | query: $q, k: {limit}, bind_score: score}}
            hits[id, score] := ~node:fts_body{{id | query: $q, k: {limit}, bind_score: score}}

            ?[id, type, name, body, score, validity] :=
                hits[id, score],
                *node{{id, type, name, body, validity @ 'NOW'}}
            :order -score, validity
            "#
        ),
    };

    let rows = store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Immutable)?;

    // Multiple FTS hits per id (one from name, one from body) come back
    // as separate rows; keep the highest-score row per id, preserving
    // overall descending-score ordering.
    let mut best: HashMap<String, (f64, NodeBrief)> = HashMap::new();
    let mut order: Vec<String> = Vec::new();
    for row in &rows.rows {
        let Some(id) = row.first().and_then(|v| v.get_str()).map(String::from) else {
            continue;
        };
        let score = row
            .get(4)
            .and_then(|v| v.get_float())
            .unwrap_or(0.0);
        let brief = parse_brief(row.as_slice(), excerpt_chars);
        match best.get(&id) {
            Some((prev, _)) if *prev >= score => continue,
            None => order.push(id.clone()),
            _ => {}
        }
        best.insert(id, (score, brief));
    }

    let mut out: Vec<(f64, NodeBrief)> = order
        .into_iter()
        .filter_map(|id| best.remove(&id))
        .collect();
    // Re-sort because dedup might have stuck a higher-score body hit
    // behind a lower-score name hit in `order`.
    out.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    Ok(out.into_iter().take(limit).map(|(_, b)| b).collect())
}
