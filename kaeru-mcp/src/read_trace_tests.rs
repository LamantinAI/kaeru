use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use std::time::Duration;

use kaeru_core::{
    EdgeType, KaeruConfig, Store, api, link_remote_to, models::ReadTraceEvent, new_node_id,
};
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const REMOTE: &str = "00000000-0000-0000-0000-000000000001";
const MISSING: &str = "00000000-0000-0000-0000-000000000002";

async fn mock_cloud() -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        while let Ok((mut socket, _)) = listener.accept().await {
            tokio::spawn(async move {
                let mut request = Vec::new();
                let mut buffer = [0; 512];
                while request.len() < 8192 && !request.windows(4).any(|w| w == b"\r\n\r\n") {
                    let n = socket.read(&mut buffer).await.unwrap_or(0);
                    if n == 0 {
                        return;
                    }
                    request.extend_from_slice(&buffer[..n]);
                }
                let request = String::from_utf8_lossy(&request);
                let path = request.split_whitespace().nth(1).unwrap_or("");
                let node = json!({"id":REMOTE,"node_type":"episode","tier":"operational","name":"cloud-record","body":"never-log-cloud-content","body_excerpt":"never-log-cloud-content","tags":[],"layer":"warm"});
                let (status, body) = if path.starts_with("/api/v1/initiatives/work/nodes") {
                    if path.contains("q=failed") {
                        (503, json!({"error":"unavailable"}))
                    } else if path.contains("q=missing") {
                        (200, json!([]))
                    } else {
                        (200, json!([node]))
                    }
                } else if path == "/api/v1/initiatives" {
                    (200, json!([{"name":"work","nodes":2}]))
                } else if path == format!("/api/v1/nodes/{REMOTE}") {
                    (200, node)
                } else if path == "/api/v1/initiatives/work/edges" {
                    (200, json!([]))
                } else {
                    (404, json!({"error":"not found"}))
                };
                let body = body.to_string();
                let response = format!(
                    "HTTP/1.1 {status} response\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                let _ = socket.write_all(response.as_bytes()).await;
            });
        }
    });
    (address, task)
}

fn fixture() -> (Arc<Store>, std::path::PathBuf, String) {
    let directory = std::env::temp_dir().join(format!("kaeru-adapter-trace-{}", new_node_id()));
    let cfg = KaeruConfig {
        vault_path: directory.join("vault"),
        read_trace_dir: directory.join("trace"),
        read_trace_enabled: true,
        ..KaeruConfig::defaults()
    };
    let store = Arc::new(Store::open_in_memory_with(cfg).unwrap());
    let created = api::execute(
        &store,
        "note",
        json!({"initiative":"work","name":"local-source","body":"local data"}),
        &api::ApiOptions::local(),
    );
    let api::ResponseData::Mutation(data) = created.data else {
        panic!("mutation expected")
    };
    let id = data.primary.unwrap().id;
    link_remote_to(
        &store,
        &id,
        &REMOTE.into(),
        EdgeType::RefersTo,
        Some("team"),
    )
    .unwrap();
    link_remote_to(
        &store,
        &id,
        &MISSING.into(),
        EdgeType::RefersTo,
        Some("team"),
    )
    .unwrap();
    (store, directory, id)
}

fn verify(store: &Store, directory: &std::path::Path, local: &str) {
    assert!(store.shutdown_read_trace(Duration::from_secs(3)));
    let mut events = Vec::<ReadTraceEvent>::new();
    for entry in fs::read_dir(directory.join("trace")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|ext| ext == "jsonl") {
            for line in fs::read_to_string(path).unwrap().lines() {
                events.push(serde_json::from_str(line).unwrap());
            }
        }
    }
    assert_eq!(
        events.len(),
        7,
        "no duplicate events, no auxiliary fetches or pull receipts: {events:?}"
    );
    let recalls: Vec<_> = events
        .iter()
        .filter(|e| e.operation == "cloud_recall")
        .collect();
    assert_eq!(recalls.len(), 2);
    assert_eq!(recalls[0].node_ids, vec![REMOTE]);
    assert!(recalls[1].node_ids.is_empty());
    assert_eq!(
        events
            .iter()
            .find(|e| e.operation == "cloud_links")
            .unwrap()
            .node_ids,
        vec![REMOTE]
    );
    assert_eq!(
        events
            .iter()
            .find(|e| e.operation == "read")
            .unwrap()
            .node_ids,
        vec![local]
    );
    assert_eq!(
        events
            .iter()
            .find(|e| e.operation == "sync_review")
            .unwrap()
            .node_ids,
        vec![local]
    );
    let listing = events
        .iter()
        .find(|e| e.operation == "cloud_initiatives")
        .unwrap();
    assert!(listing.node_ids.is_empty());
    assert_eq!(listing.initiatives, vec!["work"]);
    let encoded = serde_json::to_string(&events).unwrap();
    assert!(!encoded.contains("never-log-cloud-content"));
    assert!(!encoded.contains(MISSING));
    fs::remove_dir_all(directory).unwrap();
}

use crate::canonical;
use crate::cloud_client::{CloudClient, CloudRegistry};

#[tokio::test]
async fn mcp_local_and_cloud_reads_produce_one_metadata_event_per_delivered_result() {
    let (address, server) = mock_cloud().await;
    let (store, directory, local) = fixture();
    let clouds = CloudRegistry::new(
        HashMap::from([(
            "team".into(),
            CloudClient::new("team".into(), address, String::new()),
        )]),
        Some("team".into()),
    );
    for (name, args) in [
        ("clouds", json!({})),
        ("read", json!({"initiative":"work","node":local})),
        (
            "cloud_recall",
            json!({"initiative":"work","cloud":"team","limit":1}),
        ),
        (
            "cloud_recall",
            json!({"initiative":"work","cloud":"team","query":"failed"}),
        ),
        (
            "cloud_recall",
            json!({"initiative":"work","cloud":"team","query":"missing"}),
        ),
        ("cloud_links", json!({"initiative":"work","name":local})),
        ("pull", json!({"initiative":"work","id":REMOTE})),
        ("cloud_initiatives", json!({})),
        ("policy", json!({"initiative":"work","policy":"team"})),
        ("sync_review", json!({"initiative":"work"})),
    ] {
        canonical::call(store.clone(), &clouds, name, args)
            .await
            .unwrap();
    }
    verify(&store, &directory, &local);
    server.abort();
}
