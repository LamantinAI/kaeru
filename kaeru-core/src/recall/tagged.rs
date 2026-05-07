//! `tagged` — list nodes whose `tags` array contains the given tag at
//! NOW. Slices the graph by tag (`kind:observation`, `sig:high`,
//! `role:review`, …).

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;

use crate::errors::Result;
use crate::store::Store;

use super::NodeBrief;
use super::parse_brief;

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
