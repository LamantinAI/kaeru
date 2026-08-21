//! `KaeruServer` — rmcp tool-router glue. Each `#[tool]` method here
//! is a thin wrapper that destructures `Parameters<T>` and forwards to
//! the corresponding `tools::<group>::<fn>`. The actual logic lives
//! there, the param structs in `params.rs`, the shared utilities in
//! `utils.rs`. This file stays focused on tool registration so that
//! the agent-facing surface (descriptions, schemas) reads top-to-bottom.
//!
//! The `#[tool_router]` macro requires every `#[tool]` to be in one
//! impl block, so we group routing here and keep behaviour split
//! across files.

use std::sync::Arc;

use kaeru_core::Store;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
};
use rmcp::{ErrorData as McpError, ServerHandler, tool, tool_handler, tool_router};
use tokio_util::sync::CancellationToken;

use crate::cloud_client::CloudRegistry;
use crate::hygiene::HygieneScheduler;
use crate::params::*;
use crate::tools;

#[derive(Clone)]
pub struct KaeruServer {
    store: Arc<Store>,
    /// Named clouds this daemon can reach. Empty when no cloud is configured
    /// — the cloud tools then report that sharing is unavailable. Tools pick
    /// a cloud by explicit `cloud` argument, else the registry's default.
    clouds: CloudRegistry,
    /// Decides when a hygiene pass is due and runs it off the reactor. Every
    /// tool that reads or writes an initiative nudges it; the nudge is a
    /// no-op unless a pass is actually due.
    hygiene: HygieneScheduler,
    /// Filled by `Self::tool_router()` (macro-generated); read by the
    /// `#[tool_handler]`-generated `ServerHandler` impl, but the
    /// dead-code analyser doesn't see that path.
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
}

impl KaeruServer {
    pub fn new(
        store: Store,
        clouds: CloudRegistry,
        cancel: CancellationToken,
        hygiene_enabled: bool,
    ) -> Self {
        let store = Arc::new(store);
        let hygiene = HygieneScheduler::new(Arc::clone(&store), cancel, hygiene_enabled);
        Self {
            store,
            clouds,
            hygiene,
            tool_router: Self::tool_router(),
        }
    }

    /// Shared substrate handle — used by the read-only `/graph.json` viz
    /// endpoint, which exports the whole graph for the visualizer.
    pub fn store(&self) -> Arc<Store> {
        self.store.clone()
    }

    /// The background hygiene scheduler; `main` starts its sweep timer.
    pub fn hygiene_scheduler(&self) -> &HygieneScheduler {
        &self.hygiene
    }

    /// Post-processes a tool result: prepends any hygiene headline waiting for
    /// this initiative, then nudges the scheduler.
    ///
    /// Delivery rides on tool responses because that is the only channel that
    /// reaches both Claude Code and Codex: MCP `notifications/message` is
    /// received by both and displayed by neither (anthropics/claude-code#3174,
    /// #33679; openai/codex#18056).
    ///
    /// Applied to `awake` (the session entry point) and to the capture verbs
    /// (every write), which together cover both the "agent arrived" and "agent
    /// wrote something" triggers. The sweep timer covers initiatives nobody
    /// touches.
    fn after_tool(
        &self,
        initiative: Option<&str>,
        result: Result<CallToolResult, McpError>,
    ) -> Result<CallToolResult, McpError> {
        let Ok(mut result) = result else {
            return result;
        };
        if let Some(init) = initiative
            && let Ok(Some(headline)) = kaeru_core::hygiene::take_pending_report(&self.store, init)
        {
            result
                .content
                .insert(0, Content::text(format!("{headline}\n")));
        }
        self.hygiene.consider(initiative);
        Ok(result)
    }
}

