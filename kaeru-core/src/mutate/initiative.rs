//! Initiative-level mutations: `rename_initiative`, `delete_initiative`.
//!
//! An initiative is a scoping key, not a stored node — it lives in the
//! junction relations (`node_initiative`, `edge_initiative`) and the
//! `initiative` policy table. These verbs move or drop every trace of an
//! initiative name in one pass. Both take **explicit** names (no reliance
//! on `Store::current_initiative`), so the cloud can call them too.

use crate::board_node_id;
use std::collections::BTreeMap;

use cozo::{DataValue, NamedRows, ScriptMutability};

use super::forget;
use crate::errors::{Error, Result};
use crate::graph::NodeId;
use crate::graph::audit::write_audit;
use crate::recall::node_brief_by_id;
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

/// Result of [`attach_node`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttachStats {
    /// True if the node already belonged to the initiative, so the attach
    /// was a no-op.
    pub already_member: bool,
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
    store.atomic(|store| {
        let old = old.trim();
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

        move_scope_metadata(store,old,new_t,false,true)?;
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
    })
}

/// Counts moved by [`merge_initiative`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MergeStats {
    /// Nodes re-homed into the target (some may already have been members).
    pub nodes: usize,
    /// Edges re-homed into the target.
    pub edges: usize,
    /// True when the source carried a share policy the target already had, so
    /// the source's was dropped rather than overwriting the target's.
    pub policy_kept_target: bool,
}

/// Merges initiative `source` **into** `target`: every node and edge of the
/// source becomes a member of the target, and the source name is removed.
///
/// This exists because the two names of one split project could not be
/// rejoined by any single verb (#86). `rename_initiative` refuses a target
/// that already exists — deliberately, so a rename cannot silently merge —
/// which left `attach` on every node one at a time, followed by
/// `delete_initiative` on the emptied name. That sequence has a trap in it:
/// `delete_initiative` **forgets** every node exclusive to the name it
/// removes, so a node the operator missed is destroyed by the cleanup meant
/// to be safe.
///
/// Doing it as one operation is the whole point: memberships are added to the
/// target *before* the source's rows are dropped, so at no moment is a node
/// exclusive to a name being deleted, and nothing can be left behind to
/// forget. Nothing is retracted here at all — a merge only ever adds a
/// membership and drops a junction row.
///
/// The target's share policy wins when both have one. A merge should not be
/// able to loosen where an initiative may go: `MergeStats::policy_kept_target`
/// says when that happened so the caller can mention it.
pub fn merge_initiative(store: &Store, source: &str, target: &str) -> Result<MergeStats> {
    store.atomic(|store| {
        let source = source.trim();
        let target = target.trim();
        if source.is_empty() || target.is_empty() {
            return Err(Error::Invalid(
                "both initiative names must be non-empty".to_string(),
            ));
        }
        if source == target {
            return Err(Error::Invalid(
                "source and target are the same initiative".to_string(),
            ));
        }
        if board_node_id(store,source,None)?.is_some()&&board_node_id(store,target,None)?.is_some(){
            return Err(Error::Invalid("both initiatives have board registries; retain one explicitly before merging so status meaning is not chosen by node ID".into()));
        }
        validate_slot_merge(store, source, target)?;
        let nodes = node_ids_in(store, source)?;
        if nodes.is_empty() {
            return Err(Error::NotFound(format!(
                "initiative `{source}` has no nodes — nothing to merge"
            )));
        }

        let edges = run_read(
            store,
            "?[edge_pk] := *edge_initiative{initiative, edge_pk}, initiative = $init",
            one("init", source),
        )?
        .rows
        .len();

        let mut both = BTreeMap::new();
        both.insert("source".to_string(), DataValue::Str(source.into()));
        both.insert("target".to_string(), DataValue::Str(target.into()));

        // Add first, remove second. The order is the safety property: a node is a
        // member of the target before it stops being a member of the source, so
        // it is never briefly homeless.
        run_mut(
            store,
            r#"
            ?[initiative, node_id] := *node_initiative{initiative: si, node_id}, si = $source,
                                      initiative = $target
            :put node_initiative {initiative, node_id}
            "#,
            both.clone(),
        )?;
        run_mut(
            store,
            r#"
            ?[initiative, edge_pk] := *edge_initiative{initiative: si, edge_pk}, si = $source,
                                      initiative = $target
            :put edge_initiative {initiative, edge_pk}
            "#,
            both.clone(),
        )?;

        // The target keeps its own policy; the source's row is only moved when
        // the target has none. Widening where an initiative may be shared is not
        // something a merge should do quietly.
        let target_has_policy = !run_read(
            store,
            "?[share_policy] := *initiative{name, share_policy}, name = $n",
            one("n", target),
        )?
        .rows
        .is_empty();
        let source_has_policy = !run_read(
            store,
            "?[share_policy] := *initiative{name, share_policy}, name = $n",
            one("n", source),
        )?
        .rows
        .is_empty();
        if !target_has_policy && source_has_policy {
            run_mut(
                store,
                r#"
                ?[name, share_policy] := *initiative{name: s, share_policy}, s = $source,
                                         name = $target
                :put initiative {name => share_policy}
                "#,
                both.clone(),
            )?;
        }

        run_mut(
            store,
            r#"
            ?[initiative, node_id] := *node_initiative{initiative, node_id}, initiative = $source
            :rm node_initiative {initiative, node_id}
            "#,
            one("source", source),
        )?;
        run_mut(
            store,
            r#"
            ?[initiative, edge_pk] := *edge_initiative{initiative, edge_pk}, initiative = $source
            :rm edge_initiative {initiative, edge_pk}
            "#,
            one("source", source),
        )?;
        run_mut(
            store,
            r#"
            ?[name] := *initiative{name}, name = $source
            :rm initiative {name}
            "#,
            one("source", source),
        )?;

        move_scope_metadata(store, source, target, true, !target_has_policy)?;
        write_audit(
            store.db_ref(),
            "merge_initiative",
            "system",
            &[source.to_string(), target.to_string()],
        )?;
        Ok(MergeStats {
            nodes: nodes.len(),
            edges,
            policy_kept_target: target_has_policy && source_has_policy,
        })
    })
}

