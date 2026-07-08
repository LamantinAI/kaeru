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

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};

use super::{
    ReassertRow, attach_node_to_initiative, build_body_tags, merge_tags, now_validity_seconds,
    read_node_now, reassert_node_now, retract_node_at, tags_literal,
};
use crate::errors::{Error, Result};
use crate::graph::audit::write_audit;
use crate::graph::{Layer, NodeId, new_node_id};
use crate::store::Store;

/// Auto-derives a name from the body's first words (mirrors `jot`),
/// then captures a `Task` node with `kind:task` + `status:open` +
/// optional `due:<YYYY-MM-DD>` tag.
///
/// `due_iso` is a pre-formatted date string (`"2026-05-10"`); the CLI
/// / MCP layer parses user input into this canonical form before
/// calling. Pass `None` for tasks without a deadline.
pub fn write_task(store: &Store, body: &str, due_iso: Option<&str>) -> Result<NodeId> {
    write_task_with_layer(store, body, due_iso, Layer::default())
}

/// Captures a `Task` node with an explicit memory layer, stamped at
/// creation so the todo lands in the right recall priority band without
/// a follow-up `set_layer`.
pub fn write_task_with_layer(
    store: &Store,
    body: &str,
    due_iso: Option<&str>,
    layer: Layer,
) -> Result<NodeId> {
    let id = new_node_id();
    let name = derive_task_name(body, &id);

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));
    params.insert("name".to_string(), DataValue::Str(name.into()));
    params.insert("body".to_string(), DataValue::Str(body.into()));
    params.insert("layer".to_string(), DataValue::Str(layer.as_str().into()));

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
        ?[id, validity, type, tier, name, body, tags, initiatives, properties, layer] <-
            [[$id, [{now_secs}.0, true], 'task', 'operational', $name, $body, {tags}, null, null, $layer]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties, layer}}
        "#
    );
    store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Mutable)?;

    attach_node_to_initiative(store, &id)?;
    write_audit(store.db_ref(), "write_task", "system", &[id.clone()])?;
    Ok(id)
}

/// Marks an existing task as done. RMW: re-assert the row with
/// `status:done`, then retract the open one. The id, name, body, `layer`,
/// `visibility`, `properties`, and manual tags are preserved.
///
/// We don't preserve the original `due:` tag; once the task is done, the
/// deadline is no longer actionable. (If a future need surfaces — e.g.
/// analytics on "missed deadlines" — we'd add explicit `done_at:` and copy
/// `due:` over.)
///
/// Errors with `NotFound` if the task isn't currently asserted at NOW.
pub fn complete_task(store: &Store, task_id: &NodeId) -> Result<()> {
    let current = read_node_now(store, task_id)?
        .ok_or_else(|| Error::NotFound(format!("task {task_id} not found at NOW")))?;
    let body_text = current.body.clone().unwrap_or_default();

    let fresh = build_body_tags(&["kind:task", "status:done"], &body_text);
    let tags = merge_tags(
        &current.tags,
        &["status:", "due:", "lang:", "topic:"],
        fresh,
    );

    // Re-assert first, retract second, same timestamp — see
    // `reassert_node_now` for the ordering invariant.
    let secs = now_validity_seconds();
    reassert_node_now(
        store,
        task_id,
        ReassertRow {
            secs,
            type_: &current.type_,
            tier: &current.tier,
            name: &current.name,
            body: Some(&body_text),
            tags,
            visibility: &current.visibility,
            layer: &current.layer,
        },
    )?;
    retract_node_at(store, task_id, secs)?;

    write_audit(
        store.db_ref(),
        "complete_task",
        "system",
        &[task_id.clone()],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::graph::{Layer, Visibility};
    use crate::store::Store;
    use crate::{at, complete_task, set_visibility, write_task_with_layer};

    /// `complete_task` used to re-assert with an incomplete column list,
    /// resetting `layer` / `visibility` to schema defaults on every `done`.
    #[test]
    fn complete_task_preserves_layer_and_visibility() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        let id = write_task_with_layer(&store, "ship the fix", Some("2026-07-10"), Layer::Hot)
            .expect("task");
        set_visibility(&store, &id, Visibility::Shared).expect("set vis");

        std::thread::sleep(Duration::from_millis(1100));
        complete_task(&store, &id).expect("done");

        let snap = at(&store, &id, 9_999_999_999.0)
            .expect("at")
            .expect("still resolves");
        assert_eq!(snap.layer, "hot", "layer survives done");
        assert_eq!(snap.visibility, "shared", "visibility survives done");
        assert!(snap.tags.iter().any(|t| t == "status:done"));
        assert!(
            !snap.tags.iter().any(|t| t.starts_with("status:open")),
            "open status dropped: {:?}",
            snap.tags
        );
        assert!(
            !snap.tags.iter().any(|t| t.starts_with("due:")),
            "due: deliberately dropped on done: {:?}",
            snap.tags
        );
    }
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
