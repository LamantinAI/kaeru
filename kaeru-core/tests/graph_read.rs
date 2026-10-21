use std::collections::BTreeSet;
use std::thread::sleep;
use std::time::Duration;

use chrono::Utc;
use kaeru_core::api::{self, ApiOptions, Response, ResponseData};
use kaeru_core::{Layer, NodeType, Store, Tier, Visibility, upsert_node};
use serde_json::{Value, from_value, json, to_value};

fn record(store: &Store, id: &str, kind: NodeType, scope: &str, visibility: Visibility) {
    upsert_node(
        store,
        &id.into(),
        kind,
        Tier::Operational,
        id,
        Some("SECRET_BODY must never be in graph"),
        &[],
        Some(scope),
        visibility,
        Layer::Warm,
        None,
    )
    .unwrap();
}
fn viewer() -> ApiOptions {
    ApiOptions {
        include_local: false,
        allowed_initiatives: Some(vec!["work".into()]),
        ..ApiOptions::local()
    }
}
fn call(store: &Store, args: Value, options: &ApiOptions) -> Response {
    api::execute(store, "context", args, options)
}

#[test]
fn structural_graph_respects_scope_visibility_remote_edges_and_incomplete_trails() {
    let store = Store::open_in_memory().unwrap();
    record(&store, "a", NodeType::Episode, "work", Visibility::Shared);
    record(&store, "b", NodeType::Episode, "work", Visibility::Shared);
    record(
        &store,
        "hidden",
        NodeType::Episode,
        "work",
        Visibility::Local,
    );
    record(
        &store,
        "elsewhere",
        NodeType::Episode,
        "elsewhere",
        Visibility::Shared,
    );
    record(&store, "trail", NodeType::Chain, "work", Visibility::Shared);
    store.run("?[src,dst,edge_type,validity,weight,properties,dst_store] <- [['a','b','causal',[1000.0,true],0.5,null,'local'],['a','hidden','refers_to',[1000.0,true],1.0,null,'local'],['b','a','refers_to',[1000.0,true],1.0,null,'cloud']] :put edge {src,dst,edge_type,validity=>weight,properties,dst_store}").unwrap();
    store.run("?[chain_id,position,node_id] <- [['trail',0,'a'],['trail',1,'hidden'],['trail',2,'b']] :put chain_member {chain_id,position=>node_id}").unwrap();
    let response = call(
        &store,
        json!({"view":"graph","budget_chars":64000}),
        &viewer(),
    );
    assert!(!response.is_error(), "{response:?}");
    assert!(!response.coverage.as_ref().unwrap().rules_complete);
    let encoded = to_value(&response).unwrap();
    assert!(!encoded.to_string().contains("SECRET_BODY"));
    jsonschema::validator_for(&api::output_schema_for("context"))
        .unwrap()
        .validate(&encoded)
        .unwrap();
    let ResponseData::Graph(data) = response.data else {
        panic!("graph expected")
    };
    assert_eq!(
        data.graph
            .nodes
            .iter()
            .map(|n| n.id.as_str())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["a", "b", "trail"])
    );
    assert!(data.graph.nodes.iter().all(|n| n.created_at.is_none()));
    assert_eq!(data.graph.edges.len(), 1);
    assert_eq!(data.graph.edges[0].edge_type, "causal");
    assert!(
        data.graph.members.is_empty(),
        "never splice around hidden members"
    );
    assert_eq!(api::tool_definitions().len(), 35);
    assert!(
        call(
            &store,
            json!({"view":"graph","initiatives":["elsewhere"]}),
            &viewer()
        )
        .is_error()
    );
}

fn paged_fixture() -> Store {
    let store = Store::open_in_memory().unwrap();
    for n in 0..25 {
        record(
            &store,
            &format!("record-{n:03}"),
            NodeType::Episode,
            "work",
            Visibility::Shared,
        );
    }
    record(&store, "trail", NodeType::Chain, "work", Visibility::Shared);
    let rows = (0..25)
        .map(|n| format!("['trail',{n},'record-{n:03}']"))
        .collect::<Vec<_>>()
        .join(",");
    store.run(&format!("?[chain_id,position,node_id] <- [{rows}] :put chain_member {{chain_id,position=>node_id}}")).unwrap();
    store.run("?[src,dst,edge_type,validity,weight,properties,dst_store] := *node{id:src,type @ 'NOW'},type='episode',dst='trail',edge_type='causal',validity=[1000.0,true],weight=1.0,properties=null,dst_store='local' :put edge {src,dst,edge_type,validity=>weight,properties,dst_store}").unwrap();
    store
}

