//! Soft links to cloud nodes: `link_cloud` records a reference without copying,
//! `cloud_links` resolves those references lazily — each routed back to the
//! cloud it was created against (multi-cloud aware).

use kaeru_core::{EdgeType, Error, cloud_links, link_remote_to};
use serde::Deserialize;
use serde_json::{Value, from_str, json};

use crate::{KaeruMemory, resolve};

// ─────────────────────────────────────────────────────────────────────────────
// link_cloud (local) — soft-link a local node to a cloud node by id.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct LinkCloudArgs {
    pub name: String,
    pub cloud_id: String,
    #[serde(default)]
    pub edge_type: Option<String>,
    /// Which cloud the dst lives in (recorded as `dst_store = cloud:<name>`).
    #[serde(default)]
    pub cloud: Option<String>,
    pub initiative: String,
}

pub(super) async fn do_link_cloud(mem: &KaeruMemory, a: LinkCloudArgs) -> Value {
    // The verb names its initiative, so there is nothing to fall back to.
    let init = a.initiative.clone();
    // Refuse to bake a cloud name we can't resolve later — that would dangle.
    if let Some(cn) = a.cloud.as_deref()
        && !mem.cloud_registry().contains(cn)
    {
        return json!({
            "error": format!("unknown cloud `{cn}` — configured: [{}]", mem.cloud_registry().names().join(", "))
        });
    }

    let name = a.name.clone();
    let cloud_id = a.cloud_id.clone();
    let cloud_name = a.cloud.clone();
    let edge_type_s = a
        .edge_type
        .clone()
        .unwrap_or_else(|| "refers_to".to_string());

    mem.blocking_in(Some(init), move |s| {
        let edge = match edge_type_s.parse::<EdgeType>() {
            Ok(e) => e,
            Err(e) => return json!({ "error": e.to_string() }),
        };
        let src = match resolve(s, &name) {
            Ok(id) => id,
            Err(error) => return json!({"error":error}),
        };
        match link_remote_to(s, &src, &cloud_id, edge, cloud_name.as_deref()) {
            Ok(()) => {
                let tag = cloud_name
                    .as_deref()
                    .map(|n| format!("cloud:{n}:{cloud_id}"))
                    .unwrap_or_else(|| format!("cloud:{cloud_id}"));
                json!({ "message": format!("soft-linked `{name}` -[{}]-> {tag}", edge.as_str()) })
            }
            Err(e) => json!({ "error": e.to_string() }),
        }
    })
    .await
}

// ─────────────────────────────────────────────────────────────────────────────
// cloud_links (network) — resolve a node's cloud soft links.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CloudLinksArgs {
    pub name: String,
    pub initiative: String,
}

pub(super) async fn do_cloud_links(mem: &KaeruMemory, a: CloudLinksArgs) -> Value {
    // The verb names its initiative, so there is nothing to fall back to.
    let init = a.initiative.clone();
    if mem.cloud_registry().is_empty() {
        return json!({ "error": "cloud not configured" });
    }

    let name = a.name.clone();
    let links = mem
        .blocking_in(Some(init), move |s| {
            let id = resolve(s, &name).map_err(Error::Invalid)?;
            cloud_links(s, &id)
        })
        .await;
    let links = match links {
        Ok(l) => l,
        Err(e) => return json!({ "error": e.to_string() }),
    };
    if links.is_empty() {
        mem.store
            .record_read_trace("cloud_links", vec![], vec![a.initiative.clone()]);
        return json!({ "name": a.name, "links": [] });
    }

    let mut out: Vec<Value> = Vec::with_capacity(links.len());
    let mut delivered_ids = Vec::new();
    for (edge_type, cloud_name, dst) in &links {
        let tag = cloud_name
            .as_deref()
            .map(|n| format!("cloud:{n}:{dst}"))
            .unwrap_or_else(|| format!("cloud:{dst}"));
        let Ok(client) = mem.cloud_registry().resolve(cloud_name.as_deref()) else {
            out.push(json!({
                "edge_type": edge_type, "target": tag, "resolved": false,
                "note": format!("cloud `{}` not configured", cloud_name.as_deref().unwrap_or("default")),
            }));
            continue;
        };
        match client.get_node(dst).await {
            Ok((200..=299, body)) => {
                let v: Value = from_str(&body).unwrap_or(Value::Null);
                if v.get("name").and_then(Value::as_str).is_some() {
                    delivered_ids.push(dst.clone());
                }
                out.push(json!({
                    "edge_type": edge_type,
                    "target": tag,
                    "resolved": true,
                    "name": v.get("name").and_then(|x| x.as_str()).unwrap_or("?"),
                    "node_type": v.get("node_type").and_then(|x| x.as_str()).unwrap_or("?"),
                }));
            }
            Ok((code, _)) => out.push(json!({
                "edge_type": edge_type, "target": tag, "resolved": false, "status": code,
            })),
            Err(e) => out.push(json!({
                "edge_type": edge_type, "target": tag, "resolved": false, "error": e,
            })),
        }
    }
    mem.store
        .record_read_trace("cloud_links", delivered_ids, vec![a.initiative.clone()]);
    json!({ "name": a.name, "links": out })
}
