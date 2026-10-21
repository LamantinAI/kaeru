//! Conflict detection and effective destination state, without writes.

use std::collections::{BTreeMap, BTreeSet};

use boards::validate_board_merge;
use cozo::DataValue;
use metadata::merge_metadata;
use records::{conflict, edge_equal, effective, history_type, node_equal};

use super::{storage as db, validate::invalid};
use crate::config::amg::MAX_ORIGINS;
use crate::models::amg::{
    AmgDecision, AmgDocument, AmgEdge, AmgImportReport, AmgNode, AmgSlot, AmgTrail,
};
use crate::{Result, Store};

mod boards;
mod metadata;
mod records;

pub(super) struct NodeChange {
    pub value: AmgNode,
    pub before: Option<AmgNode>,
    pub before_memberships: Vec<String>,
    pub write: bool,
    pub existed: bool,
    pub add: Vec<String>,
}
pub(super) struct EdgeChange {
    pub value: AmgEdge,
    pub before_memberships: Vec<String>,
    pub write: bool,
    pub existed: bool,
    pub add: Vec<String>,
}
pub(super) struct Plan {
    pub nodes: Vec<NodeChange>,
    pub edges: Vec<EdgeChange>,
    pub trails: Vec<(AmgTrail, Option<AmgTrail>)>,
    pub invalidated_trails: Vec<(AmgTrail, AmgTrail)>,
    pub slots: Vec<(AmgSlot, Option<AmgSlot>)>,
    pub decisions: Vec<(AmgDecision, Option<AmgDecision>)>,
    pub report: AmgImportReport,
}

pub(super) fn build(
    store: &Store,
    document: &AmgDocument,
    force: bool,
    dry_run: bool,
    digest: String,
) -> Result<Plan> {
    let ids = document
        .data
        .nodes
        .iter()
        .map(|n| n.id.clone())
        .collect::<BTreeSet<_>>();
    let current = db::nodes(store, &ids)?;
    let current_edges = db::edges(store, &ids)?;
    // Junctions survive retraction, so live rows alone cannot describe the merge.
    let node_memberships = db::memberships(store, false)?;
    let edge_memberships = db::memberships(store, true)?;
    let mut plan = Plan {
        nodes: vec![],
        edges: vec![],
        trails: vec![],
        invalidated_trails: vec![],
        slots: vec![],
        decisions: vec![],
        report: AmgImportReport {
            dry_run,
            force,
            document_sha256: digest,
            ..Default::default()
        },
    };
    for name in &document.metadata.initiatives {
        if db::query(
            store,
            "?[name] := *initiative{name},name=$name",
            BTreeMap::from([("name".into(), DataValue::Str(name.clone().into()))]),
        )?
        .rows
        .is_empty()
        {
            plan.report.updated_metadata += 1;
        }
    }
    for source in &document.data.nodes {
        let before = current.get(&source.id).cloned();
        let old_type = if let Some(n) = &before {
            Some(n.node_type.clone())
        } else {
            history_type(store, &source.id)?
        };
        let existed = old_type.is_some();
        if old_type
            .as_ref()
            .is_some_and(|kind| *kind != source.node_type)
        {
            return Err(invalid(format!(
                "cannot change node type under existing identity {}",
                source.id
            )));
        }
        let before_memberships = node_memberships
            .get(&source.id)
            .cloned()
            .unwrap_or_default();
        let value = effective(source, before.as_ref(), &before_memberships);
        let write = before.as_ref().is_none_or(|old| !node_equal(&value, old));
        if write && existed && !force {
            return Err(conflict("node", &source.id));
        }
        let add = source
            .initiatives
            .iter()
            .filter(|n| !before_memberships.contains(n))
            .cloned()
            .collect::<Vec<_>>();
        plan.nodes.push(NodeChange {
            value,
            before,
            before_memberships,
            write,
            existed,
            add,
        });
    }
    for source in &document.data.edges {
        let key = db::edge_key(&source.src, &source.dst, &source.edge_type);
        let before = current_edges.get(&key).cloned();
        let params = BTreeMap::from([
            ("src".into(), DataValue::Str(source.src.clone().into())),
            ("dst".into(), DataValue::Str(source.dst.clone().into())),
            (
                "kind".into(),
                DataValue::Str(source.edge_type.clone().into()),
            ),
        ]);
        let existed = !db::query(
            store,
            "?[validity] := *edge{src,dst,edge_type,validity},src=$src,dst=$dst,edge_type=$kind",
            params,
        )?
        .rows
        .is_empty();
        let write = before.as_ref().is_none_or(|old| !edge_equal(source, old));
        if write && existed && !force {
            return Err(conflict("edge", &key));
        }
        let before_memberships = edge_memberships
            .get(&db::legacy_edge_key(
                &source.src,
                &source.dst,
                &source.edge_type,
            ))
            .cloned()
            .unwrap_or_default();
        let add = source
            .initiatives
            .iter()
            .filter(|n| !before_memberships.contains(n))
            .cloned()
            .collect();
        let mut value = source.clone();
        value.initiatives.extend(before_memberships.iter().cloned());
        value.initiatives.sort();
        value.initiatives.dedup();
        plan.edges.push(EdgeChange {
            value,
            before_memberships,
            write,
            existed,
            add,
        });
    }
    merge_metadata(store, document, &current, force, &mut plan)?;
    validate_board_merge(store, &plan)?;
    for node in &plan.nodes {
        if node.write && node.value.origins.len() >= MAX_ORIGINS {
            return Err(invalid("no room for import provenance"));
        }
        if node.write {
            if node.existed {
                plan.report.updated_nodes += 1;
            } else {
                plan.report.created_nodes += 1;
            }
        } else {
            plan.report.unchanged_nodes += 1;
        }
        plan.report.updated_metadata += node.add.len();
    }
    for edge in &plan.edges {
        if edge.write && edge.value.origins.len() >= MAX_ORIGINS {
            return Err(invalid("no room for import provenance"));
        }
        if edge.write {
            if edge.existed {
                plan.report.updated_edges += 1;
            } else {
                plan.report.created_edges += 1;
            }
        } else {
            plan.report.unchanged_edges += 1;
        }
        plan.report.updated_metadata += edge.add.len();
    }
    plan.report.updated_metadata +=
        plan.trails.len() + plan.invalidated_trails.len() + plan.slots.len() + plan.decisions.len();
    Ok(plan)
}
