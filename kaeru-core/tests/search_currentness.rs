use kaeru_core::{
    EdgeType, Store, Visibility,
    api::{self, ApiOptions, Response, ResponseData},
    link,
    models::NodeData,
    set_visibility,
};
use serde_json::{Value, json};

fn call(store: &Store, operation: &str, args: Value, options: &ApiOptions) -> Response {
    let response = api::execute(store, operation, args, options);
    assert!(!response.is_error(), "{operation}: {response:?}");
    jsonschema::validator_for(&api::output_schema_for(operation))
        .unwrap()
        .validate(&serde_json::to_value(&response).unwrap())
        .unwrap();
    response
}

fn local(store: &Store, operation: &str, args: Value) -> Response {
    call(store, operation, args, &ApiOptions::local())
}

fn primary(response: Response) -> String {
    let ResponseData::Mutation(data) = response.data else {
        panic!("expected mutation");
    };
    data.primary.unwrap().id
}

fn note(store: &Store, name: &str, body: &str) -> String {
    primary(local(
        store,
        "note",
        json!({"initiative":"work","name":name,"body":body}),
    ))
}

fn relate(store: &Store, from: &str, to: &str, kind: &str) {
    local(
        store,
        "relate",
        json!({
            "initiative":"work","action":"connect","from":from,"to":to,"type":kind,"weight":1.0
        }),
    );
}

fn hits(response: &Response) -> &[NodeData] {
    let ResponseData::Search(data) = &response.data else {
        panic!("expected search");
    };
    &data.hits
}

#[test]
fn a_stale_lexical_hit_identifies_and_includes_its_accessible_replacement() {
    let store = Store::open_in_memory().unwrap();
    let old = note(&store, "router-plan", "Old router configuration.");
    let new = note(&store, "new-direction", "Use the revised configuration.");
    relate(&store, &new, &old, "supersedes");

    let response = local(
        &store,
        "search",
        json!({"initiatives":["work"],"text":"router"}),
    );
    let stale = hits(&response).iter().find(|n| n.id == old).unwrap();
    assert_eq!(stale.currentness, "superseded");
    assert_eq!(stale.successors, [new.clone()]);
    assert!(
        hits(&response)
            .iter()
            .any(|n| n.id == new && n.currentness == "current")
    );
}

#[test]
fn a_disputed_hit_is_marked_and_an_ordinary_read_names_the_counterevidence() {
    let store = Store::open_in_memory().unwrap();
    let fact = note(&store, "router-speed", "Router speed is 90 Mbps.");
    let evidence = note(&store, "speed-measurement", "Measured 40 Mbps over Wi-Fi.");
    relate(&store, &evidence, &fact, "contradicts");

    let found = local(
        &store,
        "search",
        json!({"initiatives":["work"],"text":"router"}),
    );
    assert_eq!(
        hits(&found)
            .iter()
            .find(|n| n.id == fact)
            .unwrap()
            .currentness,
        "contested"
    );
    let read = local(&store, "read", json!({"initiative":"work","node":fact}));
    let ResponseData::Record(data) = read.data else {
        panic!("expected record")
    };
    assert!(
        data.conflicts
            .iter()
            .any(|e| e.from == evidence && e.to == fact && e.edge_type == "contradicts")
    );
}

#[test]
fn a_refuted_hypothesis_is_marked_in_search_without_a_graph_expansion() {
    let store = Store::open_in_memory().unwrap();
    let claim = primary(local(
        &store,
        "claim",
        json!({
            "initiative":"work","text":"A larger cache improves router performance."
        }),
    ));
    local(
        &store,
        "verdict",
        json!({
            "initiative":"work","hypothesis":claim,"status":"refuted",
            "evidence":{"method":"The measurement showed no improvement."}
        }),
    );
    let response = local(
        &store,
        "search",
        json!({"initiatives":["work"],"text":"router"}),
    );
    let hit = hits(&response).iter().find(|n| n.id == claim).unwrap();
    assert_eq!(hit.currentness, "refuted");
    assert_eq!(hit.status.as_deref(), Some("refuted"));
}

#[test]
fn read_expands_incoming_and_outgoing_links_beyond_provenance_edges() {
    let store = Store::open_in_memory().unwrap();
    let center = note(&store, "center", "The selected record.");
    let other = note(&store, "linked-record", "A linked fact.");
    let blocker = note(&store, "blocker", "An unfinished dependency.");
    relate(&store, &center, &other, "refers_to");
    relate(&store, &blocker, &center, "blocks");

    let response = local(
        &store,
        "read",
        json!({"initiative":"work","node":center,"include":["relations"]}),
    );
    let ResponseData::Record(data) = response.data else {
        panic!("expected record")
    };
    assert!(
        data.relations
            .iter()
            .any(|e| e.from == center && e.to == other && e.edge_type == "refers_to")
    );
    assert!(
        data.relations
            .iter()
            .any(|e| e.from == blocker && e.to == center && e.edge_type == "blocks")
    );
}

#[test]
fn inaccessible_peers_do_not_leak_through_search_metadata_or_expanded_relations() {
    for hidden_scope in ["private", "work"] {
        let store = Store::open_in_memory().unwrap();
        let old = note(&store, "router-plan", "Public router configuration.");
        set_visibility(&store, &old, Visibility::Shared).unwrap();
        let hidden = primary(local(
            &store,
            "note",
            json!({
                "initiative":hidden_scope,"name":"hidden-peer","body":"Private replacement."
            }),
        ));
        // Exercise the initiative ceiling and visibility gate independently.
        if hidden_scope == "private" {
            set_visibility(&store, &hidden, Visibility::Shared).unwrap();
        }
        link(&store, &hidden, &old, EdgeType::Supersedes).unwrap();
        link(&store, &hidden, &old, EdgeType::Contradicts).unwrap();
        let options = ApiOptions {
            allowed_initiatives: Some(vec!["work".into()]),
            include_local: false,
            ..ApiOptions::local()
        };
        let found = call(
            &store,
            "search",
            json!({"initiatives":["work"],"text":"router"}),
            &options,
        );
        assert_eq!(hits(&found).len(), 1);
        assert_eq!(hits(&found)[0].currentness, "current");
        assert!(hits(&found)[0].successors.is_empty());
        let read = call(
            &store,
            "read",
            json!({"initiative":"work","node":old,"include":["relations"]}),
            &options,
        );
        for response in [&found, &read] {
            let serialized = serde_json::to_string(response).unwrap();
            assert!(!serialized.contains(&hidden));
            assert!(!serialized.contains("hidden-peer"));
        }
        let ResponseData::Record(data) = read.data else {
            panic!("expected record")
        };
        assert!(data.conflicts.is_empty());
        assert!(data.relations.is_empty());
    }
}
