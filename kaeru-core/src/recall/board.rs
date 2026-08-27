//! Task-board reads — the status registry + a bucketed board view.
//!
//! A board is a per-initiative status *registry*: one `Board` node whose
//! `properties.statuses` is the ordered `{key, label}` vocabulary. Task nodes
//! carry `status:<key>`; a board view buckets the initiative's tasks into the
//! registry's columns, in order, empty columns included.
//!
//! Until an initiative customizes its board (via `board_status`), no `Board`
//! node exists and the *effective* statuses are the built-in defaults — so a
//! plain read never writes. `set_status` validates against these same
//! effective statuses.

use std::collections::BTreeMap;

use cozo::{DataValue, JsonData, ScriptMutability};

use super::truncate_excerpt;
use crate::errors::Result;
use crate::graph::NodeId;
use crate::graph::temporal::validity_seconds;
use crate::store::Store;

/// The built-in status vocabulary an initiative starts with, before it
/// customizes its board. `open` first (matches `write_task`'s default), `done`
/// last.
pub const DEFAULT_STATUSES: [(&str, &str); 3] = [
    ("open", "Open"),
    ("in-progress", "In Progress"),
    ("done", "Done"),
];

/// One column of the registry: a stable `key` (the `status:<key>` tag) plus a
/// human `label` (freely re-editable without touching task tags).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardStatus {
    pub key: String,
    pub label: String,
}

/// A task as it appears on the board — lighter than `NodeBrief`, with the
/// `due:` date surfaced for card display and the assertion `ts` for the
/// time-lapse scrubber.
#[derive(Debug, Clone, PartialEq)]
pub struct BoardTask {
    pub id: NodeId,
    pub name: String,
    pub body_excerpt: Option<String>,
    pub due: Option<String>,
    pub ts: Option<f64>,
    /// `local` or `shared`. Carried because a consumer that sends a board
    /// anywhere has to decide whether each card may go, and asking per card
    /// would be one query each — the row is already being read here.
    pub visibility: String,
}

/// One column: its status plus the tasks bucketed into it.
#[derive(Debug, Clone, PartialEq)]
pub struct BoardColumn {
    pub key: String,
    pub label: String,
    pub tasks: Vec<BoardTask>,
}

/// A full board view for an initiative — columns in registry order.
#[derive(Debug, Clone, PartialEq)]
pub struct BoardView {
    pub initiative: String,
    pub columns: Vec<BoardColumn>,
}

/// The bi-temporal read modifier: `'NOW'` for the present, or a literal unix
/// timestamp to read the graph as it stood at that moment. Every board read
/// funnels its `@ …` through here so the whole board — columns *and* cards —
/// can be rewound together.
fn at_expr(at: Option<f64>) -> String {
    match at {
        Some(secs) => format!("{secs}"),
        None => "'NOW'".to_string(),
    }
}

/// The `Board` node's id for `initiative` (as of `at`, or NOW), or `None` when
/// the initiative hasn't customized its board yet. Datalog guarantees at most
/// one board per initiative in practice (created find-or-create); the first is
/// taken.
pub(crate) fn board_node_id(
    store: &Store,
    initiative: &str,
    at: Option<f64>,
) -> Result<Option<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    let script = format!(
        r#"
        ?[id] := *node_initiative{{initiative, node_id: id}},
                 initiative = $init,
                 *node{{id, type @ {at}}}, type = 'board'
    "#,
        at = at_expr(at)
    );
    let rows = store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Immutable)?;
    Ok(rows
        .rows
        .first()
        .and_then(|r| r.first())
        .and_then(|v| v.get_str())
        .map(String::from))
}

/// Reads a board node's `properties.statuses` into an ordered `Vec` (as of
/// `at`, or NOW). Malformed / missing entries are skipped defensively.
pub(crate) fn read_board_statuses(
    store: &Store,
    board_id: &NodeId,
    at: Option<f64>,
) -> Result<Vec<BoardStatus>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("bid".to_string(), DataValue::Str(board_id.clone().into()));
    let script = format!(
        r#"
        ?[properties] := *node{{id, properties @ {at}}}, id = $bid
    "#,
        at = at_expr(at)
    );
    let rows = store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Immutable)?;
    let Some(DataValue::Json(JsonData(v))) = rows.rows.first().and_then(|r| r.first()) else {
        return Ok(Vec::new());
    };
    let statuses = v
        .get("statuses")
        .and_then(|s| s.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|e| {
                    let key = e.get("key").and_then(|x| x.as_str())?.to_string();
                    let label = e
                        .get("label")
                        .and_then(|x| x.as_str())
                        .map(String::from)
                        .unwrap_or_else(|| key.clone());
                    Some(BoardStatus { key, label })
                })
                .collect()
        })
        .unwrap_or_default();
    Ok(statuses)
}

