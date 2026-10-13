//! The two purely-local cloud tools — no network, no client needed.
//!
//! - `policy` — read / set an initiative's sticky `share_policy` (sharing Gate 1).
//! - `sync_review` — split a team initiative's still-local nodes into
//!   propose-share (guard-clean) vs keep-local (secret-guard flagged). It
//!   proposes; it never pushes.

use kaeru_core::{get_share_policy, guard, local_nodes_for_review};
use serde::Deserialize;
use serde_json::{Value, json};

use super::format_hit;
use crate::KaeruMemory;

// ─────────────────────────────────────────────────────────────────────────────
// policy (local) — read / set an initiative's share_policy (Gate 1).
// ─────────────────────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────────────────────
// sync_review (local) — split still-local nodes into propose-share / keep-local.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SyncReviewArgs {
    pub initiative: String,
}

pub(super) async fn do_sync_review(mem: &KaeruMemory, a: SyncReviewArgs) -> Value {
    // The verb names its initiative, so there is nothing to fall back to.
    let init = a.initiative.clone();
    let init2 = init.clone();
    mem.blocking(move |s| {
        let pol = match get_share_policy(s, &init2) {
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
        let locals = match local_nodes_for_review(s, &init2) {
            Ok(l) => l,
            Err(e) => return json!({ "error": e.to_string() }),
        };
        let mut propose: Vec<Value> = Vec::new();
        let mut keep: Vec<Value> = Vec::new();
        for n in &locals {
            let target = format!(
                "{}\n{}\n{}",
                n.name,
                n.body.clone().unwrap_or_default(),
                n.properties
                    .as_ref()
                    .map(|p| p.to_string())
                    .unwrap_or_default()
            );
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
