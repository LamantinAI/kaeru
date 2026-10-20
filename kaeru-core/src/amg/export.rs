//! One coherent current-state snapshot; never expand selected scopes by traversal.

use std::collections::{BTreeMap, BTreeSet};

use chrono::{SecondsFormat, Utc};

use super::{
    codec::json_digest,
    storage as db,
    validate::{invalid, validate},
};
use crate::config::amg::{AmgLimits, PROFILE, SCHEMA_VERSION};
use crate::models::amg::{
    AmgDecision, AmgDocument, AmgGraph, AmgMetadata, AmgOmission, AmgScope, AmgSelection, AmgSlot,
    AmgSoftware, AmgTrail,
};
use crate::{Result, Store};

pub fn snapshot(
    store: &Store,
    selection: &AmgSelection,
    limits: &AmgLimits,
) -> Result<AmgDocument> {
    store.atomic(|store| capture(store, selection, limits))
}

fn capture(store: &Store, selection: &AmgSelection, limits: &AmgLimits) -> Result<AmgDocument> {
    let exported_at = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    let memberships = db::memberships(store, false)?;
    let mut known = memberships
        .values()
        .flatten()
        .cloned()
        .collect::<BTreeSet<_>>();
    for row in db::query(store, "?[name] := *initiative{name}", BTreeMap::new())?.rows {
        known.insert(db::string(&row[0])?);
    }
    for names in db::memberships(store, true)?.into_values() {
        known.extend(names);
    }
    let (scope, names) = match selection {
        AmgSelection::All => (AmgScope::All, known),
        AmgSelection::Initiatives(names) => {
            let selected = names.iter().cloned().collect::<BTreeSet<_>>();
            if selected.is_empty() || selected.len() != names.len() || !selected.is_subset(&known) {
                return Err(invalid(
                    "initiative selection is empty, duplicated or unknown",
                ));
            }
            (AmgScope::Initiatives, selected)
        }
    };
    let mut ids = BTreeSet::new();
    for row in db::query(
        store,
        "?[id,type] := *node{id,type @ 'NOW'},type!='audit_event',type!='placeholder'",
        BTreeMap::new(),
    )?
    .rows
    {
        let id = db::string(&row[0])?;
        if scope == AmgScope::All
            || memberships
                .get(&id)
                .is_some_and(|m| m.iter().any(|n| names.contains(n)))
        {
            ids.insert(id);
        }
    }
    if ids.len() > limits.nodes {
        return Err(invalid("snapshot node count exceeds limit"));
    }
    let mut nodes = db::nodes(store, &ids)?;
    let mut omitted = BTreeMap::<String, usize>::new();
    let mut trail_members = BTreeMap::<String, Vec<(i64, String)>>::new();
    for row in db::query(
        store,
        "?[chain_id,position,node_id] := *chain_member{chain_id,position,node_id}",
        BTreeMap::new(),
    )?
    .rows
    {
        let id = db::string(&row[0])?;
        if ids.contains(&id) {
            trail_members.entry(id).or_default().push((
                row[1]
                    .get_int()
                    .ok_or_else(|| invalid("invalid trail position"))?,
                db::string(&row[2])?,
            ));
        }
    }
    for members in trail_members.values_mut() {
        members.sort();
    }
    loop {
        let remove = nodes
            .values()
            .filter(|n| {
                n.node_type == "chain"
                    && trail_members
                        .get(&n.id)
                        .is_some_and(|members| members.iter().any(|(_, id)| !ids.contains(id)))
            })
            .map(|n| n.id.clone())
            .collect::<Vec<_>>();
        if remove.is_empty() {
            break;
        }
        for id in remove {
            nodes.remove(&id);
            ids.remove(&id);
            *omitted.entry("trail_outside_scope".into()).or_default() += 1;
        }
    }
    for node in nodes.values_mut() {
        node.initiatives.retain(|name| names.contains(name));
        node.tags.retain(|tag| !tag.starts_with("seen:"));
    }
    let mut trails = BTreeMap::new();
    for node in nodes.values().filter(|node| node.node_type == "chain") {
        let members = trail_members
            .get(&node.id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(|(_, id)| id)
            .collect();
        trails.insert(
            node.id.clone(),
            AmgTrail {
                id: node.id.clone(),
                members,
                fingerprints: None,
                stale: None,
            },
        );
    }
    for row in db::query(
        store,
        "?[chain_id,fingerprints,stale] := *api_trail_summary{chain_id,fingerprints,stale}",
        BTreeMap::new(),
    )?
    .rows
    {
        if let Some(trail) = trails.get_mut(&db::string(&row[0])?) {
            let mut fingerprints: BTreeMap<String, String> =
                serde_json::from_value(db::json(&row[1]))
                    .map_err(|_| invalid("invalid trail fingerprints"))?;
            let lost = fingerprints.keys().any(|id| !ids.contains(id));
            fingerprints.retain(|id, _| ids.contains(id));
            let mismatched = trail.members.iter().any(|id| {
                let n = &nodes[id];
                let hash = json_digest(&serde_json::json!([n.name, n.body]));
                fingerprints.get(id) != Some(&hash)
            });
            trail.stale = Some(
                lost || mismatched
                    || fingerprints.len() != trail.members.len()
                    || row[2].get_bool().unwrap_or(true),
            );
            trail.fingerprints = Some(fingerprints);
        }
    }
    let mut edges = vec![];
    for mut edge in db::edges(store, &ids)?.into_values() {
        if scope != AmgScope::All
            && !edge.initiatives.is_empty()
            && !edge.initiatives.iter().any(|n| names.contains(n))
        {
            continue;
        }
        edge.initiatives.retain(|name| names.contains(name));
        edges.push(edge);
    }
    db::check_legacy_keys(store, &edges)?;
    let mut slots = vec![];
    for row in db::query(
        store,
        "?[initiative,slot,node_id] := *slot_occupant{initiative,slot,node_id}",
        BTreeMap::new(),
    )?
    .rows
    {
        let initiative = db::string(&row[0])?;
        let node = db::string(&row[2])?;
        if names.contains(&initiative) && ids.contains(&node) {
            slots.push(AmgSlot {
                initiative,
                slot: db::string(&row[1])?,
                node,
            });
        }
    }
    slots.sort_by(|a, b| (&a.initiative, &a.slot).cmp(&(&b.initiative, &b.slot)));
    let mut decisions = vec![];
    for row in db::query(store,"?[hypothesis,version,status,evidence] := *api_hypothesis_decision{hypothesis,version,status,evidence}",BTreeMap::new())?.rows {
        let id=db::string(&row[0])?;
        if let Some(node)=nodes.get(&id) {
            if row[1].get_int()==Some(db::version(node.recorded_at.as_ref())?) {
                decisions.push(AmgDecision{node:id,status:db::string(&row[2])?,evidence:db::strings(&row[3])?});
            }
        }
    }
    decisions.sort_by(|a, b| a.node.cmp(&b.node));
    // External cloud locations are runtime configuration, not portable knowledge.
    for row in db::query(
        store,
        "?[src,dst_store] := *edge{src,dst_store @ 'NOW'},dst_store!='local'",
        BTreeMap::new(),
    )?
    .rows
    {
        if ids.contains(&db::string(&row[0])?) {
            *omitted.entry("external_store_edge".into()).or_default() += 1;
        }
    }
    let document = AmgDocument {
        metadata: AmgMetadata {
            schema_version: SCHEMA_VERSION,
            exported_at,
            software: AmgSoftware {
                name: "kaeru".into(),
                version: env!("CARGO_PKG_VERSION").into(),
            },
            profile: PROFILE.into(),
            scope,
            initiatives: names.into_iter().collect(),
            omissions: omitted
                .into_iter()
                .map(|(reason, count)| AmgOmission { reason, count })
                .collect(),
        },
        data: AmgGraph {
            nodes: nodes.into_values().collect(),
            edges,
            trails: trails.into_values().collect(),
            slots,
            decisions,
        },
    };
    validate(&document, limits)?;
    Ok(document)
}