/// What [`delete_initiative`] would destroy, without destroying it.
///
/// `delete_initiative` forgets every node exclusive to the name it removes.
/// That is reasonable for an initiative genuinely not wanted and a data-loss
/// trap for a duplicate — which is the case someone reaching for it after a
/// split is most likely in (#86). The caller is expected to say the number
/// out loud before acting on it.
pub fn delete_initiative_impact(store: &Store, name: &str) -> Result<DeleteStats> {
    let name = name.trim();
    let mut unscoped = 0usize;
    let mut forgotten = 0usize;
    for nid in node_ids_in(store, name)? {
        let elsewhere = run_read(
            store,
            "?[initiative] := *node_initiative{initiative, node_id}, node_id = $nid",
            one("nid", &nid),
        )?
        .rows
        .len();
        // Membership in this initiative is one of the rows counted.
        if elsewhere > 1 {
            unscoped += 1
        } else {
            forgotten += 1
        }
    }
    Ok(DeleteStats {
        unscoped,
        forgotten,
    })
}

/// Deletes initiative `name`: drops its membership rows and policy, then
/// `forget`s every node that was **exclusive** to it (now in no initiative
/// at all). Nodes shared with other initiatives only lose this one
/// membership. Forgetting is bi-temporal — the assertions survive in
/// history, so a delete is recoverable via `at(<past>)`.
pub fn delete_initiative(store: &Store, name: &str) -> Result<DeleteStats> {
    store.atomic(|store| {
        let name = name.trim();
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

        remove_scope_metadata(store, name)?;
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
    })
}

