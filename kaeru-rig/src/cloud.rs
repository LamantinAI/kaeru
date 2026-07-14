//! Cloud sharing & recall tools — the in-process local↔cloud bridge.
//!
//! Mirror of the daemon's `kaeru-mcp/src/tools/cloud.rs`, adapted to rig:
//! bodies are `async` (they `.await` the [`CloudClient`]) and every local-store
//! span goes through `mem.blocking(...)` (a blocking thread), so a network
//! round-trip never stalls the async executor and a Cozo read never runs on it.
//!
//! - `policy`       — read / set an initiative's sticky `share_policy` (Gate 1). Local.
//! - `share`        — push a node to the team cloud through both gates.
//! - `cloud_recall` — list what the cloud holds for an initiative (discovery).
//! - `pull`         — materialise a cloud node into the local vault (the recall).
//! - `link_cloud`   — soft-link a local node to a cloud node by id. Local.
//! - `cloud_links`  — resolve a node's cloud soft links (lazy fetch).
//! - `sync_review`  — split still-local nodes into propose-share / keep-local. Local.
//!
//! Sharing is gated: the initiative must permit it (`SharePolicy::permits_share`)
//! and the node must clear the deterministic pre-share secret guard. Both gates
//! fail *safe* — a refusal is a message, never a silent push.

use std::str::FromStr;

use kaeru_core::{EdgeType, GuardHit, Layer, NodeType, SharePolicy, Tier, Visibility, guard};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::cloud_client::CloudClient;
use crate::{KaeruMemory, mem_tool_cloud, resolve};

/// The initiative a cloud call targets: an explicit arg, else the memory's own.
fn target_initiative(mem: &KaeruMemory, arg: &Option<String>) -> Option<String> {
    arg.clone().or_else(|| mem.initiative().map(String::from))
}

/// Resolves the target [`CloudClient`] (by explicit name, else the default), or
/// an error `Value` naming what's configured.
fn cloud_or_err(mem: &KaeruMemory, name: Option<&str>) -> Result<CloudClient, Value> {
    mem.cloud(name).cloned().ok_or_else(|| {
        let names = mem.clouds().names().join(", ");
        json!({
            "error": format!(
                "cloud not configured (configured: [{names}]) — pass a valid `cloud`, or set a default"
            )
        })
    })
}

/// A guard hit as `rule: "fragment"`. The fragment is `GuardHit.matched`, which
/// the guard already truncates for safe display — showing it is what lets a
/// human tell a real secret from a false positive without re-reading the body.
fn format_hit(h: &GuardHit) -> String {
    format!("{}: {:?}", h.rule, h.matched)
}

// ─────────────────────────────────────────────────────────────────────────────
// policy (local) — read / set an initiative's share_policy (Gate 1).
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PolicyArgs {
    #[serde(default)]
    pub initiative: Option<String>,
    /// Omit to read; `private` / `team` / `ask` to set.
    #[serde(default)]
    pub policy: Option<String>,
}