#[test]
fn compact_pages_roundtrip_and_preserve_chain_order_without_per_node_payloads() {
    let store = paged_fixture();
    let options = viewer();
    let mut args = json!({"view":"graph","include":["created_at"],"budget_chars":1500});
    let (mut ids, mut members, mut edges, mut pages) = (BTreeSet::new(), vec![], 0, 0);
    let mut snapshot = None;
    let mut edge_only = false;
    loop {
        let response = call(&store, args, &options);
        assert!(!response.is_error(), "{response:?}");
        let encoded = to_value(&response).unwrap();
        assert!(encoded.to_string().chars().count() <= 1500);
        jsonschema::validator_for(&api::output_schema_for("context"))
            .unwrap()
            .validate(&encoded)
            .unwrap();
        let decoded: Response = from_value(encoded).unwrap();
        let ResponseData::Graph(data) = decoded.data else {
            panic!("graph DTO roundtrip")
        };
        if let Some(id) = &snapshot {
            assert_eq!(id, &data.graph.snapshot_id);
        } else {
            snapshot = Some(data.graph.snapshot_id.clone());
        }
        edge_only |= data.graph.nodes.is_empty() && !data.graph.edges.is_empty();
        for n in data.graph.nodes {
            assert!(n.created_at.is_some());
            assert!(ids.insert(n.id));
        }
        edges += data.graph.edges.len();
        members.extend(data.graph.members);
        pages += 1;
        if let Some(next) = response.continuation {
            args = next.next_arguments;
        } else {
            break;
        }
    }
    assert!(pages > 3 && edge_only);
    assert_eq!(ids.len(), 26);
    assert_eq!(edges, 25);
    assert_eq!(members.len(), 25);
    assert_eq!(
        members.iter().map(|m| m.position).collect::<Vec<_>>(),
        (0..25).collect::<Vec<_>>()
    );
}

#[test]
fn graph_continuation_rejects_future_visibility_revocation_without_a_write() {
    let store = paged_fixture();
    let options = viewer();
    let transition = Utc::now().timestamp_micros() + 200_000;
    store.run(&format!("?[id,validity,type,tier,name,body,tags,initiatives,properties,visibility,layer] <- [['record-000',[{transition}.0,true],'episode','operational','private','new private',null,null,null,'local','warm']] :put node {{id,validity=>type,tier,name,body,tags,initiatives,properties,visibility,layer}}")).unwrap();
    let first = call(
        &store,
        json!({"view":"graph","budget_chars":1500}),
        &options,
    );
    assert!(!first.is_error(), "{first:?}");
    let next = first.continuation.unwrap().next_arguments;
    while Utc::now().timestamp_micros() <= transition {
        sleep(Duration::from_millis(5));
    }
    let response = call(&store, next, &options);
    assert!(
        response.issues.iter().any(|i| i.code == "CURSOR_STALE"),
        "{response:?}"
    );
}

#[test]
fn empty_graph_is_unambiguously_typed_and_scope_selectors_are_checked() {
    let store = Store::open_in_memory().unwrap();
    let options = viewer();
    let response = call(&store, json!({"view":"graph","budget_chars":512}), &options);
    assert!(!response.is_error(), "{response:?}");
    let roundtrip: Response = from_value(to_value(response).unwrap()).unwrap();
    assert!(matches!(roundtrip.data, ResponseData::Graph(_)));
    for args in [
        json!({"view":"graph","initiative":"work","initiatives":["work"]}),
        json!({"view":"map","initiatives":["work"]}),
        json!({"view":"graph","query":"irrelevant"}),
    ] {
        assert!(call(&store, args, &options).is_error());
    }
}
