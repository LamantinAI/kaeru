//! Private logical items used to reconstruct heterogeneous read pages.
use crate::config::api::API_BODY_CHUNK_CHARS;
use crate::models::{
    HistoryEvent, NodeData, RelationData, ResponseData, RetentionProposal, SlotChange, TrailData,
};
use serde_json::Value;

#[derive(Clone)]
pub(super) enum Item {
    Hit(NodeData),
    Section(usize, NodeData),
    Slot(SlotChange),
    Proposal(RetentionProposal),
    Version(HistoryEvent),
    Initiative(String),
    Metadata(Value),
    Lineage(RelationData),
    Body(String, usize),
    Conflict(RelationData),
    Reason(TrailData),
    Relation(RelationData),
    Provenance(NodeData),
    Path(String),
}

pub(super) fn items(data: &ResponseData) -> Vec<Item> {
    match data {
        ResponseData::Search(d) => d.hits.iter().cloned().map(Item::Hit).collect(),
        ResponseData::Context(d) => d
            .sections
            .iter()
            .enumerate()
            .flat_map(|(i, s)| s.records.iter().cloned().map(move |n| Item::Section(i, n)))
            .chain(d.slots.iter().cloned().map(Item::Slot))
            .chain(d.proposals.iter().cloned().map(Item::Proposal))
            .collect(),
        ResponseData::History(d) => d
            .versions
            .iter()
            .cloned()
            .map(Item::Version)
            .chain(d.metadata_events.iter().cloned().map(Item::Metadata))
            .chain(d.lineage.iter().cloned().map(Item::Lineage))
            .collect(),
        ResponseData::Initiatives(d) => d
            .initiatives
            .iter()
            .cloned()
            .map(Item::Initiative)
            .collect(),
        ResponseData::Record(d) => {
            let chars: Vec<char> = d.node.body.as_deref().unwrap_or("").chars().collect();
            let body = chars
                .chunks(API_BODY_CHUNK_CHARS)
                .enumerate()
                .map(|(i, c)| Item::Body(c.iter().collect(), i * API_BODY_CHUNK_CHARS));
            d.conflicts
                .iter()
                .cloned()
                .map(Item::Conflict)
                .chain(d.reasoning.iter().cloned().map(Item::Reason))
                .chain(d.relations.iter().cloned().map(Item::Relation))
                .chain(d.provenance.iter().cloned().map(Item::Provenance))
                .chain(d.path.iter().cloned().map(Item::Path))
                .chain(body)
                .collect()
        }
        ResponseData::Mutation(_) | ResponseData::Graph(_) => vec![],
    }
}

pub(super) fn empty_data(data: &ResponseData) -> ResponseData {
    let mut d = data.clone();
    match &mut d {
        ResponseData::Search(d) => d.hits.clear(),
        ResponseData::Context(d) => {
            for section in &mut d.sections {
                section.records.clear();
            }
            d.slots.clear();
            d.proposals.clear();
        }
        ResponseData::History(d) => {
            d.versions.clear();
            d.metadata_events.clear();
            d.lineage.clear();
        }
        ResponseData::Initiatives(d) => d.initiatives.clear(),
        ResponseData::Record(d) => {
            d.node.body = Some(String::new());
            d.conflicts.clear();
            d.reasoning.clear();
            d.relations.clear();
            d.provenance.clear();
            d.path.clear();
        }
        ResponseData::Mutation(_) | ResponseData::Graph(_) => {}
    }
    d
}

pub(super) fn push(data: &mut ResponseData, item: &Item) {
    match (data, item) {
        (ResponseData::Search(d), Item::Hit(n)) => d.hits.push(n.clone()),
        (ResponseData::Context(d), Item::Section(i, n)) => d.sections[*i].records.push(n.clone()),
        (ResponseData::Context(d), Item::Slot(n)) => d.slots.push(n.clone()),
        (ResponseData::Context(d), Item::Proposal(n)) => d.proposals.push(n.clone()),
        (ResponseData::History(d), Item::Version(n)) => d.versions.push(n.clone()),
        (ResponseData::History(d), Item::Metadata(n)) => d.metadata_events.push(n.clone()),
        (ResponseData::History(d), Item::Lineage(n)) => d.lineage.push(n.clone()),
        (ResponseData::Initiatives(d), Item::Initiative(n)) => d.initiatives.push(n.clone()),
        (ResponseData::Record(d), Item::Body(text, _)) => {
            d.node.body.as_mut().unwrap().push_str(text)
        }
        (ResponseData::Record(d), Item::Conflict(n)) => d.conflicts.push(n.clone()),
        (ResponseData::Record(d), Item::Reason(n)) => d.reasoning.push(n.clone()),
        (ResponseData::Record(d), Item::Relation(n)) => d.relations.push(n.clone()),
        (ResponseData::Record(d), Item::Provenance(n)) => d.provenance.push(n.clone()),
        (ResponseData::Record(d), Item::Path(n)) => d.path.push(n.clone()),
        _ => unreachable!("pagination items belong to their response data"),
    }
}