async fn do_policy(mem: &KaeruMemory, a: PolicyArgs) -> Value {
    let Some(init) = target_initiative(mem, &a.initiative) else {
        return json!({ "error": "no initiative — scope the memory or pass `initiative`" });
    };
    match a.policy {
        Some(p) => {
            let parsed = match SharePolicy::from_str(&p) {
                Ok(x) => x,
                Err(e) => return json!({ "error": format!("bad policy `{p}`: {e}") }),
            };
            let init2 = init.clone();
            let r = mem
                .blocking(move |s| kaeru_core::set_share_policy(s, &init2, parsed))
                .await;
            match r {
                Ok(()) => json!({ "initiative": init, "policy": parsed.as_str() }),
                Err(e) => json!({ "error": e.to_string() }),
            }
        }
        None => {
            let init2 = init.clone();
            let r = mem
                .blocking(move |s| kaeru_core::get_share_policy(s, &init2))
                .await;
            match r {
                Ok(cur) => json!({
                    "initiative": init,
                    "policy": cur.as_str(),
                    "permits_share": cur.permits_share(),
                }),
                Err(e) => json!({ "error": e.to_string() }),
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// share (network) — both gates, then push node + shareable edges.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ShareArgs {
    pub name: String,
    #[serde(default)]
    pub initiative: Option<String>,
    #[serde(default)]
    pub cloud: Option<String>,
    #[serde(default)]
    pub force: Option<bool>,
}

async fn do_share(mem: &KaeruMemory, a: ShareArgs) -> Value {
    let Some(init) = target_initiative(mem, &a.initiative) else {
        return json!({ "error": "no initiative — scope the memory or pass `initiative`" });
    };
    let client = match cloud_or_err(mem, a.cloud.as_deref()) {
        Ok(c) => c,
        Err(v) => return v,
    };
    let force = a.force.unwrap_or(false);

    // Resolve the name under the target initiative.
    let name = a.name.clone();
    let id = mem
        .blocking_in(Some(init.clone()), move |s| resolve(s, &name))
        .await;

    match share_node(mem, &client, id, init, force).await {
        Ok(msg) => json!({ "message": msg }),
        Err(e) => json!({ "error": e }),
    }
}

/// Runs both gates on `id` and, on pass, pushes it — marking the local copy
/// `shared` only **after** the cloud accepts, so `shared` always means "is in
/// the cloud". `Ok(message)` is the user-facing outcome (shared, or refused);
/// `Err(e)` is an infrastructure failure (store/network).
async fn share_node(
    mem: &KaeruMemory,
    client: &CloudClient,
    id: String,
    initiative: String,
    force: bool,
) -> Result<String, String> {
    // Gate 1 — initiative policy.
    let init1 = initiative.clone();
    let pol = mem
        .blocking(move |s| kaeru_core::get_share_policy(s, &init1))
        .await
        .map_err(|e| e.to_string())?;
    if !pol.permits_share() {
        return Ok(format!(
            "not shared: initiative `{initiative}` is `{}` — set policy=team to allow.",
            pol.as_str()
        ));
    }

    // Read the node.
    let id2 = id.clone();
    let full = match mem
        .blocking(move |s| kaeru_core::read_node_full(s, &id2))
        .await
        .map_err(|e| e.to_string())?
    {
        Some(f) => f,
        None => return Err(format!("node `{id}` not found at NOW")),
    };

    // Gate 2 — strict pre-share secret guard over name + body.
    let scan_target = format!("{}\n{}", full.name, full.body.clone().unwrap_or_default());
    let hits = guard::scan_public(&scan_target);
    if !hits.is_empty() && !force {
        let shown: Vec<String> = hits.iter().map(format_hit).collect();
        return Ok(format!(
            "not shared: pre-share guard flagged {} secret(s) [{}]. Remove them, or force=true.",
            hits.len(),
            shown.join("; ")
        ));
    }

    // Push the node.
    let body = json!({
        "id": full.id,
        "node_type": full.node_type,
        "tier": full.tier,
        "name": full.name,
        "body": full.body,
        "tags": full.tags,
        "initiative": initiative,
        "layer": full.layer,
    });
    let (code, resp) = client
        .post_node(&body)
        .await
        .map_err(|e| format!("cloud POST failed: {e}"))?;
    if !(200..300).contains(&code) {
        return Ok(format!("not shared: cloud rejected ({code}): {resp}"));
    }

    // Mark shared locally, then gather edges to already-shared neighbours.
    let fid = full.id.clone();
    mem.blocking(move |s| kaeru_core::set_visibility(s, &fid, Visibility::Shared))
        .await
        .map_err(|e| e.to_string())?;

    let fid2 = full.id.clone();
    let edge_bodies = mem
        .blocking(move |s| {
            let edges = kaeru_core::edges_of(s, &fid2)?;
            let mut out: Vec<Value> = Vec::new();
            for (src, dst, edge_type, weight) in &edges {
                let other = if *src == fid2 { dst } else { src };
                if kaeru_core::get_visibility(s, other)? == Visibility::Shared {
                    out.push(json!({
                        "src": src, "dst": dst, "edge_type": edge_type, "weight": weight
                    }));
                }
            }
            Ok::<_, kaeru_core::Error>(out)
        })
        .await
        .map_err(|e| e.to_string())?;

    let mut edges_pushed = 0;
    for ebody in &edge_bodies {
        if let Ok((ecode, _)) = client.post_edge(ebody).await
            && (200..300).contains(&ecode)
        {
            edges_pushed += 1;
        }
    }
    let edge_note = if edges_pushed > 0 {
        format!(" (+{edges_pushed} edge(s))")
    } else {
        String::new()
    };
    Ok(format!(
        "shared `{}` → cloud (id {}){edge_note}",
        full.name, full.id
    ))
}

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

// ─────────────────────────────────────────────────────────────────────────────
// sync_review (local) — split still-local nodes into propose-share / keep-local.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SyncReviewArgs {
    #[serde(default)]
    pub initiative: Option<String>,
}

async fn do_sync_review(mem: &KaeruMemory, a: SyncReviewArgs) -> Value {
    let Some(init) = target_initiative(mem, &a.initiative) else {
        return json!({ "error": "no initiative — scope the memory or pass `initiative`" });
    };
    let init2 = init.clone();
    mem.blocking(move |s| {
        let pol = match kaeru_core::get_share_policy(s, &init2) {
            Ok(p) => p,
            Err(e) => return json!({ "error": e.to_string() }),
        };
        if !pol.permits_share() {
            return json!({
                "message": format!(
                    "initiative `{init2}` is `{}` — nothing to sync; set policy=team first.",
                    pol.as_str()
                )
            });
        }
        let locals = match kaeru_core::local_nodes_for_review(s, &init2) {
            Ok(l) => l,
            Err(e) => return json!({ "error": e.to_string() }),
        };
        let mut propose: Vec<Value> = Vec::new();
        let mut keep: Vec<Value> = Vec::new();
        for n in &locals {
            let target = format!("{}\n{}", n.name, n.body.clone().unwrap_or_default());
            let hits = guard::scan_public(&target);
            if hits.is_empty() {
                propose.push(json!({ "name": n.name, "node_type": n.node_type, "id": n.id }));
            } else {
                keep.push(json!({
                    "name": n.name, "node_type": n.node_type, "id": n.id,
                    "flagged": hits.iter().map(format_hit).collect::<Vec<_>>(),
                }));
            }
        }
        json!({ "initiative": init2, "propose_share": propose, "keep_local": keep })
    })
    .await
}

// ─────────────────────────────────────────────────────────────────────────────
// Tool definitions.
// ─────────────────────────────────────────────────────────────────────────────

mem_tool_cloud!(
    /// `kaeru_policy` — read or set an initiative's cloud sharing policy.
    Policy,
    "kaeru_policy",
    "Read or set an initiative's cloud sharing policy (Gate 1). Omit `policy` to read. \
     Values: private (default — never leaves), team (shared nodes may sync), ask.",
    PolicyArgs,
    { "type": "object", "properties": {
        "initiative": { "type": "string", "description": "initiative (default: the memory's own)" },
        "policy": { "type": "string", "description": "private | team | ask (omit to read)" }
    } },
    |mem, a| do_policy(mem, a).await
);

mem_tool_cloud!(
    /// `kaeru_share` — push a node to the team cloud (both gates).
    Share,
    "kaeru_share",
    "Share a node to the team cloud. Gated: the initiative must be `team` (set via kaeru_policy) \
     AND the node must pass the pre-share secret guard. On success it's marked shared locally and \
     pushed under the same id. force=true overrides the guard. `cloud` targets a named cloud.",
    ShareArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "node name or id to share" },
        "initiative": { "type": "string", "description": "initiative (default: the memory's own)" },
        "cloud": { "type": "string", "description": "named cloud (default: the configured default)" },
        "force": { "type": "boolean", "description": "override the secret guard" }
    }, "required": ["name"] },
    |mem, a| do_share(mem, a).await
);

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

