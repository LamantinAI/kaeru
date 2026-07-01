//! Hypothesis-experiment cycle: `claim`, `test`, `confirm`, `refute`.

use kaeru_core::{EdgeType, HypothesisStatus, Store};
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{
    capture_result, derive_auto_name, parse_layer, resolve_name, text, to_mcp, with_initiative,
};

pub fn claim(
    store: &Store,
    text_arg: &str,
    about: Option<&str>,
    layer: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let auto_name = derive_auto_name(text_arg, "claim");
        let layer = parse_layer(layer)?;
        let id = kaeru_core::formulate_hypothesis_with_layer(store, &auto_name, text_arg, layer)
            .map_err(to_mcp)?;
        if let Some(a) = about {
            let target = resolve_name(store, a)?;
            kaeru_core::link(store, &id, &target, EdgeType::RefersTo).map_err(to_mcp)?;
        }
        Ok(capture_result(
            store,
            &id,
            initiative,
            &format!("claimed: {auto_name} — {id}"),
        ))
    })
}

pub fn test_hypothesis(
    store: &Store,
    hypothesis: &str,
    method: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let hyp_id = resolve_name(store, hypothesis)?;
        let auto_name = derive_auto_name(method, "experiment");
        let exp_id =
            kaeru_core::run_experiment(store, &hyp_id, &auto_name, method).map_err(to_mcp)?;
        Ok(text(&format!("experiment: {auto_name} — {exp_id}")))
    })
}

pub fn confirm(
    store: &Store,
    hypothesis: &str,
    by: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let hyp_id = resolve_name(store, hypothesis)?;
        let by_id = resolve_name(store, by)?;
        kaeru_core::update_hypothesis_status(store, &hyp_id, HypothesisStatus::Supported, &by_id)
            .map_err(to_mcp)?;
        Ok(text(&format!("confirmed: {hypothesis}")))
    })
}

pub fn refute(
    store: &Store,
    hypothesis: &str,
    by: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let hyp_id = resolve_name(store, hypothesis)?;
        let by_id = resolve_name(store, by)?;
        kaeru_core::update_hypothesis_status(store, &hyp_id, HypothesisStatus::Refuted, &by_id)
            .map_err(to_mcp)?;
        Ok(text(&format!("refuted: {hypothesis}")))
    })
}
