//! Discovery and recall: `cloud_recall` lists what the cloud holds for an
//! initiative, `pull` materialises one of those nodes into the local vault and
//! rebuilds whatever edges both of whose endpoints are now local.

use std::str::FromStr;

use kaeru_core::{
    EdgeType, Layer, NodeType, Tier, Visibility, link_with_weight, node_brief_by_id, upsert_node,
};
use serde::Deserialize;
use serde_json::{Value, from_str, json};

use super::cloud_or_err;
use crate::KaeruMemory;
use crate::cloud_client::CloudClient;

// ─────────────────────────────────────────────────────────────────────────────
// cloud_recall (network) — list the cloud's shared nodes for an initiative.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CloudRecallArgs {
    pub initiative: String,
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

pub(super) async fn do_cloud_recall(mem: &KaeruMemory, a: CloudRecallArgs) -> Value {
    // The verb names its initiative, so there is nothing to fall back to.
    let init = a.initiative.clone();
    let client = match cloud_or_err(mem, a.cloud.as_deref()) {
        Ok(c) => c,
        Err(v) => return v,
    };

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
    let v: Value = match from_str(&resp) {
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
        .blocking(move |s| {
            upsert_node(
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
    let items: Vec<Value> = from_str::<Value>(&resp)
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
            let both_local = node_brief_by_id(s, src).ok().flatten().is_some()
                && node_brief_by_id(s, dst).ok().flatten().is_some();
            if !both_local {
                continue;
            }
            let Ok(edge) = et.parse::<EdgeType>() else {
                continue;
            };
            if link_with_weight(s, src, dst, edge, *weight).is_ok() {
                linked += 1;
            }
        }
        linked
    })
    .await
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
