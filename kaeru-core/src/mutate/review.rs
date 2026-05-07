//! Review-flow mutations: `mark_resolved` (close a question via a
//! `supersedes` edge) and `mark_under_review` (flag a target via a
//! `contradicts` edge with a fresh review-episode handle).

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;

use crate::errors::Result;
use crate::graph::NodeId;
use crate::graph::audit::write_audit;
use crate::graph::new_node_id;
use crate::store::Store;

use super::attach_edge_to_initiative;
use super::attach_node_to_initiative;
use super::build_body_tags;
use super::now_validity_seconds;
use super::tags_literal;

/// Closes an open question by recording that `by` supersedes the `question`.
///
/// Effect: a `supersedes` edge from `by` → `question` and one
/// `mark_resolved` audit event. Reads through `walk(by, [Supersedes], 1)`
/// then connect resolution to the closed question.
pub fn mark_resolved(
    store: &Store,
    question_id: &NodeId,
    by: &NodeId,
) -> Result<()> {
    let edge_secs = now_validity_seconds();
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("src".to_string(), DataValue::Str(by.clone().into()));
    params.insert("dst".to_string(), DataValue::Str(question_id.clone().into()));
    let script = format!(
        r#"
        ?[src, dst, edge_type, validity, weight, properties] <-
            [[$src, $dst, 'supersedes', [{edge_secs}.0, true], 1.0, null]]
        :put edge {{src, dst, edge_type, validity => weight, properties}}
        "#
    );
    store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Mutable)?;

    attach_edge_to_initiative(store, by, question_id, "supersedes")?;
    write_audit(
        store.db_ref(),
        "mark_resolved",
        "system",
        &[by.clone(), question_id.clone()],
    )?;
    Ok(())
}

/// Surfaces a contradiction or doubt about `target_id` without mutating
/// the target itself. Creates a high-significance review episode carrying
/// the human/agent reason and connects it to the target with a
/// `contradicts` edge.
///
/// `under_review`-flow downstream (e.g. lint reports) queries for nodes
/// that have inbound `contradicts` edges; those are the candidates for
/// resolution. The target's own content is untouched (non-destructive).
///
/// Returns the id of the review episode so the caller can hold a handle
/// for follow-up reads or further linking.
pub fn mark_under_review(
    store: &Store,
    target_id: &NodeId,
    reason: &str,
) -> Result<NodeId> {
    let review_id = new_node_id();
    let short_target = target_id.chars().take(8).collect::<String>();
    let review_name = format!("review:{short_target}");

    // Step 1 — review episode (raw insert; the wrapper audit at the end
    // covers the whole operation).
    let assert_secs = now_validity_seconds();
    let mut p1: BTreeMap<String, DataValue> = BTreeMap::new();
    p1.insert("id".to_string(), DataValue::Str(review_id.clone().into()));
    p1.insert("name".to_string(), DataValue::Str(review_name.into()));
    p1.insert("body".to_string(), DataValue::Str(reason.into()));
    let all_tags = build_body_tags(
        &["kind:observation", "sig:high", "role:review"],
        reason,
    );
    let tags = tags_literal(&all_tags);
    let s1 = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{assert_secs}.0, true], 'episode', 'operational', $name, $body, {tags}, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store.db_ref().run_script(&s1, p1, ScriptMutability::Mutable)?;

    // Step 2 — contradicts edge from review → target.
    let edge_secs = now_validity_seconds();
    let mut p2: BTreeMap<String, DataValue> = BTreeMap::new();
    p2.insert("src".to_string(), DataValue::Str(review_id.clone().into()));
    p2.insert("dst".to_string(), DataValue::Str(target_id.clone().into()));
    let s2 = format!(
        r#"
        ?[src, dst, edge_type, validity, weight, properties] <-
            [[$src, $dst, 'contradicts', [{edge_secs}.0, true], 1.0, null]]
        :put edge {{src, dst, edge_type, validity => weight, properties}}
        "#
    );
    store.db_ref().run_script(&s2, p2, ScriptMutability::Mutable)?;

    attach_node_to_initiative(store, &review_id)?;
    attach_edge_to_initiative(store, &review_id, target_id, "contradicts")?;
    write_audit(
        store.db_ref(),
        "mark_under_review",
        "system",
        &[target_id.clone(), review_id.clone()],
    )?;
    Ok(review_id)
}
