//! `kaeru-rig` — [`rig`] Tools that give a rig agent a **persistent memory**
//! backed by kaeru.
//!
//! Where [`octo-rig`] hands a model one dynamic dispatch tool (Octo's action
//! space is whatever connectors are registered), kaeru's curator API is a
//! fixed, typed surface — so this adapter exposes the **full curator verb set**
//! as discrete tools, each with its own schema, over a shared `Arc<Store>`. A
//! rig agent gets the same memory vocabulary a Claude Code session has over MCP:
//! capture, recall, time-travel, knowledge chains, the hypothesis cycle,
//! consolidation, metabolism, and session re-entry.
//!
//! Pick the tools an agent should have and add them to its toolset:
//!
//! ```ignore
//! let store = Arc::new(Store::open_with_config(KaeruConfig::from_env()?)?);
//! let mem = KaeruMemory::with_initiative(store, "auth-rewrite");
//! let agent = client
//!     .agent(model)
//!     .preamble("You have a persistent memory. `kaeru_awake` first; recall before \
//!                answering; remember what's settled.")
//!     .tool(mem.awake())
//!     .tool(mem.remember())
//!     .tool(mem.recall())
//!     .tool(mem.read())
//!     .tool(mem.link())
//!     // …add whichever others fit the agent
//!     .build();
//! ```
//!
//! Errors are returned to the model as data (`{"error": …}`) rather than failing
//! the tool call — the model then reports them honestly, as in `octo-rig`.
//!
//! **Threading.** kaeru's substrate is synchronous (embedded Cozo/RocksDB).
//! Every tool runs its store work on a blocking thread via
//! [`tokio::task::spawn_blocking`], so a tool call never blocks the async
//! executor. A single `Arc<Store>` is one RocksDB writer — point one
//! `KaeruMemory` (one initiative) at one vault.
//!
//! **Out of scope here:** the cloud sharing verbs (`share` / `pull` /
//! `cloud_recall` / …) need an HTTP client to a `kaeru-cloud` service; that is
//! the `kaeru-mcp` daemon's job, not the embedded adapter's.

use std::sync::Arc;

use kaeru_core::{
    NodeBrief, NodeId, Store, node_brief_by_id, recall_id_by_name, recall_id_by_name_global,
};
use serde_json::{Value, json};

mod capture;
mod chains;
mod evolve;
mod lookup;
mod manage;
mod reason;

pub use capture::*;
pub use chains::*;
pub use evolve::*;
pub use lookup::*;
pub use manage::*;
pub use reason::*;

/// Shared handle to the kaeru substrate plus the initiative an agent works in.
/// Clone it freely — every clone shares the same `Arc<Store>`. The `mem.*()`
/// methods build the individual tools.
#[derive(Clone)]
pub struct KaeruMemory {
    store: Arc<Store>,
    initiative: Option<Arc<str>>,
}

impl KaeruMemory {
    /// Cross-initiative memory (no active initiative; reads span every project).
    pub fn new(store: Arc<Store>) -> Self {
        Self {
            store,
            initiative: None,
        }
    }

    /// Memory scoped to one initiative — captures attach to it and reads
    /// default-filter to it, mirroring `kaeru --initiative <name>`.
    pub fn with_initiative(store: Arc<Store>, initiative: impl Into<String>) -> Self {
        Self {
            store,
            initiative: Some(Arc::from(initiative.into())),
        }
    }

    /// Runs synchronous store work `f` on a blocking thread, scoped to this
    /// memory's initiative. Scoping + the work go through [`Store::scoped`],
    /// which serializes scope sessions on the store — so even two
    /// `KaeruMemory` handles with *different* initiatives sharing one
    /// `Arc<Store>` across the `spawn_blocking` pool can't interleave each
    /// other's scope. A panic / join failure surfaces as `{"error": …}`.
    pub(crate) async fn run<F>(&self, f: F) -> Value
    where
        F: FnOnce(&Store) -> Value + Send + 'static,
    {
        let store = self.store.clone();
        let init = self.initiative.clone();
        tokio::task::spawn_blocking(move || store.scoped(init.as_deref(), f))
            .await
            .unwrap_or_else(|e| json!({ "error": format!("memory task failed: {e}") }))
    }
}

