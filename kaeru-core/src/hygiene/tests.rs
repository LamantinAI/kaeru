use super::*;
use crate::graph::{EdgeType, Significance};
use crate::graph::{Layer, NodeId};
use crate::mutate::now_validity_seconds;
use crate::mutate::{get_layer, link, set_layer};
use crate::{EpisodeKind, attach_node, write_episode};
use cozo::{DataValue, ScriptMutability};
use std::collections::BTreeMap;

#[test]
fn failed_batch_rolls_back_layer_and_does_not_publish_report_changes() {
    let store = Store::open_in_memory().unwrap();
    let id = episode_in(&store, "work", "candidate");
    set_layer(&store, &id, Layer::Core).unwrap();
    store.run("::remove hygiene_demotion").unwrap();
    let mut report = HygieneReport::default();
    let candidate = HygieneCandidate {
        node_id: id.clone(),
        name: "candidate".into(),
        action: HygieneAction::DemoteFromCore,
        from: Layer::Core,
        to: Layer::Hot,
        reason: "fixture".into(),
    };
    assert!(apply_batch(&store, &[candidate], &mut report).is_err());
    assert_eq!(get_layer(&store, &id).unwrap(), Layer::Core);
    assert_eq!(report.applied(), 0);
    assert!(report.lines.is_empty());
}

/// Writes an episode into `init` and returns its id.
fn episode_in(store: &Store, init: &str, name: &str) -> NodeId {
    let id = write_episode(
        store,
        EpisodeKind::Observation,
        Significance::Medium,
        name,
        "body of the note",
    )
    .expect("write");
    attach_node(store, &id, init).expect("attach");
    id
}

/// Backdates a node so the age rules can be tested without sleeping for
/// two weeks. Moves the row to an older validity key: asserting an older
/// version alongside the current one would change nothing, since `@ 'NOW'`
/// resolves the newest assertion.
fn backdate(store: &Store, id: &NodeId, days: u64) {
    let secs = now_validity_seconds() - days * 86_400;
    let mut read: BTreeMap<String, DataValue> = BTreeMap::new();
    read.insert("id".to_string(), DataValue::Str(id.clone().into()));
    let current = store
        .db_ref()
        .run_script(
            r#"
            ?[validity, type, tier, name, body, tags, initiatives, properties, visibility, layer] :=
                *node{id, validity, type, tier, name, body, tags, initiatives, properties, visibility, layer @ 'NOW'},
                id = $id
            "#,
            read,
            ScriptMutability::Immutable,
        )
        .expect("read current row");
    let row = current.rows.first().expect("node exists").clone();

    let mut rm: BTreeMap<String, DataValue> = BTreeMap::new();
    rm.insert("id".to_string(), DataValue::Str(id.clone().into()));
    rm.insert("v".to_string(), row[0].clone());
    store
        .db_ref()
        .run_script(
            "?[id, validity] <- [[$id, $v]] :rm node {id, validity}",
            rm,
            ScriptMutability::Mutable,
        )
        .expect("drop the current key");

    let mut put: BTreeMap<String, DataValue> = BTreeMap::new();
    put.insert("id".to_string(), DataValue::Str(id.clone().into()));
    put.insert(
        "validity".to_string(),
        DataValue::List(vec![
            DataValue::from(i64::try_from(secs).expect("recorded seconds fit i64")),
            DataValue::Bool(true),
        ]),
    );
    for (index, column) in crate::mutate::NODE_VALUE_COLUMNS.iter().enumerate() {
        put.insert((*column).to_string(), row[index + 1].clone());
    }
    let cols = crate::mutate::NODE_VALUE_COLUMNS.join(", ");
    let placeholders = crate::mutate::NODE_VALUE_COLUMNS
        .iter()
        .map(|c| format!("${c}"))
        .collect::<Vec<_>>()
        .join(", ");
    store
        .db_ref()
        .run_script(
            &format!(
                r#"
                ?[id, validity, {cols}] <- [[$id, $validity, {placeholders}]]
                :put node {{id, validity => {cols}}}
                "#
            ),
            put,
            ScriptMutability::Mutable,
        )
        .expect("reinsert at the older validity");
}

