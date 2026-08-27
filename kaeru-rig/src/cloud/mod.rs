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
//! - [`share`] — `share`: both gates, then push the node and its shareable edges.
//! - [`pull`] — `cloud_recall` + `pull`: discovery, then materialise locally.
//! - [`links`] — `link_cloud` + `cloud_links`: soft references, resolved lazily.

use kaeru_core::GuardHit;
use serde_json::{Value, json};

use crate::KaeruMemory;
use crate::cloud_client::CloudClient;

mod links;
mod policy;
mod pull;
mod share;

pub use links::{CloudLinks, CloudLinksArgs, LinkCloud, LinkCloudArgs};
pub use policy::{Policy, PolicyArgs, SyncReview, SyncReviewArgs};
pub use pull::{CloudRecall, CloudRecallArgs, Pull, PullArgs};
pub use share::{Share, ShareArgs};

/// Resolves the target [`CloudClient`], or an error `Value` naming what is
/// configured and why it will not guess.
pub(super) fn cloud_or_err(mem: &KaeruMemory, name: Option<&str>) -> Result<CloudClient, Value> {
    // Through `resolve`, not `get`: with several clouds configured an unnamed
    // call is refused rather than sent to the default. See the daemon-side
    // rule this mirrors (#65) — a cloud write cannot be undone in the wrong
    // place, so the cost of guessing is not symmetric with the cost of asking.
    mem.clouds()
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use kaeru_core::{EpisodeKind, Significance, Store};
    use rig::tool::Tool;
    use serde_json::json;

    use super::{PolicyArgs, ShareArgs, SyncReviewArgs};
    use crate::KaeruMemory;

    fn args<T: serde::de::DeserializeOwned>(v: serde_json::Value) -> T {
        serde_json::from_value(v).expect("args")
    }

    fn seed(store: &Store, name: &str, body: &str) {
        store.scoped(Some("t"), |s| {
            kaeru_core::write_episode(s, EpisodeKind::Observation, Significance::Low, name, body)
                .expect("write");
        });
    }

    /// `policy` (a local tool — no cloud needed) sets then reads an initiative's
    /// share policy.
    #[tokio::test]
    async fn policy_sets_then_reads() {
        let store = Arc::new(Store::open_in_memory().expect("open"));
        let mem = KaeruMemory::with_initiative(store, "t");

        let set = mem
            .policy()
            .call(args::<PolicyArgs>(
                json!({ "initiative": "t", "policy": "team" }),
            ))
            .await
            .unwrap();
        assert_eq!(
            set["policy"], "team",
            "set echoes the new policy; got {set}"
        );

        let get = mem
            .policy()
            .call(args::<PolicyArgs>(json!({ "initiative": "t" })))
            .await
            .unwrap();
        assert_eq!(get["policy"], "team");
        assert_eq!(get["permits_share"], true, "team permits share; got {get}");
    }

    /// `sync_review` (local) splits nodes by the pre-share secret guard: a clean
    /// note is proposed, one carrying a secret marker is kept local.
    #[tokio::test]
    async fn sync_review_splits_clean_from_secret() {
        let store = Arc::new(Store::open_in_memory().expect("open"));
        let mem = KaeruMemory::with_initiative(store.clone(), "t");
        mem.policy()
            .call(args::<PolicyArgs>(
                json!({ "initiative": "t", "policy": "team" }),
            ))
            .await
            .unwrap();

        seed(
            &store,
            "clean-note",
            "an ordinary observation about the design",
        );
        seed(&store, "leaky-note", "the db password: hunter2 stays here");

        let out = mem
            .sync_review()
            .call(args::<SyncReviewArgs>(json!({ "initiative": "t" })))
            .await
            .unwrap();
        let propose = out["propose_share"].as_array().expect("propose array");
        let keep = out["keep_local"].as_array().expect("keep array");
        assert!(
            propose.iter().any(|n| n["name"] == "clean-note"),
            "clean node proposed; got {out}"
        );
        assert!(
            keep.iter().any(|n| n["name"] == "leaky-note"),
            "secret node kept local; got {out}"
        );
    }

    /// A memory built without a cloud reports "cloud not configured" from a
    /// network tool — no panic, no hang, an actionable message.
    #[tokio::test]
    async fn network_tool_without_cloud_is_not_configured() {
        let store = Arc::new(Store::open_in_memory().expect("open"));
        let mem = KaeruMemory::with_initiative(store, "t");

        let out = mem
            .share()
            .call(args::<ShareArgs>(json!({ "name": "whatever" })))
            .await
            .unwrap();
        assert!(
            out["error"]
                .as_str()
                .unwrap_or("")
                .contains("no cloud is configured"),
            "no-cloud share is not-configured; got {out}"
        );
    }

    /// The rule this mirrors from the daemon (#65): with several clouds
    /// configured, an unnamed call is refused rather than routed to a default.
    /// A cloud write cannot be undone in the wrong place, so the cost of
    /// guessing is not symmetric with the cost of asking.
    #[tokio::test]
    async fn several_clouds_refuse_an_unnamed_share() {
        use std::collections::HashMap;

        use crate::cloud_client::{CloudClient, CloudRegistry};

        let store = Arc::new(Store::open_in_memory().expect("open"));
        let clients: HashMap<String, CloudClient> = ["alpha", "beta"]
            .iter()
            .map(|n| {
                (
                    n.to_string(),
                    CloudClient::new(n.to_string(), format!("http://{n}.test"), String::new()),
                )
            })
            .collect();
        let mem = KaeruMemory::with_clouds(
            store,
            "t",
            CloudRegistry::new(clients, Some("alpha".to_string())),
        );

        let out = mem
            .share()
            .call(args::<ShareArgs>(json!({ "name": "whatever" })))
            .await
            .unwrap();
        let err = out["error"].as_str().unwrap_or("");
        assert!(err.contains("alpha, beta"), "lists the choices; got {out}");
        assert!(err.contains("cannot be undone"), "says why; got {out}");
    }
}
