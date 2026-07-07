//! Tier-promotion / demotion: `consolidate_out` (operational → archival)
//! and `consolidate_in` (archival → operational). Provenance via
//! `derived_from` is replicated onto the new node so it survives the
//! tier boundary.

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};

use super::{
    attach_edge_to_initiative, attach_node_to_initiative, attach_node_to_initiative_named,
    build_body_tags, initiatives_of_node, now_validity_seconds, read_derived_from_targets,
    tags_literal,
};
use crate::errors::Result;
use crate::graph::audit::write_audit;
use crate::graph::{NodeId, NodeType, Tier, new_node_id};
use crate::store::Store;

/// Tier-promotion mutation: turns an operational node into an archival
/// counterpart, preserving provenance.
///
/// Behaviour:
///  1. Read the operational node's outgoing `derived_from` edges at NOW.
///  2. Retract the operational node.
///  3. Assert a fresh archival node (new id, `new_type`, `Tier::Archival`,
///     `new_name`, `new_body`).
///  4. Replicate every `derived_from` edge onto the new node so
///     `recollect_provenance` returns the same ancestor set when called
///     on either side of the tier boundary.
///  5. Create a `consolidated_to` edge from the old (operational) id to
///     the new (archival) id — a query handle for "what did this draft
///     turn into?".
///  6. Single audit event covering the consolidation as a whole.
///
/// Like `supersedes`, the substrate-level writes are not atomic; a failure
/// between steps leaves the graph in an intermediate state recoverable via
/// `lint`.
pub fn consolidate_out(
    store: &Store,
    operational_id: &NodeId,
    new_type: NodeType,
    new_name: &str,
    new_body: &str,
) -> Result<NodeId> {
    consolidate(
        store,
        operational_id,
        new_type,
        Tier::Archival,
        new_name,
        new_body,
        "consolidate_out",
    )
}

/// Tier-demotion mutation: brings an archival node back into the
/// operational tier (typically because it needs revision while the agent
/// is actively working on it). Mirror of [`consolidate_out`] — same
/// shape, opposite tier transition. The `consolidated_to` edge still goes
/// from the old (archival) id to the new (operational) id, recording the
/// direction of the consolidation event itself.
pub fn consolidate_in(
    store: &Store,
    archival_id: &NodeId,
    new_type: NodeType,
    new_name: &str,
    new_body: &str,
) -> Result<NodeId> {
    consolidate(
        store,
        archival_id,
        new_type,
        Tier::Operational,
        new_name,
        new_body,
        "consolidate_in",
    )
}