/// Adds `node_id` to `initiative` as an **additive** membership: the node
/// gains a second home without losing any it already has, and without
/// copying — same id, edges, and history. This is the repair primitive for
/// initiative fragmentation: a node captured under the wrong (or a stale)
/// initiative can be re-homed under the right one after the fact.
///
/// Idempotent — the junction PK `(initiative, node_id)` dedups, so attaching
/// an existing member is a no-op (reported via [`AttachStats::already_member`]).
/// Errors if the node does not exist at NOW, so no dangling membership row is
/// created for a bogus id.
pub fn attach_node(store: &Store, node_id: &NodeId, initiative: &str) -> Result<AttachStats> {
    store.atomic(|store| {
        let init = initiative.trim();
        if init.is_empty() {
            return Err(Error::Invalid(
                "initiative name must not be empty".to_string(),
            ));
        }
        if node_brief_by_id(store, node_id)?.is_none() {
            return Err(Error::NotFound(format!("no node {node_id:?} at NOW")));
        }

        let mut params = one("init", init);
        params.insert("nid".to_string(), DataValue::Str(node_id.clone().into()));

        let already_member = !run_read(
            store,
            "?[initiative] := *node_initiative{initiative, node_id}, \
             initiative = $init, node_id = $nid",
            params.clone(),
        )?
        .rows
        .is_empty();

        run_mut(
            store,
            r#"
            ?[initiative, node_id] <- [[$init, $nid]]
            :put node_initiative {initiative, node_id}
            "#,
            params,
        )?;

        write_audit(
            store.db_ref(),
            "attach_node",
            "system",
            &[init.to_string(), node_id.clone()],
        )?;
        Ok(AttachStats { already_member })
    })
}

#[cfg(test)]
mod tests {
    use super::{
        attach_node, delete_initiative, delete_initiative_impact, merge_initiative,
        rename_initiative,
    };
    use crate::graph::EdgeType;
    use crate::store::Store;
    use crate::{
        EpisodeKind, SharePolicy, Significance, get_share_policy, link, list_initiatives,
        node_brief_by_id, recall_id_by_name, set_share_policy, write_episode,
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

    #[test]
    fn attach_node_adds_membership_without_moving() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("proj-a");
        let n = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "shared-fact",
            "body",
        )
        .unwrap();

        // Additive attach to a second initiative — the node lives in both now.
        let stats = attach_node(&store, &n, "proj-b").unwrap();
        assert!(!stats.already_member);

        store.use_initiative("proj-a");
        assert_eq!(
            recall_id_by_name(&store, "shared-fact").unwrap(),
            Some(n.clone()),
            "still resolves under the original initiative"
        );
        store.use_initiative("proj-b");
        assert_eq!(
            recall_id_by_name(&store, "shared-fact").unwrap(),
            Some(n.clone()),
            "now also resolves under the new initiative"
        );

        let inits = list_initiatives(&store).unwrap();
        assert!(inits.iter().any(|i| i == "proj-a"));
        assert!(inits.iter().any(|i| i == "proj-b"));

        // Idempotent: re-attaching is a reported no-op.
        assert!(
            attach_node(&store, &n, "proj-b").unwrap().already_member,
            "second attach is a no-op"
        );

