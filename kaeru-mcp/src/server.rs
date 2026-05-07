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

use rmcp::ErrorData as McpError;
use rmcp::ServerHandler;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::model::Implementation;
use rmcp::model::ProtocolVersion;
use rmcp::model::ServerCapabilities;
use rmcp::model::ServerInfo;
use rmcp::tool;
use rmcp::tool_handler;
use rmcp::tool_router;

use kaeru_core::Store;

use crate::params::*;
use crate::tools;

#[derive(Clone)]
pub struct KaeruServer {
    store: Arc<Store>,
    /// Filled by `Self::tool_router()` (macro-generated); read by the
    /// `#[tool_handler]`-generated `ServerHandler` impl, but the
    /// dead-code analyser doesn't see that path.
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
}

impl KaeruServer {
    pub fn new(store: Store) -> Self {
        Self {
            store: Arc::new(store),
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl KaeruServer {
    // ----- Re-entry / session -------------------------------------------
    #[tool(description = "Restore session context: pinned set, recent episodes (24h), open reviews. Run this when re-entering a project.")]
    fn awake(&self, Parameters(p): Parameters<ScopeOnly>) -> Result<CallToolResult, McpError> {
        tools::session::awake(&self.store, p.initiative.as_deref())
    }

    #[tool(description = "Print a terminal-readable map of the substrate: counts by tier/type, provenance forests, open questions, edge stats.")]
    fn overview(&self, Parameters(p): Parameters<ScopeOnly>) -> Result<CallToolResult, McpError> {
        tools::session::overview(&self.store, p.initiative.as_deref())
    }

    #[tool(description = "List initiatives that have at least one node attached. Use this first when re-entering, then pick one for subsequent calls.")]
    fn initiatives(&self) -> Result<CallToolResult, McpError> {
        tools::session::initiatives(&self.store)
    }

    #[tool(description = "List episodes whose latest assertion is within the time window (defaults 24h). Use `since` like `30m`, `3h`, `2d`, or raw seconds.")]
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

    #[tool(description = "Show resolved configuration: vault path and every cap (initiative not relevant).")]
    fn config(&self) -> Result<CallToolResult, McpError> {
        tools::session::config(&self.store)
    }

    // ----- Capture -------------------------------------------------------
    #[tool(description = "Write a deliberately-named operational episode. Use when you know you'll want to recall by exact name.")]
    fn episode(&self, Parameters(p): Parameters<EpisodeParams>) -> Result<CallToolResult, McpError> {
        tools::capture::episode(&self.store, &p.name, &p.body, p.initiative.as_deref())
    }

    #[tool(description = "Low-friction episode write — auto-named from body's first words plus a unique id suffix. Defaults to observation/low.")]
    fn jot(&self, Parameters(p): Parameters<JotParams>) -> Result<CallToolResult, McpError> {
        tools::capture::jot(&self.store, &p.body, p.initiative.as_deref())
    }

    #[tool(description = "Create a typed edge between two named nodes. Edge type defaults to `refers_to`.")]
    fn link(&self, Parameters(p): Parameters<LinkParams>) -> Result<CallToolResult, McpError> {
        tools::capture::link(&self.store, &p.from, &p.to, &p.edge_type, p.initiative.as_deref())
    }

    #[tool(description = "Retract a previously-asserted edge. Bi-temporal — historical reads still see it.")]
    fn unlink(&self, Parameters(p): Parameters<LinkParams>) -> Result<CallToolResult, McpError> {
        tools::capture::unlink(&self.store, &p.from, &p.to, &p.edge_type, p.initiative.as_deref())
    }

    #[tool(description = "Record an archival reference. Two flavours: external source (pass `url` for papers / gists / dashboards) OR persona / entity (skip `url` for people, places, books without links). Both land in archival tier — long-term recall.")]
    fn cite(&self, Parameters(p): Parameters<CiteParams>) -> Result<CallToolResult, McpError> {
        tools::capture::cite(&self.store, &p.name, p.url.as_deref(), &p.body, p.initiative.as_deref())
    }

    // ----- Lookup --------------------------------------------------------
    #[tool(description = "Look up a node id by exact name. Returns the id or `(not found)`.")]
    fn recall(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        tools::lookup::recall(&self.store, &p.name, p.initiative.as_deref())
    }

    #[tool(description = "Drill into a node — name → brief + 1-hop drill-down children (sources via derived_from, parts via part_of).")]
    fn drill(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        tools::lookup::drill(&self.store, &p.name, p.initiative.as_deref())
    }

    #[tool(description = "Walk derived_from ancestors of a node back to its sources — the provenance chain.")]
    fn trace(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        tools::lookup::trace(&self.store, &p.name, p.initiative.as_deref())
    }

    #[tool(description = "Full-text search across name and body via Cozo FTS. No stemming — search the form you wrote. For inflection-tolerant matching across any language append `*`: `утечк*` finds `утечку`/`утечке`, `token*` finds `tokens`/`tokenize`. Search in the SAME language as the original capture, not in English. Results are ordered by score, then newest-first within equal scores.")]
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

    #[tool(description = "List nodes whose `tags` array contains the given tag — exact match. Common tag families: `kind:<type>` (observation, experiment, idea, reference, …), `sig:<level>` (low/medium/high), `role:<role>` (jot/review/synthesise/revised), `lang:<code>` (ru/en/mixed/other — auto-detected from body), `topic:<word>` (up to 5 content tokens auto-derived from body — same form as in body, no stemming), `status:<state>` (only for hypotheses). For loose matching use the `search` tool with `prefix*` instead. Newest-first when multiple match.")]
    fn tagged(&self, Parameters(p): Parameters<TaggedParams>) -> Result<CallToolResult, McpError> {
        tools::lookup::tagged(&self.store, &p.tag, p.initiative.as_deref())
    }

    #[tool(description = "Show every edge between two nodes (both directions) at NOW. Answers `why are A and B connected?`.")]
    fn between(&self, Parameters(p): Parameters<BetweenParams>) -> Result<CallToolResult, McpError> {
        tools::lookup::between(&self.store, &p.a, &p.b, p.initiative.as_deref())
    }

    // ----- Bi-temporal ---------------------------------------------------
    #[tool(description = "Time-travel: return what a node looked like at a past moment. `when` accepts unix seconds, RFC-3339, or duration ago (`5m`, `2h`).")]
    fn at(&self, Parameters(p): Parameters<AtParams>) -> Result<CallToolResult, McpError> {
        tools::temporal::at(&self.store, &p.name, &p.when, p.initiative.as_deref())
    }

    #[tool(description = "Print every assertion / retraction recorded for a node, chronologically. + means asserted, - means retracted.")]
    fn history(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        tools::temporal::history(&self.store, &p.name, p.initiative.as_deref())
    }

    // ----- Hypothesis cycle ---------------------------------------------
    #[tool(description = "Formulate a hypothesis. Auto-named. Optional `about` links via refers_to.")]
    fn claim(&self, Parameters(p): Parameters<ClaimParams>) -> Result<CallToolResult, McpError> {
        tools::hypothesis::claim(&self.store, &p.text, p.about.as_deref(), p.initiative.as_deref())
    }

    #[tool(description = "Run an experiment against an open hypothesis. Auto-named from the method body.")]
    fn test(&self, Parameters(p): Parameters<TestParams>) -> Result<CallToolResult, McpError> {
        tools::hypothesis::test_hypothesis(&self.store, &p.hypothesis, &p.method, p.initiative.as_deref())
    }

    #[tool(description = "Mark a hypothesis as supported, attaching `by` as the verifying evidence.")]
    fn confirm(&self, Parameters(p): Parameters<VerdictParams>) -> Result<CallToolResult, McpError> {
        tools::hypothesis::confirm(&self.store, &p.hypothesis, &p.by, p.initiative.as_deref())
    }

    #[tool(description = "Mark a hypothesis as refuted, attaching `by` as the falsifying counter-evidence.")]
    fn refute(&self, Parameters(p): Parameters<VerdictParams>) -> Result<CallToolResult, McpError> {
        tools::hypothesis::refute(&self.store, &p.hypothesis, &p.by, p.initiative.as_deref())
    }

    // ----- Review-flow ---------------------------------------------------
    #[tool(description = "Flag a node for review — creates a high-significance review episode + contradicts edge. Target unchanged.")]
    fn flag(&self, Parameters(p): Parameters<FlagParams>) -> Result<CallToolResult, McpError> {
        tools::review::flag(&self.store, &p.target, &p.reason, p.initiative.as_deref())
    }

    #[tool(description = "Resolve an open question by recording that `by` answers it (creates a supersedes edge).")]
    fn resolve(&self, Parameters(p): Parameters<ResolveParams>) -> Result<CallToolResult, McpError> {
        tools::review::resolve(&self.store, &p.question, &p.by, p.initiative.as_deref())
    }

    // ----- Consolidation -------------------------------------------------
    #[tool(description = "Promote operational draft → archival counterpart. Provenance via derived_from is replicated across the tier.")]
    fn settle(&self, Parameters(p): Parameters<ConsolidateParams>) -> Result<CallToolResult, McpError> {
        tools::consolidate::settle(&self.store, &p.source, &p.new_type, &p.new_name, &p.new_body, p.initiative.as_deref())
    }

    #[tool(description = "Bring an archival node back into the operational tier (mirror of `settle`).")]
    fn reopen(&self, Parameters(p): Parameters<ConsolidateParams>) -> Result<CallToolResult, McpError> {
        tools::consolidate::reopen(&self.store, &p.source, &p.new_type, &p.new_name, &p.new_body, p.initiative.as_deref())
    }

    #[tool(description = "Many-to-one consolidation — create a new node from several seeds, with derived_from edges to each.")]
    fn synthesise(&self, Parameters(p): Parameters<SynthesiseParams>) -> Result<CallToolResult, McpError> {
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

    #[tool(description = "Replace a node with a fresh one carrying new content, connected by a supersedes edge. Use when the change is large enough to warrant a new identity.")]
    fn supersede(&self, Parameters(p): Parameters<SupersedeParams>) -> Result<CallToolResult, McpError> {
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
    #[tool(description = "Capture a todo as a Task node. Auto-named from body. Tags: kind:task, status:open, optional due:YYYY-MM-DD. `due` accepts ISO date, RFC-3339, or future duration like `3d`/`2w`.")]
    fn task(&self, Parameters(p): Parameters<TaskParams>) -> Result<CallToolResult, McpError> {
        tools::task::task(&self.store, &p.body, p.due.as_deref(), p.initiative.as_deref())
    }

    #[tool(description = "Mark a task done — RMW retract+reassert with status:done, preserving id and name. Accepts task name or UUIDv7 id.")]
    fn done(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        tools::task::done(&self.store, &p.name, p.initiative.as_deref())
    }

    // ----- Metabolism ----------------------------------------------------
    #[tool(description = "Bi-temporal forget — retract a node and every edge connected to it. Historical reads still see it; reads at NOW skip.")]
    fn forget(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        tools::metabolism::forget(&self.store, &p.name, p.initiative.as_deref())
    }

    #[tool(description = "Rewrite a node's body and/or rename. Implemented as retract+reassert so history sees both versions.")]
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
    #[tool(description = "Diagnostic snapshot — orphan nodes (no edges) and unresolved reviews (inbound contradicts).")]
    fn lint(&self, Parameters(p): Parameters<ScopeOnly>) -> Result<CallToolResult, McpError> {
        tools::lint::lint(&self.store, p.initiative.as_deref())
    }

    #[tool(description = "Snapshot the substrate as an Obsidian-friendly markdown vault (README + INDEX + LOG + pages). Output dir is created if missing.")]
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
            "kaeru — cognitive memory for LLM agents. \
             Re-entry ritual: call `initiatives` to see projects, then `awake` (process state — what was open) \
             then `overview` (epistemic state — what the project knows), both with the chosen `initiative`. \
             \
             Always pass `initiative` on every call once known; without it, mutations stay un-tagged and \
             reads are cross-initiative. \
             \
             LANGUAGE: store and search in the user's NATIVE language. Don't translate Russian to English on \
             capture; don't translate Russian queries to English on lookup. Each node carries a `lang:*` tag \
             auto-detected from body script. \
             \
             SEARCH: `search` is FTS without stemming. Append `*` for inflection-tolerant matching across \
             any language: `утечк*`, `token*`, `verlier*`. \
             \
             TAGS: every node auto-tags `kind:*`, `sig:*`, `role:*` (when applicable), `lang:*`, and up to 5 \
             `topic:<word>` tokens from body. Slice by tag via `tagged \"topic:...\"` etc. \
             \
             FRESHNESS: search/recall results sort newest-first within equal scores; recent captures beat stale ones. \
             \
             Capture with `jot` (auto-named) or `episode` (deliberate name); link with `link`. \
             Inquire with `drill <name>`, `trace <name>`, `search <query>`, `tagged <tag>`. \
             Reason with `claim/test/confirm/refute`. Bi-temporal handle: `at`, `history`."
                .to_string(),
        )
    }
}
