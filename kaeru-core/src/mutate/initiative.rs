//! Initiative-level mutations: `rename_initiative`, `delete_initiative`.
//!
//! An initiative is a scoping key, not a stored node — it lives in the
//! junction relations (`node_initiative`, `edge_initiative`) and the
//! `initiative` policy table. These verbs move or drop every trace of an
//! initiative name in one pass. Both take **explicit** names (no reliance
//! on `Store::current_initiative`), so the cloud can call them too.

use std::collections::BTreeMap;

use cozo::{DataValue, NamedRows, ScriptMutability};

use super::forget;
use crate::errors::{Error, Result};
use crate::graph::audit::write_audit;
use crate::store::Store;

/// Counts moved by [`rename_initiative`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenameStats {
    pub nodes: usize,
    pub edges: usize,
}

/// Counts affected by [`delete_initiative`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteStats {
    /// Nodes that lost this membership but remain in other initiatives.
    pub unscoped: usize,
    /// Nodes that were exclusive to this initiative and got forgotten.
    pub forgotten: usize,
}

fn one(k: &str, v: &str) -> BTreeMap<String, DataValue> {
    let mut m = BTreeMap::new();
    m.insert(k.to_string(), DataValue::Str(v.into()));
    m
}

fn run_mut(store: &Store, script: &str, params: BTreeMap<String, DataValue>) -> Result<()> {
    store
        .db_ref()
        .run_script(script, params, ScriptMutability::Mutable)?;
    Ok(())
}

fn run_read(store: &Store, script: &str, params: BTreeMap<String, DataValue>) -> Result<NamedRows> {
    Ok(store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?)
}

/// Node ids attached to `initiative` through the junction.
fn node_ids_in(store: &Store, initiative: &str) -> Result<Vec<String>> {
    let rows = run_read(
        store,
        "?[node_id] := *node_initiative{initiative, node_id}, initiative = $init",
        one("init", initiative),
    )?;
    Ok(rows
        .rows
        .iter()
        .filter_map(|r| r.first().and_then(|v| v.get_str()).map(String::from))
        .collect())
}

/// Renames initiative `old` to `new` across both junction relations and
/// the policy table. Fails if `new` already exists (has members or a
/// policy row) — pick a fresh name rather than silently merging.
pub fn rename_initiative(store: &Store, old: &str, new: &str) -> Result<RenameStats> {
    let new_t = new.trim();
    if new_t.is_empty() {
        return Err(Error::Invalid(
            "new initiative name must not be empty".to_string(),
        ));
    }
    if old == new_t {
        return Err(Error::Invalid(
            "old and new names are identical".to_string(),
        ));
    }

    // Collision guard: refuse if `new` already has any node or a policy row.
    let target_has_nodes = !node_ids_in(store, new_t)?.is_empty();
    let target_has_policy = !run_read(
        store,
        "?[share_policy] := *initiative{name, share_policy}, name = $n",
        one("n", new_t),
    )?
    .rows
    .is_empty();
    if target_has_nodes || target_has_policy {
        return Err(Error::Invalid(format!(
            "target initiative `{new_t}` already exists — rename into a fresh name"
        )));
    }

    let nodes = node_ids_in(store, old)?;
    let edges = run_read(
        store,
        "?[edge_pk] := *edge_initiative{initiative, edge_pk}, initiative = $init",
        one("init", old),
    )?
    .rows
    .len();

    let mut both = BTreeMap::new();
    both.insert("old".to_string(), DataValue::Str(old.into()));
    both.insert("new".to_string(), DataValue::Str(new_t.into()));

    // node_initiative: add (new, node_id) for each old row, then drop old.
    run_mut(
        store,
        r#"
        ?[initiative, node_id] := *node_initiative{initiative: oi, node_id}, oi = $old, initiative = $new
        :put node_initiative {initiative, node_id}
        "#,
        both.clone(),
    )?;
    run_mut(
        store,
        r#"
        ?[initiative, node_id] := *node_initiative{initiative, node_id}, initiative = $old
        :rm node_initiative {initiative, node_id}
        "#,
        one("old", old),
    )?;

    // edge_initiative: same move.
    run_mut(
        store,
        r#"
        ?[initiative, edge_pk] := *edge_initiative{initiative: oi, edge_pk}, oi = $old, initiative = $new
        :put edge_initiative {initiative, edge_pk}
        "#,
        both.clone(),
    )?;
    run_mut(
        store,
        r#"
        ?[initiative, edge_pk] := *edge_initiative{initiative, edge_pk}, initiative = $old
        :rm edge_initiative {initiative, edge_pk}
        "#,
        one("old", old),
    )?;

    // initiative policy table: move the row if present.
    run_mut(
        store,
        r#"
        ?[name, share_policy] := *initiative{name: o, share_policy}, o = $old, name = $new
        :put initiative {name => share_policy}
        "#,
        both,
    )?;
    run_mut(
        store,
        r#"
        ?[name] := *initiative{name}, name = $old
        :rm initiative {name}
        "#,
        one("old", old),
    )?;

    write_audit(
        store.db_ref(),
        "rename_initiative",
        "system",
        &[old.to_string(), new_t.to_string()],
    )?;
    Ok(RenameStats {
        nodes: nodes.len(),
        edges,
    })
}

