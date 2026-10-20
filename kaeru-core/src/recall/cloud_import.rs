//! Bounded local work for importing the edges of one cloud node.
//!
//! A pull is not an initiative-wide reconciliation. Only edges incident to
//! the imported node can have acquired a newly available endpoint. Endpoint
//! existence is checked in batches, without rendering bodies or resolving names.

use std::collections::{BTreeMap, HashSet};

use cozo::{DataValue, ScriptMutability};

use crate::config::cloud::CLOUD_ENDPOINT_BATCH_SIZE;
use crate::models::{CloudEdge, EdgeImportResult};
use crate::{EdgeType, Error, Result, Store, link_with_weight};

/// Restore only `target`'s missing edges, retaining local weights AND local
/// retractions. A repeated pull must not undo an explicit local unlink.
/// Network access belongs to adapters; every substrate access stays here.
/// The scope guard is held for one mutation, never for the entire import.
/// Concurrent local edits must use `Store::scoped`, as adapters do; callers
/// bypassing that guard are not serialized by this helper.
pub fn restore_cloud_edges(
    store: &Store,
    initiative: &str,
    target: &str,
    edges: &[CloudEdge],
) -> Result<EdgeImportResult> {
    restore_cloud_edges_cancellable(store, initiative, target, edges, || false)
}

