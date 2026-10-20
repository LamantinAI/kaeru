//! Client schema presentation must retain the canonical validation contract.

use std::sync::OnceLock;

use jsonschema::Validator;
use jsonschema::draft202012::options;
use kaeru_core::api;
use proptest::arbitrary::any;
use proptest::collection::{btree_map, vec};
use proptest::prelude::{Just, ProptestConfig, Strategy};
use proptest::sample::select;
use proptest::{prop_assert_eq, prop_oneof, proptest};
use serde_json::{Value, json};

fn advertised_validators() -> &'static Vec<Validator> {
    static VALIDATORS: OnceLock<Vec<Validator>> = OnceLock::new();
    VALIDATORS.get_or_init(|| {
        api::tool_definitions()
            .iter()
            .map(|tool| {
                options()
                    .should_validate_formats(true)
                    .build(&tool.input_schema)
                    .unwrap()
            })
            .collect()
    })
}

#[test]
fn conditional_and_union_requests_keep_their_original_acceptance() {
    let cases = [
        (
            "cloud_recall",
            json!({"initiative":"p","id":"node-a"}),
            true,
        ),
        ("cloud_recall", json!({"initiative":"p","id":""}), false),
        (
            "cloud_recall",
            json!({"initiative":"p","unknown":"node-a"}),
            false,
        ),
        ("context", json!({}), false),
        ("context", json!({"view":"diagnostics"}), true),
        ("context", json!({"initiative":"p"}), true),
        (
            "context",
            json!({"initiative":"p","recorded_at":"2026-10-01T00:00:00Z"}),
            false,
        ),
        (
            "context",
            json!({"initiative":"p","view":"board","recorded_at":"2026-10-01T00:00:00Z"}),
            true,
        ),
        ("search", json!({"initiatives":["p"]}), false),
        ("search", json!({"initiatives":["p"],"text":"term"}), true),
        (
            "search",
            json!({"initiatives":["p"],"filters":{"layers":["cold"]}}),
            true,
        ),
        (
            "reference",
            json!({"initiative":"p","name":"source"}),
            false,
        ),
        (
            "reference",
            json!({"initiative":"p","name":"source","source":{"text":"Exact."}}),
            true,
        ),
        (
            "note",
            json!({"initiative":"p","body":"rule","kind":"constraint","options":{"layer":"core"}}),
            true,
        ),
        (
            "note",
            json!({"initiative":"p","body":"rule","type":"constraint"}),
            false,
        ),
        (
            "note",
            json!({"initiative":"p","body":"rule","applicability":{"through":"2026-11-01"}}),
            false,
        ),
        (
            "note",
            json!({"initiative":"p","body":"rule","kind":"constraint","applicability":{"through":"2026-11-01"}}),
            true,
        ),
        (
            "note",
            json!({"initiative":"p","body":"rule","options":{"expected_slot_revision":"r"}}),
            false,
        ),
        (
            "note",
            json!({"initiative":"p","body":"rule","options":{"slot":"handoff","expected_slot_revision":"r"}}),
            true,
        ),
        (
            "note",
            json!({"initiative":"p","body":"rule","sharing":{"visibility":"local","cloud":"team"}}),
            false,
        ),
        ("read", json!({"initiative":"p","node":"name"}), true),
        ("read", json!({"initiative":"p","name":"name"}), false),
        (
            "read",
            json!({"initiative":"p","node":"name","include":["path"]}),
            false,
        ),
        (
            "read",
            json!({"initiative":"p","node":"name","include":["path"],"to":"other"}),
            true,
        ),
        (
            "read",
            json!({"initiative":"p","node":"name","edge_types":["derived_from"]}),
            false,
        ),
        (
            "read",
            json!({"initiative":"p","node":"name","include":["relations"],"edge_types":["derived_from"]}),
            true,
        ),
        (
            "read",
            json!({"initiative":"p","node":"name","include":["relations"],"recorded_at":"2026-10-01T00:00:00Z"}),
            false,
        ),
        (
            "verdict",
            json!({"initiative":"p","hypothesis":"name","status":"inconclusive"}),
            true,
        ),
        (
            "verdict",
            json!({"initiative":"p","hypothesis":"name","status":"open"}),
            false,
        ),
    ];
    for (name, args, expected) in cases {
        let index = api::tool_definitions()
            .iter()
            .position(|t| t.name == name)
            .unwrap();
        assert_eq!(
            api::validate(name, &args).is_ok(),
            expected,
            "canonical {name}: {args}"
        );
        assert_eq!(
            advertised_validators()[index].is_valid(&args),
            expected,
            "advertised {name}: {args}"
        );
    }
}

fn json_value() -> impl Strategy<Value = Value> {
    let leaf = prop_oneof![
        Just(Value::Null),
        any::<bool>().prop_map(Value::Bool),
        (-2_i32..5).prop_map(|n| json!(n)),
        select(vec![
            "p",
            "name",
            "core",
            "constraint",
            "diagnostics",
            "board",
            "path",
            "relations",
            "inconclusive",
            "2026-10-01T00:00:00Z"
        ])
        .prop_map(|s| json!(s)),
    ];
    leaf.prop_recursive(2, 20, 4, |inner| {
        prop_oneof![
            vec(inner.clone(), 0..4).prop_map(Value::Array),
            btree_map(
                select(vec![
                    "name",
                    "id",
                    "text",
                    "layer",
                    "slot",
                    "through",
                    "visibility",
                    "cloud",
                    "include"
                ]),
                inner,
                0..4
            )
            .prop_map(|map| json!(map)),
        ]
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]
    #[test]
    fn advertised_and_canonical_catalogues_agree_on_generated_inputs(
        arguments in btree_map(
            select(vec!["initiative", "initiatives", "body", "name", "node", "kind", "type", "options", "applicability", "sharing", "view", "text", "filters", "action", "status", "to", "include", "edge_types", "recorded_at", "source", "url", "request_id"]),
            json_value(), 0..12,
        )
    ) {
        let args = json!(arguments);
        for (tool, validator) in api::tool_definitions().iter().zip(advertised_validators()) {
            prop_assert_eq!(api::validate(&tool.name, &args).is_ok(), validator.is_valid(&args), "{}: {}", tool.name, args);
        }
    }
}
