//! Cloud sharing & recall tools — the local↔cloud bridge.
//!
//! - `policy`  — read / set an initiative's sticky `share_policy` (Gate 1).
//! - `share`   — push a node to the team cloud through both gates.
//! - `cloud_recall` — list what the cloud holds for an initiative (discovery).
//! - `pull`    — materialise a cloud node into the local vault (the recall).
//!
//! `policy` is purely local (no HTTP); the other three talk to `kaeru-cloud`
//! and are async. Sharing is gated: the initiative must permit it
//! (`SharePolicy::permits_share`) and the node must clear the deterministic
//! pre-share secret guard. Both gates fail *safe* — a refusal returns an
//! explanatory message, never a silent push.

use std::str::FromStr;

use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;
use serde_json::Value;

use kaeru_core::{EdgeType, Error, Layer, NodeType, SharePolicy, Store, Tier, Visibility};

use crate::cloud_client::CloudClient;
use crate::utils::{resolve_name_or_id, text, to_mcp, with_initiative};

fn not_configured() -> McpError {
    McpError::internal_error(
        "cloud not configured — set KAERU_MCP_CLOUD_URL (and KAERU_MCP_CLOUD_TOKEN)".to_string(),
        None,
    )
}

/// Reads or sets an initiative's sticky cloud `share_policy` (Gate 1).
/// Omit `policy` to read the current value.
pub fn policy(
    store: &Store,
    initiative: &str,
    policy: Option<&str>,
) -> Result<CallToolResult, McpError> {
    match policy {
        Some(p) => {
            let parsed = SharePolicy::from_str(p).map_err(to_mcp)?;
            kaeru_core::set_share_policy(store, initiative, parsed).map_err(to_mcp)?;
            Ok(text(&format!(
                "share_policy[{initiative}] = {}",
                parsed.as_str()
            )))
        }
        None => {
            let cur = kaeru_core::get_share_policy(store, initiative).map_err(to_mcp)?;
            let note = if cur.permits_share() {
                "sharing allowed"
            } else {
                "sharing blocked"
            };
            Ok(text(&format!(
                "share_policy[{initiative}] = {} ({note})",
                cur.as_str()
            )))
        }
    }
}

