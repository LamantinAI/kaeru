//! Real binary commands use only disposable vaults and files.
use kaeru_core::config::amg::AmgLimits;
use kaeru_core::models::amg::AmgSelection;
use kaeru_core::{
    KaeruConfig, Store, amg,
    api::{ApiOptions, ResponseData, execute},
};
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

fn command(home: &Path, args: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_kaeru-mcp"));
    for (key, _) in std::env::vars().filter(|(key, _)| key.starts_with("KAERU_")) {
        command.env_remove(key);
    }
    command
        .env("HOME", home)
        .env("KAERU_MCP_WEB_CONFIG_FILE", home.join("web.toml"))
        .env("KAERU_MCP_CLOUDS_FILE", home.join("clouds.toml"))
        .env("KAERU_READ_TRACE_ENABLED", "false")
        .args(args)
        .output()
        .unwrap()
}
fn seed(path: &Path) -> String {
    let store = Store::open_with_config(KaeruConfig {
        vault_path: path.into(),
        ..Default::default()
    })
    .unwrap();
    let response = execute(
        &store,
        "note",
        serde_json::json!({"initiative":"alpha","name":"Portable","body":"Full source: перенос знаний","kind":"observation"}),
        &ApiOptions::local(),
    );
    match response.data {
        ResponseData::Mutation(data) => data.primary.unwrap().id,
        other => panic!("{other:?}"),
    }
}
#[test]
fn binary_exports_inspects_and_imports_without_touching_an_unrelated_vault() {
    let home = tempfile::tempdir().unwrap();
    let source = home.path().join("source");
    let target = home.path().join("target");
    let file = home.path().join("knowledge.amg");
    let id = seed(&source);
    let args = [
        "amg",
        "export",
        "--vault",
        source.to_str().unwrap(),
        "--initiative",
        "alpha",
        "--output",
        file.to_str().unwrap(),
    ];
    let exported = command(home.path(), &args);
    assert!(
        exported.status.success(),
        "{}",
        String::from_utf8_lossy(&exported.stderr)
    );
    assert!(file.exists());
    let inspected = command(home.path(), &["amg", "inspect", file.to_str().unwrap()]);
    assert!(
        inspected.status.success(),
        "{}",
        String::from_utf8_lossy(&inspected.stderr)
    );
    let view: serde_json::Value = serde_json::from_slice(&inspected.stdout).unwrap();
    assert_eq!(
        view["metadata"]["initiatives"],
        serde_json::json!(["alpha"])
    );
    assert_eq!(view["nodes"], 1);
    let original = fs::read(&file).unwrap();
    assert!(!command(home.path(), &args).status.success());
    assert_eq!(fs::read(&file).unwrap(), original);
    let import = [
        "amg",
        "import",
        file.to_str().unwrap(),
        "--vault",
        target.to_str().unwrap(),
    ];
    let first = command(home.path(), &import);
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&first.stdout).unwrap();
    assert_eq!(report["created_nodes"], 1);
    let repeated = command(home.path(), &import);
    assert!(repeated.status.success());
    let report: serde_json::Value = serde_json::from_slice(&repeated.stdout).unwrap();
    assert_eq!(report["created_nodes"], 0);
    let store = Store::open(&target).unwrap();
    assert!(kaeru_core::read_node_full(&store, &id).unwrap().is_some());
}
#[test]
fn unsupported_schema_and_bad_arguments_fail_before_creating_destination() {
    let home = tempfile::tempdir().unwrap();
    let source = Store::open_in_memory_with(KaeruConfig::default()).unwrap();
    let mut bytes = amg::encode(
        &amg::snapshot(&source, &AmgSelection::All, &AmgLimits::default()).unwrap(),
        &AmgLimits::default(),
    )
    .unwrap();
    let key = b"schema_version";
    let index = bytes
        .windows(key.len())
        .position(|bytes| bytes == key)
        .unwrap()
        + key.len();
    assert_eq!(bytes[index], 1);
    bytes[index] = 2;
    let file = home.path().join("future.amg");
    let target = home.path().join("must-not-exist");
    fs::write(&file, bytes).unwrap();
    let result = command(
        home.path(),
        &[
            "amg",
            "import",
            file.to_str().unwrap(),
            "--vault",
            target.to_str().unwrap(),
            "--force",
        ],
    );
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("unsupported"));
    assert!(!target.exists());
    assert!(
        !command(
            home.path(),
            &[
                "amg",
                "export",
                "--vault",
                target.to_str().unwrap(),
                "--output",
                file.to_str().unwrap()
            ]
        )
        .status
        .success()
    );
    assert!(!target.exists());
}
#[test]
fn busy_vault_is_not_opened_a_second_time() {
    let home = tempfile::tempdir().unwrap();
    let path = home.path().join("busy");
    let _held = Store::open_with_config(KaeruConfig {
        vault_path: path.clone(),
        ..Default::default()
    })
    .unwrap();
    let output = home.path().join("must-not-exist.amg");
    let result = command(
        home.path(),
        &[
            "amg",
            "export",
            "--vault",
            path.to_str().unwrap(),
            "--all",
            "--output",
            output.to_str().unwrap(),
        ],
    );
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("Stop the daemon"));
    assert!(!output.exists());
}
#[test]
fn root_help_and_auth_commands_remain_available() {
    let home = tempfile::tempdir().unwrap();
    let help = command(home.path(), &["--help"]);
    assert!(help.status.success());
    let text = String::from_utf8_lossy(&help.stdout);
    assert!(text.contains("amg") && text.contains("--auth"));
    let auth = command(home.path(), &["--auth", "disabled"]);
    assert!(
        auth.status.success(),
        "{}",
        String::from_utf8_lossy(&auth.stderr)
    );
    let status = command(home.path(), &["--auth-status"]);
    assert_eq!(String::from_utf8_lossy(&status.stdout).trim(), "disabled");
}

#[test]
fn an_older_disk_vault_gains_amg_relations_and_keeps_import_provenance_after_reopen() {
    let home = tempfile::tempdir().unwrap();
    let target = home.path().join("older");
    let sentinel = seed(&target);
    {
        let store = Store::open(&target).unwrap();
        store.run("::remove amg_origin").unwrap();
        store.run("::remove amg_import").unwrap();
        store
            .run("?[name] <- [['0012_amg_provenance']] :rm migration_journal {name}")
            .unwrap();
    }
    let source_path = home.path().join("source");
    let imported = seed(&source_path);
    let file = home.path().join("migration.amg");
    let output = command(
        home.path(),
        &[
            "amg",
            "export",
            "--vault",
            source_path.to_str().unwrap(),
            "--all",
            "--output",
            file.to_str().unwrap(),
        ],
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = command(
        home.path(),
        &[
            "amg",
            "import",
            file.to_str().unwrap(),
            "--vault",
            target.to_str().unwrap(),
        ],
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    for _ in 0..2 {
        let store = Store::open(&target).unwrap();
        assert!(
            kaeru_core::read_node_full(&store, &sentinel)
                .unwrap()
                .is_some()
        );
        let doc = amg::snapshot(&store, &AmgSelection::All, &AmgLimits::default()).unwrap();
        assert_eq!(
            doc.data
                .nodes
                .iter()
                .find(|node| node.id == imported)
                .unwrap()
                .origins
                .len(),
            1
        );
        assert_eq!(
            store
                .run_read("?[id] := *amg_import{id}")
                .unwrap()
                .rows
                .len(),
            1
        );
        assert_eq!(
            store
                .run_read("?[name] := *migration_journal{name},name='0012_amg_provenance'")
                .unwrap()
                .rows
                .len(),
            1
        );
    }
}