/// Cooperative cancellation is checked before reads and between mutations.
/// A running Cozo query is not preempted; already imported data stays valid.
pub fn restore_cloud_edges_cancellable(
    store: &Store,
    initiative: &str,
    target: &str,
    edges: &[CloudEdge],
    cancelled: impl Fn() -> bool,
) -> Result<EdgeImportResult> {
    if cancelled() {
        return Err(Error::Invalid(
            "edge import cancelled; node may already be local".into(),
        ));
    }
    let mut candidates = Vec::new();
    let mut endpoints = HashSet::new();
    let mut seen = HashSet::new();
    for (src, dst, kind, weight) in edges {
        if src != target && dst != target {
            continue;
        }
        let Ok(edge_type) = kind.parse::<EdgeType>() else {
            continue;
        };
        if src.is_empty() || dst.is_empty() || !weight.is_finite() || !(0.0..=1.0).contains(weight)
        {
            continue;
        }
        let kind = edge_type.as_str().to_string();
        if !seen.insert((src.clone(), dst.clone(), kind.clone())) {
            continue;
        }
        endpoints.insert(src.clone());
        endpoints.insert(dst.clone());
        candidates.push((src.clone(), dst.clone(), kind.clone(), edge_type, *weight));
    }
    // Do not scan local relations at all for an unrelated cloud graph.
    if candidates.is_empty() {
        return Ok(EdgeImportResult::default());
    }
    let mut params = BTreeMap::new();
    params.insert("target".to_string(), DataValue::Str(target.into()));
    // No @ modifier: a retracted local row is a decision too. Rows whose
    // destination is remote must not suppress creation of a local edge.
    let rows = store.db_ref().run_script(r#"
        ?[src, dst, edge_type] := src = $target, *edge{src, dst, edge_type, dst_store}, dst_store = 'local'
        ?[src, dst, edge_type] := dst = $target, *edge{src, dst, edge_type, dst_store}, dst_store = 'local'
    "#, params, ScriptMutability::Immutable)?;
    let known: HashSet<(String, String, String)> = rows
        .rows
        .iter()
        .filter_map(|r| {
            Some((
                r.first()?.get_str()?.into(),
                r.get(1)?.get_str()?.into(),
                r.get(2)?.get_str()?.into(),
            ))
        })
        .collect();
    let mut result = EdgeImportResult::default();
    candidates.retain(|(src, dst, kind, _, _)| {
        if known.contains(&(src.clone(), dst.clone(), kind.clone())) {
            result.kept += 1;
            false
        } else {
            true
        }
    });
    if candidates.is_empty() {
        return Ok(result);
    }
    let ids: Vec<String> = endpoints.into_iter().collect();
    let present = existing_node_ids(store, &ids)?;
    for (src, dst, kind, edge_type, weight) in candidates {
        if cancelled() {
            return Err(Error::Invalid(
                "edge import cancelled; partial import retained".into(),
            ));
        }
        if !present.contains(&src) || !present.contains(&dst) {
            continue;
        }
        // Batch reads are only a fast path. A scoped local writer may have
        // changed the edge or forgotten an endpoint while we yielded. Recheck
        // under the same guard as the write so its local decision wins.
        let outcome = store.scoped(Some(initiative), |s| -> Result<Option<bool>> {
            if cancelled() {
                return Err(Error::Invalid(
                    "edge import cancelled; partial import retained".into(),
                ));
            }
            if has_local_edge_history(s, &src, &dst, &kind)? {
                return Ok(Some(false));
            }
            if existing_node_ids(s, &[src.clone(), dst.clone()])?.len()
                != if src == dst { 1 } else { 2 }
            {
                return Ok(None);
            }
            link_with_weight(s, &src, &dst, edge_type, weight)?;
            Ok(Some(true))
        })?;
        match outcome {
            Some(true) => result.linked += 1,
            Some(false) => result.kept += 1,
            None => {}
        }
    }
    Ok(result)
}

/// A keyed historical lookup: an explicit local retraction is authoritative.
fn has_local_edge_history(store: &Store, src: &str, dst: &str, kind: &str) -> Result<bool> {
    let params = BTreeMap::from([
        ("src".into(), DataValue::Str(src.into())),
        ("dst".into(), DataValue::Str(dst.into())),
        ("kind".into(), DataValue::Str(kind.into())),
    ]);
    let rows = store.db_ref().run_script(
        "?[src] := src = $src, dst = $dst, edge_type = $kind,
         *edge{src, dst, edge_type, dst_store}, dst_store = 'local'
         :limit 1",
        params,
        ScriptMutability::Immutable,
    )?;
    Ok(!rows.rows.is_empty())
}

/// Batch, parameterised, live-node existence lookup. It deliberately ignores
/// active scope, like the old node_brief_by_id check, and never reads bodies.
pub fn existing_node_ids(store: &Store, ids: &[String]) -> Result<HashSet<String>> {
    let mut present = HashSet::new();
    for chunk in ids.chunks(CLOUD_ENDPOINT_BATCH_SIZE) {
        let mut params = BTreeMap::new();
        params.insert(
            "ids".to_string(),
            DataValue::List(
                chunk
                    .iter()
                    .map(|id| DataValue::Str(id.as_str().into()))
                    .collect(),
            ),
        );
        let rows = store.db_ref().run_script(
            "candidates[id] := id in $ids
?[id] := candidates[id], *node{id @ 'NOW'}",
            params,
            ScriptMutability::Immutable,
        )?;
        for row in rows.rows {
            if let Some(id) = row.first().and_then(|v| v.get_str()) {
                present.insert(id.to_string());
            }
        }
    }
    Ok(present)
}

#[cfg(test)]
mod tests {
    use super::{existing_node_ids, restore_cloud_edges};
    use crate::{
        EdgeType, EpisodeKind, Significance, Store, edges_of, forget, link_with_weight, unlink,
        write_episode,
    };
    fn node(s: &Store, name: &str) -> String {
        s.scoped(Some("alpha"), |s| {
            write_episode(s, EpisodeKind::Observation, Significance::Low, name, "body")
        })
        .unwrap()
    }
    #[test]
    fn cancelled_import_does_not_mutate() {
        let s = Store::open_in_memory().unwrap();
        let a = node(&s, "a");
        let b = node(&s, "b");
        let edges = vec![(a.clone(), b, "refers_to".into(), 0.8)];
        assert!(super::restore_cloud_edges_cancellable(&s, "alpha", &a, &edges, || true).is_err());
        assert!(edges_of(&s, &a).unwrap().is_empty());
    }
    #[test]
    fn cancellation_between_mutations_retains_valid_partial_import() {
        let s = Store::open_in_memory().unwrap();
        let a = node(&s, "a");
        let b = node(&s, "b");
        let c = node(&s, "c");
        let edges = vec![
            (a.clone(), b, "refers_to".into(), 0.8),
            (a.clone(), c, "refers_to".into(), 0.6),
        ];
        let result = super::restore_cloud_edges_cancellable(&s, "alpha", &a, &edges, || {
            !edges_of(&s, &a).unwrap().is_empty()
        });
        assert!(result.is_err());
        assert_eq!(edges_of(&s, &a).unwrap().len(), 1);
        // An explicit resume fills only the missing edge.
        let resumed = restore_cloud_edges(&s, "alpha", &a, &edges).unwrap();
        assert_eq!((resumed.linked, resumed.kept), (1, 1));
    }
    #[test]
    fn local_decisions_after_snapshot_win_over_import() {
        use std::cell::Cell;

        for retract in [false, true] {
            let s = Store::open_in_memory().unwrap();
            let a = node(&s, "a");
            let b = node(&s, "b");
            let checks = Cell::new(0);
            let edges = vec![(a.clone(), b.clone(), "refers_to".into(), 0.8)];
            let result = super::restore_cloud_edges_cancellable(&s, "alpha", &a, &edges, || {
                let n = checks.get() + 1;
                checks.set(n);
                // Model a scoped writer completing after the import's batch
                // snapshot but before its per-edge critical section.
                if n == 2 {
                    s.scoped(Some("alpha"), |s| {
                        if retract {
                            unlink(s, &a, &b, EdgeType::RefersTo)
                        } else {
                            link_with_weight(s, &a, &b, EdgeType::RefersTo, 0.2)
                        }
                    })
                    .unwrap();
                }
                false
            })
            .unwrap();
            assert_eq!((result.linked, result.kept), (0, 1));
            let live = edges_of(&s, &a).unwrap();
            if retract {
                assert!(live.is_empty());
            } else {
                assert_eq!(live[0].3, 0.2);
            }
        }
    }

    #[test]
    fn endpoint_forgotten_after_batch_lookup_is_not_linked() {
        use std::cell::Cell;

        let s = Store::open_in_memory().unwrap();
        let a = node(&s, "a");
        let b = node(&s, "b");
        let checks = Cell::new(0);
        let edges = vec![(a.clone(), b.clone(), "refers_to".into(), 0.8)];
        let result = super::restore_cloud_edges_cancellable(&s, "alpha", &a, &edges, || {
            let n = checks.get() + 1;
            checks.set(n);
            if n == 2 {
                s.scoped(Some("alpha"), |s| forget(s, &b)).unwrap();
            }
            false
        })
        .unwrap();
        assert_eq!(result.linked, 0);
        assert!(edges_of(&s, &a).unwrap().is_empty());
    }

    #[test]
    fn simultaneous_pulls_only_link_each_missing_edge_once() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        let s = Arc::new(Store::open_in_memory().unwrap());
        let a = node(&s, "a");
        let b = node(&s, "b");
        let edges = Arc::new(vec![(a.clone(), b, "refers_to".into(), 0.8)]);
        let barrier = Arc::new(Barrier::new(2));
        let threads: Vec<_> = (0..2)
            .map(|_| {
                let (s, edges, a, barrier) = (s.clone(), edges.clone(), a.clone(), barrier.clone());
                thread::spawn(move || {
                    let checks = std::cell::Cell::new(0);
                    super::restore_cloud_edges_cancellable(&s, "alpha", &a, &edges, || {
                        checks.set(checks.get() + 1);
                        if checks.get() == 2 {
                            barrier.wait();
                        }
                        false
                    })
                    .unwrap()
                })
            })
            .collect();
        let results: Vec<_> = threads.into_iter().map(|t| t.join().unwrap()).collect();
        assert_eq!(results.iter().map(|r| r.linked).sum::<usize>(), 1);
        assert_eq!(results.iter().map(|r| r.kept).sum::<usize>(), 1);
        assert_eq!(edges_of(&s, &a).unwrap().len(), 1);
    }

    #[test]
    fn import_uses_requested_scope_and_later_scoped_reads_keep_theirs() {
        let s = Store::open_in_memory().unwrap();
        let a = node(&s, "a");
        let b = node(&s, "b");
        s.use_initiative("beta");
        let edges = vec![(a.clone(), b, "refers_to".into(), 0.8)];
        restore_cloud_edges(&s, "alpha", &a, &edges).unwrap();
        assert_eq!(s.current_initiative().as_deref(), Some("alpha"));
        s.scoped(Some("beta"), |s| {
            assert_eq!(s.current_initiative().as_deref(), Some("beta"));
        });
    }
    #[test]
    fn preserves_weights_retractions_and_idempotence() {
        let s = Store::open_in_memory().unwrap();
        let a = node(&s, "a");
        let b = node(&s, "b");
        let c = node(&s, "c");
        let edges = vec![
            (a.clone(), b.clone(), "refers_to".into(), 0.8),
            (c.clone(), a.clone(), "refers_to".into(), 0.6),
        ];
        let first = restore_cloud_edges(&s, "alpha", &a, &edges).unwrap();
        assert_eq!((first.linked, first.kept), (2, 0));
        s.scoped(Some("alpha"), |s| {
            link_with_weight(s, &a, &b, EdgeType::RefersTo, 0.2)
        })
        .unwrap();
        s.scoped(Some("alpha"), |s| unlink(s, &c, &a, EdgeType::RefersTo))
            .unwrap();
        let again = restore_cloud_edges(&s, "alpha", &a, &edges).unwrap();
        assert_eq!((again.linked, again.kept), (0, 2));
        let live = edges_of(&s, &a).unwrap();
        assert_eq!(live.len(), 1);
        assert_eq!(live[0].3, 0.2);
    }
    #[test]
    fn skips_missing_retracted_unknown_and_unrelated_edges() {
        let s = Store::open_in_memory().unwrap();
        let a = node(&s, "a");
        let b = node(&s, "b");
        let c = node(&s, "c");
        forget(&s, &c).unwrap();
        let edges = vec![
            (a.clone(), "missing".into(), "refers_to".into(), 0.9),
            (a.clone(), c.clone(), "refers_to".into(), 0.9),
            (a.clone(), b.clone(), "future_kind".into(), 0.9),
            (b.clone(), b.clone(), "refers_to".into(), 0.9),
        ];
        assert_eq!(
            restore_cloud_edges(&s, "alpha", &a, &edges).unwrap().linked,
            0
        );
        assert_eq!(
            existing_node_ids(&s, &[a.clone(), b.clone(), c])
                .unwrap()
                .len(),
            2
        );
        // Scope is guarded per mutation; reads from a second initiative cannot
        // accidentally inherit alpha from an unguarded set-then-write sequence.
        assert!(
            s.scoped(Some("beta"), |s| crate::recall_id_by_name(s, "a"))
                .unwrap()
                .is_none()
        );
    }
    #[test]
    fn canonical_edge_types_preserve_local_decisions_and_invalid_weights_are_skipped() {
        let s = Store::open_in_memory().unwrap();
        let a = node(&s, "a");
        let b = node(&s, "b");
        let c = node(&s, "c");
        unlink(&s, &a, &b, EdgeType::RefersTo).unwrap();
        let edges = vec![
            (a.clone(), b.clone(), "REFERS-TO".into(), 0.8),
            (a.clone(), c.clone(), "refers-to".into(), 0.4),
            (a.clone(), c.clone(), "refers_to".into(), 0.9),
            (a.clone(), a.clone(), "refers_to".into(), f64::NAN),
            (a.clone(), a.clone(), "refers_to".into(), -0.1),
            (a.clone(), a.clone(), "refers_to".into(), 1.1),
        ];
        let result = restore_cloud_edges(&s, "alpha", &a, &edges).unwrap();
        assert_eq!((result.linked, result.kept), (1, 1));
        assert_eq!(
            edges_of(&s, &a).unwrap(),
            vec![(a, c, "refers_to".into(), 0.4)]
        );
    }

    #[test]
    fn endpoint_batches_cross_the_chunk_boundary() {
        let s = Store::open_in_memory().unwrap();
        let ids: Vec<_> = (0..260).map(|i| node(&s, &format!("node-{i}"))).collect();
        let mut requested = ids.clone();
        requested.extend(["missing".into(), ids[0].clone()]);
        let present = existing_node_ids(&s, &requested).unwrap();
        assert_eq!(present.len(), ids.len());
        assert!(ids.iter().all(|id| present.contains(id)));
    }

    #[test]
    fn unrelated_scale_costs_no_local_queries_and_duplicates_keep_first_weight() {
        let s = Store::open_in_memory().unwrap();
        let a = node(&s, "a");
        let b = node(&s, "b");
        let mut edges: Vec<_> = (0..2000)
            .map(|i| (format!("x{i}"), format!("y{i}"), "refers_to".into(), 0.9))
            .collect();
        edges.push((a.clone(), b.clone(), "refers_to".into(), 0.4));
        edges.push((a.clone(), b.clone(), "refers_to".into(), 0.8));
        assert_eq!(
            restore_cloud_edges(&s, "alpha", &a, &edges).unwrap().linked,
            1
        );
        assert_eq!(edges_of(&s, &a).unwrap()[0].3, 0.4);
    }
}
