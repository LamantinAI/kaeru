use std::sync::Arc;

use kaeru_core::{
    Store, Visibility,
    api::{self, ApiOptions, Response, ResponseData, State},
};
use serde_json::{Value, json};

fn run(store: &Store, name: &str, args: Value) -> Response {
    let r = api::execute(store, name, args, &ApiOptions::local());
    assert!(!r.is_error(), "{name}: {r:?}");
    let validator = jsonschema::validator_for(&api::output_schema_for(name)).unwrap();
    validator
        .validate(&serde_json::to_value(&r).unwrap())
        .unwrap_or_else(|e| panic!("{name} response violates advertised schema: {e}"));
    r
}
fn id(r: &Response) -> String {
    match &r.data {
        ResponseData::Mutation(d) => d.primary.as_ref().unwrap().id.clone(),
        _ => panic!(),
    }
}
fn note(store: &Store, name: &str, body: &str) -> String {
    id(&run(
        store,
        "note",
        json!({"initiative":"work","name":name,"body":body}),
    ))
}
fn edge(store: &Store, a: &str, b: &str, kind: &str) {
    run(
        store,
        "relate",
        json!({"initiative":"work","action":"connect","from":a,"to":b,"type":kind,"weight":0.9}),
    );
}

#[test]
fn task_completion_and_reopening_keep_deadlines_independent_of_board_order() {
    let store = Store::open_in_memory().unwrap();
    let task = id(&run(
        &store,
        "task",
        json!({"initiative":"work","body":"Sign off.","due":"2000-01-01"}),
    ));
    run(
        &store,
        "task_update",
        json!({"initiative":"work","task":task,"status":"done"}),
    );
    run(
        &store,
        "board_config",
        json!({"initiative":"work","action":"reorder","order":["done","open","in-progress"]}),
    );
    store.scoped(Some("work"), |s| {
        assert!(kaeru_core::open_tasks(s).unwrap().is_empty())
    });
    assert!(
        kaeru_core::read_node_full(&store, &task)
            .unwrap()
            .unwrap()
            .tags
            .contains(&"due:2000-01-01".into())
    );
    run(
        &store,
        "task_update",
        json!({"initiative":"work","task":task,"status":"open"}),
    );
    store.scoped(Some("work"), |s| {
        assert!(kaeru_core::open_tasks(s).unwrap()[0].overdue)
    });
}

#[test]
fn closing_review_flags_preserves_substantive_counterevidence_and_rejects_stale_cas() {
    let store = Store::open_in_memory().unwrap();
    let target = note(&store, "fact", "A fact.");
    let evidence = note(&store, "counterexample", "Contradictory measurement.");
    edge(&store, &evidence, &target, "contradicts");
    run(
        &store,
        "review",
        json!({"initiative":"work","action":"open","target":target,"reason":"Need another check."}),
    );
    let before = run(&store, "read", json!({"initiative":"work","node":target}));
    let revision = match before.data {
        ResponseData::Record(d) => d.node.revision,
        _ => panic!(),
    };
    run(
        &store,
        "review",
        json!({"initiative":"work","action":"open","target":target,"reason":"A second concern."}),
    );
    let stale = api::execute(
        &store,
        "review",
        json!({"initiative":"work","action":"close","target":target,"expected_revision":revision}),
        &ApiOptions::local(),
    );
    assert!(matches!(stale.state, State::Rejected));
    assert_eq!(stale.issues[0].code, "CONFLICT");
    run(
        &store,
        "review",
        json!({"initiative":"work","action":"close","target":target,"resolution":"Flag checks complete; counterexample remains."}),
    );
    let edges = kaeru_core::edges_of(&store, &target).unwrap();
    assert!(
        edges
            .iter()
            .any(|e| e.0 == evidence && e.2 == "contradicts")
    );
}

