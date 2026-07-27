//! The two purely-local cloud tools — no network, no client needed.
//!
//! - `policy` — read / set an initiative's sticky `share_policy` (sharing Gate 1).
//! - `sync_review` — split a team initiative's still-local nodes into
//!   propose-share (guard-clean) vs keep-local (secret-guard flagged). It
//!   proposes; it never pushes.

use std::str::FromStr;

use kaeru_core::{SharePolicy, guard};
use serde::Deserialize;
use serde_json::{Value, json};

use super::format_hit;
use crate::{KaeruMemory, mem_tool_cloud, target_initiative};

// ─────────────────────────────────────────────────────────────────────────────
// policy (local) — read / set an initiative's share_policy (Gate 1).
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PolicyArgs {
    #[serde(default)]
    pub initiative: Option<String>,
    /// Omit to read; `private` / `team` / `ask` to set.
    #[serde(default)]
    pub policy: Option<String>,
}

async fn do_policy(mem: &KaeruMemory, a: PolicyArgs) -> Value {
    let Some(init) = target_initiative(mem, &a.initiative) else {
        return json!({ "error": "no initiative — scope the memory or pass `initiative`" });
    };
    match a.policy {
        Some(p) => {
            let parsed = match SharePolicy::from_str(&p) {
                Ok(x) => x,
                Err(e) => return json!({ "error": format!("bad policy `{p}`: {e}") }),
            };
            let init2 = init.clone();
            let r = mem
                .blocking(move |s| kaeru_core::set_share_policy(s, &init2, parsed))
                .await;
            match r {
                Ok(()) => json!({ "initiative": init, "policy": parsed.as_str() }),
                Err(e) => json!({ "error": e.to_string() }),
            }
        }
        None => {
            let init2 = init.clone();
            let r = mem
                .blocking(move |s| kaeru_core::get_share_policy(s, &init2))
                .await;
            match r {
                Ok(cur) => json!({
                    "initiative": init,
                    "policy": cur.as_str(),
                    "permits_share": cur.permits_share(),
                }),
                Err(e) => json!({ "error": e.to_string() }),
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// sync_review (local) — split still-local nodes into propose-share / keep-local.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SyncReviewArgs {
    #[serde(default)]
    pub initiative: Option<String>,
}

async fn do_sync_review(mem: &KaeruMemory, a: SyncReviewArgs) -> Value {
    let Some(init) = target_initiative(mem, &a.initiative) else {
        return json!({ "error": "no initiative — scope the memory or pass `initiative`" });
    };
    let init2 = init.clone();
    mem.blocking(move |s| {
        let pol = match kaeru_core::get_share_policy(s, &init2) {
            Ok(p) => p,
            Err(e) => return json!({ "error": e.to_string() }),
        };
        if !pol.permits_share() {
            return json!({
                "message": format!(
                    "initiative `{init2}` is `{}` — nothing to sync; set policy=team first.",
                    pol.as_str()
                )
            });
        }
        let locals = match kaeru_core::local_nodes_for_review(s, &init2) {
            Ok(l) => l,
            Err(e) => return json!({ "error": e.to_string() }),
        };
        let mut propose: Vec<Value> = Vec::new();
        let mut keep: Vec<Value> = Vec::new();
        for n in &locals {
            let target = format!("{}\n{}", n.name, n.body.clone().unwrap_or_default());
            let hits = guard::scan_public(&target);
            if hits.is_empty() {
                propose.push(json!({ "name": n.name, "node_type": n.node_type, "id": n.id }));
            } else {
                keep.push(json!({
                    "name": n.name, "node_type": n.node_type, "id": n.id,
                    "flagged": hits.iter().map(format_hit).collect::<Vec<_>>(),
                }));
            }
        }
        json!({ "initiative": init2, "propose_share": propose, "keep_local": keep })
    })
    .await
}

mem_tool_cloud!(
    /// `kaeru_policy` — read or set an initiative's cloud sharing policy.
    Policy,
    "kaeru_policy",
    "Read or set an initiative's cloud sharing policy (Gate 1). Omit `policy` to read. \
     Values: private (default — never leaves), team (shared nodes may sync), ask.",
    PolicyArgs,
    { "type": "object", "properties": {
        "initiative": { "type": "string", "description": "initiative (default: the memory's own)" },
        "policy": { "type": "string", "description": "private | team | ask (omit to read)" }
    } },
    |mem, a| do_policy(mem, a).await
);

mem_tool_cloud!(
    /// `kaeru_sync_review` — split still-local nodes into propose / keep.
    SyncReview,
    "kaeru_sync_review",
    "Batch sync-review of a team initiative's still-local nodes: splits them into propose_share \
     (guard-clean) vs keep_local (secret-guard flagged). Review once, then kaeru_share the \
     approved ones — low-friction periodic sharing instead of deciding per capture.",
    SyncReviewArgs,
    { "type": "object", "properties": {
        "initiative": { "type": "string", "description": "initiative (default: the memory's own)" }
    } },
    |mem, a| do_sync_review(mem, a).await
);
