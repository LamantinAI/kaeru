//! Hypothesis-experiment cycle: `claim`, `test`, `confirm`, `refute`.

use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use kaeru_core::EdgeType;
use kaeru_core::HypothesisStatus;
use kaeru_core::Store;

use crate::utils::derive_auto_name;
use crate::utils::resolve_name;
use crate::utils::text;
use crate::utils::to_mcp;
use crate::utils::with_initiative;

pub fn claim(
    store: &Store,
    text_arg: &str,
    about: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let auto_name = derive_auto_name(text_arg, "claim");
        let id = kaeru_core::formulate_hypothesis(store, &auto_name, text_arg)
            .map_err(to_mcp)?;
        if let Some(a) = about {
            let target = resolve_name(store, a)?;
            kaeru_core::link(store, &id, &target, EdgeType::RefersTo).map_err(to_mcp)?;
        }
        Ok(text(&format!("claimed: {auto_name} — {id}")))
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
        let exp_id = kaeru_core::run_experiment(store, &hyp_id, &auto_name, method)
            .map_err(to_mcp)?;
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
        kaeru_core::update_hypothesis_status(
            store,
            &hyp_id,
            HypothesisStatus::Supported,
            &by_id,
        )
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
        kaeru_core::update_hypothesis_status(
            store,
            &hyp_id,
            HypothesisStatus::Refuted,
            &by_id,
        )
        .map_err(to_mcp)?;
        Ok(text(&format!("refuted: {hypothesis}")))
    })
}
