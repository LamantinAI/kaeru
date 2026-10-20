use kaeru_core::config::amg::AmgLimits;
use kaeru_core::models::amg::AmgSelection;
use kaeru_core::{
    Store, amg,
    api::{ApiOptions, ResponseData, execute},
};
use serde_json::json;

fn note(store: &Store, initiative: &str, name: &str, body: &str) -> String {
    let response = execute(
        store,
        "note",
        json!({"initiative":initiative,"name":name,"body":body,"kind":"observation"}),
        &ApiOptions::local(),
    );
    match response.data {
        ResponseData::Mutation(data) => data.primary.expect("created").id,
        other => panic!("note failed: {other:?}"),
    }
}
#[test]
fn current_nodes_roundtrip_and_repeat_without_new_versions() {
    let source = Store::open_in_memory().unwrap();
    let id = note(&source, "alpha", "Node: one", "body: with: colons");
    let limits = AmgLimits::default();
    let doc = amg::snapshot(&source, &AmgSelection::All, &limits).unwrap();
    let encoded = amg::encode(&doc, &limits).unwrap();
    assert_eq!(&encoded[..4], b"amg:");
    let decoded = amg::decode(encoded.as_slice(), &limits).unwrap();
    assert_eq!(decoded, doc);
    let target = Store::open_in_memory().unwrap();
    let result = amg::import(&target, &decoded, false, false, &limits).unwrap();
    assert_eq!(result.created_nodes, 1);
    let n = kaeru_core::read_node_full(&target, &id).unwrap().unwrap();
    assert_eq!(n.body.as_deref(), Some("body: with: colons"));
    let before = kaeru_core::history(&target, &id).unwrap();
    let again = amg::import(&target, &decoded, false, false, &limits).unwrap();
    assert_eq!(again.updated_nodes, 0);
    assert_eq!(again.created_nodes, 0);
    assert_eq!(again.updated_metadata, 0);
    assert_eq!(
        kaeru_core::history(&target, &id).unwrap().len(),
        before.len()
    );
    let exported = amg::snapshot(&target, &AmgSelection::All, &limits).unwrap();
    assert_eq!(exported.data.nodes.len(), 1);
    assert_eq!(exported.data.nodes[0].origins.len(), 1);
}
#[test]
fn edge_weight_and_properties_survive_exactly() {
    let source = Store::open_in_memory().unwrap();
    let a = note(&source, "alpha", "A", "a");
    let b = note(&source, "alpha", "B", "b");
    kaeru_core::link_with_weight(&source, &a, &b, kaeru_core::EdgeType::DerivedFrom, 0.25).unwrap();
    let limits = AmgLimits::default();
    let mut doc = amg::snapshot(&source, &AmgSelection::All, &limits).unwrap();
    doc.data.edges[0].weight = 0.123456789123;
    doc.data.edges[0].properties = json!({"rationale":"important: exact"});
    let target = Store::open_in_memory().unwrap();
    amg::import(&target, &doc, false, false, &limits).unwrap();
    let exported = amg::snapshot(&target, &AmgSelection::All, &limits).unwrap();
    assert_eq!(exported.data.edges[0].weight, 0.123456789123);
    assert_eq!(
        exported.data.edges[0].properties,
        doc.data.edges[0].properties
    );
}

