//! MCP and Rig expose the same names and complete input schemas.
//!
//! Both adapters consume the core catalogue. These tests compare the actual
//! registrations, including nested definitions and required action variants.
//! Deliberate transport-only operations must be listed in `DAEMON_ONLY`.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use kaeru_core::Store;
use kaeru_rig::KaeruMemory;
use serde_json::Value;
use tokio_util::sync::CancellationToken;

use crate::cloud_client::CloudRegistry;
use crate::server::KaeruServer;

/// Verbs the daemon has and the adapter deliberately does not, with why.
///
/// Keep it short and keep the reason honest: every entry is a place where an
/// agent's vocabulary depends on how it reached kaeru.
const DAEMON_ONLY: &[(&str, &str)] = &[];

type Shape = Value;
fn shape_of(schema: &Value) -> Shape {
    schema.clone()
}

fn daemon_surface() -> BTreeMap<String, Shape> {
    // Read actual registrations, the same catalogue a client receives.
    let server = KaeruServer::new(
        Store::open_in_memory().expect("open"),
        CloudRegistry::default(),
        CancellationToken::new(),
        false,
    );
    let _server = server;
    KaeruServer::tool_definitions()
        .into_iter()
        .map(|t| {
            let schema = Value::Object((*t.input_schema).clone());
            (t.name.to_string(), shape_of(&schema))
        })
        .collect()
}

async fn adapter_surface() -> BTreeMap<String, Shape> {
    let store = Arc::new(Store::open_in_memory().expect("open"));
    let mem = KaeruMemory::with_clouds(store, "parity", kaeru_rig::CloudRegistry::default());
    let mut out = BTreeMap::new();
    for def in mem
        .local_tool_definitions()
        .await
        .into_iter()
        .chain(mem.cloud_tool_definitions().await)
    {
        let verb = def
            .name
            .strip_prefix("kaeru_")
            .unwrap_or(&def.name)
            .to_string();
        out.insert(verb, shape_of(&def.parameters));
    }
    out
}

#[tokio::test]
async fn the_two_surfaces_offer_the_same_verbs() {
    let daemon = daemon_surface();
    let adapter = adapter_surface().await;
    let allowed: BTreeSet<&str> = DAEMON_ONLY.iter().map(|(verb, _)| *verb).collect();

    let missing: Vec<&String> = daemon
        .keys()
        .filter(|v| !adapter.contains_key(*v) && !allowed.contains(v.as_str()))
        .collect();
    let extra: Vec<&String> = adapter
        .keys()
        .filter(|v| !daemon.contains_key(*v))
        .collect();

    assert!(
        missing.is_empty() && extra.is_empty(),
        "the rig adapter must offer the daemon's verbs and no others.\n  \
         missing from rig: {missing:?}\n  not in the daemon: {extra:?}"
    );
    for (verb, _) in DAEMON_ONLY {
        assert!(
            daemon.contains_key(*verb),
            "`{verb}` is excused from parity but the daemon no longer has it"
        );
    }
}

#[tokio::test]
async fn the_same_verb_takes_the_same_parameters() {
    let daemon = daemon_surface();
    let adapter = adapter_surface().await;

    let mut differ = Vec::new();
    for (verb, want) in &daemon {
        let Some(got) = adapter.get(verb) else {
            continue; // reported by the verb-set test
        };
        if want != got {
            differ.push(format!("  {verb}: complete input schemas differ"));
        }
    }
    assert!(
        differ.is_empty(),
        "a verb must take the same parameters on both surfaces:\n{}",
        differ.join("\n")
    );
}
