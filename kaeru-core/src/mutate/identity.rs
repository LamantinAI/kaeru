//! Preserve identity references across a content-preserving promotion. A changed
//! fact retains metadata/dependents, but cannot inherit evidence endorsing the old fact.

use std::collections::BTreeMap;

use cozo::{DataValue, JsonData, ScriptMutability};
use serde_json::Value;

use super::{attach_node_to_initiative_named, edge_version_seconds, initiatives_of_node};
use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

pub(crate) fn carry_identity(
    store: &Store,
    old: &NodeId,
    new: &NodeId,
    unchanged_content: bool,
) -> Result<()> {
    for initiative in initiatives_of_node(store, old)? {
        attach_node_to_initiative_named(store, new, &initiative)?;
    }
    let p = || {
        BTreeMap::from([
            ("old".into(), DataValue::Str(old.clone().into())),
            ("new".into(), DataValue::Str(new.clone().into())),
        ])
    };
    let properties = store.db_ref().run_script(
        "?[properties] := *node{id, properties @ 'NOW'}, id = $old",
        p(),
        ScriptMutability::Immutable,
    )?;
    if let Some(DataValue::Json(properties)) = properties.rows.first().and_then(|r| r.first()) {
        let mut value = properties.0.clone();
        if !unchanged_content && let Some(object) = value.as_object_mut() {
            object.remove("source");
            object.remove("current_decision");
        }
        let mut params = p();
        params.insert("properties".into(), DataValue::Json(JsonData(value)));
        store.db_ref().run_script(
            "?[id, validity, type, tier, name, body, tags, initiatives, properties, visibility, layer] := \
             *node{id, validity, type, tier, name, body, tags, initiatives, visibility, layer @ 'NOW'}, \
             id = $new, properties = $properties \
             :put node {id, validity => type, tier, name, body, tags, initiatives, properties, visibility, layer}",
            params, ScriptMutability::Mutable,
        )?;
    }

    let edges = store.db_ref().run_script(
        "?[src, dst, edge_type, weight, properties, dst_store] := \
         *edge{src, dst, edge_type, weight, properties, dst_store @ 'NOW'}, src = $old \
         ?[src, dst, edge_type, weight, properties, dst_store] := \
         *edge{src, dst, edge_type, weight, properties, dst_store @ 'NOW'}, dst = $old",
        p(),
        ScriptMutability::Immutable,
    )?;
    for row in edges.rows {
        let Some(src) = row[0].get_str() else {
            continue;
        };
        let Some(dst) = row[1].get_str() else {
            continue;
        };
        let Some(kind) = row[2].get_str() else {
            continue;
        };
        // Lineage itself intentionally refers to retired identities. Never turn
        // the just-written new→old marker into new→new self-supersession.
        if src == new || dst == new || matches!(kind, "supersedes" | "consolidated_to") {
            continue;
        }
        let new_src = if src == old { new.as_str() } else { src };
        let new_dst = if dst == old { new.as_str() } else { dst };
        let old_pk = format!("{src}|{dst}|{kind}");
        let new_pk = format!("{new_src}|{new_dst}|{kind}");
        let mut params = BTreeMap::from([
            ("src".into(), DataValue::Str(new_src.into())),
            ("dst".into(), DataValue::Str(new_dst.into())),
            ("kind".into(), row[2].clone()),
            ("weight".into(), row[3].clone()),
            ("properties".into(), row[4].clone()),
            ("dst_store".into(), row[5].clone()),
        ]);
        if unchanged_content
            || !matches!(kind, "verifies" | "falsifies" | "targets" | "contradicts")
        {
            let seconds = edge_version_seconds(store, &new_src.into(), &new_dst.into(), kind)?;
            store.db_ref().run_script(
                &format!(
                    "?[src, dst, edge_type, validity, weight, properties, dst_store] <- \
                 [[$src, $dst, $kind, [{seconds}.0, true], $weight, $properties, $dst_store]] \
                 :put edge {{src, dst, edge_type, validity => weight, properties, dst_store}}"
                ),
                params.clone(),
                ScriptMutability::Mutable,
            )?;
            params.insert("old_pk".into(), DataValue::Str(old_pk.clone().into()));
            params.insert("new_pk".into(), DataValue::Str(new_pk.into()));
            store.db_ref().run_script(
                "?[initiative, edge_pk] := *edge_initiative{initiative, edge_pk: previous}, \
                 previous = $old_pk, edge_pk = $new_pk :put edge_initiative {initiative, edge_pk}",
                params.clone(),
                ScriptMutability::Mutable,
            )?;
        }
        params.insert("src".into(), DataValue::Str(src.into()));
        params.insert("dst".into(), DataValue::Str(dst.into()));
        let seconds = edge_version_seconds(store, &src.into(), &dst.into(), kind)?;
        store.db_ref().run_script(
            &format!(
                "?[src, dst, edge_type, validity, weight, properties, dst_store] <- \
             [[$src, $dst, $kind, [{seconds}.0, false], $weight, $properties, $dst_store]] \
             :put edge {{src, dst, edge_type, validity => weight, properties, dst_store}}"
            ),
            params,
            ScriptMutability::Mutable,
        )?;
    }
    store.db_ref().run_script(
        "?[node_id, reason, pinned_at] := *session_pin{node_id: previous, reason, pinned_at}, \
         previous = $old, node_id = $new :put session_pin {node_id => reason, pinned_at}",
        p(),
        ScriptMutability::Mutable,
    )?;
    store.db_ref().run_script(
        "?[node_id] <- [[$old]] :rm session_pin {node_id}",
        p(),
        ScriptMutability::Mutable,
    )?;
    store.db_ref().run_script(
        "?[initiative, slot, node_id, set_at] := *slot_occupant{initiative, slot, node_id: previous, set_at}, \
         previous = $old, node_id = $new :put slot_occupant {initiative, slot => node_id, set_at}",
        p(), ScriptMutability::Mutable,
    )?;
    let summaries = store.db_ref().run_script(
        "?[chain_id,fingerprints,stale] := *api_trail_summary{chain_id,fingerprints,stale}, \
         *chain_member{chain_id,node_id},node_id=$old \
         ?[chain_id,fingerprints,stale] := *api_trail_summary{chain_id,fingerprints,stale}, \
         *chain_member{chain_id,node_id},node_id=$new",
        p(),
        ScriptMutability::Immutable,
    )?;
    for row in summaries.rows {
        let mut fingerprints = Value::from(row[1].clone());
        if let Some(map) = fingerprints.as_object_mut()
            && let Some(previous) = map.remove(old)
        {
            map.insert(new.clone(), previous);
        }
        let stale = !unchanged_content || row[2].get_bool().unwrap_or(true);
        store.db_ref().run_script(
            "?[chain_id,fingerprints,stale] <- [[$id,$fingerprints,$stale]] :put api_trail_summary {chain_id=>fingerprints,stale}",
            BTreeMap::from([("id".into(),row[0].clone()),("fingerprints".into(),DataValue::Json(JsonData(fingerprints))),("stale".into(),DataValue::Bool(stale))]),ScriptMutability::Mutable,
        )?;
    }
    Ok(())
}
