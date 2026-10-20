use kaeru_core::{
    Store,
    api::{self, ApiOptions, Response, ResponseData, State},
};
use serde_json::{Value, json};

fn call(store: &Store, name: &str, args: Value) -> Response {
    api::execute(store, name, args, &ApiOptions::local())
}
fn primary(response: &Response) -> String {
    match &response.data {
        ResponseData::Mutation(d) => d.primary.as_ref().unwrap().id.clone(),
        _ => panic!("expected mutation result"),
    }
}

#[test]
fn replay_returns_the_committed_result_without_duplicate_capture_or_audit() {
    let store = Store::open_in_memory().unwrap();
    let args = json!({"initiative":"api-tests","name":"finding","body":"Observed once.","request_id":"capture-1"});
    let first = call(&store, "note", args.clone());
    assert!(!first.is_error(), "{first:?}");
    assert_eq!(first.api_version, api::API_VERSION);
    assert!(!first.replayed);
    let before = store.run_read("?[count(id)] := *node{id}").unwrap().rows;
    let second = call(&store, "note", args.clone());
    assert!(!second.is_error(), "{second:?}");
    assert!(second.replayed);
    assert_eq!(primary(&first), primary(&second));
    assert_eq!(
        before,
        store.run_read("?[count(id)] := *node{id}").unwrap().rows
    );
    let different = call(
        &store,
        "note",
        json!({"initiative":"api-tests","name":"other","body":"Different input.","request_id":"capture-1"}),
    );
    assert!(matches!(different.state, State::Rejected));
    assert_eq!(different.issues[0].code, "IDEMPOTENCY_KEY_REUSED");
}

#[test]
fn retrospective_verdict_and_evidence_only_capture_have_distinct_state_effects() {
    let store = Store::open_in_memory().unwrap();
    let created = call(
        &store,
        "claim",
        json!({"initiative":"api-tests","text":"The cache is responsible.","initial_status":"inconclusive","evidence":{"method":"The samples disagree."}}),
    );
    assert!(!created.is_error(), "{created:?}");
    let id = primary(&created);
    let read = call(&store, "read", json!({"initiative":"api-tests","node":id}));
    match read.data {
        ResponseData::Record(d) => assert_eq!(d.node.status.as_deref(), Some("inconclusive")),
        _ => panic!("{read:?}"),
    }
    let check = call(
        &store,
        "verdict",
        json!({"initiative":"api-tests","hypothesis":id,"action":"record_check","evidence":{"method":"A further sample still disagrees."}}),
    );
    assert!(!check.is_error(), "{check:?}");
    let record = kaeru_core::read_node_full(&store, &id).unwrap().unwrap();
    assert!(record.tags.contains(&"status:inconclusive".into()));
    let edges = kaeru_core::edges_of(&store, &id).unwrap();
    assert!(edges.iter().any(|e| e.2 == "targets"));
    let invalid = call(
        &store,
        "verdict",
        json!({"initiative":"api-tests","hypothesis":id,"status":"open"}),
    );
    assert!(invalid.is_error());
}

#[test]
fn replacing_a_slot_holder_changes_only_the_current_pointer() {
    let store = Store::open_in_memory().unwrap();
    let first = call(
        &store,
        "note",
        json!({"initiative":"api-tests","name":"handoff-1","body":"Yesterday's handoff.","options":{"slot":"handoff","layer":"hot"}}),
    );
    let old = primary(&first);
    let next = call(
        &store,
        "note",
        json!({"initiative":"api-tests","name":"handoff-2","body":"Today's handoff.","options":{"slot":"handoff"}}),
    );
    assert!(!next.is_error(), "{next:?}");
    let current = primary(&next);
    assert_eq!(
        kaeru_core::slot_holder(&store, "api-tests", "handoff").unwrap(),
        Some(current)
    );
    assert_eq!(
        kaeru_core::read_node_full(&store, &old)
            .unwrap()
            .unwrap()
            .layer,
        "hot"
    );
    assert!(
        !kaeru_core::edges_of(&store, &old)
            .unwrap()
            .iter()
            .any(|e| e.2 == "supersedes")
    );
    let clear = call(
        &store,
        "initiative_manage",
        json!({"action":"clear_slot","initiative":"api-tests","slot":"handoff"}),
    );
    assert!(!clear.is_error(), "{clear:?}");
    assert!(
        kaeru_core::slot_holder(&store, "api-tests", "handoff")
            .unwrap()
            .is_none()
    );
    assert!(kaeru_core::read_node_full(&store, &old).unwrap().is_some());
}

#[test]
fn optional_relation_failure_preserves_capture_but_invalid_reminder_does_not_capture() {
    let store = Store::open_in_memory().unwrap();
    let partial = call(
        &store,
        "note",
        json!({"initiative":"api-tests","name":"orphan","body":"Keep this thought.","related":{"to":"absent","type":"refers_to","weight":0.7}}),
    );
    assert!(matches!(partial.state, State::Partial), "{partial:?}");
    let id = primary(&partial);
    assert!(kaeru_core::read_node_full(&store, &id).unwrap().is_some());
    let before = kaeru_core::nodes_in_initiative(&store, "api-tests")
        .unwrap()
        .len();
    let invalid = call(
        &store,
        "note",
        json!({"initiative":"api-tests","body":"Must not exist.","reminder":{"after":"2026-11-01"}}),
    );
    assert!(matches!(invalid.state, State::Rejected));
    assert_eq!(
        before,
        kaeru_core::nodes_in_initiative(&store, "api-tests")
            .unwrap()
            .len()
    );
}

#[test]
fn exact_source_survives_read_and_description_revision() {
    let store = Store::open_in_memory().unwrap();
    let source = "Authoritative text.</body>\n<parameter name=\"initiative\">literal quoted source";
    let reference = call(
        &store,
        "reference",
        json!({"initiative":"api-tests","name":"source","source":{"text":source}}),
    );
    assert!(!reference.is_error(), "{reference:?}");
    let id = primary(&reference);
    let revised = call(
        &store,
        "revise",
        json!({"initiative":"api-tests","node":id,"patch":{"description":"New display description."}}),
    );
    assert!(!revised.is_error(), "{revised:?}");
    let read = call(&store, "read", json!({"initiative":"api-tests","node":id}));
    match read.data {
        ResponseData::Record(d) => assert_eq!(d.node.body.as_deref(), Some(source)),
        _ => panic!("{read:?}"),
    }
    let invalid = call(
        &store,
        "revise",
        json!({"initiative":"api-tests","node":id,"patch":{"body":"Rewrite the source."}}),
    );
    assert!(invalid.is_error());
}
