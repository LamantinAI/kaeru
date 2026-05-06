//! `between` — list every edge that connects two named nodes (in
//! either direction) at NOW.
//!
//! Answers the agent question "why are A and B connected?" — neither
//! `walk` (typed traversal) nor `summary_view` (1-hop drill-down)
//! cover this directly because they don't enumerate edges between a
//! specific pair.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;

use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

/// One edge that connects the pair queried by [`between`].
///
/// `direction` is `true` when the edge points from the first argument
/// (`a`) to the second (`b`), `false` for the reverse direction.
#[derive(Debug, Clone, PartialEq)]
pub struct EdgeRow {
    pub edge_type: String,
    pub a_to_b: bool,
}

/// Returns every edge valid at NOW between `a` and `b`, in either
/// direction. Initiative-scoped via the store's `current_initiative`
/// (each endpoint must be in scope, mirroring `read_edges` in
/// `export`). Empty result means the pair is unconnected at NOW.
pub fn between(store: &Store, a: &NodeId, b: &NodeId) -> Result<Vec<EdgeRow>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("a".to_string(), DataValue::Str(a.clone().into()));
    params.insert("b".to_string(), DataValue::Str(b.clone().into()));

    // Two rules unioned: a → b and b → a. Initiative scope, when set,
    // requires both endpoints to be attached.
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
                forward[edge_type] := *edge{src, dst, edge_type @ 'NOW'},
                                       src = $a, dst = $b,
                                       *node_initiative{initiative, node_id: src},
                                       initiative = $init,
                                       *node_initiative{initiative: i2, node_id: dst},
                                       i2 = $init
                reverse[edge_type] := *edge{src, dst, edge_type @ 'NOW'},
                                       src = $b, dst = $a,
                                       *node_initiative{initiative, node_id: src},
                                       initiative = $init,
                                       *node_initiative{initiative: i2, node_id: dst},
                                       i2 = $init
                ?[edge_type, a_to_b] := forward[edge_type], a_to_b = true
                ?[edge_type, a_to_b] := reverse[edge_type], a_to_b = false
            "#
        }
        None => {
            r#"
                forward[edge_type] := *edge{src, dst, edge_type @ 'NOW'},
                                       src = $a, dst = $b
                reverse[edge_type] := *edge{src, dst, edge_type @ 'NOW'},
                                       src = $b, dst = $a
                ?[edge_type, a_to_b] := forward[edge_type], a_to_b = true
                ?[edge_type, a_to_b] := reverse[edge_type], a_to_b = false
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let edges = rows
        .rows
        .iter()
        .filter_map(|row| {
            let edge_type = row.first().and_then(|v| v.get_str())?.to_string();
            let a_to_b = row.get(1).and_then(|v| v.get_bool())?;
            Some(EdgeRow { edge_type, a_to_b })
        })
        .collect();
    Ok(edges)
}
