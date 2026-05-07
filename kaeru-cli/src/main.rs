//! `kaeru` — command-line surface over the cognitive memory graph.
//!
//! Subcommands are thin wrappers over `kaeru-core` primitives. Output is
//! plain text, human-readable; structured (JSON) output is a future
//! addition. The vault path is read from `KAERU_VAULT_PATH` (auto-routed
//! through `KaeruConfig::from_env`); platform defaults apply if unset.
//!
//! This file declares the `Cli` / `Command` enum and dispatches each
//! variant to a handler in `commands::<group>`. Format helpers live in
//! `format.rs` and parsing helpers in `parse.rs`.

mod commands;
mod format;
mod parse;

use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

use kaeru_core::KaeruConfig;
use kaeru_core::Result;
use kaeru_core::Store;

/// `kaeru` is a cognitive memory layer for LLM agents — a typed graph
/// stored in an embedded RocksDB-backed CozoDB substrate.
///
/// Two tiers, biological analogy: **operational** (cognitive /
/// hippocampus) is the high-velocity working graph where the agent
/// thinks — episodes, drafts, hypotheses, experiments. **Archival**
/// (recollection / cortex) is the settled long-term store — ideas,
/// outcomes, summaries. Every node and edge is bi-temporal: assertions
/// and retractions accumulate, time-travel is native.
///
/// You interact with the substrate through a curator API: a small set
/// of primitives (`episode`, `recall`, `pin`, `summary`, `lint`, ...)
/// that the agent or a human composes. `kaeru` is a facilitator, not an
/// enforcer — commands hint when context is missing but never block.
///
/// The vault lives at `$KAERU_VAULT_PATH` if set, otherwise a
/// platform-specific default (`~/.local/share/kaeru` on Linux, etc).
/// All `KAERU_*` env vars override the defaults in `kaeru config`.
#[derive(Parser, Debug)]
#[command(
    name = "kaeru",
    version,
    about = "Cognitive memory layer for LLM agents",
    long_about,
    after_help = "TYPICAL WORKFLOW\n\
        \n  \
        # First time — defaults to platform path; verify with:\n  \
        kaeru config\n\
        \n  \
        # See what projects exist, pick one:\n  \
        kaeru initiatives\n\
        \n  \
        # Re-entry ritual: what was open, then what the project knows:\n  \
        kaeru --initiative <name> awake\n  \
        kaeru --initiative <name> overview\n\
        \n  \
        # Capture a quick thought (auto-named):\n  \
        kaeru --initiative <name> jot 'noticed token expiry differs'\n\
        \n  \
        # Drill into something by name:\n  \
        kaeru --initiative <name> drill <node-name>\n\
        \n  \
        # Snapshot to a markdown vault:\n  \
        kaeru --initiative <name> export /tmp/snap\n\
        \n\
        ENVIRONMENT\n\
        \n  \
        KAERU_VAULT_PATH         override the vault location\n  \
        KAERU_ACTIVE_WINDOW_SIZE soft cap on `awake` pinned set (default 15)\n  \
        KAERU_RECENT_*, KAERU_*  see `kaeru config` for the full list"
)]
struct Cli {
    /// Restrict the operation to a specific initiative. Mutations are
    /// auto-attached to this initiative; reads are filtered to it.
    /// Get the list of known names with `kaeru initiatives`.
    ///
    /// Without `--initiative`, mutations are cross-initiative
    /// (un-tagged) and reads cover the whole substrate.
    #[arg(long, global = true)]
    initiative: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Print the resolved configuration (vault path, caps).
    ///
    /// Shows the active vault directory and every tunable cap. Use this
    /// to confirm that `KAERU_VAULT_PATH` is actually being picked up,
    /// or to discover what env vars exist for tweaking behaviour.
    Config,