#[test]
fn saved_reasoning_is_delivered_without_an_expansion_flag_and_tracks_identity_transfer() {
    let store = Store::open_in_memory().unwrap();
    let a = note(&store, "measurement", "Measured the effect.");
    let b = note(&store, "conclusion", "The observed result.");
    edge(&store, &a, &b, "causal");
    run(
        &store,
        "trail",
        json!({"initiative":"work","route":{"from":a,"to":b},"summary":"The measurement explains why this conclusion was chosen."}),
    );
    let first = run(&store, "read", json!({"initiative":"work","node":b}));
    match first.data {
        ResponseData::Record(d) => {
            assert_eq!(d.reasoning[0].stale, Some(false));
            assert!(
                d.reasoning[0]
                    .summary
                    .as_ref()
                    .unwrap()
                    .contains("explains why")
            );
        }
        _ => panic!(),
    }
    let promoted = id(&run(
        &store,
        "lifecycle",
        json!({"initiative":"work","node":b,"target_tier":"archival"}),
    ));
    let preserved = run(&store, "read", json!({"initiative":"work","node":promoted}));
    match preserved.data {
        ResponseData::Record(d) => assert_eq!(d.reasoning[0].stale, Some(false)),
        _ => panic!(),
    }
    run(
        &store,
        "revise",
        json!({"initiative":"work","node":a,"patch":{"body":"The measurement was corrected."}}),
    );
    let changed = run(&store, "read", json!({"initiative":"work","node":promoted}));
    match changed.data {
        ResponseData::Record(d) => assert_eq!(d.reasoning[0].stale, Some(true)),
        _ => panic!(),
    }
}

#[test]
fn graph_reads_choose_an_authorized_route_and_never_emit_hidden_steps() {
    let store = Store::open_in_memory().unwrap();
    let start = note(&store, "start", "Start.");
    let hidden = note(&store, "hidden", "Private step.");
    let visible = note(&store, "visible", "Public step.");
    let end = note(&store, "end", "End.");
    edge(&store, &start, &hidden, "causal");
    edge(&store, &hidden, &end, "causal");
    edge(&store, &start, &visible, "causal");
    edge(&store, &visible, &end, "causal");
    for n in [&start, &visible, &end] {
        kaeru_core::set_visibility(&store, n, Visibility::Shared).unwrap();
    }
    let options = ApiOptions {
        allowed_initiatives: Some(vec!["work".into()]),
        caller: "viewer".into(),
        ..ApiOptions::default()
    };
    let r = api::execute(
        &store,
        "read",
        json!({"initiative":"work","node":start,"to":end,"include":["path","relations"]}),
        &options,
    );
    assert!(!r.is_error(), "{r:?}");
    let serialized = serde_json::to_string(&r).unwrap();
    assert!(!serialized.contains(&hidden));
    assert!(!serialized.contains("Private step"));
    match r.data {
        ResponseData::Record(d) => assert_eq!(d.path, vec![start, visible, end]),
        _ => panic!(),
    }
}

#[test]
fn paged_source_reads_reassemble_exact_bytes_with_bounded_responses() {
    let store = Store::open_in_memory().unwrap();
    let source = "Original 🦭 with quoted source.\n".repeat(180);
    let reference = id(&run(
        &store,
        "reference",
        json!({"initiative":"work","name":"long-source","source":{"text":source}}),
    ));
    let mut args = json!({"initiative":"work","node":reference,"budget_chars":2500});
    let mut reconstructed = String::new();
    let mut pages = 0;
    loop {
        let r = run(&store, "read", args);
        pages += 1;
        assert!(
            serde_json::to_string(&r).unwrap().chars().count() <= 2500,
            "page exceeded budget"
        );
        match &r.data {
            ResponseData::Record(d) => reconstructed.push_str(d.node.body.as_deref().unwrap_or("")),
            _ => panic!(),
        }
        match r.continuation {
            Some(next) => args = next.next_arguments,
            None => break,
        };
        assert!(pages < 100);
    }
    assert_eq!(reconstructed, source);
    assert!(pages > 1);
}

#[test]
fn concurrent_retries_have_one_capture_and_a_shared_result() {
    let store = Arc::new(Store::open_in_memory().unwrap());
    let threads=(0..8).map(|_|{let store=store.clone();std::thread::spawn(move||id(&run(&store,"note",json!({"initiative":"work","body":"Only one capture.","request_id":"parallel-retry"}))))}).collect::<Vec<_>>();
    let ids = threads
        .into_iter()
        .map(|t| t.join().unwrap())
        .collect::<Vec<_>>();
    assert!(ids.iter().all(|id| id == &ids[0]));
    assert_eq!(
        kaeru_core::nodes_in_initiative(&store, "work")
            .unwrap()
            .len(),
        1
    );
}

