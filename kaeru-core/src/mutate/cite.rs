//! `cite` — record an external reference (paper, gist, dashboard,
//! Habr article, …) as an archival-tier `Reference` node with the
//! URL stored in the `properties` JSON field for clean access.

use std::collections::BTreeMap;

use cozo::{DataValue, JsonData, ScriptMutability};
use serde_json::json;

use super::{attach_node_to_initiative, build_body_tags, now_validity_seconds, tags_literal};
use crate::errors::Result;
use crate::graph::audit::write_audit;
use crate::graph::{Layer, NodeId, new_node_id};
use crate::store::Store;

/// Creates an archival `Reference` node carrying `body` as its summary
/// and an optional `url` in `properties.url`. Returns the new node id.
///
/// `url` is optional so the same primitive covers two flavours of
/// reference: external citations (papers, gists, dashboards — pass
/// `Some(url)`) and persona / entity records (people, places, things
/// — pass `None`). Both end up in the archival tier because the
/// agent typically wants long-lived recall on them.
pub fn cite(store: &Store, name: &str, url: Option<&str>, body: &str) -> Result<NodeId> {
    cite_with_layer(store, name, url, body, Layer::default())
}

/// Creates an archival `Reference` node with an explicit memory layer.
/// The layer is stamped at creation, so the node is born with its place
/// in the recall priority order — no follow-up `set_layer` needed.
pub fn cite_with_layer(
    store: &Store,
    name: &str,
    url: Option<&str>,
    body: &str,
    layer: Layer,
) -> Result<NodeId> {
    let id = new_node_id();
    let payload = match url {
        Some(u) => json!({ "url": u }),
        None => json!({}),
    };
    let now_secs = now_validity_seconds();

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));
    params.insert("name".to_string(), DataValue::Str(name.into()));
    params.insert("body".to_string(), DataValue::Str(body.into()));
    params.insert("properties".to_string(), DataValue::Json(JsonData(payload)));
    params.insert("layer".to_string(), DataValue::Str(layer.as_str().into()));

    let all_tags = build_body_tags(&["kind:reference"], body);
    let tags = tags_literal(&all_tags);
    let script = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties, layer] <-
            [[$id, [{now_secs}.0, true], 'reference', 'archival', $name, $body, {tags}, null, $properties, $layer]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties, layer}}
        "#
    );
    store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Mutable)?;

    attach_node_to_initiative(store, &id)?;
    write_audit(store.db_ref(), "cite", "system", &[id.clone()])?;
    Ok(id)
}