    /// Restore session context — pinned set, recent episodes, open reviews.
    ///
    /// Single call an agent makes when re-entering a project. Returns
    /// the active initiative, the persisted pin set (newest-first),
    /// episodes written in the last ~24 hours, and any nodes flagged via
    /// a `contradicts` edge that are still open. Read-only — does not
    /// write an audit event itself.
    Awake,

    /// Print a terminal-readable map of the substrate — counts by
    /// tier/type, provenance forests rooted at archival nodes, the
    /// open-review queue, and edge statistics. Honours `--initiative`.
    ///
    /// Use this on re-entry alongside `awake`: `awake` says "what was I
    /// doing", `overview` says "what does this project know".
    Overview,

    /// Write a new operational episode.
    ///
    /// Episodes are the primary working-graph unit: observations,
    /// decisions, scratch thoughts. Defaults: kind = observation,
    /// significance = medium. The returned id is what every other
    /// command takes; recall it later by name with `kaeru recall <name>`.
    Episode {
        /// Short, unique-ish name. Used by `kaeru recall <name>` for
        /// later lookup, so pick something memorable and distinct.
        name: String,
        /// Free-form body. Quote it if it contains spaces.
        body: String,
    },

    /// Low-friction episode write — no name, no type choice. The name
    /// is derived from the body's first words plus a short id suffix
    /// (always unique). Defaults to observation / low-significance.
    ///
    /// Use this when you're thinking out loud and don't want to stop
    /// to choose a name. For load-bearing thoughts use `kaeru episode`
    /// with a deliberate name.
    Jot {
        /// Free-form body. Quote it if it contains spaces.
        body: String,
    },

    /// Create a typed edge between two named nodes.
    ///
    /// Both endpoints are resolved through `kaeru recall <name>`, so
    /// you don't have to remember UUIDs. Edge type defaults to
    /// `refers-to` (the most generic association); pass `--type` for
    /// anything load-bearing (`causal`, `derived-from`, `contradicts`,
    /// …). Both kebab-case (`refers-to`) and snake_case (`refers_to`)
    /// forms are accepted.
    Link {
        /// Source node name.
        from: String,
        /// Destination node name.
        to: String,
        /// Edge type. Defaults to `refers-to`.
        #[arg(long, default_value = "refers-to")]
        r#type: String,
    },

    /// Look up a node id by its name. Prints the id, or `(not found)`.
    ///
    /// The cheap path from a human-readable handle to the UUIDv7 the
    /// rest of the API expects. Names are not unique — if multiple
    /// nodes share a name, the first match at NOW is returned.
    Recall {
        /// Name to look up (case-sensitive, exact match).
        name: String,
    },

    /// Full-text search across `name` and `body` via Cozo FTS.
    ///
    /// Use this when you don't remember an exact name. Tokens match
    /// case-insensitively after splitting on non-alphanumeric; there's
    /// no stemming, so search for the form you wrote (`token` finds
    /// `token` not `tokens`). Cozo FTS supports `AND` / `OR` / `NOT`
    /// operators and quoted phrases — see Cozo docs for the full
    /// grammar. `--initiative` scopes the search.
    Search {
        /// Query string. Quote it if it contains spaces or operators.
        query: String,
        /// Maximum results to return. Capped internally.
        #[arg(long, default_value_t = 10)]
        limit: usize,
    },

    /// Add a node to the active window with a justification.
    ///
    /// Pins are persisted in the substrate so a process restart
    /// restores the active window. Re-pinning the same id updates the
    /// reason and timestamp. Capped at `active_window_size`.
    Pin {
        /// Node id (UUIDv7) — typically obtained from `kaeru recall <name>`
        /// or copied from `kaeru episode` output.
        id: String,
        /// Why it deserves a place in the active window. Surfaces in
        /// `awake` next time and helps remember why this is open.
        reason: String,
    },

    /// Remove a node from the active window.
    ///
    /// No-op if the id wasn't pinned. The node itself is untouched —
    /// this is a session-level operation, not a graph mutation.
    Unpin {
        /// Node id to unpin.
        id: String,
    },