/// Deletes initiative `name`: drops its membership rows and policy, then
/// `forget`s every node that was **exclusive** to it (now in no initiative
/// at all). Nodes shared with other initiatives only lose this one
/// membership. Forgetting is bi-temporal — the assertions survive in
/// history, so a delete is recoverable via `at(<past>)`.
pub fn delete_initiative(store: &Store, name: &str) -> Result<DeleteStats> {
    let nodes = node_ids_in(store, name)?;

    run_mut(
        store,
        r#"
        ?[initiative, node_id] := *node_initiative{initiative, node_id}, initiative = $init
        :rm node_initiative {initiative, node_id}
        "#,
        one("init", name),
    )?;
    run_mut(
        store,
        r#"
        ?[initiative, edge_pk] := *edge_initiative{initiative, edge_pk}, initiative = $init
        :rm edge_initiative {initiative, edge_pk}
        "#,
        one("init", name),
    )?;
    run_mut(
        store,
        r#"
        ?[name] := *initiative{name}, name = $init
        :rm initiative {name}
        "#,
        one("init", name),
    )?;

    // Forget nodes that are now in no initiative at all.
    let mut forgotten = 0usize;
    for nid in &nodes {
        let still = run_read(
            store,
            "?[initiative] := *node_initiative{initiative, node_id}, node_id = $nid",
            one("nid", nid),
        )?;
        if still.rows.is_empty() {
            forget(store, nid)?;
            forgotten += 1;
        }
    }

    write_audit(
        store.db_ref(),
        "delete_initiative",
        "system",
        &[name.to_string()],
    )?;
    Ok(DeleteStats {
        unscoped: nodes.len() - forgotten,
        forgotten,
    })
}

#[cfg(test)]
mod tests {
    use super::{delete_initiative, rename_initiative};
    use crate::graph::EdgeType;
    use crate::store::Store;
    use crate::{
        EpisodeKind, SharePolicy, Significance, get_share_policy, link, list_initiatives,
        recall_id_by_name, set_share_policy, write_episode,
    };

    #[test]
    fn rename_moves_nodes_edges_and_policy() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("old-proj");
        let a = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "a",
            "A",
        )
        .unwrap();
        let b = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "b",
            "B",
        )
        .unwrap();
        link(&store, &a, &b, EdgeType::Causal).unwrap();
        set_share_policy(&store, "old-proj", SharePolicy::Team).unwrap();

        let stats = rename_initiative(&store, "old-proj", "new-proj").unwrap();
        assert_eq!(stats.nodes, 2);

        let inits = list_initiatives(&store).unwrap();
        assert!(inits.iter().any(|n| n == "new-proj"));
        assert!(!inits.iter().any(|n| n == "old-proj"), "old name gone");

        // Policy moved to the new name; old falls back to the default.
        assert_eq!(
            get_share_policy(&store, "new-proj").unwrap(),
            SharePolicy::Team
        );
        assert_eq!(
            get_share_policy(&store, "old-proj").unwrap(),
            SharePolicy::Private
        );

        // Nodes resolve under the new scope, not the old.
        store.use_initiative("new-proj");
        assert!(recall_id_by_name(&store, "a").unwrap().is_some());
        store.use_initiative("old-proj");
        assert!(recall_id_by_name(&store, "a").unwrap().is_none());
    }

    #[test]
    fn rename_rejects_existing_target() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("a");
        write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "na",
            "x",
        )
        .unwrap();
        store.use_initiative("b");
        write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "nb",
            "y",
        )
        .unwrap();
        assert!(
            rename_initiative(&store, "a", "b").is_err(),
            "merge into existing refused"
        );
    }

    #[test]
    fn delete_forgets_exclusive_keeps_shared() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("proj");
        let x = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "x-excl",
            "X",
        )
        .unwrap();
        let y = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "y-shared",
            "Y",
        )
        .unwrap();

        // Also attach y to a second initiative `keep` (direct junction write).
        store
            .run(&format!(
                "?[initiative, node_id] <- [['keep', '{y}']] :put node_initiative {{initiative, node_id}}"
            ))
            .unwrap();

        // Whole-second validity: cross the boundary so the forget retraction
        // wins over the same-second assertion (real deletes happen far later
        // than creation; only the test races the clock).
        std::thread::sleep(std::time::Duration::from_millis(1100));

        let stats = delete_initiative(&store, "proj").unwrap();
        assert_eq!(stats.forgotten, 1, "x was exclusive → forgotten");
        assert_eq!(stats.unscoped, 1, "y stays in `keep`");

        // proj is gone; keep remains with y.
        let inits = list_initiatives(&store).unwrap();
        assert!(!inits.iter().any(|n| n == "proj"));
        assert!(inits.iter().any(|n| n == "keep"));

        store.use_initiative("keep");
        assert!(
            recall_id_by_name(&store, "y-shared").unwrap().is_some(),
            "y kept"
        );
        store.clear_initiative();
        assert!(
            recall_id_by_name(&store, "x-excl").unwrap().is_none(),
            "x forgotten at NOW"
        );
        let _ = x;
    }
}