#[test]
fn supersession_cannot_cycle_past_the_normal_recall_hop_cap() {
    let store = Store::open_in_memory().unwrap();
    let nodes = (0..8)
        .map(|i| note(&store, &format!("node-{i}"), "Fact."))
        .collect::<Vec<_>>();
    for pair in nodes.windows(2) {
        edge(&store, &pair[0], &pair[1], "supersedes");
    }
    let cycle = api::execute(
        &store,
        "relate",
        json!({"initiative":"work","action":"connect","from":nodes[7],"to":nodes[0],"type":"supersedes","weight":0.9}),
        &ApiOptions::local(),
    );
    assert!(matches!(cycle.state, State::Rejected));
    assert!(
        !kaeru_core::edges_of(&store, &nodes[7])
            .unwrap()
            .iter()
            .any(|e| e.0 == nodes[7] && e.1 == nodes[0])
    );
}

#[test]
fn rename_moves_slots_and_sharing_restrictions_and_merge_rejects_conflicting_slots() {
    let store = Store::open_in_memory().unwrap();
    let holder = id(&run(
        &store,
        "note",
        json!({"initiative":"work","body":"Current handoff.","options":{"slot":"handoff"}}),
    ));
    run(
        &store,
        "policy",
        json!({"initiative":"work","policy":"team","clouds":["approved"]}),
    );
    run(
        &store,
        "initiative_manage",
        json!({"action":"rename","old":"work","new":"renamed"}),
    );
    assert_eq!(
        kaeru_core::slot_holder(&store, "renamed", "handoff").unwrap(),
        Some(holder.clone())
    );
    assert_eq!(
        kaeru_core::slot_holder(&store, "work", "handoff").unwrap(),
        None
    );
    assert!(!kaeru_core::permits_cloud(&store, "renamed", "unapproved").unwrap());
    id(&run(
        &store,
        "note",
        json!({"initiative":"other","body":"Different handoff.","options":{"slot":"handoff"}}),
    ));
    let merge = api::execute(
        &store,
        "initiative_manage",
        json!({"action":"merge","source":"renamed","target":"other"}),
        &ApiOptions::local(),
    );
    assert!(matches!(merge.state, State::Rejected));
    assert_eq!(
        kaeru_core::slot_holder(&store, "renamed", "handoff").unwrap(),
        Some(holder)
    );
}

#[test]
fn truncated_rules_are_disclosed_and_truncated_reminders_are_not_delivered() {
    let store = Store::open_in_memory().unwrap();
    let constraint = id(&run(
        &store,
        "note",
        json!({"initiative":"work","kind":"constraint","body":"Do something carefully. ".repeat(40)}),
    ));
    let reminder = id(&run(
        &store,
        "note",
        json!({"initiative":"work","body":"Reminder text. ".repeat(40),"reminder":{"after":"2000-01-01","window_days":3.0}}),
    ));
    let response = run(
        &store,
        "context",
        json!({"initiative":"work","budget_chars":64000}),
    );
    assert!(!response.coverage.as_ref().unwrap().rules_complete);
    assert!(
        response
            .coverage
            .as_ref()
            .unwrap()
            .omitted
            .iter()
            .any(|o| o.section == "rules")
    );
    assert!(
        serde_json::to_string(&response)
            .unwrap()
            .contains(&constraint)
    );
    assert!(
        !kaeru_core::read_node_full(&store, &reminder)
            .unwrap()
            .unwrap()
            .tags
            .iter()
            .any(|t| t.starts_with("seen:"))
    );
}

#[test]
fn continuation_binds_caller_policy_and_current_memory() {
    let store = Store::open_in_memory().unwrap();
    let source = id(&run(
        &store,
        "reference",
        json!({"initiative":"work","name":"long","source":{"text":"Unicode 日本語 🐸. ".repeat(600)}}),
    ));
    let first = run(
        &store,
        "read",
        json!({"initiative":"work","node":source,"budget_chars":2500}),
    );
    let continuation = first.continuation.unwrap().next_arguments;
    let other = ApiOptions {
        caller: "different-caller".into(),
        ..ApiOptions::local()
    };
    let denied = api::execute(&store, "read", continuation.clone(), &other);
    assert_eq!(denied.issues[0].code, "CURSOR_STALE");
    note(&store, "changed", "New write.");
    let stale = api::execute(&store, "read", continuation, &ApiOptions::local());
    assert_eq!(stale.issues[0].code, "CURSOR_STALE");
}

