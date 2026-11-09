//! Batched live-node existence lookups without payload hydration.
use crate::config::cloud::CLOUD_ENDPOINT_BATCH_SIZE;
use crate::{Result, Store};
use cozo::{DataValue, ScriptMutability};
use std::collections::{BTreeMap, HashSet};

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
