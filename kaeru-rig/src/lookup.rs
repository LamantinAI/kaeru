//! Read side: exact recall, search, drill, trace, neighbours, tags, edges,
//! layers, and bi-temporal time-travel.
//!
//! Every tool here mirrors the daemon's verb of the same name, down to the
//! parameter names (#98) — `name`, not `name_or_id`; `when` as the same human
//! string `at` takes over MCP; and `initiative` on all of them, because a
//! per-call scope is how an agent reaches a second project without a second
//! memory handle.

use kaeru_core::{
    Layer, at, between, fuzzy_recall, history, neighbours, parse_when, read_node_full,
    recall_by_layer, recall_id_by_name, recollect_idea, recollect_outcome, recollect_provenance,
    summary_view, tagged, tags_like,
};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::{brief, briefs, in_chains, mem_tool_in, resolve};

#[derive(Debug, Deserialize)]
pub struct RecallArgs {
    pub name: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_recall` — exact-name lookup.
    Recall,
    "kaeru_recall",
    "Look up one memory by its EXACT name — the cheapest read there is, when you already know \
     what a thing is called. Returns the node's id and a short excerpt; `kaeru_at` reads it in \
     full. For fuzzy matching use `kaeru_search`. Pass `initiative` to resolve the name within a \
     specific project; omit for your default.",
    RecallArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "exact node name" },
        "initiative": { "type": "string", "description": "optional initiative (project) to resolve within; omit for your default" }
    }, "required": ["name"] },
    |store, args| match recall_id_by_name(store, &args.name) {
        Ok(Some(id)) => match kaeru_core::node_brief_by_id(store, &id) {
            Ok(Some(b)) => json!({ "found": true, "node": brief(&b) }),
            Ok(None) => json!({ "found": true, "id": id }),
            Err(e) => json!({ "error": e.to_string() }),
        },
        // An exact miss is where an agent concludes "memory is empty", so it
        // gets pointed at the verb that does not need the exact spelling.
        Ok(None) => json!({
            "found": false,
            "name": args.name,
            "hint": format!(
                "no node named `{}` — `kaeru_search` with \"{}*\" matches by prefix across \
                 name and body.",
                args.name, args.name
            ),
        }),
        Err(e) => json!({ "error": e.to_string() }),
    }
);

#[derive(Debug, Deserialize)]
pub struct SearchArgs {
    pub query: String,
    #[serde(default = "default_search_limit")]
    pub limit: usize,
    #[serde(default)]
    pub initiative: Option<String>,
}

/// The daemon's default, so the same query returns the same number of hits
/// whichever surface asked.
fn default_search_limit() -> usize {
    10
}

mem_tool_in!(
    /// `kaeru_search` — fuzzy full-text search across memory.
    Search,
    "kaeru_search",
    "Search long-term memory for what you've stored before (matches name + body; `word*` matches \
     by prefix). Returns name + short excerpt + id. Search before answering so you build on past \
     work; `kaeru_at` reads a hit in full. Pass `initiative` to search one project; omit for your \
     default — and note that a scoped search cannot see a sibling project's nodes.",
    SearchArgs,
    { "type": "object", "properties": {
        "query": { "type": "string", "description": "search terms; `word*` matches by prefix" },
        "limit": { "type": "integer", "description": "max results (default 10)" },
        "initiative": { "type": "string", "description": "optional initiative (project) to search; omit for your default" }
    }, "required": ["query"] },
    |store, args| match fuzzy_recall(store, &args.query, args.limit) {
        Ok(hits) if hits.is_empty() => json!({
            "results": [],
            "hint": format!(
                "no matches — widen it: query \"{}*\" (prefix match), kaeru_tagged with \
                 topic:<theme>, or kaeru_recent for what's fresh.",
                args.query
            ),
        }),
        Ok(hits) => json!({ "results": briefs(&hits) }),
        Err(e) => json!({ "error": e.to_string() }),
    }
);

#[derive(Debug, Deserialize)]
pub struct AtArgs {
    pub name: String,
    /// The moment to read: unix seconds, `2h` / `3d` ago, `2026-05-06`, or an
    /// RFC-3339 datetime. Omitted reads the node as it stands now.
    #[serde(default)]
    pub when: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_at` — read a node in full, now or as it stood at a past moment.
    At,
    "kaeru_at",
    "Read one memory IN FULL — the whole untruncated body and every field. Omit `when` for the \
     current version; pass `when` to time-travel: `2h`, `3d`, `2026-05-06`, an RFC-3339 datetime, \
     or raw unix seconds. `kaeru_history` shows when it changed. Use this after a search when an \
     excerpt isn't enough.",
    AtArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "node name or id" },
        "when": { "type": "string", "description": "moment to read: `2h`, `3d`, `2026-05-06`, RFC-3339, or unix seconds; omit for now" },
        "initiative": { "type": "string", "description": "optional initiative (project) to resolve within; omit for your default" }
    }, "required": ["name"] },
    |store, args| {
        let id = resolve(store, &args.name);
        let Some(when) = args.when.as_deref().filter(|w| !w.trim().is_empty()) else {
            return match read_node_full(store, &id) {
                Ok(Some(n)) => json!({
                    "id": n.id, "name": n.name, "type": n.node_type, "tier": n.tier,
                    "body": n.body, "tags": n.tags, "layer": n.layer,
                    "visibility": n.visibility, "in_chains": in_chains(store, &id)
                }),
                Ok(None) => json!({ "found": false, "query": args.name }),
                Err(e) => json!({ "error": e.to_string() }),
            };
        };
        let seconds = match parse_when(when) {
            Ok(s) => s,
            Err(e) => return json!({ "error": e.to_string() }),
        };
        match at(store, &id, seconds) {
            Ok(Some(s)) => json!({
                "name": s.name, "type": s.node_type, "tier": s.tier, "body": s.body,
                "tags": s.tags, "layer": s.layer, "visibility": s.visibility,
                "at_unix_seconds": seconds
            }),
            Ok(None) => json!({ "found": false, "query": args.name, "at": when }),
            Err(e) => json!({ "error": e.to_string() }),
        }
    }
);

