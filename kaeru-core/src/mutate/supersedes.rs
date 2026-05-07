//! `supersedes` — replaces an old node with a freshly-asserted one,
//! connected via a `supersedes` edge.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;

use crate::errors::Result;
use crate::graph::NodeId;
use crate::graph::NodeType;
use crate::graph::Tier;
use crate::graph::audit::write_audit;
use crate::graph::new_node_id;
use crate::store::Store;

use super::attach_edge_to_initiative;
use super::attach_node_to_initiative;
use super::build_body_tags;
use super::now_validity_seconds;
use super::tags_literal;

/// Replaces `old_id` with a freshly-asserted node carrying the new content,
/// connected to the old by a `supersedes` edge.
///
/// Three substrate writes happen in sequence:
///  1. retract `old_id` (assertion = false at now);
///  2. assert a new node with a new id at now;
///  3. write a `supersedes` edge from old → new.
/// Followed by one `audit_event` capturing the operation as a whole.
///
/// Reads through `at(t)` for `t` *after* the supersedes will resolve through
/// the substrate's bi-temporal mechanics: `old_id` reads as nothing,
/// `new_id` reads as the new content. Earlier `t` still resolves the old.
///
/// Note: the three writes are not atomic at the substrate level. A failure
/// between steps leaves the graph in an intermediate state — recoverable
/// through `lint`, but not transparent. A transactional path is a future
/// improvement.
pub fn supersedes(
    store: &Store,
    old_id: &NodeId,
    new_type: NodeType,
    new_tier: Tier,
    new_name: &str,
    new_body: &str,
) -> Result<NodeId> {
    let new_id = new_node_id();

    // Step 1 — retract old. Required non-key fields get placeholder values;
    // they are never observable because the row is a retraction.
    let retract_secs = now_validity_seconds();
    let mut p1: BTreeMap<String, DataValue> = BTreeMap::new();
    p1.insert("old_id".to_string(), DataValue::Str(old_id.clone().into()));
    let s1 = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$old_id, [{retract_secs}, false], 'placeholder', 'operational', '', null, null, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store.db_ref().run_script(&s1, p1, ScriptMutability::Mutable)?;

    // Step 2 — assert new node.
    let assert_secs = now_validity_seconds();
    let mut p2: BTreeMap<String, DataValue> = BTreeMap::new();
    p2.insert("id".to_string(), DataValue::Str(new_id.clone().into()));
    p2.insert("name".to_string(), DataValue::Str(new_name.into()));
    p2.insert("body".to_string(), DataValue::Str(new_body.into()));
    let kind_tag = format!("kind:{}", new_type.as_str());
    let all_tags = build_body_tags(&[kind_tag.as_str()], new_body);
    let tags = tags_literal(&all_tags);
    let s2 = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{assert_secs}, true], '{}', '{}', $name, $body, {tags}, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#,
        new_type.as_str(),
        new_tier.as_str(),
    );
    store.db_ref().run_script(&s2, p2, ScriptMutability::Mutable)?;

    // Step 3 — supersedes edge. Inlined here to avoid the inner audit that
    // `link` would write; this whole operation gets one audit at the end.
    let edge_secs = now_validity_seconds();
    let mut p3: BTreeMap<String, DataValue> = BTreeMap::new();
    p3.insert("src".to_string(), DataValue::Str(old_id.clone().into()));
    p3.insert("dst".to_string(), DataValue::Str(new_id.clone().into()));
    let s3 = format!(
        r#"
        ?[src, dst, edge_type, validity, weight, properties] <-
            [[$src, $dst, 'supersedes', [{edge_secs}, true], 1.0, null]]
        :put edge {{src, dst, edge_type, validity => weight, properties}}
        "#
    );
    store.db_ref().run_script(&s3, p3, ScriptMutability::Mutable)?;

    attach_node_to_initiative(store, &new_id)?;
    attach_edge_to_initiative(store, old_id, &new_id, "supersedes")?;
    write_audit(
        store.db_ref(),
        "supersedes",
        "system",
        &[old_id.clone(), new_id.clone()],
    )?;
    Ok(new_id)
}
