//! Task primitives: `write_task` (capture a todo with optional due
//! date) and `complete_task` (flip status open → done).
//!
//! Tasks are operational `Task` nodes with auto-derived names (like
//! `jot`) and structured tags: `kind:task`, `status:open` /
//! `status:done`, optional `due:YYYY-MM-DD`, plus the standard
//! `topic:*` and `lang:*` autotags from the body.
//!
//! `complete_task` is RMW: it reads the current row at NOW, retracts,
//! and re-asserts with `status:done`. The id and name are preserved
//! so `recall` / `drill` continue to work; `history` shows the
//! transition; `tagged "status:open"` no longer surfaces the task.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;

use crate::errors::Error;
use crate::errors::Result;
use crate::graph::NodeId;
use crate::graph::audit::write_audit;
use crate::graph::new_node_id;
use crate::store::Store;

use super::attach_node_to_initiative;
use super::build_body_tags;
use super::now_validity_seconds;
use super::read_name_body_now;
use super::tags_literal;

/// Auto-derives a name from the body's first words (mirrors `jot`),
/// then captures a `Task` node with `kind:task` + `status:open` +
/// optional `due:<YYYY-MM-DD>` tag.
///
/// `due_iso` is a pre-formatted date string (`"2026-05-10"`); the CLI
/// / MCP layer parses user input into this canonical form before
/// calling. Pass `None` for tasks without a deadline.
pub fn write_task(store: &Store, body: &str, due_iso: Option<&str>) -> Result<NodeId> {
    let id = new_node_id();
    let name = derive_task_name(body, &id);

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));
    params.insert("name".to_string(), DataValue::Str(name.into()));
    params.insert("body".to_string(), DataValue::Str(body.into()));

    let due_tag;
    let mut fixed: Vec<&str> = vec!["kind:task", "status:open"];
    if let Some(d) = due_iso {
        due_tag = format!("due:{d}");
        fixed.push(&due_tag);
    }
    let all_tags = build_body_tags(&fixed, body);
    let tags = tags_literal(&all_tags);

    let now_secs = now_validity_seconds();
    let script = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{now_secs}.0, true], 'task', 'operational', $name, $body, {tags}, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Mutable)?;

    attach_node_to_initiative(store, &id)?;
    write_audit(store.db_ref(), "write_task", "system", &[id.clone()])?;
    Ok(id)
}

/// Marks an existing task as done. RMW: retract the open row, reassert
/// with `status:done` (and any other tags re-derived from the body).
/// The id and name are preserved.
///
/// Errors with `NotFound` if the task isn't currently asserted at NOW.
pub fn complete_task(store: &Store, task_id: &NodeId) -> Result<()> {
    let (name, body) = read_name_body_now(store, task_id)?
        .ok_or_else(|| Error::NotFound(format!("task {task_id} not found at NOW")))?;
    let body_text = body.unwrap_or_default();

    // Step 1 — retract current row.
    let retract_secs = now_validity_seconds();
    let mut p1: BTreeMap<String, DataValue> = BTreeMap::new();
    p1.insert("id".to_string(), DataValue::Str(task_id.clone().into()));
    let s1 = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{retract_secs}.0, false], 'task', 'operational', 'placeholder', null, null, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store.db_ref().run_script(&s1, p1, ScriptMutability::Mutable)?;

    // Step 2 — reassert with status:done. We don't preserve the
    // original `due:` tag; once the task is done, the deadline is no
    // longer actionable. (If a future need surfaces — e.g. analytics
    // on "missed deadlines" — we'd add explicit `done_at:` and copy
    // `due:` over.)
    let assert_secs = now_validity_seconds();
    let all_tags = build_body_tags(&["kind:task", "status:done"], &body_text);
    let tags = tags_literal(&all_tags);
    let mut p2: BTreeMap<String, DataValue> = BTreeMap::new();
    p2.insert("id".to_string(), DataValue::Str(task_id.clone().into()));
    p2.insert("name".to_string(), DataValue::Str(name.into()));
    p2.insert("body".to_string(), DataValue::Str(body_text.into()));
    let s2 = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{assert_secs}.0, true], 'task', 'operational', $name, $body, {tags}, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store.db_ref().run_script(&s2, p2, ScriptMutability::Mutable)?;

    write_audit(store.db_ref(), "complete_task", "system", &[task_id.clone()])?;
    Ok(())
}

/// Same shape as `derive_jot_name` in `episode.rs` but with a `task-`
/// fallback prefix when the body has no usable tokens.
fn derive_task_name(body: &str, id: &NodeId) -> String {
    const MAX_WORDS: usize = 5;
    let mut words: Vec<String> = Vec::new();
    for raw in body.split_whitespace() {
        let cleaned: String = raw
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect::<String>()
            .to_lowercase();
        if cleaned.is_empty() {
            continue;
        }
        words.push(cleaned);
        if words.len() >= MAX_WORDS {
            break;
        }
    }
    let id_suffix: String = id
        .chars()
        .rev()
        .take(6)
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    if words.is_empty() {
        format!("task-{id_suffix}")
    } else {
        format!("{}-{id_suffix}", words.join("-"))
    }
}