/// The parameters of every verb that reads one node by name.
#[derive(Debug, Deserialize)]
pub struct NodeArgs {
    pub name: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_drill` — a node plus its one-hop `derived_from` / `part_of` tree.
    Drill,
    "kaeru_drill",
    "Drill into a node: returns it plus the children it was derived from or that are part of it, \
     as excerpts, each labelled with its edge type and direction. Follows `derived_from` and \
     `part_of` only — `kaeru_neighbours` follows every edge type.",
    NodeArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "node name or id" },
        "initiative": { "type": "string", "description": "optional initiative (project) to resolve within; omit for your default" }
    }, "required": ["name"] },
    |store, args| {
        let id = resolve(store, &args.name);
        match summary_view(store, &id) {
            Ok(view) => json!({
                "root": brief(&view.root),
                "children": briefs(
                    &view.children.iter().map(|c| c.brief.clone()).collect::<Vec<_>>()
                ),
                "in_chains": in_chains(store, &id)
            }),
            Err(e) => json!({ "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct NeighboursArgs {
    pub name: String,
    #[serde(default)]
    pub edge_type: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_neighbours` — everything one hop away, across all edge types.
    Neighbours,
    "kaeru_neighbours",
    "List every node ONE HOP from this one, in BOTH directions, across ALL edge types — the way \
     to discover what a memory is connected to. `kaeru_drill` follows only derived_from and \
     part_of, so a contradiction or a supersession shows up here when drill reports nothing. Each \
     line names the edge type and which way it points. Optional `edge_type` (comma-separated) \
     narrows it.",
    NeighboursArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "node name or id" },
        "edge_type": { "type": "string", "description": "optional comma-separated edge types to narrow to" },
        "initiative": { "type": "string", "description": "optional initiative (project) to resolve within; omit for your default" }
    }, "required": ["name"] },
    |store, args| {
        let id = resolve(store, &args.name);
        let filter: Option<Vec<String>> = args.edge_type.as_ref().map(|raw| {
            raw.split([',', ' '])
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
                .collect()
        });
        let types: Vec<kaeru_core::EdgeType> = match &filter {
            Some(raw) => {
                let mut parsed = Vec::new();
                for t in raw {
                    match t.parse::<kaeru_core::EdgeType>() {
                        Ok(et) => parsed.push(et),
                        Err(e) => return json!({ "error": format!("bad edge type `{t}`: {e}") }),
                    }
                }
                parsed
            }
            None => Vec::new(),
        };
        match neighbours(store, &id, &types) {
            Ok(found) => {
                let out: Vec<Value> = found
                    .iter()
                    .map(|n| json!({
                        "node": brief(&n.brief),
                        "edge_type": n.edge_type,
                        "outgoing": n.outgoing,
                    }))
                    .collect();
                json!({ "neighbours": out })
            }
            Err(e) => json!({ "error": e.to_string() }),
        }
    }
);

mem_tool_in!(
    /// `kaeru_trace` — walk `derived_from` provenance ancestors.
    Trace,
    "kaeru_trace",
    "Trace a memory's provenance: walks `derived_from` ancestors so you can see what a conclusion \
     was built on.",
    NodeArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "node name or id" },
        "initiative": { "type": "string", "description": "optional initiative (project) to resolve within; omit for your default" }
    }, "required": ["name"] },
    |store, args| {
        let id = resolve(store, &args.name);
        match recollect_provenance(store, &id) {
            Ok(chain) => json!({ "provenance": briefs(&chain) }),
            Err(e) => json!({ "error": e.to_string() }),
        }
    }
);