#[test]
fn force_adds_a_current_version_and_preserves_history_and_tombstones() {
    let source = Store::open_in_memory().unwrap();
    let id = note(&source, "alpha", "A", "source version");
    let limits = AmgLimits::default();
    let original = amg::snapshot(&source, &AmgSelection::All, &limits).unwrap();
    let target = Store::open_in_memory().unwrap();
    amg::import(&target, &original, false, false, &limits).unwrap();
    let mut next = original.clone();
    next.data.nodes[0].body = Some("forced current".into());
    next.data.nodes[0].properties = json!({"extra":true});
    let before = kaeru_core::history(&target, &id).unwrap();
    assert!(amg::import(&target, &next, false, false, &limits).is_err());
    assert_eq!(
        kaeru_core::history(&target, &id).unwrap().len(),
        before.len()
    );
    let forced = amg::import(&target, &next, true, false, &limits).unwrap();
    assert_eq!(forced.updated_nodes, 1);
    assert_eq!(
        kaeru_core::read_node_full(&target, &id)
            .unwrap()
            .unwrap()
            .body
            .as_deref(),
        Some("forced current")
    );
    let history = kaeru_core::history(&target, &id).unwrap();
    assert!(
        history
            .iter()
            .any(|r| r.body.as_deref() == Some("source version"))
    );
    assert!(history.last().unwrap().seconds > before.last().unwrap().seconds);
    kaeru_core::forget(&target, &id).unwrap();
    assert!(kaeru_core::read_node_full(&target, &id).unwrap().is_none());
    assert!(amg::import(&target, &original, false, false, &limits).is_err());
    amg::import(&target, &original, true, false, &limits).unwrap();
    assert_eq!(
        kaeru_core::read_node_full(&target, &id)
            .unwrap()
            .unwrap()
            .body
            .as_deref(),
        Some("source version")
    );
    let h = kaeru_core::history(&target, &id).unwrap();
    assert!(h.iter().any(|r| !r.asserted));
    let unchanged = amg::import(&target, &original, true, false, &limits).unwrap();
    assert_eq!(unchanged.updated_nodes, 0);
    assert_eq!(kaeru_core::history(&target, &id).unwrap().len(), h.len());
}

#[test]
fn one_and_multiple_initiatives_keep_only_closed_edges_and_memberships() {
    let source = Store::open_in_memory().unwrap();
    let a = note(&source, "alpha", "A", "alpha");
    let b = note(&source, "beta", "B", "beta");
    let both = note(&source, "alpha", "Shared", "shared");
    kaeru_core::attach_node(&source, &both, "secret").unwrap();
    source
        .scoped(Some("alpha"), |s| {
            kaeru_core::link(s, &a, &both, kaeru_core::EdgeType::RefersTo)
        })
        .unwrap();
    source
        .scoped(Some("alpha"), |s| {
            kaeru_core::link(s, &a, &b, kaeru_core::EdgeType::DerivedFrom)
        })
        .unwrap();
    let limits = AmgLimits::default();
    let one = amg::snapshot(
        &source,
        &AmgSelection::Initiatives(vec!["alpha".into()]),
        &limits,
    )
    .unwrap();
    assert_eq!(one.data.nodes.len(), 2);
    assert_eq!(one.data.edges.len(), 1);
    assert!(
        one.data
            .nodes
            .iter()
            .all(|n| n.initiatives == vec!["alpha"])
    );
    assert!(!String::from_utf8_lossy(&amg::encode(&one, &limits).unwrap()).contains("secret"));
    let two = amg::snapshot(
        &source,
        &AmgSelection::Initiatives(vec!["alpha".into(), "beta".into()]),
        &limits,
    )
    .unwrap();
    assert_eq!(two.data.nodes.len(), 3);
    assert_eq!(two.data.edges.len(), 2);
    assert!(amg::snapshot(&source, &AmgSelection::Initiatives(vec![]), &limits).is_err());
    assert!(
        amg::snapshot(
            &source,
            &AmgSelection::Initiatives(vec!["alpah".into()]),
            &limits
        )
        .is_err()
    );
}

#[test]
fn conflict_late_in_a_bundle_and_dry_run_leave_no_partial_nodes() {
    let source = Store::open_in_memory().unwrap();
    let one = note(&source, "alpha", "A", "old");
    let two = note(&source, "alpha", "B", "another");
    let limits = AmgLimits::default();
    let doc = amg::snapshot(&source, &AmgSelection::All, &limits).unwrap();
    let target = Store::open_in_memory().unwrap();
    let dry = amg::import(&target, &doc, false, true, &limits).unwrap();
    assert_eq!(dry.created_nodes, 2);
    assert!(kaeru_core::read_node_full(&target, &one).unwrap().is_none());
    let mut existing = doc.clone();
    existing.data.nodes.retain(|n| n.id == two);
    existing.data.nodes[0].body = Some("local".into());
    amg::import(&target, &existing, false, false, &limits).unwrap();
    assert!(amg::import(&target, &doc, false, false, &limits).is_err());
    assert!(kaeru_core::read_node_full(&target, &one).unwrap().is_none());
    assert_eq!(
        kaeru_core::read_node_full(&target, &two)
            .unwrap()
            .unwrap()
            .body
            .as_deref(),
        Some("local")
    );
}