#[tool_router]
impl KaeruServer {
    // ----- Re-entry / session -------------------------------------------
    #[tool(
        description = "Restore session context: pinned set, recent episodes (24h), open reviews. Run this when re-entering a project."
    )]
    fn awake(&self, Parameters(p): Parameters<ScopeOnly>) -> Result<CallToolResult, McpError> {
        let result = tools::session::awake(&self.store, p.initiative.as_deref());
        self.after_tool(p.initiative.as_deref(), result)
    }

    #[tool(
        description = "Print a terminal-readable map of the substrate: counts by tier/type, provenance forests, open questions, edge stats."
    )]
    fn overview(&self, Parameters(p): Parameters<ScopeOnly>) -> Result<CallToolResult, McpError> {
        tools::session::overview(&self.store, p.initiative.as_deref())
    }

    #[tool(
        description = "List initiatives that have at least one node attached. Use this first when re-entering, then pick one for subsequent calls."
    )]
    fn initiatives(&self) -> Result<CallToolResult, McpError> {
        tools::session::initiatives(&self.store)
    }

    #[tool(
        description = "List episodes whose latest assertion is within the time window (defaults 24h). Use `since` like `30m`, `3h`, `2d`, or raw seconds."
    )]
    fn recent(&self, Parameters(p): Parameters<RecentParams>) -> Result<CallToolResult, McpError> {
        tools::session::recent(&self.store, &p.since, p.initiative.as_deref())
    }

    #[tool(description = "Pin a node to the active window. Accepts either a name or a UUIDv7 id.")]
    fn pin(&self, Parameters(p): Parameters<PinParams>) -> Result<CallToolResult, McpError> {
        tools::session::pin(&self.store, &p.name, &p.reason, p.initiative.as_deref())
    }

    #[tool(description = "Unpin a node. Accepts name or id.")]
    fn unpin(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        tools::session::unpin(&self.store, &p.name, p.initiative.as_deref())
    }

    #[tool(
        description = "Show resolved configuration: vault path and every cap (initiative not relevant)."
    )]
    fn config(&self) -> Result<CallToolResult, McpError> {
        tools::session::config(&self.store)
    }

    #[tool(
        description = "Read FIRST before bulk-importing knowledge. Returns the import playbook: scope by initiative, pick the verb by epistemic status, stamp the memory layer at creation by importance, and link after capturing."
    )]
    fn import(&self) -> Result<CallToolResult, McpError> {
        tools::session::import_guide()
    }

    #[tool(
        description = "Explicit layered recall — surface nodes from specific memory layers (default `cold,frozen`) that `awake` does not load. Use when you deliberately need archived/not-surfaced material. `layers` is a comma/space list; scoped to `initiative` when given."
    )]
    fn surface(
        &self,
        Parameters(p): Parameters<SurfaceParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::session::surface(&self.store, p.layers.as_deref(), p.initiative.as_deref())
    }

    // ----- Slots & hygiene ------------------------------------------------
    #[tool(
        description = "Make a node the live holder of a ROLE in an initiative — `handoff`, `entrypoint`, `queue`, `prod-state`. A role holds exactly one node: taking it archives the previous holder to `cold` and links `supersedes`, so a project can never end up with three current handoffs. Nothing is deleted; the predecessor stays readable via `at` / `surface layers=cold`."
    )]
    fn slot(&self, Parameters(p): Parameters<SlotParams>) -> Result<CallToolResult, McpError> {
        let result = tools::slots::slot(&self.store, &p.initiative, &p.slot, &p.name);
        self.after_tool(Some(&p.initiative), result)
    }

    #[tool(description = "List the filled roles of an initiative and which node holds each.")]
    fn slots(&self, Parameters(p): Parameters<InitiativeOnly>) -> Result<CallToolResult, McpError> {
        tools::slots::slots(&self.store, &p.initiative)
    }

    #[tool(
        description = "Free a role without touching the node that held it — its layer stays as it is."
    )]
    fn unslot(&self, Parameters(p): Parameters<SlotScope>) -> Result<CallToolResult, McpError> {
        tools::slots::unslot(&self.store, &p.initiative, &p.slot)
    }

    #[tool(
        description = "Hygiene status for an initiative: node and core counts, when the last pass ran, whether one is due, and exactly what the next pass would move. Passes run on their own — when writes accumulate, when core grows past its threshold, or on the sweep timer — and only ever change a node's layer, reversibly. `force=true` runs one now."
    )]
    fn hygiene(
        &self,
        Parameters(p): Parameters<HygieneParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::hygiene::hygiene(&self.store, &self.hygiene, &p.initiative, p.force)
    }

    // ----- Capture -------------------------------------------------------
    #[tool(
        description = "Write a deliberately-named operational episode. Use when you know you'll want to recall by exact name. Pass visibility=shared (in a team initiative) to capture and push to the cloud in one call."
    )]
    async fn episode(
        &self,
        Parameters(p): Parameters<EpisodeParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = tools::capture::episode(
            &self.store,
            self.clouds.get(None),
            &p.name,
            &p.body,
            p.layer.as_deref(),
            p.visibility.as_deref(),
            p.initiative.as_deref(),
        )
        .await;
        self.after_tool(p.initiative.as_deref(), result)
    }

    #[tool(
        description = "Low-friction episode write — auto-named from body's first words plus a unique id suffix. Defaults to observation/low. Pass visibility=shared (in a team initiative) to capture and push to the cloud in one call."
    )]
    async fn jot(&self, Parameters(p): Parameters<JotParams>) -> Result<CallToolResult, McpError> {
        let result = tools::capture::jot(
            &self.store,
            self.clouds.get(None),
            &p.body,
            p.layer.as_deref(),
            p.visibility.as_deref(),
            p.initiative.as_deref(),
        )
        .await;
        self.after_tool(p.initiative.as_deref(), result)
    }

    #[tool(
        description = "Create a typed edge between two nodes (by name or id). Endpoints resolve in the active initiative first, then across all initiatives, so a link may span initiatives. Edge type defaults to `refers_to`. Optional `weight` (0..1) or `strong=true` sets the connection strength used by knowledge chains — stronger links make shorter chain paths."
    )]
    fn link(&self, Parameters(p): Parameters<LinkParams>) -> Result<CallToolResult, McpError> {
        tools::capture::link(
            &self.store,
            &p.from,
            &p.to,
            &p.edge_type,
            p.weight,
            p.strong,
            p.initiative.as_deref(),
        )
    }

    #[tool(
        description = "Retract a previously-asserted edge. Bi-temporal — historical reads still see it."
    )]
    fn unlink(&self, Parameters(p): Parameters<LinkParams>) -> Result<CallToolResult, McpError> {
        tools::capture::unlink(
            &self.store,
            &p.from,
            &p.to,
            &p.edge_type,
            p.initiative.as_deref(),
        )
    }

    #[tool(
        description = "Set an existing edge's connection strength (weight 0..1) in place. Stronger edges make shorter knowledge-chain paths; use to tune which links matter after the fact."
    )]
    fn reweight(
        &self,
        Parameters(p): Parameters<ReweightParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::capture::reweight(
            &self.store,
            &p.from,
            &p.to,
            &p.edge_type,
            p.weight,
            p.initiative.as_deref(),
        )
    }

    // ----- Knowledge chains ---------------------------------------------
    #[tool(
        description = "Save the shortest weighted path between two nodes as a knowledge chain — an ordered, recallable reasoning trail. Stronger links (see `link` weight/strong) make shorter paths. Pass `summary` to note why the trail matters (it labels the chain for later triage). Idempotent — an identical chain is reused, not duplicated. Reports if the two are unconnected."
    )]
    fn chain(&self, Parameters(p): Parameters<ChainParams>) -> Result<CallToolResult, McpError> {
        tools::chain::chain(
            &self.store,
            &p.from,
            &p.to,
            p.name.as_deref(),
            p.summary.as_deref(),
            p.initiative.as_deref(),
        )
    }

    #[tool(
        description = "List the knowledge chains a node belongs to. When a single node is context-poor, see its chains and `read_chain` the relevant one."
    )]
    fn chains(&self, Parameters(p): Parameters<ChainsParams>) -> Result<CallToolResult, McpError> {
        tools::chain::chains(&self.store, &p.name, p.initiative.as_deref())
    }

    #[tool(
        description = "Read a knowledge chain's ordered members in full — the connected reasoning trail, instead of an isolated node."
    )]
    fn read_chain(
        &self,
        Parameters(p): Parameters<ReadChainParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::chain::read_chain(&self.store, &p.name, p.initiative.as_deref())
    }

    #[tool(
        description = "Refresh a chain the graph has outgrown. With no `to`, regenerate it — recompute the shortest path between its current endpoints (picks up new edges / re-weights). With `to`, extend the trail out to that node. Keeps the chain's id, name, and summary."
    )]
    fn rechain(
        &self,
        Parameters(p): Parameters<RechainParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::chain::rechain(
            &self.store,
            &p.chain,
            p.to.as_deref(),
            p.initiative.as_deref(),
        )
    }

    #[tool(
        description = "Compute the shortest weighted path between two nodes WITHOUT saving it (preview). Use `chain` to persist one."
    )]
    fn path(&self, Parameters(p): Parameters<PathParams>) -> Result<CallToolResult, McpError> {
        tools::chain::path(&self.store, &p.from, &p.to, p.initiative.as_deref())
    }

    #[tool(
        description = "Record an archival reference. Two flavours: external source (pass `url` for papers / gists / dashboards) OR persona / entity (skip `url` for people, places, books without links). Both land in archival tier — long-term recall. Pass visibility=shared (in a team initiative) to capture and push to the cloud in one call."
    )]
    async fn cite(
        &self,
        Parameters(p): Parameters<CiteParams>,
    ) -> Result<CallToolResult, McpError> {
        let result = tools::capture::cite(
            &self.store,
            self.clouds.get(None),
            &p.name,
            p.url.as_deref(),
            &p.body,
            p.layer.as_deref(),
            p.visibility.as_deref(),
            p.initiative.as_deref(),
        )
        .await;
        self.after_tool(p.initiative.as_deref(), result)
    }

    // ----- Cloud sharing & recall ---------------------------------------
    #[tool(
        description = "Read or set an initiative's cloud sharing policy (Gate 1). Omit `policy` to read. Values: private (default — never leaves), team (shared nodes may sync), ask. Default for any initiative is private."
    )]
    fn policy(&self, Parameters(p): Parameters<PolicyParams>) -> Result<CallToolResult, McpError> {
        tools::cloud::policy(&self.store, &p.initiative, p.policy.as_deref())
    }

    #[tool(
        description = "Share a node to the team cloud. Gated: the initiative must be `team` (set via `policy`) AND the node must pass the pre-share secret guard. On success the node is marked shared locally and a copy is pushed to the cloud under the same id. Pass force=true to override the guard. In a multi-cloud setup pass `cloud` to target a specific cloud (default: the configured default)."
    )]
    async fn share(
        &self,
        Parameters(p): Parameters<ShareParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::cloud::share(
            &self.store,
            self.clouds.get(p.cloud.as_deref()),
            &p.name,
            &p.initiative,
            p.force,
        )
        .await
    }

    #[tool(
        description = "List shared nodes the cloud holds for an initiative — discovery for cross-session / cross-user recall. Then `pull` one to bring it into the local vault. In a multi-cloud setup pass `cloud` to target a specific cloud."
    )]
    async fn cloud_recall(
        &self,
        Parameters(p): Parameters<CloudRecallParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::cloud::cloud_recall(self.clouds.get(p.cloud.as_deref()), &p.initiative).await
    }

    #[tool(
        description = "Pull a shared node from the cloud into the local vault by id, attaching it to the given initiative — the recall mechanism for team knowledge you don't have locally yet. In a multi-cloud setup pass `cloud` to target a specific cloud."
    )]
    async fn pull(
        &self,
        Parameters(p): Parameters<PullParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::cloud::pull(
            &self.store,
            self.clouds.get(p.cloud.as_deref()),
            &p.id,
            &p.initiative,
        )
        .await
    }

    #[tool(
        description = "Soft-link a local node to a cloud node by id (dst_store=cloud) — a reference without copying. Resolved lazily via `cloud_links`. Edge type defaults to refers_to. In a multi-cloud setup pass `cloud` to record which cloud the dst lives in."
    )]
    fn link_cloud(
        &self,
        Parameters(p): Parameters<LinkCloudParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::cloud::link_cloud(
            &self.store,
            &self.clouds,
            &p.name,
            &p.cloud_id,
            p.edge_type.as_deref().unwrap_or("refers_to"),
            p.cloud.as_deref(),
            &p.initiative,
        )
    }

    #[tool(
        description = "Resolve a node's cloud soft links — fetch and show the cloud nodes they point to. The lazy-resolution path for soft links. Routes each link to the cloud it was created against (multi-cloud aware)."
    )]
    async fn cloud_links(
        &self,
        Parameters(p): Parameters<CloudLinksParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::cloud::cloud_links(&self.store, &self.clouds, &p.name, &p.initiative).await
    }

    #[tool(
        description = "Batch sync-review of a team initiative's still-local nodes: splits them into PROPOSE SHARE (guard-clean) vs KEEP LOCAL (secret-guard flagged). Review once, then `share` the approved ones — low-friction periodic sharing instead of deciding per capture."
    )]
    fn sync_review(
        &self,
        Parameters(p): Parameters<SyncReviewParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::cloud::sync_review(&self.store, &p.initiative)
    }

    #[tool(
        description = "Rename an initiative — moves all its nodes, edges, and sharing policy to the new name (fails if the new name already exists). Local by default; pass cloud=true to ALSO rename it in the shared cloud (team-wide, affects everyone)."
    )]
    async fn rename_initiative(
        &self,
        Parameters(p): Parameters<RenameInitiativeParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::initiative::rename_initiative(
            &self.store,
            self.clouds.get(None),
            &p.old,
            &p.new,
            p.cloud,
        )
        .await
    }

    #[tool(
        description = "Delete an initiative — drops its scoping and forgets nodes exclusive to it (bi-temporal: recoverable via `at` at a past time). Nodes shared with other initiatives only lose this membership. Local by default; pass cloud=true to ALSO delete it from the shared cloud (team-wide, removes it for everyone)."
    )]
    async fn delete_initiative(
        &self,
        Parameters(p): Parameters<DeleteInitiativeParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::initiative::delete_initiative(&self.store, self.clouds.get(None), &p.name, p.cloud)
            .await
    }

    #[tool(
        description = "Add a node to another initiative (additive multi-membership) — repair initiative fragmentation by giving a node captured under the wrong or a stale initiative a second home, without moving or copying it (same id, edges, history). The node is resolved across all initiatives. Idempotent. Local only."
    )]
    fn attach(&self, Parameters(p): Parameters<AttachParams>) -> Result<CallToolResult, McpError> {
        tools::initiative::attach(&self.store, &p.node, &p.to)
    }

    // ----- Lookup --------------------------------------------------------
    #[tool(description = "Look up a node id by exact name. Returns the id or `(not found)`.")]
    fn recall(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        tools::lookup::recall(&self.store, &p.name, p.initiative.as_deref())
    }

    #[tool(
        description = "Drill into a node — name → brief + 1-hop drill-down children (sources via derived_from, parts via part_of)."
    )]
    fn drill(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        tools::lookup::drill(&self.store, &p.name, p.initiative.as_deref())
    }

    #[tool(
        description = "Walk derived_from ancestors of a node back to its sources — the provenance chain."
    )]
    fn trace(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        tools::lookup::trace(&self.store, &p.name, p.initiative.as_deref())
    }

    #[tool(
        description = "Full-text search across name and body via Cozo FTS. No stemming — search the form you wrote. For inflection-tolerant matching across any language append `*`: `утечк*` finds `утечку`/`утечке`, `token*` finds `tokens`/`tokenize`. Search in the SAME language as the original capture, not in English. Results are ordered by score, then newest-first within equal scores."
    )]
    fn search(&self, Parameters(p): Parameters<SearchParams>) -> Result<CallToolResult, McpError> {
        tools::lookup::search(&self.store, &p.query, p.limit, p.initiative.as_deref())
    }

    #[tool(description = "List archival ideas — long-term cortex memory of stable ideas.")]
    fn ideas(&self, Parameters(p): Parameters<ScopeOnly>) -> Result<CallToolResult, McpError> {
        tools::lookup::ideas(&self.store, p.initiative.as_deref())
    }

    #[tool(description = "List archival outcomes — settled results.")]
    fn outcomes(&self, Parameters(p): Parameters<ScopeOnly>) -> Result<CallToolResult, McpError> {
        tools::lookup::outcomes(&self.store, p.initiative.as_deref())
    }

    #[tool(
        description = "List nodes whose `tags` array contains the given tag — exact match. Common tag families: `kind:<type>` (observation, experiment, idea, reference, …), `sig:<level>` (low/medium/high), `role:<role>` (jot/review/synthesise/revised), `lang:<code>` (ru/en/mixed/other — auto-detected from body), `topic:<word>` (up to 5 content tokens auto-derived from body — same form as in body, no stemming), `status:<state>` (only for hypotheses). For loose matching use the `search` tool with `prefix*` instead. Newest-first when multiple match."
    )]
    fn tagged(&self, Parameters(p): Parameters<TaggedParams>) -> Result<CallToolResult, McpError> {
        tools::lookup::tagged(&self.store, &p.tag, p.initiative.as_deref())
    }

    #[tool(
        description = "Show every edge between two nodes (both directions) at NOW. Answers `why are A and B connected?`."
    )]
    fn between(
        &self,
        Parameters(p): Parameters<BetweenParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::lookup::between(&self.store, &p.a, &p.b, p.initiative.as_deref())
    }

    // ----- Bi-temporal ---------------------------------------------------
    #[tool(
        description = "Read a node IN FULL — every field plus the complete, untruncated body. `drill` / `search` / `recall` only show short excerpts; reach for `at` when you need a node's whole content. Optional `when` time-travels to a past moment (unix seconds, RFC-3339, or `5m` / `2h` ago); omit it for the node as it is now."
    )]
    fn at(&self, Parameters(p): Parameters<AtParams>) -> Result<CallToolResult, McpError> {
        tools::temporal::at(
            &self.store,
            &p.name,
            p.when.as_deref(),
            p.initiative.as_deref(),
        )
    }

    #[tool(
        description = "Print every assertion / retraction recorded for a node, chronologically. + means asserted, - means retracted."
    )]
    fn history(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        tools::temporal::history(&self.store, &p.name, p.initiative.as_deref())
    }

    // ----- Hypothesis cycle ---------------------------------------------
    #[tool(
        description = "Formulate a hypothesis. Auto-named. Optional `about` links via refers_to."
    )]
    fn claim(&self, Parameters(p): Parameters<ClaimParams>) -> Result<CallToolResult, McpError> {
        tools::hypothesis::claim(
            &self.store,
            &p.text,
            p.about.as_deref(),
            p.layer.as_deref(),
            p.initiative.as_deref(),
        )
    }

    #[tool(
        description = "Run an experiment against an open hypothesis. Auto-named from the method body."
    )]
    fn test(&self, Parameters(p): Parameters<TestParams>) -> Result<CallToolResult, McpError> {
        tools::hypothesis::test_hypothesis(
            &self.store,
            &p.hypothesis,
            &p.method,
            p.initiative.as_deref(),
        )
    }

    #[tool(
        description = "Mark a hypothesis as supported, attaching `by` as the verifying evidence."
    )]
    fn confirm(
        &self,
        Parameters(p): Parameters<VerdictParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::hypothesis::confirm(&self.store, &p.hypothesis, &p.by, p.initiative.as_deref())
    }

    #[tool(
        description = "Mark a hypothesis as refuted, attaching `by` as the falsifying counter-evidence."
    )]
    fn refute(&self, Parameters(p): Parameters<VerdictParams>) -> Result<CallToolResult, McpError> {
        tools::hypothesis::refute(&self.store, &p.hypothesis, &p.by, p.initiative.as_deref())
    }

    // ----- Review-flow ---------------------------------------------------
    #[tool(
        description = "Flag a node for review — creates a high-significance review episode + contradicts edge. Target unchanged."
    )]
    fn flag(&self, Parameters(p): Parameters<FlagParams>) -> Result<CallToolResult, McpError> {
        tools::review::flag(&self.store, &p.target, &p.reason, p.initiative.as_deref())
    }

    #[tool(
        description = "Resolve an open question by recording that `by` answers it (creates a supersedes edge)."
    )]
    fn resolve(
        &self,
        Parameters(p): Parameters<ResolveParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::review::resolve(&self.store, &p.question, &p.by, p.initiative.as_deref())
    }

    #[tool(
        description = "Close an open review on a node — retracts its contradicts edge(s) so it leaves the review queue; the doubt stays in history. Optional `resolution` note is recorded as provenance."
    )]
    fn close_review(
        &self,
        Parameters(p): Parameters<CloseReviewParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::review::close_review(
            &self.store,
            &p.target,
            p.resolution.as_deref(),
            p.initiative.as_deref(),
        )
    }

    // ----- Consolidation -------------------------------------------------
    #[tool(
        description = "Promote operational draft → archival counterpart. Provenance via derived_from is replicated across the tier."
    )]
    fn settle(
        &self,
        Parameters(p): Parameters<ConsolidateParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::consolidate::settle(
            &self.store,
            &p.source,
            &p.new_type,
            &p.new_name,
            &p.new_body,
            p.initiative.as_deref(),
        )
    }

    #[tool(
        description = "Bring an archival node back into the operational tier (mirror of `settle`)."
    )]
    fn reopen(
        &self,
        Parameters(p): Parameters<ConsolidateParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::consolidate::reopen(
            &self.store,
            &p.source,
            &p.new_type,
            &p.new_name,
            &p.new_body,
            p.initiative.as_deref(),
        )
    }

    #[tool(
        description = "Many-to-one consolidation — create a new node from several seeds, with derived_from edges to each."
    )]
    fn synthesise(
        &self,
        Parameters(p): Parameters<SynthesiseParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::consolidate::synthesise(
            &self.store,
            &p.from,
            &p.new_type,
            &p.new_name,
            &p.new_body,
            p.tier.as_deref(),
            p.initiative.as_deref(),
        )
    }

    #[tool(
        description = "Replace a node with a fresh one carrying new content, connected by a supersedes edge. Use when the change is large enough to warrant a new identity."
    )]
    fn supersede(
        &self,
        Parameters(p): Parameters<SupersedeParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::consolidate::supersede(
            &self.store,
            &p.old,
            &p.new_type,
            &p.new_name,
            &p.new_body,
            p.tier.as_deref(),
            p.initiative.as_deref(),
        )
    }

    // ----- Tasks (todos) -------------------------------------------------
    #[tool(
        description = "Capture a todo as a Task node. Auto-named from body. Tags: kind:task, status:open, optional due:YYYY-MM-DD. `due` accepts ISO date, RFC-3339, or future duration like `3d`/`2w`."
    )]
    fn task(&self, Parameters(p): Parameters<TaskParams>) -> Result<CallToolResult, McpError> {
        tools::task::task(
            &self.store,
            &p.body,
            p.due.as_deref(),
            p.layer.as_deref(),
            p.initiative.as_deref(),
        )
    }

    #[tool(
        description = "Mark a task done — RMW retract+reassert with status:done, preserving id and name. Accepts task name or UUIDv7 id."
    )]
    fn done(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        tools::task::done(&self.store, &p.name, p.initiative.as_deref())
    }

    // ----- Task board ----------------------------------------------------
    #[tool(
        description = "Show the initiative's task board: status columns (from its registry, in order, empties included) with the tasks bucketed into them. Requires `initiative`. Optional `when` rewinds the whole board — columns and cards — to a past moment (unix seconds, RFC-3339, or `5m` / `2h` ago)."
    )]
    fn board(&self, Parameters(p): Parameters<BoardParams>) -> Result<CallToolResult, McpError> {
        tools::board::board(&self.store, p.initiative.as_deref(), p.when.as_deref())
    }

    #[tool(
        description = "Move a task to a board column: sets its `status:<key>`, strictly validated against the initiative's board registry (unknown status is refused). The general form of `done`."
    )]
    fn set_status(
        &self,
        Parameters(p): Parameters<SetStatusParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::board::set_status(&self.store, &p.task, &p.status, p.initiative.as_deref())
    }

    #[tool(
        description = "Customize the initiative's board columns: action add (key,label?) / remove (key) / relabel (key,label) / reorder (order = all keys permuted). The board is created from defaults [open, in-progress, done] on first edit."
    )]
    fn board_status(
        &self,
        Parameters(p): Parameters<BoardStatusParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::board::board_status(
            &self.store,
            p.initiative.as_deref(),
            &p.action,
            p.key.as_deref(),
            p.label.as_deref(),
            p.order.as_deref(),
        )
    }

    // ----- Metabolism ----------------------------------------------------
    #[tool(
        description = "Bi-temporal forget — retract a node and every edge connected to it. Historical reads still see it; reads at NOW skip."
    )]
    fn forget(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        tools::metabolism::forget(&self.store, &p.name, p.initiative.as_deref())
    }

    #[tool(
        description = "Set a node's memory layer — controls recall priority (injected Core → Hot → Warm → Cold → Frozen). Accepts name or id; layer one of core/hot/warm/cold/frozen."
    )]
    fn layer(&self, Parameters(p): Parameters<LayerParams>) -> Result<CallToolResult, McpError> {
        tools::metabolism::set_layer(&self.store, &p.name, &p.layer, p.initiative.as_deref())
    }

    #[tool(
        description = "Rewrite a node's body and/or rename. Implemented as retract+reassert so history sees both versions."
    )]
    fn revise(&self, Parameters(p): Parameters<ReviseParams>) -> Result<CallToolResult, McpError> {
        tools::metabolism::revise(
            &self.store,
            &p.name,
            p.body.as_deref(),
            p.rename.as_deref(),
            p.initiative.as_deref(),
        )
    }

    // ----- Diagnostics / snapshot ---------------------------------------
    #[tool(
        description = "Diagnostic snapshot — orphan nodes (no edges) and unresolved reviews (inbound contradicts)."
    )]
    fn lint(&self, Parameters(p): Parameters<ScopeOnly>) -> Result<CallToolResult, McpError> {
        tools::lint::lint(&self.store, p.initiative.as_deref())
    }

    #[tool(
        description = "Reflect on the store: a computed maintenance work-list with how to act on each part — orphans to link, open reviews to resolve, chains gone stale (rechain), settled operational nodes to promote into cortex, and shared/cloud items that need YOUR sign-off (never auto-rebalanced). Good for a periodic tidy pass (e.g. a cron)."
    )]
    fn reflect(&self, Parameters(p): Parameters<ScopeOnly>) -> Result<CallToolResult, McpError> {
        tools::lint::reflect(&self.store, p.initiative.as_deref())
    }

    #[tool(
        description = "Snapshot the substrate as an Obsidian-friendly markdown vault (README + INDEX + LOG + pages). Output dir is created if missing."
    )]
    fn export(&self, Parameters(p): Parameters<ExportParams>) -> Result<CallToolResult, McpError> {
        tools::vault::export(&self.store, &p.output_dir, p.initiative.as_deref())
    }
}