mem_tool_in!(
    /// `kaeru_history` — every bi-temporal revision of a node.
    History,
    "kaeru_history",
    "Show the bi-temporal revision history of a node: each assertion / retraction with its \
     timestamp, so you can see how a memory evolved.",
    NodeArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "node name or id" },
        "initiative": { "type": "string", "description": "optional initiative (project) to resolve within; omit for your default" }
    }, "required": ["name"] },
    |store, args| {
        let id = resolve(store, &args.name);
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

/// The parameters of a verb that reads a whole initiative.
#[derive(Debug, Deserialize)]
pub struct ScopeArgs {
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_ideas` — list archival ideas.
    Ideas,
    "kaeru_ideas",
    "List the archival `idea` nodes — settled, long-term thinking promoted out of operational work.",
    ScopeArgs,
    { "type": "object", "properties": {
        "initiative": { "type": "string", "description": "optional initiative (project) to read; omit for your default" }
    } },
    |store, _args| match recollect_idea(store) {
        Ok(v) => json!({ "ideas": briefs(&v) }),
        Err(e) => json!({ "error": e.to_string() }),
    }
);

mem_tool_in!(
    /// `kaeru_outcomes` — list archival outcomes.
    Outcomes,
    "kaeru_outcomes",
    "List the archival `outcome` nodes — durable results promoted out of operational work.",
    ScopeArgs,
    { "type": "object", "properties": {
        "initiative": { "type": "string", "description": "optional initiative (project) to read; omit for your default" }
    } },
    |store, _args| match recollect_outcome(store) {
        Ok(v) => json!({ "outcomes": briefs(&v) }),
        Err(e) => json!({ "error": e.to_string() }),
    }
);

#[derive(Debug, Deserialize)]
pub struct TaggedArgs {
    pub tag: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_tagged` — read every node carrying a tag.
    Tagged,
    "kaeru_tagged",
    "List every memory carrying an exact tag, e.g. `kind:experiment`, `sig:high`, `topic:auth`, \
     `status:open`, `lang:ru`. Tags use the exact stored form (no stemming); `topic:` tags are a \
     node's most-mentioned words, weighted toward a chosen name. A miss comes back with the near \
     tags that do exist rather than a bare empty list.",
    TaggedArgs,
    { "type": "object", "properties": {
        "tag": { "type": "string", "description": "exact tag, e.g. topic:auth" },
        "initiative": { "type": "string", "description": "optional initiative (project) to read; omit for your default" }
    }, "required": ["tag"] },
    |store, args| match tagged(store, &args.tag) {
        Ok(v) if v.is_empty() => {
            // Exact-match is right for a slice, but it makes every near miss
            // look like an empty vault — which is what taught agents to stop
            // reaching for this verb at all.
            let fragment = args.tag.split_once(':').map(|(_, v)| v).unwrap_or(&args.tag);
            let near: Vec<serde_json::Value> = tags_like(store, fragment)
                .unwrap_or_default()
                .into_iter()
                .filter(|(t, _)| *t != args.tag)
                .map(|(t, n)| json!({ "tag": t, "nodes": n }))
                .collect();
            json!({ "tagged": [], "near": near })
        }
        Ok(v) => json!({ "tagged": briefs(&v) }),
        Err(e) => json!({ "error": e.to_string() }),
    }
);

#[derive(Debug, Deserialize)]
pub struct BetweenArgs {
    pub a: String,
    pub b: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_between` — every edge connecting two nodes.
    Between,
    "kaeru_between",
    "Show every edge that connects two nodes (in either direction) — answers \"why are A and B \
     related?\".",
    BetweenArgs,
    { "type": "object", "properties": {
        "a": { "type": "string", "description": "first node name or id" },
        "b": { "type": "string", "description": "second node name or id" },
        "initiative": { "type": "string", "description": "optional initiative (project) to resolve within; omit for your default" }
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
    /// Layers to surface, comma- or space-separated — the same string the
    /// daemon takes. Defaults to `cold,frozen`: the archived material
    /// `kaeru_awake` does not load.
    #[serde(default)]
    pub layers: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_surface` — bring back archived layers on demand.
    Surface,
    "kaeru_surface",
    "Surface memories from specific importance layers on demand. Defaults to the archived layers \
     (cold + frozen) that `kaeru_awake` deliberately doesn't load. `layers` is a comma/space list \
     of: core, hot, warm, cold, frozen.",
    SurfaceArgs,
    { "type": "object", "properties": {
        "layers": { "type": "string", "description": "comma/space list of layer names (default `cold,frozen`)" },
        "initiative": { "type": "string", "description": "optional initiative (project) to read; omit for your default" }
    } },
    |store, args| {
        let spec = args.layers.as_deref().unwrap_or("").trim().to_string();
        let mut layers = Vec::new();
        for name in spec.split([',', ' ']).map(str::trim).filter(|s| !s.is_empty()) {
            match name.parse::<Layer>() {
                Ok(l) => layers.push(l),
                Err(e) => return json!({ "error": format!("bad layer `{name}`: {e}") }),
            }
        }
        if layers.is_empty() {
            layers = vec![Layer::Cold, Layer::Frozen];
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