/// A configuration that fires on small fixtures.
fn eager_store() -> Store {
    let mut cfg = crate::config::KaeruConfig::defaults();
    cfg.hygiene_writes_trigger = 2;
    cfg.hygiene_core_trigger = 3;
    cfg.hygiene_promote_in_degree = 2;
    Store::open_in_memory_with(cfg).expect("open")
}

#[test]
fn archives_old_unreferenced_journal_entries() {
    let store = eager_store();
    let old = episode_in(&store, "proj", "deployed-something");
    let fresh = episode_in(&store, "proj", "deployed-today");
    backdate(&store, &old, 30);

    let report = run_pass(&store, "proj", || true)
        .expect("pass")
        .expect("pass was due");

    assert_eq!(report.archived, 1, "only the old entry was archived");
    assert_eq!(get_layer(&store, &old).expect("layer"), Layer::Cold);
    assert_eq!(
        get_layer(&store, &fresh).expect("layer"),
        Layer::Warm,
        "today's entry stays where it is"
    );
}

#[test]
fn referenced_journal_entries_are_left_alone() {
    let store = eager_store();
    let referenced = episode_in(&store, "proj", "the-lesson");
    let pointer = episode_in(&store, "proj", "entrypoint");
    backdate(&store, &referenced, 30);
    link(&store, &pointer, &referenced, EdgeType::RefersTo).expect("link");

    let report = run_pass(&store, "proj", || true)
        .expect("pass")
        .expect("due");

    assert_eq!(report.archived, 0, "something references it");
    assert_eq!(get_layer(&store, &referenced).expect("layer"), Layer::Warm);
}

#[test]
fn core_is_demoted_one_step_never_archived() {
    let store = eager_store();
    let stale = episode_in(&store, "proj", "stale-core-note");
    set_layer(&store, &stale, Layer::Core).expect("core");
    backdate(&store, &stale, 60);
    // Two more nodes so the write trigger fires.
    episode_in(&store, "proj", "filler-a");
    episode_in(&store, "proj", "filler-b");

    let report = run_pass(&store, "proj", || true)
        .expect("pass")
        .expect("due");

    assert_eq!(report.demoted, 1);
    assert_eq!(
        get_layer(&store, &stale).expect("layer"),
        Layer::Hot,
        "core drops one step, to hot — never straight to the archive"
    );
}

#[test]
fn heavily_referenced_nodes_are_promoted_but_never_into_core() {
    let store = eager_store();
    let hub = episode_in(&store, "proj", "the-map");
    for i in 0..3 {
        let referrer = episode_in(&store, "proj", &format!("note-{i}"));
        link(&store, &referrer, &hub, EdgeType::RefersTo).expect("link");
    }

    let report = run_pass(&store, "proj", || true)
        .expect("pass")
        .expect("due");

    assert_eq!(report.promoted, 1);
    assert_eq!(
        get_layer(&store, &hub).expect("layer"),
        Layer::Hot,
        "promotion stops at hot; growing core stays a deliberate act"
    );
}

#[test]
fn pinned_nodes_are_never_touched() {
    let store = eager_store();
    let pinned = episode_in(&store, "proj", "pinned-old-note");
    backdate(&store, &pinned, 30);
    crate::session::pin(&store, &pinned, "keep in view").expect("pin");
    episode_in(&store, "proj", "filler");

    let report = run_pass(&store, "proj", || true)
        .expect("pass")
        .expect("due");

    assert_eq!(report.archived, 0);
    assert_eq!(
        get_layer(&store, &pinned).expect("layer"),
        Layer::Warm,
        "a pin outranks every hygiene rule"
    );
}