/// Tool constructors. Each returns one rig `Tool` bound to this memory; add the
/// ones an agent should have to its toolset.
impl KaeruMemory {
    // capture
    pub fn remember(&self) -> Remember {
        Remember(self.clone())
    }
    pub fn cite(&self) -> Cite {
        Cite(self.clone())
    }
    pub fn link(&self) -> Link {
        Link(self.clone())
    }
    pub fn unlink(&self) -> Unlink {
        Unlink(self.clone())
    }
    pub fn task(&self) -> Task {
        Task(self.clone())
    }
    pub fn done(&self) -> Done {
        Done(self.clone())
    }
    // lookup
    pub fn recall(&self) -> Recall {
        Recall(self.clone())
    }
    pub fn read(&self) -> Read {
        Read(self.clone())
    }
    pub fn drill(&self) -> Drill {
        Drill(self.clone())
    }
    pub fn trace(&self) -> Trace {
        Trace(self.clone())
    }
    pub fn ideas(&self) -> Ideas {
        Ideas(self.clone())
    }
    pub fn outcomes(&self) -> Outcomes {
        Outcomes(self.clone())
    }
    pub fn tagged(&self) -> Tagged {
        Tagged(self.clone())
    }
    pub fn between(&self) -> Between {
        Between(self.clone())
    }
    pub fn surface(&self) -> Surface {
        Surface(self.clone())
    }
    pub fn at(&self) -> At {
        At(self.clone())
    }
    pub fn history(&self) -> History {
        History(self.clone())
    }
    // chains
    pub fn chain(&self) -> Chain {
        Chain(self.clone())
    }
    pub fn chains(&self) -> Chains {
        Chains(self.clone())
    }
    pub fn read_chain(&self) -> ReadChain {
        ReadChain(self.clone())
    }
    pub fn rechain(&self) -> Rechain {
        Rechain(self.clone())
    }
    pub fn path(&self) -> Path {
        Path(self.clone())
    }
    // reason: hypothesis + review
    pub fn claim(&self) -> Claim {
        Claim(self.clone())
    }
    pub fn test(&self) -> Test {
        Test(self.clone())
    }
    pub fn confirm(&self) -> Confirm {
        Confirm(self.clone())
    }
    pub fn refute(&self) -> Refute {
        Refute(self.clone())
    }
    pub fn flag(&self) -> Flag {
        Flag(self.clone())
    }
    pub fn resolve(&self) -> Resolve {
        Resolve(self.clone())
    }
    // evolve: consolidation + metabolism
    pub fn settle(&self) -> Settle {
        Settle(self.clone())
    }
    pub fn reopen(&self) -> Reopen {
        Reopen(self.clone())
    }
    pub fn synthesise(&self) -> Synthesise {
        Synthesise(self.clone())
    }
    pub fn supersede(&self) -> Supersede {
        Supersede(self.clone())
    }
    pub fn forget(&self) -> Forget {
        Forget(self.clone())
    }
    pub fn revise(&self) -> Revise {
        Revise(self.clone())
    }
    pub fn layer(&self) -> SetLayer {
        SetLayer(self.clone())
    }
    // manage: session + initiative + diagnostics + snapshot
    pub fn awake(&self) -> Awake {
        Awake(self.clone())
    }
    pub fn overview(&self) -> Overview {
        Overview(self.clone())
    }
    pub fn initiatives(&self) -> Initiatives {
        Initiatives(self.clone())
    }
    pub fn recent(&self) -> Recent {
        Recent(self.clone())
    }
    pub fn pin(&self) -> Pin {
        Pin(self.clone())
    }
    pub fn unpin(&self) -> Unpin {
        Unpin(self.clone())
    }
    pub fn rename_initiative(&self) -> RenameInitiative {
        RenameInitiative(self.clone())
    }
    pub fn delete_initiative(&self) -> DeleteInitiative {
        DeleteInitiative(self.clone())
    }
    pub fn attach(&self) -> Attach {
        Attach(self.clone())
    }
    pub fn lint(&self) -> Lint {
        Lint(self.clone())
    }
    pub fn reflect(&self) -> Reflect {
        Reflect(self.clone())
    }
    pub fn export(&self) -> Export {
        Export(self.clone())
    }
}

