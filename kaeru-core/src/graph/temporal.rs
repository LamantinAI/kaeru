//! Temporal queries — bi-temporal point-in-time and history.
//!
//! The substrate stores `Validity` (timestamp + assertion flag) in node and
//! edge primary keys. This module exposes the two practical reads:
//!
//! - [`at`] — what the node looked like at a particular moment.
//! - [`history`] — every assertion / retraction recorded for a node.

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability, Validity};

use crate::errors::{Error, Result};
use crate::graph::NodeId;
use crate::store::Store;

/// Full snapshot of a node at a given moment — every user-visible field
/// plus the **untruncated** body. `at` returns this, which makes it the
/// way to read a node *in full*: `drill` / `search` / `recall` only show
/// short body excerpts.
#[derive(Debug, Clone, PartialEq)]
pub struct NodeSnapshot {
    pub node_type: String,
    pub tier: String,
    pub name: String,
    pub body: Option<String>,
    pub tags: Vec<String>,
    pub layer: String,
    pub visibility: String,
    /// Unix seconds of the validity that was in effect at the read time —
    /// i.e. when this version of the node was asserted. `None` if the
    /// substrate returned no parseable validity.
    pub ts: Option<f64>,
}

/// One row in a node's bi-temporal history, ordered by validity.
#[derive(Debug, Clone)]
pub struct Revision {
    /// Unix seconds (float) of the validity timestamp.
    pub seconds: f64,
    /// `true` if this row is an assertion, `false` if a retraction.
    pub asserted: bool,
    pub name: String,
    pub body: Option<String>,
}

/// Returns the **full** node as-of `at_seconds` (Unix seconds) — every
/// field plus the untruncated body — or `None` if no row was valid at that
/// time. For the latest committed state use `read_node_full`: a burst of
/// revisions can allocate recorded seconds ahead of the wall-clock cutoff.
pub fn at(store: &Store, id: &NodeId, at_seconds: f64) -> Result<Option<NodeSnapshot>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));

    let script = format!(
        r#"
        ?[type, tier, name, body, tags, layer, visibility, validity] :=
            id = $id, *node{{id, type, tier, name, body, tags, layer, visibility, validity @ {at_seconds}}}
        "#
    );
    let rows = store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Immutable)?;

    let result = rows.rows.first().map(|row| {
        let s = |i: usize| {
            row.get(i)
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_default()
        };
        NodeSnapshot {
            node_type: s(0),
            tier: s(1),
            name: s(2),
            body: row.get(3).and_then(|v| v.get_str()).map(String::from),
            tags: row.get(4).map(snapshot_tags).unwrap_or_default(),
            layer: row
                .get(5)
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_else(|| "warm".to_string()),
            visibility: row
                .get(6)
                .and_then(|v| v.get_str())
                .map(String::from)
                .unwrap_or_else(|| "local".to_string()),
            ts: validity_seconds(row.get(7)),
        }
    });
    Ok(result)
}

/// Extracts a `Vec<String>` from a Cozo list column value (`tags`);
/// non-list (e.g. `null`) yields an empty vec.
fn snapshot_tags(v: &DataValue) -> Vec<String> {
    match v {
        DataValue::List(items) => items
            .iter()
            .filter_map(|x| x.get_str().map(String::from))
            .collect(),
        _ => Vec::new(),
    }
}

/// Full assertion / retraction history of a node, ordered by validity ascending.
pub fn history(store: &Store, id: &NodeId) -> Result<Vec<Revision>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));

    // Without the `@` modifier, the read sees every bi-temporal row,
    // including retractions.
    let script = r#"
        ?[validity, name, body] := id = $id, *node{id, validity, name, body}
        :order validity
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let mut revisions = Vec::with_capacity(rows.rows.len());
    for row in &rows.rows {
        let (seconds, asserted) = parse_validity(row.first())?;
        let name = row
            .get(1)
            .and_then(|v| v.get_str())
            .map(String::from)
            .unwrap_or_default();
        let body = row.get(2).and_then(|v| v.get_str()).map(String::from);
        revisions.push(Revision {
            seconds,
            asserted,
            name,
            body,
        });
    }
    revisions.sort_by(|a, b| {
        a.seconds
            .total_cmp(&b.seconds)
            .then(a.asserted.cmp(&b.asserted))
    });
    Ok(revisions)
}

