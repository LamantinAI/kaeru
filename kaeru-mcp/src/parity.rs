//! The two surfaces are one API — this is what keeps them that way (#98).
//!
//! `kaeru-rig` is the in-process form of the same curator API this daemon
//! serves over MCP, and an agent must see the same verbs with the same
//! parameters whichever way it reaches kaeru. They drifted: at 0.7.3 the
//! daemon had 71 tools and the adapter 61, `kaeru_recall` on the rig side was
//! this crate's `search`, half the rig tools could not take an `initiative`
//! at all, and `link`'s deliberately-required `weight` was optional there.
//! Albert configures memory both ways and names its tools in its prompt, so
//! every one of those differences broke one of the two modes.
//!
//! The test compares the **names** and, per tool, the **property names and
//! the required set**. Not the descriptions: the wording is written for two
//! different readers and a shared phrasing would be a worse fit for both.
//! Not the JSON types either — `schemars` and a hand-written schema disagree
//! about how to spell the same thing often enough that asserting on it would
//! teach people to edit the test.
//!
//! When a verb is genuinely daemon-only, add it to [`DAEMON_ONLY`] with the
//! reason. That list is the whole allowance; anything else is a diff and this
//! test fails naming it.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use kaeru_core::Store;
use kaeru_rig::KaeruMemory;
use rmcp::handler::server::router::tool::ToolRouter;
use serde_json::Value;
use tokio_util::sync::CancellationToken;

use crate::cloud_client::CloudRegistry;
use crate::server::KaeruServer;

/// Verbs the daemon has and the adapter deliberately does not, with why.
///
/// Keep it short and keep the reason honest: every entry is a place where an
/// agent's vocabulary depends on how it reached kaeru.
const DAEMON_ONLY: &[(&str, &str)] = &[(
    "import",
    "the bulk-import playbook is written for a session that shells into a vault; \
     an embedder imports through its own code path",
)];

/// The shape of one tool's parameters, compared across the two surfaces.
#[derive(Debug, PartialEq, Eq)]
struct Shape {
    properties: BTreeSet<String>,
    required: BTreeSet<String>,
}

fn shape_of(schema: &Value) -> Shape {
    let names = |key: &str| -> BTreeSet<String> {
        match key {
            "properties" => schema
                .get("properties")
                .and_then(Value::as_object)
                .map(|o| o.keys().cloned().collect())
                .unwrap_or_default(),
            _ => schema
                .get("required")
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .filter_map(Value::as_str)
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_default(),
        }
    };
    Shape {
        properties: names("properties"),
        required: names("required"),
    }
}

fn daemon_surface() -> BTreeMap<String, Shape> {
    // The router is macro-generated and private, so read it off a server —
    // which is also the shape a client meets.
    let server = KaeruServer::new(
        Store::open_in_memory().expect("open"),
        CloudRegistry::default(),
        CancellationToken::new(),
        false,
    );
    let router: &ToolRouter<KaeruServer> = server.router();
    router
        .list_all()
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
            differ.push(format!(
                "  {verb}:\n    daemon: properties {:?} required {:?}\n    rig:    properties {:?} required {:?}",
                want.properties, want.required, got.properties, got.required
            ));
        }
    }
    assert!(
        differ.is_empty(),
        "a verb must take the same parameters on both surfaces:\n{}",
        differ.join("\n")
    );
}