#[tool_handler]
impl ServerHandler for KaeruServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder().enable_tools().build(),
        )
        .with_server_info(Implementation::from_build_env())
        .with_protocol_version(ProtocolVersion::LATEST)
        .with_instructions(
            "kaeru — your memory of record across sessions. Prefer it to scratchpads and note files. If your \
             runtime ships its own memory, treat it as a pointer here so knowledge doesn't fork.\n\nENTRY: \
             `initiatives` → `awake` (what was open) → `overview` (what the project knows). Pass \
             `initiative` on EVERY call — without it writes stay untagged and reads span projects.\n\nTWO \
             TIERS: operational (in-flight: observations, claims, open questions) → archival (settled: \
             outcomes, ideas, references). `settle` promotes what stopped changing; `awake` loads archival \
             as `cortex` every session. `layer=core` is injected uncapped — reserve it for the few \
             always-needed facts.\n\nCAPTURE by epistemic status, not length: `jot` fleeting note · \
             `episode` observation tied to current work · `cite <name> --body` settled doc kept verbatim, \
             URL optional — ADRs, specs, glossaries, YOUR OWN settled docs · \
             `claim`→`test`→`confirm`/`refute` hypotheses · `task`/`done` todos with deadlines. Don't \
             capture everything as `episode`.\n\nALWAYS LINK a new node: `search` for related → `link a b \
             --edge_type`. Types: refers_to (default), causal, derived_from, contradicts, part_of, blocks, \
             targets, supersedes, verifies, falsifies, temporal. `strong=true` on load-bearing edges. An \
             island is found only by exact name.\n\nTHEN CHAIN: once work runs observation→decision, `chain \
             from to --summary` saves that trail; `chains <node>` lists them, `read_chain` replays one. A \
             chain is the WHY a fresh agent reads.\n\nREAD: `recall` exact name · `search q*` fuzzy (`*` \
             matches inflections) · `drill` node+children · `at <name>` FULL text — drill/search show \
             excerpts only · `at <name> when=2h` past state · `history` versions · `trace` provenance · \
             `between` how two nodes connect · `tagged \"topic:x\"` · `surface layers=cold` archived · \
             `board` open tasks.\n\nLANGUAGE: store and search in the user's own language; never translate \
             on capture or lookup.",
        )
    }
}