#[test]
fn historical_board_cards_keep_their_old_column_and_body() {
    let store = Store::open_in_memory().unwrap();
    let task = id(&run(
        &store,
        "task",
        json!({"initiative":"work","body":"Old task body."}),
    ));
    let recorded = kaeru_core::history(&store, &task)
        .unwrap()
        .into_iter()
        .find(|v| v.asserted)
        .unwrap()
        .seconds;
    run(
        &store,
        "task_update",
        json!({"initiative":"work","task":task,"status":"done"}),
    );
    run(
        &store,
        "revise",
        json!({"initiative":"work","node":task,"patch":{"body":"New task body."}}),
    );
    let time = chrono::DateTime::<chrono::Utc>::from_timestamp(recorded as i64, 0)
        .unwrap()
        .to_rfc3339();
    let response = run(
        &store,
        "context",
        json!({"initiative":"work","view":"board","recorded_at":time}),
    );
    match response.data {
        ResponseData::Context(data) => {
            let old = &data.sections[0].records[0];
            assert_eq!(old.body.as_deref(), Some("Old task body."));
            assert_eq!(old.status.as_deref(), Some("open"));
        }
        _ => panic!(),
    }
}

#[test]
fn refresh_without_a_new_summary_does_not_certify_stale_reasoning() {
    let store = Store::open_in_memory().unwrap();
    let a = note(&store, "a", "First premise.");
    let b = note(&store, "b", "Second premise.");
    edge(&store, &a, &b, "causal");
    let trail = id(&run(
        &store,
        "trail",
        json!({"initiative":"work","route":{"members":[a,b]},"summary":"Because of the original premise."}),
    ));
    run(
        &store,
        "revise",
        json!({"initiative":"work","node":a,"patch":{"body":"Changed premise."}}),
    );
    run(
        &store,
        "trail",
        json!({"initiative":"work","action":"refresh","trail":trail}),
    );
    let response = run(&store, "read", json!({"initiative":"work","node":b}));
    match response.data {
        ResponseData::Record(data) => assert_eq!(data.reasoning[0].stale, Some(true)),
        _ => panic!(),
    }
}

#[test]
fn history_pages_include_metadata_events_and_keep_all_versions() {
    let store = Store::open_in_memory().unwrap();
    let record = note(&store, "history", "Initial.");
    for i in 0..12 {
        run(
            &store,
            "revise",
            json!({"initiative":"work","node":record,"patch":{"body":format!("Version {i}.")}}),
        );
    }
    let expected = kaeru_core::history(&store, &record).unwrap().len();
    let mut args = json!({"initiative":"work","node":record,"budget_chars":1800});
    let mut versions = 0;
    let mut seen = 0;
    loop {
        let response = run(&store, "history", args);
        if let ResponseData::History(data) = &response.data {
            versions += data.versions.len();
        }
        seen += 1;
        assert!(seen < 100);
        if let Some(next) = response.continuation {
            args = next.next_arguments;
        } else {
            break;
        }
    }
    assert_eq!(versions, expected);
    assert!(seen > 1);
}

#[test]
fn a_partial_remote_capture_replays_the_existing_local_record() {
    let store = Store::open_in_memory().unwrap();
    let args = json!({"initiative":"work","body":"Share if possible.","sharing":{"visibility":"shared"},"request_id":"remote-pending"});
    let first = api::execute(&store, "note", args.clone(), &ApiOptions::local());
    assert!(matches!(first.state, State::Partial));
    let replay = api::execute(&store, "note", args, &ApiOptions::local());
    assert!(replay.replayed);
    assert_eq!(id(&first), id(&replay));
}

#[test]
fn historical_metadata_filters_private_applicability_targets() {
    let store = Store::open_in_memory().unwrap();
    let task = id(&run(
        &store,
        "task",
        json!({"initiative":"work","body":"Private task."}),
    ));
    let rule = id(&run(
        &store,
        "note",
        json!({"initiative":"work","kind":"constraint","body":"A shared rule.","applicability":{"task":task}}),
    ));
    kaeru_core::set_visibility(&store, &rule, Visibility::Shared).unwrap();
    let time = kaeru_core::history(&store, &rule)
        .unwrap()
        .into_iter()
        .find(|h| h.asserted)
        .unwrap()
        .seconds;
    let recorded = chrono::DateTime::<chrono::Utc>::from_timestamp(time as i64, 0)
        .unwrap()
        .to_rfc3339();
    let options = ApiOptions {
        allowed_initiatives: Some(vec!["work".into()]),
        caller: "viewer".into(),
        ..ApiOptions::default()
    };
    let response = api::execute(
        &store,
        "read",
        json!({"initiative":"work","node":rule,"recorded_at":recorded}),
        &options,
    );
    assert!(!response.is_error(), "{response:?}");
    assert!(!serde_json::to_string(&response).unwrap().contains(&task));
}

