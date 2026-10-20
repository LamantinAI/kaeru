//! Discovery and recall: `cloud_recall` lists what the cloud holds for an
//! initiative, `pull` materialises one of those nodes into the local vault and
//! rebuilds whatever edges both of whose endpoints are now local.

use std::str::FromStr;

use kaeru_core::{CloudEdge, Layer, NodeType, Tier, Visibility, restore_cloud_edges};
use serde::Deserialize;
use serde_json::{Value, from_str, json};

use super::cloud_or_err;
use crate::KaeruMemory;
use crate::cloud_client::CloudClient;

// ─────────────────────────────────────────────────────────────────────────────
// cloud_recall (network) — list the cloud's shared nodes for an initiative.
// ─────────────────────────────────────────────────────────────────────────────

use kaeru_core::models::{CloudPullInput as PullArgs, CloudRecallInput as CloudRecallArgs};

pub(super) async fn do_cloud_recall(mem: &KaeruMemory, a: CloudRecallArgs) -> Value {
    // The verb names its initiative, so there is nothing to fall back to.
    let init = a.initiative.clone();
    let client = match cloud_or_err(mem, a.cloud.as_deref()) {
        Ok(c) => c,
        Err(v) => return v,
    };

    if let Some(id) = a.id.as_deref() {
        let (code, resp) = match client.get_node_in_initiative(&init, id).await {
            Ok(x) => x,
            Err(e) => return json!({ "error": format!("cloud GET failed: {e}") }),
        };
        if code == 404 {
            return json!({ "message": format!("cloud has no node {id}") });
        }
        if !(200..300).contains(&code) {
            return json!({ "error": format!("cloud GET failed ({code}): {resp}") });
        }
        let v: Value = match serde_json::from_str(&resp) {
            Ok(val) => val,
            Err(e) => return json!({ "error": format!("bad cloud response: {e}") }),
        };
        mem.store
            .record_read_trace("cloud_recall", vec![id.into()], vec![init.clone()]);
        return json!({ "initiative": init, "node": v });
    }

    let (code, resp) = match client
        .list_initiative(
            &init,
            a.limit.unwrap_or(50),
            a.offset.unwrap_or(0),
            a.query.as_deref(),
        )
        .await
    {
        Ok(x) => x,
        Err(e) => return json!({ "error": format!("cloud list failed: {e}") }),
    };
    if !(200..300).contains(&code) {
        return json!({ "error": format!("cloud list failed ({code}): {resp}") });
    }
    let arr: Value = match from_str(&resp) {
        Ok(v) => v,
        Err(e) => return json!({ "error": format!("bad cloud response: {e}") }),
    };
    let items = arr.as_array().cloned().unwrap_or_default();
    mem.store.record_read_trace(
        "cloud_recall",
        items
            .iter()
            .filter_map(|n| n.get("id")?.as_str().map(str::to_owned))
            .collect(),
        vec![init.clone()],
    );
    json!({ "initiative": init, "count": items.len(), "nodes": items })
}

// ─────────────────────────────────────────────────────────────────────────────
// pull (network) — materialise a cloud node locally, then relink its edges.
// ─────────────────────────────────────────────────────────────────────────────