/// The race the design is built around: a node is collected as a
/// candidate, an agent moves it, and the batch must then leave it alone.
#[test]
fn a_node_touched_after_collection_is_skipped() {
    let store = eager_store();
    let node = episode_in(&store, "proj", "about-to-be-rescued");
    backdate(&store, &node, 30);
    episode_in(&store, "proj", "filler");

    let candidates = collect(&store, "proj").expect("collect");
    assert_eq!(candidates.len(), 1, "the old note is a candidate");

    // The agent decides it matters, between collection and application.
    set_layer(&store, &node, Layer::Core).expect("agent promotes it");

    let mut report = HygieneReport::default();
    apply_batch(&store, &candidates, &mut report).expect("apply");

    assert_eq!(report.archived, 0);
    assert_eq!(report.skipped, 1, "the sweep stood down");
    assert_eq!(
        get_layer(&store, &node).expect("layer"),
        Layer::Core,
        "the agent's decision survived the sweep"
    );
}

#[test]
fn a_second_pass_over_a_clean_graph_does_nothing() {
    let store = eager_store();
    let old = episode_in(&store, "proj", "old-note");
    backdate(&store, &old, 30);
    episode_in(&store, "proj", "filler");

    let first = run_pass(&store, "proj", || true)
        .expect("pass")
        .expect("due");
    assert_eq!(first.archived, 1);

    // Nothing new was written, so the pass is not due again...
    assert!(due(&store, "proj").expect("due").is_none());

    // ...and even when forced, it finds nothing to move.
    let candidates = collect(&store, "proj").expect("collect");
    assert!(
        candidates.is_empty(),
        "an already-swept graph yields no candidates: {candidates:?}"
    );
}

#[test]
fn trigger_fires_on_writes_then_resets() {
    let store = eager_store();
    assert!(
        due(&store, "proj").expect("due").is_none(),
        "an empty initiative is not due"
    );

    episode_in(&store, "proj", "one");
    episode_in(&store, "proj", "two");
    let reason = due(&store, "proj")
        .expect("due")
        .expect("two writes fire it");
    assert!(
        reason.contains("writes"),
        "reason names the trigger: {reason}"
    );

    run_pass(&store, "proj", || true)
        .expect("pass")
        .expect("due");
    assert!(
        due(&store, "proj").expect("due").is_none(),
        "the pass reset the counter"
    );
}

#[test]
fn report_is_delivered_once_then_cleared() {
    let store = eager_store();
    let old = episode_in(&store, "proj", "old-note");
    backdate(&store, &old, 30);
    episode_in(&store, "proj", "filler");
    run_pass(&store, "proj", || true)
        .expect("pass")
        .expect("due");

    let first = take_pending_report(&store, "proj").expect("take");
    let cue = first.as_deref().expect("a cue is waiting");
    // A re-orientation cue, not a work report: it states the consequence
    // and points at the next step. A tally of archived/demoted/promoted is
    // accounting the agent never asked for and can't act on.
    assert!(cue.contains("shifted"), "states the consequence: {cue}");
    assert!(cue.contains("awake proj"), "points at the next step: {cue}");
    assert!(
        !cue.contains("archived") && !cue.contains("demoted"),
        "no bookkeeping in the cue: {cue}"
    );
    assert_eq!(
        take_pending_report(&store, "proj").expect("take again"),
        None,
        "delivered once, then cleared"
    );
}

