use kaeru_core::{
    EdgeType, HypothesisStatus, Layer, NodeType, Store, Tier, Visibility, at, consolidate_out,
    formulate_hypothesis, history, jot, link_with_weight, read_node_full, supersedes,
    update_hypothesis_status, upsert_node,
};
use serde_json::json;

#[test]
fn rapid_verdicts_remain_visible_and_each_earlier_version_is_readable() {
    let store = Store::open_in_memory().unwrap();
    let hyp = formulate_hypothesis(&store, "cache-cause", "The cache is responsible.").unwrap();
    let evidence = jot(&store, "The two samples disagree.").unwrap();
    for status in [
        HypothesisStatus::Supported,
        HypothesisStatus::Refuted,
        HypothesisStatus::Inconclusive,
    ] {
        update_hypothesis_status(&store, &hyp, status, Some(&evidence)).unwrap();
        let node = read_node_full(&store, &hyp).unwrap().unwrap();
        assert!(node.tags.contains(&format!("status:{}", status.as_str())));
    }
    let versions = history(&store, &hyp).unwrap();
    assert!(
        versions
            .windows(2)
            .all(|pair| pair[0].seconds <= pair[1].seconds)
    );
    let assertions: Vec<_> = versions.iter().filter(|v| v.asserted).collect();
    assert_eq!(assertions.len(), 4);
    for (version, expected) in
        assertions
            .iter()
            .zip(["open", "supported", "refuted", "inconclusive"])
    {
        let snapshot = at(&store, &hyp, version.seconds).unwrap().unwrap();
        assert!(snapshot.tags.contains(&format!("status:{expected}")));
        assert_eq!(snapshot.body.as_deref(), Some("The cache is responsible."));
    }
    let before = read_node_full(&store, &evidence).unwrap().unwrap();
    assert!(
        update_hypothesis_status(&store, &evidence, HypothesisStatus::Supported, None).is_err()
    );
    assert_eq!(read_node_full(&store, &evidence).unwrap().unwrap(), before);
}

#[test]
fn promotion_preserves_incoming_evidence_outgoing_relations_metadata_and_weights() {
    let store = Store::open_in_memory().unwrap();
    store.scoped(Some("research"), |store| {
        let hypothesis = formulate_hypothesis(store, "cause", "The cache is responsible.").unwrap();
        let evidence = jot(store, "Cache disabled, discrepancy remains.").unwrap();
        let dependent = jot(store, "The mitigation follows this finding.").unwrap();
        let node = read_node_full(store, &hypothesis).unwrap().unwrap();
        upsert_node(
            store,
            &hypothesis,
            NodeType::Hypothesis,
            Tier::Operational,
            &node.name,
            node.body.as_deref(),
            &node.tags,
            Some("research"),
            Visibility::Local,
            Layer::Core,
            Some(&json!({"measurement": "batch-17"})),
        )
        .unwrap();
        link_with_weight(store, &evidence, &hypothesis, EdgeType::Falsifies, 0.98).unwrap();
        link_with_weight(store, &hypothesis, &dependent, EdgeType::RefersTo, 0.73).unwrap();
        let old_version = history(store, &hypothesis)
            .unwrap()
            .into_iter()
            .filter(|v| v.asserted)
            .last()
            .unwrap();
        let successor = consolidate_out(
            store,
            &hypothesis,
            NodeType::Outcome,
            "cause",
            "The cache is responsible.",
        )
        .unwrap();
        let promoted = read_node_full(store, &successor).unwrap().unwrap();
        assert_eq!(
            promoted.properties,
            Some(json!({"measurement": "batch-17"}))
        );
        assert_eq!(promoted.layer, "core");
        let edges = store
            .run_read(&format!(
                "?[src, dst, edge_type, weight] := *edge{{src, dst, edge_type, weight @ 'NOW'}}, \
             dst = '{successor}' ?[src, dst, edge_type, weight] := \
             *edge{{src, dst, edge_type, weight @ 'NOW'}}, src = '{successor}'"
            ))
            .unwrap();
        assert!(edges.rows.iter().any(|r| r[0].get_str() == Some(&evidence)
            && r[2].get_str() == Some("falsifies")
            && r[3].get_float() == Some(0.98)));
        assert!(edges.rows.iter().any(|r| r[1].get_str() == Some(&dependent)
            && r[2].get_str() == Some("refers_to")
            && r[3].get_float() == Some(0.73)));
        assert!(read_node_full(store, &hypothesis).unwrap().is_none());
        assert_eq!(
            at(store, &hypothesis, old_version.seconds)
                .unwrap()
                .unwrap()
                .body
                .as_deref(),
            Some("The cache is responsible.")
        );
    });
}

#[test]
fn changed_content_does_not_inherit_the_predecessors_endorsement() {
    let store = Store::open_in_memory().unwrap();
    let old = jot(&store, "Three attempts.").unwrap();
    let evidence = jot(&store, "Measured three attempts.").unwrap();
    link_with_weight(&store, &evidence, &old, EdgeType::Verifies, 0.9).unwrap();
    let new = supersedes(
        &store,
        &old,
        NodeType::Episode,
        Tier::Operational,
        "retry-limit-v2",
        "Five attempts.",
    )
    .unwrap();
    let endorsements = store
        .run_read(&format!(
            "?[src] := *edge{{src, dst, edge_type @ 'NOW'}}, dst = '{new}', edge_type = 'verifies'"
        ))
        .unwrap();
    assert!(endorsements.rows.is_empty());
    assert_eq!(
        history(&store, &old)
            .unwrap()
            .iter()
            .filter(|r| r.asserted)
            .count(),
        1
    );
}