        // A bogus id is refused, so no dangling membership row is created.
        assert!(
            attach_node(
                &store,
                &"01900000-0000-7000-0000-000000000000".to_string(),
                "proj-b"
            )
            .is_err(),
            "attaching a non-existent node errors"
        );
    }

    /// Seeds `count` episodes under `init` and returns their ids.
    fn seed_n(store: &Store, init: &str, count: usize) -> Vec<String> {
        store.use_initiative(init);
        (0..count)
            .map(|i| {
                write_episode(
                    store,
                    EpisodeKind::Observation,
                    Significance::Low,
                    &format!("{init}-note-{i}"),
                    "body",
                )
                .unwrap()
            })
            .collect()
    }

    /// The whole point of the verb: everything moves, in one step, and the
    /// source name is gone. `rename_initiative` refuses an existing target on
    /// purpose, so before this there was no way at all to rejoin two names of
    /// one project (#86).
    #[test]
    fn merge_rehomes_everything_and_removes_the_source() {
        let store = Store::open_in_memory().expect("open");
        let ids = seed_n(&store, "alpha_beta", 3);
        seed_n(&store, "alpha-beta", 1);
        link(&store, &ids[0], &ids[1], EdgeType::RefersTo).unwrap();

        let stats = merge_initiative(&store, "alpha_beta", "alpha-beta").unwrap();
        assert_eq!(stats.nodes, 3);

        let names = list_initiatives(&store).unwrap();
        assert!(!names.iter().any(|n| n == "alpha_beta"), "source gone");
        assert!(names.iter().any(|n| n == "alpha-beta"), "target stands");
        for id in &ids {
            assert!(
                node_brief_by_id(&store, id).unwrap().is_some(),
                "every node still reads — a merge forgets nothing"
            );
        }
    }

    /// The safety property that makes this verb worth having over
    /// attach-then-delete, stated as the contrast the report draws:
    /// `delete_initiative` on the duplicate destroys the notes in it, and a
    /// merge cannot, because memberships are added to the target before the
    /// source's rows are dropped — no node is ever briefly in no initiative.
    #[test]
    fn merge_keeps_what_delete_would_destroy() {
        let alive = |store: &Store, ids: &[String]| {
            ids.iter()
                .filter(|id| node_brief_by_id(store, id).unwrap().is_some())
                .count()
        };

        let merged = Store::open_in_memory().expect("open");
        let kept = seed_n(&merged, "duplicate", 4);
        seed_n(&merged, "canonical", 1);
        merge_initiative(&merged, "duplicate", "canonical").unwrap();
        assert_eq!(alive(&merged, &kept), 4, "a merge forgets nothing");

        let deleted = Store::open_in_memory().expect("open");
        let lost = seed_n(&deleted, "duplicate", 4);
        seed_n(&deleted, "canonical", 1);
        // Validities are whole seconds: a retract inside the second of the
        // assert cannot be ordered against it, so cross the boundary before
        // asking the substrate to forget.
        std::thread::sleep(std::time::Duration::from_millis(1100));
        delete_initiative(&deleted, "duplicate").unwrap();
        assert_eq!(
            alive(&deleted, &lost),
            0,
            "while the obvious cleanup destroys every note in the duplicate"
        );
    }

    /// A merge must not quietly widen where an initiative may be shared, so
    /// the target's policy wins and the caller is told it happened.
    #[test]
    fn the_target_keeps_its_own_share_policy() {
        let store = Store::open_in_memory().expect("open");
        seed_n(&store, "loose", 1);
        seed_n(&store, "strict", 1);
        set_share_policy(&store, "loose", SharePolicy::Team).unwrap();
        set_share_policy(&store, "strict", SharePolicy::Private).unwrap();

        let stats = merge_initiative(&store, "loose", "strict").unwrap();
        assert!(stats.policy_kept_target, "and it says so");
        assert_eq!(
            get_share_policy(&store, "strict").unwrap(),
            SharePolicy::Private,
            "merging a `team` initiative in did not open `strict`"
        );
    }

    /// The source's policy moves only into a target that has none, so a merge
    /// does not silently drop a restriction either.
    #[test]
    fn a_target_without_a_policy_inherits_the_sources() {
        let store = Store::open_in_memory().expect("open");
        seed_n(&store, "source", 1);
        seed_n(&store, "target", 1);
        set_share_policy(&store, "source", SharePolicy::Team).unwrap();

        let stats = merge_initiative(&store, "source", "target").unwrap();
        assert!(!stats.policy_kept_target);
        assert_eq!(
            get_share_policy(&store, "target").unwrap(),
            SharePolicy::Team
        );
    }

    /// `delete_initiative_impact` answers the question the verb never asked
    /// out loud: how many nodes are about to be destroyed.
    #[test]
    fn the_delete_impact_is_knowable_before_the_delete() {
        let store = Store::open_in_memory().expect("open");
        let ids = seed_n(&store, "doomed", 3);
        seed_n(&store, "other", 1);
        attach_node(&store, &ids[0], "other").unwrap();

        let impact = delete_initiative_impact(&store, "doomed").unwrap();
        assert_eq!(impact.forgotten, 2, "two live only here");
        assert_eq!(impact.unscoped, 1, "one has a second home");

        // And the prediction matches what actually happens.
        let stats = delete_initiative(&store, "doomed").unwrap();
        assert_eq!(stats.forgotten, impact.forgotten);
        assert_eq!(stats.unscoped, impact.unscoped);
    }
}