/// A pass that moves nothing must not clear a cue an earlier pass left —
/// the sweep timer fires every few hours, and would otherwise swallow the
/// notice before the agent ever came back to collect it.
#[test]
fn an_empty_pass_does_not_swallow_an_undelivered_cue() {
    let store = eager_store();
    let old = episode_in(&store, "proj", "old-note");
    backdate(&store, &old, 30);
    episode_in(&store, "proj", "filler");
    run_pass(&store, "proj", || true)
        .expect("pass")
        .expect("due");

    // A second pass over the now-clean graph — forced, since the trigger
    // reset, which is exactly what the sweep timer or `hygiene force=true`
    // does. It finds nothing to move.
    let second = force_pass(&store, "proj", || true)
        .expect("pass")
        .expect("forced");
    assert_eq!(second.applied(), 0, "nothing left to do");

    let cue = take_pending_report(&store, "proj").expect("take");
    // Specifically the FIRST pass's cue (it moved one node) — not the empty
    // pass's own "0 nodes", which would mean it overwrote the notice.
    assert!(
        cue.as_deref().is_some_and(|s| s.contains("1 node ")),
        "the first pass's cue survived the empty one: {cue:?}"
    );
}

/// The promise the batching exists for: a sweep in progress must not stall
/// an agent writing to the same store. The writer thread runs for the
/// whole duration of the pass and must get through several writes — if the
/// pass held the guard end to end, it would land zero until the sweep
/// finished.
#[test]
fn a_pass_in_progress_does_not_stall_concurrent_writers() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier};

    let mut cfg = crate::config::KaeruConfig::defaults();
    cfg.hygiene_writes_trigger = 2;
    cfg.hygiene_batch_size = 5; // 40 candidates → 8 batches
    let store = Arc::new(Store::open_in_memory_with(cfg).expect("open"));

    for i in 0..40 {
        let id = episode_in(&store, "proj", &format!("old-note-{i}"));
        backdate(&store, &id, 30);
    }

    let writes = Arc::new(AtomicUsize::new(0));
    let barrier = Arc::new(Barrier::new(2));
    let sweeper = {
        let store = Arc::clone(&store);
        let barrier = Arc::clone(&barrier);
        std::thread::spawn(move || {
            barrier.wait();
            run_pass(&store, "proj", || {
                std::thread::sleep(std::time::Duration::from_millis(4));
                true
            })
            .expect("pass")
            .expect("due")
        })
    };

    barrier.wait();

    // Write from this thread while the sweep runs in the other one.
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(500);
    while std::time::Instant::now() < deadline {
        store.scoped(Some("proj"), |s| {
            crate::jot(s, "concurrent write").expect("jot");
        });
        writes.fetch_add(1, Ordering::Relaxed);
        if sweeper.is_finished() {
            break;
        }
    }
    let report = sweeper.join().expect("sweeper finished");

    assert_eq!(report.archived, 40, "the sweep did its work");
    assert!(
        writes.load(Ordering::Relaxed) >= 3,
        "the writer got through {} writes while the sweep ran — the guard \
         is released between batches",
        writes.load(Ordering::Relaxed)
    );
}

/// `force` means force. Found by driving the daemon over MCP: the tool
/// advertised "runs one now", but the forced path went through the same
/// due-check and answered "nothing was due".
#[test]
fn a_forced_pass_runs_even_when_nothing_is_due() {
    let store = eager_store();
    let old = episode_in(&store, "proj", "old-note");
    backdate(&store, &old, 30);
    episode_in(&store, "proj", "filler");

    run_pass(&store, "proj", || true)
        .expect("pass")
        .expect("due");
    assert!(due(&store, "proj").expect("due").is_none());

    let another = episode_in(&store, "proj", "another-old-note");
    backdate(&store, &another, 30);
    assert!(
        due(&store, "proj").expect("due").is_none(),
        "one write is below the trigger"
    );

    let report = force_pass(&store, "proj", || true)
        .expect("forced")
        .expect("a forced pass always runs");
    assert_eq!(report.trigger, "forced");
    assert_eq!(report.archived, 1);
    assert_eq!(get_layer(&store, &another).expect("layer"), Layer::Cold);
}

