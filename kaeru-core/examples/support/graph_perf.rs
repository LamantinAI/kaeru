//! Whole-graph API simulation, including response serialization, without HTTP/GL.
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, BTreeSet};
use std::hash::{Hash, Hasher};
use std::time::Instant;

use kaeru_core::Store;
use kaeru_core::api::{ApiOptions, ResponseData, execute};
use serde_json::{Value, json, to_vec};

type Nodes = BTreeMap<String, Value>;
type Edges = BTreeSet<String>;

fn pages(
    store: &Store,
    operation: &str,
    mut args: Value,
    calls: &mut usize,
    bytes: &mut usize,
) -> Vec<ResponseData> {
    let mut data = vec![];
    loop {
        let response = execute(store, operation, args, &ApiOptions::local());
        assert!(!response.is_error(), "{response:?}");
        *calls += 1;
        *bytes += to_vec(&response).unwrap().len();
        data.push(response.data);
        if let Some(next) = response.continuation {
            args = next.next_arguments;
        } else {
            break;
        }
    }
    data
}

fn legacy(store: &Store, calls: &mut usize, bytes: &mut usize) -> (Nodes, Edges) {
    let mut nodes = Nodes::new();
    let mut edges = Edges::new();
    for page in pages(store, "initiatives", json!({}), calls, bytes) {
        let ResponseData::Initiatives(data) = page else {
            unreachable!()
        };
        for initiative in data.initiatives {
            for page in pages(
                store,
                "context",
                json!({"initiative":initiative,"view":"map","budget_chars":64000}),
                calls,
                bytes,
            ) {
                let ResponseData::Context(data) = page else {
                    unreachable!()
                };
                for n in data.sections.into_iter().flat_map(|s| s.records) {
                    nodes.insert(
                        n.id.clone(),
                        json!([
                            n.id,
                            n.name,
                            n.node_type,
                            n.tier,
                            n.layer,
                            n.tags,
                            n.initiatives,
                            n.created_at
                        ]),
                    );
                }
            }
        }
    }
    for (id, node) in &nodes {
        let initiative = node[6][0].as_str().unwrap();
        for page in pages(
            store,
            "read",
            json!({"initiative":initiative,"node":{"id":id},"include":["relations","reasoning"],"budget_chars":64000}),
            calls,
            bytes,
        ) {
            let ResponseData::Record(data) = page else {
                unreachable!()
            };
            for e in data.relations {
                if e.dst_store == "local"
                    && nodes.contains_key(&e.from)
                    && nodes.contains_key(&e.to)
                {
                    edges.insert(json!([e.from, e.to, e.edge_type, e.weight]).to_string());
                }
            }
        }
    }
    (nodes, edges)
}

fn compact(store: &Store, calls: &mut usize, bytes: &mut usize) -> (Nodes, Edges) {
    let mut nodes = Nodes::new();
    let mut edges = Edges::new();
    for page in pages(
        store,
        "context",
        json!({"view":"graph","include":["created_at"],"budget_chars":64000}),
        calls,
        bytes,
    ) {
        let ResponseData::Graph(data) = page else {
            unreachable!()
        };
        for n in data.graph.nodes {
            nodes.insert(
                n.id.clone(),
                json!([
                    n.id,
                    n.name,
                    n.node_type,
                    n.tier,
                    n.layer,
                    n.tags,
                    n.initiatives,
                    n.created_at
                ]),
            );
        }
        for e in data.graph.edges {
            edges.insert(json!([e.from, e.to, e.edge_type, e.weight]).to_string());
        }
    }
    (nodes, edges)
}

pub fn run(
    store: &Store,
    n: u32,
    repeats: u32,
    backend: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for repeat in 0..repeats {
        let mut reference = None;
        for (name, run) in [
            (
                "legacy",
                legacy as fn(&Store, &mut usize, &mut usize) -> (Nodes, Edges),
            ),
            ("compact", compact),
        ] {
            let (mut calls, mut bytes) = (0, 0);
            let start = Instant::now();
            let (nodes, edges) = run(store, &mut calls, &mut bytes);
            let elapsed = start.elapsed().as_secs_f64() * 1000.0;
            let fingerprint = to_vec(&json!([nodes, edges])).unwrap();
            let mut hash = DefaultHasher::new();
            fingerprint.hash(&mut hash);
            if let Some(old) = &reference {
                assert_eq!(old, &fingerprint, "whole graph structural equivalence");
            } else {
                reference = Some(fingerprint);
            }
            println!(
                "{}",
                json!({"pipeline":name,"nodes":n,"returned_nodes":nodes.len(),"returned_edges":edges.len(),"backend":backend,"repeat":repeat,"elapsed_ms":elapsed,"api_calls":calls,"response_bytes":bytes,"fingerprint":format!("{:016x}",hash.finish()),"profile":if cfg!(debug_assertions){"debug"}else{"release"}})
            );
        }
    }
    Ok(())
}
