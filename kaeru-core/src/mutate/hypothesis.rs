//! Hypothesis-experiment cycle: `formulate_hypothesis`, `run_experiment`,
//! `update_hypothesis_status`. Status transitions are RMW (read current
//! tags, retract, re-assert with new status tag) plus an optional
//! `verifies` / `falsifies` edge.

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};

use super::{
    ReassertRow, attach_edge_to_initiative, attach_node_to_initiative, build_body_tags, merge_tags,
    now_validity_seconds, read_node_now, reassert_node_now, retract_node_at, tags_literal,
};
use crate::errors::{Error, Result};
use crate::graph::audit::write_audit;
use crate::graph::{HypothesisStatus, Layer, NodeId, new_node_id};
use crate::store::Store;

/// Creates a new hypothesis node carrying `claim` as its body.
/// Initial status is `Open` (encoded in tags). Returns the hypothesis id.
pub fn formulate_hypothesis(store: &Store, name: &str, claim: &str) -> Result<NodeId> {
    formulate_hypothesis_with_layer(store, name, claim, Layer::default())
}

/// Creates a hypothesis node with an explicit memory layer, stamped at
/// creation so the claim is born in the right recall priority band.
pub fn formulate_hypothesis_with_layer(
    store: &Store,
    name: &str,
    claim: &str,
    layer: Layer,
) -> Result<NodeId> {
    let id = new_node_id();
    let now_secs = now_validity_seconds();

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));
    params.insert("name".to_string(), DataValue::Str(name.into()));
    params.insert("body".to_string(), DataValue::Str(claim.into()));
    params.insert("layer".to_string(), DataValue::Str(layer.as_str().into()));

    let all_tags = build_body_tags(&["status:open"], claim);
    let tags = tags_literal(&all_tags);
    let script = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties, layer] <-
            [[$id, [{now_secs}.0, true], 'hypothesis', 'operational', $name, $body, {tags}, null, null, $layer]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties, layer}}
        "#
    );
    store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Mutable)?;

    attach_node_to_initiative(store, &id)?;
    write_audit(
        store.db_ref(),
        "formulate_hypothesis",
        "system",
        &[id.clone()],
    )?;
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
    store
        .db_ref()
        .run_script(&s1, p1, ScriptMutability::Mutable)?;

    // Step 2 — targets edge: experiment → hypothesis.
    let edge_secs = now_validity_seconds();
    let mut p2: BTreeMap<String, DataValue> = BTreeMap::new();
    p2.insert("src".to_string(), DataValue::Str(id.clone().into()));
    p2.insert(
        "dst".to_string(),
        DataValue::Str(hypothesis_id.clone().into()),
    );
    let s2 = format!(
        r#"
        ?[src, dst, edge_type, validity, weight, properties] <-
            [[$src, $dst, 'targets', [{edge_secs}.0, true], 1.0, null]]
        :put edge {{src, dst, edge_type, validity => weight, properties}}
        "#
    );
    store
        .db_ref()
        .run_script(&s2, p2, ScriptMutability::Mutable)?;

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

/// Rewrites the hypothesis with a new status tag — preserving id, name,
/// body, `layer`, `visibility`, `properties`, and manual tags — and links
/// the evidence node `by` via a `verifies` or `falsifies` edge depending on
/// the new status.
///
/// `Open` and `Inconclusive` produce no verdict edge — they just update the
/// status tag.
pub fn update_hypothesis_status(
    store: &Store,
    hypothesis_id: &NodeId,
    new_status: HypothesisStatus,
    by: &NodeId,
) -> Result<()> {
    // RMW: read the current row so the rewrite preserves everything the
    // status transition doesn't touch (name, body, layer, visibility,
    // properties, manual tags).
    let current = read_node_now(store, hypothesis_id)?
        .ok_or_else(|| Error::NotFound(format!("hypothesis {hypothesis_id} not found at NOW")))?;

    let status_full = format!("status:{}", new_status.as_str());
    let fresh: Vec<String> = match current.body.as_deref() {
        Some(b) => build_body_tags(&[status_full.as_str()], b),
        None => vec![status_full.clone()],
    };
    let tags = merge_tags(&current.tags, &["status:", "lang:", "topic:"], fresh);

    // Re-assert first, retract second, same timestamp — see
    // `reassert_node_now` for the ordering invariant.
    let secs = now_validity_seconds();
    reassert_node_now(
        store,
        hypothesis_id,
        ReassertRow {
            secs,
            type_: &current.type_,
            tier: &current.tier,
            name: &current.name,
            body: current.body.as_deref(),
            tags,
            visibility: &current.visibility,
            layer: &current.layer,
        },
    )?;
    retract_node_at(store, hypothesis_id, secs)?;

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
        p3.insert(
            "dst".to_string(),
            DataValue::Str(hypothesis_id.clone().into()),
        );
        let s3 = format!(
            r#"
            ?[src, dst, edge_type, validity, weight, properties] <-
                [[$src, $dst, '{edge_type_str}', [{edge_secs}.0, true], 1.0, null]]
            :put edge {{src, dst, edge_type, validity => weight, properties}}
            "#
        );
        store
            .db_ref()
            .run_script(&s3, p3, ScriptMutability::Mutable)?;
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::graph::{HypothesisStatus, Layer, Visibility};
    use crate::store::Store;
    use crate::{
        at, formulate_hypothesis_with_layer, jot, set_visibility, update_hypothesis_status,
    };

    /// The status transition used to re-assert with an incomplete column
    /// list, resetting `layer` / `visibility` to schema defaults on every
    /// `confirm` / `refute`.
    #[test]
    fn confirm_preserves_layer_and_visibility() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        let hyp = formulate_hypothesis_with_layer(&store, "h1", "the cache is stale", Layer::Hot)
            .expect("claim");
        set_visibility(&store, &hyp, Visibility::Shared).expect("set vis");
        let evidence = jot(&store, "repro log attached").expect("jot");

        std::thread::sleep(Duration::from_millis(1100));
        update_hypothesis_status(&store, &hyp, HypothesisStatus::Supported, &evidence)
            .expect("confirm");

        let snap = at(&store, &hyp, 9_999_999_999.0)
            .expect("at")
            .expect("still resolves");
        assert_eq!(snap.layer, "hot", "layer survives confirm");
        assert_eq!(snap.visibility, "shared", "visibility survives confirm");
        assert!(snap.tags.iter().any(|t| t == "status:supported"));
        assert!(
            !snap.tags.iter().any(|t| t == "status:open"),
            "old status dropped: {:?}",
            snap.tags
        );
    }
}