pub(super) async fn do_pull(mem: &KaeruMemory, a: PullArgs) -> Value {
    // The verb names its initiative, so there is nothing to fall back to.
    let init = a.initiative.clone();
    let client = match cloud_or_err(mem, a.cloud.as_deref()) {
        Ok(c) => c,
        Err(v) => return v,
    };

    let (code, resp) = match client.get_node(&a.id).await {
        Ok(x) => x,
        Err(e) => return json!({ "error": format!("cloud GET failed: {e}") }),
    };
    if code == 404 {
        return json!({ "message": format!("cloud has no node {}", a.id) });
    }
    if !(200..300).contains(&code) {
        return json!({ "error": format!("cloud GET failed ({code}): {resp}") });
    }
    let v: Value = match serde_json::from_str(&resp) {
        Ok(v) => v,
        Err(e) => return json!({ "error": format!("bad cloud response: {e}") }),
    };

    let node_type_s = v.get("node_type").and_then(|x| x.as_str()).unwrap_or("");
    let tier_s = v.get("tier").and_then(|x| x.as_str()).unwrap_or("");
    let name = v
        .get("name")
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string();
    let body = v.get("body").and_then(|x| x.as_str()).map(String::from);
    let tags: Vec<String> = v
        .get("tags")
        .and_then(|x| x.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|t| t.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    let layer = v
        .get("layer")
        .and_then(|x| x.as_str())
        .map(|s| Layer::from_str(s).unwrap_or_default())
        .unwrap_or_default();
    let node_type = match NodeType::from_str(node_type_s) {
        Ok(t) => t,
        Err(e) => return json!({ "error": e.to_string() }),
    };
    let tier = match Tier::from_str(tier_s) {
        Ok(t) => t,
        Err(e) => return json!({ "error": e.to_string() }),
    };

    // A cloud too old to send `properties` sends nothing, and `None` tells
    // `upsert_node` to keep whatever is local rather than clear it — pulling a
    // `cite` to refresh it used to destroy the URL it was cited for (#85).
    let properties = v.get("properties").filter(|p| !p.is_null()).cloned();

    let id = a.id.clone();
    let init_w = init.clone();
    let name_w = name.clone();
    let up = mem
        .blocking_in(Some(init.clone()), move |s| {
            kaeru_core::upsert_node(
                s,
                &id,
                node_type,
                tier,
                &name_w,
                body.as_deref(),
                &tags,
                Some(&init_w),
                Visibility::Shared,
                layer,
                properties.as_ref(),
            )
        })
        .await;
    if let Err(e) = up {
        return json!({ "error": e.to_string() });
    }

    let edges = match recreate_local_edges(mem, &client, init.clone(), a.id.clone()).await {
        Ok(n) => n,
        Err(e) => {
            return json!({"error": format!("node imported, but edge restore failed: {e}"), "node_imported": true});
        }
    };
    let edge_note = if edges > 0 {
        format!(" (+{edges} edge(s) linked)")
    } else {
        String::new()
    };
    json!({
        "message": format!(
            "pulled `{name}` from cloud into local initiative `{init}` (id {}){edge_note}",
            a.id
        )
    })
}

/// Restore just the pulled node's edges; never hide an incomplete import.
async fn recreate_local_edges(
    mem: &KaeruMemory,
    client: &CloudClient,
    initiative: String,
    target: String,
) -> Result<usize, String> {
    let (code, resp) = client.list_edges(&initiative).await?;
    if !(200..300).contains(&code) {
        return Err(format!(
            "cloud `{}` edge restore failed ({code})",
            client.name()
        ));
    }
    let items: Vec<Value> =
        serde_json::from_str(&resp).map_err(|e| format!("invalid cloud edge list: {e}"))?;
    let edges: Vec<CloudEdge> = items
        .iter()
        .filter_map(|v| {
            Some((
                v.get("src")?.as_str()?.into(),
                v.get("dst")?.as_str()?.into(),
                v.get("edge_type")?.as_str()?.into(),
                v.get("weight").and_then(Value::as_f64).unwrap_or(1.0),
            ))
        })
        .collect();
    // This primitive scopes each mutation itself. Do not hold the outer
    // blocking_in guard and then re-lock it inside restore_cloud_edges.
    let store = mem.store.clone();
    tokio::task::spawn_blocking(move || restore_cloud_edges(&store, &initiative, &target, &edges))
        .await
        .map_err(|e| format!("edge import task failed: {e}"))?
        .map(|r| r.linked)
        .map_err(|e| e.to_string())
}

#[derive(Debug, Deserialize)]
pub struct CloudInitiativesArgs {
    /// Which cloud to ask, when several are configured.
    #[serde(default)]
    pub cloud: Option<String>,
}

pub(super) async fn do_cloud_initiatives(mem: &KaeruMemory, a: CloudInitiativesArgs) -> Value {
    let client = match mem.cloud_registry().resolve(a.cloud.as_deref()) {
        Ok(c) => c,
        Err(why) => return json!({ "error": why }),
    };
    match client.list_initiatives().await {
        Ok((code, body)) if (200..300).contains(&code) => {
            let items = from_str::<Value>(&body)
                .ok()
                .and_then(|v| v.as_array().cloned())
                .unwrap_or_default();
            mem.store.record_read_trace(
                "cloud_initiatives",
                vec![],
                items
                    .iter()
                    .filter_map(|n| n.get("name")?.as_str().map(str::to_owned))
                    .collect(),
            );
            json!({
                "cloud": client.name(),
                "initiatives": items
                    .iter()
                    .map(|it| json!({
                        "name": it.get("name").and_then(|x| x.as_str()).unwrap_or(""),
                        "nodes": it.get("nodes").and_then(|x| x.as_u64()).unwrap_or(0),
                    }))
                    .collect::<Vec<_>>(),
            })
        }
        Ok((code, body)) => json!({ "cloud": client.name(), "status": code, "error": body }),
        Err(e) => json!({ "cloud": client.name(), "error": e }),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use axum::http::StatusCode;
    use axum::{Json, Router, routing::get};
    use kaeru_core::{Store, read_node_full};
    use serde_json::json;
    use tokio::net::TcpListener;

    use super::{PullArgs, do_pull};
    use crate::KaeruMemory;
    use crate::cloud_client::{CloudClient, CloudRegistry};

    #[tokio::test]
    async fn full_recall_is_read_only_scoped_and_pull_reports_partial_import() {
        let app = Router::new()
            .route("/api/v1/nodes/node-a", get(|| async { Json(json!({
                "id":"node-a", "name":"full-note", "node_type":"reference", "tier":"archival",
                "layer":"hot", "body":"complete body", "tags":[]
            })) }))
            .route("/api/v1/initiatives/alpha/nodes", get(|| async { Json(json!([{"id":"node-a"}])) }))
            .route("/api/v1/initiatives/beta/nodes", get(|| async { Json(json!([])) }))
            .route("/api/v1/initiatives/alpha/edges", get(|| async { StatusCode::SERVICE_UNAVAILABLE }));
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let cloud = CloudClient::new("test".into(), format!("http://{addr}"), String::new());
        let directory = std::env::temp_dir().join(format!(
            "kaeru-rig-cloud-read-{}",
            kaeru_core::new_node_id()
        ));
        let store = Arc::new(
            Store::open_in_memory_with(kaeru_core::KaeruConfig {
                read_trace_enabled: true,
                read_trace_dir: directory.join("trace"),
                vault_path: directory.join("vault"),
                ..kaeru_core::KaeruConfig::defaults()
            })
            .unwrap(),
        );
        let mem = KaeruMemory::with_clouds(
            store.clone(),
            "beta",
            CloudRegistry::new(HashMap::from([("test".into(), cloud)]), None),
        );
        for (initiative, found) in [("alpha", true), ("beta", false)] {
            use rig::tool::Tool;
            let out = mem
                .cloud_recall()
                .call(json!({"initiative":initiative,"id":"node-a"}))
                .await
                .unwrap();
            assert_eq!(out.to_string().contains("complete body"), found);
            assert!(read_node_full(&store, &"node-a".into()).unwrap().is_none());
        }
        let out = do_pull(
            &mem,
            PullArgs {
                initiative: "alpha".into(),
                id: "node-a".into(),
                cloud: None,
            },
        )
        .await;
        assert_eq!(out["node_imported"], true);
        assert!(out["error"].as_str().unwrap().contains("node imported"));
        assert!(read_node_full(&store, &"node-a".into()).unwrap().is_some());
        assert!(store.shutdown_read_trace(std::time::Duration::from_secs(3)));
        let events: Vec<kaeru_core::models::ReadTraceEvent> =
            std::fs::read_dir(directory.join("trace"))
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| path.extension().is_some_and(|e| e == "jsonl"))
                .flat_map(|path| {
                    std::fs::read_to_string(path)
                        .unwrap()
                        .lines()
                        .map(|line| serde_json::from_str(line).unwrap())
                        .collect::<Vec<_>>()
                })
                .collect();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].operation, "cloud_recall");
        assert_eq!(events[0].node_ids, vec!["node-a"]);
        assert_eq!(events[0].initiatives, vec!["alpha"]);
        task.abort();
        std::fs::remove_dir_all(directory).unwrap();
    }
}
