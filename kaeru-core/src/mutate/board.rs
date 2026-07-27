//! Task-board mutations — the status registry + moving tasks between columns.
//!
//! `set_status` is the generalized `complete_task`: an RMW that swaps a task's
//! `status:<key>` tag, validated against the initiative's effective status
//! registry (strict — an unknown key is refused, so a typo can't spawn a
//! phantom column). Column customization goes through `add_status` /
//! `remove_status` / `relabel_status` / `reorder_statuses`, which materialize a
//! `Board` node (seeded from the defaults on first edit) and rewrite its
//! `properties.statuses`. The `key` is stable (it's the task tag); the `label`
//! is free to change without touching any task.

use std::collections::{BTreeMap, BTreeSet};

use cozo::{DataValue, JsonData, ScriptMutability};
use serde_json::{Value, json};

use super::{
    ReassertRow, attach_node_to_initiative_named, build_body_tags, merge_tags,
    now_validity_seconds, read_node_now, reassert_node_now, retract_node_at, tags_literal,
};
use crate::errors::{Error, Result};
use crate::graph::audit::write_audit;
use crate::graph::{NodeId, new_node_id};
use crate::recall::board::{
    BoardStatus, DEFAULT_STATUSES, board_node_id, effective_statuses, read_board_statuses,
};
use crate::store::Store;

/// Moves a task to `status` (its board column) — an RMW that swaps the
/// `status:<key>` tag, preserving id, name, body, `layer`, `visibility`,
/// `properties`, `due:`, and manual tags. **Strict**: `status` must be a key in
/// `initiative`'s effective registry, else `Invalid` listing the known keys.
/// The generalized `complete_task` (which is `set_status(.., "done")` minus the
/// `due:` drop).
pub fn set_status(store: &Store, initiative: &str, task_id: &NodeId, status: &str) -> Result<()> {
    let statuses = effective_statuses(store, initiative)?;
    if !statuses.iter().any(|s| s.key == status) {
        let known: Vec<&str> = statuses.iter().map(|s| s.key.as_str()).collect();
        return Err(Error::Invalid(format!(
            "unknown status `{status}`; known on `{initiative}`: [{}]",
            known.join(", ")
        )));
    }

    let current = read_node_now(store, task_id)?
        .ok_or_else(|| Error::NotFound(format!("task {task_id} not found at NOW")))?;
    if current.type_ != "task" {
        return Err(Error::Invalid(format!(
            "{task_id} is not a task (type: {})",
            current.type_
        )));
    }

    let body_text = current.body.clone().unwrap_or_default();
    let status_tag = format!("status:{status}");
    let fresh = build_body_tags(&["kind:task", &status_tag], &body_text);
    // Drop the re-derived families; KEEP `due:` (the deadline still stands,
    // unlike `complete_task`) and any manual tags.
    let tags = merge_tags(&current.tags, &["status:", "lang:", "topic:"], fresh);

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

    write_audit(store.db_ref(), "set_status", "system", &[task_id.clone()])?;
    Ok(())
}

/// Finds `initiative`'s `Board` node, creating it (seeded from
/// [`DEFAULT_STATUSES`]) if it doesn't exist yet. Returns the board id.
pub fn ensure_board(store: &Store, initiative: &str) -> Result<NodeId> {
    if let Some(id) = board_node_id(store, initiative)? {
        return Ok(id);
    }
    let id = new_node_id();
    let statuses = default_statuses();
    let secs = now_validity_seconds();
    put_board(store, &id, initiative, &statuses, secs)?;
    attach_node_to_initiative_named(store, &id, initiative)?;
    write_audit(store.db_ref(), "ensure_board", "system", &[id.clone()])?;
    Ok(id)
}

fn default_statuses() -> Vec<BoardStatus> {
    DEFAULT_STATUSES
        .iter()
        .map(|(k, l)| BoardStatus {
            key: (*k).to_string(),
            label: (*l).to_string(),
        })
        .collect()
}

