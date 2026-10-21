use cozo::ScriptMutability;
use serde_json::json;

use crate::api::{self, ApiOptions};
use crate::store::test_support::capture;
use crate::{Layer, NodeType, Store, Tier, Visibility, upsert_node};

#[test]
fn compact_graph_uses_bulk_structural_queries_without_bodies_or_node_revisions() {
    let store = Store::open_in_memory().unwrap();
    store.run("?[id,validity,type,tier,name,body,tags,initiatives,properties,visibility,layer] := i in int_range(0,500),id=concat('node-',to_string(i)),validity=[1000.0,true],type='episode',tier='operational',name=id,body='not loaded',tags=null,initiatives=null,properties=null,visibility='shared',layer='warm' :put node {id,validity=>type,tier,name,body,tags,initiatives,properties,visibility,layer}").unwrap();
    store.run("?[initiative,node_id] := *node{id:node_id @ 'NOW'},initiative='perf' :put node_initiative {initiative,node_id}").unwrap();
    let (result, queries) = capture(|| {
        api::execute(
            &store,
            "context",
            json!({"view":"graph","budget_chars":64000}),
            &ApiOptions::local(),
        )
    });
    assert!(!result.is_error(), "{result:?}");
    assert!(queries.len() < 15, "{} queries: {queries:?}", queries.len());
    for (sql, params) in queries {
        assert!(
            !sql.contains("body")
                && !sql.contains("properties")
                && !sql.contains("api_trail_summary"),
            "{sql}"
        );
        assert!(!params.contains_key("id"), "no scalar hydration: {sql}");
        if sql.contains("*node{") {
            assert!(
                sql.contains("@ 'NOW'"),
                "no history without created_at: {sql}"
            );
        }
    }
}

#[test]
fn real_api_point_reads_use_prefix_joins_including_quoted_legacy_ids() {
    let store = Store::open_in_memory().unwrap();
    let id = "legacy-'quoted\\id".to_string();
    upsert_node(
        &store,
        &id,
        NodeType::Episode,
        Tier::Operational,
        "legacy",
        Some("literal"),
        &[],
        Some("perf"),
        Visibility::Local,
        Layer::Warm,
        None,
    )
    .unwrap();
    let (result, queries) = capture(|| {
        api::execute(
            &store,
            "read",
            json!({"initiative":"perf","node":{"id":id}}),
            &ApiOptions::local(),
        )
    });
    assert!(!result.is_error(), "{result:?}");
    let mut nodes = 0;
    let mut memberships = 0;
    for (sql, params) in queries {
        let keyed_node = sql.contains("*node{") && params.contains_key("id");
        let membership = sql.contains("*node_initiative{") && params.contains_key("nid");
        if !keyed_node && !membership {
            continue;
        }
        let plan = store
            .db_ref()
            .run_script(
                &format!("::explain {{{sql}}}"),
                params,
                ScriptMutability::Immutable,
            )
            .unwrap();
        let text = format!("{plan:?}");
        assert!(text.contains("stored_prefix_join"), "{sql}\n{text}");
        if membership {
            assert!(text.contains(":node_initiative:by_node"));
            memberships += 1;
        } else {
            nodes += 1;
        }
    }
    assert!(
        nodes >= 3 && memberships >= 1,
        "nodes={nodes} memberships={memberships}"
    );
}

#[test]
fn indexed_incident_edges_match_full_scan_payload_across_versions_and_retractions() {
    use std::collections::BTreeMap;

    use cozo::{DataValue, JsonData};
    use serde_json::Value;
    let store = Store::open_in_memory().unwrap();
    let a = "a".to_string();
    let b = "b".to_string();
    for id in [&a, &b] {
        upsert_node(
            &store,
            id,
            NodeType::Episode,
            Tier::Operational,
            id,
            Some("body"),
            &[],
            Some("perf"),
            Visibility::Local,
            Layer::Warm,
            None,
        )
        .unwrap();
    }
    let compare = || {
        let rows=store.db_ref().run_script("?[src,dst,edge_type,weight,properties,dst_store] := *edge{src,dst,edge_type,weight,properties,dst_store @ 'NOW'},src=$id\n?[src,dst,edge_type,weight,properties,dst_store] := *edge{src,dst,edge_type,weight,properties,dst_store @ 'NOW'},dst=$id",BTreeMap::from([("id".into(),DataValue::Str(a.clone().into()))]),ScriptMutability::Immutable).unwrap();
        let actual = super::relations(&store, &a, &ApiOptions::local())
            .unwrap()
            .into_iter()
            .map(|e| {
                json!([
                    e.from,
                    e.to,
                    e.edge_type,
                    e.weight,
                    e.properties,
                    e.dst_store
                ])
            })
            .collect::<Vec<_>>();
        let expected = rows
            .rows
            .into_iter()
            .map(|row| Value::Array(row.into_iter().map(Value::from).collect()))
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
    };
    let put = |src: &str,
               dst: &str,
               kind: &str,
               time: i64,
               asserted: bool,
               weight: f64,
               scope: &str| {
        store.db_ref().run_script(&format!("?[src,dst,edge_type,validity,weight,properties,dst_store] <- [[$src,$dst,$kind,[{time}.0,{asserted}],$weight,$props,$scope]] :put edge {{src,dst,edge_type,validity=>weight,properties,dst_store}}"),BTreeMap::from([("src".into(),DataValue::Str(src.into())),("dst".into(),DataValue::Str(dst.into())),("kind".into(),DataValue::Str(kind.into())),("weight".into(),DataValue::from(weight)),("props".into(),DataValue::Json(JsonData(json!({"version":time})))),("scope".into(),DataValue::Str(scope.into()))]),ScriptMutability::Mutable).unwrap();
    };
    compare();
    put(&a, &a, "refers_to", 10, true, 0.25, "local");
    compare();
    put(&b, &a, "causal", 10, true, 0.5, "local");
    put(&a, &b, "derived_from", 10, true, 0.75, "local");
    compare();
    put(&b, &a, "causal", 11, true, 0.8, "remote");
    compare();
    put(&a, &a, "refers_to", 11, false, 0.0, "local");
    compare();
    put(&a, &a, "refers_to", 12, true, 1.0, "local");
    compare();
    put(
        &b,
        &a,
        "causal",
        chrono::Utc::now().timestamp_micros() + 60_000_000,
        true,
        0.99,
        "future",
    );
    compare();
}
