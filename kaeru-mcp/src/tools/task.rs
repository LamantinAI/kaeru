//! Personal-life capture tools: `task`, `done`.

use kaeru_core::{Store, Visibility, get_visibility};
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{
    arc_closed_hint, parse_due_to_iso, parse_layer, resolve_name_or_id, text, to_mcp,
    with_initiative,
};

pub fn task(
    store: &Store,
    body: &str,
    due: Option<&str>,
    layer: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let due_iso = match due {
            Some(d) => Some(parse_due_to_iso(d)?),
            None => None,
        };
        let layer = parse_layer(layer)?;
        let id = kaeru_core::write_task_with_layer(store, body, due_iso.as_deref(), layer)
            .map_err(to_mcp)?;
        let name = kaeru_core::node_brief_by_id(store, &id)
            .ok()
            .flatten()
            .map(|b| b.name)
            .unwrap_or_default();
        let label = match due_iso.as_deref() {
            Some(d) => format!("task: {name} (due {d}) — {id}"),
            None => format!("task: {name} — {id}"),
        };
        Ok(text(&label))
    })
}

pub fn done(
    store: &Store,
    name_or_id: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name_or_id)?;
        kaeru_core::complete_task(store, &id).map_err(to_mcp)?;
        let mut msg = format!("done: {name_or_id}");
        msg.push_str(&arc_closed_hint(store, &id));
        if get_visibility(store, &id).map_err(to_mcp)? == Visibility::Shared {
            msg.push_str(
                "\n⚠ cloud copy is stale — run `share` on this node to push the new version.",
            );
        }
        Ok(text(&msg))
    })
}

#[cfg(test)]
mod tests {
    use kaeru_core::{EdgeType, EpisodeKind, Significance, Store};
    use rmcp::model::CallToolResult;

    use super::done;

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

    fn ep(store: &Store, name: &str) -> String {
        kaeru_core::write_episode(
            store,
            EpisodeKind::Observation,
            Significance::Low,
            name,
            name,
        )
        .expect("write")
    }

    /// The lost arc from the audit: five episodes ending in one that reported
    /// production verification, and all five left operational forever. The
    /// moment the work actually ends is the only observable one — "stopped
    /// changing" is visible between sessions, not inside them.
    #[test]
    fn closing_a_task_with_work_around_it_asks_what_it_concluded() {
        let store = store_t();
        let task = kaeru_core::write_task(&store, "ship the thing", None).expect("task");
        for n in ["research", "implementation", "deployed-and-verified"] {
            let e = ep(&store, n);
            kaeru_core::link(&store, &e, &task, EdgeType::RefersTo).expect("link");
        }

        let out = text_of(done(&store, &task, Some("t")).unwrap());
        assert!(out.contains("closes an arc"), "{out}");
        assert!(out.contains("3 operational nodes"), "counts them: {out}");
        assert!(out.contains("`settle <name>`"), "names the verb: {out}");
        assert!(
            out.contains("`synthesise"),
            "and the many-to-one one: {out}"
        );
    }

    /// One neighbour is a detail, not an arc — the hint has to stay quiet or
    /// it becomes furniture on every completion.
    #[test]
    fn a_lone_task_gets_no_arc_hint() {
        let store = store_t();
        let task = kaeru_core::write_task(&store, "a chore", None).expect("task");
        let e = ep(&store, "one-note");
        kaeru_core::link(&store, &e, &task, EdgeType::RefersTo).expect("link");

        let out = text_of(done(&store, &task, Some("t")).unwrap());
        assert!(!out.contains("closes an arc"), "{out}");
    }

    /// Already-archival neighbours are not outstanding work — an arc whose
    /// nodes were settled long ago is not an arc waiting to be settled.
    #[test]
    fn settled_neighbours_do_not_count_as_open_work() {
        let store = store_t();
        let task = kaeru_core::write_task(&store, "ship it", None).expect("task");
        for n in ["a", "b", "c"] {
            let e = ep(&store, n);
            kaeru_core::link(&store, &e, &task, EdgeType::RefersTo).expect("link");
            std::thread::sleep(std::time::Duration::from_millis(1100));
            kaeru_core::consolidate_out(&store, &e, kaeru_core::NodeType::Outcome, n, n)
                .expect("settle");
        }

        let out = text_of(done(&store, &task, Some("t")).unwrap());
        assert!(!out.contains("closes an arc"), "{out}");
    }
}
