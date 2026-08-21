//! Open-work reads — the loose ends re-entry must not drop.
//!
//! `awake` restores what was *touched*; these reads restore what is still
//! *owed*. Both a task past its due date and a claim still awaiting a verdict
//! live as a `status:` tag on an ordinary node, reachable only through
//! `tagged` — a verb an agent has to think of first. Nothing surfaced them on
//! re-entry, so in practice they were written and never revisited. These
//! primitives pull them into the re-entry bundle instead.
//!
//! "Open" is defined against the initiative's own status registry (see
//! [`board`](super::board)): the registry's **last** column is the terminal
//! one (`done` in the built-in default), everything before it is still open.
//! A task whose `status:` tag matches no column counts as open too — the same
//! fallback the board view uses, so drift surfaces rather than disappears.

use std::collections::BTreeMap;

use chrono::Utc;
use cozo::{DataValue, ScriptMutability};

use super::board::{BoardStatus, DEFAULT_STATUSES, effective_statuses};
use super::{NodeBrief, parse_brief};
use crate::errors::Result;
use crate::graph::NodeId;
use crate::graph::temporal::validity_seconds;
use crate::recall::truncate_excerpt;
use crate::store::Store;

/// A task that hasn't reached its board's terminal column, with the `due:`
/// date lifted out of the tag list and compared against today.
#[derive(Debug, Clone, PartialEq)]
pub struct OpenTask {
    pub id: NodeId,
    pub name: String,
    pub body_excerpt: Option<String>,
    /// The `status:` key the task currently carries — empty when it has none
    /// (legacy drift), which still counts as open.
    pub status: String,
    /// `due:` date as `YYYY-MM-DD`, when the task carries one.
    pub due: Option<String>,
    /// True when `due` is strictly before today (UTC). A task due *today* is
    /// not overdue yet — the day isn't over.
    pub overdue: bool,
    pub ts: Option<f64>,
}

/// Today as `YYYY-MM-DD` (UTC) — the boundary `overdue` is measured against.
/// Dates are stored as plain ISO strings, so a lexicographic compare is a
/// chronological one.
fn today_iso() -> String {
    Utc::now().format("%Y-%m-%d").to_string()
}

/// Every still-open task in scope, deadline-first: overdue ones (oldest first),
/// then the rest by due date, then undated tasks newest-first.
///
/// Scoped to the active initiative when one is set; cross-initiative otherwise
/// (and then judged against the built-in status vocabulary, since a registry
/// is per-initiative).
pub fn open_tasks(store: &Store) -> Result<Vec<OpenTask>> {
    // A status registry is per-initiative; with no initiative in scope there
    // is none to read, so the built-in vocabulary decides what "done" means.
    let statuses = match store.current_initiative() {
        Some(init) => effective_statuses(store, &init)?,
        None => DEFAULT_STATUSES
            .iter()
            .map(|(k, l)| BoardStatus {
                key: (*k).to_string(),
                label: (*l).to_string(),
            })
            .collect(),
    };
    // The registry's last column is the terminal one; `effective_statuses`
    // never returns an empty vocabulary, so this always names a real column.
    let terminal = statuses.last().map(|s| s.key.clone()).unwrap_or_default();
    let excerpt_chars = store.config().body_excerpt_chars;
    let today = today_iso();

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
            ?[id, name, body, tags, validity] :=
                *node_initiative{initiative, node_id: id}, initiative = $init,
                *node{id, type, name, body, tags, validity @ 'NOW'}, type = 'task'
            "#
        }
        None => {
            r#"
            ?[id, name, body, tags, validity] :=
                *node{id, type, name, body, tags, validity @ 'NOW'}, type = 'task'
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let mut tasks: Vec<OpenTask> = Vec::new();
    for row in &rows.rows {
        let tags: Vec<&str> = match row.get(3) {
            Some(DataValue::List(items)) => items.iter().filter_map(|x| x.get_str()).collect(),
            _ => Vec::new(),
        };
        let status = tags
            .iter()
            .find_map(|t| t.strip_prefix("status:"))
            .unwrap_or("")
            .to_string();
        // Terminal column only — an unknown status is drift, not completion,
        // so it fails this check and stays visible.
        if status == terminal {
            continue;
        }
        let due = tags
            .iter()
            .find_map(|t| t.strip_prefix("due:"))
            .map(String::from);
        let overdue = due.as_deref().is_some_and(|d| d < today.as_str());
        tasks.push(OpenTask {
            id: row
                .first()
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_default(),
            name: row
                .get(1)
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_default(),
            body_excerpt: row
                .get(2)
                .and_then(|v| v.get_str())
                .map(|s| truncate_excerpt(s, excerpt_chars)),
            status,
            due,
            overdue,
            ts: validity_seconds(row.get(4)),
        });
    }

    // Dated tasks first, ascending — which puts the most overdue at the top —
    // then undated ones newest-first.
    tasks.sort_by(|a, b| match (&a.due, &b.due) {
        (Some(x), Some(y)) => x.cmp(y),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => {
            b.ts.unwrap_or(0.0)
                .partial_cmp(&a.ts.unwrap_or(0.0))
                .unwrap_or(std::cmp::Ordering::Equal)
        }
    });
    Ok(tasks)
}