/// Parses a `DataValue::Validity` into `(unix_seconds_f64, asserted)`.
///
/// Cozo's `Validity` stores the timestamp as an `i64` wrapped in `Reverse<>`.
/// When values are inserted as `[float, bool]` literals (the form every
/// kaeru mutation uses, see `mutate.rs::now_validity_seconds`), Cozo
/// preserves the integer value as-is — so stored timestamps are in
/// seconds-since-epoch on the kaeru side. Cozo's own `current_validity()`
/// (used by `@ 'NOW'`) writes microseconds; we don't read that path back
/// out, so this parser only handles the seconds-scale values we wrote.
/// Best-effort read of a validity column into Unix seconds. Returns `Some`
/// only for an **asserted** `Validity`; `None` for a retraction, a missing
/// column, or any non-Validity value — so a caller can pass a result row's
/// last column without first knowing whether it carries validity.
pub(crate) fn validity_seconds(dv: Option<&DataValue>) -> Option<f64> {
    match dv {
        Some(DataValue::Validity(Validity {
            timestamp,
            is_assert,
        })) if is_assert.0 => Some(timestamp.0.0 as f64),
        _ => None,
    }
}

pub(crate) fn parse_validity(dv: Option<&DataValue>) -> Result<(f64, bool)> {
    let dv = dv.ok_or_else(|| Error::Substrate("missing validity column".to_string()))?;
    let DataValue::Validity(Validity {
        timestamp,
        is_assert,
    }) = dv
    else {
        return Err(Error::Substrate(format!(
            "expected Validity DataValue, got {dv:?}"
        )));
    };
    let seconds = timestamp.0.0 as f64;
    Ok((seconds, is_assert.0))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use cozo::{DataValue, ScriptMutability};

    use super::at;
    use crate::store::Store;
    use crate::{
        EpisodeKind, Significance, history, node_brief_by_id, read_node_full, write_episode,
    };

    /// Verify the optimizer assumption behind all four keyed reads without
    /// a wall-clock threshold or a private performance fixture.
    #[test]
    fn bound_node_keys_use_prefix_join_plans() {
        let store = Store::open_in_memory().unwrap();
        for query in [
            "?[id,type,name,body,validity] := id = $id, *node{id,type,name,body,validity @ 'NOW'}",
            "?[type,tier,name,body,tags,visibility,layer,properties] := id = $id, *node{id,type,tier,name,body,tags,visibility,layer,properties @ 'NOW'}",
            "?[type,tier,name,body,tags,layer,visibility,validity] := id = $id, *node{id,type,tier,name,body,tags,layer,visibility,validity @ 9999999999.0}",
            "?[validity,name,body] := id = $id, *node{id,validity,name,body} :order validity",
        ] {
            let plan = store
                .db_ref()
                .run_script(
                    &format!("::explain {{ {query} }}"),
                    BTreeMap::from([("id".into(), DataValue::Str("missing".into()))]),
                    ScriptMutability::Immutable,
                )
                .unwrap();
            let op = plan.headers.iter().position(|h| h == "op").unwrap();
            assert!(
                plan.rows
                    .iter()
                    .any(|r| r[op].get_str() == Some("stored_prefix_join")),
                "keyed read did not use the primary key: {plan:?}"
            );
        }
    }

    /// Exercise the keyed readers against explicit validity boundaries, not
    /// wall-clock sleeps: historical assertions, withdrawal and resurrection.
    #[test]
    fn keyed_reads_preserve_temporal_versions_and_missing_results() {
        let store = Store::open_in_memory().unwrap();
        store.run(r#"
            rows[id, validity, type, tier, name, body, tags, visibility, layer, properties] <- [
                ['quoted\'id', [100.0, true], 'reference', 'archival', 'first', 'body one', ['one'], 'shared', 'hot', {'url': 'https://example.test/1'}],
                ['quoted\'id', [200.0, true], 'reference', 'archival', 'second', 'body two', ['two'], 'local', 'warm', {'url': 'https://example.test/2'}],
                ['quoted\'id', [300.0, false], 'reference', 'archival', 'second', 'body two', ['two'], 'local', 'warm', null],
                ['quoted\'id', [400.0, true], 'reference', 'archival', 'third', 'body three', ['three'], 'shared', 'core', {'url': 'https://example.test/3'}],
                ['withdrawn', [100.0, true], 'reference', 'archival', 'gone', 'old', [], 'local', 'warm', null],
                ['withdrawn', [200.0, false], 'reference', 'archival', 'gone', 'old', [], 'local', 'warm', null],
                ['unrelated', [100.0, true], 'reference', 'archival', 'other', 'not the requested body', [], 'local', 'warm', null]
            ]
            ?[id, validity, type, tier, name, body, tags, visibility, layer, properties, initiatives] :=
                rows[id, validity, type, tier, name, body, tags, visibility, layer, properties], initiatives = null
            :put node {id, validity => type, tier, name, body, tags, visibility, layer, properties, initiatives}
        "#).unwrap();
        let id = "quoted'id".to_string();
        for (time, name) in [
            (99.0, None),
            (100.0, Some("first")),
            (199.0, Some("first")),
            (200.0, Some("second")),
            (299.0, Some("second")),
            (300.0, None),
            (399.0, None),
            (400.0, Some("third")),
        ] {
            let snapshot = at(&store, &id, time).unwrap();
            assert_eq!(
                snapshot.as_ref().map(|n| n.name.as_str()),
                name,
                "time={time}"
            );
            if let Some(snapshot) = snapshot {
                // Compare the full old post-filter query with the keyed result.
                let params = BTreeMap::from([("id".into(), DataValue::Str(id.as_str().into()))]);
                let old = store.db_ref().run_script(&format!(
                    "?[name, body, tags, layer, visibility] := *node{{id, name, body, tags, layer, visibility @ {time}}}, id = $id"
                ), params, ScriptMutability::Immutable).unwrap();
                assert_eq!(old.rows[0][0].get_str(), Some(snapshot.name.as_str()));
                assert_eq!(old.rows[0][1].get_str(), snapshot.body.as_deref());
                assert_eq!(old.rows[0][3].get_str(), Some(snapshot.layer.as_str()));
                assert_eq!(old.rows[0][4].get_str(), Some(snapshot.visibility.as_str()));
            }
        }
        let revisions = history(&store, &id).unwrap();
        assert_eq!(
            revisions
                .iter()
                .map(|r| (r.seconds as u64, r.asserted, r.name.as_str()))
                .collect::<Vec<_>>(),
            vec![
                (100, true, "first"),
                (200, true, "second"),
                (300, false, "second"),
                (400, true, "third")
            ]
        );
        // ID readers remain deliberately cross-initiative.
        store.use_initiative("unrelated-active-scope");
        let brief = node_brief_by_id(&store, &id).unwrap().unwrap();
        let full = read_node_full(&store, &id).unwrap().unwrap();
        assert_eq!(brief.name, "third");
        assert_eq!(brief.ts, Some(400.0));
        assert_eq!(full.body.as_deref(), Some("body three"));
        assert_eq!(full.tags, vec!["three"]);
        assert_eq!(full.layer, "core");
        assert_eq!(full.visibility, "shared");
        assert_eq!(full.properties.unwrap()["url"], "https://example.test/3");
        for absent in ["missing", "withdrawn"] {
            let absent = absent.to_string();
            assert!(node_brief_by_id(&store, &absent).unwrap().is_none());
            assert!(read_node_full(&store, &absent).unwrap().is_none());
            assert!(at(&store, &absent, 999.0).unwrap().is_none());
        }
        assert!(history(&store, &"missing".into()).unwrap().is_empty());
        assert_eq!(history(&store, &"withdrawn".into()).unwrap().len(), 2);
    }

    /// A freshly written node exposes its assertion time both through `at`
    /// (the full snapshot) and through a `NodeBrief` — in Unix **seconds**,
    /// not Cozo's microsecond `@ 'NOW'` scale — and the two agree.
    #[test]
    fn at_and_brief_expose_assertion_ts_in_seconds() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        let id = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "n",
            "body",
        )
        .unwrap();

        // Read as-of the far future so the still-valid asserted row is seen.
        let snap = at(&store, &id, 9_999_999_999.0).unwrap().expect("snapshot");
        let ts = snap.ts.expect("snapshot carries a validity ts");
        assert!(
            (1_700_000_000.0..5_000_000_000.0).contains(&ts),
            "ts is Unix seconds, not micros; got {ts}"
        );

        let brief = node_brief_by_id(&store, &id).unwrap().expect("brief");
        assert_eq!(brief.ts, Some(ts), "brief ts matches snapshot ts");
    }
}