/// Reading the headline is not a pass. An earlier version stamped
/// `last_run_at` while clearing the report, which pushed the "N days
/// since the last pass" trigger forward every time an agent picked one up.
#[test]
fn taking_the_report_does_not_move_the_last_run_stamp() {
    let store = eager_store();
    let old = episode_in(&store, "proj", "old-note");
    backdate(&store, &old, 30);
    episode_in(&store, "proj", "filler");
    run_pass(&store, "proj", || true)
        .expect("pass")
        .expect("due");

    let before = state(&store, "proj").expect("state");
    assert!(before.last_run_at > 0.0);

    take_pending_report(&store, "proj")
        .expect("take")
        .expect("a headline was waiting");

    let after = state(&store, "proj").expect("state");
    assert_eq!(
        after.last_run_at, before.last_run_at,
        "the stamp survived the read"
    );
    assert_eq!(after.nodes_at_last_run, before.nodes_at_last_run);
}

#[test]
fn hygiene_moves_are_attributed_in_the_audit_trail() {
    let store = eager_store();
    let old = episode_in(&store, "proj", "old-note");
    backdate(&store, &old, 30);
    episode_in(&store, "proj", "filler");
    run_pass(&store, "proj", || true)
        .expect("pass")
        .expect("due");

    let rows = store
        .db_ref()
        .run_script(
            r#"?[id, properties] := *node{id, type, properties @ 'NOW'}, type = 'audit_event'"#,
            BTreeMap::new(),
            ScriptMutability::Immutable,
        )
        .expect("audit");
    let has_hygiene_actor = rows
        .rows
        .iter()
        .any(|r| format!("{:?}", r.get(1)).contains(HYGIENE_ACTOR));
    assert!(
        has_hygiene_actor,
        "the sweep's moves are separable from an agent's own layer calls"
    );
}

/// The reported case: a core of recent, well-referenced nodes. Neither
/// half of the age-and-unreferenced rule applies to any of them, so before
/// the overflow rule the pass reported "nothing to do · trigger: core
/// holds N nodes" — naming an oversized core as its reason and then doing
/// nothing about it, forever, on every write.
#[test]
fn an_oversized_core_of_fresh_referenced_nodes_is_brought_down() {
    let store = eager_store(); // ceiling 3
    let mut ids = Vec::new();
    for i in 0..6 {
        let id = episode_in(&store, "proj", &format!("core-note-{i}"));
        set_layer(&store, &id, Layer::Core).expect("core");
        ids.push(id);
    }
    // Every one of them referenced, so the existing rule cannot fire.
    let anchor = episode_in(&store, "proj", "the-thing-they-describe");
    for id in &ids {
        link(&store, &anchor, id, EdgeType::DerivedFrom).expect("link");
    }

    let report = run_pass(&store, "proj", || true)
        .expect("pass")
        .expect("pass was due");

    assert_eq!(report.core_after, 3, "core came down to its ceiling");
    assert_eq!(report.demoted, 3);
    assert!(
        !report.summary().contains("nothing to do"),
        "the pass reports what it did: {}",
        report.summary()
    );
    // And a second pass is no longer due — the trigger it satisfied does
    // not re-fire on every write from here on.
    assert!(
        due(&store, "proj").expect("due").is_none()
            || !due(&store, "proj")
                .expect("due")
                .unwrap()
                .contains("core holds"),
        "the core trigger is satisfied"
    );
}

/// A demoted node lands in `hot`, still in the working set, and one
/// `layer <name> core` away from where it was.
#[test]
fn overflow_demotion_is_one_reversible_step() {
    let store = eager_store();
    let mut ids = Vec::new();
    for i in 0..5 {
        let id = episode_in(&store, "proj", &format!("core-note-{i}"));
        set_layer(&store, &id, Layer::Core).expect("core");
        ids.push(id);
    }

    run_pass(&store, "proj", || true)
        .expect("pass")
        .expect("due");

    let demoted: Vec<_> = ids
        .iter()
        .filter(|id| get_layer(&store, id).expect("layer") == Layer::Hot)
        .collect();
    assert_eq!(demoted.len(), 2, "exactly the overflow, and only to hot");
    for id in &ids {
        let l = get_layer(&store, id).expect("layer");
        assert!(
            l == Layer::Core || l == Layer::Hot,
            "never further than one step: {l:?}"
        );
    }
}

