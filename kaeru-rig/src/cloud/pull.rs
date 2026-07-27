//! Discovery and recall: `cloud_recall` lists what the cloud holds for an
//! initiative, `pull` materialises one of those nodes into the local vault and
//! rebuilds whatever edges both of whose endpoints are now local.

use std::str::FromStr;

use kaeru_core::{EdgeType, Layer, NodeType, Tier, Visibility};
use serde::Deserialize;
use serde_json::{Value, json};

use super::cloud_or_err;
use crate::cloud_client::CloudClient;
use crate::{KaeruMemory, mem_tool_cloud, target_initiative};

// ─────────────────────────────────────────────────────────────────────────────
// cloud_recall (network) — list the cloud's shared nodes for an initiative.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CloudRecallArgs {
    #[serde(default)]
    pub initiative: Option<String>,
    #[serde(default)]
    pub cloud: Option<String>,
}

async fn do_cloud_recall(mem: &KaeruMemory, a: CloudRecallArgs) -> Value {
    let Some(init) = target_initiative(mem, &a.initiative) else {
        return json!({ "error": "no initiative — scope the memory or pass `initiative`" });
    };
    let client = match cloud_or_err(mem, a.cloud.as_deref()) {
        Ok(c) => c,
        Err(v) => return v,
    };

    let (code, resp) = match client.list_initiative(&init).await {
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
    #[serde(default)]
    pub initiative: Option<String>,
    #[serde(default)]
    pub cloud: Option<String>,
}

async fn do_pull(mem: &KaeruMemory, a: PullArgs) -> Value {
    let Some(init) = target_initiative(mem, &a.initiative) else {
        return json!({ "error": "no initiative — scope the memory or pass `initiative`" });
    };
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

    let id = a.id.clone();
    let init_w = init.clone();
    let name_w = name.clone();
    let up = mem
        .blocking(move |s| {
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
            )
        })
        .await;
    if let Err(e) = up {
        return json!({ "error": e.to_string() });
    }

    let edges = recreate_local_edges(mem, &client, init.clone()).await;
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

/// Fetches the initiative's cloud edges and `link`s locally every one whose
/// both endpoints already exist locally. Idempotent at NOW.
async fn recreate_local_edges(
    mem: &KaeruMemory,
    client: &CloudClient,
    initiative: String,
) -> usize {
    let (code, resp) = match client.list_edges(&initiative).await {
        Ok(x) => x,
        Err(_) => return 0,
    };
    if !(200..300).contains(&code) {
        return 0;
    }
    let items: Vec<Value> = serde_json::from_str::<Value>(&resp)
        .ok()
        .and_then(|v| v.as_array().cloned())
        .unwrap_or_default();
    let parsed: Vec<(String, String, String, f64)> = items
        .iter()
        .filter_map(|it| {
            let src = it.get("src")?.as_str()?.to_string();
            let dst = it.get("dst")?.as_str()?.to_string();
            let et = it.get("edge_type")?.as_str()?.to_string();
            let weight = it.get("weight").and_then(|x| x.as_f64()).unwrap_or(1.0);
            Some((src, dst, et, weight))
        })
        .collect();
    if parsed.is_empty() {
        return 0;
    }

    mem.blocking_in(Some(initiative), move |s| {
        let mut linked = 0;
        for (src, dst, et, weight) in &parsed {
            let both_local = kaeru_core::node_brief_by_id(s, src)
                .ok()
                .flatten()
                .is_some()
                && kaeru_core::node_brief_by_id(s, dst)
                    .ok()
                    .flatten()
                    .is_some();
            if !both_local {
                continue;
            }
            let Ok(edge) = et.parse::<EdgeType>() else {
                continue;
            };
            if kaeru_core::link_with_weight(s, src, dst, edge, *weight).is_ok() {
                linked += 1;
            }
        }
        linked
    })
    .await
}

mem_tool_cloud!(
    /// `kaeru_cloud_recall` — list what the cloud holds for an initiative.
    CloudRecall,
    "kaeru_cloud_recall",
    "List the shared nodes the cloud holds for an initiative — discovery for cross-session / \
     cross-user recall. Then kaeru_pull one to bring it local. `cloud` targets a named cloud.",
    CloudRecallArgs,
    { "type": "object", "properties": {
        "initiative": { "type": "string", "description": "initiative (default: the memory's own)" },
        "cloud": { "type": "string", "description": "named cloud (default: the configured default)" }
    } },
    |mem, a| do_cloud_recall(mem, a).await
);

mem_tool_cloud!(
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
    }, "required": ["id"] },
    |mem, a| do_pull(mem, a).await
);