#[test]
fn properties_clear_and_local_memberships_and_sharing_policy_survive() {
    let source = Store::open_in_memory().unwrap();
    let id = note(&source, "alpha", "A", "a");
    let limits = AmgLimits::default();
    let mut doc = amg::snapshot(&source, &AmgSelection::All, &limits).unwrap();
    doc.data.nodes[0].properties = json!({"will":"clear"});
    doc.data.nodes[0].visibility = "shared".into();
    let target = Store::open_in_memory().unwrap();
    kaeru_core::set_share_policy(&target, "alpha", kaeru_core::SharePolicy::Team).unwrap();
    amg::import(&target, &doc, false, false, &limits).unwrap();
    assert_eq!(
        kaeru_core::get_share_policy(&target, "alpha").unwrap(),
        kaeru_core::SharePolicy::Team
    );
    assert_eq!(
        kaeru_core::read_node_full(&target, &id)
            .unwrap()
            .unwrap()
            .visibility,
        "local"
    );
    kaeru_core::attach_node(&target, &id, "other-local").unwrap();
    doc.data.nodes[0].properties = serde_json::Value::Null;
    amg::import(&target, &doc, true, false, &limits).unwrap();
    let state = amg::snapshot(&target, &AmgSelection::All, &limits).unwrap();
    let n = state.data.nodes.iter().find(|n| n.id == id).unwrap();
    assert!(n.properties.is_null());
    assert!(n.initiatives.contains(&"other-local".into()));
}

#[test]
fn immutable_sources_validate_hashes_and_force_mints_an_administrative_revision() {
    let source = Store::open_in_memory().unwrap();
    let response = execute(
        &source,
        "reference",
        json!({"initiative":"alpha","name":"Exact source","source":{"text":"original"}}),
        &ApiOptions::local(),
    );
    let id = match response.data {
        ResponseData::Mutation(data) => data.primary.unwrap().id,
        other => panic!("{other:?}"),
    };
    let limits = AmgLimits::default();
    let mut doc = amg::snapshot(&source, &AmgSelection::All, &limits).unwrap();
    doc.data.nodes[0].properties["source"]["origin"] = json!("verified_artifact");
    doc.data.nodes[0].properties["source"]["artifact"] = json!("/this/path/must/not/be/read");
    let target = Store::open_in_memory().unwrap();
    amg::import(&target, &doc, false, false, &limits).unwrap();
    let imported = amg::snapshot(&target, &AmgSelection::All, &limits).unwrap();
    assert_eq!(
        imported.data.nodes[0].properties["source"]["origin"],
        "imported_artifact"
    );
    let mut bad = doc.clone();
    bad.data.nodes[0].body = Some("tampered".into());
    assert!(amg::import(&target, &bad, true, false, &limits).is_err());
    use sha2::{Digest, Sha256};
    let body = "a new exact source";
    doc.data.nodes[0].body = Some(body.into());
    doc.data.nodes[0].properties["source"]["original"] = json!(body);
    doc.data.nodes[0].properties["source"]["sha256"] = json!(format!("{:x}", Sha256::digest(body)));
    amg::import(&target, &doc, true, false, &limits).unwrap();
    let history = kaeru_core::history(&target, &id).unwrap();
    assert!(
        history
            .iter()
            .any(|r| r.body.as_deref() == Some("original"))
    );
    assert!(history.iter().any(|r| r.body.as_deref() == Some(body)));
}

fn mutation(store: &Store, operation: &str, args: serde_json::Value) -> String {
    let r = execute(store, operation, args, &ApiOptions::local());
    match r.data {
        ResponseData::Mutation(data) => {
            data.primary
                .unwrap_or_else(|| panic!("{operation}: {:?}", r.issues))
                .id
        }
        other => panic!("{operation}: {other:?}"),
    }
}

