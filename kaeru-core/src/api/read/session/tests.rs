use std::collections::{BTreeMap, BTreeSet};
use std::thread::sleep;
use std::time::Duration;

use chrono::Utc;
use cozo::{DataValue, ScriptMutability};
use serde_json::json;

use super::{current, current_at, memo, prefetch, raw_node, run, visibility};
use crate::api::access::check_node;
use crate::api::read::{node, relations, scope_nodes};
use crate::api::{ApiOptions, execute};
use crate::config::api::{API_READ_BATCH_SIZE, API_READ_CACHE_RECORDS};
use crate::models::ResponseData;
use crate::store::test_support::capture;
use crate::{Layer, NodeType, Store, Tier, Visibility, improve, upsert_node};

fn record(store: &Store, id: &str, scope: &str, visibility: Visibility) {
    upsert_node(
        store,
        &id.into(),
        NodeType::Episode,
        Tier::Operational,
        id,
        Some("original"),
        &[],
        Some(scope),
        visibility,
        Layer::Warm,
        None,
    )
    .unwrap();
}

fn bulk_fixture(count: usize) -> Store {
    let store = Store::open_in_memory().unwrap();
    let nodes = (0..count)
        .map(|n| DataValue::List(vec![DataValue::Str(format!("node-{n:03}").into())]))
        .collect();
    store.db_ref().run_script(
        "input[id] <- $nodes\n?[id,validity,type,tier,name,body,tags,initiatives,properties,visibility,layer] := input[id],validity=[1000.0,true],type='episode',tier='operational',name=id,body='body',tags=null,initiatives=null,properties=null,visibility='local',layer='warm' :put node {id,validity=>type,tier,name,body,tags,initiatives,properties,visibility,layer}",
        BTreeMap::from([("nodes".into(), DataValue::List(nodes))]), ScriptMutability::Mutable,
    ).unwrap();
    store.run("?[initiative,node_id] := *node{id:node_id @ 'NOW'},initiative='perf' :put node_initiative {initiative,node_id}").unwrap();
    store
}

#[test]
fn context_crossing_batch_boundary_uses_bounded_bulk_queries() {
    let store = bulk_fixture(129);
    let (response, queries) = capture(|| {
        execute(
            &store,
            "context",
            json!({"initiative":"perf","view":"map","budget_chars":64000}),
            &ApiOptions::local(),
        )
    });
    assert!(!response.is_error(), "{response:?}");
    let ResponseData::Context(data) = response.data else {
        panic!("context expected")
    };
    assert_eq!(data.sections.iter().flat_map(|s| &s.records).count(), 129);
    assert!(queries.iter().any(|(_, params)| {
        params
            .get("ids")
            .is_some_and(|v| matches!(v, DataValue::List(ids) if ids.len() == 64))
    }));
    assert!(
        queries.len() < 60,
        "{} read queries for 129 records: {queries:?}",
        queries.len()
    );
}

#[test]
fn result_from_old_generation_cannot_be_published_into_new_frame() {
    let store = Store::open_in_memory().unwrap();
    record(&store, "record", "perf", Visibility::Local);
    let options = ApiOptions::local();
    run(&store, &options, || {
        let old_generation = store.generation();
        improve(&store, &"record".into(), "new", "new body").unwrap();
        assert!(
            current_at(&store, None, old_generation, |s| s
                .visible
                .insert("record".into(), Some("stale".into())))
            .is_none()
        );
        let result: Result<String, ()> = memo(
            &store,
            None,
            |s| s.visible.get("record").cloned().flatten(),
            |s, v| {
                s.visible.insert("record".into(), Some(v));
            },
            || {
                improve(&store, &"record".into(), "newer", "newest body").unwrap();
                Ok("stale".into())
            },
        );
        assert_eq!(result.unwrap(), "stale");
        assert!(current(&store, None, |s| s.visible.contains_key("record")) == Some(false));
    });
}

#[test]
fn bulk_body_query_excludes_hidden_records_and_matches_demand_revision() {
    let store = Store::open_in_memory().unwrap();
    record(&store, "visible", "perf", Visibility::Shared);
    record(&store, "hidden", "perf", Visibility::Local);
    let options = ApiOptions {
        include_local: false,
        ..ApiOptions::local()
    };
    let expected = node(&store, "visible", &options, true).unwrap();
    let (actual, queries) = capture(|| {
        run(&store, &options, || {
            prefetch(&store, &["visible".into(), "hidden".into()], &options).unwrap();
            node(&store, "visible", &options, true).unwrap()
        })
    });
    assert_eq!(
        serde_json::to_value(actual).unwrap(),
        serde_json::to_value(expected).unwrap()
    );
    let body_query = queries
        .iter()
        .find(|(sql, _)| sql.contains("?[id,type,tier,name,body"))
        .unwrap();
    assert_eq!(
        body_query.1["ids"],
        DataValue::List(vec![DataValue::List(vec![DataValue::Str(
            "visible".into()
        )])])
    );
}

