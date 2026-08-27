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
mod cloud_permission_tests {
    use super::{initiative_clouds, permits_cloud, set_initiative_clouds};
    use crate::store::Store;

    /// An initiative that never asked for a restriction keeps behaving exactly
    /// as it did — the empty set means "no restriction" rather than "nothing
    /// permitted", which is what makes this additive.
    #[test]
    fn an_unrestricted_initiative_permits_every_cloud() {
        let store = Store::open_in_memory().expect("open");
        assert!(initiative_clouds(&store, "t").expect("read").is_empty());
        assert!(permits_cloud(&store, "t", "anything").expect("check"));
    }

    /// The gap #65 could not close: with several clouds configured a caller
    /// must name one, but any valid name is accepted whatever the initiative.
    /// This is where an initiative says which names are valid *for it*.
    #[test]
    fn a_restricted_initiative_permits_only_its_own_clouds() {
        let store = Store::open_in_memory().expect("open");
        set_initiative_clouds(&store, "t", &["work".into(), "archive".into()]).expect("set");

        assert!(permits_cloud(&store, "t", "work").expect("check"));
        assert!(permits_cloud(&store, "t", "archive").expect("check"));
        assert!(!permits_cloud(&store, "t", "personal").expect("check"));
        assert_eq!(
            initiative_clouds(&store, "t").expect("read"),
            vec!["archive".to_string(), "work".to_string()],
            "sorted, so the list reads the same every time"
        );
    }

    /// Setting replaces rather than accumulates, and an empty list clears —
    /// otherwise a restriction could only ever be widened.
    #[test]
    fn setting_replaces_and_empty_clears() {
        let store = Store::open_in_memory().expect("open");
        set_initiative_clouds(&store, "t", &["work".into()]).expect("set");
        set_initiative_clouds(&store, "t", &["archive".into()]).expect("replace");
        assert_eq!(
            initiative_clouds(&store, "t").expect("read"),
            vec!["archive"]
        );

        set_initiative_clouds(&store, "t", &[]).expect("clear");
        assert!(initiative_clouds(&store, "t").expect("read").is_empty());
        assert!(permits_cloud(&store, "t", "anywhere").expect("check"));
    }

    /// One initiative's restriction is not another's.
    #[test]
    fn restrictions_are_per_initiative() {
        let store = Store::open_in_memory().expect("open");
        set_initiative_clouds(&store, "locked", &["work".into()]).expect("set");
        assert!(!permits_cloud(&store, "locked", "personal").expect("check"));
        assert!(permits_cloud(&store, "open", "personal").expect("check"));
    }
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

/// Restricts an initiative to a named set of clouds, replacing whatever it had.
/// An **empty** list clears the restriction — the initiative may then go to any
/// configured cloud, which is how every initiative behaves before this is ever
/// called.
///
/// This is the second half of the first share gate. `share_policy` says
/// *whether* an initiative may leave; this says *where to*. Until it existed,
/// `team` opened an initiative to every cloud the daemon could reach at once,
/// and the choice of destination lived entirely in the caller's argument —
/// fine with one cloud, and not a permission model with several of differing
/// trust.
pub fn set_initiative_clouds(store: &Store, initiative: &str, clouds: &[String]) -> Result<()> {
    let mut clear: BTreeMap<String, DataValue> = BTreeMap::new();
    clear.insert("init".to_string(), DataValue::Str(initiative.into()));
    store.db_ref().run_script(
        r#"
        ?[initiative, cloud] := *initiative_cloud{initiative, cloud}, initiative = $init
        :rm initiative_cloud {initiative, cloud}
        "#,
        clear,
        ScriptMutability::Mutable,
    )?;

    for cloud in clouds {
        let name = cloud.trim();
        if name.is_empty() {
            continue;
        }
        let mut p: BTreeMap<String, DataValue> = BTreeMap::new();
        p.insert("init".to_string(), DataValue::Str(initiative.into()));
        p.insert("cloud".to_string(), DataValue::Str(name.into()));
        store.db_ref().run_script(
            r#"
            ?[initiative, cloud] <- [[$init, $cloud]]
            :put initiative_cloud {initiative, cloud}
            "#,
            p,
            ScriptMutability::Mutable,
        )?;
    }

    write_audit(
        store.db_ref(),
        "set_initiative_clouds",
        "system",
        &[initiative.to_string()],
    )?;
    Ok(())
}

/// The clouds an initiative is restricted to, sorted. Empty means unrestricted.
pub fn initiative_clouds(store: &Store, initiative: &str) -> Result<Vec<String>> {
    let mut p: BTreeMap<String, DataValue> = BTreeMap::new();
    p.insert("init".to_string(), DataValue::Str(initiative.into()));
    let rows = store.db_ref().run_script(
        r#"
        ?[cloud] := *initiative_cloud{initiative, cloud}, initiative = $init
        :order cloud
        "#,
        p,
        ScriptMutability::Immutable,
    )?;
    Ok(rows
        .rows
        .iter()
        .filter_map(|r| r.first().and_then(|v| v.get_str()).map(String::from))
        .collect())
}

/// Whether `initiative` may be shared into `cloud`.
///
/// Unrestricted initiatives permit everything, so this is `true` for every
/// initiative that has not opted into a list — the behaviour before the
/// relation existed, preserved by construction rather than by a default value.
pub fn permits_cloud(store: &Store, initiative: &str, cloud: &str) -> Result<bool> {
    let allowed = initiative_clouds(store, initiative)?;
    Ok(allowed.is_empty() || allowed.iter().any(|c| c == cloud))
}
