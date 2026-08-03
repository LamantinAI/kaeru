//! Slots — one live node per role, per initiative.
//!
//! A *slot* is a role an initiative fills exactly once: `handoff`,
//! `entrypoint`, `queue`, `prod-state`. Without it, "the current handoff" is
//! a naming convention the agent has to maintain by hand — and it doesn't:
//! graphs in the wild carry three handoff nodes at once, all reading as
//! equally current, because writing a new one has no effect on the old one.
//!
//! [`occupy_slot`] makes that structural. Taking a slot closes the previous
//! holder: a `supersedes` edge records the succession and the predecessor
//! drops to `cold`. Nothing is deleted — `at`, `history` and `surface` still
//! reach it; it simply stops competing for the context window.
//!
//! Concurrency: the read-check-write here is NOT self-synchronised. Callers
//! must run it inside a single `Store::scoped` closure (the MCP adapter's
//! `with_initiative` already does), which serialises it against every other
//! scoped caller. The store guard is not reentrant, so this module must not
//! take it itself.

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};

use crate::errors::Result;
use crate::graph::audit::write_audit;
use crate::graph::{EdgeType, Layer, NodeId};
use crate::store::Store;

/// What taking a slot did: which role was filled, and whom it displaced.
#[derive(Debug, Clone)]
pub struct SlotOutcome {
    /// The role that now points at the new node.
    pub slot: String,
    /// The previous holder, demoted to `cold` and linked by `supersedes`.
    /// `None` when the slot was empty (first write, or after a `forget`).
    pub previous: Option<NodeId>,
}

/// Returns the node currently holding `slot` in `initiative`, if any.
pub fn slot_holder(store: &Store, initiative: &str, slot: &str) -> Result<Option<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    params.insert("slot".to_string(), DataValue::Str(slot.into()));
    let script = r#"
        ?[node_id] := *slot_occupant{initiative, slot, node_id},
            initiative = $init, slot = $slot
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    Ok(rows
        .rows
        .first()
        .and_then(|r| r.first())
        .and_then(|v| v.get_str())
        .map(String::from))
}

/// Every filled slot of `initiative`, as `(slot, node_id)` pairs.
pub fn slots_in(store: &Store, initiative: &str) -> Result<Vec<(String, NodeId)>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    let script = r#"
        ?[slot, node_id] := *slot_occupant{initiative, slot, node_id}, initiative = $init
        :order slot
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    Ok(rows
        .rows
        .iter()
        .filter_map(|r| {
            let slot = r.first()?.get_str()?.to_string();
            let node = r.get(1)?.get_str()?.to_string();
            Some((slot, node))
        })
        .collect())
}

/// Makes `node_id` the live holder of `slot` in `initiative`, closing the
/// previous holder if there was one: `supersedes` edge from old to new (the
/// direction [`crate::supersedes`] uses), then the predecessor drops to
/// `cold`.
///
/// Re-taking a slot with the node that already holds it is a no-op — it must
/// not supersede itself or demote itself to `cold`.
///
/// Deliberately does NOT retract the predecessor: unlike `supersedes`, whose
/// successor carries the same content forward, a slot's members are distinct
/// records (yesterday's handoff is still a true account of yesterday). They
/// stay readable through `at` / `history` / `surface layers=cold`.
pub fn occupy_slot(
    store: &Store,
    initiative: &str,
    slot: &str,
    node_id: &NodeId,
) -> Result<SlotOutcome> {
    let previous = slot_holder(store, initiative, slot)?;

    let displaced = match previous {
        Some(ref prev) if prev != node_id => {
            super::edge::link(store, prev, node_id, EdgeType::Supersedes)?;
            super::layer::set_layer(store, prev, Layer::Cold)?;
            Some(prev.clone())
        }
        _ => None,
    };

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    params.insert("slot".to_string(), DataValue::Str(slot.into()));
    params.insert("nid".to_string(), DataValue::Str(node_id.clone().into()));
    let script = r#"
        ?[initiative, slot, node_id] <- [[$init, $slot, $nid]]
        :put slot_occupant {initiative, slot => node_id}
    "#;
    store
        .db_ref()
        .run_script(script, params, ScriptMutability::Mutable)?;

    write_audit(store.db_ref(), "occupy_slot", "system", &[node_id.clone()])?;

    Ok(SlotOutcome {
        slot: slot.to_string(),
        previous: displaced,
    })
}