fn consolidate(
    store: &Store,
    old_id: &NodeId,
    new_type: NodeType,
    new_tier: Tier,
    new_name: &str,
    new_body: &str,
    audit_op: &str,
) -> Result<NodeId> {
    // Step 0 — collect the old node's outgoing `derived_from` targets so
    // we can replicate them on the new node. Read first; the retraction
    // below doesn't drop the edge rows, but reading before mutating keeps
    // the data flow easy to follow.
    let provenance_targets = read_derived_from_targets(store, old_id)?;

    let new_id = new_node_id();
    let new_type_str = new_type.as_str();
    let new_tier_str = new_tier.as_str();

    // Step 1 — retract old.
    let retract_secs = now_validity_seconds();
    let mut p1: BTreeMap<String, DataValue> = BTreeMap::new();
    p1.insert("old_id".to_string(), DataValue::Str(old_id.clone().into()));
    let s1 = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$old_id, [{retract_secs}.0, false], 'placeholder', 'operational', '', null, null, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store
        .db_ref()
        .run_script(&s1, p1, ScriptMutability::Mutable)?;

    // Step 2 — assert new node at the target tier.
    let assert_secs = now_validity_seconds();
    let mut p2: BTreeMap<String, DataValue> = BTreeMap::new();
    p2.insert("id".to_string(), DataValue::Str(new_id.clone().into()));
    p2.insert("name".to_string(), DataValue::Str(new_name.into()));
    p2.insert("body".to_string(), DataValue::Str(new_body.into()));
    let kind_tag = format!("kind:{}", new_type_str);
    let all_tags = build_body_tags(&[kind_tag.as_str()], new_body);
    let tags = tags_literal(&all_tags);
    let s2 = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{assert_secs}.0, true], '{new_type_str}', '{new_tier_str}', $name, $body, {tags}, null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store
        .db_ref()
        .run_script(&s2, p2, ScriptMutability::Mutable)?;
    attach_node_to_initiative(store, &new_id)?;
    // With no active scope the junction write above was a no-op — but the
    // old node belonged somewhere, and it was just retracted. Losing its
    // replacement from those initiatives is never intended (the node
    // becomes invisible to every scoped read), so inherit the source's
    // memberships instead.
    if store.current_initiative().is_none() {
        for init in initiatives_of_node(store, old_id)? {
            attach_node_to_initiative_named(store, &new_id, &init)?;
        }
    }

    // Step 3 — replicate derived_from edges so provenance survives the
    // tier boundary.
    for target in &provenance_targets {
        let edge_secs = now_validity_seconds();
        let mut p_edge: BTreeMap<String, DataValue> = BTreeMap::new();
        p_edge.insert("src".to_string(), DataValue::Str(new_id.clone().into()));
        p_edge.insert("dst".to_string(), DataValue::Str(target.clone().into()));
        let s_edge = format!(
            r#"
            ?[src, dst, edge_type, validity, weight, properties] <-
                [[$src, $dst, 'derived_from', [{edge_secs}.0, true], 1.0, null]]
            :put edge {{src, dst, edge_type, validity => weight, properties}}
            "#
        );
        store
            .db_ref()
            .run_script(&s_edge, p_edge, ScriptMutability::Mutable)?;
        attach_edge_to_initiative(store, &new_id, target, "derived_from")?;
    }

    // Step 4 — consolidated_to edge: old → new.
    let edge_secs = now_validity_seconds();
    let mut p_link: BTreeMap<String, DataValue> = BTreeMap::new();
    p_link.insert("src".to_string(), DataValue::Str(old_id.clone().into()));
    p_link.insert("dst".to_string(), DataValue::Str(new_id.clone().into()));
    let s_link = format!(
        r#"
        ?[src, dst, edge_type, validity, weight, properties] <-
            [[$src, $dst, 'consolidated_to', [{edge_secs}.0, true], 1.0, null]]
        :put edge {{src, dst, edge_type, validity => weight, properties}}
        "#
    );
    store
        .db_ref()
        .run_script(&s_link, p_link, ScriptMutability::Mutable)?;
    attach_edge_to_initiative(store, old_id, &new_id, "consolidated_to")?;

    write_audit(
        store.db_ref(),
        audit_op,
        "system",
        &[old_id.clone(), new_id.clone()],
    )?;
    Ok(new_id)
}

#[cfg(test)]
mod tests {
    use crate::graph::{NodeType, Tier};
    use crate::store::Store;

    /// A consolidation performed with no active initiative scope used to
    /// leave the replacement node without any membership — invisible to
    /// every scoped read — while the source (which had memberships) got
    /// retracted. The replacement must inherit the source's initiatives.
    #[test]
    fn consolidate_without_scope_inherits_source_initiatives() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("demo");
        let draft = crate::jot(&store, "scoped draft").expect("jot");

        store.clear_initiative();
        let settled =
            crate::consolidate_out(&store, &draft, NodeType::Outcome, "settled-x", "body")
                .expect("consolidate");

        let inits = super::super::initiatives_of_node(&store, &settled).expect("junction read");
        assert_eq!(inits, vec!["demo".to_string()]);
    }

    /// Same guarantee for `synthesise`: with no scope active the new node
    /// inherits the union of the seeds' initiatives.
    #[test]
    fn synthesise_without_scope_inherits_union_of_seed_initiatives() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("alpha");
        let a = crate::jot(&store, "seed a").expect("jot a");
        store.use_initiative("beta");
        let b = crate::jot(&store, "seed b").expect("jot b");

        store.clear_initiative();
        let s = crate::synthesise(
            &store,
            &[a, b],
            NodeType::Summary,
            Tier::Archival,
            "union-synth",
            "body",
        )
        .expect("synthesise");

        let inits = super::super::initiatives_of_node(&store, &s).expect("junction read");
        assert_eq!(inits, vec!["alpha".to_string(), "beta".to_string()]);
    }

    /// Explicit scope still wins — consolidation under an active initiative
    /// attaches there and only there (existing behaviour, pinned by a test).
    #[test]
    fn consolidate_with_scope_attaches_to_that_scope() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("demo");
        let draft = crate::jot(&store, "scoped draft").expect("jot");
        let settled =
            crate::consolidate_out(&store, &draft, NodeType::Outcome, "settled-y", "body")
                .expect("consolidate");
        let inits = super::super::initiatives_of_node(&store, &settled).expect("junction read");
        assert_eq!(inits, vec!["demo".to_string()]);
    }
}
