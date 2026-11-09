//! Active continuations survive saturation for both payload representations.
use kaeru_core::api::{self, ApiOptions, Response, ResponseData};
use kaeru_core::{Layer, NodeType, Store, Tier, Visibility, upsert_node};
use serde_json::{Value, json};
use std::sync::{Arc, mpsc::channel};
use std::time::Duration;

fn fixture() -> (Arc<Store>, String) {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let body = "known body λ ".repeat(1800);
    upsert_node(
        &store,
        &"long-card".into(),
        NodeType::Episode,
        Tier::Operational,
        "long-card",
        Some(&body),
        &[],
        Some("work"),
        Visibility::Shared,
        Layer::Warm,
        None,
    )
    .unwrap();
    for n in 0..24 {
        let id = format!("record-{n:03}");
        upsert_node(
            &store,
            &id,
            NodeType::Episode,
            Tier::Operational,
            &id,
            Some("unused"),
            &[],
            Some("work"),
            Visibility::Shared,
            Layer::Warm,
            None,
        )
        .unwrap();
    }
    (store, body)
}
fn call(store: &Store, operation: &str, args: Value, caller: usize) -> Response {
    api::execute(
        store,
        operation,
        args,
        &ApiOptions {
            caller: format!("reader-{caller}"),
            ..ApiOptions::local()
        },
    )
}
fn run_saturation(graph: bool) {
    let (store, body) = fixture();
    let operation = if graph { "context" } else { "read" };
    let args = if graph {
        json!({"initiative":"work","view":"graph","budget_chars":2000})
    } else {
        json!({"initiative":"work","node":"long-card","budget_chars":6000})
    };
    let (ready, started) = channel();
    std::thread::scope(|threads| {
        let mut release = Vec::new();
        for reader in 0..16 {
            let args = args.clone();
            let ready = ready.clone();
            let (resume, wait) = channel();
            release.push(resume);
            let store = &store;
            let body = &body;
            threads.spawn(move || {
                let mut page = call(store, operation, args, reader);
                assert!(!page.is_error(), "{page:?}");
                assert!(page.continuation.is_some());
                ready.send(()).unwrap();
                wait.recv_timeout(Duration::from_secs(5)).unwrap();
                let mut text = String::new();
                let mut ids = std::collections::BTreeSet::new();
                let mut last = None;
                loop {
                    match &page.data {
                        ResponseData::Record(data) => {
                            text.push_str(data.node.body.as_deref().unwrap())
                        }
                        ResponseData::Graph(data) => {
                            for node in &data.graph.nodes {
                                assert!(ids.insert(node.id.clone()), "duplicate node");
                            }
                        }
                        _ => panic!("unexpected page"),
                    }
                    if let Some(next) = page.continuation {
                        last = Some(next.next_arguments.clone());
                        page = call(store, operation, next.next_arguments, reader);
                        assert!(!page.is_error(), "{page:?}");
                    } else {
                        break;
                    }
                }
                if graph {
                    assert_eq!(ids.len(), 25);
                } else {
                    assert_eq!(&text, body);
                }
                let replay = call(store, operation, last.unwrap(), reader);
                assert!(replay.is_error(), "completed cursor must be released");
            });
        }
        for _ in 0..16 {
            started.recv_timeout(Duration::from_secs(5)).unwrap();
        }
        let overflow = call(&store, operation, args, 16);
        assert!(overflow.is_error(), "{overflow:?}");
        assert_eq!(overflow.issues[0].code, "SNAPSHOT_CAPACITY");
        for resume in release {
            resume.send(()).unwrap();
        }
    });
    let next = if graph {
        json!({"initiative":"work","view":"graph","budget_chars":2000})
    } else {
        json!({"initiative":"work","node":"long-card","budget_chars":6000})
    };
    let response = call(&store, operation, next, 17);
    assert!(
        !response.is_error(),
        "completed entries must free slots: {response:?}"
    );
}
#[test]
fn sixteen_ordinary_snapshots_complete_without_eviction() {
    run_saturation(false);
}
#[test]
fn sixteen_graph_snapshots_complete_without_eviction() {
    run_saturation(true);
}