    /// One-level hierarchical view of `id` — name, body excerpt,
    /// drill-down children.
    ///
    /// The PageIndex-style navigation surface: an agent reads the seed
    /// brief, scans children, decides which child to expand by calling
    /// `summary <child-id>`. Drill-down follows outgoing `derived_from`
    /// (sources) and incoming `part_of` (parts).
    Summary {
        /// Seed node id.
        id: String,
    },

    /// Diagnostic snapshot — orphan nodes and the open-review queue.
    ///
    /// Read-only health check. **Orphans** are nodes with no edges at
    /// NOW — usually fresh or genuinely garbage. **Unresolved reviews**
    /// are nodes flagged via `mark_under_review` that have an inbound
    /// `contradicts` edge nobody has resolved yet.
    Lint,

    /// List episodes whose latest assertion is within `--since`. Default
    /// window: 24h. Honours `--initiative`.
    Recent {
        /// Time window — `30m`, `3h`, `2d`, or raw seconds. Defaults to 24h.
        #[arg(long, default_value = "24h")]
        since: String,
    },

    /// Drill into a node — name → brief + 1-hop drill-down children
    /// (sources via `derived_from`, parts via `part_of`). Composite of
    /// `recall` + `summary` so the agent reaches a node by name in
    /// one round-trip.
    Drill {
        /// Node name.
        name: String,
    },

    /// Walk `derived_from` ancestors of a node back to its sources.
    /// Returns the provenance chain — what this node was built from.
    Trace {
        /// Node name.
        name: String,
    },

    /// List archival ideas valid at NOW (cortex memory of long-term
    /// ideas). Honours `--initiative`.
    Ideas,

    /// List archival outcomes valid at NOW — settled results the agent
    /// has decided are "this is what we found". Honours `--initiative`.
    Outcomes,

    /// Formulate a hypothesis. Auto-named from the claim text plus a
    /// short id suffix. Optionally link `--about <name>` to record what
    /// the claim is reasoning over.
    Claim {
        /// The claim itself, free-form.
        text: String,
        /// Existing node this claim is about. Creates a `refers_to`
        /// edge if present.
        #[arg(long)]
        about: Option<String>,
    },

    /// Run an experiment against an open hypothesis. Auto-names from
    /// the method body.
    Test {
        /// Hypothesis name.
        hypothesis: String,
        /// Description of how the experiment was conducted.
        #[arg(long)]
        method: String,
    },

    /// Mark a hypothesis as supported, attaching `--by <evidence>` as
    /// the verifying node.
    Confirm {
        /// Hypothesis name.
        hypothesis: String,
        /// Evidence node name.
        #[arg(long)]
        by: String,
    },

    /// Mark a hypothesis as refuted, attaching `--by <evidence>` as
    /// the falsifying node.
    Refute {
        /// Hypothesis name.
        hypothesis: String,
        /// Counter-evidence node name.
        #[arg(long)]
        by: String,
    },

    /// Flag a node for review — creates a high-significance review
    /// episode and connects it to the target via `contradicts`. The
    /// target is untouched (non-destructive).
    Flag {
        /// Target node name.
        target: String,
        /// Reason / description of the concern.
        #[arg(long)]
        reason: String,
    },

    /// Resolve an open question by recording that `--by <answer>`
    /// supersedes it.
    Resolve {
        /// Question name.
        question: String,
        /// Answer / resolution node name.
        #[arg(long)]
        by: String,
    },

    /// Promote an operational draft to an archival counterpart.
    /// Provenance via `derived_from` is replicated so the chain
    /// survives the tier boundary.
    Settle {
        /// Operational draft name.
        draft: String,
        /// Archival type (`idea`, `outcome`, `summary`, …).
        #[arg(long, value_name = "TYPE")]
        r#as: String,
        /// New name for the archival node.
        #[arg(long)]
        name: String,
        /// Body for the archival node.
        #[arg(long)]
        body: String,
    },

