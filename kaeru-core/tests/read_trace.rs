use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use kaeru_core::{
    EdgeType, KaeruConfig, Layer, NodeType, Store, Tier, Visibility,
    api::{self, ApiOptions, Response, ResponseData},
    link,
    models::ReadTraceEvent,
    new_node_id, upsert_node,
};
use serde_json::{Value, json};

struct Fixture {
    store: Store,
    directory: PathBuf,
}
impl Fixture {
    fn new(enabled: bool, broken: bool) -> Self {
        let directory = std::env::temp_dir().join(format!("kaeru-api-trace-{}", new_node_id()));
        fs::create_dir_all(&directory).unwrap();
        let config = KaeruConfig {
            read_trace_enabled: enabled,
            read_trace_dir: directory.join("journal"),
            vault_path: directory.join("vault"),
            ..KaeruConfig::defaults()
        };
        if broken {
            fs::write(&config.read_trace_dir, b"not a directory").unwrap();
        }
        Self {
            store: Store::open_in_memory_with(config).unwrap(),
            directory,
        }
    }
    fn call(&self, name: &str, args: Value) -> Response {
        let result = api::execute(&self.store, name, args, &ApiOptions::local());
        assert!(!result.is_error(), "{result:?}");
        result
    }
    fn note(&self, name: &str) -> String {
        let result = self.call(
            "note",
            json!({"initiative":"work","name":name,"body":"group content"}),
        );
        let ResponseData::Mutation(data) = result.data else {
            panic!("mutation expected")
        };
        data.primary.unwrap().id
    }
    fn events(&self) -> Vec<ReadTraceEvent> {
        assert!(self.store.shutdown_read_trace(Duration::from_secs(3)));
        let mut events = Vec::new();
        for entry in fs::read_dir(self.directory.join("journal")).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "jsonl") {
                for line in fs::read_to_string(path).unwrap().lines() {
                    events.push(serde_json::from_str(line).unwrap());
                }
            }
        }
        events
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = self.store.shutdown_read_trace(Duration::from_secs(2));
        let _ = fs::remove_dir_all(&self.directory);
    }
}

#[test]
fn trace_records_materialized_content_not_names_bodies_or_incidental_references() {
    let f = Fixture::new(true, false);
    let id = new_node_id();
    let phantom = new_node_id();
    f.store.use_initiative("work");
    upsert_node(
        &f.store,
        &id,
        NodeType::Episode,
        Tier::Operational,
        "never-log-this-name",
        Some("never log this body"),
        &[],
        Some("work"),
        Visibility::Local,
        Layer::Warm,
        Some(&json!({"id":phantom})),
    )
    .unwrap();
    let successor = f.note("successor");
    link(&f.store, &successor, &id, EdgeType::Supersedes).unwrap();
    let before = f
        .store
        .run_read("?[count(id)] := *node{id,type @ 'NOW'},type='audit_event'")
        .unwrap()
        .rows;
    let response = f.call(
        "read",
        json!({"initiative":"work","node":id,"include":["relations"]}),
    );
    let ResponseData::Record(data) = response.data else {
        panic!("record expected")
    };
    assert!(data.node.successors.contains(&successor));
    let after = f
        .store
        .run_read("?[count(id)] := *node{id,type @ 'NOW'},type='audit_event'")
        .unwrap()
        .rows;
    assert_eq!(before, after, "journaling adds no graph writes");
    let events = f.events();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].node_ids, vec![id]);
    assert_eq!(events[0].initiatives, vec!["work"]);
    let serialized = serde_json::to_value(&events[0]).unwrap();
    assert_eq!(serialized.as_object().unwrap().len(), 4);
    let encoded = serialized.to_string();
    for forbidden in [
        &phantom,
        &successor,
        "never-log-this-name",
        "never log this body",
    ] {
        assert!(!encoded.contains(forbidden));
    }
}

#[test]
fn pages_empty_reads_and_history_are_traced_but_denied_reads_and_mutations_are_not() {
    let f = Fixture::new(true, false);
    for name in ["one", "two", "three"] {
        f.note(name);
    }
    let first = f.call(
        "search",
        json!({"initiatives":["work"],"text":"group","limit":1}),
    );
    let first_id = match &first.data {
        ResponseData::Search(d) => d.hits[0].id.clone(),
        _ => panic!(),
    };
    let second = f.call("search", first.continuation.unwrap().next_arguments);
    let second_id = match &second.data {
        ResponseData::Search(d) => d.hits[0].id.clone(),
        _ => panic!(),
    };
    assert_ne!(first_id, second_id);
    f.call(
        "search",
        json!({"initiatives":["work"],"text":"no-such-lexical-term"}),
    );
    f.call("history", json!({"initiative":"work","node":first_id}));
    f.call("policy", json!({"initiative":"work"}));
    let denied = api::execute(
        &f.store,
        "read",
        json!({"initiative":"work","node":first_id}),
        &ApiOptions {
            allowed_initiatives: Some(vec!["elsewhere".into()]),
            ..ApiOptions::local()
        },
    );
    assert!(denied.is_error());
    let events = f.events();
    assert_eq!(events.len(), 5);
    let searches: Vec<_> = events.iter().filter(|e| e.operation == "search").collect();
    assert_eq!(searches[0].node_ids, vec![first_id.clone()]);
    assert_eq!(searches[1].node_ids, vec![second_id]);
    assert!(searches[2].node_ids.is_empty());
    assert_eq!(
        events
            .iter()
            .find(|e| e.operation == "history")
            .unwrap()
            .node_ids,
        vec![first_id]
    );
    assert!(
        events
            .iter()
            .find(|e| e.operation == "policy")
            .unwrap()
            .node_ids
            .is_empty()
    );
}

#[test]
fn context_and_retention_log_delivered_records_once() {
    let f = Fixture::new(true, false);
    f.note("context fact");
    let context = f.call("context", json!({"initiative":"work"}));
    f.call(
        "retention",
        json!({"initiative":"work","action":"preview_hygiene"}),
    );
    let ResponseData::Context(data) = context.data else {
        panic!("context expected")
    };
    let mut expected: Vec<_> = data
        .sections
        .iter()
        .flat_map(|s| s.records.iter().map(|r| r.id.clone()))
        .chain(data.proposals.iter().map(|p| p.id.clone()))
        .collect();
    expected.sort();
    expected.dedup();
    let events = f.events();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].operation, "context");
    assert_eq!(events[0].node_ids, expected);
    assert_eq!(events[1].operation, "retention");
}

#[test]
fn disabled_or_unwritable_journals_leave_reads_successful() {
    for (enabled, broken) in [(false, false), (true, true)] {
        let f = Fixture::new(enabled, broken);
        let id = f.note("safe read");
        let response = f.call("read", json!({"initiative":"work","node":id}));
        assert!(matches!(response.data, ResponseData::Record(_)));
        if !enabled {
            assert!(!f.directory.join("journal").exists());
        } else {
            assert!(!f.store.flush_read_trace(Duration::from_secs(2)));
        }
    }
}