#[test]
fn ordered_trails_slots_and_staleness_survive_roundtrip() {
    let source = Store::open_in_memory().unwrap();
    let a = note(&source, "alpha", "А", "наблюдение");
    let b = note(&source, "alpha", "Б", "решение");
    source
        .scoped(Some("alpha"), |s| {
            kaeru_core::link(s, &a, &b, kaeru_core::EdgeType::DerivedFrom)
        })
        .unwrap();
    let chain = mutation(
        &source,
        "trail",
        json!({"initiative":"alpha","name":"Reasoning","route":{"members":[a,b]},"summary":"A supports B"}),
    );
    kaeru_core::occupy_slot(&source, "alpha", "handoff", &b).unwrap();
    // The stored summary becomes stale when a member changes.
    kaeru_core::improve(&source, &a, "А", "changed observation").unwrap();
    let limits = AmgLimits::default();
    let doc = amg::snapshot(&source, &AmgSelection::All, &limits).unwrap();
    assert_eq!(doc.data.trails[0].stale, Some(true));
    let target = Store::open_in_memory().unwrap();
    amg::import(&target, &doc, false, false, &limits).unwrap();
    let copied = amg::snapshot(&target, &AmgSelection::All, &limits).unwrap();
    let trail = copied.data.trails.iter().find(|t| t.id == chain).unwrap();
    assert_eq!(trail.members, vec![a, b.clone()]);
    assert_eq!(trail.stale, Some(true));
    assert_eq!(
        kaeru_core::slot_holder(&target, "alpha", "handoff").unwrap(),
        Some(b)
    );
    let no_op = amg::import(&target, &doc, false, false, &limits).unwrap();
    assert_eq!(no_op.updated_metadata, 0);
    assert_eq!(no_op.updated_nodes, 0);
}

#[test]
fn partial_trails_are_omitted_instead_of_splicing_a_new_reasoning_path() {
    let source = Store::open_in_memory().unwrap();
    let a = note(&source, "alpha", "A", "a");
    let b = note(&source, "alpha", "B", "b");
    let c = note(&source, "alpha", "C", "c");
    source
        .scoped(Some("alpha"), |s| {
            kaeru_core::link(s, &a, &b, kaeru_core::EdgeType::DerivedFrom)
        })
        .unwrap();
    source
        .scoped(Some("alpha"), |s| {
            kaeru_core::link(s, &b, &c, kaeru_core::EdgeType::DerivedFrom)
        })
        .unwrap();
    let chain = mutation(
        &source,
        "trail",
        json!({"initiative":"alpha","name":"Path","route":{"members":[a,b,c]},"summary":"ABC"}),
    );
    // Put the trail and endpoints in another slice, leaving its middle member outside.
    kaeru_core::attach_node(&source, &a, "slice").unwrap();
    kaeru_core::attach_node(&source, &c, "slice").unwrap();
    kaeru_core::attach_node(&source, &chain, "slice").unwrap();
    let doc = amg::snapshot(
        &source,
        &AmgSelection::Initiatives(vec!["slice".into()]),
        &AmgLimits::default(),
    )
    .unwrap();
    assert!(doc.data.trails.is_empty());
    assert!(doc.data.nodes.iter().all(|n| n.id != chain));
    assert!(
        doc.metadata
            .omissions
            .iter()
            .any(|o| o.reason == "trail_outside_scope" && o.count == 1)
    );
}

#[test]
fn verdicts_bind_to_the_new_local_version_and_keep_their_evidence() {
    let source = Store::open_in_memory().unwrap();
    let evidence = note(&source, "alpha", "Evidence", "manual check");
    let claim = mutation(
        &source,
        "claim",
        json!({"initiative":"alpha","text":"Claim with evidence","initial_status":"supported","evidence":{"node":evidence}}),
    );
    let limits = AmgLimits::default();
    let mut doc = amg::snapshot(&source, &AmgSelection::All, &limits).unwrap();
    assert_eq!(doc.data.decisions.len(), 1);
    let target = Store::open_in_memory().unwrap();
    amg::import(&target, &doc, false, false, &limits).unwrap();
    let old = kaeru_core::history(&target, &claim).unwrap();
    let hypothesis = doc.data.nodes.iter_mut().find(|n| n.id == claim).unwrap();
    hypothesis.body = Some("Updated current claim".into());
    amg::import(&target, &doc, true, false, &limits).unwrap();
    let result = amg::snapshot(&target, &AmgSelection::All, &limits).unwrap();
    assert_eq!(result.data.decisions, doc.data.decisions);
    assert!(
        kaeru_core::history(&target, &claim)
            .unwrap()
            .last()
            .unwrap()
            .seconds
            > old.last().unwrap().seconds
    );
    let current = kaeru_core::read_node_full(&target, &claim)
        .unwrap()
        .unwrap();
    assert!(current.tags.contains(&"status:supported".into()));
}

