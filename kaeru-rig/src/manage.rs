//! Session re-entry, initiative management, diagnostics, and snapshot export.

use kaeru_core::{
    OpenTask, attach_node, awake, export_vault, lint, list_initiatives, merge_initiative, overview,
    parse_duration_secs, pin, recent_writes, reflect, suggest_initiative, unpin,
};
use serde::Deserialize;
use serde_json::json;

use crate::lookup::ScopeArgs;
use crate::{
    briefs, briefs_by_ids, mem_tool, mem_tool_in, mem_tool_unscoped, resolve, resolve_global,
};

/// A verb that takes nothing at all — the daemon's `initiatives` is the only
/// read that is deliberately cross-project.
#[derive(Debug, Deserialize)]
pub struct NoArgs {}

mem_tool_in!(
    /// `kaeru_awake` — load the re-entry context for the active initiative.
    Awake,
    "kaeru_awake",
    "Load your working memory for this project: what was pinned, what happened recently, what's \
     still under review, plus the unfinished work — open tasks (overdue first), claims awaiting a \
     verdict, and the saved reasoning trails. Call this first when picking up a session to \
     recover context.",
    ScopeArgs,
    { "type": "object", "properties": {
        "initiative": { "type": "string", "description": "optional initiative (project) to read; omit for your default" }
    } },
    |store, _args| match awake(store) {
        Ok(ctx) => {
            let mut out = json!({
                "initiative": ctx.initiative,
                "all_initiatives": ctx.all_initiatives,
                "cortex": briefs(&ctx.cortex),
                "layered": ctx
                    .layered
                    .iter()
                    .map(|b| json!({
                        "layer": b.layer.as_str(),
                        "nodes": briefs(&b.nodes),
                        "chars": b.chars,
                        "omitted": b.omitted,
                    }))
                    .collect::<Vec<_>>(),
                "pinned": briefs_by_ids(store, &ctx.pinned),
                "recent": briefs_by_ids(store, &ctx.recent),
                "under_review": briefs_by_ids(store, &ctx.under_review),
                // What the graph has already ruled on, by id: a listed node
                // something live has replaced or refuted (#92).
                "superseded": ctx
                    .verdicts
                    .iter()
                    .map(|(id, v)| (id.clone(), v.phrase()))
                    .collect::<std::collections::BTreeMap<_, _>>(),
                "open_tasks": open_tasks_json(&ctx.open_tasks),
                "open_claims": briefs(&ctx.open_claims),
                "chains": briefs(&ctx.chains),
            });
            // Did-you-mean parity with MCP: an active scope that matches no known
            // initiative (a typo, or a fresh project) suggests the closest one.
            if let Some(active) = ctx.initiative.as_deref()
                && !ctx.all_initiatives.iter().any(|n| n == active)
                && let Ok(Some(s)) = suggest_initiative(store, active)
            {
                out["did_you_mean"] = json!(s);
            }
            out
        }
        Err(e) => json!({ "error": e.to_string() }),
    }
);

mem_tool_in!(
    /// `kaeru_overview` — a terminal-readable map of the initiative's subgraph.
    Overview,
    "kaeru_overview",
    "Get a readable map of what this project's memory knows — the subgraph overview. Pairs with \
     `kaeru_awake` (process state) to answer \"what does this project know\".",
    ScopeArgs,
    { "type": "object", "properties": {
        "initiative": { "type": "string", "description": "optional initiative (project) to read; omit for your default" }
    } },
    |store, _args| match overview(store) {
        Ok(text) => json!({ "overview": text }),
        Err(e) => json!({ "error": e.to_string() }),
    }
);

mem_tool!(
    /// `kaeru_initiatives` — list every project the substrate knows.
    Initiatives,
    "kaeru_initiatives",
    "List every initiative (project) the memory knows about, across all of them.",
    NoArgs,
    { "type": "object", "properties": {} },
    |store, _args| match list_initiatives(store) {
        Ok(names) => json!({ "initiatives": names }),
        Err(e) => json!({ "error": e.to_string() }),
    }
);

