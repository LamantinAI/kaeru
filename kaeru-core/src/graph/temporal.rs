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
/// time. Pass the current time to read the node as it is now.
pub fn at(store: &Store, id: &NodeId, at_seconds: f64) -> Result<Option<NodeSnapshot>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));

    let script = format!(
        r#"
        ?[type, tier, name, body, tags, layer, visibility, validity] :=
            *node{{id, type, tier, name, body, tags, layer, visibility, validity @ {at_seconds}}}, id = $id
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
        ?[validity, name, body] := *node{id, validity, name, body}, id = $id
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
    use super::at;
    use crate::store::Store;
    use crate::{EpisodeKind, Significance, node_brief_by_id, write_episode};

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
