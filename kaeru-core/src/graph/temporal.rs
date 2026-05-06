//! Temporal queries — bi-temporal point-in-time and history.
//!
//! The substrate stores `Validity` (timestamp + assertion flag) in node and
//! edge primary keys. This module exposes the two practical reads:
//!
//! - [`at`] — what the node looked like at a particular moment.
//! - [`history`] — every assertion / retraction recorded for a node.

use cozo::DataValue;
use cozo::ScriptMutability;
use cozo::Validity;
use std::collections::BTreeMap;

use crate::errors::Error;
use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

/// Snapshot of a node's user-visible fields at a given Unix timestamp.
#[derive(Debug, Clone, PartialEq)]
pub struct NodeSnapshot {
    pub name: String,
    pub body: Option<String>,
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

/// Returns name + body of a node as-of `at_seconds` (Unix seconds), or
/// `None` if no row was valid at that time.
pub fn at(store: &Store, id: &NodeId, at_seconds: f64) -> Result<Option<NodeSnapshot>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));

    let script = format!(
        r#"
        ?[name, body] := *node{{id, name, body @ {at_seconds}}}, id = $id
        "#
    );
    let rows = store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Immutable)?;

    let result = rows.rows.first().map(|row| {
        let name = row
            .first()
            .and_then(|v| v.get_str())
            .map(String::from)
            .unwrap_or_default();
        let body = row.get(1).and_then(|v| v.get_str()).map(String::from);
        NodeSnapshot { name, body }
    });
    Ok(result)
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
