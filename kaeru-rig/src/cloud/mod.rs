//! Cloud sharing & recall tools — the in-process local↔cloud bridge.
//!
//! Mirror of the daemon's `kaeru-mcp/src/tools/cloud.rs`, adapted to rig: bodies
//! are `async` (they `.await` the [`CloudClient`]) and every local-store span
//! goes through `mem.blocking(...)` (a blocking thread), so a network round-trip
//! never stalls the async executor and a Cozo read never runs on it.
//!
//! One submodule per concern; this `mod.rs` holds what they share and re-exports
//! the tool surface:
//!
//! - [`policy`] — `policy` + `sync_review`: the two purely-local tools.
//! - [`edges`] — mirroring a local `link` / `unlink` / `reweight` (#85).
//! - [`share`] — `share`: both gates, then push the node and its shareable edges.
//! - [`pull`] — `cloud_recall` + `pull`: discovery, then materialise locally.
//! - [`links`] — `link_cloud` + `cloud_links`: soft references, resolved lazily.

use kaeru_core::GuardHit;
use serde_json::{Value, from_value, json};

use crate::KaeruMemory;
use crate::cloud_client::CloudClient;

mod edges;
mod links;
mod policy;
mod pull;
mod share;

pub(crate) use edges::propagate_edge;

pub(crate) async fn call(mem: &KaeruMemory, name: &str, args: Value) -> Value {
    macro_rules! decode {
        ($handler:path) => { match from_value(args) {
            Ok(a) => $handler(mem, a).await,
            Err(e) => json!({"error": e.to_string()}),
        } };
    }
    match name {
        "clouds" => {
            mem.store.record_read_trace("clouds", vec![], vec![]);
            json!({"clouds": mem.cloud_registry().names()})
        }
        "share" => decode!(share::do_share),
        "unshare" => decode!(share::do_unshare),
        "cloud_initiatives" => decode!(pull::do_cloud_initiatives),
        "cloud_recall" => decode!(pull::do_cloud_recall),
        "pull" => decode!(pull::do_pull),
        "link_cloud" => decode!(links::do_link_cloud),
        "cloud_links" => decode!(links::do_cloud_links),
        "sync_review" => decode!(policy::do_sync_review),
        _ => json!({"error": "unknown cloud operation"}),
    }
}

/// Resolves the target [`CloudClient`], or an error `Value` naming what is
/// configured and why it will not guess.
pub(super) fn cloud_or_err(mem: &KaeruMemory, name: Option<&str>) -> Result<CloudClient, Value> {
    // Through `resolve`, not `get`: with several clouds configured an unnamed
    // call is refused rather than sent to the default. See the daemon-side
    // rule this mirrors (#65) — a cloud write cannot be undone in the wrong
    // place, so the cost of guessing is not symmetric with the cost of asking.
    mem.cloud_registry()
        .resolve(name)
        .cloned()
        .map_err(|error| json!({ "error": error }))
}

/// A guard hit as `rule: "fragment"`. The fragment is `GuardHit.matched`, which
/// the guard already truncates for safe display — showing it is what lets a
/// human tell a real secret from a false positive without re-reading the body.
pub(super) fn format_hit(h: &GuardHit) -> String {
    format!("{}: {:?}", h.rule, h.matched)
}