// ── Shared helpers used by tool bodies ───────────────────────────────────────

/// Resolves a node reference that may be a name or a raw id, falling back to
/// treating the input as an id when no name matches.
pub(crate) fn resolve(store: &Store, name_or_id: &str) -> String {
    recall_id_by_name(store, name_or_id)
        .ok()
        .flatten()
        .unwrap_or_else(|| name_or_id.to_string())
}

/// Like [`resolve`], but resolves across **all** initiatives. Tool bodies run
/// inside `KaeruMemory::run` → `Store::scoped(<memory initiative>)`, so a plain
/// `resolve` only sees the memory's own initiative — no good for `attach`,
/// which targets a node living under a different one. `recall_id_by_name_global`
/// ignores the active scope without re-locking the scope guard.
pub(crate) fn resolve_global(store: &Store, name_or_id: &str) -> String {
    recall_id_by_name_global(store, name_or_id)
        .ok()
        .flatten()
        .unwrap_or_else(|| name_or_id.to_string())
}

/// One [`NodeBrief`] as a compact JSON object.
pub(crate) fn brief(b: &NodeBrief) -> Value {
    json!({ "id": b.id, "name": b.name, "type": b.node_type, "excerpt": b.body_excerpt })
}

/// A slice of briefs as a JSON array.
pub(crate) fn briefs(v: &[NodeBrief]) -> Value {
    Value::Array(v.iter().map(brief).collect())
}

/// Resolves a slice of ids to brief JSON objects (skipping any that vanished),
/// capped to keep tool output bounded.
pub(crate) fn briefs_by_ids(store: &Store, ids: &[NodeId]) -> Value {
    let items: Vec<Value> = ids
        .iter()
        .take(50)
        .filter_map(|id| node_brief_by_id(store, id).ok().flatten())
        .map(|b| brief(&b))
        .collect();
    Value::Array(items)
}