#[test]
fn captures_return_the_revision_after_their_relation_supplement() {
    let store = Store::open_in_memory().unwrap();
    let related = note(&store, "related", "Existing result.");
    let response = run(
        &store,
        "note",
        json!({"initiative":"work","body":"A linked observation.","related":{"to":related,"type":"refers_to","weight":0.7}}),
    );
    let record = match response.data {
        ResponseData::Mutation(data) => data.primary.unwrap(),
        _ => panic!(),
    };
    run(
        &store,
        "pin",
        json!({"initiative":"work","node":record.id,"enabled":true,"reason":"Follow up.","expected_revision":record.revision}),
    );
}

#[test]
fn hygiene_preview_pages_reasons_without_writing_and_force_keeps_a_durable_log() {
    let store = Store::open_in_memory().unwrap();
    for i in 0..16 {
        let record = kaeru_core::new_node_id();
        store.run(&format!("?[id,validity,type,tier,name,body,tags,initiatives,properties,visibility,layer] <- [['{record}',[1.0,true],'episode','operational','old-{i}','Old observation.',null,null,null,'local','warm']] :put node {{id,validity=>type,tier,name,body,tags,initiatives,properties,visibility,layer}}")).unwrap();
        kaeru_core::attach_node(&store, &record, "work").unwrap();
    }
    let before = store
        .run_read("?[id,validity,properties,layer] := *node{id,validity,properties,layer}")
        .unwrap()
        .rows;
    let mut args = json!({"initiative":"work","action":"preview_hygiene","budget_chars":1800});
    let mut proposals = 0;
    loop {
        let response = run(&store, "retention", args);
        if let ResponseData::Context(data) = &response.data {
            for proposal in &data.proposals {
                assert!(!proposal.reason.is_empty());
                assert_eq!(proposal.to, "cold");
            }
            proposals += data.proposals.len();
        }
        if let Some(next) = response.continuation {
            args = next.next_arguments;
        } else {
            break;
        }
    }
    assert_eq!(proposals, 16);
    assert_eq!(
        store
            .run_read("?[id,validity,properties,layer] := *node{id,validity,properties,layer}")
            .unwrap()
            .rows,
        before
    );
    let forced = run(
        &store,
        "retention",
        json!({"initiative":"work","action":"run_hygiene"}),
    );
    match forced.data {
        ResponseData::Mutation(data) => {
            assert_eq!(data.created.len(), 1);
            let log = kaeru_core::read_node_full(&store, &data.created[0].id)
                .unwrap()
                .unwrap();
            assert!(log.name.starts_with("hygiene-"));
            assert_eq!(log.layer, "cold");
        }
        _ => panic!(),
    }
}

#[test]
fn exports_preserve_the_exact_source_and_apply_the_scope_before_content_selection() {
    let store = Store::open_in_memory().unwrap();
    let exact = "Exact source.\n<parameter name=\"body\">literal quoted material";
    run(
        &store,
        "reference",
        json!({"initiative":"work","name":"source","source":{"text":exact}}),
    );
    run(
        &store,
        "note",
        json!({"initiative":"other","body":"PRIVATE OTHER PROJECT."}),
    );
    let directory = std::env::temp_dir().join(kaeru_core::new_node_id());
    let options = ApiOptions {
        artifact_root: Some(directory.clone()),
        ..ApiOptions::local()
    };
    let response = api::execute(
        &store,
        "export",
        json!({"format":"markdown","initiative":"work","destination":"snapshot.zip"}),
        &options,
    );
    assert!(!response.is_error(), "{response:?}");
    let mut archive =
        zip::ZipArchive::new(std::fs::File::open(directory.join("snapshot.zip")).unwrap()).unwrap();
    let mut contents = String::new();
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).unwrap();
        std::io::Read::read_to_string(&mut entry, &mut contents).unwrap();
    }
    assert!(contents.contains(exact));
    assert!(!contents.contains("PRIVATE OTHER PROJECT"));
    drop(archive);
    std::fs::remove_dir_all(directory).unwrap();
}