/// Hypothesis nodes still tagged `status:open` — claims written but never
/// confirmed or refuted. Newest-first, scoped to the active initiative.
///
/// Deliberately narrower than `tagged "status:open"`: tasks carry the same tag
/// and have their own section, so this filters to `hypothesis` and returns the
/// claims alone.
pub fn open_claims(store: &Store) -> Result<Vec<NodeBrief>> {
    let excerpt_chars = store.config().body_excerpt_chars;
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
            ?[id, type, name, body, validity] :=
                *node_initiative{initiative, node_id: id}, initiative = $init,
                *node{id, type, name, body, tags, validity @ 'NOW'},
                type = 'hypothesis',
                !is_null(tags),
                is_in('status:open', tags)
            :order validity
            "#
        }
        None => {
            r#"
            ?[id, type, name, body, validity] :=
                *node{id, type, name, body, tags, validity @ 'NOW'},
                type = 'hypothesis',
                !is_null(tags),
                is_in('status:open', tags)
            :order validity
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    Ok(rows
        .rows
        .iter()
        .map(|r| parse_brief(r, excerpt_chars))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::{open_claims, open_tasks};
    use crate::store::Store;
    use crate::{complete_task, formulate_hypothesis, write_task};

    fn store_t() -> Store {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        store
    }

    #[test]
    fn a_completed_task_leaves_the_open_list() {
        let store = store_t();
        let id = write_task(&store, "ship the thing", None).expect("task");
        assert_eq!(open_tasks(&store).expect("read").len(), 1);
        complete_task(&store, &id).expect("done");
        assert!(
            open_tasks(&store).expect("read").is_empty(),
            "a done task is not open work any more"
        );
    }

    #[test]
    fn a_past_due_date_reads_as_overdue_and_sorts_first() {
        let store = store_t();
        write_task(&store, "no deadline", None).expect("task");
        write_task(&store, "far future", Some("2999-01-01")).expect("task");
        write_task(&store, "long past", Some("2000-01-01")).expect("task");

        let tasks = open_tasks(&store).expect("read");
        assert_eq!(tasks.len(), 3);
        assert!(tasks[0].overdue, "the past-due task leads: {tasks:?}");
        assert_eq!(tasks[0].due.as_deref(), Some("2000-01-01"));
        assert!(!tasks[1].overdue, "a future deadline is not overdue");
        assert_eq!(tasks[2].due, None, "undated tasks sink to the bottom");
    }

    #[test]
    fn open_claims_are_hypotheses_only_not_every_status_open_node() {
        let store = store_t();
        write_task(&store, "a task also carries status:open", None).expect("task");
        formulate_hypothesis(&store, "the-claim", "caching wins here").expect("claim");

        let claims = open_claims(&store).expect("read");
        assert_eq!(claims.len(), 1, "the task must not leak in: {claims:?}");
        assert_eq!(claims[0].name, "the-claim");
    }
}
