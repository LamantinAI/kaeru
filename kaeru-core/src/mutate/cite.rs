//! `cite` — record an external reference (paper, gist, dashboard,
//! Habr article, …) as an archival-tier `Reference` node with the
//! URL stored in the `properties` JSON field for clean access.

use std::collections::BTreeMap;

use cozo::{DataValue, JsonData, ScriptMutability};
use serde_json::json;

use super::{
    attach_node_to_initiative, build_body_tags, guard_core_needs_initiative, now_validity_seconds,
    tags_literal,
};
use crate::errors::Result;
use crate::graph::audit::write_audit;
use crate::graph::{Layer, NodeId, new_node_id};
use crate::sanitize::strip_tool_call_markup;
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
    guard_core_needs_initiative(store, layer)?;

    // Scrub any leaked tool-call wire-format before it reaches the graph —
    // a malformed caller can spill the invocation envelope into these strings.
    let name_owned = strip_tool_call_markup(name).0;
    let body_owned = strip_tool_call_markup(body).0;
    let (name, body) = (name_owned.as_str(), body_owned.as_str());

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

    let all_tags = build_body_tags(&["kind:reference"], Some(name), body);
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

#[cfg(test)]
mod tests {
    use super::cite_with_layer;
    use crate::graph::{Layer, at};
    use crate::store::Store;

    /// A cite whose body carries leaked tool-call wire-format stores the
    /// scrubbed content — the write boundary defends the graph regardless of
    /// how the caller (an arbitrary LLM) formatted the call.
    #[test]
    fn cite_scrubs_leaked_markup_from_body() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        let dirty = "real content here.</body>\n<parameter name=\"initiative\">t";
        let id =
            cite_with_layer(&store, "clean-name", None, dirty, Layer::default()).expect("cite");

        let snap = at(&store, &id, 9_999_999_999.0)
            .expect("at")
            .expect("snapshot");
        assert_eq!(snap.body.as_deref(), Some("real content here."));
    }
}
