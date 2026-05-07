//! Hypothesis-experiment cycle: `formulate_hypothesis`, `run_experiment`,
//! `update_hypothesis_status`. Status transitions are RMW (read current
//! tags, retract, re-assert with new status tag) plus an optional
//! `verifies` / `falsifies` edge.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;

use crate::errors::Error;
use crate::errors::Result;
use crate::graph::HypothesisStatus;
use crate::graph::NodeId;
use crate::graph::audit::write_audit;
use crate::graph::new_node_id;
use crate::store::Store;

use super::attach_edge_to_initiative;
use super::attach_node_to_initiative;
use super::build_body_tags;
use super::now_validity_seconds;
use super::read_name_body_now;
use super::tags_literal;

/// Creates a new hypothesis node carrying `claim` as its body.
/// Initial status is `Open` (encoded in tags). Returns the hypothesis id.
pub fn formulate_hypothesis(store: &Store, name: &str, claim: &str) -> Result<NodeId> {
    let id = new_node_id();
    let now_secs = now_validity_seconds();

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));
    params.insert("name".to_string(), DataValue::Str(name.into()));
    params.insert("body".to_string(), DataValue::Str(claim.into()));

    let all_tags = build_body_tags(&["status:open"], claim);
    let tags = tags_literal(&all_tags);
    let script = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{now_secs}.0, true], 'hypothesis', 'operational', $name, $body, {tags}, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Mutable)?;

    attach_node_to_initiative(store, &id)?;
    write_audit(store.db_ref(), "formulate_hypothesis", "system", &[id.clone()])?;
    Ok(id)
}

/// Creates an experiment node carrying `method` as its body and links it to
/// `hypothesis_id` via a `targets` edge.
pub fn run_experiment(
    store: &Store,
    hypothesis_id: &NodeId,
    name: &str,
    method: &str,
) -> Result<NodeId> {
    let id = new_node_id();
    let now_secs = now_validity_seconds();

    // Step 1 — experiment node.
    let mut p1: BTreeMap<String, DataValue> = BTreeMap::new();
    p1.insert("id".to_string(), DataValue::Str(id.clone().into()));
    p1.insert("name".to_string(), DataValue::Str(name.into()));
    p1.insert("body".to_string(), DataValue::Str(method.into()));
    let all_tags = build_body_tags(&["kind:experiment"], method);
    let tags = tags_literal(&all_tags);
    let s1 = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{now_secs}.0, true], 'experiment', 'operational', $name, $body, {tags}, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store.db_ref().run_script(&s1, p1, ScriptMutability::Mutable)?;

    // Step 2 — targets edge: experiment → hypothesis.
    let edge_secs = now_validity_seconds();
    let mut p2: BTreeMap<String, DataValue> = BTreeMap::new();
    p2.insert("src".to_string(), DataValue::Str(id.clone().into()));
    p2.insert("dst".to_string(), DataValue::Str(hypothesis_id.clone().into()));
    let s2 = format!(
        r#"
        ?[src, dst, edge_type, validity, weight, properties] <-
            [[$src, $dst, 'targets', [{edge_secs}.0, true], 1.0, null]]
        :put edge {{src, dst, edge_type, validity => weight, properties}}
        "#
    );
    store.db_ref().run_script(&s2, p2, ScriptMutability::Mutable)?;

    attach_node_to_initiative(store, &id)?;
    attach_edge_to_initiative(store, &id, hypothesis_id, "targets")?;
    write_audit(
        store.db_ref(),
        "run_experiment",
        "system",
        &[id.clone(), hypothesis_id.clone()],
    )?;
    Ok(id)
}

/// Rewrites the hypothesis with a new status tag (preserving id, name, body,
/// and other fields), and links the evidence node `by` via a `verifies` or
/// `falsifies` edge depending on the new status.
///
/// `Open` and `Inconclusive` produce no verdict edge — they just update the
/// status tag.
pub fn update_hypothesis_status(
    store: &Store,
    hypothesis_id: &NodeId,
    new_status: HypothesisStatus,
    by: &NodeId,
) -> Result<()> {
    // RMW: read current name+body so we can rewrite the row preserving them.
    let (name, body) = read_name_body_now(store, hypothesis_id)?
        .ok_or_else(|| Error::NotFound(format!("hypothesis {hypothesis_id} not found at NOW")))?;

    // Step 1 — retract current row.
    let retract_secs = now_validity_seconds();
    let mut p1: BTreeMap<String, DataValue> = BTreeMap::new();
    p1.insert("id".to_string(), DataValue::Str(hypothesis_id.clone().into()));
    let s1 = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{retract_secs}.0, false], 'hypothesis', 'operational', 'placeholder', null, null, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store.db_ref().run_script(&s1, p1, ScriptMutability::Mutable)?;

    // Step 2 — re-assert with new status tag.
    let assert_secs = now_validity_seconds();
    let status_full = format!("status:{}", new_status.as_str());
    // Compute tags first (non-consuming) so we can still move `body`
    // into the params map below.
    let all_tags: Vec<String> = match body.as_deref() {
        Some(b) => build_body_tags(&[status_full.as_str()], b),
        None => vec![status_full.clone()],
    };
    let tags = tags_literal(&all_tags);
    let mut p2: BTreeMap<String, DataValue> = BTreeMap::new();
    p2.insert("id".to_string(), DataValue::Str(hypothesis_id.clone().into()));
    p2.insert("name".to_string(), DataValue::Str(name.into()));
    match body {
        Some(b) => {
            p2.insert("body".to_string(), DataValue::Str(b.into()));
        }
        None => {
            p2.insert("body".to_string(), DataValue::Null);
        }
    }
    let s2 = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{assert_secs}.0, true], 'hypothesis', 'operational', $name, $body, {tags}, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store.db_ref().run_script(&s2, p2, ScriptMutability::Mutable)?;

    // Step 3 — verdict edge (only for Supported / Refuted).
    let verdict_edge = match new_status {
        HypothesisStatus::Supported => Some("verifies"),
        HypothesisStatus::Refuted => Some("falsifies"),
        HypothesisStatus::Open | HypothesisStatus::Inconclusive => None,
    };
    if let Some(edge_type_str) = verdict_edge {
        let edge_secs = now_validity_seconds();
        let mut p3: BTreeMap<String, DataValue> = BTreeMap::new();
        p3.insert("src".to_string(), DataValue::Str(by.clone().into()));
        p3.insert("dst".to_string(), DataValue::Str(hypothesis_id.clone().into()));
        let s3 = format!(
            r#"
            ?[src, dst, edge_type, validity, weight, properties] <-
                [[$src, $dst, '{edge_type_str}', [{edge_secs}.0, true], 1.0, null]]
            :put edge {{src, dst, edge_type, validity => weight, properties}}
            "#
        );
        store.db_ref().run_script(&s3, p3, ScriptMutability::Mutable)?;
        attach_edge_to_initiative(store, by, hypothesis_id, edge_type_str)?;
    }

    write_audit(
        store.db_ref(),
        "update_hypothesis_status",
        "system",
        &[hypothesis_id.clone(), by.clone()],
    )?;
    Ok(())
}