/// A namespace change includes its pointers, sharing restrictions and upkeep
/// state. Junctions are current state; node/edge history remains untouched.
fn validate_slot_merge(store: &Store, source: &str, target: &str) -> Result<()> {
    let rows = run_read(
        store,
        "?[slot] := *slot_occupant{initiative: s,slot,node_id: a},*slot_occupant{initiative: t,slot,node_id: b},s=$source,t=$target,a!=b",
        BTreeMap::from([
            ("source".into(), DataValue::Str(source.into())),
            ("target".into(), DataValue::Str(target.into())),
        ]),
    )?;
    if !rows.rows.is_empty() {
        return Err(Error::Invalid("current slots have different holders; clear or reconcile them before merging initiatives".into()));
    }
    Ok(())
}

fn move_scope_metadata(
    store: &Store,
    source: &str,
    target: &str,
    merge: bool,
    inherit_clouds: bool,
) -> Result<()> {
    let params = BTreeMap::from([
        ("source".into(), DataValue::Str(source.into())),
        ("target".into(), DataValue::Str(target.into())),
    ]);
    for (relation, keys, values) in [
        ("slot_occupant", "initiative,slot", "node_id,set_at"),
        ("api_slot_revision", "initiative,slot", "revision"),
        (
            "initiative_hygiene",
            "initiative",
            "last_run_at,nodes_at_last_run,pending_report",
        ),
    ] {
        let key_tail = keys.strip_prefix("initiative").unwrap();
        let guard = if merge {
            format!(", not *{relation}{{initiative: t{key_tail}}}, t=$target")
        } else {
            String::new()
        };
        let script = format!(
            "?[{keys},{values}] := *{relation}{{initiative: previous{key_tail},{values}}},previous=$source,initiative=$target{guard} :put {relation} {{{keys}=>{values}}}"
        );
        // Preserve the target's housekeeping on merge; node/slot revisions
        // still invalidate through membership and pointer fingerprints.
        run_mut(store, &script, params.clone())?;
    }
    if inherit_clouds {
        run_mut(
            store,
            "?[initiative,cloud] := *initiative_cloud{initiative: previous,cloud},previous=$source,initiative=$target :put initiative_cloud {initiative,cloud}",
            params,
        )?;
    }
    remove_scope_metadata(store, source)
}

fn remove_scope_metadata(store: &Store, scope: &str) -> Result<()> {
    for (relation, keys) in [
        ("slot_occupant", "initiative,slot"),
        ("api_slot_revision", "initiative,slot"),
        ("initiative_hygiene", "initiative"),
        ("initiative_cloud", "initiative,cloud"),
    ] {
        run_mut(
            store,
            &format!(
                "?[{keys}] := *{relation}{{{keys}}},initiative=$scope :rm {relation} {{{keys}}}"
            ),
            one("scope", scope),
        )?;
    }
    Ok(())
}
