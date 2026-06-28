//! `reflect` — the maintenance work-list for a reflection pass.
//!
//! Where [`lint`](crate::lint) returns raw hygiene issues, `reflect` computes a
//! fuller, actionable picture: orphans and open reviews (from `lint`), chains
//! the graph has made stale, operational nodes that have settled (cortex
//! candidates), and the shared nodes whose rebalancing must be escalated to the
//! user. The store computes *what* to fix; the adapter pairs it with *how*.

use std::collections::BTreeMap;

use cozo::{DataValue, NamedRows, ScriptMutability};

use super::{lint, shortest_path};
use crate::errors::Result;
use crate::graph::NodeId;
use crate::graph::temporal::validity_seconds;
use crate::mutate::now_validity_seconds;
use crate::store::Store;

/// The computed reflection work-list. Each field is a set of node ids to act
/// on; an all-empty report means the store is already tidy.
#[derive(Debug, Clone, Default)]
pub struct ReflectionReport {
    /// Nodes with no edges at NOW — `link` them or `forget` them.
    pub orphans: Vec<NodeId>,
    /// Nodes with an open `contradicts` review — `resolve` or `refute`.
    pub open_reviews: Vec<NodeId>,
    /// Chains whose stored members no longer match the shortest path between
    /// their endpoints (the graph changed underneath, or the endpoints became
    /// unreachable) — `rechain` to refresh.
    pub stale_chains: Vec<NodeId>,
    /// Operational, linked nodes untouched past `reflect_settle_age_secs` —
    /// settled work to `settle` / `cite` into the archival cortex.
    pub cortex_candidates: Vec<NodeId>,
    /// Shared nodes in scope. Touching the cloud (re-share, edge rebalance) is
    /// the user's call — escalate, don't auto-rewrite.
    pub shared: Vec<NodeId>,
}

/// Builds the reflection work-list for the active initiative (cross-initiative
/// when none is selected). Read-only — it computes, never mutates.
pub fn reflect(store: &Store) -> Result<ReflectionReport> {
    let hygiene = lint(store)?;
    Ok(ReflectionReport {
        orphans: hygiene.orphans,
        open_reviews: hygiene.unresolved_reviews,
        stale_chains: stale_chains(store)?,
        cortex_candidates: cortex_candidates(store)?,
        shared: shared_nodes(store)?,
    })
}

fn first_col_ids(rows: &NamedRows) -> Vec<NodeId> {
    rows.rows
        .iter()
        .filter_map(|r| r.first().and_then(|v| v.get_str()).map(String::from))
        .collect()
}

/// Chains whose materialised path is out of date.
fn stale_chains(store: &Store) -> Result<Vec<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
            ?[id] := *node{id, type @ 'NOW'}, type = 'chain',
                     *node_initiative{initiative, node_id: id}, initiative = $init
            "#
        }
        None => r#"?[id] := *node{id, type @ 'NOW'}, type = 'chain'"#,
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let mut stale = Vec::new();
    for cid in first_col_ids(&rows) {
        let members = chain_members(store, &cid)?;
        if members.len() < 2 {
            continue;
        }
        let recomputed = shortest_path(store, &members[0], &members[members.len() - 1])?;
        if recomputed != members {
            stale.push(cid);
        }
    }
    Ok(stale)
}

/// Ordered member ids of a chain, raw from the junction.
fn chain_members(store: &Store, chain_id: &NodeId) -> Result<Vec<NodeId>> {
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
    Ok(rows
        .rows
        .iter()
        .filter_map(|r| r.get(1).and_then(|v| v.get_str()).map(String::from))
        .collect())
}

