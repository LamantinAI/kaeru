//! `under_review_pinned` — open-review queue surfaced by
//! `mark_under_review`. Targets of inbound `contradicts` edges valid at NOW.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;

use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

/// Returns nodes that have inbound `contradicts` edges valid at NOW —
/// the open-review queue surfaced by `mark_under_review`.
///
/// These are the targets of an unresolved review: a downstream `lint` or
/// session-restoration pass uses this to surface "things you flagged but
/// didn't close" so they don't fall out of attention.
///
/// If the store carries a current initiative, the query also joins
/// `node_initiative` so only targets that belong to that initiative
/// surface.
pub fn under_review_pinned(store: &Store) -> Result<Vec<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
                ?[target] := *edge{src, dst: target, edge_type @ 'NOW'},
                              edge_type = 'contradicts',
                              *node_initiative{initiative, node_id: target},
                              initiative = $init
            "#
        }
        None => {
            r#"
                ?[target] := *edge{src, dst: target, edge_type @ 'NOW'},
                              edge_type = 'contradicts'
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    let ids: Vec<NodeId> = rows
        .rows
        .iter()
        .filter_map(|row| row.first().and_then(|v| v.get_str()).map(String::from))
        .collect();
    Ok(ids)
}
