//! `create_chain` — materialize a "knowledge chain": the shortest weighted
//! path between two nodes, saved as a first-class `Chain` node plus an
//! ordered `chain_member` list. A node can then list the chains it's in
//! (`chains_of`) and read the whole reasoning trail (`read_chain`).

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};

use crate::errors::{Error, Result};
use crate::graph::audit::write_audit;
use crate::graph::{Layer, NodeId, NodeType, Tier, Visibility, new_node_id};
use crate::recall::{node_brief_by_id, shortest_path};
use crate::store::Store;

use super::upsert_node;

/// Computes the shortest weighted path `from → to` and saves it as a chain.
/// Returns the new chain node id, or `None` when the two are unreachable
/// (no path). `name` defaults to one derived from the endpoints. The chain
/// node lands in the active initiative; chains are `local` for now (cloud /
/// cross-initiative chains are a later pass).
pub fn create_chain(
    store: &Store,
    from: &NodeId,
    to: &NodeId,
    name: Option<&str>,
) -> Result<Option<NodeId>> {
    let path = shortest_path(store, from, to)?;
    if path.len() < 2 {
        return Ok(None);
    }

    // A chain of N nodes spans N-1 hops. Refuse anything past the cap so a
    // sprawling path can't be frozen into a single unwieldy node.
    let hops = path.len() - 1;
    let cap = store.config().chain_max_hops;
    if hops > cap {
        return Err(Error::Invalid(format!(
            "chain spans {hops} hops, over the cap of {cap} (raise KAERU_CHAIN_MAX_HOPS to allow)"
        )));
    }

    let from_name = node_brief_by_id(store, from)?
        .map(|b| b.name)
        .unwrap_or_default();
    let to_name = node_brief_by_id(store, to)?
        .map(|b| b.name)
        .unwrap_or_default();
    let chain_name = name
        .map(String::from)
        .unwrap_or_else(|| format!("chain-{from_name}-to-{to_name}"));
    let body = format!(
        "reasoning chain: {from_name} → {to_name} ({} nodes)",
        path.len()
    );

    let chain_id = new_node_id();
    let initiative = store.current_initiative();
    upsert_node(
        store,
        &chain_id,
        NodeType::Chain,
        Tier::Operational,
        &chain_name,
        Some(&body),
        &[],
        initiative.as_deref(),
        Visibility::Local,
        Layer::Warm,
    )?;

    for (i, nid) in path.iter().enumerate() {
        let mut p: BTreeMap<String, DataValue> = BTreeMap::new();
        p.insert("cid".to_string(), DataValue::Str(chain_id.clone().into()));
        p.insert("nid".to_string(), DataValue::Str(nid.clone().into()));
        let script = format!(
            r#"
            ?[chain_id, position, node_id] <- [[$cid, {i}, $nid]]
            :put chain_member {{chain_id, position => node_id}}
            "#
        );
        store
            .db_ref()
            .run_script(&script, p, ScriptMutability::Mutable)?;
    }

    write_audit(
        store.db_ref(),
        "create_chain",
        "system",
        &[chain_id.clone(), from.clone(), to.clone()],
    )?;
    Ok(Some(chain_id))
}

#[cfg(test)]
mod tests {
    use super::create_chain;
    use crate::config::KaeruConfig;
    use crate::errors::Error;
    use crate::graph::EdgeType;
    use crate::store::Store;
    use crate::{
        EpisodeKind, Significance, chains_of, link_with_weight, read_chain, write_episode,
    };

    #[test]
    fn create_chain_saves_ordered_path_and_membership() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("p");
        let a = write_episode(&store, EpisodeKind::Observation, Significance::Low, "a", "A").unwrap();
        let b = write_episode(&store, EpisodeKind::Observation, Significance::Low, "b", "B").unwrap();
        let c = write_episode(&store, EpisodeKind::Observation, Significance::Low, "c", "C").unwrap();
        // Strong straight line a→b→c.
        link_with_weight(&store, &a, &b, EdgeType::RefersTo, 0.9).unwrap();
        link_with_weight(&store, &b, &c, EdgeType::RefersTo, 0.9).unwrap();

        let chain_id = create_chain(&store, &a, &c, Some("auth-trail")).unwrap().expect("path exists");

        // read_chain returns the ordered members a, b, c.
        let members = read_chain(&store, &chain_id).unwrap();
        let ids: Vec<&String> = members.iter().map(|m| &m.id).collect();
        assert_eq!(ids, vec![&a, &b, &c], "ordered trail; got {ids:?}");

        // chains_of(b) lists this chain.
        let chains = chains_of(&store, &b).unwrap();
        assert!(chains.iter().any(|ch| ch.id == chain_id), "b knows its chain");
        assert_eq!(chains[0].node_type, "chain");
        assert_eq!(chains[0].name, "auth-trail");

        // Unreachable pair → None.
        let lonely = write_episode(&store, EpisodeKind::Observation, Significance::Low, "lonely", "L").unwrap();
        assert!(create_chain(&store, &a, &lonely, None).unwrap().is_none());
    }

    /// A path longer than `chain_max_hops` is refused with `Error::Invalid`.
    #[test]
    fn create_chain_refuses_path_over_hop_cap() {
        let mut cfg = KaeruConfig::defaults();
        cfg.chain_max_hops = 1; // allow a single hop only
        let store = Store::open_in_memory_with(cfg).expect("open");
        store.use_initiative("p");
        let a = write_episode(&store, EpisodeKind::Observation, Significance::Low, "a", "A").unwrap();
        let b = write_episode(&store, EpisodeKind::Observation, Significance::Low, "b", "B").unwrap();
        let c = write_episode(&store, EpisodeKind::Observation, Significance::Low, "c", "C").unwrap();
        link_with_weight(&store, &a, &b, EdgeType::RefersTo, 0.9).unwrap();
        link_with_weight(&store, &b, &c, EdgeType::RefersTo, 0.9).unwrap();

        // a→b is one hop: fine.
        assert!(create_chain(&store, &a, &b, None).unwrap().is_some());
        // a→b→c is two hops: over the cap.
        match create_chain(&store, &a, &c, None) {
            Err(Error::Invalid(msg)) => assert!(msg.contains("hops"), "got {msg}"),
            other => panic!("expected Invalid, got {other:?}"),
        }
    }
}
