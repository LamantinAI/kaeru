//! `synthesise` — many-to-one consolidation that preserves provenance via
//! `derived_from` edges from the new node to each seed.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;

use crate::errors::Error;
use crate::errors::Result;
use crate::graph::NodeId;
use crate::graph::NodeType;
use crate::graph::Tier;
use crate::graph::audit::write_audit;
use crate::graph::new_node_id;
use crate::store::Store;

use super::attach_edge_to_initiative;
use super::attach_node_to_initiative;
use super::now_validity_seconds;

/// Many-to-one consolidation: creates a new node carrying the synthesised
/// content and links it to each seed via `derived_from`, preserving the
/// provenance chain.
///
/// Used as the substrate primitive behind operations like:
///   - "4 web-research dossiers → one decision draft" (research consolidation),
///   - "several scratch notes → one concept" (idea formation),
///   - "many episodes → one summary" (recall compression).
///
/// Each `derived_from` edge points from the new node *to* the seed: reading
/// the new node's provenance walks `derived_from` and recovers the source
/// material. Writes one `audit_event` covering the synthesis as a whole.
pub fn synthesise(
    store: &Store,
    seeds: &[NodeId],
    target_type: NodeType,
    target_tier: Tier,
    name: &str,
    body: &str,
) -> Result<NodeId> {
    if seeds.is_empty() {
        return Err(Error::Invalid(
            "synthesise requires at least one seed node".to_string(),
        ));
    }

    let new_id = new_node_id();
    let target_type_str = target_type.as_str();
    let target_tier_str = target_tier.as_str();

    // Step 1 — assert the consolidating node.
    let assert_secs = now_validity_seconds();
    let mut p1: BTreeMap<String, DataValue> = BTreeMap::new();
    p1.insert("id".to_string(), DataValue::Str(new_id.clone().into()));
    p1.insert("name".to_string(), DataValue::Str(name.into()));
    p1.insert("body".to_string(), DataValue::Str(body.into()));
    let s1 = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{assert_secs}.0, true], '{target_type_str}', '{target_tier_str}', $name, $body, null, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store.db_ref().run_script(&s1, p1, ScriptMutability::Mutable)?;

    attach_node_to_initiative(store, &new_id)?;

    // Step 2 — derived_from edges from the new node to each seed. One
    // substrate write per seed to keep error attribution clean (a malformed
    // seed id gets caught at its own write).
    for seed in seeds {
        let edge_secs = now_validity_seconds();
        let mut p_edge: BTreeMap<String, DataValue> = BTreeMap::new();
        p_edge.insert("src".to_string(), DataValue::Str(new_id.clone().into()));
        p_edge.insert("dst".to_string(), DataValue::Str(seed.clone().into()));
        let s_edge = format!(
            r#"
            ?[src, dst, edge_type, validity, weight, properties] <-
                [[$src, $dst, 'derived_from', [{edge_secs}.0, true], 1.0, null]]
            :put edge {{src, dst, edge_type, validity => weight, properties}}
            "#
        );
        store
            .db_ref()
            .run_script(&s_edge, p_edge, ScriptMutability::Mutable)?;
        attach_edge_to_initiative(store, &new_id, seed, "derived_from")?;
    }

    // Step 3 — single audit event covering the whole synthesis.
    let mut affected = Vec::with_capacity(seeds.len() + 1);
    affected.push(new_id.clone());
    affected.extend_from_slice(seeds);
    write_audit(store.db_ref(), "synthesise", "system", &affected)?;

    Ok(new_id)
}