/// Frees `slot` without touching whichever node held it. Used when a role
/// stops being meaningful for an initiative; the ex-holder keeps its layer.
pub fn release_slot(store: &Store, initiative: &str, slot: &str) -> Result<Option<NodeId>> {
    let previous = slot_holder(store, initiative, slot)?;
    if previous.is_none() {
        return Ok(None);
    }
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    params.insert("slot".to_string(), DataValue::Str(slot.into()));
    let script = r#"
        ?[initiative, slot] <- [[$init, $slot]]
        :rm slot_occupant {initiative, slot}
    "#;
    store
        .db_ref()
        .run_script(script, params, ScriptMutability::Mutable)?;
    Ok(previous)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Significance;
    use crate::mutate::layer::get_layer;
    use crate::{EpisodeKind, write_episode};

    fn episode(store: &Store, name: &str) -> NodeId {
        write_episode(
            store,
            EpisodeKind::Observation,
            Significance::Medium,
            name,
            "body",
        )
        .expect("write episode")
    }

    /// The core invariant: a second write to the same slot displaces the
    /// first — the graph cannot end up with two live handoffs.
    #[test]
    fn second_write_displaces_the_first() {
        let store = Store::open_in_memory().expect("open");
        let first = episode(&store, "handoff-monday");
        let second = episode(&store, "handoff-tuesday");

        let outcome = occupy_slot(&store, "proj", "handoff", &first).expect("first");
        assert!(outcome.previous.is_none(), "empty slot displaces nobody");

        let outcome = occupy_slot(&store, "proj", "handoff", &second).expect("second");
        assert_eq!(
            outcome.previous.as_ref(),
            Some(&first),
            "second write reports whom it displaced"
        );

        assert_eq!(
            slot_holder(&store, "proj", "handoff").expect("holder"),
            Some(second.clone()),
            "the slot points at the newest node"
        );
        assert_eq!(
            get_layer(&store, &first).expect("layer"),
            Layer::Cold,
            "the predecessor is archived, not deleted"
        );
        assert_eq!(
            get_layer(&store, &second).expect("layer"),
            Layer::Warm,
            "the new holder keeps its own layer"
        );
    }

    /// The predecessor stays readable — archiving is not deletion.
    #[test]
    fn displaced_node_survives_and_is_linked() {
        let store = Store::open_in_memory().expect("open");
        let first = episode(&store, "handoff-one");
        let second = episode(&store, "handoff-two");
        occupy_slot(&store, "proj", "handoff", &first).expect("first");
        occupy_slot(&store, "proj", "handoff", &second).expect("second");

        let still_there = crate::mutate::read_node_now(&store, &first).expect("read");
        assert!(
            still_there.is_some(),
            "displaced node still resolves at NOW"
        );

        let reachable = crate::walk(&store, &first, &[EdgeType::Supersedes], 1).expect("walk");
        assert!(
            reachable.contains(&second),
            "supersedes edge points from the old holder to the new one: {reachable:?}"
        );
    }

    /// Re-taking a slot with its current holder must not supersede or
    /// archive that node — otherwise an idempotent retry would bury it.
    #[test]
    fn retaking_with_the_same_node_is_a_no_op() {
        let store = Store::open_in_memory().expect("open");
        let only = episode(&store, "entrypoint");
        occupy_slot(&store, "proj", "entrypoint", &only).expect("first");
        let outcome = occupy_slot(&store, "proj", "entrypoint", &only).expect("again");

        assert!(outcome.previous.is_none(), "nothing was displaced");
        assert_eq!(
            get_layer(&store, &only).expect("layer"),
            Layer::Warm,
            "the holder was not archived by re-taking its own slot"
        );
    }

    /// Slots are per-initiative: the same role name in another initiative is
    /// a different slot and displaces nothing.
    #[test]
    fn slots_are_scoped_per_initiative() {
        let store = Store::open_in_memory().expect("open");
        let a = episode(&store, "handoff-a");
        let b = episode(&store, "handoff-b");
        occupy_slot(&store, "proj-a", "handoff", &a).expect("a");
        occupy_slot(&store, "proj-b", "handoff", &b).expect("b");

        assert_eq!(
            slot_holder(&store, "proj-a", "handoff").expect("a"),
            Some(a.clone())
        );
        assert_eq!(get_layer(&store, &a).expect("layer"), Layer::Warm);
        assert_eq!(
            slots_in(&store, "proj-a").expect("slots"),
            vec![("handoff".to_string(), a)]
        );
    }

    /// Two threads racing for the same slot must leave exactly one holder,
    /// and the loser must be archived rather than left live alongside — the
    /// failure this whole module exists to make impossible. `occupy_slot` is
    /// not self-synchronised by design; the guarantee comes from running it
    /// inside `Store::scoped`, exactly as the MCP adapter's `with_initiative`
    /// does. This test asserts that contract holds under real contention.
    #[test]
    fn concurrent_writers_leave_exactly_one_holder() {
        use std::sync::{Arc, Barrier};

        let store = Arc::new(Store::open_in_memory().expect("open"));
        let first = episode(&store, "handoff-racer-a");
        let second = episode(&store, "handoff-racer-b");
        let barrier = Arc::new(Barrier::new(2));

        let handles: Vec<_> = [first.clone(), second.clone()]
            .into_iter()
            .map(|node| {
                let store = Arc::clone(&store);
                let barrier = Arc::clone(&barrier);
                std::thread::spawn(move || {
                    barrier.wait();
                    store.scoped(Some("proj"), |s| {
                        occupy_slot(s, "proj", "handoff", &node).expect("occupy")
                    })
                })
            })
            .collect();
        let outcomes: Vec<SlotOutcome> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        let holder = slot_holder(&store, "proj", "handoff")
            .expect("holder")
            .expect("someone holds the slot");
        assert!(
            holder == first || holder == second,
            "the holder is one of the racers"
        );

        // Exactly one writer arrived second and therefore displaced someone.
        let displacements = outcomes.iter().filter(|o| o.previous.is_some()).count();
        assert_eq!(
            displacements, 1,
            "one writer went first (displacing nobody), one went second"
        );

        let loser = if holder == first { &second } else { &first };
        assert_eq!(
            get_layer(&store, loser).expect("layer"),
            Layer::Cold,
            "the racer that lost is archived, not left live beside the winner"
        );
    }

    /// Releasing a slot frees the role without touching the node.
    #[test]
    fn release_frees_the_role_and_leaves_the_node_alone() {
        let store = Store::open_in_memory().expect("open");
        let held = episode(&store, "queue");
        occupy_slot(&store, "proj", "queue", &held).expect("occupy");

        let freed = release_slot(&store, "proj", "queue").expect("release");
        assert_eq!(freed.as_ref(), Some(&held));
        assert_eq!(slot_holder(&store, "proj", "queue").expect("holder"), None);
        assert_eq!(
            get_layer(&store, &held).expect("layer"),
            Layer::Warm,
            "releasing a slot does not archive its holder"
        );
        assert_eq!(
            release_slot(&store, "proj", "queue").expect("second release"),
            None,
            "releasing an empty slot is a no-op"
        );
    }
}
