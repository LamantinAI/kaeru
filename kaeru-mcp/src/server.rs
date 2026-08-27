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

use crate::cloud_client::{CloudClient, CloudRegistry};
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

impl KaeruServer {
    /// The cloud an operation should act on, or a refusal the caller can hand
    /// straight back to the agent.
    ///
    /// Every cloud-touching tool goes through here rather than
    /// `clouds.get(None)`, so that "which cloud" is answered in one place and
    /// answered out loud — see `CloudRegistry::resolve`.
    fn cloud_for(&self, name: Option<&str>) -> Result<&CloudClient, McpError> {
        self.clouds
            .resolve(name)
            .map_err(|msg| McpError::invalid_params(msg, None))
    }

    /// The cloud a capture verb should publish to — resolved **only** when the
    /// caller actually asked to share.
    ///
    /// A plain local capture must not be refused for failing to name a cloud
    /// it was never going to touch; a `visibility=shared` capture must not be
    /// published to a cloud nobody named. Resolving lazily is what keeps both
    /// true.
    fn cloud_when_sharing(
        &self,
        visibility: Option<&str>,
        name: Option<&str>,
    ) -> Result<Option<&CloudClient>, McpError> {
        if !crate::utils::parse_wants_shared(visibility)? {
            return Ok(None);
        }
        self.cloud_for(name).map(Some)
    }

    /// The cloud an initiative-wide verb should also act on, or `None` for
    /// local-only.
    ///
    /// These used to take `cloud: bool`, which could only ever mean "and the
    /// default one" — an implicit target for the two operations whose own
    /// descriptions say "team-wide" and "removes it for everyone". A name is
    /// required instead, and omitting it means local-only rather than "guess".
    fn cloud_when_named(&self, name: Option<&str>) -> Result<Option<&CloudClient>, McpError> {
        match name {
            None => Ok(None),
            Some(n) => self.cloud_for(Some(n)).map(Some),
        }
    }
}

