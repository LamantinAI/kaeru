//! `cite` — record an external reference (paper, gist, dashboard,
//! Habr article, …) as an archival-tier `Reference` node with the
//! URL stored in the `properties` JSON field for clean access.

use cozo::DataValue;
use cozo::JsonData;
use cozo::ScriptMutability;
use serde_json::json;
use std::collections::BTreeMap;

use crate::errors::Result;
use crate::graph::NodeId;
use crate::graph::audit::write_audit;
use crate::graph::new_node_id;
use crate::store::Store;

use super::attach_node_to_initiative;
use super::build_body_tags;
use super::now_validity_seconds;
use super::tags_literal;

/// Creates an archival `Reference` node carrying `body` as its summary
/// and `url` in `properties.url`. Returns the new node id.
pub fn cite(store: &Store, name: &str, url: &str, body: &str) -> Result<NodeId> {
    let id = new_node_id();
    let payload = json!({ "url": url });
    let now_secs = now_validity_seconds();

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));
    params.insert("name".to_string(), DataValue::Str(name.into()));
    params.insert("body".to_string(), DataValue::Str(body.into()));
    params.insert(
        "properties".to_string(),
        DataValue::Json(JsonData(payload)),
    );

    let all_tags = build_body_tags(&["kind:reference"], body);
    let tags = tags_literal(&all_tags);
    let script = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{now_secs}.0, true], 'reference', 'archival', $name, $body, {tags}, null, $properties]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Mutable)?;

    attach_node_to_initiative(store, &id)?;
    write_audit(store.db_ref(), "cite", "system", &[id.clone()])?;
    Ok(id)
}