/// The status vocabulary in force for `initiative` **now**: the customized
/// board's registry if one exists, otherwise the built-in [`DEFAULT_STATUSES`].
/// Never writes — the read side and `set_status` both validate against this.
pub fn effective_statuses(store: &Store, initiative: &str) -> Result<Vec<BoardStatus>> {
    effective_statuses_at(store, initiative, None)
}

/// [`effective_statuses`] as of `at` (unix seconds), or NOW when `None` — so a
/// rewound board shows the columns as they stood then, not today's.
pub fn effective_statuses_at(
    store: &Store,
    initiative: &str,
    at: Option<f64>,
) -> Result<Vec<BoardStatus>> {
    if let Some(board_id) = board_node_id(store, initiative, at)? {
        let statuses = read_board_statuses(store, &board_id, at)?;
        if !statuses.is_empty() {
            return Ok(statuses);
        }
    }
    Ok(DEFAULT_STATUSES
        .iter()
        .map(|(k, l)| BoardStatus {
            key: (*k).to_string(),
            label: (*l).to_string(),
        })
        .collect())
}

/// The board view for `initiative`: every effective column in order (empty
/// ones included), with the initiative's tasks bucketed by their `status:`
/// tag. A task whose status isn't a known column (legacy / drift) falls into
/// the first column so it never disappears.
pub fn board_view(store: &Store, initiative: &str) -> Result<BoardView> {
    board_view_at(store, initiative, None)
}

/// [`board_view`] as of `at` (unix seconds), or NOW when `None` — the board as
/// it stood at that moment: the columns of the day *and* each task in the
/// column it was in then.
///
/// This is free of any extra bookkeeping: a card's column is a bi-temporal tag
/// on the task, so rewinding the substrate rewinds the board. It's what lets a
/// UI scrub a sprint's history rather than only show its end state.
pub fn board_view_at(store: &Store, initiative: &str, at: Option<f64>) -> Result<BoardView> {
    let statuses = effective_statuses_at(store, initiative, at)?;
    let excerpt_chars = store.config().body_excerpt_chars;

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    let script = format!(
        r#"
        ?[id, name, body, tags, visibility, validity] :=
            *node_initiative{{initiative, node_id: id}}, initiative = $init,
            *node{{id, type, name, body, tags, visibility, validity @ {at}}}, type = 'task'
    "#,
        at = at_expr(at)
    );
    let rows = store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Immutable)?;

    // Column index by key; the first column is the fallback bucket.
    let mut columns: Vec<BoardColumn> = statuses
        .iter()
        .map(|s| BoardColumn {
            key: s.key.clone(),
            label: s.label.clone(),
            tasks: Vec::new(),
        })
        .collect();
    let index: BTreeMap<String, usize> = statuses
        .iter()
        .enumerate()
        .map(|(i, s)| (s.key.clone(), i))
        .collect();

    for row in &rows.rows {
        let id = row
            .first()
            .and_then(|v| v.get_str())
            .map(String::from)
            .unwrap_or_default();
        let name = row
            .get(1)
            .and_then(|v| v.get_str())
            .map(String::from)
            .unwrap_or_default();
        let body_excerpt = row
            .get(2)
            .and_then(|v| v.get_str())
            .map(|s| truncate_excerpt(s, excerpt_chars));
        let tags: Vec<&str> = match row.get(3) {
            Some(DataValue::List(items)) => items.iter().filter_map(|x| x.get_str()).collect(),
            _ => Vec::new(),
        };
        let status = tags
            .iter()
            .find_map(|t| t.strip_prefix("status:"))
            .unwrap_or("");
        let due = tags
            .iter()
            .find_map(|t| t.strip_prefix("due:"))
            .map(String::from);
        let visibility = row
            .get(4)
            .and_then(|v| v.get_str())
            .map(String::from)
            .unwrap_or_else(|| "local".to_string());
        let ts = validity_seconds(row.get(5));

        let col = index.get(status).copied().unwrap_or(0);
        if let Some(c) = columns.get_mut(col) {
            c.tasks.push(BoardTask {
                id,
                name,
                body_excerpt,
                due,
                ts,
                visibility,
            });
        }
    }

    Ok(BoardView {
        initiative: initiative.to_string(),
        columns,
    })
}
