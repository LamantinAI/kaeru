//! Hygiene tools: `forget`, `revise`, `layer`.

use std::str::FromStr;

use kaeru_core::{Error, Layer, Store, Visibility, get_visibility, set_layer as core_set_layer};
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{resolve_name_or_id, text, to_mcp, with_initiative};

pub fn forget(
    store: &Store,
    name_or_id: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name_or_id)?;
        kaeru_core::forget(store, &id).map_err(to_mcp)?;
        Ok(text(&format!("forgot: {name_or_id}")))
    })
}

pub fn set_layer(
    store: &Store,
    name_or_id: &str,
    layer: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let parsed = Layer::from_str(layer).map_err(to_mcp)?;
        let id = resolve_name_or_id(store, name_or_id)?;
        core_set_layer(store, &id, parsed).map_err(to_mcp)?;
        Ok(text(&format!("layer: {name_or_id} → {}", parsed.as_str())))
    })
}

pub fn revise(
    store: &Store,
    name: &str,
    body: Option<&str>,
    rename: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name)?;
        let brief = kaeru_core::node_brief_by_id(store, &id)
            .map_err(to_mcp)?
            .ok_or_else(|| to_mcp(Error::NotFound(format!("node {name:?} not found at NOW"))))?;
        let new_name = rename.unwrap_or(&brief.name);
        let preserved_body = if body.is_none() {
            // Read the body through `at` — the full snapshot. `summary_view`
            // returns a truncated excerpt, and a rename-only revise used to
            // silently replace a long body with that excerpt. The substrate
            // stores whole-second validities and rejects fractional ones, so
            // round up to cover a node asserted within the current second.
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| (d.as_secs() + 1) as f64)
                .unwrap_or(0.0);
            kaeru_core::at(store, &id, now)
                .map_err(to_mcp)?
                .and_then(|snap| snap.body)
                .unwrap_or_default()
        } else {
            String::new()
        };
        let new_body = body.unwrap_or(&preserved_body);
        kaeru_core::improve(store, &id, new_name, new_body).map_err(to_mcp)?;
        let mut msg = format!("revised: {name} → {new_name}");
        if get_visibility(store, &id).map_err(to_mcp)? == Visibility::Shared {
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

    use super::revise;

    /// A rename-only revise must carry the FULL body over — it used to read
    /// the preserved body through `summary_view`, whose excerpt truncates,
    /// silently shortening any long node on rename.
    #[test]
    fn rename_only_revise_preserves_full_body() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        let long_body = "word ".repeat(400); // far beyond any excerpt cap
        let id = kaeru_core::jot(&store, &long_body).expect("jot");

        // Cross the whole-second validity boundary, like the core suite.
        std::thread::sleep(std::time::Duration::from_millis(1100));
        revise(&store, &id, None, Some("renamed-note"), Some("t")).expect("revise");

        let snap = kaeru_core::at(&store, &id, 9_999_999_999.0)
            .expect("at")
            .expect("resolves");
        assert_eq!(snap.name, "renamed-note");
        assert_eq!(
            snap.body.as_deref(),
            Some(long_body.as_str()),
            "full body survives a rename-only revise"
        );
    }
}