#[tool_router]
impl KaeruServer {
    // ----- Re-entry / session -------------------------------------------
    #[tool(
        description = "Restore session context: pinned set, recent episodes (24h), open reviews, plus the read-back of unfinished work — open tasks (overdue first), claims awaiting a verdict, and the saved reasoning trails. Run this when re-entering a project."
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
        description = "Show resolved configuration: vault path, the configured clouds and which is default, and every cap (initiative not relevant)."
    )]
    fn config(&self) -> Result<CallToolResult, McpError> {
        tools::session::config(&self.store, &self.clouds)
    }

    #[tool(
        description = "List the clouds this daemon can reach, with their endpoints and which one is default. Ask this before any cloud verb in an unfamiliar setup: with more than one cloud configured, `share` / `pull` / `cloud_recall` and the initiative verbs require `cloud` named explicitly, and nothing is routed to a default you did not choose."
    )]
    fn clouds(&self) -> Result<CallToolResult, McpError> {
        tools::session::clouds(&self.clouds)
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
            self.cloud_when_sharing(p.visibility.as_deref(), p.cloud.as_deref())?,
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
            self.cloud_when_sharing(p.visibility.as_deref(), p.cloud.as_deref())?,
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
        description = "Why is this here? Reads the saved reasoning that leads to a node — the state → reasoning → decision trail, not an isolated record. Give it a chain to read its ordered steps, or any node to see the chain it belongs to (read directly when there is only one, listed for triage when there are several). Replaces the former `chains` + `read_chain` pair."
    )]
    fn why(&self, Parameters(p): Parameters<WhyParams>) -> Result<CallToolResult, McpError> {
        tools::chain::why(&self.store, &p.name, p.initiative.as_deref())
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
        description = "Compute the shortest weighted path between two nodes WITHOUT writing anything — a preview. Edge weight is the cost, so stronger links (`link --strong`) make shorter paths. `chain` saves the same path as a recallable trail; use `path` to look first when you are not sure the two are meaningfully connected."
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
            self.cloud_when_sharing(p.visibility.as_deref(), p.cloud.as_deref())?,
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
        description = "Read or set an initiative's cloud sharing policy (Gate 1). Omit both arguments to read. `policy` says WHETHER it may leave: private (default for any initiative — never leaves), team (shared nodes may sync), ask. `clouds` says WHERE TO: a comma-separated list restricting the initiative to those clouds, empty string to clear. An initiative with no list may go to any configured cloud, so this changes nothing until you ask for it."
    )]
    fn policy(&self, Parameters(p): Parameters<PolicyParams>) -> Result<CallToolResult, McpError> {
        tools::cloud::policy(
            &self.store,
            &p.initiative,
            p.policy.as_deref(),
            p.clouds.as_deref(),
        )
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
            Some(self.cloud_for(p.cloud.as_deref())?),
            &p.name,
            &p.initiative,
            p.force,
        )
        .await
    }

    #[tool(
        description = "Withdraw a node from a cloud: retracts the cloud copy and marks the node local again. The inverse of `share`, which had none. Use it for a node sent to the wrong cloud, one the pre-share guard should have caught, or anything that should not have left the machine. The cloud retraction is bi-temporal — the node leaves `cloud_recall` and the listings while its history stays intact. To CORRECT a shared node instead of withdrawing it, `revise` it and `share` again: the push is an upsert under the same id."
    )]
    async fn unshare(
        &self,
        Parameters(p): Parameters<UnshareParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::cloud::unshare(
            &self.store,
            Some(self.cloud_for(p.cloud.as_deref())?),
            &p.name,
            &p.initiative,
        )
        .await
    }

    #[tool(
        description = "List the initiatives a cloud holds, with how many nodes each has shared. The map of the second tier: use it when you don't know what the team has, before `cloud_recall` on one of them. Note an initiative in the cloud is INDEPENDENT of the local one with the same name — same name, different contents. (`clouds` lists the clouds this daemon can reach; this lists what is inside one.)"
    )]
    async fn cloud_initiatives(
        &self,
        Parameters(p): Parameters<CloudScopeParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::cloud::cloud_initiatives(Some(self.cloud_for(p.cloud.as_deref())?)).await
    }

    #[tool(
        description = "SEARCH or list what the cloud holds for an initiative — the second tier your local `search` cannot reach. Pass `query` to match shared names and excerpts; omit it to list everything. Then `pull <id>` brings one into the local vault. Reach for this whenever a `team` initiative's local answer looks complete: the cloud may hold nodes this machine has never seen. Paged at 25, reporting the true total and the exact call for the next page. In a multi-cloud setup `cloud` is required."
    )]
    async fn cloud_recall(
        &self,
        Parameters(p): Parameters<CloudRecallParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::cloud::cloud_recall(
            Some(self.cloud_for(p.cloud.as_deref())?),
            &p.initiative,
            p.query.as_deref(),
            p.limit,
            p.offset,
        )
        .await
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
            Some(self.cloud_for(p.cloud.as_deref())?),
            &p.id,
            &p.initiative,
        )
        .await
    }

    #[tool(
        description = "Soft-link a local node to a cloud node by id — a reference, with NO copy in your vault. Use it instead of `pull` when the cloud node is someone else's to maintain and you only need to point at it: a pull makes a copy that silently goes stale when the owner revises it, while a soft link resolves live through `cloud_links`. Pull when you need the content locally; link when you need the citation. Edge type defaults to refers_to; in a multi-cloud setup pass `cloud` to record where the dst lives."
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
        description = "Resolve a node's cloud soft links — fetches the cloud nodes they point at, live, so you see what they say NOW rather than what they said when the link was made. The read half of `link_cloud`. Routes each link to the cloud it was created against."
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
        description = "Rename an initiative — moves all its nodes, edges, and sharing policy to the new name (fails if the new name already exists). Local by default. Pass `cloud=\"<name>\"` to ALSO rename it in that shared cloud, which is team-wide and affects everyone; with several clouds configured the name is required, because this cannot be undone in the wrong one."
    )]
    async fn rename_initiative(
        &self,
        Parameters(p): Parameters<RenameInitiativeParams>,
    ) -> Result<CallToolResult, McpError> {
        let cloud = self.cloud_when_named(p.cloud.as_deref())?;
        tools::initiative::rename_initiative(&self.store, cloud, &p.old, &p.new, cloud.is_some())
            .await
    }

    #[tool(
        description = "Delete an initiative — drops its scoping and forgets nodes exclusive to it (bi-temporal: recoverable via `at` at a past time). Nodes shared with other initiatives only lose this membership. Local by default. Pass `cloud=\"<name>\"` to ALSO delete it from that shared cloud, which removes it for everyone and CANNOT be undone there; with several clouds configured the name is required rather than defaulted."
    )]
    async fn delete_initiative(
        &self,
        Parameters(p): Parameters<DeleteInitiativeParams>,
    ) -> Result<CallToolResult, McpError> {
        let cloud = self.cloud_when_named(p.cloud.as_deref())?;
        tools::initiative::delete_initiative(&self.store, cloud, &p.name, cloud.is_some()).await
    }

    #[tool(
        description = "Add a node to another initiative (additive multi-membership) — repair initiative fragmentation by giving a node captured under the wrong or a stale initiative a second home, without moving or copying it (same id, edges, history). The node is resolved across all initiatives. Idempotent. Local only."
    )]
    fn attach(&self, Parameters(p): Parameters<AttachParams>) -> Result<CallToolResult, McpError> {
        tools::initiative::attach(&self.store, &p.node, &p.to)
    }

    // ----- Lookup --------------------------------------------------------
    #[tool(
        description = "Look up a node id by EXACT name — no fuzziness, no stemming. Returns the id alone, so follow it with `at <name>` for the full text or `drill <name>` for its neighbours. When you don't know the exact name, `search` is the verb; a miss here tells you whether the name lives in another initiative, is spelled differently, or is absent."
    )]
    fn recall(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        tools::lookup::recall(&self.store, &p.name, p.initiative.as_deref())
    }

    #[tool(
        description = "Drill into a node — its brief plus one hop of children (sources via derived_from, parts via part_of). The fast way to see what a memory is attached to. Bodies come back as EXCERPTS: use `at <name>` when you need the whole text, and `between a b` when you want the edges rather than the neighbours."
    )]
    fn drill(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        tools::lookup::drill(&self.store, &p.name, p.initiative.as_deref())
    }

    #[tool(
        description = "Walk derived_from ancestors of a node back to its sources — where a conclusion CAME FROM. Use it before trusting a synthesised or settled node: it shows the raw material the claim was built on. `why` is the sibling verb for the reasoning trail; `trace` is the material one."
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

    #[tool(
        description = "List the initiative's archival IDEAS — proposals that settled without yet becoming results. Part of the cortex `awake` loads every session, so this is the deliberate deep read when you want them all rather than the layered slice. `outcomes` is the sibling for results; `settle` is how a node gets here."
    )]
    fn ideas(&self, Parameters(p): Parameters<ScopeOnly>) -> Result<CallToolResult, McpError> {
        tools::lookup::ideas(&self.store, p.initiative.as_deref())
    }

    #[tool(
        description = "List the initiative's archival OUTCOMES — what the work actually concluded. This is the highest-value read for a fresh agent: results, not the working notes that produced them. `trace` walks any of them back to its sources; `ideas` lists the proposals that have not become results yet."
    )]
    fn outcomes(&self, Parameters(p): Parameters<ScopeOnly>) -> Result<CallToolResult, McpError> {
        tools::lookup::outcomes(&self.store, p.initiative.as_deref())
    }

    #[tool(
        description = "List nodes whose `tags` array contains the given tag — exact match. Common tag families: `kind:<type>` (observation, experiment, idea, reference, …), `sig:<level>` (low/medium/high), `role:<role>` (jot/review/synthesise/revised), `lang:<code>` (ru/en/mixed/other — auto-detected from body), `topic:<word>` (up to 5 auto-derived tokens — a node\'s MOST-MENTIONED words, weighted toward a name somebody chose; a compound like `figma-макет` is also tagged by its parts), `status:<state>` (hypotheses and tasks). Exact match, no stemming — but a miss comes back with the near tags that DO exist in scope, so an empty answer tells you what to ask instead. For loose matching over text use `search prefix*`. Newest-first when multiple match."
    )]
    fn tagged(&self, Parameters(p): Parameters<TaggedParams>) -> Result<CallToolResult, McpError> {
        tools::lookup::tagged(&self.store, &p.tag, p.initiative.as_deref())
    }

    #[tool(
        description = "Show every edge between two nodes, both directions, at NOW — answers \"are A and B connected, and how?\". Edges are typed (refers_to, causal, derived_from, contradicts, part_of, blocks, targets, supersedes, verifies, falsifies, temporal), so the answer says what KIND of connection it is. `path` finds a route when there is no direct edge; `drill` lists neighbours rather than edges."
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
        description = "Print every assertion / retraction recorded for a node, chronologically — `+` asserted, `-` retracted. This is how you see that a node CHANGED, and when. Accepts a former name too: a node renamed by `revise` or `supersede` is still reachable by the name it used to carry. Pair with `at <name> when=<t>` to read any of those versions in full."
    )]
    fn history(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        tools::temporal::history(&self.store, &p.name, p.initiative.as_deref())
    }

    // ----- Hypothesis cycle ---------------------------------------------
    #[tool(
        description = "Record a hypothesis. Auto-named. If you ALREADY know how it turned out — the usual case, since you reach memory after the check has run — pass `verdict` (supported/refuted/inconclusive) and `by` (the evidence node) and it lands settled in this one call. Without a verdict it is an open question, and `awake` will keep surfacing it until one arrives. Optional `about` links via refers_to."
    )]
    fn claim(&self, Parameters(p): Parameters<ClaimParams>) -> Result<CallToolResult, McpError> {
        tools::hypothesis::claim(
            &self.store,
            &p.text,
            p.about.as_deref(),
            p.verdict.as_deref(),
            p.by.as_deref(),
            p.layer.as_deref(),
            p.initiative.as_deref(),
        )
    }

    #[tool(
        description = "Record what you actually checked, and attach it to a hypothesis. Past tense — this documents a check that already ran, it does not schedule one (and it is not `cargo test`). Pass `method` to write the result up as a new experiment node, or `node` to point at something you already captured."
    )]
    fn evidence(
        &self,
        Parameters(p): Parameters<EvidenceParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::hypothesis::evidence(
            &self.store,
            &p.hypothesis,
            p.method.as_deref(),
            p.node.as_deref(),
            p.initiative.as_deref(),
        )
    }

    #[tool(
        description = "Mark a hypothesis as supported. `by` (the verifying evidence node) is optional — record the verdict even with nothing to point at yet rather than leaving the claim open with the answer buried in its text."
    )]
    fn confirm(
        &self,
        Parameters(p): Parameters<VerdictParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::hypothesis::confirm(
            &self.store,
            &p.hypothesis,
            p.by.as_deref(),
            p.initiative.as_deref(),
        )
    }

    #[tool(
        description = "Mark a hypothesis as refuted. `by` (the falsifying counter-evidence node) is optional, same as for `confirm`."
    )]
    fn refute(&self, Parameters(p): Parameters<VerdictParams>) -> Result<CallToolResult, McpError> {
        tools::hypothesis::refute(
            &self.store,
            &p.hypothesis,
            p.by.as_deref(),
            p.initiative.as_deref(),
        )
    }

    #[tool(
        description = "Mark a hypothesis as inconclusive — the check ran and did not decide. A real third verdict, not a failure to answer: it closes the claim out of the open queue while recording that the question stayed open on the merits. Writes no verdict edge, so `by` is not needed."
    )]
    fn inconclusive(
        &self,
        Parameters(p): Parameters<VerdictParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::hypothesis::inconclusive(
            &self.store,
            &p.hypothesis,
            p.by.as_deref(),
            p.initiative.as_deref(),
        )
    }

    // ----- Review-flow ---------------------------------------------------
    #[tool(
        description = "Flag a node as doubtful — writes a review episode carrying your REASON and a contradicts edge to the target. The target itself is untouched: the doubt is recorded beside it, not written into it. It then shows up in `awake`'s under-review list until `close_review` or `resolve` settles it. Not the same as `link contradicts`, which records the edge without the reason."
    )]
    fn flag(&self, Parameters(p): Parameters<FlagParams>) -> Result<CallToolResult, McpError> {
        tools::review::flag(&self.store, &p.target, &p.reason, p.initiative.as_deref())
    }

    #[tool(
        description = "Resolve an open question by recording that `by` answers it — a supersedes edge from the answer to the question, so the question stays readable as history instead of being deleted. For a doubt raised by `flag`, `close_review` is the matching verb."
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
        description = "Promote a node that stopped changing from the operational tier into the archival one — this is how knowledge hardens. `settle <name>` ALONE IS ENOUGH: with no new_* the node keeps its name and its full body, and the type is derived (episode/task/experiment/hypothesis → outcome; draft/scratch → idea). Provenance via derived_from is replicated across the tier boundary, and manual tags come with it. Don't demote finished work to a cold layer instead — a layer is how eagerly a node loads, a tier is whether it is still in flight."
    )]
    fn settle(
        &self,
        Parameters(p): Parameters<ConsolidateParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::consolidate::settle(
            &self.store,
            &p.source,
            p.new_type.as_deref(),
            p.new_name.as_deref(),
            p.new_body.as_deref(),
            p.initiative.as_deref(),
        )
    }

    #[tool(
        description = "Bring an archival node back into the operational tier — `settle`'s mirror, for settled knowledge that turned out to still be in flight. `unsettle <name>` alone is enough: name, body and type all carry over unless you say otherwise."
    )]
    fn unsettle(
        &self,
        Parameters(p): Parameters<ConsolidateParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::consolidate::unsettle(
            &self.store,
            &p.source,
            p.new_type.as_deref(),
            p.new_name.as_deref(),
            p.new_body.as_deref(),
            p.initiative.as_deref(),
        )
    }

    #[tool(
        description = "Many-to-one consolidation — write one durable node from several seeds, with a derived_from edge to each so `trace` can walk back to them. Use when scattered observations have converged into a single finding; `settle` is the one-to-one version, which promotes a single node in place."
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
        description = "Replace a node with a fresh one carrying new content, connected by a supersedes edge. Use when the change is large enough to warrant a new identity; `revise` edits in place instead. new_type is optional — it defaults to the old node's."
    )]
    fn supersede(
        &self,
        Parameters(p): Parameters<SupersedeParams>,
    ) -> Result<CallToolResult, McpError> {
        tools::consolidate::supersede(
            &self.store,
            &p.old,
            p.new_type.as_deref(),
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
        description = "Mark a task done — the id, name and manual tags survive; only the status moves, and `history` shows the transition. Accepts a task name or a UUIDv7 id. For any other column of the initiative's board use `set_status`; `done` is the shortcut for the terminal one."
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
        description = "Rewrite a node's body and/or rename it IN PLACE — the id survives, and `history` shows both versions. This is the verb for correcting or extending a node. Use `supersede` instead when the change is big enough to deserve a new identity, and `settle` when the node is not changing at all, just finished."
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
        description = "Diagnostic snapshot of graph hygiene: orphan nodes (no edges at all), unresolved reviews, and dangling edges whose endpoint was retracted. Read-only. `reflect` is the fuller version — it pairs each finding with what to do about it, and adds overdue tasks, stale chains and cortex candidates."
    )]
    fn lint(&self, Parameters(p): Parameters<ScopeOnly>) -> Result<CallToolResult, McpError> {
        tools::lint::lint(&self.store, p.initiative.as_deref())
    }

    #[tool(
        description = "Reflect on the store: a computed maintenance work-list with how to act on each part — orphans to link, open reviews to resolve, chains gone stale (rechain), settled operational nodes to promote into cortex, and shared/cloud items that need YOUR sign-off (never auto-rebalanced). Run it when a piece of work ends, and before a session stops — that is the moment nothing else marks."
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
            "kaeru — your memory of record across sessions. Prefer it to scratchpads and notes. If your runtime \
             has its own memory, point it here so knowledge doesn't fork.\n\nENTRY: \
             `initiatives` → `awake` (what was open) → `overview` (what it knows). Pass \
             `initiative` on EVERY call — without it writes stay untagged and reads span projects.\n\nTWO \
             TIERS: operational (in-flight: observations, claims, open questions) → archival (settled: \
             outcomes, ideas, references). `settle <name>` ALONE promotes what stopped changing — name and \
             body carry over; don't demote finished work to `cold` instead. `awake` loads archival \
             as `cortex` every session. `layer=core` is injected uncapped — reserve it for the few \
             always-needed facts.\n\nCAPTURE by epistemic status, not length: `jot` fleeting note · \
             `episode` observation tied to current work · `cite <name> --body` settled doc kept verbatim, \
             URL optional — ADRs, specs, glossaries, YOUR OWN settled docs · \
             `claim --verdict refuted --by <ev>` — the answer WITH the claim, you usually know it \
             already · `task`/`done` todos with deadlines. Don't capture everything as `episode`.\n\nALWAYS LINK a new node: `search` for related → `link a b \
             --edge_type`. Types: refers_to (default), causal, derived_from, contradicts, part_of, blocks, \
             targets, supersedes, verifies, falsifies, temporal. `strong=true` on load-bearing edges. An \
             island is found only by exact name.\n\nTHEN CHAIN: once work runs observation→decision, `chain \
             from to --summary` saves that trail; `why <node>` reads the reasoning that leads there. A \
             chain is the WHY a fresh agent reads.\n\nREAD: `recall` exact name · `search q*` fuzzy · `drill` node+children · `at <name>` FULL text — drill/search show \
             excerpts only · `at <name> when=2h` past state · `history` versions · `trace` provenance · \
             `between` how two nodes connect · `tagged \"topic:x\"` · `surface layers=cold` archived · \
             `board` open tasks.\n\nCLOUD: a shared initiative has a team tier: `cloud_recall`.\n\nLANGUAGE: store and search in \
             the user's language; never translate.",
        )
    }
}

