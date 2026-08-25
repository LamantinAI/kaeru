//! Graph metabolism: consolidation across tiers, supersession, forgetting,
//! revision, and layer re-filing.

use kaeru_core::{
    Layer, NodeType, Store, Tier, consolidate_in, consolidate_out, forget, improve,
    node_brief_by_id, read_node_full, set_layer, supersedes, synthesise,
};
use serde::Deserialize;
use serde_json::json;

use crate::{mem_tool, resolve};

/// What a promote-in-place inherits from the node it replaces: the type it
/// should become, its current name, and its full body.
///
/// Consolidation used to demand all three re-authored on every call, and the
/// price is why the tier model went unused — four outcomes across 1245 nodes,
/// while the same finished work got demoted to a cold layer instead. `derive`
/// turns the node's own type into the successor's default: `settled_form` on
/// the way out, identity on the way back in.
fn inherited(
    store: &Store,
    id: &str,
    asked_type: Option<&str>,
    derive: impl Fn(NodeType) -> NodeType,
) -> Result<(NodeType, String, String), String> {
    let full = read_node_full(store, &id.to_string())
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("no node {id:?} at NOW"))?;
    let ty = match asked_type {
        Some(t) => t.parse::<NodeType>().map_err(|e| e.to_string())?,
        None => derive(
            full.node_type
                .parse::<NodeType>()
                .map_err(|e| e.to_string())?,
        ),
    };
    Ok((ty, full.name, full.body.unwrap_or_default()))
}

#[derive(Debug, Deserialize)]
pub struct SettleArgs {
    pub name_or_id: String,
    #[serde(default)]
    pub as_type: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
}

