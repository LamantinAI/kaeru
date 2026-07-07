//! Review-flow mutations: `mark_resolved` (close a question via a
//! `supersedes` edge), `mark_under_review` (flag a target via a
//! `contradicts` edge with a fresh review-episode handle), and
//! `resolve_review` (close that flag non-destructively — retract the
//! `contradicts` edge, optionally recording a resolution episode).

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};

use super::{
    attach_edge_to_initiative, attach_node_to_initiative, build_body_tags, now_validity_seconds,
    tags_literal,
};
use crate::errors::Result;
use crate::graph::audit::write_audit;
use crate::graph::{NodeId, new_node_id};
use crate::store::Store;

/// Closes an open question by recording that `by` supersedes the `question`.
///
/// Effect: a `supersedes` edge from `by` → `question` and one
/// `mark_resolved` audit event. Reads through `walk(by, [Supersedes], 1)`
/// then connect resolution to the closed question.
///
/// This is the Question-closing flow — it is **not** how a
/// [`mark_under_review`] flag is cleared. The open-review queue keys on
/// inbound `contradicts` edges, which this verb never touches; use
/// [`resolve_review`] to close a review.
pub fn mark_resolved(store: &Store, question_id: &NodeId, by: &NodeId) -> Result<()> {
    let edge_secs = now_validity_seconds();
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("src".to_string(), DataValue::Str(by.clone().into()));
    params.insert(
        "dst".to_string(),
        DataValue::Str(question_id.clone().into()),
    );
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
pub fn mark_under_review(store: &Store, target_id: &NodeId, reason: &str) -> Result<NodeId> {
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
    let all_tags = build_body_tags(&["kind:observation", "sig:high", "role:review"], reason);
    let tags = tags_literal(&all_tags);
    let s1 = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{assert_secs}.0, true], 'episode', 'operational', $name, $body, {tags}, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store
        .db_ref()
        .run_script(&s1, p1, ScriptMutability::Mutable)?;

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
    store
        .db_ref()
        .run_script(&s2, p2, ScriptMutability::Mutable)?;

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

/// Closes an open review on `target_id`, non-destructively. The mirror of
/// [`mark_under_review`]: that opens a review with a reason episode + a
/// `contradicts` edge; this retracts every inbound `contradicts` edge valid at
/// NOW, so `target` leaves the open-review queue (`under_review_pinned`, `lint`,
/// `reflect`) while the doubt itself survives in bi-temporal history — a
/// `history` read or a `@ <past>` query still sees it.
///
/// When `resolution` is given, the *why* is recorded as a first-class
/// resolution episode (`role:resolution`, mirroring the review's reason
/// episode) that `supersedes` each closed review — a walkable "doubt → what
/// settled it" trail. Omit it for a bare close.
///
/// Distinct from [`mark_resolved`], which closes a Question via `supersedes`
/// and does not touch the review queue.
///
/// Returns the ids of the review episodes it closed (the `src` of each
/// retracted edge). Empty when `target` had no open review — a harmless no-op.
pub fn resolve_review(
    store: &Store,
    target_id: &NodeId,
    resolution: Option<&str>,
) -> Result<Vec<NodeId>> {
    // Step 1 — the open reviewers: sources of inbound `contradicts` valid at NOW.
    let mut rp: BTreeMap<String, DataValue> = BTreeMap::new();
    rp.insert("dst".to_string(), DataValue::Str(target_id.clone().into()));
    let read = r#"
        ?[src] := *edge{src, dst, edge_type @ 'NOW'},
                  dst = $dst, edge_type = 'contradicts'
    "#;
    let rows = store
        .db_ref()
        .run_script(read, rp, ScriptMutability::Immutable)?;
    let reviewers: Vec<NodeId> = rows
        .rows
        .iter()
        .filter_map(|row| row.first().and_then(|v| v.get_str()).map(String::from))
        .collect();
    if reviewers.is_empty() {
        return Ok(Vec::new());
    }

    // Step 2 — retract each contradicts edge (bi-temporal `[now, false]`, the
    // same mechanism as `unlink`: the assertion stays in history).
    for src in &reviewers {
        let edge_secs = now_validity_seconds();
        let mut p: BTreeMap<String, DataValue> = BTreeMap::new();
        p.insert("src".to_string(), DataValue::Str(src.clone().into()));
        p.insert("dst".to_string(), DataValue::Str(target_id.clone().into()));
        let script = format!(
            r#"
            ?[src, dst, edge_type, validity, weight, properties] <-
                [[$src, $dst, 'contradicts', [{edge_secs}.0, false], 1.0, null]]
            :put edge {{src, dst, edge_type, validity => weight, properties}}
            "#
        );
        store
            .db_ref()
            .run_script(&script, p, ScriptMutability::Mutable)?;
    }

    // Step 3 — optional resolution provenance: a resolution episode that
    // supersedes each closed review, mirroring the review's reason episode.
    if let Some(note) = resolution {
        let resolution_id = new_node_id();
        let short_target = target_id.chars().take(8).collect::<String>();
        let resolution_name = format!("resolution:{short_target}");

        let assert_secs = now_validity_seconds();
        let mut pn: BTreeMap<String, DataValue> = BTreeMap::new();
        pn.insert(
            "id".to_string(),
            DataValue::Str(resolution_id.clone().into()),
        );
        pn.insert("name".to_string(), DataValue::Str(resolution_name.into()));
        pn.insert("body".to_string(), DataValue::Str(note.into()));
        let all_tags = build_body_tags(&["kind:observation", "sig:high", "role:resolution"], note);
        let tags = tags_literal(&all_tags);
        let sn = format!(
            r#"
            ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
                [[$id, [{assert_secs}.0, true], 'episode', 'operational', $name, $body, {tags}, null, null]]
            :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
            "#
        );
        store
            .db_ref()
            .run_script(&sn, pn, ScriptMutability::Mutable)?;
        attach_node_to_initiative(store, &resolution_id)?;

        for src in &reviewers {
            let edge_secs = now_validity_seconds();
            let mut pe: BTreeMap<String, DataValue> = BTreeMap::new();
            pe.insert(
                "src".to_string(),
                DataValue::Str(resolution_id.clone().into()),
            );
            pe.insert("dst".to_string(), DataValue::Str(src.clone().into()));
            let se = format!(
                r#"
                ?[src, dst, edge_type, validity, weight, properties] <-
                    [[$src, $dst, 'supersedes', [{edge_secs}.0, true], 1.0, null]]
                :put edge {{src, dst, edge_type, validity => weight, properties}}
                "#
            );
            store
                .db_ref()
                .run_script(&se, pe, ScriptMutability::Mutable)?;
            attach_edge_to_initiative(store, &resolution_id, src, "supersedes")?;
        }
    }

    // Step 4 — one audit for the whole close (target + every review it closed).
    let mut audit_nodes = vec![target_id.clone()];
    audit_nodes.extend(reviewers.iter().cloned());
    write_audit(store.db_ref(), "resolve_review", "system", &audit_nodes)?;

    Ok(reviewers)
}
