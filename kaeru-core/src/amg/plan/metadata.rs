//! Ordered trails, slots and version-bound verdicts.

use std::collections::{BTreeMap, BTreeSet};

use cozo::DataValue;

use super::super::{storage as db, validate::invalid};
use super::{Plan, records::conflict};
use crate::models::amg::{AmgDecision, AmgDocument, AmgNode, AmgSlot, AmgTrail};
use crate::{Result, Store};

fn trail(store: &Store, id: &str) -> Result<Option<AmgTrail>> {
    let mut members = vec![];
    for row in db::query(store,"?[position,node_id] := *chain_member{chain_id,position,node_id},chain_id=$id :order position",BTreeMap::from([("id".into(),DataValue::Str(id.into()))]))?.rows { members.push(db::string(&row[1])?); }
    let rows = db::query(
        store,
        "?[fingerprints,stale] := *api_trail_summary{chain_id,fingerprints,stale},chain_id=$id",
        BTreeMap::from([("id".into(), DataValue::Str(id.into()))]),
    )?;
    let (fingerprints, stale) = if let Some(row) = rows.rows.first() {
        (
            Some(
                serde_json::from_value(db::json(&row[0]))
                    .map_err(|_| invalid("invalid local trail summary"))?,
            ),
            Some(row[1].get_bool().unwrap_or(true)),
        )
    } else {
        (None, None)
    };
    if members.is_empty() && fingerprints.is_none() {
        return Ok(None);
    }
    Ok(Some(AmgTrail {
        id: id.into(),
        members,
        fingerprints,
        stale,
    }))
}
fn decision(store: &Store, node: &AmgNode) -> Result<Option<AmgDecision>> {
    let version = db::version(node.recorded_at.as_ref())?;
    let rows = db::query(
        store,
        "?[status,evidence] := *api_hypothesis_decision{hypothesis,version,status,evidence},hypothesis=$id,version=$version",
        BTreeMap::from([
            ("id".into(), DataValue::Str(node.id.clone().into())),
            ("version".into(), DataValue::from(version)),
        ]),
    )?;
    rows.rows
        .first()
        .map(|row| {
            Ok(AmgDecision {
                node: node.id.clone(),
                status: db::string(&row[0])?,
                evidence: db::strings(&row[1])?,
            })
        })
        .transpose()
}

pub(super) fn merge_metadata(
    store: &Store,
    document: &AmgDocument,
    current: &BTreeMap<String, AmgNode>,
    force: bool,
    plan: &mut Plan,
) -> Result<()> {
    let indexes = plan
        .nodes
        .iter()
        .enumerate()
        .map(|(index, n)| (n.value.id.clone(), index))
        .collect::<BTreeMap<_, _>>();
    for incoming in &document.data.trails {
        let before = trail(store, &incoming.id)?.or_else(|| {
            current
                .get(&incoming.id)
                .filter(|n| n.node_type == "chain")
                .map(|_| AmgTrail {
                    id: incoming.id.clone(),
                    members: vec![],
                    fingerprints: None,
                    stale: None,
                })
        });
        if before.as_ref() == Some(incoming) {
            continue;
        }
        if before.is_some() && !force {
            return Err(conflict("trail", &incoming.id));
        }
        plan.nodes[indexes[&incoming.id]].write = true;
        plan.trails.push((incoming.clone(), before));
    }
    let incoming_decisions = document
        .data
        .decisions
        .iter()
        .map(|d| d.node.as_str())
        .collect::<BTreeSet<_>>();
    for incoming in &document.data.decisions {
        let before = current
            .get(&incoming.node)
            .map(|node| decision(store, node))
            .transpose()?
            .flatten();
        let node = &mut plan.nodes[indexes[&incoming.node]];
        if before.as_ref() == Some(incoming) && !node.write {
            continue;
        }
        if before.is_some() && before.as_ref() != Some(incoming) && !force {
            return Err(conflict("decision", &incoming.node));
        }
        node.write = true;
        plan.decisions.push((incoming.clone(), before));
    }
    // If a newer hypothesis has no decision, its old version's verdict is not
    // reused. Preserve old rows: the new recorded version deliberately has none.
    for node in &plan.nodes {
        if node.write
            && node.value.node_type == "hypothesis"
            && !incoming_decisions.contains(node.value.id.as_str())
        {
            if current
                .get(&node.value.id)
                .map(|old| decision(store, old))
                .transpose()?
                .flatten()
                .is_some()
                && !force
            {
                return Err(conflict("decision", &node.value.id));
            }
        }
    }
    for incoming in &document.data.slots {
        let params = BTreeMap::from([
            (
                "initiative".into(),
                DataValue::Str(incoming.initiative.clone().into()),
            ),
            ("slot".into(), DataValue::Str(incoming.slot.clone().into())),
        ]);
        let rows = db::query(
            store,
            "?[node_id] := *slot_occupant{initiative,slot,node_id},initiative=$initiative,slot=$slot",
            params,
        )?;
        let before = rows
            .rows
            .first()
            .map(|row| {
                db::string(&row[0]).map(|node| AmgSlot {
                    initiative: incoming.initiative.clone(),
                    slot: incoming.slot.clone(),
                    node,
                })
            })
            .transpose()?;
        if before.as_ref() == Some(incoming) {
            continue;
        }
        if before.is_some() && !force {
            return Err(conflict("slot", &incoming.slot));
        }
        plan.slots.push((incoming.clone(), before));
    }
    // Plan before-images as well as writes so dry-run counts include invalidation.
    let imported = document
        .data
        .trails
        .iter()
        .map(|t| t.id.as_str())
        .collect::<BTreeSet<_>>();
    let mut invalidated = BTreeSet::new();
    for node in plan.nodes.iter().filter(|n| n.write && n.before.is_some()) {
        for row in db::query(
            store,
            "?[chain_id] := *chain_member{chain_id,node_id},node_id=$id",
            BTreeMap::from([("id".into(), DataValue::Str(node.value.id.clone().into()))]),
        )?
        .rows
        {
            let id = db::string(&row[0])?;
            if imported.contains(id.as_str()) || !invalidated.insert(id.clone()) {
                continue;
            }
            if let Some(before) = trail(store, &id)?.filter(|t| t.stale == Some(false)) {
                let mut after = before.clone();
                after.stale = Some(true);
                plan.invalidated_trails.push((before, after));
            }
        }
    }
    Ok(())
}