#[cfg(test)]
mod tests {
    use rmcp::ServerHandler;

    use super::*;

    /// Claude Code truncates MCP server instructions at roughly 2048
    /// characters, silently and mid-word. The ontology used to be 4434 chars
    /// long, so 53% of it — chains, tags, the search idiom, half the capture
    /// dispatch — never reached any agent, and the verbs described in that
    /// tail were measurably the dead ones (#48).
    ///
    /// The budget is what makes the text land, so it is asserted rather than
    /// documented: an instruction block that grows past it is a regression, not
    /// a style question.
    #[test]
    fn instructions_fit_the_client_truncation_budget() {
        const BUDGET: usize = 2048;
        let store = Store::open_in_memory().expect("open");
        let server = KaeruServer::new(
            store,
            CloudRegistry::default(),
            CancellationToken::new(),
            false,
        );
        let instructions = server
            .get_info()
            .instructions
            .expect("the server ships instructions");

        assert!(
            instructions.len() <= BUDGET,
            "instructions are {} chars, {} over the ~{BUDGET}-char client limit — \
             everything past the cut is invisible to the agent",
            instructions.len(),
            instructions.len() - BUDGET
        );
    }

    /// The tail is where the previously-truncated verbs live. If the text is
    /// ever re-expanded, this is the half that disappears first — so assert the
    /// weakest-covered ones are present at all.
    #[test]
    fn instructions_still_name_the_easily_lost_verbs() {
        let store = Store::open_in_memory().expect("open");
        let server = KaeruServer::new(
            store,
            CloudRegistry::default(),
            CancellationToken::new(),
            false,
        );
        let text = server.get_info().instructions.unwrap_or_default();
        for verb in [
            "chain",
            "chains",
            "read_chain",
            "tagged",
            "trace",
            "surface",
            "between",
            "at",
            "history",
            "cite",
            "task",
        ] {
            assert!(
                text.contains(verb),
                "`{verb}` is not mentioned — an unmentioned verb is one an agent never learns"
            );
        }
    }
}
