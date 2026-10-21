//! Exercise the public agent contract using disposable vaults and artifacts.
use std::fs;
use std::io::{Cursor, Read};
use std::sync::{Arc, Barrier};

use kaeru_core::config::amg::AmgLimits;
use kaeru_core::{
    Store, amg,
    api::{self, ApiOptions, Response, ResponseData},
};
use serde_json::json;
use tempfile::TempDir;

fn note(store: &Store, scope: &str, name: &str, body: &str) -> String {
    let response = api::execute(
        store,
        "note",
        json!({"initiative":scope,"name":name,"body":body}),
        &ApiOptions::local(),
    );
    match response.data {
        ResponseData::Mutation(data) => data.primary.unwrap().id,
        other => panic!("{other:?}"),
    }
}
fn options(dir: &TempDir) -> ApiOptions {
    ApiOptions {
        artifact_root: Some(dir.path().join("exports")),
        ..ApiOptions::local()
    }
}
fn artifact(response: &Response) -> &kaeru_core::models::Artifact {
    assert!(!response.is_error(), "{response:?}");
    match &response.data {
        ResponseData::Mutation(data) => &data.artifacts[0],
        _ => panic!("{response:?}"),
    }
}
#[test]
fn both_agent_formats_preserve_sources_and_never_emit_a_json_directory() {
    let store = Store::open_in_memory().unwrap();
    let exact = "Точный текст.\n```\n[not a heading] ../file";
    let response = api::execute(
        &store,
        "reference",
        json!({"initiative":"alpha","name":"../unsafe [name]","source":{"text":exact}}),
        &ApiOptions::local(),
    );
    assert!(!response.is_error());
    note(&store, "beta", "Hidden", "other project");
    let dir = TempDir::new().unwrap();
    let opts = options(&dir);
    for format in ["amg", "markdown"] {
        let response = api::execute(
            &store,
            "export",
            json!({"format":format,"initiative":"alpha","destination":format}),
            &opts,
        );
        let entry = artifact(&response);
        assert!(
            entry
                .path
                .ends_with(if format == "amg" { ".amg" } else { ".zip" })
        );
        let bytes = fs::read(&entry.path).unwrap();
        assert_eq!(entry.bytes, bytes.len());
        assert_eq!(entry.initiatives, vec!["alpha"]);
        if format == "amg" {
            let doc = amg::decode(bytes.as_slice(), &AmgLimits::default()).unwrap();
            assert_eq!(doc.data.nodes[0].body.as_deref(), Some(exact));
        } else {
            let mut zip = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
            let mut text = String::new();
            for index in 0..zip.len() {
                let mut file = zip.by_index(index).unwrap();
                assert!(file.name().ends_with(".md"));
                assert!(!file.name().contains(".."));
                assert!(file.enclosed_name().is_some());
                file.read_to_string(&mut text).unwrap();
            }
            assert!(text.contains(exact));
            assert!(!text.contains("other project"));
        }
    }
}
#[test]
fn all_one_and_multiple_selectors_are_explicit_and_exclusive() {
    let store = Store::open_in_memory().unwrap();
    note(&store, "alpha", "A", "a");
    note(&store, "beta", "B", "b");
    let dir = TempDir::new().unwrap();
    let opts = options(&dir);
    for (index, (selection, count)) in [
        (json!({"initiative":"alpha"}), 1),
        (json!({"initiatives":["alpha","beta"]}), 2),
        (json!({"all":true}), 2),
    ]
    .into_iter()
    .enumerate()
    {
        let mut args = selection;
        args["format"] = json!("amg");
        args["destination"] = json!(format!("export-{index}"));
        let response = api::execute(&store, "export", args, &opts);
        assert_eq!(artifact(&response).nodes, count);
    }
    for selection in [
        json!({}),
        json!({"all":false}),
        json!({"initiatives":[]}),
        json!({"initiatives":["alpha","alpha"]}),
        json!({"all":true,"initiative":"alpha"}),
        json!({"initiatives":["alpha"],"initiative":"alpha"}),
    ] {
        let mut args = selection;
        args["format"] = json!("amg");
        args["destination"] = json!("invalid");
        assert!(api::execute(&store, "export", args, &opts).is_error());
    }
    assert!(
        api::validate(
            "export",
            &json!({"format":"json","all":true,"destination":"bad"})
        )
        .is_err()
    );
}
#[test]
fn access_denials_precede_snapshot_and_filesystem_effects() {
    let store = Store::open_in_memory().unwrap();
    note(&store, "alpha", "A", "a");
    note(&store, "secret", "B", "private");
    let dir = TempDir::new().unwrap();
    let mut opts = options(&dir);
    opts.allowed_initiatives = Some(vec!["alpha".into()]);
    for selection in [
        json!({"all":true}),
        json!({"initiative":"secret"}),
        json!({"initiatives":["alpha","secret"]}),
    ] {
        let mut args = selection;
        args["format"] = json!("amg");
        args["destination"] = json!("forbidden");
        assert!(api::execute(&store, "export", args, &opts).is_error());
    }
    opts.denied_initiatives = vec!["alpha".into()];
    assert!(
        api::execute(
            &store,
            "export",
            json!({"format":"markdown","initiative":"alpha","destination":"denied"}),
            &opts
        )
        .is_error()
    );
    opts.denied_initiatives.clear();
    opts.include_local = false;
    assert!(
        api::execute(
            &store,
            "export",
            json!({"format":"amg","initiative":"alpha","destination":"readonly"}),
            &opts
        )
        .is_error()
    );
    assert!(!opts.artifact_root.unwrap().exists());
}
#[test]
fn selected_export_drops_other_membership_names_and_boundary_links() {
    let store = Store::open_in_memory().unwrap();
    let a = note(&store, "alpha", "A", "a");
    let b = note(&store, "secret", "B", "unavailable body");
    kaeru_core::attach_node(&store, &a, "secret").unwrap();
    store
        .scoped(Some("alpha"), |s| {
            kaeru_core::link(s, &a, &b, kaeru_core::EdgeType::RefersTo)
        })
        .unwrap();
    let dir = TempDir::new().unwrap();
    let mut opts = options(&dir);
    opts.allowed_initiatives = Some(vec!["alpha".into()]);
    let response = api::execute(
        &store,
        "export",
        json!({"format":"amg","initiative":"alpha","destination":"selected"}),
        &opts,
    );
    let doc = amg::decode(
        fs::File::open(&artifact(&response).path).unwrap(),
        &AmgLimits::default(),
    )
    .unwrap();
    assert_eq!(doc.data.nodes.len(), 1);
    assert_eq!(doc.data.nodes[0].initiatives, vec!["alpha"]);
    assert!(doc.data.edges.is_empty());
}
#[test]
fn restricted_archives_refuse_secrets_even_inside_exact_source_tags_or_properties() {
    let token = format!("sk-{}", "x".repeat(30));
    for location in ["source", "tags", "properties"] {
        let store = Store::open_in_memory().unwrap();
        if location == "source" {
            api::execute(
                &store,
                "reference",
                json!({"initiative":"alpha","name":"Original","source":{"text":token}}),
                &ApiOptions::local(),
            );
        } else {
            let id = note(&store, "alpha", "A", "a");
            // Prepare valid portable typed fields through the already-tested import path.
            let mut doc = amg::snapshot(
                &store,
                &kaeru_core::models::amg::AmgSelection::All,
                &AmgLimits::default(),
            )
            .unwrap();
            let n = doc.data.nodes.iter_mut().find(|n| n.id == id).unwrap();
            if location == "tags" {
                n.tags.push(token.clone());
            } else {
                n.properties = json!({"nested":{"key":token}});
            }
            amg::import(&store, &doc, true, false, &AmgLimits::default()).unwrap();
        }
        let dir = TempDir::new().unwrap();
        let mut opts = options(&dir);
        opts.allowed_initiatives = Some(vec!["alpha".into()]);
        for format in ["markdown", "amg"] {
            let response = api::execute(
                &store,
                "export",
                json!({"format":format,"initiative":"alpha","destination":location}),
                &opts,
            );
            assert!(response.is_error(), "{location}: {response:?}");
            assert_eq!(response.issues[0].code, "EXPORT_REQUIRES_REDACTION");
            assert!(!serde_json::to_string(&response).unwrap().contains(&token));
        }
        assert!(!opts.artifact_root.unwrap().exists());
    }
}
#[test]
fn missing_root_invalid_paths_and_existing_files_are_rejected() {
    let store = Store::open_in_memory().unwrap();
    note(&store, "alpha", "A", "a");
    let dir = TempDir::new().unwrap();
    let opts = options(&dir);
    assert!(
        api::execute(
            &store,
            "export",
            json!({"format":"amg","initiative":"alpha","destination":"archive"}),
            &ApiOptions::local()
        )
        .is_error()
    );
    for path in ["../outside.amg", "wrong.zip", "missing/child.amg"] {
        assert!(
            api::execute(
                &store,
                "export",
                json!({"format":"amg","initiative":"alpha","destination":path}),
                &opts
            )
            .is_error()
        );
    }
    fs::write(
        opts.artifact_root.as_ref().unwrap().join("existing.amg"),
        b"keep original",
    )
    .unwrap();
    assert!(
        api::execute(
            &store,
            "export",
            json!({"format":"amg","initiative":"alpha","destination":"existing.amg"}),
            &opts
        )
        .is_error()
    );
    assert_eq!(
        fs::read(opts.artifact_root.as_ref().unwrap().join("existing.amg")).unwrap(),
        b"keep original"
    );
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(
            dir.path(),
            opts.artifact_root.as_ref().unwrap().join("outside-link"),
        )
        .unwrap();
        assert!(
            api::execute(
                &store,
                "export",
                json!({"format":"amg","initiative":"alpha","destination":"outside-link/leak.amg"}),
                &opts
            )
            .is_error()
        );
        assert!(!dir.path().join("leak.amg").exists());
    }
}
#[test]
fn replay_preserves_the_artifact_and_rechecks_authorization() {
    let store = Store::open_in_memory().unwrap();
    note(&store, "alpha", "A", "a");
    let dir = TempDir::new().unwrap();
    let opts = options(&dir);
    let args = json!({"format":"amg","initiative":"alpha","destination":"repeat","request_id":"export-one"});
    let first = api::execute(&store, "export", args.clone(), &opts);
    let path = artifact(&first).path.clone();
    let original = fs::read(&path).unwrap();
    let again = api::execute(&store, "export", args.clone(), &opts);
    assert!(again.replayed);
    assert_eq!(fs::read(path).unwrap(), original);
    let mut restricted = opts;
    restricted.denied_initiatives = vec!["alpha".into()];
    assert!(api::execute(&store, "export", args, &restricted).is_error());
}
#[test]
fn competing_stores_cannot_overwrite_the_same_archive() {
    let dir = TempDir::new().unwrap();
    let opts = options(&dir);
    let gate = Arc::new(Barrier::new(2));
    let threads = (0..2)
        .map(|i| {
            let opts = opts.clone();
            let gate = gate.clone();
            std::thread::spawn(move || {
                let store = Store::open_in_memory().unwrap();
                note(&store, "alpha", "A", &format!("writer-{i}"));
                gate.wait();
                api::execute(
                    &store,
                    "export",
                    json!({"format":"amg","initiative":"alpha","destination":"race"}),
                    &opts,
                )
            })
        })
        .collect::<Vec<_>>();
    let responses = threads
        .into_iter()
        .map(|t| t.join().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(responses.iter().filter(|r| !r.is_error()).count(), 1);
    let final_doc = amg::decode(
        fs::File::open(opts.artifact_root.unwrap().join("race.amg")).unwrap(),
        &AmgLimits::default(),
    )
    .unwrap();
    assert_eq!(final_doc.data.nodes.len(), 1);
}
