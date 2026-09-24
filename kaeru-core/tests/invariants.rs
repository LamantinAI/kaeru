//! The substrate's invariants, checked over random operation sequences (#97).
//!
//! Every invariant here had a hand-picked example test before this file, and
//! hand-picked examples are exactly what missed #93 and #94: both were found
//! by a human reading the code, not by the suite. A property test writes the
//! examples nobody thought of — the second pass over the same graph, the
//! revision that lands in the same second as the write it revises.
//!
//! Deliberately small: each case opens an in-memory vault and runs a short
//! sequence, so the whole file stays a few seconds rather than becoming
//! something people skip.

use kaeru_core::hygiene::force_pass;
use kaeru_core::{
    EdgeType, Layer, Store, at, history, improve, jot, link, occupy_slot, recollect_provenance,
    set_layer, slots_in,
};
use proptest::prelude::*;

/// A vault with one initiative, which is the shape every verb expects.
fn vault() -> Store {
    let store = Store::open_in_memory().expect("open");
    store.use_initiative("proj");
    store
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(24))]

    /// **The past of a versioned column never changes.** Whatever is written
    /// later, a read as of an earlier moment returns what was true then.
    ///
    /// This is the promise `at` is for, and the one #95 found holes in —
    /// there the hole was which columns are versioned at all. Here the
    /// question is whether the versioned ones stay put under a burst of
    /// revisions, including the ones that land inside a single second (#96).
    #[test]
    fn the_past_of_a_versioned_column_stays_put(bodies in prop::collection::vec("[a-z ]{4,40}", 2..6)) {
        let store = vault();
        let id = jot(&store, &bodies[0]).expect("jot");

        // Revise repeatedly. Each write lands past the key's newest row, so
        // the versions carry distinct timestamps even inside one second.
        let mut expected: Vec<(f64, String)> = Vec::new();
        for (i, body) in bodies.iter().enumerate() {
            improve(&store, &id, &format!("name-{i}"), body).expect("revise");
            let newest = history(&store, &id)
                .expect("history")
                .into_iter()
                .filter(|r| r.asserted)
                .map(|r| r.seconds)
                .fold(f64::MIN, f64::max);
            expected.push((newest, body.clone()));
        }

        // Every recorded moment still reads as what was written then, after
        // every later revision has landed.
        for (seconds, body) in &expected {
            let snap = at(&store, &id, *seconds)
                .expect("at")
                .expect("a version is valid at a moment it was written");
            prop_assert_eq!(snap.body.as_deref(), Some(body.as_str()));
        }
    }

    /// **A slot holds at most one node.** That is the whole reason slots
    /// exist: a project cannot end up with three current handoffs.
    #[test]
    fn a_slot_holds_exactly_one_node(count in 1usize..6) {
        let store = vault();
        let mut last = String::new();
        for i in 0..count {
            let id = jot(&store, &format!("handoff {i}")).expect("jot");
            occupy_slot(&store, "proj", "handoff", &id).expect("occupy");
            last = id;
        }

        let filled = slots_in(&store, "proj").expect("slots");
        let holders: Vec<&(String, String)> =
            filled.iter().filter(|(slot, _)| slot == "handoff").collect();
        prop_assert_eq!(holders.len(), 1, "one role, one holder: {:?}", filled);
        prop_assert_eq!(&holders[0].1, &last, "and it is the one taken last");
    }

    /// **Hygiene settles.** A pass over a graph nothing else has touched must
    /// reach a fixed point rather than moving the same node again and again —
    /// which is exactly what #94 broke: "one step" held inside a pass and not
    /// across two.
    #[test]
    fn hygiene_reaches_a_fixed_point(
        layers in prop::collection::vec(prop_oneof![
            Just(Layer::Core),
            Just(Layer::Hot),
            Just(Layer::Warm),
        ], 1..6)
    ) {
        let store = vault();
        for (i, layer) in layers.iter().enumerate() {
            let id = jot(&store, &format!("node {i}")).expect("jot");
            set_layer(&store, &id, *layer).expect("layer");
        }

        // Two passes to settle, as the module promises, then a third that
        // must find nothing left to do.
        force_pass(&store, "proj", || true).expect("pass 1");
        force_pass(&store, "proj", || true).expect("pass 2");
        let third = force_pass(&store, "proj", || true)
            .expect("pass 3")
            .expect("forced passes always run");
        prop_assert_eq!(
            third.applied(),
            0,
            "a third pass over an untouched graph moves nothing: {:?}",
            third.lines
        );
    }

    /// **Provenance terminates.** `link` does not refuse a cycle — nothing in
    /// the write path checks acyclicity, which is worth knowing — so the
    /// invariant that actually matters is that a read cannot be trapped by
    /// one. `recollect_provenance` is bounded by `provenance_max_hops`.
    #[test]
    fn provenance_terminates_even_through_a_cycle(ring in 2usize..5) {
        let store = vault();
        let ids: Vec<String> = (0..ring)
            .map(|i| jot(&store, &format!("step {i}")).expect("jot"))
            .collect();
        for i in 0..ring {
            // …and the last one closes the ring.
            link(&store, &ids[i], &ids[(i + 1) % ring], EdgeType::DerivedFrom).expect("link");
        }

        let walked = recollect_provenance(&store, &ids[0]).expect("provenance");
        prop_assert!(
            walked.len() <= store.config().provenance_max_hops as usize * ring + ring,
            "the walk is bounded rather than looping: {} nodes",
            walked.len()
        );
    }
}