/// Writes the board node at `[secs, true]` with `properties.statuses = statuses`.
/// A raw `:put` (not `reassert_node_now`, which copies `properties` forward —
/// here we're deliberately *changing* them). `name` is deterministic so an
/// update reconstructs the row without reading it back.
fn put_board(
    store: &Store,
    board_id: &NodeId,
    initiative: &str,
    statuses: &[BoardStatus],
    secs: u64,
) -> Result<()> {
    let arr: Vec<Value> = statuses
        .iter()
        .map(|s| json!({ "key": s.key, "label": s.label }))
        .collect();
    let payload = json!({ "statuses": arr });

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(board_id.clone().into()));
    params.insert(
        "name".to_string(),
        DataValue::Str(format!("board-{initiative}").into()),
    );
    params.insert("properties".to_string(), DataValue::Json(JsonData(payload)));
    let tags = tags_literal(&["kind:board".to_string()]);
    let script = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties, layer] <-
            [[$id, [{secs}.0, true], 'board', 'operational', $name, null, {tags}, null, $properties, 'warm']]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties, layer}}
        "#
    );
    store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Mutable)?;
    Ok(())
}

/// Ensures the board, reads its statuses, applies `f`, and writes them back
/// (reassert new + retract old at one timestamp — the `complete_task` pattern).
/// Returns the new status list. `f` reports a domain error (`Invalid` /
/// `NotFound`) to reject the edit without writing.
fn update_statuses<F>(store: &Store, initiative: &str, f: F) -> Result<Vec<BoardStatus>>
where
    F: FnOnce(&mut Vec<BoardStatus>) -> Result<()>,
{
    let board_id = ensure_board(store, initiative)?;
    let mut statuses = read_board_statuses(store, &board_id)?;
    if statuses.is_empty() {
        statuses = default_statuses();
    }
    f(&mut statuses)?;

    let secs = now_validity_seconds();
    put_board(store, &board_id, initiative, &statuses, secs)?;
    retract_node_at(store, &board_id, secs)?;
    write_audit(
        store.db_ref(),
        "board_status",
        "system",
        &[board_id.clone()],
    )?;
    Ok(statuses)
}

/// Adds a new column to the registry (appended last). Errors if `key` exists.
pub fn add_status(
    store: &Store,
    initiative: &str,
    key: &str,
    label: &str,
) -> Result<Vec<BoardStatus>> {
    update_statuses(store, initiative, |st| {
        if st.iter().any(|s| s.key == key) {
            return Err(Error::Invalid(format!("status `{key}` already exists")));
        }
        st.push(BoardStatus {
            key: key.to_string(),
            label: label.to_string(),
        });
        Ok(())
    })
}

/// Removes a column. Tasks still tagged `status:<key>` aren't touched — they
/// fall into the first column in `board_view` until re-statused. Refuses to
/// remove the last column.
pub fn remove_status(store: &Store, initiative: &str, key: &str) -> Result<Vec<BoardStatus>> {
    update_statuses(store, initiative, |st| {
        let before = st.len();
        st.retain(|s| s.key != key);
        if st.len() == before {
            return Err(Error::NotFound(format!("no status `{key}`")));
        }
        if st.is_empty() {
            return Err(Error::Invalid("cannot remove the last status".to_string()));
        }
        Ok(())
    })
}

/// Renames a column's `label` — cheap, touches no task (the `key` is stable).
pub fn relabel_status(
    store: &Store,
    initiative: &str,
    key: &str,
    label: &str,
) -> Result<Vec<BoardStatus>> {
    update_statuses(store, initiative, |st| {
        let s = st
            .iter_mut()
            .find(|s| s.key == key)
            .ok_or_else(|| Error::NotFound(format!("no status `{key}`")))?;
        s.label = label.to_string();
        Ok(())
    })
}