    /// Bring an archival node back into the operational tier (e.g.
    /// because it needs revision while the agent is actively working
    /// on it). Mirror of `settle`.
    Reopen {
        /// Archival node name.
        archival: String,
        /// New operational type (`draft`, `episode`, …).
        #[arg(long, value_name = "TYPE")]
        r#as: String,
        /// New name for the operational node.
        #[arg(long)]
        name: String,
        /// Body for the operational node.
        #[arg(long)]
        body: String,
    },

    /// Many-to-one consolidation — create a new node from several
    /// seeds, with a `derived_from` edge to each so provenance walks
    /// back to the source material.
    Synthesise {
        /// Comma-separated seed names.
        #[arg(long, value_delimiter = ',')]
        from: Vec<String>,
        /// Type of the synthesised node (defaults `summary`).
        #[arg(long, value_name = "TYPE", default_value = "summary")]
        r#as: String,
        /// Name for the synthesised node.
        #[arg(long)]
        name: String,
        /// Body for the synthesised node.
        #[arg(long)]
        body: String,
        /// Tier override. Defaults: archival types
        /// (`idea`/`outcome`/`reference`) go to archival, others
        /// stay operational.
        #[arg(long, value_name = "TIER")]
        tier: Option<String>,
    },

    /// Show every edge between two nodes (in either direction) at NOW.
    /// Answers "why are A and B connected?" — neither `drill` nor
    /// `trace` enumerate edges between a specific pair.
    Between {
        /// First node name.
        a: String,
        /// Second node name.
        b: String,
    },

    /// List nodes whose `tags` array contains `<tag>` at NOW. Slice the
    /// graph by `kind:observation`, `sig:high`, `role:review`, custom
    /// tags, etc.
    Tagged {
        /// Tag value (case-sensitive, exact match).
        tag: String,
    },

    /// Record an archival reference. Two flavours:
    ///
    /// - **External source** (paper, gist, dashboard): pass `--url` —
    ///   it lands in `properties.url`.
    /// - **Persona / entity** (a person, place, book without a link):
    ///   skip `--url`; the body alone tells who/what/where.
    ///
    /// Both go into the archival tier — these are things the agent
    /// recalls long-term, not work-in-progress thoughts.
    Cite {
        /// Short, recallable name (e.g. `transformer-paper` or `nikita-host`).
        name: String,
        /// Optional URL of the source. Skip for persona / entity records.
        #[arg(long)]
        url: Option<String>,
        /// One-paragraph summary — what's at the link, or who this entity is.
        #[arg(long)]
        body: String,
    },

    /// Retract an existing edge through the bi-temporal substrate.
    /// Historical reads still see it; reads at NOW skip it.
    Unlink {
        /// Source node name.
        from: String,
        /// Destination node name.
        to: String,
        /// Edge type (defaults `refers-to` to mirror `link`).
        #[arg(long, default_value = "refers-to")]
        r#type: String,
    },

    /// Replace a node with a fresh one carrying new content, connected
    /// via a `supersedes` edge. The old node is retracted; reads at
    /// NOW resolve through the new one.
    ///
    /// Use this when the change is large enough that history should
    /// note "this is a new thing that replaces the old". For in-place
    /// content tweaks use `revise` instead.
    Supersede {
        /// Old node name (or id).
        old: String,
        /// New node type.
        #[arg(long, value_name = "TYPE")]
        r#as: String,
        /// New node name.
        #[arg(long)]
        name: String,
        /// New body.
        #[arg(long)]
        body: String,
        /// Tier override. Defaults from `--as` (archival types go to
        /// archival, others operational).
        #[arg(long, value_name = "TIER")]
        tier: Option<String>,
    },

