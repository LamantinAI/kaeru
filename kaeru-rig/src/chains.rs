//! Knowledge chains: materialize and read the strongest weighted reasoning
//! trail between two memories.

use kaeru_core::{
    chains_of, create_chain, extend_chain, read_chain, regenerate_chain, shortest_path,
};
use serde::Deserialize;
use serde_json::json;

use crate::{briefs, briefs_by_ids, mem_tool, resolve};

#[derive(Debug, Deserialize)]
pub struct ChainArgs {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
}

mem_tool!(
    /// `kaeru_chain` — save the strongest path between two nodes as a chain.
    Chain,
    "kaeru_chain",
    "Save the strongest weighted path between two memories as a reusable knowledge chain — an \
     ordered, recallable reasoning trail. Stronger links (see `kaeru_link` weight) make shorter \
     paths. Pass `summary` to note why the trail matters (labels it for later triage). Idempotent \
     — an identical chain is reused, not duplicated. Reports if the two are unconnected.",
    ChainArgs,
    { "type": "object", "properties": {
        "from": { "type": "string", "description": "start node name or id" },
        "to": { "type": "string", "description": "end node name or id" },
        "name": { "type": "string", "description": "optional name for the chain" },
        "summary": { "type": "string", "description": "optional one-line note on why this trail matters" }
    }, "required": ["from", "to"] },
    |store, args| {
        let from = resolve(store, &args.from);
        let to = resolve(store, &args.to);
        match create_chain(store, &from, &to, args.name.as_deref(), args.summary.as_deref()) {
            Ok(Some(o)) => json!({ "chained": true, "chain_id": o.id, "reused": o.reused }),
            Ok(None) => json!({ "chained": false, "reason": "no path between the two" }),
            Err(e) => json!({ "chained": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct WhyArgs {
    pub name_or_id: String,
}

mem_tool!(
    /// `kaeru_why` — the saved reasoning that leads to a node.
    ///
    /// Replaces the former `kaeru_chains` + `kaeru_read_chain` pair, which had
    /// no organic calls between them: the only advert for reading a trail lived
    /// in the output of listing them, and nobody called that either. One verb,
    /// one entry point, and a name that says what a chain is for.
    Why,
    "kaeru_why",
    "Why is this here? Reads the saved reasoning leading to a memory — the state → reasoning → \
     decision trail, not an isolated record. Give it a chain to read its ordered steps, or any \
     node to see the chain it belongs to (read directly when there is only one, listed when \
     there are several).",
    WhyArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "a chain, or any node in one" }
    }, "required": ["name_or_id"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
        let is_chain = kaeru_core::node_brief_by_id(store, &id)
            .ok()
            .flatten()
            .is_some_and(|b| b.node_type == "chain");
        if is_chain {
            return match read_chain(store, &id) {
                Ok(v) => json!({ "chain": args.name_or_id, "trail": briefs(&v) }),
                Err(e) => json!({ "error": e.to_string() }),
            };
        }
        match chains_of(store, &id) {
            Err(e) => json!({ "error": e.to_string() }),
            Ok(v) if v.is_empty() => json!({
                "chains": [],
                "hint": "no saved reasoning leads here yet — `kaeru_chain from to` saves a trail"
            }),
            // One chain is the answer, not a menu: read it rather than making
            // the caller spend another turn on the only possible choice.
            Ok(v) if v.len() == 1 => match read_chain(store, &v[0].id) {
                Ok(t) => json!({ "chain": v[0].name, "trail": briefs(&t) }),
                Err(e) => json!({ "error": e.to_string() }),
            },
            Ok(v) => json!({ "chains": briefs(&v) }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct RechainArgs {
    pub chain: String,
    #[serde(default)]
    pub to: Option<String>,
}

mem_tool!(
    /// `kaeru_rechain` — regenerate or extend a chain after graph changes.
    Rechain,
    "kaeru_rechain",
    "Refresh a chain the graph has outgrown. With no `to`, regenerate it (recompute the shortest \
     path between its current endpoints). With `to`, extend the trail out to that node. Keeps the \
     chain's id, name, and summary.",
    RechainArgs,
    { "type": "object", "properties": {
        "chain": { "type": "string", "description": "chain name or id" },
        "to": { "type": "string", "description": "omit to regenerate; node name/id to extend to" }
    }, "required": ["chain"] },
    |store, args| {
        let cid = resolve(store, &args.chain);
        let result = match &args.to {
            Some(t) => {
                let to = resolve(store, t);
                extend_chain(store, &cid, &to)
            }
            None => regenerate_chain(store, &cid),
        };
        match result {
            Ok(Some(s)) => json!({ "ok": true, "members": s.members, "changed": s.changed }),
            Ok(None) => json!({ "ok": false, "reason": "endpoint unreachable — chain unchanged" }),
            Err(e) => json!({ "ok": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct PathArgs {
    pub from: String,
    pub to: String,
}

mem_tool!(
    /// `kaeru_path` — preview the strongest path without saving it.
    Path,
    "kaeru_path",
    "Compute the strongest weighted path between two memories WITHOUT saving it (preview). Use \
     `kaeru_chain` to persist one.",
    PathArgs,
    { "type": "object", "properties": {
        "from": { "type": "string", "description": "start node name or id" },
        "to": { "type": "string", "description": "end node name or id" }
    }, "required": ["from", "to"] },
    |store, args| {
        let from = resolve(store, &args.from);
        let to = resolve(store, &args.to);
        match shortest_path(store, &from, &to) {
            Ok(ids) if ids.is_empty() => json!({ "path": [], "reason": "no path between the two" }),
            Ok(ids) => json!({ "path": briefs_by_ids(store, &ids) }),
            Err(e) => json!({ "error": e.to_string() }),
        }
    }
);
