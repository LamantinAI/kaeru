use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::channel;
use std::thread;

use serde_json::json;

use super::ExecutionControl;
use crate::api::{ApiOptions, execute_with_control};
use crate::store::test_support::capture;
use crate::{Layer, NodeType, Store, Tier, Visibility, upsert_node};

fn fixture() -> Store {
    let store = Store::open_in_memory().unwrap();
    for n in 0..3 {
        upsert_node(
            &store,
            &format!("record-{n}"),
            NodeType::Episode,
            Tier::Operational,
            "record",
            Some("body"),
            &[],
            Some("perf"),
            Visibility::Local,
            Layer::Warm,
            None,
        )
        .unwrap();
    }
    store
}

#[test]
fn pre_cancelled_read_does_no_storage_work() {
    let store = fixture();
    let control = ExecutionControl::new(|| true);
    let (response, queries) = capture(|| {
        execute_with_control(
            &store,
            "context",
            json!({"initiative":"perf","view":"map"}),
            &ApiOptions::local(),
            &control,
        )
    });
    assert!(
        response
            .issues
            .iter()
            .any(|issue| issue.code == "CANCELLED")
    );
    assert!(queries.is_empty());
}

#[test]
fn cancellation_during_read_is_rejection_not_a_successful_partial_map() {
    let store = fixture();
    let probes = Arc::new(AtomicUsize::new(0));
    let counter = probes.clone();
    let control = ExecutionControl::new(move || counter.fetch_add(1, Ordering::SeqCst) >= 7);
    let (response, queries) = capture(|| {
        execute_with_control(
            &store,
            "context",
            json!({"initiative":"perf","view":"map"}),
            &ApiOptions::local(),
            &control,
        )
    });
    assert!(
        response
            .issues
            .iter()
            .any(|issue| issue.code == "CANCELLED"),
        "{response:?}"
    );
    assert!(
        !queries.is_empty(),
        "cancellation must happen after reading starts"
    );
    assert!(
        queries.len() < 10,
        "stop between storage batches: {queries:?}"
    );
    assert!(
        !crate::api::execute(
            &store,
            "context",
            json!({"initiative":"perf","view":"map"}),
            &ApiOptions::local()
        )
        .is_error(),
        "cancelled read must release its transaction and control frame"
    );
}

#[test]
fn queued_cancelled_read_skips_storage_after_scope_becomes_available() {
    let store = Arc::new(fixture());
    let cancelled = Arc::new(AtomicBool::new(false));
    let worker = store.clone();
    let signal = cancelled.clone();
    let (started_tx, started_rx) = channel();
    let mut started = Some(started_tx);
    let ready = Arc::new(std::sync::Mutex::new(started.take()));
    let handle = store.scoped(Some("holder"), |_| {
        let handle = thread::spawn(move || {
            let control = ExecutionControl::new(move || {
                if let Some(tx) = ready.lock().unwrap().take() {
                    tx.send(()).unwrap();
                }
                signal.load(Ordering::SeqCst)
            });
            capture(|| {
                execute_with_control(
                    &worker,
                    "context",
                    json!({"initiative":"perf","view":"map"}),
                    &ApiOptions::local(),
                    &control,
                )
            })
        });
        started_rx.recv().unwrap();
        cancelled.store(true, Ordering::SeqCst);
        handle
    });
    let (response, queries) = handle.join().unwrap();
    assert!(
        response
            .issues
            .iter()
            .any(|issue| issue.code == "CANCELLED")
    );
    assert!(queries.is_empty());
}

#[test]
fn started_command_finishes_even_if_caller_cancels() {
    let store = fixture();
    let probes = Arc::new(AtomicUsize::new(0));
    let counter = probes.clone();
    // Entry and post-lock probes admit the command; every later probe would
    // cancel it. Its transaction, supplements and entity refresh must finish.
    let control = ExecutionControl::new(move || counter.fetch_add(1, Ordering::SeqCst) >= 2);
    let response = execute_with_control(
        &store,
        "note",
        json!({"initiative":"perf","body":"committed"}),
        &ApiOptions::local(),
        &control,
    );
    assert!(!response.is_error(), "{response:?}");
    assert_eq!(probes.load(Ordering::SeqCst), 2);
    assert!(control.is_cancelled());
}

#[test]
fn cancelled_name_resolution_is_not_reported_as_not_found() {
    let store = fixture();
    let counter = Arc::new(AtomicUsize::new(0));
    let control = ExecutionControl::new(move || counter.fetch_add(1, Ordering::SeqCst) >= 3);
    let response = execute_with_control(
        &store,
        "read",
        json!({"initiative":"perf","node":"record"}),
        &ApiOptions::local(),
        &control,
    );
    assert!(
        response
            .issues
            .iter()
            .any(|issue| issue.code == "CANCELLED"),
        "{response:?}"
    );
}
