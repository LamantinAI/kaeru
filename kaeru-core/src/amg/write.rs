//! Dedicated administrative assertions preserve complete fields and local history.

use std::collections::{BTreeMap, BTreeSet};

use cozo::{DataValue, JsonData};
use serde_json::{Value, json};

use super::{plan::Plan, storage as db};
use crate::graph::audit::write_audit_details;
use crate::models::amg::{AmgDocument, AmgEdge, AmgNode, AmgOrigin};
use crate::mutate::{attach_node_to_initiative_named, edge_version_seconds, node_version_seconds};
use crate::{Result, Store, new_node_id};

fn s(value: &str) -> DataValue {
    DataValue::Str(value.into())
}
fn origin(
    document: &AmgDocument,
    node: Option<&AmgNode>,
    edge: Option<&AmgEdge>,
    digest: &str,
) -> AmgOrigin {
    AmgOrigin {
        document_sha256: digest.into(),
        exported_at: document.metadata.exported_at.clone(),
        software: document.metadata.software.clone(),
        recorded_at: node
            .and_then(|n| n.recorded_at.clone())
            .or_else(|| edge.and_then(|e| e.recorded_at.clone())),
        created_at: node
            .and_then(|n| n.created_at.clone())
            .or_else(|| edge.and_then(|e| e.created_at.clone())),
        source_visibility: node.map(|n| n.visibility.clone()),
        source_origin: node.and_then(|n| {
            n.properties
                .pointer("/source/origin")
                .and_then(Value::as_str)
                .map(str::to_string)
        }),
    }
}
fn provenance(
    store: &Store,
    kind: &str,
    key: &str,
    version: u64,
    mut origins: Vec<AmgOrigin>,
    source: AmgOrigin,
) -> Result<()> {
    if !origins.contains(&source) {
        origins.push(source);
    }
    db::write(
        store,
        "?[kind,key,version,origins] <- [[$kind,$key,$version,$origins]] :put amg_origin {kind,key,version=>origins}",
        BTreeMap::from([
            ("kind".into(), s(kind)),
            ("key".into(), s(key)),
            ("version".into(), DataValue::from(version as i64)),
            (
                "origins".into(),
                DataValue::Json(JsonData(
                    serde_json::to_value(origins).expect("origins serialize"),
                )),
            ),
        ]),
    )
}

