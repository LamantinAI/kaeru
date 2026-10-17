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
use std::sync::Arc;

use kaeru_core::{
    CloudEdge, EdgeImportResult, EdgeType, Error, Layer, NodeType, SharePolicy, Store, Tier,
    Visibility, restore_cloud_edges_cancellable,
};
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;
use serde_json::Value;
use tokio_util::sync::CancellationToken;

use crate::cloud_client::{CloudClient, CloudRegistry};
use crate::utils::{resolve_name_or_id, text, to_mcp, with_initiative};

fn not_configured() -> McpError {
    McpError::internal_error(
        "cloud not configured — set up clouds.toml or KAERU_MCP_CLOUD_URL (and KAERU_MCP_CLOUD_TOKEN)".to_string(),
        None,
    )
}

/// Renders a guard hit as `rule: "fragment"`. The fragment is
/// `GuardHit.matched`, which the guard already truncates for safe display —
/// showing it is what lets a human tell a real secret from a false positive
/// without re-reading the whole body.
fn format_hit(h: &kaeru_core::GuardHit) -> String {
    format!("{}: {:?}", h.rule, h.matched)
}

/// Reads or sets an initiative's sticky cloud `share_policy` (Gate 1).
/// Omit `policy` to read the current value.
pub fn policy(
    store: &Store,
    initiative: &str,
    policy: Option<&str>,
    clouds: Option<&str>,
) -> Result<CallToolResult, McpError> {
    // `clouds` is set independently of the policy word, so an initiative can
    // be restricted without also being re-opened, and re-opened without
    // losing its restriction.
    if let Some(list) = clouds {
        let names: Vec<String> = list
            .split([',', ' '])
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        kaeru_core::set_initiative_clouds(store, initiative, &names).map_err(to_mcp)?;
    }
    if let Some(p) = policy {
        let parsed = SharePolicy::from_str(p).map_err(to_mcp)?;
        kaeru_core::set_share_policy(store, initiative, parsed).map_err(to_mcp)?;
    }

    let cur = kaeru_core::get_share_policy(store, initiative).map_err(to_mcp)?;
    let allowed = kaeru_core::initiative_clouds(store, initiative).map_err(to_mcp)?;
    let note = if cur.permits_share() {
        "sharing allowed"
    } else {
        "sharing blocked"
    };
    let where_to = if allowed.is_empty() {
        "any configured cloud".to_string()
    } else {
        allowed
            .iter()
            .map(|c| format!("`{c}`"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    Ok(text(&format!(
        "share_policy[{initiative}] = {} ({note})\nclouds[{initiative}] = {where_to}",
        cur.as_str()
    )))
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
    // Gate 1a — may this initiative leave at all?
    let pol = kaeru_core::get_share_policy(store, initiative).map_err(to_mcp)?;
    if !pol.permits_share() {
        return Ok(format!(
            "not shared: initiative `{initiative}` is `{}` — run `policy {initiative} team` to allow.",
            pol.as_str()
        ));
    }

    // Gate 1b — and may it leave to THIS cloud? `team` used to open an
    // initiative to every configured cloud at once, leaving the destination
    // entirely to the caller's argument. That is fine with one cloud and is
    // not a permission model with several of differing trust: a valid cloud
    // name is accepted whatever the initiative, so naming the wrong one is
    // still a way out. An initiative with no list is unrestricted, so this
    // changes nothing until someone asks for it.
    if !kaeru_core::permits_cloud(store, initiative, cloud.name()).map_err(to_mcp)? {
        let allowed = kaeru_core::initiative_clouds(store, initiative).map_err(to_mcp)?;
        return Ok(format!(
            "not shared: initiative `{initiative}` may only be shared into {} — not `{}`. \
             Change it with `policy {initiative} team --clouds <names>`.",
            allowed
                .iter()
                .map(|c| format!("`{c}`"))
                .collect::<Vec<_>>()
                .join(", "),
            cloud.name()
        ));
    }

    let full = kaeru_core::read_node_full(store, &id.to_string())
        .map_err(to_mcp)?
        .ok_or_else(|| to_mcp(Error::NotFound(format!("node {id} not found at NOW"))))?;

    // Gate 2 — pre-share secret guard over name + body. Use the strict
    // scanner: sharing leaves the machine, so the gate must be at least as
    // strict as the local markdown export (which uses `scan_public`).
    // `properties` joined the payload in #85, so it joins the guard's remit in
    // the same change — a URL or a status registry is content leaving the
    // machine like any other. `tags` are still unscanned; see AGENTS.md.
    let scan_target = format!(
        "{}\n{}\n{}",
        full.name,
        full.body.clone().unwrap_or_default(),
        full.properties
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or_default()
    );
    let hits = kaeru_core::guard::scan_public(&scan_target);
    if !hits.is_empty() && !force {
        let shown: Vec<String> = hits.iter().map(format_hit).collect();
        return Ok(format!(
            "not shared: pre-share guard flagged {} secret(s) [{}]. Remove them, or force=true to override.",
            hits.len(),
            shown.join("; ")
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
        // A `reference`'s URL and a `board`'s status registry live here. The
        // payload omitted the field entirely, so of 78 references in a live
        // cloud not one carried the link it was cited for (#85).
        "properties": full.properties,
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
    let mut held_back = 0;
    let mut rejected: Vec<String> = Vec::new();
    for (src, dst, edge_type, weight) in &edges {
        let other = if *src == full.id { dst } else { src };
        if kaeru_core::get_visibility(store, other).map_err(to_mcp)? != Visibility::Shared {
            held_back += 1;
            continue;
        }
        let ebody =
            serde_json::json!({ "src": src, "dst": dst, "edge_type": edge_type, "weight": weight });
        let (ecode, eresp) = cloud
            .post_edge(&ebody)
            .await
            .map_err(|e| McpError::internal_error(format!("cloud POST edge failed: {e}"), None))?;
        if (200..300).contains(&ecode) {
            edges_pushed += 1;
        } else {
            // A non-2xx used to just skip the increment, so an edge the cloud
            // refused looked exactly like an edge that was never eligible.
            rejected.push(format!("{edge_type} ({ecode}): {}", first_line(&eresp)));
        }
    }

    Ok(format!(
        "shared `{}` → cloud (id {}){}",
        full.name,
        full.id,
        edge_outcome(edges_pushed, held_back, &rejected)
    ))
}

/// The edge half of a share result, always stated.
///
/// It used to be printed only when at least one edge went up, so a share that
/// carried the node's whole neighbourhood and a share that carried a bare node
/// produced byte-identical lines — and since `visibility=shared` at capture
/// runs when the node has no edges at all, the silent case was the common one
/// (#85).
fn edge_outcome(pushed: usize, held_back: usize, rejected: &[String]) -> String {
    let mut parts = Vec::new();
    parts.push(format!("+{pushed} edge(s)"));
    if held_back > 0 {
        parts.push(format!(
            "{held_back} held back, other endpoint not shared — re-run `share` on this node once              it is"
        ));
    }
    if !rejected.is_empty() {
        parts.push(format!(
            "{} REJECTED: {}",
            rejected.len(),
            rejected.join("; ")
        ));
    }
    format!(" ({})", parts.join(" · "))
}

/// First line of a response body, capped — enough to see why a push failed
/// without pasting a page of HTML into a tool result.
fn first_line(body: &str) -> String {
    let line = body.lines().next().unwrap_or("").trim();
    if line.chars().count() > 120 {
        format!("{}…", line.chars().take(120).collect::<String>())
    } else {
        line.to_string()
    }
}

/// What happened to an edge locally, so the cloud can be told the same thing.
pub enum EdgeChange {
    /// Created or reweighted — the cloud's `POST /edges` is an upsert.
    Upsert(f64),
    /// Retracted — `DELETE /edges`.
    Retract,
}

/// Mirrors a local edge change to the cloud when **both** endpoints are
/// already shared, appending what happened to `msg`.
///
/// `link`, `unlink` and `reweight` had no path to HTTP at all, so the graph
/// the cloud held was frozen at the moment of each node's share. Since
/// `visibility=shared` at capture runs when a node has zero edges, that meant
/// a "capture and push in one call" node structurally never carried an edge,
/// and an `unlink` was undone by the next `pull` (#85).
///
/// Silent when either endpoint is local: that edge is not the cloud's to hold
/// yet, and it goes up on its own when the other endpoint is shared. Silent
/// too when no cloud is configured — this is a local-first product and a
/// graph edit must not start failing because a daemon has no cloud.
#[allow(clippy::too_many_arguments)]
pub async fn propagate_edge(
    store: &Store,
    clouds: &CloudRegistry,
    cloud_name: Option<&str>,
    src: &str,
    dst: &str,
    edge_type: EdgeType,
    change: EdgeChange,
    initiative: Option<&str>,
    msg: &mut String,
) -> Result<(), McpError> {
    // Everything cheap and local first: an edge with a local endpoint is not
    // the cloud's to hold, and there is nothing to resolve for it.
    let Some(init) = initiative else {
        return Ok(());
    };
    let both_shared = kaeru_core::get_visibility(store, &src.to_string()).map_err(to_mcp)?
        == Visibility::Shared
        && kaeru_core::get_visibility(store, &dst.to_string()).map_err(to_mcp)?
            == Visibility::Shared;
    if !both_shared || clouds.is_empty() {
        return Ok(());
    }

    // `resolve` is the single funnel, and it refuses to guess between several
    // configured clouds. Here that must not fail the call: the local edit has
    // already happened and is the point of the verb. So the ambiguity is
    // reported instead — silence would leave local and cloud differing with
    // nothing to say it.
    let cloud = match clouds.resolve(cloud_name) {
        Ok(c) => c,
        Err(why) => {
            msg.push_str(&format!(
                "\n↳ not mirrored: {why} — pass `cloud=<name>` to send this edge"
            ));
            return Ok(());
        }
    };

    // The nodes passed the gates when they were shared, but the initiative may
    // have been closed since. Nothing more leaves an initiative that has been
    // set back to `private`.
    let pol = kaeru_core::get_share_policy(store, init).map_err(to_mcp)?;
    if !pol.permits_share()
        || !kaeru_core::permits_cloud(store, init, cloud.name()).map_err(to_mcp)?
    {
        msg.push_str(&format!(
            "\n(both endpoints are shared, but `{init}` no longer permits sharing to `{}` — the \
             cloud still holds the old edge)",
            cloud.name()
        ));
        return Ok(());
    }

    let (code, resp) = match change {
        EdgeChange::Upsert(weight) => {
            let body = serde_json::json!({
                "src": src, "dst": dst, "edge_type": edge_type.as_str(), "weight": weight
            });
            cloud.post_edge(&body).await
        }
        EdgeChange::Retract => {
            let body = serde_json::json!({
                "src": src, "dst": dst, "edge_type": edge_type.as_str()
            });
            cloud.delete_edge(&body).await
        }
    }
    .map_err(|e| McpError::internal_error(format!("cloud edge call failed: {e}"), None))?;

    if (200..300).contains(&code) {
        msg.push_str(&format!("\n↳ mirrored to cloud `{}`", cloud.name()));
    } else {
        // Said out loud rather than swallowed: local and cloud now disagree,
        // and the agent is the only one in a position to retry.
        msg.push_str(&format!(
            "\n↳ NOT mirrored to cloud `{}` ({code}): {} — local and cloud now differ",
            cloud.name(),
            first_line(&resp)
        ));
    }
    Ok(())
}

/// Initiative names out of a cloud's `GET /api/v1/initiatives` payload.
/// Best-effort: a shape we do not recognise yields nothing rather than an
/// error, since this only ever enriches a message.
fn initiative_names(body: &str) -> Vec<String> {
    serde_json::from_str::<Value>(body)
        .ok()
        .and_then(|v| v.as_array().cloned())
        .unwrap_or_default()
        .iter()
        .filter_map(|it| {
            it.get("name")
                .or_else(|| it.get("initiative"))
                .and_then(|x| x.as_str())
                .map(String::from)
        })
        .collect()
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

/// Lists the initiatives a cloud holds, with node counts.
///
/// The REST endpoint always existed and nothing on the agent's surface reached
/// it, so in a real session the answer to "what's in the cloud?" was the agent
/// shelling out to `curl` against kaeru's own API — and the user asking why it
/// wasn't using kaeru. A tool that exists in the transport but not in the verb
/// set is a tool the agent has to work around.
///
/// Not to be confused with `clouds`, which lists the clouds this daemon can
/// reach. This lists what is *inside* one of them.
pub async fn cloud_initiatives(cloud: Option<&CloudClient>) -> Result<CallToolResult, McpError> {
    let cloud = cloud.ok_or_else(not_configured)?;
    let (code, resp) = cloud
        .list_initiatives()
        .await
        .map_err(|e| McpError::internal_error(format!("cloud list failed: {e}"), None))?;
    if !(200..300).contains(&code) {
        return Ok(text(&format!(
            "cloud `{}`: list failed ({code}): {resp}",
            cloud.name()
        )));
    }
    let items = serde_json::from_str::<Value>(&resp)
        .ok()
        .and_then(|v| v.as_array().cloned())
        .unwrap_or_default();
    if items.is_empty() {
        return Ok(text(&format!(
            "cloud `{}` holds no initiatives yet.",
            cloud.name()
        )));
    }
    let mut out = format!("cloud `{}` ({} initiatives):\n", cloud.name(), items.len());
    for it in &items {
        let name = it.get("name").and_then(|x| x.as_str()).unwrap_or("");
        let nodes = it.get("nodes").and_then(|x| x.as_u64()).unwrap_or(0);
        out.push_str(&format!("  - {name} — {nodes} shared node(s)\n"));
    }
    out.push_str(
        "\n↳ `cloud_recall <initiative>` lists or searches one; an initiative here is \
         independent of the local one with the same name.",
    );
    Ok(text(&out))
}

/// Withdraws a node from a cloud and returns it to `local`.
///
/// `share` had no inverse at all, which made every mistake permanent: a node
/// sent to the wrong cloud, a body the pre-share guard should have caught, an
/// experiment that should never have left the machine. The only removal the
/// cloud offered was deleting the whole initiative around it.
///
/// Both halves run, and the local half runs even if the remote one fails: a
/// node still marked `shared` locally while the cloud no longer holds it would
/// make `revise` promise a re-share that has nothing to update, and the local
/// vault is the copy the agent actually reads. The result says plainly which
/// halves happened.
pub async fn unshare(
    store: &Store,
    cloud: Option<&CloudClient>,
    name: &str,
    initiative: &str,
) -> Result<CallToolResult, McpError> {
    let cloud = cloud.ok_or_else(not_configured)?;
    let id = with_initiative(store, Some(initiative), || resolve_name_or_id(store, name))?;

    let remote = cloud.delete_node(&id).await;
    with_initiative(store, Some(initiative), || {
        kaeru_core::set_visibility(store, &id, Visibility::Local).map_err(to_mcp)
    })?;

    let msg = match remote {
        Ok((code, _)) if (200..300).contains(&code) => format!(
            "unshared: {name} — retracted from cloud `{}` and marked local again.\n\
             ↳ the cloud keeps its history: a read at a past moment still sees it, but \
             `cloud_recall` and the listings do not.",
            cloud.name()
        ),
        Ok((code, body)) => format!(
            "unshared LOCALLY only: {name} is marked local again, but cloud `{}` answered \
             {code} — {body}\n↳ the cloud may still serve it; retry `unshare` once the cloud \
             is reachable.",
            cloud.name()
        ),
        Err(e) => format!(
            "unshared LOCALLY only: {name} is marked local again, but cloud `{}` was \
             unreachable — {e}\n↳ the cloud may still serve it; retry `unshare` when it is back.",
            cloud.name()
        ),
    };
    Ok(text(&msg))
}

/// Lists the shared nodes the cloud holds for an initiative — discovery
/// before `pull`. If `id` is specified, returns the full shared node directly
/// without materialising it into the local vault or modifying the graph.
pub async fn cloud_recall(
    cloud: Option<&CloudClient>,
    initiative: &str,
    id: Option<&str>,
    query: Option<&str>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<CallToolResult, McpError> {
    /// Page size when the caller does not choose one. Deliberately smaller
    /// than the server's own default: this output lands in a context window.
    const DEFAULT_LIMIT: usize = 25;

    let cloud = cloud.ok_or_else(not_configured)?;

    if let Some(node_id) = id {
        let (code, resp) = cloud
            .get_node_in_initiative(initiative, node_id)
            .await
            .map_err(|e| McpError::internal_error(format!("cloud GET failed: {e}"), None))?;
        if code == 404 {
            return Ok(text(&format!(
                "cloud `{}`: has no node {node_id}",
                cloud.name()
            )));
        }
        if !(200..300).contains(&code) {
            return Ok(text(&format!(
                "cloud `{}`: GET failed ({code}): {resp}",
                cloud.name()
            )));
        }
        let v: Value = serde_json::from_str(&resp)
            .map_err(|e| McpError::internal_error(format!("bad cloud response: {e}"), None))?;
        let name = v.get("name").and_then(|x| x.as_str()).unwrap_or("");
        let nt = v.get("node_type").and_then(|x| x.as_str()).unwrap_or("");
        let tier = v.get("tier").and_then(|x| x.as_str()).unwrap_or("");
        let layer = v.get("layer").and_then(|x| x.as_str()).unwrap_or("");
        let body = v.get("body").and_then(|x| x.as_str()).unwrap_or("");
        let tags: Vec<String> = v
            .get("tags")
            .and_then(|x| x.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|t| t.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let properties_line = v
            .get("properties")
            .filter(|p| !p.is_null())
            .map(|p| format!("\nproperties: {p}"))
            .unwrap_or_default();
        let tag_line = if tags.is_empty() {
            String::new()
        } else {
            format!("\ntags: {}", tags.join(", "))
        };
        return Ok(text(&format!(
            "cloud `{}` · `{initiative}`: node {node_id} (read-only, not imported)\n\
             name: {name}\ntype: {nt} · tier: {tier} · layer: {layer}{tag_line}{properties_line}\n\n{body}\n\n\
             ⤷ `pull {node_id} {initiative}` materialises it locally with edges.",
            cloud.name()
        )));
    }
    let limit = limit.unwrap_or(DEFAULT_LIMIT).max(1);
    let offset = offset.unwrap_or(0);

    let (code, resp) = cloud
        .list_initiative(initiative, limit, offset, query)
        .await
        .map_err(|e| McpError::internal_error(format!("cloud list failed: {e}"), None))?;
    if !(200..300).contains(&code) {
        return Ok(text(&format!(
            "cloud `{}`: list failed ({code}): {resp}",
            cloud.name()
        )));
    }

    let arr: Value = serde_json::from_str(&resp)
        .map_err(|e| McpError::internal_error(format!("bad cloud response: {e}"), None))?;
    let items = arr.as_array().cloned().unwrap_or_default();

    // The true total comes from the initiative listing, which already carries
    // per-initiative counts — so a bounded page can still report honestly how
    // much there is, without changing the listing's response shape.
    let total = cloud
        .list_initiatives()
        .await
        .ok()
        .filter(|(code, _)| (200..300).contains(code))
        .and_then(|(_, body)| initiative_total(&body, initiative));

    if items.is_empty() {
        // "Empty" and "not here" used to read identically, which in a
        // multi-cloud setup is the difference between "nobody has shared
        // anything" and "you asked the wrong cloud".
        if let Some(q) = query {
            return Ok(text(&format!(
                "cloud `{}` · `{initiative}`: nothing matching {q:?}{}. \
                 Drop `query` to list everything shared.",
                cloud.name(),
                total
                    .map(|t| format!(" (of {t} shared)"))
                    .unwrap_or_default()
            )));
        }
        if offset > 0 {
            return Ok(text(&format!(
                "cloud `{}` · `{initiative}`: no nodes past offset {offset}{}.",
                cloud.name(),
                total.map(|t| format!(" (total {t})")).unwrap_or_default()
            )));
        }
        let known = cloud
            .list_initiatives()
            .await
            .ok()
            .and_then(|(code, body)| (200..300).contains(&code).then(|| initiative_names(&body)));
        let tail = match known {
            Some(names) if !names.iter().any(|n| n == initiative) => {
                let listed = if names.is_empty() {
                    "it holds none at all".to_string()
                } else {
                    format!("it holds: {}", names.join(", "))
                };
                format!(
                    " — in fact cloud `{}` has no initiative by that name; {listed}.",
                    cloud.name()
                )
            }
            _ => String::new(),
        };
        return Ok(text(&format!(
            "cloud `{}`: initiative `{initiative}` has no shared nodes yet{tail}",
            cloud.name()
        )));
    }

    let shown = items.len();
    let scope = query
        .map(|q| format!(" matching {q:?}"))
        .unwrap_or_default();
    let header = match total {
        // With a query the total is the initiative's, not the match count —
        // say which, rather than implying the filter found that many.
        Some(t) if query.is_some() => format!(
            "cloud `{}` · `{initiative}`{scope} ({shown} shown, of {t} shared):\n",
            cloud.name()
        ),
        Some(t) => format!(
            "cloud `{}` · `{initiative}` ({}–{} of {t}):\n",
            cloud.name(),
            offset + 1,
            offset + shown
        ),
        None => format!(
            "cloud `{}` · `{initiative}`{scope} ({shown} shared, from {offset}):\n",
            cloud.name()
        ),
    };
    let mut out = header;
    for it in &items {
        let id = it.get("id").and_then(|x| x.as_str()).unwrap_or("");
        let nt = it.get("node_type").and_then(|x| x.as_str()).unwrap_or("");
        let nm = it.get("name").and_then(|x| x.as_str()).unwrap_or("");
        out.push_str(&format!("  - {nm} ({nt}) — {id}\n"));
        if let Some(e) = it.get("body_excerpt").and_then(|x| x.as_str()) {
            out.push_str(&format!("    {e}\n"));
        }
    }

    // Never truncate silently: say a page ended and how to ask for the next.
    let more = match total {
        Some(t) if offset + shown < t => Some(t - offset - shown),
        _ => (shown == limit).then_some(0),
    };
    if let Some(rest) = more {
        let rest = if rest > 0 {
            format!("{rest} more")
        } else {
            "more".to_string()
        };
        out.push_str(&format!(
            "\n↳ {rest} — next page: `cloud_recall {initiative}{} --offset {} --limit {limit}`.",
            query.map(|q| format!(" --query {q:?}")).unwrap_or_default(),
            offset + shown
        ));
    }
    out.push_str("\n↳ `pull <id> <initiative>` materialises one locally.");
    Ok(text(&out))
}

/// A named initiative's node count out of `GET /api/v1/initiatives`.
fn initiative_total(body: &str, initiative: &str) -> Option<usize> {
    serde_json::from_str::<Value>(body)
        .ok()?
        .as_array()?
        .iter()
        .find(|it| it.get("name").and_then(|x| x.as_str()) == Some(initiative))
        .and_then(|it| it.get("nodes").and_then(|x| x.as_u64()))
        .map(|n| n as usize)
}

/// Pulls a shared node from the cloud into the local vault by id — the
/// recall mechanism. Materialises it under the same id, attached to the
/// given initiative, so it joins the local working graph.
pub async fn pull(
    store: Arc<Store>,
    cloud: Option<&CloudClient>,
    id: &str,
    initiative: &str,
    cancel: CancellationToken,
) -> Result<CallToolResult, McpError> {
    let cloud = cloud.ok_or_else(not_configured)?;
    let check_cancelled = || {
        if cancel.is_cancelled() {
            Err(McpError::internal_error(
                "pull cancelled before node import",
                None,
            ))
        } else {
            Ok(())
        }
    };
    check_cancelled()?;

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
        .map(|a| {
            a.iter()
                .filter_map(|t| t.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    // Preserve the cloud node's layer so pull keeps its recall priority.
    let layer = v
        .get("layer")
        .and_then(|x| x.as_str())
        .map(|s| Layer::from_str(s).unwrap_or_default())
        .unwrap_or_default();

    let node_type = NodeType::from_str(node_type_s).map_err(to_mcp)?;
    let tier = Tier::from_str(tier_s).map_err(to_mcp)?;
    // A cloud too old to send the field sends nothing, and `None` tells
    // `upsert_node` to keep whatever is local rather than clear it. That
    // ordering matters: the destructive case was pulling a `cite` to refresh
    // it and losing the URL it was cited for (#85).
    let properties = v.get("properties").filter(|p| !p.is_null()).cloned();
    check_cancelled()?;
    let write_cancel = cancel.clone();
    let write_store = store.clone();
    let write_id = id.to_string();
    let write_init = initiative.to_string();
    let write_name = name.clone();
    tokio::task::spawn_blocking(move || {
        write_store.scoped(Some(&write_init), |s| {
            if write_cancel.is_cancelled() {
                return Err(Error::Invalid("pull cancelled before node import".into()));
            }
            kaeru_core::upsert_node(
                s,
                &write_id,
                node_type,
                tier,
                &write_name,
                body.as_deref(),
                &tags,
                Some(&write_init),
                Visibility::Shared,
                layer,
                properties.as_ref(),
            )
        })
    })
    .await
    .map_err(|e| McpError::internal_error(format!("node import task failed: {e}"), None))?
    .map_err(to_mcp)?;

    // Restore this node's incident edges whose endpoints are local. Pulling
    // more nodes fills in more edges; a not-yet-pulled endpoint is skipped.
    let rebuilt = recreate_local_edges(store, cloud, initiative, id, cancel)
        .await
        .map_err(|e| {
            McpError::internal_error(
                format!(
                    "cloud `{}`: node imported, but edge restore failed: {}",
                    cloud.name(),
                    e.message
                ),
                Some(serde_json::json!({ "node_imported": true })),
            )
        })?;
    let mut edge_parts = vec![format!("+{} edge(s) linked", rebuilt.linked)];
    if rebuilt.kept > 0 {
        edge_parts.push(format!("{} already local, left as they are", rebuilt.kept));
    }
    let edge_note = format!(" ({})", edge_parts.join(" · "));

    Ok(text(&format!(
        "pulled `{name}` from cloud into local initiative `{initiative}` (id {id}){edge_note}"
    )))
}

/// Fetch the cloud graph once, then delegate bounded node-local work to core.
async fn recreate_local_edges(
    store: Arc<Store>,
    cloud: &CloudClient,
    initiative: &str,
    target: &str,
    cancel: CancellationToken,
) -> Result<EdgeImportResult, McpError> {
    let (code, resp) = cloud.list_edges(initiative).await.map_err(|e| {
        McpError::internal_error(
            format!("cloud `{}` list edges failed: {e}", cloud.name()),
            None,
        )
    })?;
    if !(200..300).contains(&code) {
        return Err(McpError::internal_error(
            format!(
                "node imported, but cloud `{}` edge restore failed ({code})",
                cloud.name()
            ),
            None,
        ));
    }
    let items: Vec<Value> = serde_json::from_str(&resp)
        .map_err(|e| McpError::internal_error(format!("invalid cloud edge list: {e}"), None))?;
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
    let initiative = initiative.to_string();
    let target = target.to_string();
    tokio::task::spawn_blocking(move || {
        restore_cloud_edges_cancellable(&store, &initiative, &target, &edges, || {
            cancel.is_cancelled()
        })
    })
    .await
    .map_err(|e| McpError::internal_error(format!("edge import task failed: {e}"), None))?
    .map_err(to_mcp)
}

/// Creates a soft link from a local node to a cloud node by id — a reference
/// without copying. Purely local; the dst is resolved later via
/// `cloud_links`. `cloud_name` (when given) records which cloud the dst lives
/// in (`dst_store = cloud:<name>`) and must be a configured cloud; omit it
/// for the default cloud.
pub fn link_cloud(
    store: &Store,
    clouds: &CloudRegistry,
    name: &str,
    cloud_id: &str,
    edge_type_str: &str,
    cloud_name: Option<&str>,
    initiative: &str,
) -> Result<CallToolResult, McpError> {
    // Refuse to bake a name we can't resolve later — that would dangle.
    if let Some(cn) = cloud_name
        && !clouds.contains(cn)
    {
        return Ok(text(&format!(
            "unknown cloud `{cn}` — configured: [{}]",
            clouds.names().join(", ")
        )));
    }
    with_initiative(store, Some(initiative), || {
        let edge: EdgeType = edge_type_str.parse().map_err(to_mcp)?;
        let src = resolve_name_or_id(store, name)?;
        kaeru_core::link_remote_to(store, &src, &cloud_id.to_string(), edge, cloud_name)
            .map_err(to_mcp)?;
        let tag = cloud_name
            .map(|n| format!("cloud:{n}:"))
            .unwrap_or_else(|| "cloud:".to_string());
        Ok(text(&format!(
            "soft-linked `{name}` -[{}]-> {tag}{cloud_id}",
            edge.as_str()
        )))
    })
}

/// Resolves a node's cloud soft links — fetches each linked cloud node and
/// shows it. The lazy-resolution path for soft links; each link is routed to
/// the cloud it was created against (multi-cloud aware).
pub async fn cloud_links(
    store: &Store,
    clouds: &CloudRegistry,
    name: &str,
    initiative: &str,
) -> Result<CallToolResult, McpError> {
    if clouds.is_empty() {
        return Err(not_configured());
    }

    let links = with_initiative(store, Some(initiative), || {
        let id = resolve_name_or_id(store, name)?;
        kaeru_core::cloud_links(store, &id).map_err(to_mcp)
    })?;

    if links.is_empty() {
        return Ok(text(&format!("`{name}` has no cloud soft links")));
    }

    let mut out = format!("cloud soft links of `{name}` ({}):\n", links.len());
    for (edge_type, cloud_name, dst) in &links {
        // Display tag: cloud:dst for the default, cloud:<name>:dst otherwise.
        let tag = cloud_name
            .as_deref()
            .map(|n| format!("cloud:{n}:{dst}"))
            .unwrap_or_else(|| format!("cloud:{dst}"));
        let Some(client) = clouds.get(cloud_name.as_deref()) else {
            out.push_str(&format!(
                "  -[{edge_type}]-> {tag} (unresolved — cloud `{}` not configured)\n",
                cloud_name.as_deref().unwrap_or("default")
            ));
            continue;
        };
        match client.get_node(dst).await {
            Ok((200..=299, body)) => {
                let v: Value = serde_json::from_str(&body).unwrap_or(Value::Null);
                let nm = v.get("name").and_then(|x| x.as_str()).unwrap_or("?");
                let nt = v.get("node_type").and_then(|x| x.as_str()).unwrap_or("?");
                out.push_str(&format!("  -[{edge_type}]-> {nm} ({nt}) — {tag}\n"));
            }
            Ok((code, _)) => {
                out.push_str(&format!("  -[{edge_type}]-> {tag} (unresolved, {code})\n"));
            }
            Err(e) => {
                out.push_str(&format!("  -[{edge_type}]-> {tag} (error: {e})\n"));
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
    let mut keep: Vec<(&kaeru_core::NodeFull, Vec<String>)> = Vec::new();
    for n in &locals {
        let target = format!("{}\n{}", n.name, n.body.clone().unwrap_or_default());
        // Strict scanner — the share gate must be ≥ the export gate (see `share`).
        let hits = kaeru_core::guard::scan_public(&target);
        if hits.is_empty() {
            propose.push(n);
        } else {
            // Rule id + the matched fragment (already truncated by the
            // guard for safe display) — without the fragment a false
            // positive can't be recognised without re-reading the body.
            keep.push((n, hits.iter().map(format_hit).collect()));
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
    for (n, hits) in &keep {
        out.push_str(&format!(
            "  - {} ({}) — {} [{}]\n",
            n.name,
            n.node_type,
            n.id,
            hits.join("; ")
        ));
    }
    out.push_str(
        "\nReview the PROPOSE list, then `share <name> <initiative>` the ones you approve.",
    );
    Ok(text(&out))
}

#[cfg(test)]
mod cloud_read_tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use axum::extract::{Path, Query};
    use axum::http::StatusCode;
    use axum::{Json, Router, routing::get};
    use kaeru_core::{Store, read_node_full, recall_id_by_name};
    use serde_json::{Value, json};
    use tokio::net::TcpListener;
    use tokio_util::sync::CancellationToken;

    use super::{cloud_recall, pull};
    use crate::cloud_client::CloudClient;

    #[tokio::test]
    async fn full_cloud_membership_pages_and_fails_closed() {
        let app = Router::new()
            .route("/api/v1/nodes/{id}", get(|Path(id): Path<String>| async move {
                Json(json!({"id": if id == "wrong-id" { "another-id" } else { &id },
                    "name": "same name", "body": "private full body"}))
            }))
            .route("/api/v1/initiatives/{initiative}/nodes", get(
                |Path(init): Path<String>, Query(q): Query<HashMap<String, String>>| async move {
                    assert_eq!(q.get("q").map(String::as_str), Some("same name"));
                    assert_eq!(q.get("limit").map(String::as_str), Some("500"));
                    let offset = q.get("offset").unwrap().parse::<usize>().unwrap();
                    match init.as_str() {
                        "alpha / β?#" if offset == 0 => Json(json!((0..500).map(|i| json!({"id":format!("other-{i}")})).collect::<Vec<_>>())),
                        "alpha / β?#" => { assert_eq!(offset, 500); Json(json!([{"id":"node-a"}])) },
                        "malformed" => Json(json!({"not":"a list"})),
                        _ => Json(json!([])),
                    }
                }
            ));
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let cloud = CloudClient::new("test".into(), format!("http://{addr}"), String::new());
        let (status, body) = cloud
            .get_node_in_initiative("alpha / β?#", "node-a")
            .await
            .unwrap();
        assert_eq!(status, 200);
        assert!(body.contains("private full body"));
        let (status, body) = cloud
            .get_node_in_initiative("beta", "node-a")
            .await
            .unwrap();
        assert_eq!(status, 404);
        assert!(!body.contains("private full body"));
        assert!(
            cloud
                .get_node_in_initiative("malformed", "node-a")
                .await
                .is_err()
        );
        assert!(
            cloud
                .get_node_in_initiative("alpha", "wrong-id")
                .await
                .is_err()
        );
        task.abort();
    }

    #[tokio::test]
    async fn cancelled_pull_never_imports_and_edge_failure_reports_partial_write() {
        let cancel = CancellationToken::new();
        let server_cancel = cancel.clone();
        let app = Router::new()
            .route(
                "/api/v1/nodes/{id}",
                get(move |Path(id): Path<String>| {
                    let cancel = server_cancel.clone();
                    async move {
                        if id == "cancel-during-get" {
                            cancel.cancel();
                        }
                        Json(
                            json!({"id":id,"name":id,"node_type":"reference", "tier":"archival",
                        "layer":"hot","body":"full body","tags":[]}),
                        )
                    }
                }),
            )
            .route(
                "/api/v1/initiatives/alpha/edges",
                get(|| async { StatusCode::SERVICE_UNAVAILABLE }),
            );
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let cloud = CloudClient::new("test".into(), format!("http://{addr}"), String::new());
        let store = Arc::new(Store::open_in_memory().unwrap());
        let pre_cancel = CancellationToken::new();
        pre_cancel.cancel();
        assert!(
            pull(
                store.clone(),
                Some(&cloud),
                "pre-cancelled",
                "alpha",
                pre_cancel
            )
            .await
            .is_err()
        );
        assert!(
            pull(
                store.clone(),
                Some(&cloud),
                "cancel-during-get",
                "alpha",
                cancel
            )
            .await
            .is_err()
        );
        assert!(
            read_node_full(&store, &"pre-cancelled".into())
                .unwrap()
                .is_none()
        );
        assert!(
            read_node_full(&store, &"cancel-during-get".into())
                .unwrap()
                .is_none()
        );
        let err = pull(
            store.clone(),
            Some(&cloud),
            "partial",
            "alpha",
            CancellationToken::new(),
        )
        .await
        .unwrap_err();
        assert_eq!(err.data.unwrap()["node_imported"], true);
        assert!(err.message.contains("node imported"));
        assert!(read_node_full(&store, &"partial".into()).unwrap().is_some());
        task.abort();
    }

    #[tokio::test]
    async fn full_cloud_read_is_scoped_and_does_not_import_then_pull_is_idempotent() {
        let app = Router::new()
            .route("/api/v1/nodes/node-a", get(|| async { Json(json!({
                "id":"node-a", "name":"full-note", "node_type":"reference", "tier":"archival",
                "layer":"hot", "visibility":"shared", "body":"complete body", "tags":[],
                "properties":{"url":"https://example.test/source"}
            })) }))
            .route("/api/v1/initiatives/alpha/nodes", get(|| async {Json(json!([{"id":"node-a"}]))}))
            .route("/api/v1/initiatives/beta/nodes", get(|| async {Json(Vec::<Value>::new())}))
            .route("/api/v1/initiatives/alpha/edges", get(|| async {Json(Vec::<Value>::new())}));
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let cloud = CloudClient::new("test".into(), format!("http://{addr}"), String::new());
        let store = Arc::new(Store::open_in_memory().unwrap());
        let out = cloud_recall(Some(&cloud), "alpha", Some("node-a"), None, None, None)
            .await
            .unwrap();
        let text = serde_json::to_string(&out).unwrap();
        assert!(text.contains("complete body") && text.contains("not imported"));
        assert!(text.contains("https://example.test/source"));
        assert!(recall_id_by_name(&store, "full-note").unwrap().is_none());
        let other = cloud_recall(Some(&cloud), "beta", Some("node-a"), None, None, None)
            .await
            .unwrap();
        assert!(
            !serde_json::to_string(&other)
                .unwrap()
                .contains("complete body")
        );
        pull(
            store.clone(),
            Some(&cloud),
            "node-a",
            "alpha",
            CancellationToken::new(),
        )
        .await
        .unwrap();
        pull(
            store.clone(),
            Some(&cloud),
            "node-a",
            "alpha",
            CancellationToken::new(),
        )
        .await
        .unwrap();
        assert_eq!(
            store
                .scoped(Some("alpha"), |s| recall_id_by_name(s, "full-note"))
                .unwrap()
                .as_deref(),
            Some("node-a")
        );
        task.abort();
    }
}
