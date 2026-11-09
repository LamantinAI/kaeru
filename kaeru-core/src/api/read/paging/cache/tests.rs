use super::*;
use crate::api::{self, ResponseData};
use crate::models::InitiativesData;
use serde_json::json;
use std::sync::{Arc, Barrier};

fn fixture() -> (Store, Arc<Response>, Args, ApiOptions) {
    let store = Store::open_in_memory().unwrap();
    let response = Arc::new(Response::read(
        "initiatives",
        ResponseData::Initiatives(InitiativesData {
            initiatives: vec!["work".into()],
        }),
        vec!["work".into()],
    ));
    (store, response, Args(json!({})), ApiOptions::local())
}
fn begin<'a>(
    store: &'a Store,
    full: Arc<Response>,
    args: &Args,
    options: &ApiOptions,
    token: &str,
) -> Lease<'a> {
    reserve(
        store,
        token.into(),
        full,
        args,
        options,
        store.generation(),
        4096,
    )
    .unwrap()
}

#[test]
fn full_cache_keeps_active_entries_updates_in_place_and_releases_completed_slots() {
    let (store, full, args, options) = fixture();
    for n in 0..API_MAX_CACHED_SNAPSHOTS {
        begin(&store, full.clone(), &args, &options, &n.to_string())
            .publish(full.clone(), 1, store.generation())
            .unwrap();
    }
    assert_eq!(
        reserve(
            &store,
            "overflow".into(),
            full.clone(),
            &args,
            &options,
            store.generation(),
            4096
        )
        .err()
        .unwrap()
        .code,
        "SNAPSHOT_CAPACITY"
    );
    let before = store.api_snapshots.lock().unwrap().entries["0"]
        .snapshot
        .expires;
    checkout(&store, "initiatives", "0".into(), 1, &args, &options)
        .unwrap()
        .publish(full.clone(), 2, store.generation())
        .unwrap();
    assert_eq!(
        store.api_snapshots.lock().unwrap().entries["0"]
            .snapshot
            .expires,
        before,
        "advancing must not extend TTL"
    );
    for n in 0..API_MAX_CACHED_SNAPSHOTS {
        checkout(
            &store,
            "initiatives",
            n.to_string(),
            if n == 0 { 2 } else { 1 },
            &args,
            &options,
        )
        .unwrap()
        .finish()
        .unwrap();
    }
    assert!(store.api_snapshots.lock().unwrap().entries.is_empty());
    assert_eq!(store.api_snapshots.lock().unwrap().bytes, 0);
    begin(&store, full, &args, &options, "replacement");
}

#[test]
fn identity_probes_and_replayed_offsets_do_not_consume_other_callers_cursors() {
    let (store, full, args, options) = fixture();
    begin(&store, full.clone(), &args, &options, "active")
        .publish(full.clone(), 1, store.generation())
        .unwrap();
    let other = ApiOptions {
        caller: "other".into(),
        ..options.clone()
    };
    let changed_policy = ApiOptions {
        include_local: !options.include_local,
        ..options.clone()
    };
    for (name, offset, args, options) in [
        ("initiatives", 1, Args(args.0.clone()), other),
        ("initiatives", 1, Args(args.0.clone()), changed_policy),
        ("history", 1, Args(args.0.clone()), options.clone()),
        (
            "initiatives",
            1,
            Args(json!({"budget_chars":8000})),
            options.clone(),
        ),
        ("initiatives", 0, Args(args.0.clone()), options.clone()),
        ("initiatives", 2, Args(args.0.clone()), options.clone()),
    ] {
        assert_eq!(
            checkout(&store, name, "active".into(), offset, &args, &options)
                .err()
                .unwrap()
                .code,
            "CURSOR_STALE"
        );
    }
    checkout(&store, "initiatives", "active".into(), 1, &args, &options)
        .unwrap()
        .publish(full, 2, store.generation())
        .unwrap();
    assert!(checkout(&store, "initiatives", "active".into(), 1, &args, &options).is_err());
    checkout(&store, "initiatives", "active".into(), 2, &args, &options)
        .unwrap()
        .finish()
        .unwrap();
    assert!(checkout(&store, "initiatives", "active".into(), 2, &args, &options).is_err());
}

#[test]
fn concurrent_duplicate_cannot_advance_or_resurrect_a_finished_cursor() {
    let (store, full, args, options) = fixture();
    begin(&store, full.clone(), &args, &options, "active")
        .publish(full, 1, store.generation())
        .unwrap();
    let ready = Barrier::new(2);
    let release = Barrier::new(2);
    std::thread::scope(|threads| {
        threads.spawn(|| {
            let lease =
                checkout(&store, "initiatives", "active".into(), 1, &args, &options).unwrap();
            ready.wait();
            release.wait();
            lease.finish().unwrap();
        });
        ready.wait();
        assert_eq!(
            checkout(&store, "initiatives", "active".into(), 1, &args, &options)
                .err()
                .unwrap()
                .code,
            "CURSOR_BUSY"
        );
        release.wait();
    });
    assert!(checkout(&store, "initiatives", "active".into(), 1, &args, &options).is_err());
    assert_eq!(store.api_snapshots.lock().unwrap().bytes, 0);
}