mem_tool!(
    /// `kaeru_settle` — promote an operational draft to archival (keeps provenance).
    Settle,
    "kaeru_settle",
    "Promote a node that stopped changing into the archival tier as settled knowledge, \
     preserving a `derived_from` link to the original. The name alone is enough: with no other \
     argument the node keeps its name and full body, and the type is derived (episode/task/\
     experiment/hypothesis -> outcome; draft/scratch -> idea). Don't demote finished work to a \
     cold layer instead — a layer is how eagerly a node loads, a tier is whether it is still in \
     flight.",
    SettleArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "node name or id" },
        "as_type": { "type": "string", "description": "optional archival type; derived from the node when omitted" },
        "name": { "type": "string", "description": "optional new name; the node's own is kept when omitted" },
        "body": { "type": "string", "description": "optional new body; the node's own is kept when omitted" }
    }, "required": ["name_or_id"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
        match inherited(store, &id, args.as_type.as_deref(), |t| t.settled_form()) {
            Ok((ty, name, body)) => {
                let name = args.name.as_deref().unwrap_or(&name);
                let body = args.body.as_deref().unwrap_or(&body);
                match consolidate_out(store, &id, ty, name, body) {
                    Ok(new_id) => json!({
                        "settled": true, "id": new_id, "name": name, "type": ty.as_str()
                    }),
                    Err(e) => json!({ "settled": false, "error": e.to_string() }),
                }
            }
            Err(e) => json!({ "settled": false, "error": e }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct UnsettleArgs {
    pub name_or_id: String,
    #[serde(default)]
    pub as_type: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
}

mem_tool!(
    /// `kaeru_unsettle` — bring archival knowledge back to operational for revision.
    Unsettle,
    "kaeru_unsettle",
    "Bring an archival node back into the operational tier for active revision — `kaeru_settle`'s \
     mirror, for settled knowledge that turned out to still be in flight. The name alone is \
     enough: name, body and type all carry over unless you say otherwise.",
    UnsettleArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "archival node name or id" },
        "as_type": { "type": "string", "description": "optional operational type; the node's own is kept when omitted" },
        "name": { "type": "string", "description": "optional new name; the node's own is kept when omitted" },
        "body": { "type": "string", "description": "optional new body; the node's own is kept when omitted" }
    }, "required": ["name_or_id"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
        match inherited(store, &id, args.as_type.as_deref(), |t| t) {
            Ok((ty, name, body)) => {
                let name = args.name.as_deref().unwrap_or(&name);
                let body = args.body.as_deref().unwrap_or(&body);
                match consolidate_in(store, &id, ty, name, body) {
                    Ok(new_id) => json!({
                        "unsettled": true, "id": new_id, "name": name, "type": ty.as_str()
                    }),
                    Err(e) => json!({ "unsettled": false, "error": e.to_string() }),
                }
            }
            Err(e) => json!({ "unsettled": false, "error": e }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct SynthesiseArgs {
    pub from: Vec<String>,
    #[serde(default)]
    pub as_type: Option<String>,
    #[serde(default)]
    pub tier: Option<String>,
    pub name: String,
    pub body: String,
}

mem_tool!(
    /// `kaeru_synthesise` — combine several nodes into one (many-to-one).
    Synthesise,
    "kaeru_synthesise",
    "Combine several memories into one synthesised node, with a `derived_from` edge from each \
     source. `as_type` defaults to summary, `tier` to archival.",
    SynthesiseArgs,
    { "type": "object", "properties": {
        "from": { "type": "array", "items": { "type": "string" }, "description": "source node names or ids" },
        "as_type": { "type": "string", "description": "result type (default summary)" },
        "tier": { "type": "string", "description": "operational | archival (default archival)" },
        "name": { "type": "string", "description": "name for the synthesised node" },
        "body": { "type": "string", "description": "the combined content" }
    }, "required": ["from", "name", "body"] },
    |store, args| {
        let seeds: Vec<String> = args.from.iter().map(|s| resolve(store, s)).collect();
        let ty = match args.as_type.as_deref().unwrap_or("summary").parse::<NodeType>() {
            Ok(t) => t,
            Err(e) => return json!({ "synthesised": false, "error": e.to_string() }),
        };
        let tier = match args.tier.as_deref().unwrap_or("archival").parse::<Tier>() {
            Ok(t) => t,
            Err(e) => return json!({ "synthesised": false, "error": e.to_string() }),
        };
        match synthesise(store, &seeds, ty, tier, &args.name, &args.body) {
            Ok(id) => json!({ "synthesised": true, "id": id }),
            Err(e) => json!({ "synthesised": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct SupersedeArgs {
    pub old: String,
    #[serde(default)]
    pub as_type: Option<String>,
    #[serde(default)]
    pub tier: Option<String>,
    pub name: String,
    pub body: String,
}

mem_tool!(
    /// `kaeru_supersede` — replace a node with a new version (old retracted).
    Supersede,
    "kaeru_supersede",
    "Create a new version that supersedes an old node — bi-temporally retracts the old one and \
     links the new with a `supersedes` edge. `as_type` defaults to the old node's own type — \
     replacing a node rarely changes what kind of thing it is. `tier` defaults from that type.",
    SupersedeArgs,
    { "type": "object", "properties": {
        "old": { "type": "string", "description": "node name or id to supersede" },
        "as_type": { "type": "string", "description": "optional new node type; the old node's own is kept when omitted" },
        "tier": { "type": "string", "description": "operational | archival (defaults from the type)" },
        "name": { "type": "string", "description": "name for the new version" },
        "body": { "type": "string", "description": "the new content" }
    }, "required": ["old", "name", "body"] },
    |store, args| {
        let old = resolve(store, &args.old);
        let ty = match inherited(store, &old, args.as_type.as_deref(), |t| t) {
            Ok((t, _, _)) => t,
            Err(e) => return json!({ "superseded": false, "error": e }),
        };
        let tier = match args.tier.as_deref() {
            Some(t) => match t.parse::<Tier>() {
                Ok(t) => t,
                Err(e) => return json!({ "superseded": false, "error": e.to_string() }),
            },
            None => ty.default_tier(),
        };
        match supersedes(store, &old, ty, tier, &args.name, &args.body) {
            Ok(id) => json!({ "superseded": true, "id": id }),
            Err(e) => json!({ "superseded": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct ForgetArgs {
    pub name_or_id: String,
}

mem_tool!(
    /// `kaeru_forget` — bi-temporal forget (retracts node + edges, history kept).
    Forget,
    "kaeru_forget",
    "Forget a memory: retracts the node and its connected edges at NOW. Bi-temporal — the history \
     is preserved, so `kaeru_at` at a past time still sees it.",
    ForgetArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "node name or id" }
    }, "required": ["name_or_id"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
        match forget(store, &id) {
            Ok(()) => json!({ "forgotten": true, "id": id }),
            Err(e) => json!({ "forgotten": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct ReviseArgs {
    pub name_or_id: String,
    #[serde(default)]
    pub name: Option<String>,
    pub body: String,
}

mem_tool!(
    /// `kaeru_revise` — rewrite a node's body (and optionally rename) in place.
    Revise,
    "kaeru_revise",
    "Rewrite a memory's body, keeping its id. Pass `name` to also rename it; omit to keep the \
     current name.",
    ReviseArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "node name or id" },
        "name": { "type": "string", "description": "optional new name (keeps current if omitted)" },
        "body": { "type": "string", "description": "the new body" }
    }, "required": ["name_or_id", "body"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
        let new_name = match args.name {
            Some(n) => n,
            None => match node_brief_by_id(store, &id) {
                Ok(Some(b)) => b.name,
                Ok(None) => return json!({ "revised": false, "error": "node not found" }),
                Err(e) => return json!({ "revised": false, "error": e.to_string() }),
            },
        };
        match improve(store, &id, &new_name, &args.body) {
            Ok(()) => json!({ "revised": true, "id": id, "name": new_name }),
            Err(e) => json!({ "revised": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct LayerArgs {
    pub name_or_id: String,
    pub layer: String,
}

mem_tool!(
    /// `kaeru_layer` — re-file a node into a memory layer.
    SetLayer,
    "kaeru_layer",
    "Set a memory's importance layer (core, hot, warm, cold, frozen) — controls how eagerly it \
     loads on re-entry. core/hot/warm load via `kaeru_awake`; cold/frozen are archived.",
    LayerArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "node name or id" },
        "layer": { "type": "string", "description": "core | hot | warm | cold | frozen" }
    }, "required": ["name_or_id", "layer"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
        match args.layer.parse::<Layer>() {
            Ok(l) => match set_layer(store, &id, l) {
                Ok(()) => {
                    let mut out = json!({ "relayered": true, "id": id, "layer": l.as_str() });
                    // cold/frozen are exactly the layers `kaeru_awake` doesn't
                    // load — say which verb reaches the node again.
                    if matches!(l, Layer::Cold | Layer::Frozen) {
                        out["hint"] = json!(
                            "out of the re-entry view now — kaeru_surface with layers \
                             cold,frozen reads it back"
                        );
                    }
                    out
                }
                Err(e) => json!({ "relayered": false, "error": e.to_string() }),
            },
            Err(e) => json!({ "relayered": false, "error": e.to_string() }),
        }
    }
);