mem_tool_cloud!(
    /// `kaeru_sync_review` — split still-local nodes into propose / keep.
    SyncReview,
    "kaeru_sync_review",
    "Batch sync-review of a team initiative's still-local nodes: splits them into propose_share \
     (guard-clean) vs keep_local (secret-guard flagged). Review once, then kaeru_share the \
     approved ones — low-friction periodic sharing instead of deciding per capture.",
    SyncReviewArgs,
    { "type": "object", "properties": {
        "initiative": { "type": "string", "description": "initiative (default: the memory's own)" }
    } },
    |mem, a| do_sync_review(mem, a).await
);

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use kaeru_core::{EpisodeKind, Significance, Store};
    use rig::tool::Tool;
    use serde_json::json;

    use super::{PolicyArgs, ShareArgs, SyncReviewArgs};
    use crate::KaeruMemory;

    fn args<T: serde::de::DeserializeOwned>(v: serde_json::Value) -> T {
        serde_json::from_value(v).expect("args")
    }

    fn seed(store: &Store, name: &str, body: &str) {
        store.scoped(Some("t"), |s| {
            kaeru_core::write_episode(s, EpisodeKind::Observation, Significance::Low, name, body)
                .expect("write");
        });
    }

    /// `policy` (a local tool — no cloud needed) sets then reads an initiative's
    /// share policy.
    #[tokio::test]
    async fn policy_sets_then_reads() {
        let store = Arc::new(Store::open_in_memory().expect("open"));
        let mem = KaeruMemory::with_initiative(store, "t");

        let set = mem
            .policy()
            .call(args::<PolicyArgs>(
                json!({ "initiative": "t", "policy": "team" }),
            ))
            .await
            .unwrap();
        assert_eq!(
            set["policy"], "team",
            "set echoes the new policy; got {set}"
        );

        let get = mem
            .policy()
            .call(args::<PolicyArgs>(json!({ "initiative": "t" })))
            .await
            .unwrap();
        assert_eq!(get["policy"], "team");
        assert_eq!(get["permits_share"], true, "team permits share; got {get}");
    }

    /// `sync_review` (local) splits nodes by the pre-share secret guard: a clean
    /// note is proposed, one carrying a secret marker is kept local.
    #[tokio::test]
    async fn sync_review_splits_clean_from_secret() {
        let store = Arc::new(Store::open_in_memory().expect("open"));
        let mem = KaeruMemory::with_initiative(store.clone(), "t");
        mem.policy()
            .call(args::<PolicyArgs>(
                json!({ "initiative": "t", "policy": "team" }),
            ))
            .await
            .unwrap();

        seed(
            &store,
            "clean-note",
            "an ordinary observation about the design",
        );
        seed(&store, "leaky-note", "the db password: hunter2 stays here");

        let out = mem
            .sync_review()
            .call(args::<SyncReviewArgs>(json!({ "initiative": "t" })))
            .await
            .unwrap();
        let propose = out["propose_share"].as_array().expect("propose array");
        let keep = out["keep_local"].as_array().expect("keep array");
        assert!(
            propose.iter().any(|n| n["name"] == "clean-note"),
            "clean node proposed; got {out}"
        );
        assert!(
            keep.iter().any(|n| n["name"] == "leaky-note"),
            "secret node kept local; got {out}"
        );
    }

    /// A memory built without a cloud reports "cloud not configured" from a
    /// network tool — no panic, no hang, an actionable message.
    #[tokio::test]
    async fn network_tool_without_cloud_is_not_configured() {
        let store = Arc::new(Store::open_in_memory().expect("open"));
        let mem = KaeruMemory::with_initiative(store, "t");

        let out = mem
            .share()
            .call(args::<ShareArgs>(json!({ "name": "whatever" })))
            .await
            .unwrap();
        assert!(
            out["error"]
                .as_str()
                .unwrap_or("")
                .contains("cloud not configured"),
            "no-cloud share is not-configured; got {out}"
        );
    }
}
