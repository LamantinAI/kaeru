//! Capture & connect: write thoughts, references, tasks, and typed links.

use kaeru_core::{
    EdgeType, EpisodeKind, Significance, cite, complete_task, jot, link_with_weight, unlink,
    write_episode, write_task,
};
use serde::Deserialize;
use serde_json::json;

use crate::{mem_tool, mem_tool_in, resolve};

#[derive(Debug, Deserialize)]
pub struct RememberArgs {
    pub body: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_remember` — save a thought to long-term memory.
    Remember,
    "kaeru_remember",
    "Save a thought to long-term memory so it survives across sessions. Give a `name` to recall \
     it by exact name later (a decision, a load-bearing fact); omit `name` for a quick auto-named \
     note. Pass `initiative` to file it under a specific project (e.g. finances, studies); omit \
     for your default initiative.",
    RememberArgs,
    { "type": "object", "properties": {
        "body": { "type": "string", "description": "the thought/fact to remember" },
        "name": { "type": "string", "description": "optional deliberate name to recall it by" },
        "initiative": { "type": "string", "description": "optional initiative (project) to file this under; omit for your default" }
    }, "required": ["body"] },
    |store, args| {
        let saved = match &args.name {
            Some(name) => {
                write_episode(store, EpisodeKind::Observation, Significance::Medium, name, &args.body)
            }
            None => jot(store, &args.body),
        };
        match saved {
            Ok(id) => json!({ "saved": true, "id": id }),
            Err(e) => json!({ "saved": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct CiteArgs {
    pub name: String,
    #[serde(default)]
    pub url: Option<String>,
    pub body: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_cite` — record an external source or a persona/entity.
    Cite,
    "kaeru_cite",
    "Record a long-term reference: an external source (pass `url` for a paper / gist / dashboard) \
     or a persona / entity (skip `url` for a person, place, or book). Lands in the archival tier. \
     Pass `initiative` to file it under a specific project; omit for your default.",
    CiteArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "name of the source / entity" },
        "url": { "type": "string", "description": "canonical URL (omit for a persona/entity)" },
        "body": { "type": "string", "description": "what it is / why it matters" },
        "initiative": { "type": "string", "description": "optional initiative (project) to file this under; omit for your default" }
    }, "required": ["name", "body"] },
    |store, args| match cite(store, &args.name, args.url.as_deref(), &args.body) {
        Ok(id) => json!({ "saved": true, "id": id }),
        Err(e) => json!({ "saved": false, "error": e.to_string() }),
    }
);

#[derive(Debug, Deserialize)]
pub struct LinkArgs {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub edge_type: Option<String>,
    #[serde(default)]
    pub weight: Option<f64>,
}

mem_tool!(
    /// `kaeru_link` — connect two memories with a typed, weighted edge.
    Link,
    "kaeru_link",
    "Connect two memories with a typed link so later recall can follow the reasoning trail \
     between them. `weight` (0..1) is how strong the connection is — strong links make shorter \
     knowledge chains. Edge types: refers_to, causal, derived_from, contradicts, part_of, blocks, \
     targets, supersedes, verifies, falsifies, temporal.",
    LinkArgs,
    { "type": "object", "properties": {
        "from": { "type": "string", "description": "source node name or id" },
        "to": { "type": "string", "description": "destination node name or id" },
        "edge_type": { "type": "string", "description": "link type (default refers_to)" },
        "weight": { "type": "number", "description": "strength 0..1 (default 1.0)" }
    }, "required": ["from", "to"] },
    |store, args| {
        let from = resolve(store, &args.from);
        let to = resolve(store, &args.to);
        let weight = args.weight.unwrap_or(1.0).clamp(0.0, 1.0);
        match args.edge_type.as_deref().unwrap_or("refers_to").parse::<EdgeType>() {
            Ok(et) => match link_with_weight(store, &from, &to, et, weight) {
                Ok(()) => json!({
                    "linked": true, "from": from, "to": to,
                    "edge_type": et.as_str(), "weight": weight
                }),
                Err(e) => json!({ "linked": false, "error": e.to_string() }),
            },
            Err(e) => json!({ "linked": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct UnlinkArgs {
    pub from: String,
    pub to: String,
    #[serde(default)]
    pub edge_type: Option<String>,
}

mem_tool!(
    /// `kaeru_unlink` — retract an edge (bi-temporal; history kept).
    Unlink,
    "kaeru_unlink",
    "Retract a previously-asserted edge between two nodes. Bi-temporal — historical reads still \
     see it; only reads at NOW skip it.",
    UnlinkArgs,
    { "type": "object", "properties": {
        "from": { "type": "string", "description": "source node name or id" },
        "to": { "type": "string", "description": "destination node name or id" },
        "edge_type": { "type": "string", "description": "link type (default refers_to)" }
    }, "required": ["from", "to"] },
    |store, args| {
        let from = resolve(store, &args.from);
        let to = resolve(store, &args.to);
        match args.edge_type.as_deref().unwrap_or("refers_to").parse::<EdgeType>() {
            Ok(et) => match unlink(store, &from, &to, et) {
                Ok(()) => json!({ "unlinked": true, "from": from, "to": to, "edge_type": et.as_str() }),
                Err(e) => json!({ "unlinked": false, "error": e.to_string() }),
            },
            Err(e) => json!({ "unlinked": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct TaskArgs {
    pub body: String,
    #[serde(default)]
    pub due: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_task` — record a todo with an optional deadline.
    Task,
    "kaeru_task",
    "Record a task / todo that should survive into the next session. `due` is an ISO date-time \
     (e.g. 2026-07-01T09:00:00Z). Open tasks resurface via `kaeru_awake`. Pass `initiative` to \
     file it under a specific project; omit for your default.",
    TaskArgs,
    { "type": "object", "properties": {
        "body": { "type": "string", "description": "what needs doing" },
        "due": { "type": "string", "description": "optional ISO deadline" },
        "initiative": { "type": "string", "description": "optional initiative (project) to file this under; omit for your default" }
    }, "required": ["body"] },
    |store, args| match write_task(store, &args.body, args.due.as_deref()) {
        Ok(id) => json!({ "created": true, "id": id }),
        Err(e) => json!({ "created": false, "error": e.to_string() }),
    }
);

#[derive(Debug, Deserialize)]
pub struct DoneArgs {
    pub name_or_id: String,
}

mem_tool!(
    /// `kaeru_done` — mark a task complete.
    Done,
    "kaeru_done",
    "Mark a task complete (sets its status to done). Pass the task's name or id.",
    DoneArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "task name or id" }
    }, "required": ["name_or_id"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
        match complete_task(store, &id) {
            Ok(()) => json!({ "done": true, "id": id }),
            Err(e) => json!({ "done": false, "error": e.to_string() }),
        }
    }
);
