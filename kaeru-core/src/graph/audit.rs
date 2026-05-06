//! `audit_event` — automatic write on every mutation.
//!
//! The substrate's `Validity` already records *what was* (content history).
//! Audit-event nodes record *who did it and why* — operational meta as a
//! first-class graph node, queryable by the curator for reasoning about
//! its own changes.

use cozo::DataValue;
use cozo::DbInstance;
use cozo::JsonData;
use cozo::ScriptMutability;
use serde_json::json;
use std::collections::BTreeMap;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::errors::Result;
use crate::graph::NodeId;
use crate::graph::new_node_id;

fn now_validity_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Writes an `audit_event` node for a curator operation.
/// Called by every mutation primitive. Returns the audit node id.
pub(crate) fn write_audit(
    db: &DbInstance,
    op: &str,
    actor: &str,
    affected_refs: &[String],
) -> Result<NodeId> {
    let id = new_node_id();
    let payload = json!({
        "op": op,
        "actor": actor,
        "affected_refs": affected_refs,
    });

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));
    params.insert(
        "name".to_string(),
        DataValue::Str(format!("audit:{op}").into()),
    );
    params.insert(
        "properties".to_string(),
        DataValue::Json(JsonData(payload)),
    );

    let now_secs = now_validity_seconds();
    let script = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{now_secs}.0, true], 'audit_event', 'operational', $name, null, null, null, $properties]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    db.run_script(&script, params, ScriptMutability::Mutable)?;
    Ok(id)
}