/// A pin outranks the sweep — including the overflow rule. What the pass
/// must not do is stay silent about the core it therefore cannot shrink.
#[test]
fn a_core_held_up_by_pins_is_reported_not_hidden() {
    let store = eager_store();
    for i in 0..5 {
        let id = episode_in(&store, "proj", &format!("pinned-core-{i}"));
        set_layer(&store, &id, Layer::Core).expect("core");
        crate::session::pin(&store, &id, "held open on purpose").expect("pin");
    }

    let report = run_pass(&store, "proj", || true)
        .expect("pass")
        .expect("pass was due");

    assert_eq!(report.demoted, 0, "pins are untouchable");
    assert_eq!(report.core_after, 5);
    let summary = report.summary();
    assert!(
        summary.contains("core still holds 5") && summary.contains("pinned"),
        "the standing condition is named, not hidden behind 'nothing to do': {summary}"
    );
    assert!(
        summary.contains("unpin") || summary.contains("layer"),
        "and the summary says what to do about it: {summary}"
    );
}

/// Least referenced first, oldest first among equals — the pass gives up
/// the least useful core node it can, not an arbitrary one.
#[test]
fn overflow_demotes_the_least_referenced_first() {
    let store = eager_store();
    let keep = episode_in(&store, "proj", "the-load-bearing-one");
    let drop = episode_in(&store, "proj", "the-orphan");
    let mid = episode_in(&store, "proj", "the-middle");
    let spare = episode_in(&store, "proj", "the-other-middle");
    for id in [&keep, &drop, &mid, &spare] {
        set_layer(&store, id, Layer::Core).expect("core");
    }
    let a = episode_in(&store, "proj", "ref-a");
    let b = episode_in(&store, "proj", "ref-b");
    link(&store, &a, &keep, EdgeType::DerivedFrom).expect("link");
    link(&store, &b, &keep, EdgeType::DerivedFrom).expect("link");
    link(&store, &a, &mid, EdgeType::DerivedFrom).expect("link");
    link(&store, &a, &spare, EdgeType::DerivedFrom).expect("link");

    run_pass(&store, "proj", || true)
        .expect("pass")
        .expect("due");

    assert_eq!(
        get_layer(&store, &drop).expect("layer"),
        Layer::Hot,
        "the unreferenced one goes first"
    );
    assert_eq!(
        get_layer(&store, &keep).expect("layer"),
        Layer::Core,
        "the most referenced one stays"
    );
}

// ── support, not contact (#92) ─────────────────────────────────────────

/// The edge that cancels a node used to protect it: an inbound
/// `supersedes` counted as "something still references this", so three
/// superseded `core` nodes were injected into every session for weeks
/// while an identical unreferenced one beside them was demoted at once.
/// Now the supersession is a demotion reason of its own — no waiting for
/// the age rule, because the node is obsolete today.
#[test]
fn a_superseded_core_node_is_demoted_and_says_why() {
    let store = eager_store();
    let stale = episode_in(&store, "proj", "resume-point-a");
    let successor = episode_in(&store, "proj", "state-evening");
    set_layer(&store, &stale, Layer::Core).expect("core");
    link(&store, &successor, &stale, EdgeType::Supersedes).expect("link");

    let candidates = collect(&store, "proj").expect("collect");
    let demotion = candidates
        .iter()
        .find(|c| c.node_id == stale)
        .expect("the superseded node is a candidate");
    assert_eq!(demotion.action, HygieneAction::DemoteFromCore);
    assert_eq!(demotion.to, Layer::Hot);
    assert_eq!(demotion.reason, "superseded by `state-evening`");

    force_pass(&store, "proj", || true).expect("pass");
    assert_eq!(get_layer(&store, &stale).expect("layer"), Layer::Hot);
}

