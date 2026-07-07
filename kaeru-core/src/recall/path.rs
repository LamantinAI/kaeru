//! Semantic shortest-path over agent-weighted edges — the core of the
//! "knowledge chains" feature. Edge `weight` is the agent's judgment of a
//! connection's strength (1 = strong); traversal cost is `1 − weight`, so a
//! chain of strong edges is the shortest path. A tiny per-hop base keeps the
//! all-default-weight case from degenerating to zero cost (fewer / stronger
//! hops win). Uses Cozo's `ShortestPathDijkstra` graph algorithm.

use std::collections::{BTreeMap, HashSet};

use cozo::{DataValue, ScriptMutability};

use super::{NodeBrief, node_brief_by_id};
use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

/// Ordered node ids along the shortest weighted path from `from` to `to`,
/// or an empty vec when unreachable. Initiative-scoped via
/// `current_initiative`; with no active initiative it is cross-initiative.
pub fn shortest_path(store: &Store, from: &NodeId, to: &NodeId) -> Result<Vec<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("from".to_string(), DataValue::Str(from.clone().into()));
    params.insert("to".to_string(), DataValue::Str(to.clone().into()));

    // Floor below which an edge is too weak to carry a chain. `0.0` (the
    // default) lets everything through; the literal is inlined into the
    // Datalog rule because Cozo can't bind a `$param` inside a comparison
    // on a fixed-rule input relation.
    let minw = store.config().chain_min_weight.clamp(0.0, 1.0);

    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            format!(
                r#"
            edges[a, b, c] := *edge{{src: a, dst: b, weight @ 'NOW'}},
                              weight >= {minw:.6},
                              *node_initiative{{initiative, node_id: a}}, initiative = $init,
                              *node_initiative{{initiative: i2, node_id: b}}, i2 = $init,
                              c = 1.0 - weight + 0.001
            starting[a] := a = $from
            goals[a] := a = $to
            ?[start, goal, dist, path] <~ ShortestPathDijkstra(edges[], starting[], goals[])
            "#
            )
        }
        None => format!(
            r#"
            edges[a, b, c] := *edge{{src: a, dst: b, weight @ 'NOW'}},
                              weight >= {minw:.6},
                              c = 1.0 - weight + 0.001
            starting[a] := a = $from
            goals[a] := a = $to
            ?[start, goal, dist, path] <~ ShortestPathDijkstra(edges[], starting[], goals[])
            "#
        ),
    };

    let rows = store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Immutable)?;

    let path = rows
        .rows
        .first()
        .and_then(|r| r.get(3))
        .map(|v| match v {
            DataValue::List(items) => items
                .iter()
                .filter_map(|x| x.get_str().map(String::from))
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        })
        .unwrap_or_default();
    Ok(path)
}

/// The scoped edge set as `[src, dst, cost]` rows — one scan, reusable across
/// many [`shortest_path_over`] calls so a batch (e.g. checking every chain in
/// `reflect`) doesn't re-scan `*edge` per call. Mirrors the edge rule inside
/// [`shortest_path`]: `cost = 1 - weight + 0.001`, weight-floored, and
/// initiative-scoped when a scope is active.
pub(crate) fn scoped_edge_rows(store: &Store) -> Result<Vec<DataValue>> {
    let minw = store.config().chain_min_weight.clamp(0.0, 1.0);
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            format!(
                r#"
                ?[a, b, c] := *edge{{src: a, dst: b, weight @ 'NOW'}},
                              weight >= {minw:.6},
                              *node_initiative{{initiative, node_id: a}}, initiative = $init,
                              *node_initiative{{initiative: i2, node_id: b}}, i2 = $init,
                              c = 1.0 - weight + 0.001
                "#
            )
        }
        None => format!(
            r#"
            ?[a, b, c] := *edge{{src: a, dst: b, weight @ 'NOW'}},
                          weight >= {minw:.6},
                          c = 1.0 - weight + 0.001
            "#
        ),
    };
    let rows = store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Immutable)?;
    Ok(rows
        .rows
        .iter()
        .map(|r| DataValue::List(r.to_vec()))
        .collect())
}

/// Shortest weighted path `from → to` over a pre-fetched edge set (from
/// [`scoped_edge_rows`]). Runs the same Cozo `ShortestPathDijkstra` as
/// [`shortest_path`], so the result is identical — it just skips the per-call
/// `*edge` scan by feeding the edges in as a constant relation.
pub(crate) fn shortest_path_over(
    store: &Store,
    edges: &[DataValue],
    from: &NodeId,
    to: &NodeId,
) -> Result<Vec<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("edges".to_string(), DataValue::List(edges.to_vec()));
    params.insert("from".to_string(), DataValue::Str(from.clone().into()));
    params.insert("to".to_string(), DataValue::Str(to.clone().into()));
    let script = r#"
        edges[a, b, c] <- $edges
        starting[a] := a = $from
        goals[a] := a = $to
        ?[start, goal, dist, path] <~ ShortestPathDijkstra(edges[], starting[], goals[])
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    Ok(rows
        .rows
        .first()
        .and_then(|r| r.get(3))
        .map(|v| match v {
            DataValue::List(items) => items
                .iter()
                .filter_map(|x| x.get_str().map(String::from))
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        })
        .unwrap_or_default())
}