#[test]
fn generation_change_invalidates_cached_bodies_and_authorization() {
    let store = Store::open_in_memory().unwrap();
    record(&store, "record", "perf", Visibility::Shared);
    let options = ApiOptions::local();
    run(&store, &options, || {
        assert_eq!(
            raw_node(&store, "record").unwrap().unwrap().body.as_deref(),
            Some("original")
        );
        improve(&store, &"record".into(), "updated", "replacement").unwrap();
        assert_eq!(
            raw_node(&store, "record").unwrap().unwrap().body.as_deref(),
            Some("replacement")
        );
        let viewer = ApiOptions {
            include_local: false,
            ..options.clone()
        };
        assert!(check_node(&store, "record", None, &viewer, false).is_ok());
        store.run("?[id,validity,type,tier,name,body,tags,initiatives,properties,visibility,layer] := *node{id,validity,type,tier,name,body,tags,initiatives,properties,layer @ 'NOW'},id='record',visibility='local' :put node {id,validity=>type,tier,name,body,tags,initiatives,properties,visibility,layer}").unwrap();
        assert!(check_node(&store, "record", None, &viewer, false).is_err());
    });
}

#[test]
fn relation_cache_does_not_cross_visibility_or_scope_policies() {
    let store = Store::open_in_memory().unwrap();
    record(&store, "public", "perf", Visibility::Shared);
    record(&store, "private", "private-scope", Visibility::Local);
    store.run("?[src,dst,edge_type,validity,weight,properties,dst_store] <- [['public','private','refers_to',[1000.0,true],1.0,null,'local']] :put edge {src,dst,edge_type,validity=>weight,properties,dst_store}").unwrap();
    let local = ApiOptions::local();
    let viewer = ApiOptions {
        include_local: false,
        ..local.clone()
    };
    let scoped = ApiOptions {
        allowed_initiatives: Some(vec!["perf".into()]),
        ..local.clone()
    };
    run(&store, &local, || {
        assert_eq!(relations(&store, "public", &local).unwrap().len(), 1);
        assert!(relations(&store, "public", &viewer).unwrap().is_empty());
        assert!(relations(&store, "public", &scoped).unwrap().is_empty());
        run(&store, &viewer, || {
            assert!(relations(&store, "public", &viewer).unwrap().is_empty())
        });
        assert_eq!(relations(&store, "public", &local).unwrap().len(), 1);
    });
}

#[test]
fn cached_payload_and_checks_are_confined_to_one_read() {
    let store = Store::open_in_memory().unwrap();
    record(&store, "record", "perf", Visibility::Local);
    let options = ApiOptions::local();
    let (_, first) = capture(|| {
        run(&store, &options, || {
            node(&store, "record", &options, true).unwrap()
        })
    });
    let (_, second) = capture(|| {
        run(&store, &options, || {
            node(&store, "record", &options, true).unwrap()
        })
    });
    assert!(!first.is_empty());
    assert_eq!(
        first.len(),
        second.len(),
        "the next read must rebuild its own cache"
    );
}

#[test]
fn read_snapshot_keeps_access_and_payload_on_one_side_of_future_transition() {
    let store = Store::open_in_memory().unwrap();
    record(&store, "record", "perf", Visibility::Shared);
    let transition = Utc::now().timestamp_micros() + 200_000;
    store.run(&format!("?[id,validity,type,tier,name,body,tags,initiatives,properties,visibility,layer] <- [['record',[{transition}.0,true],'episode','operational','private version','private body',null,null,null,'local','warm']] :put node {{id,validity=>type,tier,name,body,tags,initiatives,properties,visibility,layer}}")).unwrap();
    let options = ApiOptions {
        include_local: false,
        ..ApiOptions::local()
    };
    let generation = store.generation();
    store
        .db_ref()
        .read_snapshot(|| {
            run(&store, &options, || {
                assert_eq!(
                    visibility(&store, "record", false).unwrap().as_deref(),
                    Some("shared")
                );
                while Utc::now().timestamp_micros() <= transition {
                    sleep(Duration::from_millis(5));
                }
                // No cached body exists yet: even a new scalar or bulk query must
                // use the same temporal instant as the cached access decision.
                assert_eq!(
                    raw_node(&store, "record").unwrap().unwrap().body.as_deref(),
                    Some("original")
                );
                prefetch(&store, &["record".into()], &options).unwrap();
                assert_eq!(
                    node(&store, "record", &options, true).unwrap().visibility,
                    "shared"
                );
            });
            Ok(())
        })
        .unwrap();
    assert_eq!(generation, store.generation());
    let response = execute(
        &store,
        "read",
        json!({"initiative":"perf","node":{"id":"record"}}),
        &options,
    );
    assert!(
        response.is_error(),
        "new reads must use the new private visibility: {response:?}"
    );
}

