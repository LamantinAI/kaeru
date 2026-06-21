//! Session re-entry, initiative management, diagnostics, and snapshot export.

use kaeru_core::{
    awake, delete_initiative, export_vault, lint, list_initiatives, overview, pin, recent_episodes,
    rename_initiative, unpin,
};
use serde::Deserialize;
use serde_json::json;

use crate::lookup::NoArgs;
use crate::{briefs_by_ids, mem_tool, resolve};

mem_tool!(
    /// `kaeru_awake` — load the re-entry context for the active initiative.
    Awake,
    "kaeru_awake",
    "Load your working memory for this project: what was pinned, what happened recently, and \
     what's still under review. Call this first when picking up a session to recover context.",
    NoArgs,
    { "type": "object", "properties": {} },
    |store, _args| match awake(store) {
        Ok(ctx) => json!({
            "initiative": ctx.initiative,
            "all_initiatives": ctx.all_initiatives,
            "pinned": briefs_by_ids(store, &ctx.pinned),
            "recent": briefs_by_ids(store, &ctx.recent),
            "under_review": briefs_by_ids(store, &ctx.under_review),
        }),
        Err(e) => json!({ "error": e.to_string() }),
    }
);

mem_tool!(
    /// `kaeru_overview` — a terminal-readable map of the initiative's subgraph.
    Overview,
    "kaeru_overview",
    "Get a readable map of what this project's memory knows — the subgraph overview. Pairs with \
     `kaeru_awake` (process state) to answer \"what does this project know\".",
    NoArgs,
    { "type": "object", "properties": {} },
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
    /// Look-back window in seconds. Defaults to the configured awake window.
    #[serde(default)]
    pub window_seconds: Option<u64>,
}

mem_tool!(
    /// `kaeru_recent` — episodes from the recent past.
    Recent,
    "kaeru_recent",
    "List recent episodes — what happened lately in this project. `window_seconds` sets the \
     look-back (default: the configured awake window, ~24h).",
    RecentArgs,
    { "type": "object", "properties": {
        "window_seconds": { "type": "integer", "description": "look-back window in seconds" }
    } },
    |store, args| {
        let window = args
            .window_seconds
            .unwrap_or_else(|| store.config().awake_default_window_secs);
        match recent_episodes(store, window) {
            Ok(ids) => json!({ "recent": briefs_by_ids(store, &ids) }),
            Err(e) => json!({ "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct PinArgs {
    pub name_or_id: String,
    pub reason: String,
}

mem_tool!(
    /// `kaeru_pin` — pin a node into the active window.
    Pin,
    "kaeru_pin",
    "Pin a memory so it stays in your active working window across the session, with a reason.",
    PinArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "node name or id" },
        "reason": { "type": "string", "description": "why it's pinned" }
    }, "required": ["name_or_id", "reason"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
        match pin(store, &id, &args.reason) {
            Ok(()) => json!({ "pinned": true, "id": id }),
            Err(e) => json!({ "pinned": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct UnpinArgs {
    pub name_or_id: String,
}

mem_tool!(
    /// `kaeru_unpin` — remove a node from the active window.
    Unpin,
    "kaeru_unpin",
    "Unpin a memory — remove it from your active working window.",
    UnpinArgs,
    { "type": "object", "properties": {
        "name_or_id": { "type": "string", "description": "node name or id" }
    }, "required": ["name_or_id"] },
    |store, args| {
        let id = resolve(store, &args.name_or_id);
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
}

mem_tool!(
    /// `kaeru_rename_initiative` — rename a project (moves all its nodes/edges).
    RenameInitiative,
    "kaeru_rename_initiative",
    "Rename an initiative — moves all its nodes, edges, and sharing policy to the new name (fails \
     if the new name already exists). Local only.",
    RenameInitiativeArgs,
    { "type": "object", "properties": {
        "old": { "type": "string", "description": "current initiative name" },
        "new": { "type": "string", "description": "new initiative name (must not exist)" }
    }, "required": ["old", "new"] },
    |store, args| match rename_initiative(store, &args.old, &args.new) {
        Ok(stats) => json!({ "renamed": true, "nodes": stats.nodes, "edges": stats.edges }),
        Err(e) => json!({ "renamed": false, "error": e.to_string() }),
    }
);

#[derive(Debug, Deserialize)]
pub struct DeleteInitiativeArgs {
    pub name: String,
}

mem_tool!(
    /// `kaeru_delete_initiative` — drop a project's scoping (forgets exclusive nodes).
    DeleteInitiative,
    "kaeru_delete_initiative",
    "Delete an initiative — drops its scoping and forgets the nodes exclusive to it (bi-temporal: \
     recoverable via `kaeru_at` at a past time). Nodes shared with other initiatives only lose \
     this membership. Local only.",
    DeleteInitiativeArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "initiative to delete" }
    }, "required": ["name"] },
    |store, args| match delete_initiative(store, &args.name) {
        Ok(stats) => json!({ "deleted": true, "unscoped": stats.unscoped, "forgotten": stats.forgotten }),
        Err(e) => json!({ "deleted": false, "error": e.to_string() }),
    }
);

mem_tool!(
    /// `kaeru_lint` — surface orphans and unresolved reviews.
    Lint,
    "kaeru_lint",
    "Check the memory for hygiene issues: orphan nodes (no edges) and unresolved review flags. \
     Use it to find loose ends worth tidying.",
    NoArgs,
    { "type": "object", "properties": {} },
    |store, _args| match lint(store) {
        Ok(report) => json!({
            "orphans": report.orphans,
            "unresolved_reviews": report.unresolved_reviews,
        }),
        Err(e) => json!({ "error": e.to_string() }),
    }
);

#[derive(Debug, Deserialize)]
pub struct ExportArgs {
    /// Output directory for the markdown snapshot.
    pub path: String,
}

mem_tool!(
    /// `kaeru_export` — write an Obsidian-friendly markdown snapshot.
    Export,
    "kaeru_export",
    "Export the current initiative to an Obsidian-friendly markdown vault on disk (README / INDEX \
     / LOG plus node pages). Use when a human wants to read the memory offline.",
    ExportArgs,
    { "type": "object", "properties": {
        "path": { "type": "string", "description": "output directory for the snapshot" }
    }, "required": ["path"] },
    |store, args| match export_vault(store, &args.path) {
        Ok(summary) => json!({
            "exported": true,
            "nodes": summary.nodes_exported,
            "root": summary.root.to_string_lossy(),
        }),
        Err(e) => json!({ "exported": false, "error": e.to_string() }),
    }
);