#[test]
fn public_board_reads_do_not_disclose_a_private_registry() {
    let store = Store::open_in_memory().unwrap();
    let task = id(&run(
        &store,
        "task",
        json!({"initiative":"work","body":"Shared work."}),
    ));
    run(
        &store,
        "board_config",
        json!({"initiative":"work","action":"add","key":"internal","label":"PRIVATE REGISTRY LABEL"}),
    );
    kaeru_core::set_visibility(&store, &task, Visibility::Shared).unwrap();
    let options = ApiOptions {
        allowed_initiatives: Some(vec!["work".into()]),
        caller: "viewer".into(),
        ..ApiOptions::default()
    };
    let board = api::execute(
        &store,
        "context",
        json!({"initiative":"work","view":"board"}),
        &options,
    );
    assert!(board.is_error());
    assert_eq!(board.issues[0].code, "NOT_FOUND");
    let work = api::execute(&store, "context", json!({"initiative":"work"}), &options);
    assert!(!work.is_error());
    assert!(
        !serde_json::to_string(&work)
            .unwrap()
            .contains("PRIVATE REGISTRY LABEL")
    );
    assert!(!work.coverage.unwrap().complete);
}

#[test]
fn a_long_source_delivers_saved_reasoning_before_its_body_continuation() {
    let store = Store::open_in_memory().unwrap();
    let source = id(&run(
        &store,
        "reference",
        json!({"initiative":"work","name":"large-source","source":{"text":"Original text. ".repeat(3000)}}),
    ));
    let result = note(&store, "result", "A conclusion.");
    edge(&store, &source, &result, "causal");
    run(
        &store,
        "trail",
        json!({"initiative":"work","route":{"members":[source,result]},"summary":"The original source establishes why this result is justified."}),
    );
    let first = run(
        &store,
        "read",
        json!({"initiative":"work","node":source,"budget_chars":3000}),
    );
    assert!(first.continuation.is_some());
    match first.data {
        ResponseData::Record(data) => assert!(
            data.reasoning[0]
                .summary
                .as_ref()
                .unwrap()
                .contains("justified")
        ),
        _ => panic!(),
    }
}

#[test]
fn reminder_delivery_stays_within_budget_and_returns_one_revision_per_identity() {
    let store = Store::open_in_memory().unwrap();
    let reminder = id(&run(
        &store,
        "note",
        json!({"initiative":"work","body":"Check the pending decision.","reminder":{"after":"2000-01-01","window_days":2}}),
    ));
    let mut args = json!({"initiative":"work","budget_chars":3000});
    loop {
        let response = run(&store, "context", args);
        assert!(serde_json::to_string(&response).unwrap().chars().count() <= 3000);
        if let ResponseData::Context(data) = &response.data {
            let copies = data
                .sections
                .iter()
                .flat_map(|s| s.records.iter())
                .filter(|n| n.id == reminder)
                .collect::<Vec<_>>();
            if let Some(first) = copies.first() {
                assert!(copies.iter().all(|n| n.revision == first.revision));
            }
        }
        if let Some(next) = response.continuation {
            args = next.next_arguments;
        } else {
            break;
        }
    }
    assert!(
        kaeru_core::read_node_full(&store, &reminder)
            .unwrap()
            .unwrap()
            .tags
            .iter()
            .any(|t| t.starts_with("seen:"))
    );
}

#[test]
fn an_unchanged_task_update_does_not_create_a_new_recorded_version() {
    let store = Store::open_in_memory().unwrap();
    let task = id(&run(
        &store,
        "task",
        json!({"initiative":"work","body":"Open task.","due":"2026-12-01"}),
    ));
    let before = kaeru_core::history(&store, &task).unwrap();
    let response = run(
        &store,
        "task_update",
        json!({"initiative":"work","task":task,"status":"open","due":"2026-12-01"}),
    );
    assert!(matches!(response.state, State::Unchanged));
    assert_eq!(
        kaeru_core::history(&store, &task).unwrap().len(),
        before.len()
    );
    match response.data {
        ResponseData::Mutation(data) => assert!(data.changes.is_empty()),
        _ => panic!(),
    }
}