/// Operational, linked nodes that have sat untouched long enough to look
/// settled — candidates to consolidate into the archival cortex.
fn cortex_candidates(store: &Store) -> Result<Vec<NodeId>> {
    let cutoff = now_validity_seconds() as f64 - store.config().reflect_settle_age_secs as f64;
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
            connected[id] := *edge{src: id, dst, edge_type @ 'NOW'}
            connected[id] := *edge{src, dst: id, edge_type @ 'NOW'}
            ?[id, validity] := *node{id, type, tier, validity @ 'NOW'},
                               tier = 'operational', type != 'audit_event',
                               connected[id],
                               *node_initiative{initiative, node_id: id}, initiative = $init
            :order validity
            "#
        }
        None => {
            r#"
            connected[id] := *edge{src: id, dst, edge_type @ 'NOW'}
            connected[id] := *edge{src, dst: id, edge_type @ 'NOW'}
            ?[id, validity] := *node{id, type, tier, validity @ 'NOW'},
                               tier = 'operational', type != 'audit_event',
                               connected[id]
            :order validity
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    Ok(rows
        .rows
        .iter()
        .filter(|r| validity_seconds(r.last()).is_some_and(|ts| ts < cutoff))
        .filter_map(|r| r.first().and_then(|v| v.get_str()).map(String::from))
        .collect())
}

/// Shared nodes in scope — any cloud-touching rebalance is the user's call.
fn shared_nodes(store: &Store) -> Result<Vec<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
            ?[id] := *node{id, visibility @ 'NOW'}, visibility = 'shared',
                     *node_initiative{initiative, node_id: id}, initiative = $init
            "#
        }
        None => r#"?[id] := *node{id, visibility @ 'NOW'}, visibility = 'shared'"#,
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    Ok(first_col_ids(&rows))
}

#[cfg(test)]
mod tests {
    use super::reflect;
    use crate::config::KaeruConfig;
    use crate::graph::EdgeType;
    use crate::store::Store;
    use crate::{EpisodeKind, Significance, link_with_weight, mark_under_review, write_episode};

    #[test]
    fn reflect_flags_orphans_and_open_reviews() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("p");
        let orphan = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "lonely",
            "no edges",
        )
        .unwrap();
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
        link_with_weight(&store, &a, &b, EdgeType::RefersTo, 0.9).unwrap();
        mark_under_review(&store, &a, "needs another look").unwrap();

        let r = reflect(&store).unwrap();
        assert!(r.orphans.contains(&orphan), "orphan flagged");
        assert!(!r.orphans.contains(&a), "linked node not an orphan");
        assert!(r.open_reviews.contains(&a), "open review flagged");
    }

    #[test]
    fn reflect_flags_stale_chain_after_graph_change() {
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
        link_with_weight(&store, &a, &b, EdgeType::RefersTo, 0.9).unwrap();
        link_with_weight(&store, &b, &c, EdgeType::RefersTo, 0.9).unwrap();
        let chain = crate::create_chain(&store, &a, &c, None, None)
            .unwrap()
            .unwrap()
            .id;

        assert!(
            reflect(&store).unwrap().stale_chains.is_empty(),
            "fresh chain not stale"
        );

        // A strong direct edge changes the shortest path → chain is stale.
        link_with_weight(&store, &a, &c, EdgeType::RefersTo, 1.0).unwrap();
        assert!(
            reflect(&store).unwrap().stale_chains.contains(&chain),
            "chain flagged stale after the graph changed"
        );
    }

    #[test]
    fn reflect_flags_settled_node_as_cortex_candidate() {
        let mut cfg = KaeruConfig::defaults();
        cfg.reflect_settle_age_secs = 0; // everything linked counts as settled
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
        link_with_weight(&store, &a, &b, EdgeType::RefersTo, 0.5).unwrap();

        // Cross the whole-second boundary so the assertion is strictly older
        // than the (now) cutoff.
        std::thread::sleep(std::time::Duration::from_millis(1100));
        let r = reflect(&store).unwrap();
        assert!(
            r.cortex_candidates.contains(&a),
            "linked operational node is a cortex candidate"
        );
        assert!(r.cortex_candidates.contains(&b));
    }
}
