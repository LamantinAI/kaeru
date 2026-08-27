//! Session-restoration & vault-meta tools: `awake`, `overview`,
//! `initiatives`, `recent`, `pin`, `unpin`, `config`.

use std::str::FromStr;

use kaeru_core::{Layer, Store};
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::cloud_client::CloudRegistry;
use crate::utils::{
    brief_suffix, fmt_ts, parse_duration_secs, resolve_name_or_id, text, to_mcp, with_initiative,
};

/// How many rows each read-back section prints before deferring to the verb
/// that owns the full list. `awake` runs on every re-entry, and an initiative
/// can easily carry fifty open tasks — a section has to stay a *pointer* to
/// the work, not a dump of it. The header count is always the true total and
/// the remainder is named out loud, so nothing is silently cut.
const READBACK_CAP: usize = 10;

/// `  … and N more — <how>` for a section that ran past [`READBACK_CAP`], or
/// `""` when it fitted.
fn readback_overflow(total: usize, how: &str) -> String {
    match total.saturating_sub(READBACK_CAP) {
        0 => String::new(),
        n => format!("  … and {n} more — {how}\n"),
    }
}

pub fn awake(store: &Store, initiative: Option<&str>) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let ctx = kaeru_core::awake(store).map_err(to_mcp)?;
        let mut out = String::new();
        out.push_str(&format!(
            "initiative: {}\n",
            ctx.initiative.as_deref().unwrap_or("(none)")
        ));
        out.push_str(&format!(
            "available initiatives ({}): {}\n\n",
            ctx.all_initiatives.len(),
            if ctx.all_initiatives.is_empty() {
                "(none)".to_string()
            } else {
                ctx.all_initiatives.join(", ")
            }
        ));

        // Did-you-mean: the active scope matched no known initiative — a typo,
        // or a brand-new project. Surface a suggestion instead of a silently
        // empty context (a hint, not an error).
        if let Some(active) = ctx.initiative.as_deref()
            && !ctx.all_initiatives.iter().any(|n| n == active)
        {
            match kaeru_core::suggest_initiative(store, active).ok().flatten() {
                Some(s) => out.push_str(&format!(
                    "↳ no nodes under `{active}` yet — did you mean `{s}`? (or it's a fresh project)\n\n"
                )),
                None => out.push_str(&format!(
                    "↳ `{active}` has no nodes yet — a fresh project, or pick one from the list above.\n\n"
                )),
            }
        }

        // Layer-prioritised re-entry context: whole Core first, then Hot,
        // then Warm — load these into working context in this order.
        for bucket in &ctx.layered {
            out.push_str(&format!(
                "{} layer ({}):\n",
                bucket.layer.as_str(),
                bucket.nodes.len()
            ));
            for b in &bucket.nodes {
                out.push_str(&format!("  - {} ({}) — {}\n", b.name, b.node_type, b.id));
            }
        }
        out.push('\n');

        // Cortex — the archival tier: settled knowledge that re-enters every
        // session, separate from the operational working set above.
        out.push_str(&format!(
            "cortex — settled knowledge ({}):\n",
            ctx.cortex.len()
        ));
        for b in &ctx.cortex {
            out.push_str(&format!("  - {} ({}) — {}\n", b.name, b.node_type, b.id));
        }
        out.push('\n');

        out.push_str(&format!("pinned ({}):\n", ctx.pinned.len()));
        for id in &ctx.pinned {
            out.push_str(&format!("  - {id}{}\n", brief_suffix(store, id)));
        }
        out.push('\n');
        out.push_str(&format!("recent ({}):\n", ctx.recent.len()));
        for id in &ctx.recent {
            out.push_str(&format!("  - {id}{}\n", brief_suffix(store, id)));
        }
        out.push('\n');
        out.push_str(&format!("under review ({}):\n", ctx.under_review.len()));
        for id in &ctx.under_review {
            out.push_str(&format!("  - {id}{}\n", brief_suffix(store, id)));
        }

        // Read-back sections. Everything above is what was *touched*; these
        // three are what is still *owed* — the entities that were written and
        // then never revisited, because nothing on the re-entry path
        // mentioned them.
        // Tasks come deadline-first, so the cap keeps whatever is most urgent.
        out.push_str(&format!("\nopen tasks ({}):\n", ctx.open_tasks.len()));
        for t in ctx.open_tasks.iter().take(READBACK_CAP) {
            let when = match (&t.due, t.overdue) {
                (Some(d), true) => format!("⚠ OVERDUE {d}"),
                (Some(d), false) => format!("due {d}"),
                (None, _) => "no due date".to_string(),
            };
            out.push_str(&format!("  - [{when}] {} — {}\n", t.name, t.id));
        }
        out.push_str(&readback_overflow(ctx.open_tasks.len(), "`board`"));
        if !ctx.open_tasks.is_empty() {
            out.push_str(
                "↳ `done <name>` when finished, `set_status <name> <status>` to move it, \
                 `board` for the columns.\n",
            );
        }

        out.push_str(&format!("\nopen claims ({}):\n", ctx.open_claims.len()));
        for b in ctx.open_claims.iter().take(READBACK_CAP) {
            out.push_str(&format!("  - {} — {}\n", b.name, b.id));
            if let Some(e) = &b.body_excerpt {
                out.push_str(&format!("    {e}\n"));
            }
        }
        out.push_str(&readback_overflow(
            ctx.open_claims.len(),
            "`tagged \"status:open\"`",
        ));
        if !ctx.open_claims.is_empty() {
            out.push_str(
                "↳ still awaiting a verdict — `confirm <name> --by <evidence>` or \
                 `refute <name> --by <evidence>`.\n",
            );
        }

        out.push_str(&format!("\nchains ({}):\n", ctx.chains.len()));
        for b in ctx.chains.iter().take(READBACK_CAP) {
            out.push_str(&format!("  - {} — {}\n", b.name, b.id));
            if let Some(e) = &b.body_excerpt {
                out.push_str(&format!("    {e}\n"));
            }
        }
        out.push_str(&readback_overflow(
            ctx.chains.len(),
            "`why <name>` on any of them",
        ));
        if !ctx.chains.is_empty() {
            out.push_str("↳ saved reasoning trails — read one with `why <name>`.\n");
        }

        // The second tier. `awake` answers "what was open" from the local
        // vault alone, so on a team initiative it produces a plausible,
        // complete-looking answer while the cloud holds nodes this machine has
        // never seen — and nothing in the reply suggests there is anywhere
        // else to look. Reading the policy is local and free; counting what
        // the cloud holds would mean a network round-trip inside the verb an
        // agent calls first, which is not a cost this line is worth.
        if let Some(init) = ctx.initiative.as_deref() {
            out.push_str(&cloud_tail(store, init));
        }
        Ok(text(&out))
    })
}

