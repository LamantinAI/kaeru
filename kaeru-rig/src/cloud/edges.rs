//! Mirroring a local graph edit to the cloud — the rig half of the daemon's
//! `propagate_edge`.
//!
//! `link`, `unlink` and `reweight` write locally first and always: the local
//! edit is the point of the verb and must never depend on the network. The
//! mirror is what keeps the cloud's copy of the graph from being frozen at
//! the moment each node was shared — and when it cannot happen, the result
//! says so, because silence leaves local and cloud differing with nothing to
//! show it.

use kaeru_core::{EdgeType, Visibility};
use serde_json::{Value, json};

use crate::KaeruMemory;

/// What happened to the edge locally.
#[derive(Debug, Clone, Copy)]
pub(crate) enum EdgeChange {
    /// Created or reweighted — the cloud's `POST /edges` is an upsert.
    Upsert(f64),
    /// Retracted — `DELETE /edges`.
    Retract,
}

/// Mirrors an edge change when **both** endpoints are already shared.
///
/// Returns the note to attach to the result, or `None` when there was
/// nothing to mirror: a local endpoint, no initiative, no cloud configured.
pub(crate) async fn propagate_edge(
    mem: &KaeruMemory,
    cloud_name: Option<&str>,
    src: &str,
    dst: &str,
    edge_type: EdgeType,
    change: EdgeChange,
    initiative: Option<&str>,
) -> Option<Value> {
    // Everything cheap and local first: an edge with a local endpoint is not
    // the cloud's to hold, and there is nothing to resolve for it.
    let init = initiative?;
    let (a, b) = (src.to_string(), dst.to_string());
    let both_shared = mem
        .blocking(move |s| {
            let a_vis = kaeru_core::get_visibility(s, &a).ok()?;
            let b_vis = kaeru_core::get_visibility(s, &b).ok()?;
            Some(a_vis == Visibility::Shared && b_vis == Visibility::Shared)
        })
        .await
        .unwrap_or(false);
    if !both_shared {
        return None;
    }

    // `resolve` refuses to guess between several configured clouds. Here that
    // must not fail the call — the local edit has already happened — so the
    // ambiguity is reported instead.
    let cloud = match mem.clouds().resolve(cloud_name) {
        Ok(c) => c,
        Err(why) => {
            return Some(json!({
                "mirrored": false,
                "reason": format!("{why} — pass `cloud=<name>` to send this edge"),
            }));
        }
    };

    // The nodes passed the gates when they were shared, but the initiative
    // may have been closed since.
    let (for_policy, cloud_for_policy) = (init.to_string(), cloud.name().to_string());
    let still_permitted = mem
        .blocking(move |s| {
            let policy = kaeru_core::get_share_policy(s, &for_policy).ok()?;
            let to_this = kaeru_core::permits_cloud(s, &for_policy, &cloud_for_policy).ok()?;
            Some(policy.permits_share() && to_this)
        })
        .await
        .unwrap_or(false);
    if !still_permitted {
        return Some(json!({
            "mirrored": false,
            "reason": format!(
                "both endpoints are shared, but `{init}` no longer permits sharing to `{}` — \
                 the cloud still holds the old edge",
                cloud.name()
            ),
        }));
    }

    let call = match change {
        EdgeChange::Upsert(weight) => {
            let body = json!({
                "src": src, "dst": dst, "edge_type": edge_type.as_str(), "weight": weight
            });
            cloud.post_edge(&body).await
        }
        EdgeChange::Retract => {
            let body = json!({ "src": src, "dst": dst, "edge_type": edge_type.as_str() });
            cloud.delete_edge(&body).await
        }
    };

    Some(match call {
        Ok((code, _)) if (200..300).contains(&code) => {
            json!({ "mirrored": true, "cloud": cloud.name() })
        }
        // Said out loud rather than swallowed: local and cloud now disagree,
        // and the agent is the only one in a position to retry.
        Ok((code, body)) => json!({
            "mirrored": false,
            "cloud": cloud.name(),
            "status": code,
            "reason": body.lines().next().unwrap_or("").to_string(),
        }),
        Err(e) => json!({ "mirrored": false, "cloud": cloud.name(), "error": e }),
    })
}