/// Reorders the columns. `order` must be exactly the existing keys, permuted.
pub fn reorder_statuses(
    store: &Store,
    initiative: &str,
    order: &[String],
) -> Result<Vec<BoardStatus>> {
    update_statuses(store, initiative, |st| {
        let have: BTreeSet<&str> = st.iter().map(|s| s.key.as_str()).collect();
        let want: BTreeSet<&str> = order.iter().map(String::as_str).collect();
        if have != want {
            return Err(Error::Invalid(
                "reorder must list exactly the existing status keys".to_string(),
            ));
        }
        st.sort_by_key(|s| order.iter().position(|k| k == &s.key).unwrap_or(usize::MAX));
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::{add_status, relabel_status, remove_status, reorder_statuses, set_status};
    use crate::recall::board::board_view;
    use crate::store::Store;
    use crate::{BoardView, effective_statuses, write_task};

    fn keys(v: &BoardView) -> Vec<String> {
        v.columns.iter().map(|c| c.key.clone()).collect()
    }
    fn count_in(v: &BoardView, key: &str) -> usize {
        v.columns
            .iter()
            .find(|c| c.key == key)
            .map_or(0, |c| c.tasks.len())
    }

    #[test]
    fn defaults_bucket_then_set_status_moves() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("proj");
        let id = write_task(&store, "ship the thing", None).expect("task");

        // Before any customization: default columns, task lands in `open`.
        let v = board_view(&store, "proj").expect("board");
        assert_eq!(keys(&v), vec!["open", "in-progress", "done"]);
        assert_eq!(count_in(&v, "open"), 1, "new task in open");
        assert_eq!(count_in(&v, "in-progress"), 0);

        // Move it (cross the whole-second validity boundary first).
        std::thread::sleep(std::time::Duration::from_millis(1100));
        set_status(&store, "proj", &id, "in-progress").expect("move");
        let v = board_view(&store, "proj").expect("board");
        assert_eq!(count_in(&v, "open"), 0, "left open");
        assert_eq!(count_in(&v, "in-progress"), 1, "now in-progress");

        // Strict validation: an unknown status is refused, task unchanged.
        let err = set_status(&store, "proj", &id, "nope").unwrap_err();
        assert!(err.to_string().contains("unknown status"), "got {err}");
        assert_eq!(
            count_in(&board_view(&store, "proj").unwrap(), "in-progress"),
            1
        );
    }

    #[test]
    fn column_crud_customizes_the_registry() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("proj");

        // add a column — shows even while empty
        let st = add_status(&store, "proj", "review", "Review").expect("add");
        assert!(st.iter().any(|s| s.key == "review" && s.label == "Review"));
        assert!(
            board_view(&store, "proj")
                .unwrap()
                .columns
                .iter()
                .any(|c| c.key == "review")
        );
        assert_eq!(effective_statuses(&store, "proj").unwrap().len(), 4);
        assert!(
            add_status(&store, "proj", "review", "dup").is_err(),
            "no dup keys"
        );

        // relabel: key stable, label changes
        let st = relabel_status(&store, "proj", "review", "In Review").expect("relabel");
        assert_eq!(
            st.iter().find(|s| s.key == "review").unwrap().label,
            "In Review"
        );

        // reorder must be a permutation of existing keys
        assert!(reorder_statuses(&store, "proj", &["review".into()]).is_err());
        let st = reorder_statuses(
            &store,
            "proj",
            &[
                "open".into(),
                "review".into(),
                "in-progress".into(),
                "done".into(),
            ],
        )
        .expect("reorder");
        assert_eq!(
            st.iter().map(|s| s.key.clone()).collect::<Vec<_>>(),
            vec!["open", "review", "in-progress", "done"]
        );

        // remove
        let st = remove_status(&store, "proj", "review").expect("remove");
        assert!(!st.iter().any(|s| s.key == "review"));
        assert!(
            remove_status(&store, "proj", "review").is_err(),
            "already gone"
        );
    }
}