#[test]
fn expiry_generation_and_failed_reservations_are_bounded() {
    let (store, full, args, options) = fixture();
    drop(begin(&store, full.clone(), &args, &options, "abandoned"));
    assert!(store.api_snapshots.lock().unwrap().entries.is_empty());
    begin(&store, full.clone(), &args, &options, "expired")
        .publish(full.clone(), 1, store.generation())
        .unwrap();
    let expires = store.api_snapshots.lock().unwrap().entries["expired"]
        .snapshot
        .expires;
    store
        .api_snapshots
        .lock()
        .unwrap()
        .purge(expires, store.generation());
    assert_eq!(store.api_snapshots.lock().unwrap().bytes, 0);
    begin(&store, full.clone(), &args, &options, "changed")
        .publish(full.clone(), 1, store.generation())
        .unwrap();
    store.run(":create changed {id: Int}").unwrap();
    assert!(checkout(&store, "initiatives", "changed".into(), 1, &args, &options).is_err());
    assert_eq!(store.api_snapshots.lock().unwrap().bytes, 0);
    let lease = reserve(
        &store,
        "bytes".into(),
        full.clone(),
        &args,
        &options,
        store.generation(),
        API_MAX_CACHED_SNAPSHOT_BYTES,
    )
    .unwrap();
    assert_eq!(
        reserve(
            &store,
            "excess".into(),
            full,
            &args,
            &options,
            store.generation(),
            1
        )
        .err()
        .unwrap()
        .code,
        "SNAPSHOT_CAPACITY"
    );
    drop(lease);
    assert_eq!(store.api_snapshots.lock().unwrap().bytes, 0);
}

#[test]
fn aborted_writer_cannot_publish_or_consume_a_continuation() {
    let (store, full, args, options) = fixture();
    store.run(":create uncommitted {id: Int}").unwrap();
    begin(&store, full.clone(), &args, &options, "active")
        .publish(full.clone(), 1, store.generation())
        .unwrap();
    let result: crate::Result<()> = store.db_ref().atomic(|| {
        store.run("?[id] <- [[1]] :put uncommitted {id}")?;
        assert_eq!(
            reserve(
                &store,
                "new".into(),
                full,
                &args,
                &options,
                store.generation(),
                4096
            )
            .err()
            .unwrap()
            .code,
            "INVALID_OPERATION"
        );
        assert_eq!(
            checkout(&store, "initiatives", "active".into(), 1, &args, &options)
                .err()
                .unwrap()
                .code,
            "INVALID_OPERATION"
        );
        Err(crate::Error::Invalid("abort".into()))
    });
    assert!(result.is_err());
    checkout(&store, "initiatives", "active".into(), 1, &args, &options)
        .unwrap()
        .finish()
        .unwrap();
}

#[test]
fn assembled_snapshot_cannot_be_relabeled_after_a_writer_commits() {
    let (store, full, _, options) = fixture();
    let generation = store.generation();
    store.run(":create new_generation {id: Int}").unwrap();
    assert_eq!(
        reserve(
            &store,
            "old".into(),
            full,
            &Args(json!({})),
            &options,
            generation,
            4096
        )
        .err()
        .unwrap()
        .code,
        "CURSOR_STALE"
    );
    assert!(store.api_snapshots.lock().unwrap().entries.is_empty());
    for cursor in [
        "",
        "invalid",
        "missing:1",
        "missing:999999999999999999999999999999",
    ] {
        assert!(api::execute(&store, "initiatives", json!({"cursor":cursor}), &options).is_error());
    }
}

#[test]
fn checkout_failure_cancellation_and_unwind_preserve_the_expected_offset() {
    let (store, full, args, options) = fixture();
    begin(&store, full.clone(), &args, &options, "active")
        .publish(full.clone(), 1, store.generation())
        .unwrap();
    {
        let _lease = checkout(&store, "initiatives", "active".into(), 1, &args, &options).unwrap();
        let _control = crate::api::control::enter(crate::api::ExecutionControl::new(|| true));
        assert_eq!(
            crate::api::control::checkpoint().unwrap_err().code,
            "CANCELLED"
        );
    }
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _lease = checkout(&store, "initiatives", "active".into(), 1, &args, &options).unwrap();
        panic!("abandoned request");
    }));
    assert!(panic.is_err());
    let lease = checkout(&store, "initiatives", "active".into(), 1, &args, &options).unwrap();
    assert!(lease.publish(full, 2, store.generation() + 1).is_err());
    checkout(&store, "initiatives", "active".into(), 1, &args, &options)
        .unwrap()
        .finish()
        .unwrap();
}
