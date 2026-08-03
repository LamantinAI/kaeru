//! Sharing controls: node `visibility` (the per-node share flag) and
//! initiative `share_policy` (Gate 1). Both are deliberately low-friction:
//! the default is the safe one (`local` / `private`), and changing either
//! is an explicit, audited act — never an automatic agent decision.

use std::collections::BTreeMap;
use std::str::FromStr;

use cozo::{DataValue, ScriptMutability};

use super::rewrite_node_column_in_place;
use crate::errors::Result;
use crate::graph::audit::write_audit;
use crate::graph::{NodeId, SharePolicy, Visibility};
use crate::store::Store;

/// Changes a node's `visibility`, preserving every other attribute
/// (including `layer`). In-place rewrite of the node's current row at its
/// exact `validity` key — no new validity version is minted, mirroring
/// `set_layer`, so an `@ 'NOW'` read can never resolve two competing
/// versions.
///
/// Promotion `Local → Shared` is meant to be an explicit human act; this
/// primitive performs the flip but does not itself sync anything. Actual
/// sync stays gated by the initiative's `SharePolicy` and the pre-share
/// guard.
///
/// The read prefers the `@ 'NOW'` view; if the node is not visible at NOW
/// it falls back to the latest historical version, so the verb also
/// recovers a node left invisible by an earlier buggy rewrite.
pub fn set_visibility(store: &Store, node_id: &NodeId, visibility: Visibility) -> Result<()> {
    // Shares the generated rewrite with `set_layer`: the column list comes
    // from `NODE_VALUE_COLUMNS`, so nothing here needs updating when the
    // schema grows a column — and nothing silently resets to a default.
    rewrite_node_column_in_place(store, node_id, "visibility", visibility.as_str())?;

    write_audit(
        store.db_ref(),
        "set_visibility",
        "system",
        &[node_id.clone()],
    )?;
    Ok(())
}

/// Returns a node's current `visibility`, defaulting to `Local` if unset.
pub fn get_visibility(store: &Store, node_id: &NodeId) -> Result<Visibility> {
    let script = format!(
        r#"
        ?[visibility] := *node{{id, visibility @ 'NOW'}}, id = '{node_id}'
        "#
    );
    let rows = store.run_read(&script)?;
    let vis_str = rows
        .rows
        .first()
        .and_then(|row| row.first())
        .and_then(|v| v.get_str())
        .unwrap_or("local");
    Visibility::from_str(vis_str)
}

/// Sets the sticky `share_policy` for an initiative (Gate 1). Upserts the
/// `initiative` row. This is the one-time classification; it persists and
/// is not re-asked per capture.
pub fn set_share_policy(store: &Store, initiative: &str, policy: SharePolicy) -> Result<()> {
    let mut p: BTreeMap<String, DataValue> = BTreeMap::new();
    p.insert("name".to_string(), DataValue::Str(initiative.into()));
    p.insert("policy".to_string(), DataValue::Str(policy.as_str().into()));
    let script = r#"
        ?[name, share_policy] <- [[$name, $policy]]
        :put initiative {name => share_policy}
    "#;
    store
        .db_ref()
        .run_script(script, p, ScriptMutability::Mutable)?;

    write_audit(
        store.db_ref(),
        "set_share_policy",
        "system",
        &[initiative.to_string()],
    )?;
    Ok(())
}

/// Returns an initiative's `share_policy`, defaulting to `Private` when the
/// initiative has no explicit policy row yet — the safe default.
pub fn get_share_policy(store: &Store, initiative: &str) -> Result<SharePolicy> {
    let mut p: BTreeMap<String, DataValue> = BTreeMap::new();
    p.insert("name".to_string(), DataValue::Str(initiative.into()));
    let script = r#"
        ?[share_policy] := *initiative{name, share_policy}, name = $name
    "#;
    let rows = store
        .db_ref()
        .run_script(script, p, ScriptMutability::Immutable)?;
    let policy_str = rows
        .rows
        .first()
        .and_then(|row| row.first())
        .and_then(|v| v.get_str())
        .unwrap_or("private");
    SharePolicy::from_str(policy_str)
}

#[cfg(test)]
mod tests {
    use super::{get_share_policy, get_visibility, set_share_policy, set_visibility};
    use crate::graph::{Layer, SharePolicy, Visibility};
    use crate::store::Store;
    use crate::{EpisodeKind, Significance, get_layer, set_layer, write_episode};

    /// Fresh node defaults to `Local`; `set_visibility` flips it and the
    /// flip survives a read.
    #[test]
    fn visibility_round_trip_default_local() {
        let store = Store::open_in_memory().expect("open");
        let id = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "n",
            "b",
        )
        .unwrap();

        assert_eq!(get_visibility(&store, &id).unwrap(), Visibility::Local);
        set_visibility(&store, &id, Visibility::Shared).unwrap();
        assert_eq!(get_visibility(&store, &id).unwrap(), Visibility::Shared);
    }

    /// `set_visibility` and `set_layer` are orthogonal in-place rewrites:
    /// changing one must preserve the other. Regression guard for the
    /// "omitted defaulted column resets to default" trap.
    #[test]
    fn set_visibility_and_set_layer_preserve_each_other() {
        let store = Store::open_in_memory().expect("open");
        let id = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "n",
            "b",
        )
        .unwrap();

        // Mark shared, then change the layer — visibility must survive.
        set_visibility(&store, &id, Visibility::Shared).unwrap();
        set_layer(&store, &id, Layer::Core).unwrap();
        assert_eq!(
            get_visibility(&store, &id).unwrap(),
            Visibility::Shared,
            "layer change must not reset visibility"
        );
        assert_eq!(get_layer(&store, &id).unwrap(), Layer::Core);

        // Flip visibility back — the layer must survive.
        set_visibility(&store, &id, Visibility::Local).unwrap();
        assert_eq!(
            get_layer(&store, &id).unwrap(),
            Layer::Core,
            "visibility change must not reset layer"
        );
        assert_eq!(get_visibility(&store, &id).unwrap(), Visibility::Local);
    }

    /// Unknown initiative defaults to `Private`; `set_share_policy`
    /// persists and `permits_share` reflects the policy.
    #[test]
    fn share_policy_round_trip_default_private() {
        let store = Store::open_in_memory().expect("open");

        assert_eq!(
            get_share_policy(&store, "fresh").unwrap(),
            SharePolicy::Private
        );

        set_share_policy(&store, "team-proj", SharePolicy::Team).unwrap();
        assert_eq!(
            get_share_policy(&store, "team-proj").unwrap(),
            SharePolicy::Team
        );

        assert!(SharePolicy::Team.permits_share());
        assert!(!SharePolicy::Private.permits_share());
        assert!(!SharePolicy::Ask.permits_share());
    }
}
