//! `KaeruServer` — rmcp tool-router exposing the full curator API
//! as MCP tools. Each tool is a thin wrapper around a `kaeru-core`
//! primitive; output is a plain-text rendering, mirroring what
//! `kaeru-cli` prints.
//!
//! Stdio transport is sequential, so the `Store`'s internal initiative
//! mutex is enough — no outer lock needed. Tools that take an optional
//! `initiative` parameter set it for the duration of the call and
//! restore the previous value before returning.

use std::sync::Arc;

use rmcp::ErrorData as McpError;
use rmcp::ServerHandler;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::CallToolResult;
use rmcp::model::Content;
use rmcp::model::Implementation;
use rmcp::model::ProtocolVersion;
use rmcp::model::ServerCapabilities;
use rmcp::model::ServerInfo;
use rmcp::tool;
use rmcp::tool_handler;
use rmcp::tool_router;
use schemars::JsonSchema;
use serde::Deserialize;

use kaeru_core::EdgeType;
use kaeru_core::EpisodeKind;
use kaeru_core::HypothesisStatus;
use kaeru_core::NodeBrief;
use kaeru_core::NodeType;
use kaeru_core::Significance;
use kaeru_core::Store;
use kaeru_core::Tier;

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

// =========================================================================
// Param structs. Reuse where shapes are identical; distinct names where the
// agent-visible parameter name should differ from `name`.
// =========================================================================

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScopeOnly {
    /// Optional initiative to scope the operation to. When omitted,
    /// reads are cross-initiative; mutations end up un-tagged.
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NameScope {
    /// Node name (also accepts a UUIDv7 id where the verb supports
    /// polymorphic resolution).
    pub name: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct EpisodeParams {
    /// Short, recallable name.
    pub name: String,
    /// Free-form body.
    pub body: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct JotParams {
    /// Free-form body. Name is auto-derived from first words + id suffix.
    pub body: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LinkParams {
    /// Source node name.
    pub from: String,
    /// Destination node name.
    pub to: String,
    /// Edge type. Common values: `refers_to` (default), `causal`,
    /// `derived_from`, `contradicts`, `part_of`, `blocks`, `targets`,
    /// `supersedes`, `verifies`, `falsifies`, `temporal`,
    /// `consolidated_to`. Snake_case or kebab-case both accepted.
    #[serde(default = "default_edge_type")]
    pub edge_type: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

fn default_edge_type() -> String {
    "refers_to".to_string()
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PinParams {
    /// Node name or UUIDv7 id.
    pub name: String,
    /// Why the node deserves a place in the active window.
    pub reason: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RecentParams {
    /// Time window (e.g. `30m`, `3h`, `2d`, raw seconds). Defaults to 24h.
    #[serde(default = "default_recent_window")]
    pub since: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

fn default_recent_window() -> String {
    "24h".to_string()
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchParams {
    /// Cozo FTS query (`AND`/`OR`/`NOT`, `"phrase"`, `prefix*`).
    pub query: String,
    /// Maximum results. Capped at 50 internally.
    #[serde(default = "default_search_limit")]
    pub limit: usize,
    #[serde(default)]
    pub initiative: Option<String>,
}

fn default_search_limit() -> usize {
    10
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AtParams {
    /// Node name.
    pub name: String,
    /// Moment to query — Unix seconds, RFC-3339 (`2026-05-06T12:00:00Z`),
    /// or duration suffix (`5m`, `2h`, `3d` = "ago").
    pub when: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ClaimParams {
    /// The hypothesis text. Auto-named from first words + id suffix.
    pub text: String,
    /// Optional existing node this claim is about (refers_to edge).
    #[serde(default)]
    pub about: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TestParams {
    /// Hypothesis name.
    pub hypothesis: String,
    /// How the experiment was conducted.
    pub method: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct VerdictParams {
    /// Hypothesis name.
    pub hypothesis: String,
    /// Evidence node name (verifying for `confirm`, falsifying for `refute`).
    pub by: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FlagParams {
    /// Target node name to flag.
    pub target: String,
    /// Reason / description of the concern.
    pub reason: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ResolveParams {
    /// Question node name.
    pub question: String,
    /// Answer / resolution node name.
    pub by: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ConsolidateParams {
    /// Source node name.
    pub source: String,
    /// New node type (`idea`, `outcome`, `summary`, `draft`, …).
    pub new_type: String,
    /// New node name.
    pub new_name: String,
    /// New node body.
    pub new_body: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SynthesiseParams {
    /// Seed node names.
    pub from: Vec<String>,
    /// Type of the synthesised node (defaults `summary`).
    #[serde(default = "default_synth_type")]
    pub new_type: String,
    /// Name for the synthesised node.
    pub new_name: String,
    /// Body for the synthesised node.
    pub new_body: String,
    /// Tier override (`operational` / `archival`). Defaults from type.
    #[serde(default)]
    pub tier: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

fn default_synth_type() -> String {
    "summary".to_string()
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SupersedeParams {
    /// Old node name (or id).
    pub old: String,
    /// New node type.
    pub new_type: String,
    /// New node name.
    pub new_name: String,
    /// New node body.
    pub new_body: String,
    /// Tier override (defaults from new_type).
    #[serde(default)]
    pub tier: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReviseParams {
    /// Node name.
    pub name: String,
    /// New body. If omitted, keeps current.
    #[serde(default)]
    pub body: Option<String>,
    /// New name. If omitted, keeps current.
    #[serde(default)]
    pub rename: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CiteParams {
    /// Short, recallable name.
    pub name: String,
    /// URL of the source.
    pub url: String,
    /// One-paragraph summary.
    pub body: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BetweenParams {
    /// First node name.
    pub a: String,
    /// Second node name.
    pub b: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TaggedParams {
    /// Tag value (case-sensitive).
    pub tag: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExportParams {
    /// Output directory.
    pub output_dir: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

// =========================================================================
// Tool implementations. One #[tool] method per curator-API verb.
// =========================================================================

#[tool_router]
impl KaeruServer {
    // ----- Re-entry / session -------------------------------------------
    #[tool(description = "Restore session context: pinned set, recent episodes (24h), open reviews. Run this when re-entering a project.")]
    fn awake(&self, Parameters(p): Parameters<ScopeOnly>) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let ctx = kaeru_core::awake(&self.store).map_err(to_mcp)?;
            let mut out = String::new();
            out.push_str(&format!(
                "initiative: {}\n\n",
                ctx.initiative.as_deref().unwrap_or("(none)")
            ));
            out.push_str(&format!("pinned ({}):\n", ctx.pinned.len()));
            for id in &ctx.pinned {
                out.push_str(&format!("  - {id}{}\n", brief_suffix(&self.store, id)));
            }
            out.push('\n');
            out.push_str(&format!("recent ({}):\n", ctx.recent.len()));
            for id in &ctx.recent {
                out.push_str(&format!("  - {id}{}\n", brief_suffix(&self.store, id)));
            }
            out.push('\n');
            out.push_str(&format!("under review ({}):\n", ctx.under_review.len()));
            for id in &ctx.under_review {
                out.push_str(&format!("  - {id}{}\n", brief_suffix(&self.store, id)));
            }
            Ok(text(&out))
        })
    }

    #[tool(description = "Print a terminal-readable map of the substrate: counts by tier/type, provenance forests, open questions, edge stats.")]
    fn overview(&self, Parameters(p): Parameters<ScopeOnly>) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let report = kaeru_core::overview(&self.store).map_err(to_mcp)?;
            Ok(text(&report))
        })
    }

    #[tool(description = "List initiatives that have at least one node attached. Use this first when re-entering, then pick one for subsequent calls.")]
    fn initiatives(&self) -> Result<CallToolResult, McpError> {
        let names = kaeru_core::list_initiatives(&self.store).map_err(to_mcp)?;
        if names.is_empty() {
            return Ok(text(
                "(no initiatives yet — pass `initiative` on a mutation to register one)",
            ));
        }
        let mut out = format!("initiatives ({}):\n", names.len());
        for n in &names {
            out.push_str(&format!("  - {n}\n"));
        }
        Ok(text(&out))
    }

    #[tool(description = "List episodes whose latest assertion is within the time window (defaults 24h). Use `since` like `30m`, `3h`, `2d`, or raw seconds.")]
    fn recent(
        &self,
        Parameters(p): Parameters<RecentParams>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let window = parse_duration_secs(&p.since).map_err(to_mcp)?;
            let ids = kaeru_core::recent_episodes(&self.store, window).map_err(to_mcp)?;
            let mut out = format!("recent ({}):\n", ids.len());
            for id in &ids {
                out.push_str(&format!("  - {id}{}\n", brief_suffix(&self.store, id)));
            }
            Ok(text(&out))
        })
    }

    #[tool(description = "Pin a node to the active window. Accepts either a name or a UUIDv7 id.")]
    fn pin(&self, Parameters(p): Parameters<PinParams>) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let id = resolve_name_or_id(&self.store, &p.name)?;
            kaeru_core::pin(&self.store, &id, &p.reason).map_err(to_mcp)?;
            Ok(text(&format!("pinned: {} ({id})", p.name)))
        })
    }

    #[tool(description = "Unpin a node. Accepts name or id.")]
    fn unpin(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let id = resolve_name_or_id(&self.store, &p.name)?;
            kaeru_core::unpin(&self.store, &id).map_err(to_mcp)?;
            Ok(text(&format!("unpinned: {} ({id})", p.name)))
        })
    }

    #[tool(description = "Show resolved configuration: vault path and every cap (initiative not relevant).")]
    fn config(&self) -> Result<CallToolResult, McpError> {
        let c = self.store.config();
        let out = format!(
            "kaeru {}\nvault_path           = {}\nactive_window_size   = {}\nrecent_episodes_cap  = {}\nawake_window_secs    = {}\nsummary_children_cap = {}\nbody_excerpt_chars   = {}\nprovenance_max_hops  = {}\ndefault_max_hops     = {}\nmax_hops_cap         = {}\n",
            kaeru_core::version(),
            c.vault_path.display(),
            c.active_window_size,
            c.recent_episodes_cap,
            c.awake_default_window_secs,
            c.summary_view_children_cap,
            c.body_excerpt_chars,
            c.provenance_max_hops,
            c.default_max_hops,
            c.max_hops_cap,
        );
        Ok(text(&out))
    }

    // ----- Capture -------------------------------------------------------
    #[tool(description = "Write a deliberately-named operational episode. Use when you know you'll want to recall by exact name.")]
    fn episode(
        &self,
        Parameters(p): Parameters<EpisodeParams>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let id = kaeru_core::write_episode(
                &self.store,
                EpisodeKind::Observation,
                Significance::Medium,
                &p.name,
                &p.body,
            )
            .map_err(to_mcp)?;
            Ok(text(&format!("wrote episode: {id}")))
        })
    }

    #[tool(description = "Low-friction episode write — auto-named from body's first words plus a unique id suffix. Defaults to observation/low.")]
    fn jot(&self, Parameters(p): Parameters<JotParams>) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let id = kaeru_core::jot(&self.store, &p.body).map_err(to_mcp)?;
            let name = kaeru_core::node_brief_by_id(&self.store, &id)
                .ok()
                .flatten()
                .map(|b| b.name)
                .unwrap_or_default();
            Ok(text(&format!("jotted: {name} — {id}")))
        })
    }

    #[tool(description = "Create a typed edge between two named nodes. Edge type defaults to `refers_to`.")]
    fn link(&self, Parameters(p): Parameters<LinkParams>) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let edge: EdgeType = p.edge_type.parse().map_err(to_mcp)?;
            let from_id = resolve_name(&self.store, &p.from)?;
            let to_id = resolve_name(&self.store, &p.to)?;
            kaeru_core::link(&self.store, &from_id, &to_id, edge).map_err(to_mcp)?;
            Ok(text(&format!(
                "linked: {} -[{}]-> {}",
                p.from,
                edge.as_str(),
                p.to
            )))
        })
    }

    #[tool(description = "Retract a previously-asserted edge. Bi-temporal — historical reads still see it.")]
    fn unlink(&self, Parameters(p): Parameters<LinkParams>) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let edge: EdgeType = p.edge_type.parse().map_err(to_mcp)?;
            let from_id = resolve_name(&self.store, &p.from)?;
            let to_id = resolve_name(&self.store, &p.to)?;
            kaeru_core::unlink(&self.store, &from_id, &to_id, edge).map_err(to_mcp)?;
            Ok(text(&format!(
                "unlinked: {} -[{}]-> {}",
                p.from,
                edge.as_str(),
                p.to
            )))
        })
    }

    #[tool(description = "Record an external reference (paper / gist / dashboard) as an archival Reference node. URL goes into properties.url.")]
    fn cite(&self, Parameters(p): Parameters<CiteParams>) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let id = kaeru_core::cite(&self.store, &p.name, &p.url, &p.body).map_err(to_mcp)?;
            Ok(text(&format!("cited: {} ({}) — {id}", p.name, p.url)))
        })
    }

    // ----- Lookup --------------------------------------------------------
    #[tool(description = "Look up a node id by exact name. Returns the id or `(not found)`.")]
    fn recall(
        &self,
        Parameters(p): Parameters<NameScope>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            match kaeru_core::recall_id_by_name(&self.store, &p.name).map_err(to_mcp)? {
                Some(id) => Ok(text(&id)),
                None => Ok(text("(not found)")),
            }
        })
    }

    #[tool(description = "Drill into a node — name → brief + 1-hop drill-down children (sources via derived_from, parts via part_of).")]
    fn drill(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let id = resolve_name_or_id(&self.store, &p.name)?;
            let view = kaeru_core::summary_view(&self.store, &id).map_err(to_mcp)?;
            Ok(text(&render_summary(&view)))
        })
    }

    #[tool(description = "Walk derived_from ancestors of a node back to its sources — the provenance chain.")]
    fn trace(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let id = resolve_name(&self.store, &p.name)?;
            let ancestors = kaeru_core::recollect_provenance(&self.store, &id).map_err(to_mcp)?;
            if ancestors.is_empty() {
                return Ok(text("(no provenance)"));
            }
            let mut out = format!("provenance ({}):\n", ancestors.len());
            for b in &ancestors {
                out.push_str(&format!("  - {} ({}) — {}\n", b.name, b.node_type, b.id));
                if let Some(e) = &b.body_excerpt {
                    out.push_str(&format!("    {e}\n"));
                }
            }
            Ok(text(&out))
        })
    }

    #[tool(description = "Full-text search across name and body via Cozo FTS. No stemming — search the form you wrote. For inflection-tolerant matching across any language append `*`: `утечк*` finds `утечку`/`утечке`, `token*` finds `tokens`/`tokenize`. Search in the SAME language as the original capture, not in English. Results are ordered by score, then newest-first within equal scores.")]
    fn search(
        &self,
        Parameters(p): Parameters<SearchParams>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let hits = kaeru_core::fuzzy_recall(&self.store, &p.query, p.limit).map_err(to_mcp)?;
            if hits.is_empty() {
                return Ok(text("(no matches)"));
            }
            let mut out = format!("matches ({}):\n", hits.len());
            for b in &hits {
                out.push_str(&format!("  - {} ({}) — {}\n", b.name, b.node_type, b.id));
                if let Some(e) = &b.body_excerpt {
                    out.push_str(&format!("    {e}\n"));
                }
            }
            Ok(text(&out))
        })
    }

    #[tool(description = "List archival ideas — long-term cortex memory of stable ideas.")]
    fn ideas(
        &self,
        Parameters(p): Parameters<ScopeOnly>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let briefs = kaeru_core::recollect_idea(&self.store).map_err(to_mcp)?;
            Ok(text(&render_briefs("ideas", &briefs)))
        })
    }

    #[tool(description = "List archival outcomes — settled results.")]
    fn outcomes(
        &self,
        Parameters(p): Parameters<ScopeOnly>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let briefs = kaeru_core::recollect_outcome(&self.store).map_err(to_mcp)?;
            Ok(text(&render_briefs("outcomes", &briefs)))
        })
    }

    #[tool(description = "List nodes whose `tags` array contains the given tag — exact match. Common tag families: `kind:<type>` (observation, experiment, idea, reference, …), `sig:<level>` (low/medium/high), `role:<role>` (jot/review/synthesise/revised), `lang:<code>` (ru/en/mixed/other — auto-detected from body), `topic:<word>` (up to 5 content tokens auto-derived from body — same form as in body, no stemming), `status:<state>` (only for hypotheses). For loose matching use the `search` tool with `prefix*` instead. Newest-first when multiple match.")]
    fn tagged(
        &self,
        Parameters(p): Parameters<TaggedParams>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let briefs = kaeru_core::tagged(&self.store, &p.tag).map_err(to_mcp)?;
            Ok(text(&render_briefs(
                &format!("tagged `{}`", p.tag),
                &briefs,
            )))
        })
    }

    #[tool(description = "Show every edge between two nodes (both directions) at NOW. Answers `why are A and B connected?`.")]
    fn between(
        &self,
        Parameters(p): Parameters<BetweenParams>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let a_id = resolve_name(&self.store, &p.a)?;
            let b_id = resolve_name(&self.store, &p.b)?;
            let edges = kaeru_core::between(&self.store, &a_id, &b_id).map_err(to_mcp)?;
            if edges.is_empty() {
                return Ok(text(&format!("(no edges between {} and {})", p.a, p.b)));
            }
            let mut out = format!("edges ({}):\n", edges.len());
            for e in &edges {
                if e.a_to_b {
                    out.push_str(&format!("  {} —[{}]→ {}\n", p.a, e.edge_type, p.b));
                } else {
                    out.push_str(&format!("  {} ←[{}]— {}\n", p.a, e.edge_type, p.b));
                }
            }
            Ok(text(&out))
        })
    }

    // ----- Bi-temporal ---------------------------------------------------
    #[tool(description = "Time-travel: return what a node looked like at a past moment. `when` accepts unix seconds, RFC-3339, or duration ago (`5m`, `2h`).")]
    fn at(&self, Parameters(p): Parameters<AtParams>) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let id = resolve_name(&self.store, &p.name)?;
            let secs = parse_when(&p.when).map_err(to_mcp)?;
            match kaeru_core::at(&self.store, &id, secs).map_err(to_mcp)? {
                Some(snap) => {
                    let body = snap.body.unwrap_or_else(|| "(no body)".to_string());
                    Ok(text(&format!("{}\n\n{}", snap.name, body)))
                }
                None => Ok(text("(no row valid at that moment)")),
            }
        })
    }

    #[tool(description = "Print every assertion / retraction recorded for a node, chronologically. + means asserted, - means retracted.")]
    fn history(
        &self,
        Parameters(p): Parameters<NameScope>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let id = resolve_name(&self.store, &p.name)?;
            let revs = kaeru_core::history(&self.store, &id).map_err(to_mcp)?;
            if revs.is_empty() {
                return Ok(text("(no history)"));
            }
            let mut out = format!("history ({}):\n", revs.len());
            for r in &revs {
                let mark = if r.asserted { "+" } else { "-" };
                out.push_str(&format!("  [{mark}] t={:.0}  {}\n", r.seconds, r.name));
            }
            Ok(text(&out))
        })
    }

    // ----- Hypothesis cycle ---------------------------------------------
    #[tool(description = "Formulate a hypothesis. Auto-named. Optional `about` links via refers_to.")]
    fn claim(
        &self,
        Parameters(p): Parameters<ClaimParams>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let auto_name = derive_auto_name(&p.text, "claim");
            let id = kaeru_core::formulate_hypothesis(&self.store, &auto_name, &p.text)
                .map_err(to_mcp)?;
            if let Some(about) = &p.about {
                let target = resolve_name(&self.store, about)?;
                kaeru_core::link(&self.store, &id, &target, EdgeType::RefersTo).map_err(to_mcp)?;
            }
            Ok(text(&format!("claimed: {auto_name} — {id}")))
        })
    }

    #[tool(description = "Run an experiment against an open hypothesis. Auto-named from the method body.")]
    fn test(&self, Parameters(p): Parameters<TestParams>) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let hyp_id = resolve_name(&self.store, &p.hypothesis)?;
            let auto_name = derive_auto_name(&p.method, "experiment");
            let exp_id = kaeru_core::run_experiment(&self.store, &hyp_id, &auto_name, &p.method)
                .map_err(to_mcp)?;
            Ok(text(&format!("experiment: {auto_name} — {exp_id}")))
        })
    }

    #[tool(description = "Mark a hypothesis as supported, attaching `by` as the verifying evidence.")]
    fn confirm(
        &self,
        Parameters(p): Parameters<VerdictParams>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let hyp_id = resolve_name(&self.store, &p.hypothesis)?;
            let by_id = resolve_name(&self.store, &p.by)?;
            kaeru_core::update_hypothesis_status(
                &self.store,
                &hyp_id,
                HypothesisStatus::Supported,
                &by_id,
            )
            .map_err(to_mcp)?;
            Ok(text(&format!("confirmed: {}", p.hypothesis)))
        })
    }

    #[tool(description = "Mark a hypothesis as refuted, attaching `by` as the falsifying counter-evidence.")]
    fn refute(
        &self,
        Parameters(p): Parameters<VerdictParams>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let hyp_id = resolve_name(&self.store, &p.hypothesis)?;
            let by_id = resolve_name(&self.store, &p.by)?;
            kaeru_core::update_hypothesis_status(
                &self.store,
                &hyp_id,
                HypothesisStatus::Refuted,
                &by_id,
            )
            .map_err(to_mcp)?;
            Ok(text(&format!("refuted: {}", p.hypothesis)))
        })
    }

    // ----- Review-flow ---------------------------------------------------
    #[tool(description = "Flag a node for review — creates a high-significance review episode + contradicts edge. Target unchanged.")]
    fn flag(&self, Parameters(p): Parameters<FlagParams>) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let target_id = resolve_name(&self.store, &p.target)?;
            let review_id = kaeru_core::mark_under_review(&self.store, &target_id, &p.reason)
                .map_err(to_mcp)?;
            Ok(text(&format!(
                "flagged: {} (review id: {review_id})",
                p.target
            )))
        })
    }

    #[tool(description = "Resolve an open question by recording that `by` answers it (creates a supersedes edge).")]
    fn resolve(
        &self,
        Parameters(p): Parameters<ResolveParams>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let question_id = resolve_name(&self.store, &p.question)?;
            let by_id = resolve_name(&self.store, &p.by)?;
            kaeru_core::mark_resolved(&self.store, &question_id, &by_id).map_err(to_mcp)?;
            Ok(text(&format!("resolved: {} ← {}", p.question, p.by)))
        })
    }

    // ----- Consolidation -------------------------------------------------
    #[tool(description = "Promote operational draft → archival counterpart. Provenance via derived_from is replicated across the tier.")]
    fn settle(
        &self,
        Parameters(p): Parameters<ConsolidateParams>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let draft_id = resolve_name(&self.store, &p.source)?;
            let new_type: NodeType = p.new_type.parse().map_err(to_mcp)?;
            let id = kaeru_core::consolidate_out(
                &self.store,
                &draft_id,
                new_type,
                &p.new_name,
                &p.new_body,
            )
            .map_err(to_mcp)?;
            Ok(text(&format!(
                "settled: {} → {} ({}) — {id}",
                p.source,
                p.new_name,
                new_type.as_str()
            )))
        })
    }

    #[tool(description = "Bring an archival node back into the operational tier (mirror of `settle`).")]
    fn reopen(
        &self,
        Parameters(p): Parameters<ConsolidateParams>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let archival_id = resolve_name(&self.store, &p.source)?;
            let new_type: NodeType = p.new_type.parse().map_err(to_mcp)?;
            let id = kaeru_core::consolidate_in(
                &self.store,
                &archival_id,
                new_type,
                &p.new_name,
                &p.new_body,
            )
            .map_err(to_mcp)?;
            Ok(text(&format!(
                "reopened: {} → {} ({}) — {id}",
                p.source,
                p.new_name,
                new_type.as_str()
            )))
        })
    }

    #[tool(description = "Many-to-one consolidation — create a new node from several seeds, with derived_from edges to each.")]
    fn synthesise(
        &self,
        Parameters(p): Parameters<SynthesiseParams>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            if p.from.is_empty() {
                return Err(to_mcp(kaeru_core::Error::Invalid(
                    "from must list at least one seed".to_string(),
                )));
            }
            let new_type: NodeType = p.new_type.parse().map_err(to_mcp)?;
            let target_tier = match p.tier {
                Some(t) => parse_tier(&t).map_err(to_mcp)?,
                None => new_type.default_tier(),
            };
            let mut seed_ids = Vec::with_capacity(p.from.len());
            for n in &p.from {
                seed_ids.push(resolve_name(&self.store, n)?);
            }
            let id = kaeru_core::synthesise(
                &self.store,
                &seed_ids,
                new_type,
                target_tier,
                &p.new_name,
                &p.new_body,
            )
            .map_err(to_mcp)?;
            Ok(text(&format!(
                "synthesised: {} ({} / {}) — {id}",
                p.new_name,
                new_type.as_str(),
                target_tier.as_str()
            )))
        })
    }

    #[tool(description = "Replace a node with a fresh one carrying new content, connected by a supersedes edge. Use when the change is large enough to warrant a new identity.")]
    fn supersede(
        &self,
        Parameters(p): Parameters<SupersedeParams>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let old_id = resolve_name_or_id(&self.store, &p.old)?;
            let new_type: NodeType = p.new_type.parse().map_err(to_mcp)?;
            let target_tier = match p.tier {
                Some(t) => parse_tier(&t).map_err(to_mcp)?,
                None => new_type.default_tier(),
            };
            let id = kaeru_core::supersedes(
                &self.store,
                &old_id,
                new_type,
                target_tier,
                &p.new_name,
                &p.new_body,
            )
            .map_err(to_mcp)?;
            Ok(text(&format!(
                "superseded: {} → {} ({} / {}) — {id}",
                p.old,
                p.new_name,
                new_type.as_str(),
                target_tier.as_str()
            )))
        })
    }

    // ----- Metabolism ----------------------------------------------------
    #[tool(description = "Bi-temporal forget — retract a node and every edge connected to it. Historical reads still see it; reads at NOW skip.")]
    fn forget(&self, Parameters(p): Parameters<NameScope>) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let id = resolve_name_or_id(&self.store, &p.name)?;
            kaeru_core::forget(&self.store, &id).map_err(to_mcp)?;
            Ok(text(&format!("forgot: {}", p.name)))
        })
    }

    #[tool(description = "Rewrite a node's body and/or rename. Implemented as retract+reassert so history sees both versions.")]
    fn revise(
        &self,
        Parameters(p): Parameters<ReviseParams>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let id = resolve_name_or_id(&self.store, &p.name)?;
            let brief = kaeru_core::node_brief_by_id(&self.store, &id)
                .map_err(to_mcp)?
                .ok_or_else(|| {
                    to_mcp(kaeru_core::Error::NotFound(format!(
                        "node {:?} not found at NOW",
                        p.name
                    )))
                })?;
            let new_name = p.rename.as_deref().unwrap_or(&brief.name);
            let preserved_body = if p.body.is_none() {
                kaeru_core::summary_view(&self.store, &id)
                    .map_err(to_mcp)?
                    .root
                    .body_excerpt
                    .unwrap_or_default()
            } else {
                String::new()
            };
            let new_body = p.body.as_deref().unwrap_or(&preserved_body);
            kaeru_core::improve(&self.store, &id, new_name, new_body).map_err(to_mcp)?;
            Ok(text(&format!("revised: {} → {new_name}", p.name)))
        })
    }

    #[tool(description = "Diagnostic snapshot — orphan nodes (no edges) and unresolved reviews (inbound contradicts).")]
    fn lint(&self, Parameters(p): Parameters<ScopeOnly>) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let report = kaeru_core::lint(&self.store).map_err(to_mcp)?;
            let mut out = format!("orphans ({}):\n", report.orphans.len());
            for id in &report.orphans {
                out.push_str(&format!("  - {id}{}\n", brief_suffix(&self.store, id)));
            }
            out.push('\n');
            out.push_str(&format!(
                "unresolved reviews ({}):\n",
                report.unresolved_reviews.len()
            ));
            for id in &report.unresolved_reviews {
                out.push_str(&format!("  - {id}{}\n", brief_suffix(&self.store, id)));
            }
            Ok(text(&out))
        })
    }

    #[tool(description = "Snapshot the substrate as an Obsidian-friendly markdown vault (README + INDEX + LOG + pages). Output dir is created if missing.")]
    fn export(
        &self,
        Parameters(p): Parameters<ExportParams>,
    ) -> Result<CallToolResult, McpError> {
        with_initiative(&self.store, p.initiative.as_deref(), || {
            let summary =
                kaeru_core::export_vault(&self.store, &p.output_dir).map_err(to_mcp)?;
            Ok(text(&format!(
                "exported {} node(s), {} edge(s) → {}",
                summary.nodes_exported,
                summary.edges_exported,
                summary.root.display()
            )))
        })
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

// =========================================================================
// Helpers
// =========================================================================

fn text(s: &str) -> CallToolResult {
    CallToolResult::success(vec![Content::text(s)])
}

fn to_mcp(e: kaeru_core::Error) -> McpError {
    McpError::internal_error(e.to_string(), None)
}

fn brief_suffix(store: &Store, id: &str) -> String {
    match kaeru_core::node_brief_by_id(store, &id.to_string()) {
        Ok(Some(b)) => format!(" — {}", b.name),
        _ => String::new(),
    }
}

fn render_summary(view: &kaeru_core::SummaryView) -> String {
    let mut out = format!(
        "{} ({}) — {}\n",
        view.root.name, view.root.node_type, view.root.id
    );
    if let Some(e) = &view.root.body_excerpt {
        out.push_str(&format!("  {e}\n"));
    }
    if view.children.is_empty() {
        out.push_str("(no drill-down children)\n");
    } else {
        out.push_str(&format!("children ({}):\n", view.children.len()));
        for c in &view.children {
            out.push_str(&format!("  - {} ({}) — {}\n", c.name, c.node_type, c.id));
            if let Some(e) = &c.body_excerpt {
                out.push_str(&format!("    {e}\n"));
            }
        }
    }
    out
}

fn render_briefs(label: &str, briefs: &[NodeBrief]) -> String {
    if briefs.is_empty() {
        return format!("{label} (0): (empty)");
    }
    let mut out = format!("{label} ({}):\n", briefs.len());
    for b in briefs {
        out.push_str(&format!("  - {} ({}) — {}\n", b.name, b.node_type, b.id));
        if let Some(e) = &b.body_excerpt {
            out.push_str(&format!("    {e}\n"));
        }
    }
    out
}

/// Sets the store's current initiative (or clears it), runs `f`, then
/// restores the previous initiative. Stdio-MCP processes one tool call
/// at a time so the in-place mutation is safe.
fn with_initiative<T>(
    store: &Store,
    initiative: Option<&str>,
    f: impl FnOnce() -> Result<T, McpError>,
) -> Result<T, McpError> {
    let prev = store.current_initiative();
    match initiative {
        Some(name) => store.use_initiative(name),
        None => store.clear_initiative(),
    }
    let result = f();
    match prev {
        Some(p) => store.use_initiative(&p),
        None => store.clear_initiative(),
    }
    result
}

fn resolve_name(store: &Store, name: &str) -> Result<kaeru_core::NodeId, McpError> {
    kaeru_core::recall_id_by_name(store, name)
        .map_err(to_mcp)?
        .ok_or_else(|| {
            to_mcp(kaeru_core::Error::NotFound(format!(
                "no node named {name:?} at NOW"
            )))
        })
}

fn resolve_name_or_id(store: &Store, input: &str) -> Result<kaeru_core::NodeId, McpError> {
    // UUIDv7 has 36 chars with dashes at fixed positions; cheap heuristic.
    if input.len() == 36 && input.chars().nth(8) == Some('-') {
        return Ok(input.to_string());
    }
    resolve_name(store, input)
}

fn parse_duration_secs(s: &str) -> Result<u64, kaeru_core::Error> {
    let trimmed = s.trim();
    if trimmed.chars().all(|c| c.is_ascii_digit()) {
        return trimmed
            .parse::<u64>()
            .map_err(|e| kaeru_core::Error::Invalid(format!("bad seconds: {e}")));
    }
    if trimmed.is_empty() {
        return Err(kaeru_core::Error::Invalid("empty duration".to_string()));
    }
    let (num, unit) = trimmed.split_at(trimmed.len() - 1);
    let n: u64 = num
        .parse()
        .map_err(|e| kaeru_core::Error::Invalid(format!("bad duration: {e}")))?;
    let mult: u64 = match unit {
        "s" => 1,
        "m" => 60,
        "h" => 3600,
        "d" => 86_400,
        "w" => 7 * 86_400,
        other => {
            return Err(kaeru_core::Error::Invalid(format!(
                "unknown unit {other:?} (use s/m/h/d/w)"
            )));
        }
    };
    Ok(n.saturating_mul(mult))
}

fn parse_when(s: &str) -> Result<f64, kaeru_core::Error> {
    let trimmed = s.trim();
    if trimmed.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return trimmed
            .parse::<f64>()
            .map_err(|e| kaeru_core::Error::Invalid(format!("bad seconds: {e}")));
    }
    if let Some(last) = trimmed.chars().last() {
        if matches!(last, 's' | 'm' | 'h' | 'd' | 'w')
            && trimmed[..trimmed.len() - 1]
                .chars()
                .all(|c| c.is_ascii_digit())
        {
            let secs = parse_duration_secs(trimmed)?;
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            return Ok(now.saturating_sub(secs) as f64);
        }
    }
    // Fall back to RFC-3339 — but kaeru-core doesn't bring chrono into the
    // MCP crate path; use a minimal manual parse for `YYYY-MM-DDTHH:MM:SSZ`
    // common case. Anything else: error.
    Err(kaeru_core::Error::Invalid(format!(
        "bad timestamp {s:?}: expected unix seconds, duration suffix (5m/2h/3d), or RFC-3339 datetime"
    )))
}

fn parse_tier(s: &str) -> Result<Tier, kaeru_core::Error> {
    match s.to_lowercase().as_str() {
        "operational" | "op" => Ok(Tier::Operational),
        "archival" | "ar" => Ok(Tier::Archival),
        _ => Err(kaeru_core::Error::Invalid(format!(
            "unknown tier {s:?}"
        ))),
    }
}

fn derive_auto_name(text: &str, fallback: &str) -> String {
    const MAX_WORDS: usize = 5;
    let mut words: Vec<String> = Vec::new();
    for raw in text.split_whitespace() {
        let cleaned: String = raw
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect::<String>()
            .to_lowercase();
        if !cleaned.is_empty() {
            words.push(cleaned);
            if words.len() >= MAX_WORDS {
                break;
            }
        }
    }
    let id = kaeru_core::new_node_id();
    let suffix: String = id.chars().rev().take(6).collect::<String>().chars().rev().collect();
    if words.is_empty() {
        format!("{fallback}-{suffix}")
    } else {
        format!("{}-{suffix}", words.join("-"))
    }
}
