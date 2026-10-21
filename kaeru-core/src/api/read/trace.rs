//! Extract only delivered record identities; never inspect arbitrary JSON content.

use crate::Store;
use crate::models::{NodeData, Response, ResponseData};

pub(in crate::api) fn delivered(store: &Store, response: &Response, fallback_scope: Option<&str>) {
    if !store.read_trace_enabled() {
        return;
    }
    let mut ids = Vec::new();
    let mut initiatives = response
        .coverage
        .as_ref()
        .map(|c| c.initiatives.clone())
        .unwrap_or_default();
    if let Some(scope) = fallback_scope {
        initiatives.push(scope.into());
    }
    let mut record = |node: &NodeData| {
        ids.push(node.id.clone());
        initiatives.extend(node.initiatives.iter().cloned());
    };
    match &response.data {
        ResponseData::Graph(data) => {
            ids.extend(data.graph.nodes.iter().map(|n| n.id.clone()));
            ids.extend(
                data.graph
                    .edges
                    .iter()
                    .flat_map(|e| [e.from.clone(), e.to.clone()]),
            );
            ids.extend(
                data.graph
                    .members
                    .iter()
                    .flat_map(|m| [m.chain_id.clone(), m.node_id.clone()]),
            );
        }
        ResponseData::Record(data) => {
            record(&data.node);
            for node in &data.provenance {
                record(node);
            }
            ids.extend(data.reasoning.iter().map(|t| t.id.clone()));
        }
        ResponseData::Search(data) => {
            for node in &data.hits {
                record(node);
            }
        }
        ResponseData::Context(data) => {
            for section in &data.sections {
                for node in &section.records {
                    record(node);
                }
            }
            ids.extend(data.proposals.iter().map(|p| p.id.clone()));
        }
        ResponseData::History(data) => {
            ids.push(data.id.clone());
        }
        ResponseData::Initiatives(data) => {
            initiatives.extend(data.initiatives.iter().cloned());
        }
        ResponseData::Mutation(_) => {}
    }
    store.record_read_trace(&response.operation, ids, initiatives);
}
