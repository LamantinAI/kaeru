//! `upsert_node` — generic, id-preserving node write.
//!
//! Used by adapters that must materialise a node under a *specific* id
//! rather than minting a fresh one — chiefly `kaeru-cloud` ingesting a
//! shared node so a local soft link (`dst = <that id>`) still resolves.
//! Unlike the typed write primitives it takes the initiative **explicitly**
//! instead of reading `Store::current_initiative`, so it is safe to call
//! concurrently from a multi-request server without racing on shared
//! session state.

use std::collections::BTreeMap;

use cozo::DataValue;
use cozo::ScriptMutability;

use crate::errors::Result;
use crate::graph::Layer;
use crate::graph::NodeId;
use crate::graph::NodeType;
use crate::graph::Tier;
use crate::graph::Visibility;
use crate::graph::audit::write_audit;
use crate::store::Store;

use super::now_validity_seconds;
use super::tags_literal;

/// Upserts a node under an explicit `id`, asserting a new bi-temporal
/// version at NOW. Attaches it to `initiative` (when given) through the
/// junction relation directly — no reliance on `Store::current_initiative`,
/// so concurrent callers don't race on shared session state.
///
/// `layer` is stored as given, so a shared node keeps its recall priority
/// when pushed to / pulled from the cloud. The node's `visibility` is stored
/// as given; a node ingested into the shared cloud is typically `Shared`.
#[allow(clippy::too_many_arguments)]
pub fn upsert_node(
    store: &Store,
    id: &NodeId,
    node_type: NodeType,
    tier: Tier,
    name: &str,
    body: Option<&str>,
    tags: &[String],
    initiative: Option<&str>,
    visibility: Visibility,
    layer: Layer,
) -> Result<()> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));
    params.insert("name".to_string(), DataValue::Str(name.into()));
    params.insert(
        "body".to_string(),
        match body {
            Some(b) => DataValue::Str(b.into()),
            None => DataValue::Null,
        },
    );

    // Tags and the Validity literal must be inlined — cozo's `<-` literal
    // rule needs concrete values for List and Validity columns. Type / tier
    // / visibility are enum `as_str()`, never attacker-controlled, so
    // inlining their quoted form is safe.
    let tags_lit = tags_literal(tags);
    let now_secs = now_validity_seconds();
    let script = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties, visibility, layer] <-
            [[$id, [{now_secs}.0, true], '{ty}', '{tier}', $name, $body, {tags_lit}, null, null, '{vis}', '{layer}']]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties, visibility, layer}}
        "#,
        ty = node_type.as_str(),
        tier = tier.as_str(),
        vis = visibility.as_str(),
        layer = layer.as_str(),
    );
    store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Mutable)?;

    if let Some(init) = initiative {
        let mut p: BTreeMap<String, DataValue> = BTreeMap::new();
        p.insert("init".to_string(), DataValue::Str(init.into()));
        p.insert("nid".to_string(), DataValue::Str(id.clone().into()));
        let junction = r#"
            ?[initiative, node_id] <- [[$init, $nid]]
            :put node_initiative {initiative, node_id}
        "#;
        store
            .db_ref()
            .run_script(junction, p, ScriptMutability::Mutable)?;
    }

    write_audit(store.db_ref(), "upsert_node", "system", &[id.clone()])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::upsert_node;
    use crate::Layer;
    use crate::NodeType;
    use crate::Tier;
    use crate::Visibility;
    use crate::get_layer;
    use crate::get_visibility;
    use crate::list_initiatives;
    use crate::node_brief_by_id;
    use crate::store::Store;

    /// Ingest preserves the supplied id verbatim (so a remote soft link
    /// resolves), stores the given visibility, and attaches the node to the
    /// named initiative.
    #[test]
    fn upsert_preserves_id_and_attaches_initiative() {
        let store = Store::open_in_memory().expect("open");
        let id = "019eccee-0000-7000-8000-000000000abc".to_string();

        upsert_node(
            &store,
            &id,
            NodeType::Idea,
            Tier::Archival,
            "shared-idea",
            Some("a settled idea promoted to the cloud"),
            &["topic:auth".to_string()],
            Some("team-proj"),
            Visibility::Shared,
            Layer::Core,
        )
        .unwrap();

        let brief = node_brief_by_id(&store, &id).unwrap().expect("present");
        assert_eq!(brief.id, id, "id preserved verbatim");
        assert_eq!(brief.node_type, "idea");
        assert_eq!(brief.name, "shared-idea");
        assert_eq!(get_visibility(&store, &id).unwrap(), Visibility::Shared);
        assert_eq!(get_layer(&store, &id).unwrap(), Layer::Core, "layer preserved");
        assert!(
            list_initiatives(&store)
                .unwrap()
                .iter()
                .any(|n| n == "team-proj"),
            "node attached to its initiative"
        );
    }
}
