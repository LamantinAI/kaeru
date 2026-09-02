//! `neighbours` — every node one hop from a seed, in either direction,
//! across every edge type, each labelled with the edge that connects them.
//!
//! The discovery counterpart to the pair that came before it: `between` needs
//! both endpoints ("are A and B connected?"), and `walk` returns reachable ids
//! without the edges that reached them. `drill` (`summary_view`) follows only
//! two of the twelve edge types — `derived_from` and `part_of` — so a
//! `contradicts`, a `supersedes`, a `refers_to` (the DEFAULT of `link`, and the
//! largest bucket in real vaults) is written but never walked. This follows all
//! of them, so the graph an agent writes is the graph it can read back (#84).

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};

use super::{NodeBrief, parse_brief};
use crate::errors::Result;
use crate::graph::{EdgeType, NodeId};
use crate::store::Store;

/// One immediate neighbour of a seed: the connected node, plus the single edge
/// that connects them and which way it points. A node joined by several edges
/// appears once per edge, so each typed connection is visible on its own.
#[derive(Debug, Clone, PartialEq)]
pub struct Neighbour {
    pub brief: NodeBrief,
    pub edge_type: String,
    /// `true` when the edge points FROM the seed TO this neighbour (outgoing);
    /// `false` when it points from the neighbour to the seed (incoming).
    pub outgoing: bool,
    pub weight: f64,
}

