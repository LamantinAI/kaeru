//! Read side: search, drill, trace, archival recall, tags, edges, layers, and
//! bi-temporal time-travel.

use kaeru_core::{
    Layer, at, between, fuzzy_recall, history, read_node_full, recall_by_layer, recollect_idea,
    recollect_outcome, recollect_provenance, summary_view, tagged,
};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{brief, briefs, mem_tool, mem_tool_in, resolve};

#[derive(Debug, Deserialize)]
pub struct RecallArgs {
    pub query: String,
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_recall` — fuzzy full-text search across memory.
    Recall,
    "kaeru_recall",
    "Search long-term memory for what you've stored before (matches name + body; `word*` matches \
     by prefix). Returns name + short excerpt + id. Recall before answering so you build on past \
     work; use `kaeru_read` for a result's full body. Pass `initiative` to search a specific \
     project; omit for your default.",
    RecallArgs,
    { "type": "object", "properties": {
        "query": { "type": "string", "description": "search terms; `word*` matches by prefix" },
        "limit": { "type": "integer", "description": "max results (default 5)" },
        "initiative": { "type": "string", "description": "optional initiative (project) to search; omit for your default" }
    }, "required": ["query"] },
    |store, args| match fuzzy_recall(store, &args.query, args.limit.unwrap_or(5)) {
        Ok(hits) => json!({ "results": briefs(&hits) }),
        Err(e) => json!({ "error": e.to_string() }),
    }
);

#[derive(Debug, Deserialize)]
pub struct ReadArgs {
    pub name_or_id: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_read` — read one node in full (untruncated body + all fields).
    Read,
    "kaeru_read",
    "Read a single memory in full — the whole untruncated body and every field — by its name or \
     id. Use after `kaeru_recall` when an excerpt isn't enough. Pass `initiative` to resolve the \
     name within a specific project; omit for your default.",
    ReadArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "node name or id" },
        "initiative": { "type": "string", "description": "optional initiative (project) to resolve within; omit for your default" }
    }, "required": ["name_or_id"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
        match read_node_full(store, &id) {
            Ok(Some(n)) => json!({
                "id": n.id, "name": n.name, "type": n.node_type, "tier": n.tier,
                "body": n.body, "tags": n.tags, "layer": n.layer, "visibility": n.visibility
            }),
            Ok(None) => json!({ "found": false, "query": args.name_or_id }),
            Err(e) => json!({ "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct DrillArgs {
    pub name_or_id: String,
}

mem_tool!(
    /// `kaeru_drill` — a node plus its one-hop neighbours.
    Drill,
    "kaeru_drill",
    "Drill into a node: returns it plus its directly-connected neighbours (one hop) as excerpts. \
     The fast way to see what a memory is linked to.",
    DrillArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "node name or id" }
    }, "required": ["name_or_id"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
        match summary_view(store, &id) {
            Ok(view) => json!({ "root": brief(&view.root), "children": briefs(&view.children) }),
            Err(e) => json!({ "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct TraceArgs {
    pub name_or_id: String,
}

mem_tool!(
    /// `kaeru_trace` — walk `derived_from` provenance ancestors.
    Trace,
    "kaeru_trace",
    "Trace a memory's provenance: walks `derived_from` ancestors so you can see what a conclusion \
     was built on.",
    TraceArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "node name or id" }
    }, "required": ["name_or_id"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
        match recollect_provenance(store, &id) {
            Ok(chain) => json!({ "provenance": briefs(&chain) }),
            Err(e) => json!({ "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct NoArgs {}

mem_tool!(
    /// `kaeru_ideas` — list archival ideas.
    Ideas,
    "kaeru_ideas",
    "List the archival `idea` nodes — settled, long-term thinking promoted out of operational work.",
    NoArgs,
    { "type": "object", "properties": {} },
    |store, _args| match recollect_idea(store) {
        Ok(v) => json!({ "ideas": briefs(&v) }),
        Err(e) => json!({ "error": e.to_string() }),
    }
);

mem_tool!(
    /// `kaeru_outcomes` — list archival outcomes.
    Outcomes,
    "kaeru_outcomes",
    "List the archival `outcome` nodes — durable results promoted out of operational work.",
    NoArgs,
    { "type": "object", "properties": {} },
    |store, _args| match recollect_outcome(store) {
        Ok(v) => json!({ "outcomes": briefs(&v) }),
        Err(e) => json!({ "error": e.to_string() }),
    }
);

#[derive(Debug, Deserialize)]
pub struct TaggedArgs {
    pub tag: String,
}

mem_tool!(
    /// `kaeru_tagged` — read every node carrying a tag.
    Tagged,
    "kaeru_tagged",
    "List every memory carrying an exact tag, e.g. `kind:experiment`, `sig:high`, `topic:auth`, \
     `status:open`, `lang:ru`. Tags use the exact stored form (no stemming).",
    TaggedArgs,
    { "type": "object", "properties": {
        "tag": { "type": "string", "description": "exact tag, e.g. topic:auth" }
    }, "required": ["tag"] },
    |store, args| match tagged(store, &args.tag) {
        Ok(v) => json!({ "tagged": briefs(&v) }),
        Err(e) => json!({ "error": e.to_string() }),
    }
);

#[derive(Debug, Deserialize)]
pub struct BetweenArgs {
    pub a: String,
    pub b: String,
}

mem_tool!(
    /// `kaeru_between` — every edge connecting two nodes.
    Between,
    "kaeru_between",
    "Show every edge that connects two nodes (in either direction) — answers \"why are A and B \
     related?\".",
    BetweenArgs,
    { "type": "object", "properties": {
        "a": { "type": "string", "description": "first node name or id" },
        "b": { "type": "string", "description": "second node name or id" }
    }, "required": ["a", "b"] },
    |store, args| {
        let a = resolve(store, &args.a);
        let b = resolve(store, &args.b);
        match between(store, &a, &b) {
            Ok(edges) => {
                let out: Vec<Value> = edges
                    .iter()
                    .map(|e| json!({ "edge_type": e.edge_type, "a_to_b": e.a_to_b }))
                    .collect();
                json!({ "edges": out })
            }
            Err(e) => json!({ "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct SurfaceArgs {
    /// Layers to surface (core/hot/warm/cold/frozen). Defaults to cold + frozen
    /// — the archived material `kaeru_awake` does not load.
    #[serde(default)]
    pub layers: Option<Vec<String>>,
}

mem_tool!(
    /// `kaeru_surface` — bring back archived layers on demand.
    Surface,
    "kaeru_surface",
    "Surface memories from specific importance layers on demand. Defaults to the archived layers \
     (cold + frozen) that `kaeru_awake` deliberately doesn't load. Layers: core, hot, warm, cold, \
     frozen.",
    SurfaceArgs,
    { "type": "object", "properties": {
        "layers": { "type": "array", "items": { "type": "string" }, "description": "layer names (default cold, frozen)" }
    } },
    |store, args| {
        let raw = args.layers.unwrap_or_else(|| vec!["cold".into(), "frozen".into()]);
        let mut layers = Vec::new();
        for name in &raw {
            match name.parse::<Layer>() {
                Ok(l) => layers.push(l),
                Err(e) => return json!({ "error": format!("bad layer `{name}`: {e}") }),
            }
        }
        match recall_by_layer(store, &layers) {
            Ok(buckets) => {
                let out: Vec<Value> = buckets
                    .iter()
                    .map(|bk| json!({ "layer": bk.layer.as_str(), "nodes": briefs(&bk.nodes) }))
                    .collect();
                json!({ "layers": out })
            }
            Err(e) => json!({ "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct AtArgs {
    pub name_or_id: String,
    /// Unix timestamp (seconds) to view the node as it was at that moment.
    pub when_unix_seconds: f64,
}

mem_tool!(
    /// `kaeru_at` — time-travel: read a node as it was at a past moment.
    At,
    "kaeru_at",
    "Time-travel: read a node as it stood at a past moment, given a unix timestamp (seconds). Use \
     `kaeru_history` to see when it changed.",
    AtArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "node name or id" },
        "when_unix_seconds": { "type": "number", "description": "unix seconds of the moment to view" }
    }, "required": ["name_or_id", "when_unix_seconds"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
        match at(store, &id, args.when_unix_seconds) {
            Ok(Some(s)) => json!({
                "name": s.name, "type": s.node_type, "tier": s.tier, "body": s.body,
                "tags": s.tags, "layer": s.layer, "visibility": s.visibility
            }),
            Ok(None) => json!({ "found": false, "query": args.name_or_id, "at": args.when_unix_seconds }),
            Err(e) => json!({ "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct HistoryArgs {
    pub name_or_id: String,
}

mem_tool!(
    /// `kaeru_history` — every bi-temporal revision of a node.
    History,
    "kaeru_history",
    "Show the bi-temporal revision history of a node: each assertion / retraction with its \
     timestamp, so you can see how a memory evolved.",
    HistoryArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "node name or id" }
    }, "required": ["name_or_id"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
        match history(store, &id) {
            Ok(revs) => {
                let out: Vec<Value> = revs
                    .iter()
                    .map(|r| json!({ "seconds": r.seconds, "asserted": r.asserted, "name": r.name }))
                    .collect();
                json!({ "revisions": out })
            }
            Err(e) => json!({ "error": e.to_string() }),
        }
    }
);
