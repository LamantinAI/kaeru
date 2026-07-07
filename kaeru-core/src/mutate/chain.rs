//! `create_chain` — materialize a "knowledge chain": the shortest weighted
//! path between two nodes, saved as a first-class `Chain` node plus an
//! ordered `chain_member` list. A node can then list the chains it's in
//! (`chains_of`) and read the whole reasoning trail (`read_chain`).

use std::collections::BTreeMap;
use std::str::FromStr;

use cozo::{DataValue, ScriptMutability};

use super::upsert_node;
use crate::errors::{Error, Result};
use crate::graph::audit::write_audit;
use crate::graph::{Layer, NodeId, NodeType, Tier, Visibility, new_node_id};
use crate::recall::{chains_of, node_brief_by_id, read_node_full, shortest_path};
use crate::store::Store;

/// Result of [`create_chain`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainOutcome {
    pub id: NodeId,
    /// True when an identical chain already existed and was reused instead of
    /// creating a duplicate (dedup at creation time).
    pub reused: bool,
}

/// Counts/flags from a chain mutation ([`regenerate_chain`] / [`extend_chain`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RechainStats {
    /// Member count after the operation.
    pub members: usize,
    /// True if the member list actually changed.
    pub changed: bool,
}

/// Ordered member ids of a chain, straight from the junction. Unlike
/// [`read_chain`](crate::read_chain) it does **not** skip members whose node
/// was later forgotten — callers need the true endpoints and length.
fn chain_member_ids(store: &Store, chain_id: &NodeId) -> Result<Vec<NodeId>> {
    let mut p: BTreeMap<String, DataValue> = BTreeMap::new();
    p.insert("cid".to_string(), DataValue::Str(chain_id.clone().into()));
    let rows = store.db_ref().run_script(
        r#"
        ?[position, node_id] := *chain_member{chain_id, position, node_id}, chain_id = $cid
        :order position
        "#,
        p,
        ScriptMutability::Immutable,
    )?;
    Ok(rows
        .rows
        .iter()
        .filter_map(|r| r.get(1).and_then(|v| v.get_str()).map(String::from))
        .collect())
}