/// Every node one hop from `seed` at NOW, in either direction, following only
/// `local` edges. An empty `edge_types` means no type filter (all of them);
/// otherwise only the listed types are followed. Initiative-scoped via the
/// store's `current_initiative` — a neighbour must be attached to the active
/// initiative, so a walk never leaks nodes from another project. Self-loops and
/// `audit_event` nodes are excluded. Ordered outgoing-first, then newest-first.
pub fn neighbours(store: &Store, seed: &NodeId, edge_types: &[EdgeType]) -> Result<Vec<Neighbour>> {
    let excerpt_chars = store.config().body_excerpt_chars;
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("seed".to_string(), DataValue::Str(seed.clone().into()));

    // Empty filter = every type. Otherwise inline an `is_in(edge_type, [...])`
    // guard — the same literal-list trick `walk` uses, because a List param
    // trips Cozo's `eval::not_constant`.
    let type_guard = if edge_types.is_empty() {
        String::new()
    } else {
        let allowed = edge_types
            .iter()
            .map(|et| format!("'{}'", et.as_str()))
            .collect::<Vec<_>>()
            .join(", ");
        format!(", is_in(edge_type, [{allowed}])")
    };

    // Two source rules — the seed as `src` (outgoing) and as `dst` (incoming) —
    // joined to the node record for each neighbour's fields, and to
    // `node_initiative` for scope when an initiative is set. `weight` rides
    // along so a caller can see which links are load-bearing.
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            format!(
                r#"
                out[dst, edge_type, weight] := *edge{{src, dst, edge_type, weight, dst_store @ 'NOW'}},
                                               src = $seed, dst_store = 'local'{type_guard}
                inc[src, edge_type, weight] := *edge{{src, dst, edge_type, weight, dst_store @ 'NOW'}},
                                               dst = $seed, dst_store = 'local'{type_guard}
                ?[id, type, name, body, edge_type, outgoing, weight, validity] :=
                    out[id, edge_type, weight], outgoing = true, id != $seed,
                    *node{{id, type, name, body, validity @ 'NOW'}}, type != 'audit_event',
                    *node_initiative{{initiative, node_id: id}}, initiative = $init
                ?[id, type, name, body, edge_type, outgoing, weight, validity] :=
                    inc[id, edge_type, weight], outgoing = false, id != $seed,
                    *node{{id, type, name, body, validity @ 'NOW'}}, type != 'audit_event',
                    *node_initiative{{initiative, node_id: id}}, initiative = $init
                "#
            )
        }
        None => format!(
            r#"
            out[dst, edge_type, weight] := *edge{{src, dst, edge_type, weight, dst_store @ 'NOW'}},
                                           src = $seed, dst_store = 'local'{type_guard}
            inc[src, edge_type, weight] := *edge{{src, dst, edge_type, weight, dst_store @ 'NOW'}},
                                           dst = $seed, dst_store = 'local'{type_guard}
            ?[id, type, name, body, edge_type, outgoing, weight, validity] :=
                out[id, edge_type, weight], outgoing = true, id != $seed,
                *node{{id, type, name, body, validity @ 'NOW'}}, type != 'audit_event'
            ?[id, type, name, body, edge_type, outgoing, weight, validity] :=
                inc[id, edge_type, weight], outgoing = false, id != $seed,
                *node{{id, type, name, body, validity @ 'NOW'}}, type != 'audit_event'
            "#
        ),
    };

    let rows = store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Immutable)?;

    let mut result: Vec<Neighbour> = rows
        .rows
        .iter()
        .filter_map(|row| {
            let edge_type = row.get(4).and_then(|v| v.get_str())?.to_string();
            let outgoing = row.get(5).and_then(|v| v.get_bool())?;
            // Neutral weight (0.5) if a legacy edge somehow lacks the column.
            let weight = row.get(6).and_then(|v| v.get_float()).unwrap_or(0.5);
            let brief = parse_brief(row, excerpt_chars);
            Some(Neighbour {
                brief,
                edge_type,
                outgoing,
                weight,
            })
        })
        .collect();

    // Ordered here, not in Datalog: Cozo's `:order` over the boolean `outgoing`
    // column trips an internal BTreeMap range panic. Outgoing edges first, then
    // newest by assertion time.
    result.sort_by(|a, b| {
        b.outgoing.cmp(&a.outgoing).then_with(|| {
            b.brief
                .ts
                .partial_cmp(&a.brief.ts)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    });
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::graph::EdgeType;
    use crate::mutate::{link, write_episode};
    use crate::store::Store;
    use crate::{EpisodeKind, Significance};

    fn store_t() -> Store {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        store
    }

    fn ep(store: &Store, name: &str) -> String {
        write_episode(
            store,
            EpisodeKind::Observation,
            Significance::Low,
            name,
            "body",
        )
        .expect("write")
    }

    /// The reproduction from #84: a refuted node reached only by an INCOMING
    /// `contradicts`. `drill` reports it as attached to nothing; `neighbours`
    /// must show the edge, direction and all.
    #[test]
    fn incoming_contradicts_is_visible() {
        let store = store_t();
        let a = ep(&store, "finding-a");
        let b = ep(&store, "finding-b");
        link(&store, &b, &a, EdgeType::Contradicts).expect("link");

        let ns = super::neighbours(&store, &a, &[]).expect("neighbours");
        assert_eq!(ns.len(), 1, "the contradiction is a neighbour");
        assert_eq!(ns[0].brief.id, b);
        assert_eq!(ns[0].edge_type, "contradicts");
        assert!(!ns[0].outgoing, "b -> a is incoming to a");
    }

    /// All eleven-plus types, both directions, and the type filter.
    #[test]
    fn both_directions_and_type_filter() {
        let store = store_t();
        let seed = ep(&store, "seed");
        let child = ep(&store, "child");
        let source = ep(&store, "source");
        link(&store, &seed, &child, EdgeType::PartOf).expect("link"); // outgoing
        link(&store, &source, &seed, EdgeType::DerivedFrom).expect("link"); // incoming

        let all = super::neighbours(&store, &seed, &[]).expect("all");
        assert_eq!(all.len(), 2, "both edges surface");
        // outgoing sorts first
        assert!(all[0].outgoing && all[0].edge_type == "part_of");
        assert!(!all[1].outgoing && all[1].edge_type == "derived_from");

        let only = super::neighbours(&store, &seed, &[EdgeType::DerivedFrom]).expect("filtered");
        assert_eq!(only.len(), 1, "filter keeps only derived_from");
        assert_eq!(only[0].brief.id, source);
    }

    /// A seed with no edges comes back empty, not in error.
    #[test]
    fn isolated_seed_is_empty() {
        let store = store_t();
        let lonely = ep(&store, "lonely");
        assert!(
            super::neighbours(&store, &lonely, &[])
                .expect("neighbours")
                .is_empty()
        );
    }
}
