//! Metabolism — graph hygiene mutations. `forget` retracts a node and
//! every connected edge; `improve` rewrites name/body in place via
//! retract+reassert.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;

use crate::errors::Error;
use crate::errors::Result;
use crate::graph::NodeId;
use crate::graph::audit::write_audit;
use crate::store::Store;

use super::build_body_tags;
use super::now_validity_seconds;
use super::read_connected_edges;
use super::read_type_tier_now;
use super::tags_literal;

/// Bi-temporal forget: retracts a node and every edge connected to it
/// (inbound or outbound) at NOW. Non-destructive — historical assertions
/// stay in the substrate, so `at(t)` for `t` before the forget still
/// resolves the node and its edges normally. Reads at NOW will simply
/// skip them.
///
/// Used when a node was a mistake or genuine garbage; for content the
/// agent wants to keep but rewrite, see [`improve`].
pub fn forget(store: &Store, node_id: &NodeId) -> Result<()> {
    let edges = read_connected_edges(store, node_id)?;

    // Retract each connected edge with a single timestamp; ordering of
    // retractions inside a single forget call is irrelevant.
    let edge_secs = now_validity_seconds();
    for (src, dst, edge_type) in &edges {
        let mut p: BTreeMap<String, DataValue> = BTreeMap::new();
        p.insert("src".to_string(), DataValue::Str(src.clone().into()));
        p.insert("dst".to_string(), DataValue::Str(dst.clone().into()));
        p.insert("edge_type".to_string(), DataValue::Str(edge_type.clone().into()));
        let s = format!(
            r#"
            ?[src, dst, edge_type, validity, weight, properties] <-
                [[$src, $dst, $edge_type, [{edge_secs}.0, false], 1.0, null]]
            :put edge {{src, dst, edge_type, validity => weight, properties}}
            "#
        );
        store.db_ref().run_script(&s, p, ScriptMutability::Mutable)?;
    }

    // Retract the node itself.
    let node_secs = now_validity_seconds();
    let mut p_node: BTreeMap<String, DataValue> = BTreeMap::new();
    p_node.insert("id".to_string(), DataValue::Str(node_id.clone().into()));
    let s_node = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{node_secs}.0, false], 'placeholder', 'operational', '', null, null, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store
        .db_ref()
        .run_script(&s_node, p_node, ScriptMutability::Mutable)?;

    write_audit(store.db_ref(), "forget", "system", &[node_id.clone()])?;
    Ok(())
}

/// Rewrites a node's `name` and `body` while preserving its `type`,
/// `tier`, and id. Implemented as retract + re-assert through the
/// bi-temporal substrate, so `history` shows both the old version and
/// the new one.
///
/// MVP scope: `tags`, `initiatives`, and `properties` are reset to null
/// on the new revision. Callers who need to preserve those should write
/// dedicated primitives (`retag`, `tag`, etc.) — they will land alongside
/// the broader metabolism layer.
pub fn improve(
    store: &Store,
    node_id: &NodeId,
    new_name: &str,
    new_body: &str,
) -> Result<()> {
    let (type_str, tier_str) = read_type_tier_now(store, node_id)?
        .ok_or_else(|| Error::NotFound(format!("node {node_id} not found at NOW")))?;

    // Step 1 — retract.
    let retract_secs = now_validity_seconds();
    let mut p1: BTreeMap<String, DataValue> = BTreeMap::new();
    p1.insert("id".to_string(), DataValue::Str(node_id.clone().into()));
    let s1 = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{retract_secs}.0, false], 'placeholder', 'operational', '', null, null, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store.db_ref().run_script(&s1, p1, ScriptMutability::Mutable)?;

    // Step 2 — re-assert with new name/body, preserved type/tier.
    let assert_secs = now_validity_seconds();
    let mut p2: BTreeMap<String, DataValue> = BTreeMap::new();
    p2.insert("id".to_string(), DataValue::Str(node_id.clone().into()));
    p2.insert("name".to_string(), DataValue::Str(new_name.into()));
    p2.insert("body".to_string(), DataValue::Str(new_body.into()));
    let kind_tag = format!("kind:{}", type_str);
    let role_tag = "role:revised".to_string();
    let all_tags = build_body_tags(&[kind_tag.as_str(), role_tag.as_str()], new_body);
    let tags = tags_literal(&all_tags);
    let s2 = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{assert_secs}.0, true], '{type_str}', '{tier_str}', $name, $body, {tags}, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store.db_ref().run_script(&s2, p2, ScriptMutability::Mutable)?;

    write_audit(store.db_ref(), "improve", "system", &[node_id.clone()])?;
    Ok(())
}
