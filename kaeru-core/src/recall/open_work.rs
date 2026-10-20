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
//! [`board`](super::board)): the registry marks terminal columns explicitly (`done` in the built-in
//! default); the other columns remain open.
//! A task whose `status:` tag matches no column counts as open too — the same
//! fallback the board view uses, so drift surfaces rather than disappears.
//!
//! [`due_reminders`] is the third of these and the odd one out: it restores
//! what is owed *from now on* rather than what was already owed. See its own
//! doc for the tag contract.

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
                terminal: *k == "done",
            })
            .collect(),
    };
    // The registry's last column is the terminal one; `effective_statuses`
    // never returns an empty vocabulary, so this always names a real column.
    let terminal: Vec<_> = statuses
        .iter()
        .filter(|s| s.terminal)
        .map(|s| s.key.as_str())
        .collect();
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
        if terminal.contains(&status.as_str()) {
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

/// A reminder whose moment has arrived: a node that named a future date and
/// has reached it, with its insistence window still open.
#[derive(Debug, Clone, PartialEq)]
pub struct DueReminder {
    pub id: NodeId,
    pub name: String,
    pub body_excerpt: Option<String>,
    /// The `after:` date the author named — when this became relevant.
    pub after: String,
    /// Days of insistence the author asked for (`for:<N>d`).
    pub window_days: i64,
    /// `seen:` date, once the reminder has been delivered at least once.
    /// `None` means this is its first appearance.
    pub seen: Option<String>,
    pub ts: Option<f64>,
}

impl DueReminder {
    /// True the first time a reminder surfaces — the moment its window should
    /// start, and the only moment a `seen:` stamp should be written.
    pub fn is_first_sighting(&self) -> bool {
        self.seen.is_none()
    }
}

/// Reminders that have come due — the capture that named a future moment and
/// has reached it.
///
/// ## Why this exists
///
/// Everything a session captures is either visible from the moment it is
/// written or effectively invisible; there was no way to say "not yet
/// relevant". `task --due` looks like the exception and is not — an open task
/// appears in `awake` from creation and the date only affects its order — so a
/// reminder set three months out clutters re-entry for three months, which is
/// why nobody sets one (#90).
///
/// ## The tag contract
///
/// - `after:YYYY-MM-DD` — not relevant until this date.
/// - `for:<N>d` — how many days to keep insisting once it surfaces.
/// - `seen:YYYY-MM-DD` — written once, the first time it is delivered.
///
/// A reminder is due when `after:` has arrived **and** its window is still
/// open — either it has never been seen, or fewer than `for:` days have passed
/// since it was.
///
/// ## Why the window runs from first sight, not from the date
///
/// A window counted from `after:` can expire while nobody is looking, which
/// loses exactly the reminder that mattered — a six-day certificate cycle
/// against a fortnight away from the keyboard. Counted from `seen:`, a
/// reminder waits indefinitely for someone to show up and then gets its full
/// window. It also answers "what if several sessions need it": the window is
/// days, not sessions, so every session inside it sees the reminder. `seen:`
/// does not mean "shown, done" — it means the clock started.
///
/// ## Why there is no default window
///
/// `for:` is required, and a reminder without it is not due. The precedent is
/// `link`'s weight: across 6,003 calls of real work, the optional `strong=true`
/// was passed in 0 of 1,262 links, so every edge sat on the default and every
/// weighted path ranked on nothing. A default here would guarantee that nobody
/// ever chooses the value, and "re-measure this before quoting it" and "the
/// certificate expires" want completely different windows.
///
/// ## Not a layer move
///
/// Deliberately a read, computed from tags exactly as [`open_tasks`] computes
/// `overdue`. Promotion to `hot` was the obvious mechanism and the wrong one:
/// every layer but `core` is capped at `active_window_size` in the re-entry
/// view, so a promoted reminder can be silently truncated out of sight while
/// its author believes it fired. Being a read also means this works whether or
/// not a hygiene pass ever runs.
///
/// Ordered by `after:` ascending — the longest-standing moment first.
pub fn due_reminders(store: &Store) -> Result<Vec<DueReminder>> {
    let excerpt_chars = store.config().body_excerpt_chars;
    let today = today_iso();
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
            ?[id, name, body, tags, validity] :=
                *node_initiative{initiative, node_id: id}, initiative = $init,
                *node{id, type, name, body, tags, validity @ 'NOW'},
                type != 'audit_event',
                !is_null(tags)
            "#
        }
        None => {
            r#"
            ?[id, name, body, tags, validity] :=
                *node{id, type, name, body, tags, validity @ 'NOW'},
                type != 'audit_event',
                !is_null(tags)
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let mut out: Vec<DueReminder> = Vec::new();
    for row in &rows.rows {
        let tags: Vec<&str> = match row.get(3) {
            Some(DataValue::List(items)) => items.iter().filter_map(|x| x.get_str()).collect(),
            _ => Vec::new(),
        };
        let Some((after, window_days, seen)) = reminder_schedule(&tags, &today) else {
            continue;
        };
        out.push(DueReminder {
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
            after: after.to_string(),
            window_days,
            seen: seen.map(String::from),
            ts: validity_seconds(row.get(4)),
        });
    }

    out.sort_by(|a, b| a.after.cmp(&b.after));
    Ok(out)
}

/// Derive delivery eligibility without reading a record's content. API
/// adapters call this only after admitting its identity and visibility.
pub(crate) fn reminder_schedule<'a>(
    tags: &[&'a str],
    today: &str,
) -> Option<(&'a str, i64, Option<&'a str>)> {
    let after = tags.iter().find_map(|t| t.strip_prefix("after:"))?;
    if after > today {
        return None;
    }
    let window_days = tags.iter().find_map(|t| parse_window(t))?;
    let seen = tags.iter().find_map(|t| t.strip_prefix("seen:"));
    if seen.is_some_and(|s| !window_open(s, window_days, today)) {
        return None;
    }
    Some((after, window_days, seen))
}

/// `for:<N>d` → N. Also accepts a bare `for:<N>`, since the `d` is the only
/// unit there is and an agent dropping it means the same thing.
fn parse_window(tag: &str) -> Option<i64> {
    let raw = tag.strip_prefix("for:")?;
    let digits = raw.strip_suffix('d').unwrap_or(raw);
    match digits.parse::<i64>() {
        Ok(n) if n > 0 => Some(n),
        _ => None,
    }
}

/// Whether a reminder first seen on `seen_on` is still inside its window.
///
/// Inclusive of the last day: a one-day window seen today is still due today.
/// A `seen:` that will not parse is treated as an open window rather than a
/// closed one — a corrupt stamp should make a reminder noisy, never silent.
fn window_open(seen_on: &str, window_days: i64, today: &str) -> bool {
    let (Ok(seen), Ok(now)) = (
        chrono::NaiveDate::parse_from_str(seen_on, "%Y-%m-%d"),
        chrono::NaiveDate::parse_from_str(today, "%Y-%m-%d"),
    ) else {
        return true;
    };
    (now - seen).num_days() < window_days
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
    use std::collections::BTreeMap;

    use chrono::Utc;
    use cozo::DataValue;

    use super::{due_reminders, open_claims, open_tasks, parse_window, window_open};
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

    // ---- reminders (#90) ------------------------------------------------

    /// Writes a node carrying `tags` verbatim, so a test can state the exact
    /// tag contract under test rather than going through a capture verb.
    fn tagged_node(store: &Store, name: &str, tags: &[&str]) -> crate::graph::NodeId {
        use cozo::ScriptMutability;

        let id = crate::graph::new_node_id();
        let now = crate::mutate::now_validity_seconds();
        let tag_list = tags
            .iter()
            .map(|t| format!("'{t}'"))
            .collect::<Vec<_>>()
            .join(", ");
        let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
        params.insert("id".to_string(), DataValue::Str(id.clone().into()));
        params.insert("name".to_string(), DataValue::Str(name.into()));
        let script = format!(
            r#"
            ?[id, validity, type, tier, name, body, tags, initiatives, properties, visibility, layer] <-
                [[$id, [{now}.0, true], 'episode', 'operational', $name, 'body', [{tag_list}],
                  null, null, 'local', 'warm']]
            :put node {{id, validity => type, tier, name, body, tags, initiatives, properties, visibility, layer}}
            "#
        );
        store
            .db_ref()
            .run_script(&script, params, ScriptMutability::Mutable)
            .expect("write");
        crate::attach_node(store, &id, "t").expect("attach");
        id
    }

    /// `YYYY-MM-DD`, `days` from today — negative for the past.
    fn day(days: i64) -> String {
        (Utc::now().date_naive() + chrono::Duration::days(days))
            .format("%Y-%m-%d")
            .to_string()
    }

    /// The whole point: a capture can name a future moment and stay out of the
    /// way until it arrives. Before this there was no way to say "not yet
    /// relevant" — everything was visible from the moment it was written or
    /// effectively invisible (#90).
    #[test]
    fn a_reminder_is_silent_before_its_date_and_due_after() {
        let store = store_t();
        tagged_node(
            &store,
            "not-yet",
            &[&format!("after:{}", day(30)), "for:7d"],
        );
        assert!(
            due_reminders(&store).expect("read").is_empty(),
            "a month out is not today's business"
        );

        tagged_node(
            &store,
            "now-it-matters",
            &[&format!("after:{}", day(-1)), "for:7d"],
        );
        let due = due_reminders(&store).expect("read");
        assert_eq!(due.len(), 1, "{due:?}");
        assert_eq!(due[0].name, "now-it-matters");
        assert!(due[0].is_first_sighting(), "never delivered yet");
    }

    /// Its own date counts: a reminder for today is today's business.
    #[test]
    fn a_reminder_dated_today_is_due_today() {
        let store = store_t();
        tagged_node(&store, "today", &[&format!("after:{}", day(0)), "for:3d"]);
        assert_eq!(due_reminders(&store).expect("read").len(), 1);
    }

    /// The window runs from first sight, not from the date — so a fortnight
    /// away from the keyboard cannot eat the six-day certificate reminder.
    #[test]
    fn an_unseen_reminder_waits_however_long_it_takes() {
        let store = store_t();
        tagged_node(
            &store,
            "waited-a-year",
            &[&format!("after:{}", day(-365)), "for:2d"],
        );
        let due = due_reminders(&store).expect("read");
        assert_eq!(due.len(), 1, "still waiting to be seen: {due:?}");
    }

    /// Once seen, it keeps insisting for the window the author asked for —
    /// which answers "what if several sessions need it": the window is days,
    /// not sessions.
    #[test]
    fn a_seen_reminder_stays_due_until_its_window_closes() {
        let store = store_t();
        tagged_node(
            &store,
            "still-insisting",
            &[
                &format!("after:{}", day(-10)),
                "for:7d",
                &format!("seen:{}", day(-3)),
            ],
        );
        assert_eq!(due_reminders(&store).expect("read").len(), 1);
    }

    /// And goes quiet when it closes — no verb, no hygiene rule, it simply
    /// stops matching. One permanent line would devalue the real debts beside
    /// it, which is the disease this cures.
    #[test]
    fn a_reminder_goes_quiet_once_its_window_closes() {
        let store = store_t();
        tagged_node(
            &store,
            "had-its-turn",
            &[
                &format!("after:{}", day(-30)),
                "for:7d",
                &format!("seen:{}", day(-8)),
            ],
        );
        assert!(due_reminders(&store).expect("read").is_empty());
    }

    /// `for:` is required. A date with no window is an incomplete capture, not
    /// one to be defaulted — the `link` weight lesson: an optional value with a
    /// fallback was passed in 0 of 1,262 real calls.
    #[test]
    fn a_date_without_a_window_is_not_a_reminder() {
        let store = store_t();
        tagged_node(&store, "no-window", &[&format!("after:{}", day(-1))]);
        assert!(due_reminders(&store).expect("read").is_empty());
    }

    /// Any node type, not only tasks — a reference about an expiry, an idea to
    /// revisit, a warning. That is the argument for a tag over a task field.
    #[test]
    fn reminders_are_ordered_oldest_moment_first() {
        let store = store_t();
        tagged_node(&store, "recent", &[&format!("after:{}", day(-1)), "for:9d"]);
        tagged_node(
            &store,
            "ancient",
            &[&format!("after:{}", day(-20)), "for:9d"],
        );
        let due = due_reminders(&store).expect("read");
        assert_eq!(
            due.iter().map(|r| r.name.as_str()).collect::<Vec<_>>(),
            vec!["ancient", "recent"],
            "the longest-standing moment leads"
        );
    }

    /// A corrupt stamp makes a reminder noisy, never silent — the safe
    /// direction for something whose whole job is to arrive.
    #[test]
    fn an_unparseable_seen_stamp_leaves_the_window_open() {
        assert!(window_open("not-a-date", 1, "2026-09-09"));
    }

    /// The window is inclusive of its last day, and a bare `for:3` means the
    /// same as `for:3d` — the `d` is the only unit there is.
    #[test]
    fn the_window_boundary_and_the_unit_are_both_forgiving() {
        assert!(window_open("2026-09-09", 1, "2026-09-09"), "day one counts");
        assert!(
            !window_open("2026-09-09", 1, "2026-09-10"),
            "and then it is over"
        );
        assert_eq!(parse_window("for:3"), Some(3));
        assert_eq!(parse_window("for:3d"), Some(3));
        assert_eq!(parse_window("for:0d"), None, "a zero-day window is not one");
        assert_eq!(parse_window("for:soon"), None);
    }
}