/// Generates a rig [`Tool`](rig::tool::Tool) struct + impl backed by a
/// [`KaeruMemory`]. The body is an expression over `store: &Store` and the
/// deserialized `args`, evaluating to a `serde_json::Value` (errors as data).
/// All store work runs through [`KaeruMemory::run`] (blocking thread + scope).
macro_rules! mem_tool {
    (
        $(#[$meta:meta])*
        $tool:ident, $name:literal, $desc:expr, $args_ty:ty, $params:tt,
        |$store:ident, $a:ident| $body:expr
    ) => {
        $(#[$meta])*
        #[derive(Clone)]
        pub struct $tool(pub(crate) $crate::KaeruMemory);

        impl ::rig::tool::Tool for $tool {
            const NAME: &'static str = $name;
            type Error = ::std::convert::Infallible;
            type Args = $args_ty;
            type Output = ::serde_json::Value;

            async fn definition(
                &self,
                _prompt: ::std::string::String,
            ) -> ::rig::completion::ToolDefinition {
                ::rig::completion::ToolDefinition {
                    name: $name.to_string(),
                    description: ($desc).to_string(),
                    parameters: ::serde_json::json!($params),
                }
            }

            async fn call(
                &self,
                $a: $args_ty,
            ) -> ::core::result::Result<::serde_json::Value, ::std::convert::Infallible> {
                ::core::result::Result::Ok(self.0.run(move |$store| $body).await)
            }
        }
    };
}
pub(crate) use mem_tool;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use kaeru_core::Store;
    use rig::tool::Tool;

    use super::KaeruMemory;

    fn args<T: serde::de::DeserializeOwned>(v: serde_json::Value) -> T {
        serde_json::from_value(v).expect("args")
    }

    /// Drives a representative slice of the full toolset through rig's
    /// `Tool::call` against an in-memory vault — capture, recall, read, link,
    /// chain, time-travel, and re-entry — end to end, no LLM involved.
    #[tokio::test]
    async fn full_surface_round_trip() {
        let store = Arc::new(Store::open_in_memory().expect("open"));
        let mem = KaeruMemory::with_initiative(store, "test-proj");

        // remember two named notes.
        let a = mem
            .remember()
            .call(args(serde_json::json!({ "name": "auth-decision", "body": "platform-aware token expiry" })))
            .await
            .unwrap();
        assert_eq!(a["saved"], true, "remember saved; got {a}");
        mem.remember()
            .call(args(serde_json::json!({ "name": "expiry-bug", "body": "tokens expired early on android" })))
            .await
            .unwrap();

        // recall by a body word.
        let found = mem
            .recall()
            .call(args(serde_json::json!({ "query": "platform" })))
            .await
            .unwrap();
        assert!(
            found["results"]
                .as_array()
                .unwrap()
                .iter()
                .any(|r| r["name"] == "auth-decision"),
            "recall finds it; got {found}"
        );

        // read in full by name.
        let read = mem
            .read()
            .call(args(serde_json::json!({ "name_or_id": "auth-decision" })))
            .await
            .unwrap();
        assert_eq!(read["body"], "platform-aware token expiry");

        // link the two, weighted strong, and chain across them.
        let linked = mem
            .link()
            .call(args(serde_json::json!({ "from": "expiry-bug", "to": "auth-decision", "edge_type": "causal", "weight": 0.9 })))
            .await
            .unwrap();
        assert_eq!(linked["linked"], true, "link ok; got {linked}");

        let path = mem
            .path()
            .call(args(
                serde_json::json!({ "from": "expiry-bug", "to": "auth-decision" }),
            ))
            .await
            .unwrap();
        assert!(
            path["path"]
                .as_array()
                .map(|a| a.len() >= 2)
                .unwrap_or(false),
            "path found; got {path}"
        );

        // awake reports the active initiative + recent captures.
        let ctx = mem.awake().call(args(serde_json::json!({}))).await.unwrap();
        assert_eq!(ctx["initiative"], "test-proj");
        assert!(
            !ctx["recent"].as_array().unwrap().is_empty(),
            "recent surfaced; got {ctx}"
        );

        // tagged: episodes auto-get topic: tags from body words.
        let tagged = mem
            .tagged()
            .call(args(serde_json::json!({ "tag": "topic:platform-aware" })))
            .await
            .unwrap();
        assert!(
            tagged["tagged"].is_array(),
            "tagged returns array; got {tagged}"
        );

        // task → done.
        let task = mem
            .task()
            .call(args(serde_json::json!({ "body": "ship the adapter" })))
            .await
            .unwrap();
        assert_eq!(task["created"], true, "task created; got {task}");
        let done = mem
            .done()
            .call(args(
                serde_json::json!({ "name_or_id": task["id"].as_str().unwrap() }),
            ))
            .await
            .unwrap();
        assert_eq!(done["done"], true, "task done; got {done}");

        // hypothesis cycle: claim → test (exercises enum-bearing bodies).
        let claim = mem
            .claim()
            .call(args(serde_json::json!({
                "name": "weekend-deploys-flaky",
                "claim": "weekend deploys cause flaky tests"
            })))
            .await
            .unwrap();
        assert_eq!(claim["created"], true, "claim created; got {claim}");
        let test = mem
            .test()
            .call(args(serde_json::json!({
                "hypothesis": "weekend-deploys-flaky",
                "name": "compare-runs",
                "method": "100 runs each"
            })))
            .await
            .unwrap();
        assert_eq!(test["created"], true, "experiment created; got {test}");

        // consolidation: settle an operational note into an archival idea
        // (parses NodeType "idea").
        let settled = mem
            .settle()
            .call(args(serde_json::json!({
                "name_or_id": "auth-decision",
                "as_type": "idea",
                "name": "expiry-policy",
                "body": "platform-aware expiry is the policy"
            })))
            .await
            .unwrap();
        assert_eq!(
            settled["settled"], true,
            "settled to archival; got {settled}"
        );
    }
}
