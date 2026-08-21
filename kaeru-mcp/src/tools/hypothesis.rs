//! Hypothesis-experiment cycle: `claim`, `test`, `confirm`, `refute`.

use kaeru_core::{EdgeType, HypothesisStatus, Store, Visibility, get_visibility};
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{
    capture_result, claim_verdict_hint, derive_auto_name, parse_layer, resolve_name, text, to_mcp,
    with_initiative,
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
            &format!(
                "claimed: {auto_name} — {id}{}",
                claim_verdict_hint(&auto_name)
            ),
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
        let mut msg = format!("confirmed: {hypothesis}");
        if get_visibility(store, &hyp_id).map_err(to_mcp)? == Visibility::Shared {
            msg.push_str(
                "\n⚠ cloud copy is stale — run `share` on this node to push the new version.",
            );
        }
        Ok(text(&msg))
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
        let mut msg = format!("refuted: {hypothesis}");
        if get_visibility(store, &hyp_id).map_err(to_mcp)? == Visibility::Shared {
            msg.push_str(
                "\n⚠ cloud copy is stale — run `share` on this node to push the new version.",
            );
        }
        Ok(text(&msg))
    })
}

#[cfg(test)]
mod tests {
    use kaeru_core::Store;
    use rmcp::model::CallToolResult;

    use super::claim;

    fn text_of(r: CallToolResult) -> String {
        r.content
            .iter()
            .filter_map(|c| c.as_text().map(|t| t.text.clone()))
            .collect::<Vec<_>>()
            .join("")
    }

    /// 15 of 21 hypotheses in the live graph are open forever, and `test` has
    /// never been called. A claim is a promise to settle it later, so the
    /// capture itself names the verbs that settle it.
    #[test]
    fn a_claim_names_the_verbs_that_settle_it() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        let out =
            text_of(claim(&store, "the cache pays for itself", None, None, Some("t")).unwrap());
        assert!(
            out.contains("`confirm ") && out.contains("`refute "),
            "both verdict verbs: {out}"
        );
        assert!(
            out.contains("--by <evidence>"),
            "with the evidence arg: {out}"
        );
        assert!(
            out.contains("tagged \"status:open\""),
            "and how to list the ones still waiting: {out}"
        );
    }
}