/// `↳ cloud: …` for an initiative whose policy lets it share — the reminder
/// that the local graph is not the whole graph.
///
/// Only the policy is read, and only locally: an initiative that cannot share
/// has no second tier to mention, and one that can is exactly the case where a
/// confident local-only answer is a wrong answer. Says nothing about how many
/// nodes are up there, because knowing would cost a network call in the verb
/// an agent runs first.
fn cloud_tail(store: &Store, initiative: &str) -> String {
    let permits = kaeru_core::get_share_policy(store, initiative)
        .map(|p| p.permits_share())
        .unwrap_or(false);
    if !permits {
        return String::new();
    }
    format!(
        "\ncloud: `{initiative}` is a shared initiative — the cloud may hold nodes this vault \
         does not.\n↳ `cloud_recall {initiative}` lists what the team shared; `pull <id> \
         {initiative}` brings one in. A local-only answer here can be confidently incomplete.\n"
    )
}

pub fn overview(store: &Store, initiative: Option<&str>) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let mut report = kaeru_core::overview(store).map_err(to_mcp)?;
        if let Some(init) = initiative {
            report.push_str(&cloud_tail(store, init));
        }
        Ok(text(&report))
    })
}

pub fn initiatives(store: &Store) -> Result<CallToolResult, McpError> {
    let names = kaeru_core::list_initiatives(store).map_err(to_mcp)?;
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

pub fn recent(
    store: &Store,
    since: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let window = parse_duration_secs(since).map_err(to_mcp)?;
        let ids = kaeru_core::recent_episodes(store, window).map_err(to_mcp)?;
        let mut out = format!("recent ({}):\n", ids.len());
        for id in &ids {
            out.push_str(&format!("  - {id}{}\n", brief_suffix(store, id)));
        }
        Ok(text(&out))
    })
}