    /// Time-travel: print what a node looked like at a specific moment.
    ///
    /// `--when` accepts: Unix seconds (`1746549601`), an RFC-3339
    /// datetime (`2026-05-06T12:00:00Z`), or a duration suffix that
    /// counts back from NOW (`5m`, `2h`, `3d` — read as "5 minutes
    /// ago" etc.). If no row was valid at that moment the node prints
    /// as missing rather than as an error.
    At {
        /// Node name (resolved at NOW for the lookup; `at` returns
        /// what the node CONTAINED at the past moment).
        name: String,
        /// Moment to query.
        #[arg(long)]
        when: String,
    },

    /// Print every assertion / retraction row recorded for a node,
    /// chronologically. Each row is marked `+` (assertion) or `-`
    /// (retraction).
    ///
    /// Useful for understanding how a thought evolved — what was
    /// claimed when, when it was revised, when it was retracted by
    /// `forget` / `supersedes`.
    History {
        /// Node name.
        name: String,
    },

    /// Capture a todo as a `Task` node. Auto-named from the body's
    /// first words. Tags: `kind:task`, `status:open`, optional
    /// `due:<YYYY-MM-DD>`, plus the standard `topic:*` / `lang:*`.
    ///
    /// `--due` accepts: a date `2026-05-15`, an RFC-3339 datetime, or
    /// a duration into the future — `3d` = "due in 3 days", `2w` =
    /// "due in 2 weeks". When omitted the task has no deadline.
    Task {
        /// Free-form task description. Quote if it contains spaces.
        body: String,
        /// Optional due date / deadline.
        #[arg(long)]
        due: Option<String>,
    },

    /// Mark a task done. RMW: retracts the open row and reasserts
    /// with `status:done`. The id and name stay the same;
    /// `tagged "status:open"` no longer surfaces it.
    Done {
        /// Task name (or UUIDv7 id).
        name: String,
    },

    /// Bi-temporal forget — retract a node and every edge connected to
    /// it. Historical reads still see it; reads at NOW skip it.
    Forget {
        /// Node name.
        name: String,
    },

    /// Rewrite a node's body and/or name. Implemented as retract +
    /// re-assert so `history` sees both versions.
    Revise {
        /// Node name.
        name: String,
        /// New body. If omitted, keeps current.
        #[arg(long)]
        body: Option<String>,
        /// New name. If omitted, keeps current.
        #[arg(long, value_name = "NEW_NAME")]
        rename: Option<String>,
    },

    /// List every initiative the substrate has at least one node in.
    ///
    /// Use this first when (re-)entering a project: the agent reads the
    /// list, picks one, then runs subsequent commands with
    /// `--initiative <name>`. Names come from whatever value was in
    /// `--initiative` when the original `kaeru episode ...` ran; no
    /// initiative is registered explicitly.
    Initiatives,

