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
pub struct ChainsArgs {
    pub name_or_id: String,
}

mem_tool!(
    /// `kaeru_chains` — which chains a node belongs to.
    Chains,
    "kaeru_chains",
    "List the knowledge chains a node belongs to. When a single memory is context-poor, see its \
     chains and `kaeru_read_chain` the relevant one.",
    ChainsArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "node name or id" }
    }, "required": ["name_or_id"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
        match chains_of(store, &id) {
            Ok(v) => json!({ "chains": briefs(&v) }),
            Err(e) => json!({ "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct ReadChainArgs {
    pub name_or_id: String,
}

mem_tool!(
    /// `kaeru_read_chain` — read a chain's ordered members.
    ReadChain,
    "kaeru_read_chain",
    "Read a knowledge chain's ordered members in full — the connected reasoning trail, instead of \
     an isolated node.",
    ReadChainArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "chain name or id" }
    }, "required": ["name_or_id"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
        match read_chain(store, &id) {
            Ok(v) => json!({ "trail": briefs(&v) }),
            Err(e) => json!({ "error": e.to_string() }),
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