#[test]
fn mandatory_task_and_evidence_references_cannot_escape_a_selected_document() {
    let source = Store::open_in_memory().unwrap();
    let task = mutation(
        &source,
        "task",
        json!({"initiative":"alpha","body":"Do something"}),
    );
    let constraint = mutation(
        &source,
        "note",
        json!({"initiative":"alpha","body":"Only for this task","kind":"constraint","applicability":{"task":task}}),
    );
    kaeru_core::attach_node(&source, &constraint, "slice").unwrap();
    assert!(
        amg::snapshot(
            &source,
            &AmgSelection::Initiatives(vec!["slice".into()]),
            &AmgLimits::default()
        )
        .is_err()
    );
    let doc = amg::snapshot(&source, &AmgSelection::All, &AmgLimits::default()).unwrap();
    assert!(doc.data.nodes.iter().any(|n| n.node_type == "task"));
}

#[test]
fn storage_failure_after_node_writes_rolls_back_the_whole_import() {
    let source = Store::open_in_memory().unwrap();
    let id = note(&source, "alpha", "A", "a");
    let limits = AmgLimits::default();
    let doc = amg::snapshot(&source, &AmgSelection::All, &limits).unwrap();
    let target = Store::open_in_memory().unwrap();
    let sentinel = note(&target, "local", "Keep", "keep");
    target.run("::remove amg_import").unwrap();
    assert!(amg::import(&target, &doc, false, false, &limits).is_err());
    assert!(kaeru_core::read_node_full(&target, &id).unwrap().is_none());
    assert_eq!(
        kaeru_core::read_node_full(&target, &sentinel)
            .unwrap()
            .unwrap()
            .body
            .as_deref(),
        Some("keep")
    );
    assert!(
        target
            .run_read("?[name] := *initiative{name},name='alpha'")
            .unwrap()
            .rows
            .is_empty()
    );
}

#[test]
fn force_restores_an_edge_after_local_retraction_with_a_new_version() {
    let source = Store::open_in_memory().unwrap();
    let a = note(&source, "alpha", "A", "a");
    let b = note(&source, "alpha", "B", "b");
    kaeru_core::link(&source, &a, &b, kaeru_core::EdgeType::RefersTo).unwrap();
    let limits = AmgLimits::default();
    let doc = amg::snapshot(&source, &AmgSelection::All, &limits).unwrap();
    let target = Store::open_in_memory().unwrap();
    amg::import(&target, &doc, false, false, &limits).unwrap();
    kaeru_core::unlink(&target, &a, &b, kaeru_core::EdgeType::RefersTo).unwrap();
    assert!(amg::import(&target, &doc, false, false, &limits).is_err());
    let report = amg::import(&target, &doc, true, false, &limits).unwrap();
    assert_eq!(report.updated_edges, 1);
    assert_eq!(
        amg::snapshot(&target, &AmgSelection::All, &limits)
            .unwrap()
            .data
            .edges
            .len(),
        1
    );
    assert!(
        target
            .run_read("?[validity] := *edge{validity}")
            .unwrap()
            .rows
            .len()
            >= 3
    );
}

#[test]
fn an_edge_owned_only_by_an_unselected_scope_is_not_exported() {
    let source = Store::open_in_memory().unwrap();
    let a = note(&source, "alpha", "A", "a");
    let b = note(&source, "alpha", "B", "b");
    for id in [&a, &b] {
        kaeru_core::attach_node(&source, id, "secret").unwrap();
    }
    source
        .scoped(Some("secret"), |s| {
            kaeru_core::link(s, &a, &b, kaeru_core::EdgeType::RefersTo)
        })
        .unwrap();
    let limits = AmgLimits::default();
    let selected = amg::snapshot(
        &source,
        &AmgSelection::Initiatives(vec!["alpha".into()]),
        &limits,
    )
    .unwrap();
    assert!(selected.data.edges.is_empty());
    assert_eq!(
        amg::snapshot(&source, &AmgSelection::All, &limits)
            .unwrap()
            .data
            .edges
            .len(),
        1
    );
}