    /// Snapshot the substrate as a directory of Obsidian-friendly
    /// markdown pages.
    ///
    /// Layout: `<output-dir>/<tier>/<type>/<sanitized-name>.md`. Each
    /// page has YAML frontmatter (id, type, tier, initiatives, tags),
    /// the body, and `## Outgoing` / `## Incoming` sections grouped by
    /// edge type with `[[wikilink]]` references. The substrate stays
    /// authoritative; the snapshot is a derived view for human reading
    /// and PKM workflows.
    ///
    /// With `--initiative <name>`, only that initiative's nodes export,
    /// and only edges with both endpoints in scope appear. Without the
    /// flag, the whole substrate is dumped.
    Export {
        /// Directory to write into. Created if missing; existing files
        /// are overwritten in place.
        output_dir: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Single Store handle per invocation. Disk-mode through
    // `Store::open_with_config` so vault_path comes from the same env
    // pipeline as every other cap.
    let config = KaeruConfig::from_env()?;
    let store = Store::open_with_config(config)?;

    // If the agent passed --initiative, scope every subsequent
    // mutation/read to that initiative. Without the flag, the store
    // operates cross-initiative.
    if let Some(name) = &cli.initiative {
        store.use_initiative(name);
    }

    match cli.command {
        Command::Config => commands::session::config(&store)?,
        Command::Awake => commands::session::awake(&store)?,
        Command::Overview => commands::session::overview(&store)?,
        Command::Initiatives => commands::session::initiatives(&store)?,
        Command::Recent { since } => commands::session::recent(&store, &since)?,
        Command::Pin { id, reason } => commands::session::pin(&store, &id, &reason)?,
        Command::Unpin { id } => commands::session::unpin(&store, &id)?,

        Command::Episode { name, body } => commands::capture::episode(&store, &name, &body)?,
        Command::Jot { body } => commands::capture::jot(&store, &body)?,
        Command::Link { from, to, r#type } => commands::capture::link(&store, &from, &to, &r#type)?,

        Command::Recall { name } => commands::lookup::recall(&store, &name)?,
        Command::Search { query, limit } => commands::lookup::search(&store, &query, limit)?,
        Command::Summary { id } => commands::lookup::summary(&store, &id)?,
        Command::Drill { name } => commands::lookup::drill(&store, &name)?,
        Command::Trace { name } => commands::lookup::trace(&store, &name)?,
        Command::Ideas => commands::lookup::ideas(&store)?,
        Command::Outcomes => commands::lookup::outcomes(&store)?,

        Command::Claim { text, about } => {
            commands::hypothesis::claim(&store, &text, about.as_deref())?
        }
        Command::Test { hypothesis, method } => {
            commands::hypothesis::test(&store, &hypothesis, &method)?
        }
        Command::Confirm { hypothesis, by } => {
            commands::hypothesis::confirm(&store, &hypothesis, &by)?
        }
        Command::Refute { hypothesis, by } => {
            commands::hypothesis::refute(&store, &hypothesis, &by)?
        }

        Command::Flag { target, reason } => commands::review::flag(&store, &target, &reason)?,
        Command::Resolve { question, by } => commands::review::resolve(&store, &question, &by)?,

        Command::Settle { draft, r#as, name, body } => {
            commands::consolidate::settle(&store, &draft, &r#as, &name, &body)?
        }
        Command::Reopen { archival, r#as, name, body } => {
            commands::consolidate::reopen(&store, &archival, &r#as, &name, &body)?
        }
        Command::Synthesise { from, r#as, name, body, tier } => {
            commands::consolidate::synthesise(
                &store,
                &from,
                &r#as,
                &name,
                &body,
                tier.as_deref(),
            )?
        }

        Command::Between { a, b } => commands::lookup::between(&store, &a, &b)?,
        Command::Tagged { tag } => commands::lookup::tagged(&store, &tag)?,

        Command::Cite { name, url, body } => {
            commands::capture::cite(&store, &name, url.as_deref(), &body)?
        }
        Command::Unlink { from, to, r#type } => {
            commands::capture::unlink(&store, &from, &to, &r#type)?
        }
        Command::Supersede { old, r#as, name, body, tier } => {
            commands::capture::supersede(&store, &old, &r#as, &name, &body, tier.as_deref())?
        }

        Command::At { name, when } => commands::temporal::at(&store, &name, &when)?,
        Command::History { name } => commands::temporal::history(&store, &name)?,

        Command::Task { body, due } => commands::task::task(&store, &body, due.as_deref())?,
        Command::Done { name } => commands::task::done(&store, &name)?,

        Command::Forget { name } => commands::metabolism::forget(&store, &name)?,
        Command::Revise { name, body, rename } => commands::metabolism::revise(
            &store,
            &name,
            body.as_deref(),
            rename.as_deref(),
        )?,

        Command::Lint => commands::lint::lint(&store)?,

        Command::Export { output_dir } => commands::vault::export(&store, &output_dir)?,
    }

    Ok(())
}