pub(super) fn apply(store: &Store, document: &AmgDocument, plan: &Plan) -> Result<()> {
    let source_nodes = document
        .data
        .nodes
        .iter()
        .map(|n| (n.id.as_str(), n))
        .collect::<BTreeMap<_, _>>();
    let source_edges = document
        .data
        .edges
        .iter()
        .map(|e| (db::edge_key(&e.src, &e.dst, &e.edge_type), e))
        .collect::<BTreeMap<_, _>>();
    let mut changes = vec![];
    let mut versions = BTreeMap::new();
    for name in &document.metadata.initiatives {
        // Never copy source sharing/cloud policy or replace a destination policy.
        let params = BTreeMap::from([("name".into(), s(name))]);
        if db::query(
            store,
            "?[name] := *initiative{name},name=$name",
            params.clone(),
        )?
        .rows
        .is_empty()
        {
            db::write(store, "?[name] <- [[$name]] :put initiative {name}", params)?;
        }
    }
    for change in &plan.nodes {
        let node = &change.value;
        if change.write {
            let seconds = node_version_seconds(store, &node.id)?;
            versions.insert(node.id.clone(), seconds);
            let tags = DataValue::List(node.tags.iter().map(|tag| s(tag)).collect());
            let params = BTreeMap::from([
                ("id".into(), s(&node.id)),
                ("kind".into(), s(&node.node_type)),
                ("tier".into(), s(&node.tier)),
                ("name".into(), s(&node.name)),
                (
                    "body".into(),
                    node.body.as_deref().map_or(DataValue::Null, s),
                ),
                ("tags".into(), tags),
                (
                    "properties".into(),
                    DataValue::Json(JsonData(node.properties.clone())),
                ),
                ("visibility".into(), s(&node.visibility)),
                ("layer".into(), s(&node.layer)),
            ]);
            db::write(
                store,
                &format!(
                    "?[id,validity,type,tier,name,body,tags,initiatives,properties,visibility,layer] <- [[$id,[{seconds}.0,true],$kind,$tier,$name,$body,$tags,null,$properties,$visibility,$layer]] :put node {{id,validity=>type,tier,name,body,tags,initiatives,properties,visibility,layer}}"
                ),
                params,
            )?;
            let source = source_nodes[node.id.as_str()];
            provenance(
                store,
                "node",
                &node.id,
                seconds,
                source.origins.clone(),
                origin(document, Some(source), None, &plan.report.document_sha256),
            )?;
        }
        for name in &change.add {
            attach_node_to_initiative_named(store, &node.id, name)?;
        }
        if !change.add.is_empty() {
            changes.push(json!({"kind":"node_memberships","node":node.id,"before":change.before_memberships,"added":change.add}));
        }
    }
    for change in &plan.edges {
        let edge = &change.value;
        if change.write {
            let seconds = edge_version_seconds(store, &edge.src, &edge.dst, &edge.edge_type)?;
            db::write(
                store,
                &format!(
                    "?[src,dst,edge_type,validity,weight,properties,dst_store] <- [[$src,$dst,$kind,[{seconds}.0,true],$weight,$properties,'local']] :put edge {{src,dst,edge_type,validity=>weight,properties,dst_store}}"
                ),
                BTreeMap::from([
                    ("src".into(), s(&edge.src)),
                    ("dst".into(), s(&edge.dst)),
                    ("kind".into(), s(&edge.edge_type)),
                    ("weight".into(), DataValue::from(edge.weight)),
                    (
                        "properties".into(),
                        DataValue::Json(JsonData(edge.properties.clone())),
                    ),
                ]),
            )?;
            let key = db::edge_key(&edge.src, &edge.dst, &edge.edge_type);
            let source = source_edges[&key];
            provenance(
                store,
                "edge",
                &key,
                seconds,
                source.origins.clone(),
                origin(document, None, Some(source), &plan.report.document_sha256),
            )?;
        }
        for name in &change.add {
            db::write(
                store,
                "?[initiative,edge_pk] <- [[$initiative,$key]] :put edge_initiative {initiative,edge_pk}",
                BTreeMap::from([
                    ("initiative".into(), s(name)),
                    (
                        "key".into(),
                        s(&db::legacy_edge_key(&edge.src, &edge.dst, &edge.edge_type)),
                    ),
                ]),
            )?;
        }
    }
    for change in &plan.edges {
        if !change.add.is_empty() {
            changes.push(json!({"kind":"edge_memberships","edge":[change.value.src,change.value.dst,change.value.edge_type],"before":change.before_memberships,"added":change.add}));
        }
    }
    for (trail, before) in &plan.trails {
        let params = BTreeMap::from([("id".into(), s(&trail.id))]);
        db::write(
            store,
            "?[chain_id,position] := *chain_member{chain_id,position},chain_id=$id :rm chain_member {chain_id,position}",
            params.clone(),
        )?;
        db::write(
            store,
            "?[chain_id] := *api_trail_summary{chain_id},chain_id=$id :rm api_trail_summary {chain_id}",
            params,
        )?;
        for (index, id) in trail.members.iter().enumerate() {
            db::write(
                store,
                "?[chain_id,position,node_id] <- [[$id,$position,$node]] :put chain_member {chain_id,position=>node_id}",
                BTreeMap::from([
                    ("id".into(), s(&trail.id)),
                    ("position".into(), DataValue::from(index as i64)),
                    ("node".into(), s(id)),
                ]),
            )?;
        }
        if let (Some(fp), Some(stale)) = (&trail.fingerprints, trail.stale) {
            db::write(
                store,
                "?[chain_id,fingerprints,stale] <- [[$id,$fp,$stale]] :put api_trail_summary {chain_id=>fingerprints,stale}",
                BTreeMap::from([
                    ("id".into(), s(&trail.id)),
                    (
                        "fp".into(),
                        DataValue::Json(JsonData(
                            serde_json::to_value(fp).expect("fingerprints serialize"),
                        )),
                    ),
                    ("stale".into(), DataValue::from(stale)),
                ]),
            )?;
        }
        changes.push(json!({"kind":"trail","id":trail.id,"before":before,"after":trail}));
    }
    for (slot, before) in &plan.slots {
        db::write(
            store,
            "?[initiative,slot,node_id] <- [[$initiative,$slot,$node]] :put slot_occupant {initiative,slot=>node_id}",
            BTreeMap::from([
                ("initiative".into(), s(&slot.initiative)),
                ("slot".into(), s(&slot.slot)),
                ("node".into(), s(&slot.node)),
            ]),
        )?;
        db::write(
            store,
            "?[initiative,slot,revision] <- [[$initiative,$slot,$revision]] :put api_slot_revision {initiative,slot=>revision}",
            BTreeMap::from([
                ("initiative".into(), s(&slot.initiative)),
                ("slot".into(), s(&slot.slot)),
                ("revision".into(), s(&new_node_id())),
            ]),
        )?;
        changes.push(json!({"kind":"slot","before":before,"after":slot}));
    }
    for (decision, before) in &plan.decisions {
        let version = versions
            .get(&decision.node)
            .copied()
            .expect("decision requires a new assertion");
        db::write(
            store,
            "?[hypothesis,version,status,evidence] <- [[$node,$version,$status,$evidence]] :put api_hypothesis_decision {hypothesis,version=>status,evidence}",
            BTreeMap::from([
                ("node".into(), s(&decision.node)),
                ("version".into(), DataValue::from(version as i64)),
                ("status".into(), s(&decision.status)),
                (
                    "evidence".into(),
                    DataValue::List(decision.evidence.iter().map(|id| s(id)).collect()),
                ),
            ]),
        )?;
        changes.push(
            json!({"kind":"decision","before":before,"after":decision,"local_version":version}),
        );
    }
    // Imported summaries retain their supplied stale/unknown state. Other local
    // summaries cannot stay fresh after a referenced record's semantics change.
    for (before, after) in &plan.invalidated_trails {
        db::write(
            store,
            "?[chain_id,fingerprints,stale] := *api_trail_summary{chain_id,fingerprints},chain_id=$id,stale=true :put api_trail_summary {chain_id=>fingerprints,stale}",
            BTreeMap::from([("id".into(), s(&after.id))]),
        )?;
        changes
            .push(json!({"kind":"trail_invalidation","id":after.id,"before":before,"after":after}));
    }
    let id = new_node_id();
    let affected = plan
        .nodes
        .iter()
        .filter(|n| n.write || !n.add.is_empty())
        .map(|n| n.value.id.clone())
        .chain(
            plan.edges
                .iter()
                .filter(|e| e.write || !e.add.is_empty())
                .flat_map(|e| [e.value.src.clone(), e.value.dst.clone()]),
        )
        .chain(plan.slots.iter().map(|(slot, _)| slot.node.clone()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let details = json!({"document_sha256":plan.report.document_sha256,"metadata":document.metadata,"report":plan.report,"metadata_changes":changes});
    db::write(
        store,
        "?[id,digest,metadata,changes] <- [[$id,$digest,$meta,$changes]] :put amg_import {id=>digest,metadata,changes}",
        BTreeMap::from([
            ("id".into(), s(&id)),
            ("digest".into(), s(&plan.report.document_sha256)),
            (
                "meta".into(),
                DataValue::Json(JsonData(
                    serde_json::to_value(&document.metadata).expect("metadata serializes"),
                )),
            ),
            ("changes".into(), DataValue::Json(JsonData(details.clone()))),
        ]),
    )?;
    write_audit_details(store.db_ref(), "amg_import", "operator", &affected, details)?;
    Ok(())
}
