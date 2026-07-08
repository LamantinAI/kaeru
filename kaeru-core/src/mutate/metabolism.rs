//! Metabolism — graph hygiene mutations. `forget` retracts a node and
//! every connected edge; `improve` rewrites name/body in place via
//! retract+reassert.

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};

use super::{
    ReassertRow, build_body_tags, merge_tags, now_validity_seconds, read_connected_edges,
    read_node_now, reassert_node_now, retract_node_at,
};
use crate::errors::{Error, Result};
use crate::graph::NodeId;
use crate::graph::audit::write_audit;
use crate::store::Store;

/// Bi-temporal forget: retracts a node and every edge connected to it
/// (inbound or outbound) at NOW. Non-destructive — historical assertions
/// stay in the substrate, so `at(t)` for `t` before the forget still
/// resolves the node and its edges normally. Reads at NOW will simply
/// skip them.
///
/// Used when a node was a mistake or genuine garbage; for content the
/// agent wants to keep but rewrite, see [`improve`].
pub fn forget(store: &Store, node_id: &NodeId) -> Result<()> {
    let edges = read_connected_edges(store, node_id)?;

    // Retract each connected edge with a single timestamp; ordering of
    // retractions inside a single forget call is irrelevant.
    let edge_secs = now_validity_seconds();
    for (src, dst, edge_type) in &edges {
        let mut p: BTreeMap<String, DataValue> = BTreeMap::new();
        p.insert("src".to_string(), DataValue::Str(src.clone().into()));
        p.insert("dst".to_string(), DataValue::Str(dst.clone().into()));
        p.insert(
            "edge_type".to_string(),
            DataValue::Str(edge_type.clone().into()),
        );
        let s = format!(
            r#"
            ?[src, dst, edge_type, validity, weight, properties] <-
                [[$src, $dst, $edge_type, [{edge_secs}.0, false], 1.0, null]]
            :put edge {{src, dst, edge_type, validity => weight, properties}}
            "#
        );
        store
            .db_ref()
            .run_script(&s, p, ScriptMutability::Mutable)?;
    }

    // Retract the node itself.
    let node_secs = now_validity_seconds();
    let mut p_node: BTreeMap<String, DataValue> = BTreeMap::new();
    p_node.insert("id".to_string(), DataValue::Str(node_id.clone().into()));
    let s_node = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{node_secs}.0, false], 'placeholder', 'operational', '', null, null, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store
        .db_ref()
        .run_script(&s_node, p_node, ScriptMutability::Mutable)?;

    write_audit(store.db_ref(), "forget", "system", &[node_id.clone()])?;
    Ok(())
}

/// Rewrites a node's `name` and `body` while preserving everything else:
/// `type`, `tier`, `layer`, `visibility`, `properties`, initiative
/// memberships, and any tags outside the re-derived `lang:` / `topic:`
/// families (manual tags survive; `lang:` / `topic:` are re-derived from
/// the new body, so stale topics don't accumulate across rewrites). Implemented as re-assert + retract through the bi-temporal
/// substrate, so `history` shows both the old version and the new one.
pub fn improve(store: &Store, node_id: &NodeId, new_name: &str, new_body: &str) -> Result<()> {
    let current = read_node_now(store, node_id)?
        .ok_or_else(|| Error::NotFound(format!("node {node_id} not found at NOW")))?;

    let kind_tag = format!("kind:{}", current.type_);
    let fresh = build_body_tags(&[kind_tag.as_str(), "role:revised"], new_body);
    let tags = merge_tags(&current.tags, &["lang:", "topic:"], fresh);

    // Re-assert first, retract second, same timestamp — see
    // `reassert_node_now` for the ordering invariant.
    let secs = now_validity_seconds();
    reassert_node_now(
        store,
        node_id,
        ReassertRow {
            secs,
            type_: &current.type_,
            tier: &current.tier,
            name: new_name,
            body: Some(new_body),
            tags,
            visibility: &current.visibility,
            layer: &current.layer,
        },
    )?;
    retract_node_at(store, node_id, secs)?;

    write_audit(store.db_ref(), "improve", "system", &[node_id.clone()])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::graph::{Layer, Visibility};
    use crate::store::Store;
    use crate::{at, improve, jot_with_layer, set_visibility};

    /// `improve` used to write the new revision with an incomplete column
    /// list, silently resetting `layer` to `warm` and `visibility` to
    /// `local` (schema defaults) — a core+shared node fell out of the awake
    /// injection band and out of the cloud on every revise.
    #[test]
    fn improve_preserves_layer_and_visibility() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        let id = jot_with_layer(&store, "draft body", Layer::Core).expect("jot");
        set_visibility(&store, &id, Visibility::Shared).expect("set vis");

        // Whole-second validity: a same-second retract would collide with
        // the creation row, so cross the boundary like the rest of the suite.
        std::thread::sleep(Duration::from_millis(1100));
        improve(&store, &id, "revised-name", "revised body").expect("improve");

        let snap = at(&store, &id, 9_999_999_999.0)
            .expect("at")
            .expect("still resolves");
        assert_eq!(snap.name, "revised-name");
        assert_eq!(snap.body.as_deref(), Some("revised body"));
        assert_eq!(snap.layer, "core", "layer survives the rewrite");
        assert_eq!(snap.visibility, "shared", "visibility survives the rewrite");
        assert!(
            snap.tags.iter().any(|t| t == "role:revised"),
            "fresh tags merged in: {:?}",
            snap.tags
        );
        assert!(
            !snap.tags.iter().any(|t| t == "topic:draft"),
            "stale topic tags are re-derived, not accumulated: {:?}",
            snap.tags
        );
        assert!(
            snap.tags.iter().any(|t| t == "topic:revised"),
            "new body's topics present: {:?}",
            snap.tags
        );
    }
}