#[test]
fn decision_evidence_must_be_inside_the_exported_selection() {
    let source = Store::open_in_memory().unwrap();
    note(&source, "beta", "Proof", "proof");
    mutation(
        &source,
        "claim",
        json!({"initiative":"alpha","text":"Scoped claim","initial_status":"supported","evidence":{"node":{"name":"Proof","initiative":"beta"}}}),
    );
    let limits = AmgLimits::default();
    assert!(
        amg::snapshot(
            &source,
            &AmgSelection::Initiatives(vec!["alpha".into()]),
            &limits
        )
        .is_err()
    );
    let both = amg::snapshot(
        &source,
        &AmgSelection::Initiatives(vec!["alpha".into(), "beta".into()]),
        &limits,
    )
    .unwrap();
    assert_eq!(both.data.decisions.len(), 1);
    assert!(both.data.edges.iter().all(
        |edge| both.data.nodes.iter().any(|node| node.id == edge.src)
            && both.data.nodes.iter().any(|node| node.id == edge.dst)
    ));
}

#[test]
fn board_vocabulary_and_task_deadline_survive_and_conflicting_registry_is_rejected() {
    let source = Store::open_in_memory().unwrap();
    kaeru_core::ensure_board(&source, "alpha").unwrap();
    kaeru_core::add_status(&source, "alpha", "review", "Review").unwrap();
    let task = mutation(
        &source,
        "task",
        json!({"initiative":"alpha","body":"Review report","due":"2026-11-01"}),
    );
    kaeru_core::set_status(&source, "alpha", &task, "review").unwrap();
    let limits = AmgLimits::default();
    let doc = amg::snapshot(&source, &AmgSelection::All, &limits).unwrap();
    let target = Store::open_in_memory().unwrap();
    amg::import(&target, &doc, false, false, &limits).unwrap();
    let n = kaeru_core::read_node_full(&target, &task).unwrap().unwrap();
    assert!(n.tags.contains(&"status:review".into()));
    assert!(n.tags.contains(&"due:2026-11-01".into()));
    let other = Store::open_in_memory().unwrap();
    kaeru_core::ensure_board(&other, "alpha").unwrap();
    assert!(amg::import(&other, &doc, true, false, &limits).is_err());
    assert!(kaeru_core::read_node_full(&other, &task).unwrap().is_none());
}

#[test]
fn force_restoration_validates_memberships_retained_after_retraction() {
    let source = Store::open_in_memory().unwrap();
    let board = kaeru_core::ensure_board(&source, "alpha").unwrap();
    let limits = AmgLimits::default();
    let original = amg::snapshot(&source, &AmgSelection::All, &limits).unwrap();
    let target = Store::open_in_memory().unwrap();
    amg::import(&target, &original, false, false, &limits).unwrap();
    kaeru_core::forget(&target, &board).unwrap();
    let replacement = kaeru_core::ensure_board(&target, "alpha").unwrap();
    assert_ne!(board, replacement);
    let mut incoming = original.clone();
    incoming.metadata.initiatives = vec!["beta".into()];
    incoming.data.nodes[0].initiatives = vec!["beta".into()];
    let error = amg::import(&target, &incoming, true, false, &limits).unwrap_err();
    assert!(
        error.to_string().contains("board identity conflicts"),
        "{error}"
    );
    assert!(
        kaeru_core::read_node_full(&target, &board)
            .unwrap()
            .is_none()
    );
    assert!(
        kaeru_core::read_node_full(&target, &replacement)
            .unwrap()
            .is_some()
    );
    assert!(
        target
            .run_read("?[name] := *initiative{name},name='beta'")
            .unwrap()
            .rows
            .is_empty()
    );
}

