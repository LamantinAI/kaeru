#[path = "support/recorded_reference.rs"]
mod recorded_reference;

use chrono::DateTime;
use kaeru_core::{
    EdgeType, Error, KaeruConfig, Layer, NodeType, Store, Tier, Visibility,
    api::{self, ApiOptions, ResponseData},
    at, forget, history, improve, link_with_weight, new_node_id, read_node_full, set_layer, unlink,
    upsert_node,
};
use proptest::prelude::*;
use serde_json::json;

use recorded_reference::RecordedReference;

fn store() -> Store {
    Store::open_in_memory_with(KaeruConfig::defaults()).unwrap()
}

fn write_node(store: &Store, id: &str, seconds: u64, asserted: bool, value: u8) {
    store.run(&format!(
        "?[id,validity,type,tier,name,body,tags,initiatives,properties,visibility,layer] <- [['{id}',[{seconds}.0,{asserted}],'episode','operational','n{value}','v{value}',[],null,null,'local','warm']] :put node {{id,validity=>type,tier,name,body,tags,initiatives,properties,visibility,layer}}"
    )).unwrap();
}

fn write_edge(
    store: &Store,
    key: &(String, String, String),
    seconds: u64,
    asserted: bool,
    value: u8,
) {
    let (src, dst, kind) = key;
    store.run(&format!(
        "?[src,dst,edge_type,validity,weight,properties] <- [['{src}','{dst}','{kind}',[{seconds}.0,{asserted}],{value}.0,parse_json('{{\"value\":{value}}}')]] :put edge {{src,dst,edge_type,validity=>weight,properties}}"
    )).unwrap();
}

fn edge_at(store: &Store, key: &(String, String, String), time: &str) -> Option<(f64, u8)> {
    let (src, dst, kind) = key;
    let rows = store.run_read(&format!(
        "?[weight,properties] := *edge{{src,dst,edge_type,weight,properties @ {time}}},src='{src}',dst='{dst}',edge_type='{kind}'"
    )).unwrap();
    rows.rows.first().map(|row| {
        let props = serde_json::Value::from(row[1].clone());
        (
            row[0].get_float().unwrap(),
            props["value"].as_u64().unwrap() as u8,
        )
    })
}

#[test]
fn exact_boundaries_ties_retractions_and_same_key_rewrites_match_the_oracle() {
    let store = store();
    let id = "boundary-node".to_string();
    let key = ("a".to_string(), "b".to_string(), "refers_to".to_string());
    let mut reference = RecordedReference::new();
    for (time, asserted, value) in [
        (10, true, 1),
        (20, false, 2),
        (30, false, 3),
        (30, true, 4),
        (30, true, 5),
    ] {
        reference.put(id.clone(), time, asserted, value);
        write_node(&store, &id, time, asserted, value);
        write_edge(&store, &key, time, asserted, value);
    }
    for time in [0, 9, 10, 11, 19, 20, 21, 29, 30, 31] {
        let expected = reference.at(&id, time).copied();
        assert_eq!(
            at(&store, &id, time as f64).unwrap().and_then(|n| n.body),
            expected.map(|v| format!("v{v}")),
            "time={time}"
        );
        assert_eq!(
            edge_at(&store, &key, &format!("{time}.0")),
            expected.map(|v| (v as f64, v)),
            "edge time={time}"
        );
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(16))]
    #[test]
    fn mixed_recorded_operations_match_an_independent_model(
        operations in prop::collection::vec((0usize..2,0u64..8,any::<bool>(),0u8..20),1..16)
    ) {
        let store = store();
        let mut nodes = RecordedReference::new();
        let mut edges = RecordedReference::new();
        for &(key,time,asserted,value) in &operations {
            let id = format!("node-{key}");
            let edge = (id.clone(),"destination".to_string(),"refers_to".to_string());
            nodes.put(id.clone(),time,asserted,value);
            edges.put(edge.clone(),time,asserted,value);
            write_node(&store,&id,time,asserted,value);
            write_edge(&store,&edge,time,asserted,value);
        }
        for key in 0..2 {
            let id = format!("node-{key}");
            let edge = (id.clone(),"destination".to_string(),"refers_to".to_string());
            for time in 0..9 {
                let expected = nodes.at(&id,time).copied();
                prop_assert_eq!(at(&store,&id,time as f64).unwrap().and_then(|n|n.body),expected.map(|v|format!("v{v}")),"node={},time={},ops={:?}",id,time,operations);
                prop_assert_eq!(edge_at(&store,&edge,&format!("{time}.0")),edges.at(&edge,time).map(|v|(*v as f64,*v)),"edge={:?},time={},ops={:?}",edge,time,operations);
            }
        }
    }
}