/// Runs both share gates on node `id` and, if they pass, pushes it to the
/// cloud — marking the local copy `shared` only **after** the cloud accepts
/// it (so `shared` always means "is in the cloud"). Returns a human-readable
/// outcome. Shared by the `share` tool and the capture verbs that take
/// `visibility: shared`, so a capture-and-share is a single call.
pub async fn push_to_cloud(
    store: &Store,
    cloud: &CloudClient,
    id: &str,
    initiative: &str,
    force: bool,
) -> Result<String, McpError> {
    // Gate 1 — initiative policy.
    let pol = kaeru_core::get_share_policy(store, initiative).map_err(to_mcp)?;
    if !pol.permits_share() {
        return Ok(format!(
            "not shared: initiative `{initiative}` is `{}` — run `policy {initiative} team` to allow.",
            pol.as_str()
        ));
    }

    let full = kaeru_core::read_node_full(store, &id.to_string())
        .map_err(to_mcp)?
        .ok_or_else(|| to_mcp(Error::NotFound(format!("node {id} not found at NOW"))))?;

    // Gate 2 — pre-share secret guard over name + body.
    let scan_target = format!("{}\n{}", full.name, full.body.clone().unwrap_or_default());
    let hits = kaeru_core::guard::scan(&scan_target);
    if !hits.is_empty() && !force {
        let rules: Vec<&str> = hits.iter().map(|h| h.rule).collect();
        return Ok(format!(
            "not shared: pre-share guard flagged {} secret(s) [{}]. Remove them, or force=true to override.",
            hits.len(),
            rules.join(",")
        ));
    }

    let body = serde_json::json!({
        "id": full.id,
        "node_type": full.node_type,
        "tier": full.tier,
        "name": full.name,
        "body": full.body,
        "tags": full.tags,
        "initiative": initiative,
        "layer": full.layer,
    });
    let (code, resp) = cloud
        .post_node(&body)
        .await
        .map_err(|e| McpError::internal_error(format!("cloud POST failed: {e}"), None))?;

    if !(200..300).contains(&code) {
        return Ok(format!("not shared: cloud rejected ({code}): {resp}"));
    }
    kaeru_core::set_visibility(store, &full.id, Visibility::Shared).map_err(to_mcp)?;

    // Push edges to/from already-shared neighbours so the cloud keeps the
    // graph structure, not just the nodes. An edge whose other endpoint is
    // still local is skipped — it gets pushed when that endpoint is shared.
    let edges = kaeru_core::edges_of(store, &full.id).map_err(to_mcp)?;
    let mut edges_pushed = 0;
    for (src, dst, edge_type) in &edges {
        let other = if *src == full.id { dst } else { src };
        if kaeru_core::get_visibility(store, other).map_err(to_mcp)? != Visibility::Shared {
            continue;
        }
        let ebody = serde_json::json!({ "src": src, "dst": dst, "edge_type": edge_type });
        let (ecode, _) = cloud
            .post_edge(&ebody)
            .await
            .map_err(|e| McpError::internal_error(format!("cloud POST edge failed: {e}"), None))?;
        if (200..300).contains(&ecode) {
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

/// Shares an existing node by name/id to the team cloud (both gates → push).
pub async fn share(
    store: &Store,
    cloud: Option<&CloudClient>,
    name: &str,
    initiative: &str,
    force: bool,
) -> Result<CallToolResult, McpError> {
    let cloud = cloud.ok_or_else(not_configured)?;
    let id = with_initiative(store, Some(initiative), || resolve_name_or_id(store, name))?;
    let msg = push_to_cloud(store, cloud, &id, initiative, force).await?;
    Ok(text(&msg))
}

/// Lists the shared nodes the cloud holds for an initiative — discovery
/// before `pull`.
pub async fn cloud_recall(
    cloud: Option<&CloudClient>,
    initiative: &str,
) -> Result<CallToolResult, McpError> {
    let cloud = cloud.ok_or_else(not_configured)?;

    let (code, resp) = cloud
        .list_initiative(initiative)
        .await
        .map_err(|e| McpError::internal_error(format!("cloud list failed: {e}"), None))?;
    if !(200..300).contains(&code) {
        return Ok(text(&format!("cloud list failed ({code}): {resp}")));
    }

    let arr: Value = serde_json::from_str(&resp)
        .map_err(|e| McpError::internal_error(format!("bad cloud response: {e}"), None))?;
    let items = arr.as_array().cloned().unwrap_or_default();
    if items.is_empty() {
        return Ok(text(&format!(
            "cloud initiative `{initiative}` has no shared nodes yet"
        )));
    }

    let mut out = format!("cloud `{initiative}` ({} shared):\n", items.len());
    for it in &items {
        let id = it.get("id").and_then(|x| x.as_str()).unwrap_or("");
        let nt = it.get("node_type").and_then(|x| x.as_str()).unwrap_or("");
        let nm = it.get("name").and_then(|x| x.as_str()).unwrap_or("");
        out.push_str(&format!("  - {nm} ({nt}) — {id}\n"));
        if let Some(e) = it.get("body_excerpt").and_then(|x| x.as_str()) {
            out.push_str(&format!("    {e}\n"));
        }
    }
    out.push_str("\nUse `pull <id> <initiative>` to materialise one locally.");
    Ok(text(&out))
}

/// Pulls a shared node from the cloud into the local vault by id — the
/// recall mechanism. Materialises it under the same id, attached to the
/// given initiative, so it joins the local working graph.
pub async fn pull(
    store: &Store,
    cloud: Option<&CloudClient>,
    id: &str,
    initiative: &str,
) -> Result<CallToolResult, McpError> {
    let cloud = cloud.ok_or_else(not_configured)?;

    let (code, resp) = cloud
        .get_node(id)
        .await
        .map_err(|e| McpError::internal_error(format!("cloud GET failed: {e}"), None))?;
    if code == 404 {
        return Ok(text(&format!("cloud has no node {id}")));
    }
    if !(200..300).contains(&code) {
        return Ok(text(&format!("cloud GET failed ({code}): {resp}")));
    }

    let v: Value = serde_json::from_str(&resp)
        .map_err(|e| McpError::internal_error(format!("bad cloud response: {e}"), None))?;
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
        .map(|a| a.iter().filter_map(|t| t.as_str().map(String::from)).collect())
        .unwrap_or_default();
    // Preserve the cloud node's layer so pull keeps its recall priority.
    let layer = v
        .get("layer")
        .and_then(|x| x.as_str())
        .map(|s| Layer::from_str(s).unwrap_or_default())
        .unwrap_or_default();

    let node_type = NodeType::from_str(node_type_s).map_err(to_mcp)?;
    let tier = Tier::from_str(tier_s).map_err(to_mcp)?;

    kaeru_core::upsert_node(
        store,
        &id.to_string(),
        node_type,
        tier,
        &name,
        body.as_deref(),
        &tags,
        Some(initiative),
        Visibility::Shared,
        layer,
    )
    .map_err(to_mcp)?;

    // Rebuild structure: recreate every cloud edge of this initiative whose
    // BOTH endpoints already exist locally. Pulling more nodes fills in more
    // edges over time; an edge to a not-yet-pulled node is simply skipped.
    let edges_recreated = recreate_local_edges(store, cloud, initiative).await?;
    let edge_note = if edges_recreated > 0 {
        format!(" (+{edges_recreated} edge(s) linked)")
    } else {
        String::new()
    };

    Ok(text(&format!(
        "pulled `{name}` from cloud into local initiative `{initiative}` (id {id}){edge_note}"
    )))
}

/// Fetches the initiative's edges from the cloud and `link`s locally every
/// one whose both endpoints are already present in the local vault.
/// Idempotent at NOW (re-linking an existing edge is harmless).
async fn recreate_local_edges(
    store: &Store,
    cloud: &CloudClient,
    initiative: &str,
) -> Result<usize, McpError> {
    let (code, resp) = cloud
        .list_edges(initiative)
        .await
        .map_err(|e| McpError::internal_error(format!("cloud list edges failed: {e}"), None))?;
    if !(200..300).contains(&code) {
        return Ok(0);
    }
    let items: Vec<Value> = serde_json::from_str::<Value>(&resp)
        .ok()
        .and_then(|v| v.as_array().cloned())
        .unwrap_or_default();

    let mut linked = 0;
    for it in &items {
        let src = it.get("src").and_then(|x| x.as_str()).unwrap_or("");
        let dst = it.get("dst").and_then(|x| x.as_str()).unwrap_or("");
        let et = it.get("edge_type").and_then(|x| x.as_str()).unwrap_or("");
        if src.is_empty() || dst.is_empty() || et.is_empty() {
            continue;
        }
        let both_local = kaeru_core::node_brief_by_id(store, &src.to_string())
            .ok()
            .flatten()
            .is_some()
            && kaeru_core::node_brief_by_id(store, &dst.to_string())
                .ok()
                .flatten()
                .is_some();
        if !both_local {
            continue;
        }
        let Ok(edge) = et.parse::<EdgeType>() else {
            continue;
        };
        with_initiative(store, Some(initiative), || {
            kaeru_core::link(store, &src.to_string(), &dst.to_string(), edge).map_err(to_mcp)
        })?;
        linked += 1;
    }
    Ok(linked)
}

/// Creates a soft link from a local node to a cloud node by id
/// (`dst_store = cloud`) — a reference without copying. Purely local; the
/// dst is resolved later via `cloud_links`.
pub fn link_cloud(
    store: &Store,
    name: &str,
    cloud_id: &str,
    edge_type_str: &str,
    initiative: &str,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, Some(initiative), || {
        let edge: EdgeType = edge_type_str.parse().map_err(to_mcp)?;
        let src = resolve_name_or_id(store, name)?;
        kaeru_core::link_remote(store, &src, &cloud_id.to_string(), edge).map_err(to_mcp)?;
        Ok(text(&format!(
            "soft-linked `{name}` -[{}]-> cloud:{cloud_id}",
            edge.as_str()
        )))
    })
}

/// Resolves a node's cloud soft links — fetches each linked cloud node and
/// shows it. The lazy-resolution path for soft links.
pub async fn cloud_links(
    store: &Store,
    cloud: Option<&CloudClient>,
    name: &str,
    initiative: &str,
) -> Result<CallToolResult, McpError> {
    let cloud = cloud.ok_or_else(not_configured)?;

    let links = with_initiative(store, Some(initiative), || {
        let id = resolve_name_or_id(store, name)?;
        kaeru_core::cloud_links(store, &id).map_err(to_mcp)
    })?;

    if links.is_empty() {
        return Ok(text(&format!("`{name}` has no cloud soft links")));
    }

    let mut out = format!("cloud soft links of `{name}` ({}):\n", links.len());
    for (edge_type, dst) in &links {
        match cloud.get_node(dst).await {
            Ok((200..=299, body)) => {
                let v: Value = serde_json::from_str(&body).unwrap_or(Value::Null);
                let nm = v.get("name").and_then(|x| x.as_str()).unwrap_or("?");
                let nt = v.get("node_type").and_then(|x| x.as_str()).unwrap_or("?");
                out.push_str(&format!("  -[{edge_type}]-> {nm} ({nt}) — cloud:{dst}\n"));
            }
            Ok((code, _)) => {
                out.push_str(&format!("  -[{edge_type}]-> cloud:{dst} (unresolved, {code})\n"));
            }
            Err(e) => {
                out.push_str(&format!("  -[{edge_type}]-> cloud:{dst} (error: {e})\n"));
            }
        }
    }
    Ok(text(&out))
}

/// Batch sync-review: splits a team initiative's still-`local` nodes into
/// PROPOSE SHARE (guard-clean) vs KEEP LOCAL (secret-guard flagged). Review
/// once, then `share` the approved ones — low-friction periodic sharing
/// instead of a decision per capture. Purely local; proposes, never pushes.
pub fn sync_review(store: &Store, initiative: &str) -> Result<CallToolResult, McpError> {
    let pol = kaeru_core::get_share_policy(store, initiative).map_err(to_mcp)?;
    if !pol.permits_share() {
        return Ok(text(&format!(
            "initiative `{initiative}` is `{}` — nothing to sync. \
             Run `policy {initiative} team` to enable sharing.",
            pol.as_str()
        )));
    }

    let locals = kaeru_core::local_nodes_for_review(store, initiative).map_err(to_mcp)?;
    if locals.is_empty() {
        return Ok(text(&format!(
            "`{initiative}`: no local nodes to review (all shared, or empty)"
        )));
    }

    let mut propose: Vec<&kaeru_core::NodeFull> = Vec::new();
    let mut keep: Vec<(&kaeru_core::NodeFull, Vec<&str>)> = Vec::new();
    for n in &locals {
        let target = format!("{}\n{}", n.name, n.body.clone().unwrap_or_default());
        let hits = kaeru_core::guard::scan(&target);
        if hits.is_empty() {
            propose.push(n);
        } else {
            keep.push((n, hits.iter().map(|h| h.rule).collect()));
        }
    }

    let mut out = format!(
        "sync review — initiative `{initiative}` ({} local node(s)):\n\n",
        locals.len()
    );
    out.push_str(&format!(
        "PROPOSE SHARE ({}) — guard-clean, candidate team knowledge:\n",
        propose.len()
    ));
    for n in &propose {
        out.push_str(&format!("  - {} ({}) — {}\n", n.name, n.node_type, n.id));
    }
    out.push_str(&format!(
        "\nKEEP LOCAL ({}) — secret guard flagged:\n",
        keep.len()
    ));
    for (n, rules) in &keep {
        out.push_str(&format!(
            "  - {} ({}) — {} [{}]\n",
            n.name,
            n.node_type,
            n.id,
            rules.join(",")
        ));
    }
    out.push_str("\nReview the PROPOSE list, then `share <name> <initiative>` the ones you approve.");
    Ok(text(&out))
}