#[test]
fn all_includes_edges_owned_by_an_initiative_without_nodes() {
    let source = Store::open_in_memory().unwrap();
    let a = note(&source, "alpha", "A", "a");
    let b = note(&source, "alpha", "B", "b");
    source
        .scoped(Some("gamma"), |s| {
            kaeru_core::link(s, &a, &b, kaeru_core::EdgeType::RefersTo)
        })
        .unwrap();
    let limits = AmgLimits::default();
    let doc = amg::snapshot(&source, &AmgSelection::All, &limits).unwrap();
    assert_eq!(doc.data.edges.len(), 1);
    assert_eq!(doc.data.edges[0].initiatives, vec!["gamma"]);
    assert!(doc.metadata.initiatives.contains(&"gamma".into()));
    let target = Store::open_in_memory().unwrap();
    amg::import(&target, &doc, false, false, &limits).unwrap();
    assert_eq!(
        amg::snapshot(&target, &AmgSelection::All, &limits)
            .unwrap()
            .data
            .edges[0]
            .initiatives,
        vec!["gamma"]
    );
}

#[test]
fn force_records_before_images_when_invalidating_local_summaries() {
    let target = Store::open_in_memory().unwrap();
    let a = note(&target, "alpha", "A", "a");
    let b = note(&target, "alpha", "B", "b");
    target
        .scoped(Some("alpha"), |s| {
            kaeru_core::link(s, &a, &b, kaeru_core::EdgeType::DerivedFrom)
        })
        .unwrap();
    let chain = mutation(
        &target,
        "trail",
        json!({"initiative":"alpha","name":"Reasoning","route":{"members":[a,b]},"summary":"A supports B"}),
    );
    let limits = AmgLimits::default();
    let mut doc = amg::snapshot(&target, &AmgSelection::All, &limits).unwrap();
    assert_eq!(doc.data.trails[0].stale, Some(false));
    doc.data.nodes.retain(|n| n.id == a);
    doc.data.nodes[0].body = Some("different evidence".into());
    doc.data.edges.clear();
    doc.data.trails.clear();
    amg::import(&target, &doc, true, false, &limits).unwrap();
    let copied = amg::snapshot(&target, &AmgSelection::All, &limits).unwrap();
    assert_eq!(
        copied
            .data
            .trails
            .iter()
            .find(|t| t.id == chain)
            .unwrap()
            .stale,
        Some(true)
    );
    let receipt = target
        .run_read("?[changes] := *amg_import{changes}")
        .unwrap();
    let text = serde_json::to_value(&receipt.rows).unwrap().to_string();
    assert!(text.contains("trail_invalidation"), "{text}");
    assert!(
        text.contains("\\\"stale\\\":false") || text.contains("\"stale\":false"),
        "{text}"
    );
}

#[test]
fn force_does_not_change_an_existing_node_type() {
    let target = Store::open_in_memory().unwrap();
    let id = note(&target, "alpha", "A", "a");
    let limits = AmgLimits::default();
    let mut doc = amg::snapshot(&target, &AmgSelection::All, &limits).unwrap();
    doc.data.nodes[0].node_type = "task".into();
    let before = kaeru_core::history(&target, &id).unwrap().len();
    assert!(
        amg::import(&target, &doc, true, false, &limits)
            .unwrap_err()
            .to_string()
            .contains("cannot change node type")
    );
    assert_eq!(kaeru_core::history(&target, &id).unwrap().len(), before);
}

#[test]
fn restoring_a_task_checks_its_retained_board_memberships() {
    let target = Store::open_in_memory().unwrap();
    kaeru_core::ensure_board(&target, "alpha").unwrap();
    let task = mutation(
        &target,
        "task",
        json!({"initiative":"alpha","body":"Retired task"}),
    );
    let limits = AmgLimits::default();
    let mut incoming = amg::snapshot(&target, &AmgSelection::All, &limits).unwrap();
    kaeru_core::forget(&target, &task).unwrap();
    incoming.data.nodes.retain(|node| node.id == task);
    incoming.data.edges.clear();
    incoming.metadata.initiatives = vec!["beta".into()];
    let node = &mut incoming.data.nodes[0];
    node.initiatives = vec!["beta".into()];
    node.tags.retain(|tag| !tag.starts_with("status:"));
    node.tags.push("status:review".into());
    kaeru_core::ensure_board(&target, "beta").unwrap();
    kaeru_core::add_status(&target, "beta", "review", "Review").unwrap();
    let error = amg::import(&target, &incoming, true, false, &limits).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("incompatible with destination board in alpha"),
        "{error}"
    );
    assert!(
        kaeru_core::read_node_full(&target, &task)
            .unwrap()
            .is_none()
    );
}