/// `refute` writes a `falsifies` edge; the verdict reads the same way.
#[test]
fn a_refuted_core_node_is_demoted() {
    let store = eager_store();
    let wrong = episode_in(&store, "proj", "the-wrong-fact");
    let evidence = episode_in(&store, "proj", "the-measurement");
    set_layer(&store, &wrong, Layer::Core).expect("core");
    link(&store, &evidence, &wrong, EdgeType::Falsifies).expect("link");

    force_pass(&store, "proj", || true).expect("pass");
    assert_eq!(get_layer(&store, &wrong).expect("layer"), Layer::Hot);
}

/// Support still protects — the rule reads the edge TYPE, not the count.
#[test]
fn a_referenced_core_node_is_still_protected() {
    let store = eager_store();
    let fact = episode_in(&store, "proj", "the-load-bearing-fact");
    let referrer = episode_in(&store, "proj", "a-note-about-it");
    set_layer(&store, &fact, Layer::Core).expect("core");
    backdate(&store, &fact, 60);
    link(&store, &referrer, &fact, EdgeType::RefersTo).expect("link");

    force_pass(&store, "proj", || true).expect("pass");
    assert_eq!(
        get_layer(&store, &fact).expect("layer"),
        Layer::Core,
        "something live points at it, so it stays"
    );
}

/// `flag` writes an inbound `contradicts`. It must not protect the node
/// — that made doubting a `core` fact the way to make it immortal — and
/// it must not demote it either: the question is open, and an open
/// question about a load-bearing fact is a reason to resolve it, not to
/// unload it mid-review.
#[test]
fn a_flag_neither_protects_a_core_node_nor_demotes_it() {
    let store = eager_store();
    let fresh = episode_in(&store, "proj", "the-doubted-fact");
    let stale = episode_in(&store, "proj", "the-old-doubted-fact");
    for id in [&fresh, &stale] {
        set_layer(&store, id, Layer::Core).expect("core");
        let doubt = episode_in(&store, "proj", "why-i-doubt-it");
        link(&store, &doubt, id, EdgeType::Contradicts).expect("link");
    }
    backdate(&store, &stale, 60);

    force_pass(&store, "proj", || true).expect("pass");
    assert_eq!(
        get_layer(&store, &fresh).expect("layer"),
        Layer::Core,
        "a doubt is not a verdict"
    );
    assert_eq!(
        get_layer(&store, &stale).expect("layer"),
        Layer::Hot,
        "but it no longer shields an untouched core node from the age rule"
    );
}

/// The promote rule reads the same number. A node two live nodes
/// contradict was being raised INTO the working set, reason: "2 live
/// nodes reference it".
#[test]
fn contradicted_nodes_are_not_promoted() {
    let store = eager_store();
    let wrong = episode_in(&store, "proj", "the-contested-note");
    for i in 0..2 {
        let objection = episode_in(&store, "proj", &format!("objection-{i}"));
        link(&store, &objection, &wrong, EdgeType::Contradicts).expect("link");
    }

    let report = force_pass(&store, "proj", || true)
        .expect("pass")
        .expect("the pass ran");
    assert_eq!(report.promoted, 0, "objections are not support");
    assert_eq!(get_layer(&store, &wrong).expect("layer"), Layer::Warm);
}

/// And the archive rule: a refuted journal entry nothing else points at
/// is exactly what `cold` is for.
#[test]
fn a_refuted_journal_entry_is_archived() {
    let store = eager_store();
    let old = episode_in(&store, "proj", "what-we-believed-then");
    let evidence = episode_in(&store, "proj", "what-we-measured");
    backdate(&store, &old, 60);
    link(&store, &evidence, &old, EdgeType::Falsifies).expect("link");

    force_pass(&store, "proj", || true).expect("pass");
    assert_eq!(get_layer(&store, &old).expect("layer"), Layer::Cold);
}

