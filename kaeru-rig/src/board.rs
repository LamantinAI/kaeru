//! Task-board tools: `board` (read columns), `set_status` (move a task),
//! `board_status` (customize the column registry). Initiative-scoped — each
//! defaults to the memory's initiative, overridable per call.
//!
//! Store-only (no network), but they need the initiative *name* to reach the
//! board, so they go through the mem-aware `mem_tool_cloud!` shape and do their
//! work on `mem.blocking(...)` — the async body just awaits the blocking store
//! span, no `.await` on a client.

use serde::Deserialize;
use serde_json::{Value, json};

use crate::{KaeruMemory, mem_tool_cloud, resolve};

/// The initiative a board call targets: an explicit arg, else the memory's own.
fn target_initiative(mem: &KaeruMemory, arg: &Option<String>) -> Option<String> {
    arg.clone().or_else(|| mem.initiative().map(String::from))
}

// ── board (read) ─────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct BoardArgs {
    #[serde(default)]
    pub initiative: Option<String>,
    /// Unix seconds to rewind the board to. Omit for the board as it stands now.
    #[serde(default)]
    pub when: Option<f64>,
}

async fn do_board(mem: &KaeruMemory, a: BoardArgs) -> Value {
    let Some(init) = target_initiative(mem, &a.initiative) else {
        return json!({ "error": "no initiative — scope the memory or pass `initiative`" });
    };
    let at = a.when;
    match mem
        .blocking(move |s| kaeru_core::board_view_at(s, &init, at))
        .await
    {
        Ok(view) => json!({
            "initiative": view.initiative,
            "columns": view.columns.iter().map(|c| json!({
                "key": c.key,
                "label": c.label,
                "tasks": c.tasks.iter().map(|t| json!({
                    "id": t.id, "name": t.name, "excerpt": t.body_excerpt,
                    "due": t.due, "ts": t.ts,
                })).collect::<Vec<_>>(),
            })).collect::<Vec<_>>(),
        }),
        Err(e) => json!({ "error": e.to_string() }),
    }
}

// ── set_status (move a task) ─────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SetStatusArgs {
    pub task: String,
    pub status: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

async fn do_set_status(mem: &KaeruMemory, a: SetStatusArgs) -> Value {
    let Some(init) = target_initiative(mem, &a.initiative) else {
        return json!({ "error": "no initiative — scope the memory or pass `initiative`" });
    };
    let (task, status, init2) = (a.task.clone(), a.status.clone(), init.clone());
    let r = mem
        .blocking_in(Some(init), move |s| {
            let id = resolve(s, &task);
            kaeru_core::set_status(s, &init2, &id, &status)
        })
        .await;
    match r {
        Ok(()) => json!({ "moved": true, "task": a.task, "status": a.status }),
        Err(e) => json!({ "moved": false, "error": e.to_string() }),
    }
}

// ── board_status (customize columns) ─────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct BoardStatusArgs {
    /// `add` / `remove` / `relabel` / `reorder`.
    pub action: String,
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub order: Option<Vec<String>>,
    #[serde(default)]
    pub initiative: Option<String>,
}

async fn do_board_status(mem: &KaeruMemory, a: BoardStatusArgs) -> Value {
    let Some(init) = target_initiative(mem, &a.initiative) else {
        return json!({ "error": "no initiative — scope the memory or pass `initiative`" });
    };
    let (action, key, label, order) = (
        a.action.clone(),
        a.key.clone(),
        a.label.clone(),
        a.order.clone(),
    );

    mem.blocking(move |s| {
        let statuses = match action.as_str() {
            "add" => match &key {
                Some(k) => kaeru_core::add_status(s, &init, k, label.as_deref().unwrap_or(k)),
                None => return json!({ "error": "`key` is required for add" }),
            },
            "remove" => match &key {
                Some(k) => kaeru_core::remove_status(s, &init, k),
                None => return json!({ "error": "`key` is required for remove" }),
            },
            "relabel" => match (&key, &label) {
                (Some(k), Some(l)) => kaeru_core::relabel_status(s, &init, k, l),
                _ => return json!({ "error": "`key` and `label` are required for relabel" }),
            },
            "reorder" => match &order {
                Some(o) => kaeru_core::reorder_statuses(s, &init, o),
                None => return json!({ "error": "`order` is required for reorder" }),
            },
            other => {
                return json!({ "error": format!("unknown action `{other}` — add/remove/relabel/reorder") });
            }
        };
        match statuses {
            Ok(st) => json!({
                "statuses": st.iter().map(|s| json!({ "key": s.key, "label": s.label })).collect::<Vec<_>>()
            }),
            Err(e) => json!({ "error": e.to_string() }),
        }
    })
    .await
}

// ── tool definitions ─────────────────────────────────────────────────────────

mem_tool_cloud!(
    /// `kaeru_board` — read the initiative's task board.
    Board,
    "kaeru_board",
    "Show the task board for an initiative: status columns (from its registry, in order, empty \
     ones included) with the tasks bucketed into them. Defaults to the memory's initiative. Pass \
     `when` (unix seconds) to rewind the whole board — columns and cards — to a past moment.",
    BoardArgs,
    { "type": "object", "properties": {
        "initiative": { "type": "string", "description": "initiative (default: the memory's own)" },
        "when": { "type": "number", "description": "unix seconds to rewind the board to (omit for now)" }
    } },
    |mem, a| do_board(mem, a).await
);

mem_tool_cloud!(
    /// `kaeru_set_status` — move a task to a board column.
    SetStatus,
    "kaeru_set_status",
    "Move a task to a board column — sets its status, strictly validated against the initiative's \
     board registry (unknown status is refused). The general form of `kaeru_done`.",
    SetStatusArgs,
    { "type": "object", "properties": {
        "task": { "type": "string", "description": "task node name or id" },
        "status": { "type": "string", "description": "target column key (must exist on the board)" },
        "initiative": { "type": "string", "description": "initiative (default: the memory's own)" }
    }, "required": ["task", "status"] },
    |mem, a| do_set_status(mem, a).await
);

mem_tool_cloud!(
    /// `kaeru_board_status` — customize the board's columns.
    BoardStatusEdit,
    "kaeru_board_status",
    "Customize an initiative's board columns: action add (key, label?) / remove (key) / relabel \
     (key, label) / reorder (order = all keys permuted). The board is created from the defaults \
     [open, in-progress, done] on first edit.",
    BoardStatusArgs,
    { "type": "object", "properties": {
        "action": { "type": "string", "description": "add | remove | relabel | reorder" },
        "key": { "type": "string", "description": "status key (add/remove/relabel)" },
        "label": { "type": "string", "description": "human label (relabel; optional for add)" },
        "order": { "type": "array", "items": { "type": "string" }, "description": "all keys, permuted (reorder)" },
        "initiative": { "type": "string", "description": "initiative (default: the memory's own)" }
    }, "required": ["action"] },
    |mem, a| do_board_status(mem, a).await
);