#[derive(Debug, Deserialize)]
pub struct RecentArgs {
    /// Look-back window as a human string — `30m`, `3h`, `2d`, or raw
    /// seconds. The same vocabulary the daemon takes.
    #[serde(default = "default_since")]
    pub since: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

fn default_since() -> String {
    "24h".to_string()
}

mem_tool_in!(
    /// `kaeru_recent` — episodes from the recent past.
    Recent,
    "kaeru_recent",
    "List what was CAPTURED lately in this project — every kind of write, not only episodes: \
     references, claims, tasks and jots all count. This is what answers \"did what I just write \
     land?\", so a zero really means nothing was captured. `since` sets the look-back: `30m`, \
     `3h`, `2d`, or raw seconds (default 24h). Pass `initiative` for a specific project; omit \
     for your default.",
    RecentArgs,
    { "type": "object", "properties": {
        "since": { "type": "string", "description": "look-back window: `30m`, `3h`, `2d`, or raw seconds (default 24h)" },
        "initiative": { "type": "string", "description": "optional initiative (project); omit for your default" }
    } },
    |store, args| {
        let window = match parse_duration_secs(&args.since) {
            Ok(secs) => secs,
            Err(e) => return json!({ "error": e.to_string() }),
        };
        match recent_writes(store, window) {
            Ok(ids) => json!({ "recent": briefs_by_ids(store, &ids) }),
            Err(e) => json!({ "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct PinArgs {
    pub name: String,
    pub reason: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_pin` — pin a node into the active window.
    Pin,
    "kaeru_pin",
    "Pin a memory so it stays in your active working window across the session, with a reason.",
    PinArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "node name or id" },
        "reason": { "type": "string", "description": "why it's pinned" },
        "initiative": { "type": "string", "description": "optional initiative (project) to resolve within; omit for your default" }
    }, "required": ["name", "reason"] },
    |store, args| {
        let id = resolve(store, &args.name);
        match pin(store, &id, &args.reason) {
            Ok(()) => json!({ "pinned": true, "id": id }),
            Err(e) => json!({ "pinned": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct UnpinArgs {
    pub name: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_unpin` — remove a node from the active window.
    Unpin,
    "kaeru_unpin",
    "Unpin a memory — remove it from your active working window.",
    UnpinArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "node name or id" },
        "initiative": { "type": "string", "description": "optional initiative (project) to resolve within; omit for your default" }
    }, "required": ["name"] },
    |store, args| {
        let id = resolve(store, &args.name);
        match unpin(store, &id) {
            Ok(()) => json!({ "unpinned": true, "id": id }),
            Err(e) => json!({ "unpinned": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct RenameInitiativeArgs {
    pub old: String,
    pub new: String,
    /// Name the cloud to rename it there too — team-wide, and not undoable
    /// from here. Omitted, the rename is local.
    #[serde(default)]
    pub cloud: Option<String>,
}

mem_tool_unscoped!(
    /// `kaeru_rename_initiative` — rename a project (moves all its nodes/edges).
    RenameInitiative,
    "kaeru_rename_initiative",
    "Rename an initiative — moves all its nodes, edges, and sharing policy to the new name (fails \
     if the new name already exists). Local by default. Pass `cloud=\"<name>\"` to ALSO rename it \
     in that shared cloud, which is team-wide and affects everyone.",
    RenameInitiativeArgs,
    { "type": "object", "properties": {
        "old": { "type": "string", "description": "current initiative name" },
        "new": { "type": "string", "description": "new initiative name (must not exist)" },
        "cloud": { "type": "string", "description": "name a cloud to rename it there as well (team-wide)" }
    }, "required": ["old", "new"] },
    |mem, a| {
        // Local first: if it fails (a name collision, say) the cloud is
        // untouched, which is the order that cannot leave the two disagreeing.
        let (old, new) = (a.old.clone(), a.new.clone());
        let local = mem
            .blocking(move |s| kaeru_core::rename_initiative(s, &old, &new))
            .await;
        let stats = match local {
            Ok(stats) => stats,
            Err(e) => return json!({ "renamed": false, "error": e.to_string() }),
        };
        let mut out = json!({ "renamed": true, "nodes": stats.nodes, "edges": stats.edges });
        // A cloud is reached only when named — this one is team-wide.
        if let Some(name) = a.cloud.as_deref() {
            match mem.cloud(Some(name)) {
                Some(client) => match client.rename_initiative(&a.old, &a.new).await {
                    Ok((code, body)) if (200..300).contains(&code) => {
                        out["cloud"] = json!({ "name": client.name(), "renamed": true, "body": body });
                    }
                    Ok((code, body)) => {
                        out["cloud"] = json!({ "name": client.name(), "renamed": false, "status": code, "body": body });
                    }
                    Err(e) => out["cloud"] = json!({ "name": client.name(), "renamed": false, "error": e }),
                },
                None => out["cloud"] = json!({ "renamed": false, "error": format!("no cloud named `{name}`") }),
            }
        }
        out
    }
);

#[derive(Debug, Deserialize)]
pub struct DeleteInitiativeArgs {
    pub name: String,
    /// Name the cloud to delete it there too — removes it for everyone, and
    /// cannot be undone there.
    #[serde(default)]
    pub cloud: Option<String>,
}

mem_tool_unscoped!(
    /// `kaeru_delete_initiative` — drop a project's scoping (forgets exclusive nodes).
    DeleteInitiative,
    "kaeru_delete_initiative",
    "Delete an initiative — drops its scoping and forgets the nodes exclusive to it (bi-temporal: \
     recoverable via `kaeru_at` at a past time). Nodes shared with other initiatives only lose \
     this membership. Local by default. Pass `cloud=\"<name>\"` to ALSO delete it from that \
     shared cloud, which removes it for everyone and cannot be undone there.",
    DeleteInitiativeArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "initiative to delete" },
        "cloud": { "type": "string", "description": "name a cloud to delete it there as well (team-wide, permanent)" }
    }, "required": ["name"] },
    |mem, a| {
        let name = a.name.clone();
        let local = mem
            .blocking(move |s| kaeru_core::delete_initiative(s, &name))
            .await;
        let stats = match local {
            Ok(stats) => stats,
            Err(e) => return json!({ "deleted": false, "error": e.to_string() }),
        };
        let mut out = json!({
            "deleted": true, "unscoped": stats.unscoped, "forgotten": stats.forgotten
        });
        if let Some(cloud_name) = a.cloud.as_deref() {
            match mem.cloud(Some(cloud_name)) {
                Some(client) => match client.delete_initiative(&a.name).await {
                    Ok((code, body)) if (200..300).contains(&code) => {
                        out["cloud"] = json!({ "name": client.name(), "deleted": true, "body": body });
                    }
                    Ok((code, body)) => {
                        out["cloud"] = json!({ "name": client.name(), "deleted": false, "status": code, "body": body });
                    }
                    Err(e) => out["cloud"] = json!({ "name": client.name(), "deleted": false, "error": e }),
                },
                None => out["cloud"] = json!({ "deleted": false, "error": format!("no cloud named `{cloud_name}`") }),
            }
        }
        out
    }
);

#[derive(Debug, Deserialize)]
pub struct MergeInitiativeArgs {
    pub source: String,
    pub target: String,
}

mem_tool!(
    /// `kaeru_merge_initiative` — re-home one project's memory into another.
    MergeInitiative,
    "kaeru_merge_initiative",
    "Merge one initiative into another: every node and edge of `source` gains membership in \
     `target`, then `source` is dropped. Use it when one project's memory ended up split across \
     two names — unlike attach-then-delete it cannot lose a node you missed, because memberships \
     are added before the source's rows are removed. Local only.",
    MergeInitiativeArgs,
    { "type": "object", "properties": {
        "source": { "type": "string", "description": "initiative to merge away (disappears)" },
        "target": { "type": "string", "description": "initiative that keeps everything" }
    }, "required": ["source", "target"] },
    |store, args| match merge_initiative(store, &args.source, &args.target) {
        Ok(stats) => json!({
            "merged": true,
            "nodes": stats.nodes,
            "edges": stats.edges,
            "source": args.source,
            "target": args.target,
        }),
        Err(e) => json!({ "merged": false, "error": e.to_string() }),
    }
);

#[derive(Debug, Deserialize)]
pub struct AttachArgs {
    pub node: String,
    pub to: String,
}

mem_tool!(
    /// `kaeru_attach` — give a node a second initiative (additive multi-membership).
    Attach,
    "kaeru_attach",
    "Add a node to another initiative (additive multi-membership) — repair fragmentation by giving \
     a node captured under the wrong or a stale initiative a second home, without moving or \
     copying it (same id, edges, history). Idempotent. Local only.",
    AttachArgs,
    { "type": "object", "properties": {
        "node": { "type": "string", "description": "node name or id (resolved across all initiatives)" },
        "to": { "type": "string", "description": "target initiative to add the node to" }
    }, "required": ["node", "to"] },
    |store, args| {
        let id = resolve_global(store, &args.node);
        match attach_node(store, &id, &args.to) {
            Ok(stats) => json!({ "attached": true, "already_member": stats.already_member, "id": id, "to": args.to }),
            Err(e) => json!({ "attached": false, "error": e.to_string() }),
        }
    }
);

mem_tool_in!(
    /// `kaeru_lint` — surface orphans and unresolved reviews.
    Lint,
    "kaeru_lint",
    "Check the memory for hygiene issues: orphan nodes (no edges), unresolved review flags, \
     dangling edges (an endpoint was retracted), and pairs that supersede each other in both \
     directions. Use it to find loose ends worth tidying.",
    ScopeArgs,
    { "type": "object", "properties": {
        "initiative": { "type": "string", "description": "optional initiative (project) to read; omit for your default" }
    } },
    |store, _args| match lint(store) {
        Ok(report) => json!({
            "orphans": report.orphans,
            "unresolved_reviews": report.unresolved_reviews,
            "dangling_edges": report.dangling_edges,
            "supersedes_conflicts": report.supersedes_conflicts,
        }),
        Err(e) => json!({ "error": e.to_string() }),
    }
);

mem_tool_in!(
    /// `kaeru_reflect` — computed maintenance work-list for a reflection pass.
    Reflect,
    "kaeru_reflect",
    "Reflect on the store: a computed maintenance work-list — orphans to link, overdue tasks to \
     close, open reviews to resolve, stale chains to rechain, settled operational nodes to promote \
     into cortex, and shared/cloud items that need the user's sign-off (never auto-rebalanced). \
     Good for a periodic tidy pass.",
    ScopeArgs,
    { "type": "object", "properties": {
        "initiative": { "type": "string", "description": "optional initiative (project) to read; omit for your default" }
    } },
    |store, _args| match reflect(store) {
        Ok(r) => json!({
            "orphans": r.orphans,
            "open_reviews": r.open_reviews,
            "stale_chains": r.stale_chains,
            "cortex_candidates": r.cortex_candidates,
            // Delivered work nothing points at any more. Kept apart from the
            // candidates because the move is `layer cold`, not `settle` —
            // cortex loads whole every session (#76).
            "archivable": r.archivable,
            // What cortex holds now, so a host can price the recommendation
            // before applying it.
            "cortex_size": r.cortex_size,
            "shared_needs_user": r.shared,
            // Edges whose cloud copy can be stale — the candidate set, not a
            // diagnosis; only the cloud knows what the cloud holds (#85).
            "shared_edges": r.shared_edges,
            // Names that may be one project split in two (#86).
            "duplicate_initiatives": r.duplicate_initiatives,
            "overdue_tasks": r.overdue_tasks,
        }),
        Err(e) => json!({ "error": e.to_string() }),
    }
);

#[derive(Debug, Deserialize)]
pub struct ExportArgs {
    /// Output directory for the markdown snapshot.
    pub output_dir: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_export` — write an Obsidian-friendly markdown snapshot.
    Export,
    "kaeru_export",
    "Export the current initiative to an Obsidian-friendly markdown vault on disk (README / INDEX \
     / LOG plus node pages). Use when a human wants to read the memory offline.",
    ExportArgs,
    { "type": "object", "properties": {
        "output_dir": { "type": "string", "description": "output directory for the snapshot" },
        "initiative": { "type": "string", "description": "optional initiative (project) to export; omit for your default" }
    }, "required": ["output_dir"] },
    |store, args| match export_vault(store, &args.output_dir) {
        Ok(summary) => json!({
            "exported": true,
            "nodes": summary.nodes_exported,
            "root": summary.root.to_string_lossy(),
        }),
        Err(e) => json!({ "exported": false, "error": e.to_string() }),
    }
);

/// Open tasks as JSON cards — the `due:` date and the overdue flag lifted out
/// of the tag list, so a caller doesn't have to re-derive "is this late?".
fn open_tasks_json(tasks: &[OpenTask]) -> serde_json::Value {
    serde_json::Value::Array(
        tasks
            .iter()
            .map(|t| {
                json!({
                    "id": t.id,
                    "name": t.name,
                    "excerpt": t.body_excerpt,
                    "status": t.status,
                    "due": t.due,
                    "overdue": t.overdue,
                })
            })
            .collect(),
    )
}

mem_tool_unscoped!(
    /// `kaeru_config` — resolved configuration and the clouds in reach.
    Config,
    "kaeru_config",
    "Show resolved configuration: vault path, the configured clouds and which is default, and \
     every cap (initiative not relevant).",
    NoArgs,
    { "type": "object", "properties": {} },
    |mem, _a| {
        let config = mem.blocking(|s| s.config().clone()).await;
        json!({
            "version": kaeru_core::version(),
            "vault_path": config.vault_path.display().to_string(),
            "clouds": clouds_json(mem),
            "active_window_size": config.active_window_size,
            "recent_episodes_cap": config.recent_episodes_cap,
            "awake_window_secs": config.awake_default_window_secs,
            "summary_children_cap": config.summary_view_children_cap,
            "body_excerpt_chars": config.body_excerpt_chars,
            "provenance_max_hops": config.provenance_max_hops,
            "default_max_hops": config.default_max_hops,
            "max_hops_cap": config.max_hops_cap,
        })
    }
);

mem_tool_unscoped!(
    /// `kaeru_clouds` — which clouds this memory can reach.
    Clouds,
    "kaeru_clouds",
    "List the clouds this memory can reach, with their endpoints and which one is default. Ask \
     this before any cloud verb in an unfamiliar setup: with more than one cloud configured, \
     `kaeru_share` / `kaeru_pull` / `kaeru_cloud_recall` and the initiative verbs require `cloud` \
     named explicitly, and nothing is routed to a default you did not choose.",
    NoArgs,
    { "type": "object", "properties": {} },
    |mem, _a| {
        let list = clouds_json(mem);
        if list.as_array().is_some_and(|a| a.is_empty()) {
            json!({
                "clouds": [],
                "hint": "no clouds configured — the host application builds the registry and \
                         hands it to `KaeruMemory::with_clouds`.",
            })
        } else {
            json!({ "clouds": list })
        }
    }
);

/// The configured clouds as `[{name, endpoint, default}]`. Shared by
/// `kaeru_config` and `kaeru_clouds`, which answer the same question at
/// different widths.
fn clouds_json(mem: &crate::KaeruMemory) -> serde_json::Value {
    let registry = mem.clouds();
    let default = registry.default_name();
    serde_json::Value::Array(
        registry
            .names()
            .into_iter()
            .map(|n| {
                json!({
                    "name": n,
                    "endpoint": registry.get(Some(n)).map(|c| c.base_url()).unwrap_or(""),
                    "default": Some(n) == default,
                })
            })
            .collect(),
    )
}