#[cfg(test)]
mod tests {
    use rmcp::ServerHandler;

    use super::*;

    /// The instructions had to lose most of the ontology to fit the client's
    /// truncation budget, and the deal was that the displaced detail moves
    /// into the `#[tool]` descriptions — which the client does NOT truncate
    /// (#48). For a while it was only half a deal: the text got shorter
    /// without landing anywhere else.
    ///
    /// This pins the second half. Each entry is a concept that used to live in
    /// the instructions and now has to be carried by some tool's own
    /// description. The assertion is deliberately about the surface as a
    /// whole, not about which tool says it — where a concept belongs is a
    /// judgement call, whether it is said at all is not.
    #[test]
    fn the_displaced_ontology_lives_in_the_tool_descriptions() {
        let router = KaeruServer::tool_router();
        let surface: String = router
            .list_all()
            .iter()
            .filter_map(|t| t.description.as_deref().map(str::to_string))
            .collect::<Vec<_>>()
            .join("\n");

        for (concept, needle) in [
            ("prefix search for inflections", "`*`"),
            ("edge types are a closed set", "contradicts"),
            ("strong edges shorten chains", "strong"),
            ("excerpts are not the full text", "EXCERPT"),
            ("a chain is a reasoning trail", "trail"),
            ("provenance walks derived_from", "derived_from"),
            ("the two tiers", "archival"),
            ("the memory layers", "cold"),
            ("tasks carry deadlines", "due"),
            ("capture by epistemic status", "hypothesis"),
            ("nodes resolve by id as well as name", "UUIDv7"),
        ] {
            assert!(
                surface.contains(needle),
                "the tool descriptions no longer teach {concept:?} \
                 (looked for {needle:?}) — it was displaced from the \
                 instructions and has to be carried here"
            );
        }
    }

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
            "chain", "why", "tagged", "trace", "surface", "between", "at", "history", "cite",
            "task",
        ] {
            assert!(
                text.contains(verb),
                "`{verb}` is not mentioned — an unmentioned verb is one an agent never learns"
            );
        }
    }
}
