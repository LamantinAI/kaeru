//! Discovery and recall: `cloud_recall` lists what the cloud holds for an
//! initiative, `pull` materialises one of those nodes into the local vault and
//! rebuilds whatever edges both of whose endpoints are now local.

use std::str::FromStr;

use kaeru_core::{CloudEdge, Layer, NodeType, Tier, Visibility, restore_cloud_edges};
use serde::Deserialize;
use serde_json::{Value, json};

use super::cloud_or_err;
use crate::cloud_client::CloudClient;
use crate::{KaeruMemory, mem_tool_cloud_named};

// ─────────────────────────────────────────────────────────────────────────────
// cloud_recall (network) — list the cloud's shared nodes for an initiative.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CloudRecallArgs {
    pub initiative: String,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub cloud: Option<String>,
    /// Page size. Bounded server-side; an unpaged listing on a real corpus
    /// did not fit in the caller's context at all (#67).
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub offset: Option<usize>,
    /// Case-insensitive substring over name and excerpt — the cloud had no
    /// way to be *searched* at all, only listed and fetched by id (#57).
    #[serde(default)]
    pub query: Option<String>,
}

async fn do_cloud_recall(mem: &KaeruMemory, a: CloudRecallArgs) -> Value {
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
    let arr: Value = match serde_json::from_str(&resp) {
        Ok(v) => v,
        Err(e) => return json!({ "error": format!("bad cloud response: {e}") }),
    };
    let items = arr.as_array().cloned().unwrap_or_default();
    json!({ "initiative": init, "count": items.len(), "nodes": items })
}

// ─────────────────────────────────────────────────────────────────────────────
// pull (network) — materialise a cloud node locally, then relink its edges.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PullArgs {
    pub id: String,
    pub initiative: String,
    #[serde(default)]
    pub cloud: Option<String>,
}

async fn do_pull(mem: &KaeruMemory, a: PullArgs) -> Value {
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

mem_tool_cloud_named!(
    /// `kaeru_cloud_recall` — list what the cloud holds for an initiative.
    CloudRecall,
    "kaeru_cloud_recall",
    "Read a full shared node without importing it with `id`, or list nodes for an initiative — discovery for cross-session / \
     cross-user recall. Then kaeru_pull one to bring it local. `cloud` targets a named cloud.",
    CloudRecallArgs,
    { "type": "object", "properties": {
        "initiative": { "type": "string", "description": "initiative (project) to read in the cloud" },
        "id": { "type": "string", "description": "optional node ID for direct read-only retrieval without importing" },
        "cloud": { "type": "string", "description": "named cloud (default: the configured default)" },
        "query": { "type": "string", "description": "optional filter; matches name and body cloud-side" },
        "limit": { "type": "integer", "description": "page size (default 50)" },
        "offset": { "type": "integer", "description": "how many to skip, for the next page" }
    }, "required": ["initiative"] },
    |mem, a| do_cloud_recall(mem, a).await
);

mem_tool_cloud_named!(
    /// `kaeru_pull` — materialise a cloud node into the local vault.
    Pull,
    "kaeru_pull",
    "Pull a shared node from the cloud into the local vault by id, attaching it to the given \
     initiative — the recall mechanism for team knowledge you don't have locally yet. `cloud` \
     targets a named cloud.",
    PullArgs,
    { "type": "object", "properties": {
        "id": { "type": "string", "description": "the cloud node's id (from kaeru_cloud_recall)" },
        "initiative": { "type": "string", "description": "initiative to attach it to (default: the memory's own)" },
        "cloud": { "type": "string", "description": "named cloud (default: the configured default)" }
    }, "required": ["id", "initiative"] },
    |mem, a| do_pull(mem, a).await
);

#[derive(Debug, Deserialize)]
pub struct CloudInitiativesArgs {
    /// Which cloud to ask, when several are configured.
    #[serde(default)]
    pub cloud: Option<String>,
}

crate::mem_tool_unscoped!(
    /// `kaeru_cloud_initiatives` — what a cloud holds, by initiative.
    CloudInitiatives,
    "kaeru_cloud_initiatives",
    "List the initiatives a cloud holds, with how many shared nodes each has. Ask this before \
     `kaeru_cloud_recall` in an unfamiliar cloud: it tells an initiative that is empty here \
     apart from one this cloud has never heard of, which a recall alone cannot.",
    CloudInitiativesArgs,
    { "type": "object", "properties": {
        "cloud": { "type": "string", "description": "named cloud (default: the configured default)" }
    } },
    |mem, a| {
        let client = match mem.clouds().resolve(a.cloud.as_deref()) {
            Ok(c) => c,
            Err(why) => return json!({ "error": why }),
        };
        match client.list_initiatives().await {
            Ok((code, body)) if (200..300).contains(&code) => {
                let items = serde_json::from_str::<Value>(&body)
                    .ok()
                    .and_then(|v| v.as_array().cloned())
                    .unwrap_or_default();
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
);

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use axum::http::StatusCode;
    use axum::{Json, Router, routing::get};
    use kaeru_core::{Store, read_node_full};
    use serde_json::json;
    use tokio::net::TcpListener;

    use super::{CloudRecallArgs, PullArgs, do_cloud_recall, do_pull};
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
        let store = Arc::new(Store::open_in_memory().unwrap());
        let mem = KaeruMemory::with_clouds(
            store.clone(),
            "beta",
            CloudRegistry::new(HashMap::from([("test".into(), cloud)]), None),
        );
        for (initiative, found) in [("alpha", true), ("beta", false)] {
            let out = do_cloud_recall(
                &mem,
                CloudRecallArgs {
                    initiative: initiative.into(),
                    id: Some("node-a".into()),
                    cloud: None,
                    limit: None,
                    offset: None,
                    query: None,
                },
            )
            .await;
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
        task.abort();
    }
}
