//! Role slots: a role an initiative fills with exactly one live node.
//!
//! The daemon's `slot` / `slots` / `unslot`, verb for verb (#98). A slot is
//! the structural answer to "which handoff is the current one" — filling it
//! archives the previous holder to `cold` and links `supersedes` from the new
//! holder to it, so a project cannot drift into three current handoffs.
//!
//! These name their initiative outright rather than taking the ambient one:
//! a role belongs to a project, and picking the wrong project silently is the
//! one mistake that cannot be seen in the result.

use kaeru_core::{node_brief_by_id, occupy_slot, release_slot, slots_in};
use serde::Deserialize;
use serde_json::json;

use crate::{mem_tool_named, resolve};

#[derive(Debug, Deserialize)]
pub struct SlotArgs {
    pub initiative: String,
    pub slot: String,
    pub name: String,
}

mem_tool_named!(
    /// `kaeru_slot` — make a node the live holder of a role.
    Slot,
    "kaeru_slot",
    "Make a node the live holder of a ROLE in an initiative — `handoff`, `entrypoint`, `queue`, \
     `prod-state`. A role holds exactly one node: taking it archives the previous holder to \
     `cold` and links `supersedes` from the new holder to it, so a project can never end up with \
     three current handoffs. Nothing is deleted; the predecessor stays readable via `kaeru_at` \
     and `kaeru_surface`.",
    SlotArgs,
    { "type": "object", "properties": {
        "initiative": { "type": "string", "description": "initiative (project) the role belongs to" },
        "slot": { "type": "string", "description": "role name, e.g. handoff / entrypoint / queue" },
        "name": { "type": "string", "description": "node name or id to put in the role" }
    }, "required": ["initiative", "slot", "name"] },
    |store, args| {
        let node_id = resolve(store, &args.name);
        match occupy_slot(store, &args.initiative, &args.slot, &node_id) {
            Ok(outcome) => {
                let previous = outcome.previous.map(|prev| {
                    node_brief_by_id(store, &prev)
                        .ok()
                        .flatten()
                        .map(|b| b.name)
                        .unwrap_or(prev)
                });
                json!({
                    "slot": args.slot,
                    "initiative": args.initiative,
                    "holder": args.name,
                    "id": node_id,
                    // Named, not just counted: the predecessor left the
                    // working set, and a result that does not say so reads as
                    // if nothing else moved.
                    "previous_holder": previous,
                })
            }
            Err(e) => json!({ "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct SlotsArgs {
    pub initiative: String,
}

mem_tool_named!(
    /// `kaeru_slots` — the filled roles of an initiative.
    Slots,
    "kaeru_slots",
    "List the filled roles of an initiative and which node holds each.",
    SlotsArgs,
    { "type": "object", "properties": {
        "initiative": { "type": "string", "description": "initiative (project) to list roles for" }
    }, "required": ["initiative"] },
    |store, args| match slots_in(store, &args.initiative) {
        Ok(filled) if filled.is_empty() => json!({
            "slots": [],
            "hint": "no roles filled — a slot is a role held by exactly one live node \
                     (`handoff`, `entrypoint`, `queue`). `kaeru_slot` fills one, and each new \
                     holder archives its predecessor.",
        }),
        Ok(filled) => json!({
            "slots": filled
                .iter()
                .map(|(role, node_id)| {
                    let name = node_brief_by_id(store, node_id)
                        .ok()
                        .flatten()
                        .map(|b| b.name)
                        .unwrap_or_else(|| node_id.clone());
                    json!({ "slot": role, "holder": name, "id": node_id })
                })
                .collect::<Vec<_>>(),
        }),
        Err(e) => json!({ "error": e.to_string() }),
    }
);

#[derive(Debug, Deserialize)]
pub struct UnslotArgs {
    pub initiative: String,
    pub slot: String,
}

mem_tool_named!(
    /// `kaeru_unslot` — free a role without touching the node that held it.
    Unslot,
    "kaeru_unslot",
    "Free a role without touching the node that held it — its layer stays as it is.",
    UnslotArgs,
    { "type": "object", "properties": {
        "initiative": { "type": "string", "description": "initiative (project) the role belongs to" },
        "slot": { "type": "string", "description": "role to free" }
    }, "required": ["initiative", "slot"] },
    |store, args| match release_slot(store, &args.initiative, &args.slot) {
        Ok(Some(previous)) => {
            let name = node_brief_by_id(store, &previous)
                .ok()
                .flatten()
                .map(|b| b.name)
                .unwrap_or_else(|| previous.clone());
            json!({ "freed": true, "slot": args.slot, "was_held_by": name, "id": previous })
        }
        Ok(None) => json!({ "freed": false, "slot": args.slot, "reason": "the role was empty" }),
        Err(e) => json!({ "error": e.to_string() }),
    }
);