/// Replaces a chain's member list wholesale: drops every existing
/// `chain_member` row for `chain_id`, then writes `members` in order.
fn replace_members(store: &Store, chain_id: &NodeId, members: &[NodeId]) -> Result<()> {
    let mut clear: BTreeMap<String, DataValue> = BTreeMap::new();
    clear.insert("cid".to_string(), DataValue::Str(chain_id.clone().into()));
    store.db_ref().run_script(
        r#"
        ?[chain_id, position] := *chain_member{chain_id, position}, chain_id = $cid
        :rm chain_member {chain_id, position}
        "#,
        clear,
        ScriptMutability::Mutable,
    )?;
    for (i, nid) in members.iter().enumerate() {
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
    Ok(())
}

/// Finds an existing chain whose ordered member list is exactly `path`, so a
/// deterministic repeat of [`create_chain`] reuses it instead of duplicating.
fn find_duplicate_chain(store: &Store, path: &[NodeId]) -> Result<Option<NodeId>> {
    let Some(first) = path.first() else {
        return Ok(None);
    };
    for cand in chains_of(store, first)? {
        if chain_member_ids(store, &cand.id)? == path {
            return Ok(Some(cand.id));
        }
    }
    Ok(None)
}

/// Attaches `chain_id` to the union of `from`'s and `to`'s initiatives. Used
/// when a chain is created or reused with **no active scope**, so it isn't left
/// invisible to every scoped read. Endpoints only — the nodes the caller
/// selected — not the pass-through path members. Idempotent junction writes.
fn inherit_endpoint_initiatives(
    store: &Store,
    chain_id: &NodeId,
    from: &NodeId,
    to: &NodeId,
) -> Result<()> {
    for endpoint in [from, to] {
        for init in super::initiatives_of_node(store, endpoint)? {
            super::attach_node_to_initiative_named(store, chain_id, &init)?;
        }
    }
    Ok(())
}

/// Enforces the per-chain hop cap (`KAERU_CHAIN_MAX_HOPS`) for a member count.
fn check_hop_cap(store: &Store, member_count: usize) -> Result<()> {
    let hops = member_count.saturating_sub(1);
    let cap = store.config().chain_max_hops;
    if hops > cap {
        return Err(Error::Invalid(format!(
            "chain spans {hops} hops, over the cap of {cap} (raise KAERU_CHAIN_MAX_HOPS to allow)"
        )));
    }
    Ok(())
}

/// Computes the shortest weighted path `from → to` and saves it as a chain.
/// Returns the [`ChainOutcome`], or `None` when the two are unreachable (no
/// path). `name` defaults to one derived from the endpoints; `summary` is the
/// agent's own note on why this trail matters and becomes the chain node's
/// body (auto-derived when omitted), so `chains` can be triaged by
/// name + summary without reading every trail. Idempotent: an identical chain
/// is reused (`reused: true`) rather than duplicated. The chain node lands in
/// the active initiative; chains are `local` for now.
pub fn create_chain(
    store: &Store,
    from: &NodeId,
    to: &NodeId,
    name: Option<&str>,
    summary: Option<&str>,
) -> Result<Option<ChainOutcome>> {
    let path = shortest_path(store, from, to)?;
    if path.len() < 2 {
        return Ok(None);
    }
    check_hop_cap(store, path.len())?;

    // The path is deterministic, so a repeated call would otherwise freeze an
    // identical trail again — reuse the existing chain instead of duplicating.
    // A repeat with a fresh name/summary is the agent relabelling the trail,
    // so refresh that metadata rather than silently dropping it.
    if let Some(existing) = find_duplicate_chain(store, &path)? {
        if name.is_some() || summary.is_some() {
            let cur = read_node_full(store, &existing)?;
            let new_name = name
                .map(String::from)
                .or_else(|| cur.as_ref().map(|c| c.name.clone()))
                .unwrap_or_default();
            let new_body = summary
                .map(String::from)
                .or_else(|| cur.as_ref().and_then(|c| c.body.clone()));
            let layer = cur
                .as_ref()
                .and_then(|c| Layer::from_str(&c.layer).ok())
                .unwrap_or(Layer::Warm);
            upsert_node(
                store,
                &existing,
                NodeType::Chain,
                Tier::Operational,
                &new_name,
                new_body.as_deref(),
                &[],
                store.current_initiative().as_deref(),
                Visibility::Local,
                layer,
            )?;
        }
        // Uniform with the fresh-create path: an unscoped reuse also backfills
        // the endpoints' initiatives (idempotent), so a chain reached this way
        // is never left invisible to scoped reads either.
        if store.current_initiative().is_none() {
            inherit_endpoint_initiatives(store, &existing, from, to)?;
        }
        return Ok(Some(ChainOutcome {
            id: existing,
            reused: true,
        }));
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
    let body = summary.map(String::from).unwrap_or_else(|| {
        format!(
            "reasoning chain: {from_name} → {to_name} ({} nodes)",
            path.len()
        )
    });

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
    // With no active scope the upsert left the chain node without any
    // membership, and a chain invisible to every scoped read is never
    // intended. Inherit the initiatives of its **endpoints** — the nodes the
    // caller chose — not the pass-through path members it merely traverses.
    if initiative.is_none() {
        inherit_endpoint_initiatives(store, &chain_id, from, to)?;
    }
    replace_members(store, &chain_id, &path)?;

    write_audit(
        store.db_ref(),
        "create_chain",
        "system",
        &[chain_id.clone(), from.clone(), to.clone()],
    )?;
    Ok(Some(ChainOutcome {
        id: chain_id,
        reused: false,
    }))
}

/// Recomputes a chain's shortest weighted path between its current endpoints
/// and replaces its members in place — refreshing a trail that graph changes
/// (new edges, re-weights) may have made stale. The chain keeps its id, name,
/// and summary. Returns `None` if the endpoints are no longer reachable (chain
/// left untouched); errors if `chain` has fewer than two members.
pub fn regenerate_chain(store: &Store, chain: &NodeId) -> Result<Option<RechainStats>> {
    let old = chain_member_ids(store, chain)?;
    if old.len() < 2 {
        return Err(Error::Invalid(format!(
            "{chain:?} is not a chain (needs at least two members)"
        )));
    }
    let from = &old[0];
    let to = &old[old.len() - 1];
    let new_path = shortest_path(store, from, to)?;
    if new_path.len() < 2 {
        return Ok(None);
    }
    check_hop_cap(store, new_path.len())?;
    let changed = new_path != old;
    if changed {
        replace_members(store, chain, &new_path)?;
        write_audit(
            store.db_ref(),
            "regenerate_chain",
            "system",
            &[chain.clone()],
        )?;
    }
    Ok(Some(RechainStats {
        members: new_path.len(),
        changed,
    }))
}

/// Extends a chain by appending the shortest weighted path from its current
/// last member to `to`. The chain keeps its id, name, and summary. Returns
/// `None` if `to` is unreachable from the current end (chain left untouched);
/// errors if `chain` has no members or the result would exceed the hop cap.
pub fn extend_chain(store: &Store, chain: &NodeId, to: &NodeId) -> Result<Option<RechainStats>> {
    let old = chain_member_ids(store, chain)?;
    let Some(last) = old.last() else {
        return Err(Error::Invalid(format!(
            "{chain:?} has no members to extend"
        )));
    };
    if last == to {
        return Ok(Some(RechainStats {
            members: old.len(),
            changed: false,
        }));
    }
    let tail = shortest_path(store, last, to)?;
    if tail.len() < 2 {
        return Ok(None);
    }
    // tail[0] is the current last member; append everything after it.
    let mut members = old.clone();
    members.extend(tail.into_iter().skip(1));
    check_hop_cap(store, members.len())?;
    replace_members(store, chain, &members)?;
    write_audit(
        store.db_ref(),
        "extend_chain",
        "system",
        &[chain.clone(), to.clone()],
    )?;
    Ok(Some(RechainStats {
        members: members.len(),
        changed: true,
    }))
}

#[cfg(test)]
mod tests {
    use super::{create_chain, extend_chain, regenerate_chain};
    use crate::config::KaeruConfig;
    use crate::errors::Error;
    use crate::graph::EdgeType;
    use crate::store::Store;
    use crate::{
        EpisodeKind, Significance, chains_of, link_with_weight, node_brief_by_id, read_chain,
        write_episode,
    };

    /// Three episodes wired into a strong straight line a→b→c.
    fn line_abc(store: &Store) -> (String, String, String) {
        let mk = |n: &str| {
            write_episode(store, EpisodeKind::Observation, Significance::Low, n, n).unwrap()
        };
        let (a, b, c) = (mk("a"), mk("b"), mk("c"));
        link_with_weight(store, &a, &b, EdgeType::RefersTo, 0.9).unwrap();
        link_with_weight(store, &b, &c, EdgeType::RefersTo, 0.9).unwrap();
        (a, b, c)
    }

    /// A chain materialised with no active scope inherits the union of its
    /// members' initiatives instead of landing membership-less (issue #25
    /// family — same guarantee as consolidate/synthesise).
    #[test]
    fn create_chain_without_scope_inherits_member_initiatives() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("p");
        let (a, _b, c) = line_abc(&store);

        store.clear_initiative();
        let outcome = create_chain(&store, &a, &c, None, None)
            .expect("create")
            .expect("path exists");

        let inits = crate::mutate::initiatives_of_node(&store, &outcome.id).expect("junction read");
        assert_eq!(inits, vec!["p".to_string()]);
    }

    /// Endpoint inheritance is *endpoints only* — a pass-through node the
    /// shortest path merely traverses does not drag its own initiative onto the
    /// chain (issue #36 follow-up: over-attach of intermediate nodes).
    #[test]
    fn create_chain_inherits_endpoints_not_passthrough() {
        let store = Store::open_in_memory().expect("open");
        let mk = |init: &str, n: &str| {
            store.use_initiative(init);
            write_episode(&store, EpisodeKind::Observation, Significance::Low, n, n).unwrap()
        };
        // Endpoints under `p`; the pass-through `b` under a different scope `q`.
        let a = mk("p", "a");
        let c = mk("p", "c");
        let b = mk("q", "b");
        link_with_weight(&store, &a, &b, EdgeType::RefersTo, 0.9).unwrap();
        link_with_weight(&store, &b, &c, EdgeType::RefersTo, 0.9).unwrap();

        store.clear_initiative();
        let outcome = create_chain(&store, &a, &c, None, None)
            .expect("create")
            .expect("path exists");

        let inits = crate::mutate::initiatives_of_node(&store, &outcome.id).expect("junction read");
        // `p` from both endpoints; `q` (the pass-through) is deliberately absent.
        assert_eq!(inits, vec!["p".to_string()]);
    }

    /// The dedup/reuse path carries the same guarantee: re-running an unscoped
    /// `create_chain` on an existing trail backfills its endpoints' initiatives,
    /// so a once-membership-less chain doesn't stay invisible to scoped reads
    /// (issue #36 follow-up: inheritance was fresh-create-only).
    #[test]
    fn unscoped_reuse_backfills_endpoint_initiatives() {
        let store = Store::open_in_memory().expect("open");
        // Endpoints created with no active scope → chain starts membership-less.
        let (a, _b, c) = line_abc(&store);
        let first = create_chain(&store, &a, &c, None, None)
            .expect("create")
            .expect("path exists");
        let before = crate::mutate::initiatives_of_node(&store, &first.id).expect("junction read");
        assert!(before.is_empty(), "no scope, no endpoint memberships yet");

        // The endpoints later join `p`; a second unscoped create dedups to the
        // same trail and backfills.
        crate::mutate::attach_node_to_initiative_named(&store, &a, "p").unwrap();
        crate::mutate::attach_node_to_initiative_named(&store, &c, "p").unwrap();
        let reuse = create_chain(&store, &a, &c, None, None)
            .expect("reuse")
            .expect("path exists");
        assert_eq!(
            reuse.id, first.id,
            "same trail dedups to the existing chain"
        );
        assert!(reuse.reused);

        let after = crate::mutate::initiatives_of_node(&store, &reuse.id).expect("junction read");
        assert_eq!(
            after,
            vec!["p".to_string()],
            "reuse backfilled the endpoints"
        );
    }

    #[test]
    fn create_chain_saves_ordered_path_and_membership() {
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
        // Strong straight line a→b→c.
        link_with_weight(&store, &a, &b, EdgeType::RefersTo, 0.9).unwrap();
        link_with_weight(&store, &b, &c, EdgeType::RefersTo, 0.9).unwrap();

        let chain_id = create_chain(&store, &a, &c, Some("auth-trail"), None)
            .unwrap()
            .expect("path exists")
            .id;

        // read_chain returns the ordered members a, b, c.
        let members = read_chain(&store, &chain_id).unwrap();
        let ids: Vec<&String> = members.iter().map(|m| &m.id).collect();
        assert_eq!(ids, vec![&a, &b, &c], "ordered trail; got {ids:?}");

        // chains_of(b) lists this chain.
        let chains = chains_of(&store, &b).unwrap();
        assert!(
            chains.iter().any(|ch| ch.id == chain_id),
            "b knows its chain"
        );
        assert_eq!(chains[0].node_type, "chain");
        assert_eq!(chains[0].name, "auth-trail");

        // Unreachable pair → None.
        let lonely = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "lonely",
            "L",
        )
        .unwrap();
        assert!(
            create_chain(&store, &a, &lonely, None, None)
                .unwrap()
                .is_none()
        );
    }

    /// A path longer than `chain_max_hops` is refused with `Error::Invalid`.
    #[test]
    fn create_chain_refuses_path_over_hop_cap() {
        let mut cfg = KaeruConfig::defaults();
        cfg.chain_max_hops = 1; // allow a single hop only
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

        // a→b is one hop: fine.
        assert!(create_chain(&store, &a, &b, None, None).unwrap().is_some());
        // a→b→c is two hops: over the cap.
        match create_chain(&store, &a, &c, None, None) {
            Err(Error::Invalid(msg)) => assert!(msg.contains("hops"), "got {msg}"),
            other => panic!("expected Invalid, got {other:?}"),
        }
    }

    #[test]
    fn agent_summary_becomes_chain_body() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("p");
        let (a, _b, c) = line_abc(&store);
        let id = create_chain(&store, &a, &c, Some("trail"), Some("why these connect"))
            .unwrap()
            .expect("path")
            .id;
        let brief = node_brief_by_id(&store, &id).unwrap().unwrap();
        assert_eq!(brief.name, "trail");
        assert_eq!(brief.body_excerpt.as_deref(), Some("why these connect"));
    }

    #[test]
    fn identical_chain_is_deduped_not_duplicated() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("p");
        let (a, b, c) = line_abc(&store);
        let first = create_chain(&store, &a, &c, None, None).unwrap().unwrap();
        assert!(!first.reused);
        let second = create_chain(&store, &a, &c, None, None).unwrap().unwrap();
        assert!(second.reused, "deterministic repeat reuses the chain");
        assert_eq!(first.id, second.id);
        // b is on exactly one chain, not two.
        assert_eq!(chains_of(&store, &b).unwrap().len(), 1);
    }

    #[test]
    fn dedup_refreshes_metadata_on_reuse() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("p");
        let (a, _b, c) = line_abc(&store);
        let id = create_chain(&store, &a, &c, Some("trail"), Some("first why"))
            .unwrap()
            .unwrap()
            .id;

        // Repeat the identical path with a new summary → reused, but the
        // agent's fresh summary is applied, and the name (not re-supplied) kept.
        let again = create_chain(&store, &a, &c, None, Some("second why"))
            .unwrap()
            .unwrap();
        assert!(again.reused);
        assert_eq!(again.id, id, "same chain reused, not duplicated");

        let brief = node_brief_by_id(&store, &id).unwrap().unwrap();
        assert_eq!(
            brief.body_excerpt.as_deref(),
            Some("second why"),
            "summary refreshed on reuse"
        );
        assert_eq!(brief.name, "trail", "name preserved when not re-supplied");
    }

    #[test]
    fn regenerate_picks_up_a_new_shorter_path() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("p");
        let (a, b, c) = line_abc(&store);
        let id = create_chain(&store, &a, &c, None, None)
            .unwrap()
            .unwrap()
            .id;
        assert_eq!(read_chain(&store, &id).unwrap().len(), 3, "a→b→c");

        // A strong direct edge makes a→c the new shortest path.
        link_with_weight(&store, &a, &c, EdgeType::RefersTo, 1.0).unwrap();
        let stats = regenerate_chain(&store, &id).unwrap().expect("reachable");
        assert!(stats.changed);
        let members = read_chain(&store, &id).unwrap();
        let ids: Vec<&String> = members.iter().map(|m| &m.id).collect();
        assert_eq!(ids, vec![&a, &c], "regenerated to the direct hop");
        let _ = b;
    }

    #[test]
    fn extend_appends_path_to_a_new_node() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("p");
        let (a, b, c) = line_abc(&store);
        let d = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "d",
            "d",
        )
        .unwrap();
        link_with_weight(&store, &c, &d, EdgeType::RefersTo, 0.9).unwrap();

        let id = create_chain(&store, &a, &b, None, None)
            .unwrap()
            .unwrap()
            .id;
        let stats = extend_chain(&store, &id, &d).unwrap().expect("reachable");
        assert!(stats.changed);
        let members = read_chain(&store, &id).unwrap();
        let ids: Vec<&String> = members.iter().map(|m| &m.id).collect();
        assert_eq!(ids, vec![&a, &b, &c, &d], "extended a→b out to d");
    }
}
