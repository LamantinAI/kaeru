//! Graph metabolism: consolidation across tiers, supersession, forgetting,
//! revision, and layer re-filing.

use kaeru_core::{
    Layer, NodeType, Tier, consolidate_in, consolidate_out, forget, improve, node_brief_by_id,
    set_layer, supersedes, synthesise,
};
use serde::Deserialize;
use serde_json::json;

use crate::{mem_tool, resolve};

#[derive(Debug, Deserialize)]
pub struct SettleArgs {
    pub name_or_id: String,
    #[serde(default)]
    pub as_type: Option<String>,
    pub name: String,
    pub body: String,
}

mem_tool!(
    /// `kaeru_settle` — promote an operational draft to archival (keeps provenance).
    Settle,
    "kaeru_settle",
    "Promote an operational draft to the archival tier as settled knowledge, preserving a \
     `derived_from` link to the original. `as_type` is the archival type (idea, outcome, concept, \
     entity, summary); defaults to idea.",
    SettleArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "operational node name or id" },
        "as_type": { "type": "string", "description": "archival type (default idea)" },
        "name": { "type": "string", "description": "name for the settled node" },
        "body": { "type": "string", "description": "settled, stable form of the content" }
    }, "required": ["name_or_id", "name", "body"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
        match args.as_type.as_deref().unwrap_or("idea").parse::<NodeType>() {
            Ok(ty) => match consolidate_out(store, &id, ty, &args.name, &args.body) {
                Ok(new_id) => json!({ "settled": true, "id": new_id }),
                Err(e) => json!({ "settled": false, "error": e.to_string() }),
            },
            Err(e) => json!({ "settled": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct ReopenArgs {
    pub name_or_id: String,
    #[serde(default)]
    pub as_type: Option<String>,
    pub name: String,
    pub body: String,
}

mem_tool!(
    /// `kaeru_reopen` — bring archival knowledge back to operational for revision.
    Reopen,
    "kaeru_reopen",
    "Bring an archival node back into the operational tier for active revision, preserving \
     provenance. `as_type` is the operational type (draft, scratch, …); defaults to draft.",
    ReopenArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "archival node name or id" },
        "as_type": { "type": "string", "description": "operational type (default draft)" },
        "name": { "type": "string", "description": "name for the reopened node" },
        "body": { "type": "string", "description": "working form of the content" }
    }, "required": ["name_or_id", "name", "body"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
        match args.as_type.as_deref().unwrap_or("draft").parse::<NodeType>() {
            Ok(ty) => match consolidate_in(store, &id, ty, &args.name, &args.body) {
                Ok(new_id) => json!({ "reopened": true, "id": new_id }),
                Err(e) => json!({ "reopened": false, "error": e.to_string() }),
            },
            Err(e) => json!({ "reopened": false, "error": e.to_string() }),
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
     links the new with a `supersedes` edge. `as_type` defaults to draft, `tier` to operational.",
    SupersedeArgs,
    { "type": "object", "properties": {
        "old": { "type": "string", "description": "node name or id to supersede" },
        "as_type": { "type": "string", "description": "new node type (default draft)" },
        "tier": { "type": "string", "description": "operational | archival (default operational)" },
        "name": { "type": "string", "description": "name for the new version" },
        "body": { "type": "string", "description": "the new content" }
    }, "required": ["old", "name", "body"] },
    |store, args| {
        let old = resolve(store, &args.old);
        let ty = match args.as_type.as_deref().unwrap_or("draft").parse::<NodeType>() {
            Ok(t) => t,
            Err(e) => return json!({ "superseded": false, "error": e.to_string() }),
        };
        let tier = match args.tier.as_deref().unwrap_or("operational").parse::<Tier>() {
            Ok(t) => t,
            Err(e) => return json!({ "superseded": false, "error": e.to_string() }),
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
                Ok(()) => json!({ "relayered": true, "id": id, "layer": l.as_str() }),
                Err(e) => json!({ "relayered": false, "error": e.to_string() }),
            },
            Err(e) => json!({ "relayered": false, "error": e.to_string() }),
        }
    }
);