#[test]
fn read_snapshot_blocks_writes_and_recovers_after_error_or_panic() {
    let store = Store::open_in_memory().unwrap();
    record(&store, "record", "perf", Visibility::Local);
    let generation = store.generation();
    store
        .db_ref()
        .read_snapshot(|| {
            assert!(store.run("?[x] <- [[1]] :put forbidden_test {x}").is_err());
            assert!(store.db_ref().atomic(|| Ok(())).is_err());
            // A handled query error must not poison a successful read fallback.
            assert!(store.run_read("?[x] := *missing_relation{x}").is_err());
            assert!(
                !store
                    .run_read("?[id] := *node{id @ 'NOW'}")
                    .unwrap()
                    .rows
                    .is_empty()
            );
            Ok(())
        })
        .unwrap();
    assert_eq!(generation, store.generation());
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _: crate::Result<()> = store
            .db_ref()
            .read_snapshot(|| panic!("test read assembly unwind"));
    }));
    assert!(panic.is_err());
    improve(&store, &"record".into(), "after panic", "updated").unwrap();
    assert_eq!(
        raw_node(&store, "record").unwrap().unwrap().body.as_deref(),
        Some("updated")
    );
}

#[test]
fn nested_store_and_panicking_read_restore_the_outer_frame() {
    let outer = Store::open_in_memory().unwrap();
    let inner = Store::open_in_memory().unwrap();
    record(&outer, "same-id", "outer", Visibility::Local);
    record(&inner, "same-id", "inner", Visibility::Local);
    improve(&inner, &"same-id".into(), "inner", "inner body").unwrap();
    let options = ApiOptions::local();
    run(&outer, &options, || {
        assert_eq!(
            raw_node(&outer, "same-id")
                .unwrap()
                .unwrap()
                .body
                .as_deref(),
            Some("original")
        );
        let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            run(&inner, &options, || {
                assert_eq!(
                    raw_node(&inner, "same-id")
                        .unwrap()
                        .unwrap()
                        .body
                        .as_deref(),
                    Some("inner body")
                );
                panic!("nested frame unwind");
            });
        }));
        assert!(panic.is_err());
        assert_eq!(
            raw_node(&outer, "same-id")
                .unwrap()
                .unwrap()
                .body
                .as_deref(),
            Some("original")
        );
    });
}

#[test]
fn metadata_eviction_keeps_all_records_and_bounds_heavy_caches() {
    let count = API_READ_CACHE_RECORDS + API_READ_BATCH_SIZE;
    let store = bulk_fixture(count);
    let options = ApiOptions::local();
    let nodes = store
        .db_ref()
        .read_snapshot(|| {
            run(&store, &options, || {
                let records = scope_nodes(&store, "perf", &options).unwrap();
                current(&store, None, |frame| {
                    assert!(frame.visible.len() <= API_READ_CACHE_RECORDS);
                    assert!(frame.raw.len() <= API_READ_BATCH_SIZE * 2);
                    assert!(frame.relations.len() <= API_READ_BATCH_SIZE * 2);
                    assert!(frame.edges.len() <= API_READ_BATCH_SIZE * 2);
                });
                Ok(records)
            })
        })
        .unwrap();
    assert_eq!(nodes.len(), count);
    assert_eq!(
        nodes.iter().map(|n| &n.id).collect::<BTreeSet<_>>().len(),
        count
    );
    for id in ["node-000", "node-2048", "node-4159"] {
        let expected = node(&store, id, &options, false).unwrap();
        let actual = nodes.iter().find(|n| n.id == id).unwrap();
        assert_eq!(
            serde_json::to_value(actual).unwrap(),
            serde_json::to_value(expected).unwrap()
        );
    }
}
