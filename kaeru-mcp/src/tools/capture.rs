//! Write-side tools: `episode`, `jot`, `link`, `unlink`, `cite`.
//!
//! The capture verbs (`episode` / `jot` / `cite`) take an optional
//! `visibility`. With `visibility: shared` the freshly-created node is
//! pushed to the team cloud in the **same call** — gated exactly like
//! `share` (initiative policy + secret guard). The local `shared` flag is
//! set only after the cloud accepts it, so it never marks a node shared
//! that isn't actually in the cloud. `link` / `unlink` stay purely local.

use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use kaeru_core::{EdgeType, EpisodeKind, Significance, Store};

use crate::cloud_client::CloudClient;
use crate::tools::cloud::push_to_cloud;
use crate::utils::{
    parse_layer, parse_wants_shared, resolve_name, text, to_mcp, with_initiative,
};

/// When `want_share`, attempts to push the just-created node `id` to the
/// cloud and appends the outcome to `msg`. Needs both a configured cloud
/// and an initiative (sharing policy is per-initiative); absent either, it
/// notes that the node stayed local.
async fn maybe_share(
    store: &Store,
    cloud: Option<&CloudClient>,
    id: &str,
    initiative: Option<&str>,
    want_share: bool,
    msg: &mut String,
) -> Result<(), McpError> {
    if !want_share {
        return Ok(());
    }
    match (cloud, initiative) {
        (Some(c), Some(init)) => {
            let outcome = push_to_cloud(store, c, id, init, false).await?;
            msg.push('\n');
            msg.push_str(&outcome);
        }
        (None, _) => {
            msg.push_str("\n(shared requested, but cloud not configured — saved local)");
        }
        (_, None) => {
            msg.push_str("\n(shared requested, but no initiative — saved local; pass initiative to share)");
        }
    }
    Ok(())
}

pub async fn episode(
    store: &Store,
    cloud: Option<&CloudClient>,
    name: &str,
    body: &str,
    layer: Option<&str>,
    visibility: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    let want_share = parse_wants_shared(visibility)?;
    let id = with_initiative(store, initiative, || {
        let layer = parse_layer(layer)?;
        kaeru_core::write_episode_with_layer(
            store,
            EpisodeKind::Observation,
            Significance::Medium,
            name,
            body,
            layer,
        )
        .map_err(to_mcp)
    })?;
    let mut msg = format!("wrote episode: {name} — {id}");
    maybe_share(store, cloud, &id, initiative, want_share, &mut msg).await?;
    Ok(text(&msg))
}

pub async fn jot(
    store: &Store,
    cloud: Option<&CloudClient>,
    body: &str,
    layer: Option<&str>,
    visibility: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    let want_share = parse_wants_shared(visibility)?;
    let id = with_initiative(store, initiative, || {
        let layer = parse_layer(layer)?;
        kaeru_core::jot_with_layer(store, body, layer).map_err(to_mcp)
    })?;
    let name = kaeru_core::node_brief_by_id(store, &id)
        .ok()
        .flatten()
        .map(|b| b.name)
        .unwrap_or_default();
    let mut msg = format!("jotted: {name} — {id}");
    maybe_share(store, cloud, &id, initiative, want_share, &mut msg).await?;
    Ok(text(&msg))
}

pub fn link(
    store: &Store,
    from: &str,
    to: &str,
    edge_type_str: &str,
    weight: Option<f64>,
    strong: bool,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let edge: EdgeType = edge_type_str.parse().map_err(to_mcp)?;
        let from_id = resolve_name(store, from)?;
        let to_id = resolve_name(store, to)?;
        // Plain link = 0.5 (a neutral association); `strong` = 1.0 (a key
        // reasoning link); explicit `weight` overrides. Weight is the
        // connection strength that drives chain shortest-paths.
        let w = match (weight, strong) {
            (Some(w), _) => w,
            (None, true) => 1.0,
            (None, false) => 0.5,
        };
        kaeru_core::link_with_weight(store, &from_id, &to_id, edge, w).map_err(to_mcp)?;
        Ok(text(&format!(
            "linked: {from} -[{}]-> {to} (weight {w:.2})",
            edge.as_str()
        )))
    })
}

pub fn unlink(
    store: &Store,
    from: &str,
    to: &str,
    edge_type_str: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let edge: EdgeType = edge_type_str.parse().map_err(to_mcp)?;
        let from_id = resolve_name(store, from)?;
        let to_id = resolve_name(store, to)?;
        kaeru_core::unlink(store, &from_id, &to_id, edge).map_err(to_mcp)?;
        Ok(text(&format!(
            "unlinked: {from} -[{}]-> {to}",
            edge.as_str()
        )))
    })
}

pub async fn cite(
    store: &Store,
    cloud: Option<&CloudClient>,
    name: &str,
    url: Option<&str>,
    body: &str,
    layer: Option<&str>,
    visibility: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    let want_share = parse_wants_shared(visibility)?;
    let id = with_initiative(store, initiative, || {
        let layer = parse_layer(layer)?;
        kaeru_core::cite_with_layer(store, name, url, body, layer).map_err(to_mcp)
    })?;
    let mut msg = match url {
        Some(u) => format!("cited: {name} ({u}) — {id}"),
        None => format!("cited: {name} — {id}"),
    };
    maybe_share(store, cloud, &id, initiative, want_share, &mut msg).await?;
    Ok(text(&msg))
}