#[test]
fn public_revisions_retraction_and_failed_transaction_follow_requested_content() {
    let store = store();
    store.use_initiative("reference");
    let id = new_node_id();
    let mut reference = RecordedReference::new();
    let mut times = Vec::new();
    upsert_node(
        &store,
        &id,
        NodeType::Episode,
        Tier::Operational,
        "version-0",
        Some("first"),
        &[],
        Some("reference"),
        Visibility::Local,
        Layer::Warm,
        None,
    )
    .unwrap();
    for (index, body) in ["first", "second", "third"].iter().enumerate() {
        if index > 0 {
            improve(&store, &id, &format!("version-{index}"), body).unwrap();
        }
        // Only the allocated timestamp is observed; expected content is the
        // requested operation, never reconstructed from recorded database rows.
        let time = history(&store, &id)
            .unwrap()
            .iter()
            .map(|r| r.seconds as u64)
            .max()
            .unwrap();
        assert!(times.last().is_none_or(|previous| time > *previous));
        times.push(time);
        reference.put(id.clone(), time, true, body.to_string());
        for &at_time in &times {
            assert_eq!(
                at(&store, &id, at_time as f64)
                    .unwrap()
                    .and_then(|n| n.body)
                    .as_ref(),
                reference.at(&id, at_time)
            );
        }
        assert_eq!(
            read_node_full(&store, &id).unwrap().unwrap().body.as_ref(),
            reference.at(&id, u64::MAX)
        );
    }
    let aborted: kaeru_core::Result<()> = store.atomic(|s| {
        improve(s, &id, "uncommitted", "must not appear")?;
        Err(Error::Invalid("injected rollback".into()))
    });
    assert!(aborted.is_err());
    assert_eq!(
        read_node_full(&store, &id).unwrap().unwrap().body.as_ref(),
        reference.at(&id, u64::MAX)
    );
    forget(&store, &id).unwrap();
    let retracted = history(&store, &id)
        .unwrap()
        .iter()
        .map(|r| r.seconds as u64)
        .max()
        .unwrap();
    assert!(retracted > *times.last().unwrap());
    reference.put(id.clone(), retracted, false, String::new());
    assert!(read_node_full(&store, &id).unwrap().is_none());
    assert_eq!(
        at(&store, &id, retracted as f64)
            .unwrap()
            .and_then(|n| n.body)
            .as_ref(),
        reference.at(&id, retracted)
    );
    for time in times {
        assert_eq!(
            at(&store, &id, time as f64)
                .unwrap()
                .and_then(|n| n.body)
                .as_ref(),
            reference.at(&id, time)
        );
    }
}

#[test]
fn public_edge_revisions_and_unlink_match_the_oracle() {
    let store = store();
    let key = (
        "left".to_string(),
        "right".to_string(),
        "refers_to".to_string(),
    );
    let mut reference = RecordedReference::new();
    let mut times = Vec::new();
    for weight in [0.3, 0.9] {
        link_with_weight(&store, &key.0, &key.1, EdgeType::RefersTo, weight).unwrap();
        let rows = store.run_read("?[t] := *edge{src,dst,edge_type,validity:t},src='left',dst='right',edge_type='refers_to'").unwrap();
        let time = rows
            .rows
            .iter()
            .filter_map(|r| match &r[0] {
                cozo::DataValue::Validity(v) => Some(v.timestamp.0.0 as u64),
                _ => None,
            })
            .max()
            .unwrap();
        assert!(times.last().is_none_or(|previous| time > *previous));
        times.push(time);
        reference.put(key.clone(), time, true, weight);
        let rows = store.run_read("?[w] := *edge{src,dst,edge_type,weight:w @ 'NOW'},src='left',dst='right',edge_type='refers_to'").unwrap();
        assert_eq!(
            rows.rows[0][0].get_float().as_ref(),
            reference.at(&key, u64::MAX)
        );
    }
    unlink(&store, &key.0, &key.1, EdgeType::RefersTo).unwrap();
    let rows = store.run_read("?[t] := *edge{src,dst,edge_type,validity:t},src='left',dst='right',edge_type='refers_to'").unwrap();
    let time = rows
        .rows
        .iter()
        .filter_map(|r| match &r[0] {
            cozo::DataValue::Validity(v) => Some(v.timestamp.0.0 as u64),
            _ => None,
        })
        .max()
        .unwrap();
    assert!(time > *times.last().unwrap());
    reference.put(key.clone(), time, false, 0.0);
    let current = store.run_read("?[w] := *edge{src,dst,edge_type,weight:w @ 'NOW'},src='left',dst='right',edge_type='refers_to'").unwrap();
    assert!(current.rows.is_empty());
    for time in times {
        let rows = store.run_read(&format!("?[w] := *edge{{src,dst,edge_type,weight:w @ {time}.0}},src='left',dst='right',edge_type='refers_to'")).unwrap();
        assert_eq!(
            rows.rows[0][0].get_float().as_ref(),
            reference.at(&key, time)
        );
    }
}

#[test]
fn historical_api_content_is_distinct_from_current_metadata() {
    let store = store();
    store.use_initiative("reference");
    let id = new_node_id();
    upsert_node(
        &store,
        &id,
        NodeType::Episode,
        Tier::Operational,
        "original",
        Some("old content"),
        &[],
        Some("reference"),
        Visibility::Local,
        Layer::Warm,
        None,
    )
    .unwrap();
    let time = history(&store, &id).unwrap()[0].seconds as i64;
    improve(&store, &id, "revised", "new content").unwrap();
    set_layer(&store, &id, Layer::Cold).unwrap();
    let response = api::execute(
        &store,
        "read",
        json!({"initiative":"reference","node":id,"recorded_at":DateTime::from_timestamp(time,0).unwrap().to_rfc3339()}),
        &ApiOptions::local(),
    );
    assert!(!response.is_error(), "{response:?}");
    let ResponseData::Record(data) = response.data else {
        panic!("record expected")
    };
    assert_eq!(data.node.body.as_deref(), Some("old content"));
    assert_eq!(data.node.layer, "cold");
    assert!(data.node.metadata_is_current);
}