/// Lists the chains a node belongs to — one `NodeBrief` per `Chain` node the
/// given node is a member of (deduplicated). The recall move when a single
/// node is context-poor: see which chains it's in, then `read_chain`.
pub fn chains_of(store: &Store, node_id: &NodeId) -> Result<Vec<NodeBrief>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("nid".to_string(), DataValue::Str(node_id.clone().into()));
    let rows = store.db_ref().run_script(
        "?[chain_id] := *chain_member{chain_id, node_id}, node_id = $nid",
        params,
        ScriptMutability::Immutable,
    )?;

    let mut briefs = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for r in &rows.rows {
        if let Some(cid) = r.first().and_then(|v| v.get_str()) {
            if seen.insert(cid.to_string()) {
                if let Some(b) = node_brief_by_id(store, &cid.to_string())? {
                    briefs.push(b);
                }
            }
        }
    }
    Ok(briefs)
}

/// Returns a chain's members **in order** — the reasoning trail. Each entry
/// is the member node's brief; read the trail to get connected context
/// instead of an isolated node.
pub fn read_chain(store: &Store, chain_id: &NodeId) -> Result<Vec<NodeBrief>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("cid".to_string(), DataValue::Str(chain_id.clone().into()));
    let rows = store.db_ref().run_script(
        r#"
        ?[position, node_id] := *chain_member{chain_id, position, node_id}, chain_id = $cid
        :order position
        "#,
        params,
        ScriptMutability::Immutable,
    )?;

    let mut briefs = Vec::new();
    for r in &rows.rows {
        if let Some(nid) = r.get(1).and_then(|v| v.get_str()) {
            if let Some(b) = node_brief_by_id(store, &nid.to_string())? {
                briefs.push(b);
            }
        }
    }
    Ok(briefs)
}

#[cfg(test)]
mod tests {
    use super::shortest_path;
    use crate::config::KaeruConfig;
    use crate::graph::EdgeType;
    use crate::store::Store;
    use crate::{EpisodeKind, Significance, link_with_weight, write_episode};

    /// Two 2-hop routes a→d: a strong one (a-c-d, w=0.9) and a weak one
    /// (a-b-d, w=0.2). The path must thread the strong route.
    #[test]
    fn shortest_path_prefers_strong_edges() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("p");
        let a = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "a",
            "A",
        )
        .unwrap();
        let b = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "b",
            "B",
        )
        .unwrap();
        let c = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "c",
            "C",
        )
        .unwrap();
        let d = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "d",
            "D",
        )
        .unwrap();

        link_with_weight(&store, &a, &b, EdgeType::RefersTo, 0.2).unwrap();
        link_with_weight(&store, &b, &d, EdgeType::RefersTo, 0.2).unwrap();
        link_with_weight(&store, &a, &c, EdgeType::RefersTo, 0.9).unwrap();
        link_with_weight(&store, &c, &d, EdgeType::RefersTo, 0.9).unwrap();

        let path = shortest_path(&store, &a, &d).unwrap();
        assert_eq!(path.first(), Some(&a), "starts at a; got {path:?}");
        assert_eq!(path.last(), Some(&d), "ends at d; got {path:?}");
        assert!(path.contains(&c), "strong route a-c-d chosen; got {path:?}");
        assert!(!path.contains(&b), "weak route a-b-d avoided; got {path:?}");
    }

    /// With `chain_min_weight` raised above the only available route's edge
    /// weights, those edges drop out and the pair becomes unreachable.
    #[test]
    fn shortest_path_min_weight_filters_weak_edges() {
        let mut cfg = KaeruConfig::defaults();
        cfg.chain_min_weight = 0.5;
        let store = Store::open_in_memory_with(cfg).expect("open");
        store.use_initiative("p");
        let a = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "a",
            "A",
        )
        .unwrap();
        let b = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "b",
            "B",
        )
        .unwrap();
        // Only a weak route exists; the floor removes it.
        link_with_weight(&store, &a, &b, EdgeType::RefersTo, 0.2).unwrap();
        assert!(
            shortest_path(&store, &a, &b).unwrap().is_empty(),
            "weak edge filtered out"
        );

        // A strong-enough edge survives the same floor.
        let c = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "c",
            "C",
        )
        .unwrap();
        link_with_weight(&store, &a, &c, EdgeType::RefersTo, 0.8).unwrap();
        assert_eq!(
            shortest_path(&store, &a, &c).unwrap(),
            vec![a, c],
            "strong edge kept"
        );
    }
}
