//! Typed graph traversal from a seed node, following only edges in
//! `edge_types`, up to `max_hops` hops away.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;

use crate::graph::EdgeType;
use crate::errors::Error;
use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

/// Typed graph traversal from a seed node, following only edges in
/// `edge_types`, up to `max_hops` hops away.
///
/// `max_hops` is bounded by `config().max_hops_cap`; any larger value
/// returns `Error::Invalid` rather than silently truncating. The seed
/// itself is included in the result.
pub fn walk(
    store: &Store,
    seed: &NodeId,
    edge_types: &[EdgeType],
    max_hops: u8,
) -> Result<Vec<NodeId>> {
    let cap = store.config().max_hops_cap;
    if max_hops > cap {
        return Err(Error::Invalid(format!(
            "max_hops {max_hops} exceeds cap {cap}"
        )));
    }
    if edge_types.is_empty() {
        return Err(Error::Invalid(
            "edge_types must list at least one EdgeType to traverse".to_string(),
        ));
    }

    let allowed_lit = format!(
        "[{}]",
        edge_types
            .iter()
            .map(|et| format!("'{}'", et.as_str()))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("seed".to_string(), DataValue::Str(seed.clone().into()));

    // Recursive Datalog: base case is the seed at hop 0; recursive case
    // extends one hop at a time, bounded by `max_hops`. Edge types are
    // inlined as a literal list (List params trip `eval::not_constant`).
    //
    // Initiative-scoped variant: every visited node (seed + recursive
    // children) must be attached to the active initiative through
    // `node_initiative`. A walk from a seed outside the initiative
    // returns empty; nodes leaking out of the initiative through a
    // typed edge are not followed.
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            format!(
                r#"
                walk[node, hops] := node = $seed, hops = 0,
                                    *node_initiative{{initiative, node_id: node}},
                                    initiative = $init
                walk[node, hops] := walk[prev, h], h < {max_hops}, hops = h + 1,
                                    *edge{{src: prev, dst: node, edge_type: et @ 'NOW'}},
                                    is_in(et, {allowed_lit}),
                                    *node_initiative{{initiative, node_id: node}},
                                    initiative = $init

                ?[id] := walk[id, _]
                "#
            )
        }
        None => format!(
            r#"
            walk[node, hops] := node = $seed, hops = 0
            walk[node, hops] := walk[prev, h], h < {max_hops}, hops = h + 1,
                                *edge{{src: prev, dst: node, edge_type: et @ 'NOW'}},
                                is_in(et, {allowed_lit})

            ?[id] := walk[id, _]
            "#
        ),
    };

    let rows = store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Immutable)?;

    let ids: Vec<NodeId> = rows
        .rows
        .iter()
        .filter_map(|row| row.first().and_then(|v| v.get_str()).map(String::from))
        .collect();
    Ok(ids)
}