/// A pin still outranks every rule, cancellation included.
#[test]
fn a_pinned_superseded_core_node_stays() {
    let store = eager_store();
    let stale = episode_in(&store, "proj", "pinned-resume-point");
    let successor = episode_in(&store, "proj", "the-successor");
    set_layer(&store, &stale, Layer::Core).expect("core");
    link(&store, &successor, &stale, EdgeType::Supersedes).expect("link");
    crate::pin(&store, &stale, "keeping it in view on purpose").expect("pin");

    force_pass(&store, "proj", || true).expect("pass");
    assert_eq!(get_layer(&store, &stale).expect("layer"), Layer::Core);
}

// ── one step, across passes too (#94) ──────────────────────────────────

/// Backdates the pass's own demotion stamp, so the grace period can be
/// tested without waiting a fortnight.
fn backdate_demotion(store: &Store, id: &NodeId, days: u64) {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));
    params.insert(
        "at".to_string(),
        DataValue::from((now_validity_seconds() - days * 86_400) as f64),
    );
    store
        .db_ref()
        .run_script(
            "?[node_id, at] <- [[$id, $at]] :put hygiene_demotion {node_id => at}",
            params,
            ScriptMutability::Mutable,
        )
        .expect("backdate the stamp");
}

/// The promise is "never more than one step". A layer move does not
/// refresh the node's timestamp, so a demoted `core` episode was an old
/// unreferenced `hot` episode the moment it landed — and the next pass
/// archived it. Two passes over an unchanged graph moved it twice.
#[test]
fn a_demoted_core_episode_stays_in_the_working_set() {
    let store = eager_store();
    let old = episode_in(&store, "proj", "old-core-note");
    set_layer(&store, &old, Layer::Core).expect("core");
    backdate(&store, &old, 60);

    force_pass(&store, "proj", || true).expect("pass 1");
    assert_eq!(
        get_layer(&store, &old).expect("layer"),
        Layer::Hot,
        "pass 1 unloads it from core"
    );

    let report = force_pass(&store, "proj", || true)
        .expect("pass 2")
        .expect("the pass ran");
    assert_eq!(report.archived, 0, "pass 2 has nothing to archive");
    assert_eq!(
        get_layer(&store, &old).expect("layer"),
        Layer::Hot,
        "and it is still where a user can see it and put it back"
    );
}

/// The grace is a delay, not an exemption: once the node has sat in the
/// working set for a full journal age without anything pointing at it,
/// it is an ordinary stale journal entry again.
#[test]
fn the_grace_period_expires() {
    let store = eager_store();
    let old = episode_in(&store, "proj", "old-core-note");
    set_layer(&store, &old, Layer::Core).expect("core");
    backdate(&store, &old, 60);

    force_pass(&store, "proj", || true).expect("pass 1");
    backdate_demotion(&store, &old, 30);

    force_pass(&store, "proj", || true).expect("pass 2");
    assert_eq!(get_layer(&store, &old).expect("layer"), Layer::Cold);

    let left = store
        .run_read("?[node_id] := *hygiene_demotion{node_id}")
        .expect("read stamps");
    assert!(
        left.rows.is_empty(),
        "an expired stamp answers nothing and is not kept"
    );
}

/// The grace belongs to nodes the PASS moved. A stale journal entry
/// nobody demoted is archived on the first pass, as it always was.
#[test]
fn a_journal_entry_the_pass_never_moved_is_archived_at_once() {
    let store = eager_store();
    let old = episode_in(&store, "proj", "deployed-something");
    backdate(&store, &old, 30);

    force_pass(&store, "proj", || true).expect("pass");
    assert_eq!(get_layer(&store, &old).expect("layer"), Layer::Cold);
}