pub fn pin(
    store: &Store,
    name_or_id: &str,
    reason: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name_or_id)?;
        kaeru_core::pin(store, &id, reason).map_err(to_mcp)?;
        Ok(text(&format!("pinned: {name_or_id} ({id})")))
    })
}

pub fn unpin(
    store: &Store,
    name_or_id: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name_or_id)?;
        kaeru_core::unpin(store, &id).map_err(to_mcp)?;
        Ok(text(&format!("unpinned: {name_or_id} ({id})")))
    })
}

/// Renders the configured clouds, marking the default.
///
/// `config` printed the vault path and every cap and said nothing at all
/// about clouds, so the only way to learn what was connected — or which one
/// an unnamed call would have used — was to read the TOML by hand, from
/// outside the agent's surface (#65).
pub fn render_clouds(clouds: &CloudRegistry) -> String {
    let names = clouds.names();
    if names.is_empty() {
        return "clouds               = (none configured)\n".to_string();
    }
    let default = clouds.default_name();
    let listed = names
        .iter()
        .map(|n| {
            if Some(*n) == default {
                format!("{n} (default)")
            } else {
                (*n).to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(", ");
    let mut out = format!("clouds               = {listed}\n");
    if names.len() > 1 {
        out.push_str(
            "                       (several configured — cloud verbs require an explicit \
             `cloud`)\n",
        );
    }
    out
}

/// `clouds` — what this daemon can reach, and where.
///
/// Separate from `config` because it is the answer to a question an agent
/// asks on its own ("which clouds are there?"), not a dump of settings. Shows
/// the endpoint too: an error naming a URL was, until now, the only reliable
/// way to tell which cloud a call had actually gone to.
pub fn clouds(clouds: &CloudRegistry) -> Result<CallToolResult, McpError> {
    let names = clouds.names();
    if names.is_empty() {
        return Ok(text(
            "(no clouds configured — add one to `clouds.toml` and restart the daemon; it is read \
             at startup only)",
        ));
    }
    let default = clouds.default_name();
    let mut out = format!("clouds ({}):\n", names.len());
    for n in &names {
        let mark = if Some(*n) == default {
            " (default)"
        } else {
            ""
        };
        let url = clouds.get(Some(n)).map(|c| c.base_url()).unwrap_or("");
        out.push_str(&format!("  - {n}{mark} — {url}\n"));
    }
    // `clouds.toml` is read once, at startup. Printing when answers the
    // question that is otherwise unanswerable from inside the daemon: whether
    // an edit has taken effect. A client reconnect does not respawn it.
    if let Some(ts) = clouds.loaded_at() {
        out.push_str(&format!(
            "\nconfig read at {} — `clouds.toml` is read once at daemon startup; restart the \
             daemon to pick up an edit (a client reconnect does not).\n",
            fmt_ts(ts)
        ));
    }
    if names.len() > 1 {
        out.push_str(
            "\n↳ with more than one cloud, `share` / `pull` / `cloud_recall` and the initiative \
             verbs need `cloud` named explicitly — nothing is routed to a default you did not \
             choose.",
        );
    }
    Ok(text(&out))
}

pub fn config(store: &Store, clouds: &CloudRegistry) -> Result<CallToolResult, McpError> {
    let c = store.config();
    let out = format!(
        "kaeru {}\nvault_path           = {}\n{}active_window_size   = {}\nrecent_episodes_cap  = {}\nawake_window_secs    = {}\nsummary_children_cap = {}\nbody_excerpt_chars   = {}\nprovenance_max_hops  = {}\ndefault_max_hops     = {}\nmax_hops_cap         = {}\n",
        kaeru_core::version(),
        c.vault_path.display(),
        render_clouds(clouds),
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

/// Explicit layered recall — surfaces nodes from the requested memory
/// layers, on demand. `awake` only loads Core/Hot/Warm; this is how you
/// reach `cold` / `frozen` (archived / not-surfaced-by-default) when you
/// know you need them. `layers` is a comma/space list (e.g. `cold,frozen`);
/// defaults to `cold,frozen`. Scoped to the active initiative when given.
pub fn surface(
    store: &Store,
    layers: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let spec = layers.unwrap_or("cold,frozen");
        let mut parsed: Vec<Layer> = Vec::new();
        for tok in spec
            .split([',', ' '])
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            parsed.push(Layer::from_str(tok).map_err(to_mcp)?);
        }
        if parsed.is_empty() {
            parsed = vec![Layer::Cold, Layer::Frozen];
        }

        let buckets = kaeru_core::recall_by_layer(store, &parsed).map_err(to_mcp)?;
        let mut out = String::new();
        for bucket in &buckets {
            out.push_str(&format!(
                "{} layer ({}):\n",
                bucket.layer.as_str(),
                bucket.nodes.len()
            ));
            for b in &bucket.nodes {
                out.push_str(&format!("  - {} ({}) — {}\n", b.name, b.node_type, b.id));
            }
        }
        if out.is_empty() {
            out.push_str("(no nodes in the requested layers)");
        }
        Ok(text(&out))
    })
}

/// Static how-to-import guide. Returned verbatim so an agent about to
/// bulk-load knowledge (e.g. from a markdown export) does the right
/// thing without guessing: which verb matches which epistemic status,
/// how to stamp the memory layer at creation, and to link after writing.
pub fn import_guide() -> Result<CallToolResult, McpError> {
    let guide = r#"# kaeru import guide

Goal: load knowledge so a future agent recalls the right things first.

## 1. Scope every call
Pick/confirm the initiative first: `initiatives` -> `awake(initiative)` ->
`overview(initiative)`. Pass `initiative` on EVERY create/link call —
untagged writes are invisible to a scoped `overview`.

## 2. Choose the verb by epistemic status (not by length)
- `cite <name> --body ... [--url ...]`  -> settled facts, specs, decisions,
  references, persona/entity records. Lands in archival/reference.
- `episode <name> --body ...`           -> a named observation tied to work.
- `jot --body ...`                      -> a fleeting note (auto-named).
- `claim --text ... [--about X]`        -> a hypothesis under test
  (then `test` -> `confirm`/`refute`).
- `task --body ... [--due ...]` / `done`-> actionable todos.
- `synthesise` -> `settle`              -> promote converged operational
  seeds into one durable archival insight.

## 3. Stamp the layer AT creation (importance => recall priority)
Every create verb takes an optional `layer`. Injection order is
core -> hot -> warm -> cold -> frozen. Set it now; don't rely on a
later `layer` call.
- core   : foundational truth, always in context (architecture, current status,
           the one fact everything hinges on). Keep this set small.
- hot    : active work and recent changes; open blocking tasks; live hypotheses.
- warm   : default; useful reference, contacts, access points.
- cold   : passed stages, completed tasks, superseded notes, old probes.
- frozen : keep-for-the-record, do not surface.
A wrong-but-present layer beats a missing one; refine later if needed.

## 4. Link AFTER capturing
A node with no edges is easy to lose. After writing, `search` for related
nodes and `link from to --edge_type ...`
(`refers_to` default; also `derived_from`, `supersedes`, `causal`,
`part_of`, `blocks`, `contradicts`).

## 5. Bulk import from a markdown export
For each page: recreate it with the verb matching its tier/type and a
layer from its importance; then recreate the `## Outgoing` / `## Incoming`
edges with `link`. Don't import mechanically — drop stale operational
noise, keep settled knowledge and active work.
"#;
    Ok(text(guide))
}

#[cfg(test)]
mod tests {
    use kaeru_core::{EdgeType, EpisodeKind, Significance, Store};
    use rmcp::model::CallToolResult;

    use super::awake;

    fn text_of(r: CallToolResult) -> String {
        r.content
            .iter()
            .filter_map(|c| c.as_text().map(|t| t.text.clone()))
            .collect::<Vec<_>>()
            .join("")
    }

    fn store_t() -> Store {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        store
    }

    /// The case the audit caught live: a task due last week, the agent working
    /// the same initiative all along, and nothing on the re-entry path ever
    /// saying the deadline had passed.
    #[test]
    fn a_past_due_task_re_enters_marked_overdue() {
        let store = store_t();
        kaeru_core::write_task(&store, "renew the certificate", Some("2000-01-01")).expect("task");
        let out = text_of(awake(&store, Some("t")).unwrap());
        assert!(out.contains("open tasks (1)"), "section is there: {out}");
        assert!(
            out.contains("⚠ OVERDUE 2000-01-01"),
            "the passed deadline is called out: {out}"
        );
        assert!(out.contains("`done <name>`"), "and how to close it: {out}");
    }

    /// A completed task is not open work; the section must go quiet.
    #[test]
    fn a_done_task_leaves_the_re_entry_view() {
        let store = store_t();
        let id = kaeru_core::write_task(&store, "already handled", None).expect("task");
        kaeru_core::complete_task(&store, &id).expect("done");
        let out = text_of(awake(&store, Some("t")).unwrap());
        assert!(out.contains("open tasks (0)"), "nothing owed: {out}");
        assert!(
            !out.contains("`done <name>`"),
            "no how-to on an empty section: {out}"
        );
    }

    /// A claim written and never settled is invisible without this — the only
    /// other route is remembering `tagged "status:open"`.
    #[test]
    fn an_unsettled_claim_re_enters_as_open() {
        let store = store_t();
        kaeru_core::formulate_hypothesis(&store, "caching-wins", "the cache pays for itself")
            .expect("claim");
        let out = text_of(awake(&store, Some("t")).unwrap());
        assert!(out.contains("open claims (1)"), "section is there: {out}");
        assert!(out.contains("caching-wins"), "named: {out}");
        assert!(
            out.contains("--by <evidence>"),
            "and how to settle it: {out}"
        );
    }

    /// A chain is authored for the next session; re-entry has to show it as a
    /// named trail with its summary, not as one more line in the working set.
    #[test]
    fn a_chain_re_enters_with_its_summary_and_points_at_why() {
        let store = store_t();
        let mk = |n: &str| {
            kaeru_core::write_episode(&store, EpisodeKind::Observation, Significance::Low, n, n)
                .expect("write")
        };
        let (a, b) = (mk("start"), mk("decision"));
        kaeru_core::link_with_weight(&store, &a, &b, EdgeType::RefersTo, 0.9).expect("link");
        kaeru_core::create_chain(
            &store,
            &a,
            &b,
            Some("the-trail"),
            Some("why we picked the second option"),
        )
        .expect("chain")
        .expect("path exists");

        let out = text_of(awake(&store, Some("t")).unwrap());
        assert!(out.contains("chains (1)"), "section is there: {out}");
        assert!(out.contains("the-trail"), "named: {out}");
        assert!(
            out.contains("why we picked the second option"),
            "the author's summary is printed, not just the name: {out}"
        );
        assert!(out.contains("`why <name>`"), "and how to read it: {out}");
    }

    /// `awake` runs every re-entry, so a long list has to stay a pointer to
    /// the work rather than a dump of it — but the header count must still be
    /// the true total, and the remainder must be named, not silently cut.
    #[test]
    fn a_long_task_list_is_capped_but_never_silently() {
        let store = store_t();
        for i in 0..14 {
            kaeru_core::write_task(&store, &format!("chore number {i}"), None).expect("task");
        }
        let out = text_of(awake(&store, Some("t")).unwrap());
        assert!(out.contains("open tasks (14)"), "true total: {out}");
        assert!(
            out.contains("… and 4 more — `board`"),
            "remainder named: {out}"
        );
    }

    /// The most urgent survive the cap: tasks are ordered deadline-first, so
    /// an overdue one can never be pushed out by a pile of undated chores.
    #[test]
    fn the_cap_keeps_the_overdue_task() {
        let store = store_t();
        for i in 0..14 {
            kaeru_core::write_task(&store, &format!("chore number {i}"), None).expect("task");
        }
        kaeru_core::write_task(&store, "renew the certificate", Some("2000-01-01")).expect("task");
        let out = text_of(awake(&store, Some("t")).unwrap());
        assert!(
            out.contains("⚠ OVERDUE 2000-01-01"),
            "the deadline outranks the chores: {out}"
        );
    }

    fn registry(names: &[&str], default: Option<&str>) -> crate::cloud_client::CloudRegistry {
        use std::collections::HashMap;

        use crate::cloud_client::{CloudClient, CloudRegistry};
        let clients: HashMap<String, CloudClient> = names
            .iter()
            .map(|n| {
                (
                    n.to_string(),
                    CloudClient::new(n.to_string(), format!("https://{n}.test"), String::new()),
                )
            })
            .collect();
        CloudRegistry::new(clients, default.map(str::to_string))
    }

    /// The configuration was invisible from inside: `config` printed the vault
    /// path and every cap and said nothing about clouds, so which one an
    /// unnamed call would use could only be learned by reading the TOML by
    /// hand — from outside the agent's surface.
    #[test]
    fn config_names_the_clouds_and_the_default() {
        let store = store_t();
        let out =
            text_of(super::config(&store, &registry(&["alpha", "beta"], Some("beta"))).unwrap());
        assert!(out.contains("clouds"), "{out}");
        assert!(
            out.contains("beta (default)"),
            "the default is marked: {out}"
        );
        assert!(out.contains("alpha"), "and the others listed: {out}");
        assert!(
            out.contains("require an explicit"),
            "and says the rule that follows from there being several: {out}"
        );
    }

    /// With one cloud there is no ambiguity to warn about, so the note stays
    /// out of the way.
    #[test]
    fn a_single_cloud_gets_no_warning() {
        let store = store_t();
        let out = text_of(super::config(&store, &registry(&["only"], None)).unwrap());
        assert!(out.contains("only (default)"), "{out}");
        assert!(!out.contains("require an explicit"), "{out}");
    }

    /// `clouds` answers the question an agent asks on its own, and shows the
    /// endpoint — until now, an error naming a URL was the only reliable way
    /// to tell which cloud a call had actually reached.
    #[test]
    fn the_clouds_verb_shows_endpoints() {
        let out = text_of(super::clouds(&registry(&["alpha", "beta"], Some("alpha"))).unwrap());
        assert!(out.contains("https://alpha.test"), "{out}");
        assert!(out.contains("alpha (default)"), "{out}");
        assert!(out.contains("nothing is routed to a default"), "{out}");
    }

    /// No clouds is a configuration state, not an error, and says how to fix
    /// it — including that the file is read once at startup.
    #[test]
    fn no_clouds_says_how_to_add_one() {
        let out = text_of(super::clouds(&registry(&[], None)).unwrap());
        assert!(out.contains("clouds.toml"), "{out}");
        assert!(
            out.contains("read \n             at startup") || out.contains("at startup"),
            "{out}"
        );
    }

    /// The half-hour bug: `clouds.toml` is read once at startup, a client
    /// reconnect does not respawn the daemon, and nothing said so — the errors
    /// just kept naming the old configuration.
    #[test]
    fn the_clouds_verb_says_when_the_config_was_read() {
        let out = text_of(super::clouds(&registry(&["alpha"], None)).unwrap());
        assert!(out.contains("config read at"), "{out}");
        assert!(
            out.contains("restart the daemon"),
            "and what to do about an edit: {out}"
        );
    }

    /// The fail-silent case #57 measured: on a team initiative, `awake` gave a
    /// complete-looking answer from the local vault alone while the cloud held
    /// nodes this machine had never seen, and nothing hinted there was
    /// anywhere else to look. In one such initiative, 18 of 25 reading
    /// sessions answered "what do we know" from the local graph only.
    #[test]
    fn a_shared_initiative_says_the_local_answer_may_be_incomplete() {
        let store = store_t();
        kaeru_core::set_share_policy(&store, "t", kaeru_core::SharePolicy::Team).expect("policy");

        let out = text_of(awake(&store, Some("t")).unwrap());
        assert!(out.contains("cloud:"), "{out}");
        assert!(out.contains("`cloud_recall t`"), "names the verb: {out}");
        assert!(
            out.contains("confidently incomplete"),
            "and says why it matters: {out}"
        );
    }

    /// A private initiative has no second tier, so the line would be noise on
    /// every re-entry of every solo project.
    #[test]
    fn a_private_initiative_gets_no_cloud_line() {
        let store = store_t();
        let out = text_of(awake(&store, Some("t")).unwrap());
        assert!(!out.contains("cloud:"), "{out}");
    }

    /// Reading the policy is local, so the line costs no network call inside
    /// the verb an agent runs first — it says a tier exists, never how much is
    /// in it.
    #[test]
    fn the_cloud_line_claims_no_counts() {
        let store = store_t();
        kaeru_core::set_share_policy(&store, "t", kaeru_core::SharePolicy::Team).expect("policy");
        let out = text_of(awake(&store, Some("t")).unwrap());
        assert!(out.contains("may hold"), "hedged, not counted: {out}");
    }
}
