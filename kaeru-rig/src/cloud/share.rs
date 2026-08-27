//! `share` — push a node to the team cloud through both gates.
//!
//! Gate 1 is the initiative's `share_policy`, Gate 2 the deterministic pre-share
//! secret guard. Both fail *safe*: a refusal is a message, never a silent push.
//! The local copy is marked `shared` only **after** the cloud accepts it, so
//! `shared` always means "is in the cloud".

use kaeru_core::{Visibility, guard};
use serde::Deserialize;
use serde_json::{Value, json};

use super::{cloud_or_err, format_hit};
use crate::cloud_client::CloudClient;
use crate::{KaeruMemory, mem_tool_cloud, resolve, target_initiative};

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

    // Gate 1b — and to THIS cloud? An initiative with no list is
    // unrestricted, so this changes nothing until someone asks for it.
    let init1b = initiative.clone();
    let cloud_name = client.name().to_string();
    let permitted = mem
        .blocking(move |s| kaeru_core::permits_cloud(s, &init1b, &cloud_name))
        .await
        .map_err(|e| e.to_string())?;
    if !permitted {
        let init1c = initiative.clone();
        let allowed = mem
            .blocking(move |s| kaeru_core::initiative_clouds(s, &init1c))
            .await
            .unwrap_or_default();
        return Ok(format!(
            "not shared: initiative `{initiative}` may only be shared into {} — not `{}`.",
            allowed
                .iter()
                .map(|c| format!("`{c}`"))
                .collect::<Vec<_>>()
                .join(", "),
            client.name()
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

#[derive(Debug, Deserialize)]
pub struct UnshareArgs {
    pub name: String,
    #[serde(default)]
    pub initiative: Option<String>,
    #[serde(default)]
    pub cloud: Option<String>,
}

/// Withdraws a node from a cloud and marks it local again — `share`'s inverse,
/// which did not exist. See the daemon-side verb for the reasoning (#66); the
/// local half runs even when the remote one fails, because a node still marked
/// `shared` while the cloud no longer holds it makes every later "re-share to
/// update" promise false.
async fn do_unshare(mem: &KaeruMemory, a: UnshareArgs) -> Value {
    let Some(init) = target_initiative(mem, &a.initiative) else {
        return json!({ "error": "no initiative — scope the memory or pass `initiative`" });
    };
    let client = match cloud_or_err(mem, a.cloud.as_deref()) {
        Ok(c) => c,
        Err(e) => return e,
    };
    let name = a.name.clone();
    let id = mem
        .blocking_in(Some(init.clone()), move |s| resolve(s, &name))
        .await;

    let remote = client.delete_node(&id).await;
    let local_id = id.clone();
    let local = mem
        .blocking_in(Some(init), move |s| {
            kaeru_core::set_visibility(s, &local_id, kaeru_core::Visibility::Local)
        })
        .await;
    if let Err(e) = local {
        return json!({ "unshared": false, "error": e.to_string() });
    }

    match remote {
        Ok((code, _)) if (200..300).contains(&code) => json!({
            "unshared": true, "id": id, "cloud": client.name(),
            "note": "retracted from the cloud and marked local again; the cloud keeps its history"
        }),
        Ok((code, body)) => json!({
            "unshared": "local-only", "id": id, "cloud": client.name(),
            "error": format!("cloud answered {code}: {body}"),
            "note": "marked local again, but the cloud may still serve it — retry when reachable"
        }),
        Err(e) => json!({
            "unshared": "local-only", "id": id, "cloud": client.name(), "error": e,
            "note": "marked local again, but the cloud may still serve it — retry when reachable"
        }),
    }
}

mem_tool_cloud!(
    /// `kaeru_unshare` — withdraw a node from the cloud.
    Unshare,
    "kaeru_unshare",
    "Withdraw a node from a cloud: retracts the cloud copy and marks the node local again. The \
     inverse of kaeru_share. Use it for a node sent to the wrong cloud, one the pre-share guard \
     should have caught, or anything that should not have left the machine. The cloud retraction \
     is bi-temporal — the node leaves cloud_recall while its history stays. To CORRECT a shared \
     node instead, revise it and share again: the push is an upsert under the same id.",
    UnshareArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "node name or id to withdraw" },
        "initiative": { "type": "string", "description": "initiative (default: the memory's own)" },
        "cloud": { "type": "string", "description": "named cloud to withdraw from" }
    }, "required": ["name"] },
    |mem, a| do_unshare(mem, a).await
);
