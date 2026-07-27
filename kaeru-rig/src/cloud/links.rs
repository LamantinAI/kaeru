//! Soft links to cloud nodes: `link_cloud` records a reference without copying,
//! `cloud_links` resolves those references lazily — each routed back to the
//! cloud it was created against (multi-cloud aware).

use kaeru_core::EdgeType;
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{KaeruMemory, mem_tool_cloud, resolve, target_initiative};

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
    #[serde(default)]
    pub initiative: Option<String>,
}

async fn do_link_cloud(mem: &KaeruMemory, a: LinkCloudArgs) -> Value {
    let Some(init) = target_initiative(mem, &a.initiative) else {
        return json!({ "error": "no initiative — scope the memory or pass `initiative`" });
    };
    // Refuse to bake a cloud name we can't resolve later — that would dangle.
    if let Some(cn) = a.cloud.as_deref()
        && !mem.clouds().contains(cn)
    {
        return json!({
            "error": format!("unknown cloud `{cn}` — configured: [{}]", mem.clouds().names().join(", "))
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
        let src = resolve(s, &name);
        match kaeru_core::link_remote_to(s, &src, &cloud_id, edge, cloud_name.as_deref()) {
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
    #[serde(default)]
    pub initiative: Option<String>,
}

async fn do_cloud_links(mem: &KaeruMemory, a: CloudLinksArgs) -> Value {
    let Some(init) = target_initiative(mem, &a.initiative) else {
        return json!({ "error": "no initiative — scope the memory or pass `initiative`" });
    };
    if mem.clouds().is_empty() {
        return json!({ "error": "cloud not configured" });
    }

    let name = a.name.clone();
    let links = mem
        .blocking_in(Some(init), move |s| {
            let id = resolve(s, &name);
            kaeru_core::cloud_links(s, &id)
        })
        .await;
    let links = match links {
        Ok(l) => l,
        Err(e) => return json!({ "error": e.to_string() }),
    };
    if links.is_empty() {
        return json!({ "name": a.name, "links": [] });
    }

    let mut out: Vec<Value> = Vec::with_capacity(links.len());
    for (edge_type, cloud_name, dst) in &links {
        let tag = cloud_name
            .as_deref()
            .map(|n| format!("cloud:{n}:{dst}"))
            .unwrap_or_else(|| format!("cloud:{dst}"));
        let Some(client) = mem.cloud(cloud_name.as_deref()) else {
            out.push(json!({
                "edge_type": edge_type, "target": tag, "resolved": false,
                "note": format!("cloud `{}` not configured", cloud_name.as_deref().unwrap_or("default")),
            }));
            continue;
        };
        match client.get_node(dst).await {
            Ok((200..=299, body)) => {
                let v: Value = serde_json::from_str(&body).unwrap_or(Value::Null);
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
    json!({ "name": a.name, "links": out })
}

mem_tool_cloud!(
    /// `kaeru_link_cloud` — soft-link a local node to a cloud node by id.
    LinkCloud,
    "kaeru_link_cloud",
    "Soft-link a local node to a cloud node by id — a reference without copying, resolved lazily \
     via kaeru_cloud_links. Edge type defaults to refers_to. `cloud` records which cloud the dst \
     lives in.",
    LinkCloudArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "local source node name or id" },
        "cloud_id": { "type": "string", "description": "the cloud node's id" },
        "edge_type": { "type": "string", "description": "link type (default refers_to)" },
        "cloud": { "type": "string", "description": "named cloud the dst lives in" },
        "initiative": { "type": "string", "description": "initiative (default: the memory's own)" }
    }, "required": ["name", "cloud_id"] },
    |mem, a| do_link_cloud(mem, a).await
);

mem_tool_cloud!(
    /// `kaeru_cloud_links` — resolve a node's cloud soft links.
    CloudLinks,
    "kaeru_cloud_links",
    "Resolve a node's cloud soft links — fetch and show the cloud nodes they point to. The lazy \
     resolution path for kaeru_link_cloud; each link routes to the cloud it was created against.",
    CloudLinksArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "local node name or id" },
        "initiative": { "type": "string", "description": "initiative (default: the memory's own)" }
    }, "required": ["name"] },
    |mem, a| do_cloud_links(mem, a).await
);
