//! Bounded retrieval with currentness and stored reasoning on the ordinary path.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::time::Instant;
mod graph;

use chrono::Utc;
use serde_json::{Value, json, to_string, to_vec};

use super::node;
use crate::api::control::{checkpoint, suspend};
use crate::api::{ApiOptions, ApiResult, Args, hash, issue_from_error};
use crate::config::api::{
    API_BODY_CHUNK_CHARS, API_DEFAULT_CONTEXT_BUDGET_CHARS, API_DEFAULT_READ_BUDGET_CHARS,
    API_DEFAULT_SEARCH_LIMIT, API_MAX_CACHED_SNAPSHOTS, API_MAX_SNAPSHOT_BYTES, API_SNAPSHOT_TTL,
};
use crate::hygiene::take_pending_report;
use crate::models::{
    ContentRange, Continuation, HistoryEvent, Issue, NodeData, Omission, RelationData, Response,
    ResponseData, RetentionProposal, SlotChange, TrailData,
};
use crate::{Store, new_node_id, stamp_reminder_seen};

#[derive(Clone)]
pub(crate) struct Snapshot {
    caller: String,
    arguments_hash: String,
    policy_hash: String,
    generation: u64,
    expires: Instant,
    response: Arc<Response>,
}

fn policy_hash(options: &ApiOptions) -> String {
    hash(&json!([
        options.allowed_initiatives,
        options.denied_initiatives,
        options.include_local
    ]))
}

fn arguments_hash(name: &str, args: &Args) -> String {
    let mut a = args.0.clone();
    a.as_object_mut().unwrap().remove("cursor");
    hash(&json!([name, a]))
}

pub(in crate::api) fn continued(
    store: &Store,
    name: &str,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    checkpoint()?;
    let cursor = args.required_text("cursor")?;
    let (token, offset) = cursor
        .rsplit_once(':')
        .ok_or_else(|| Issue::new("CURSOR_STALE", "invalid continuation"))?;
    let offset = offset
        .parse::<usize>()
        .map_err(|_| Issue::new("CURSOR_STALE", "invalid continuation"))?;
    let snapshot = store
        .api_snapshots
        .lock()
        .unwrap_or_else(|p| p.into_inner())
        .get(token)
        .cloned()
        .ok_or_else(|| {
            Issue::new(
                "CURSOR_STALE",
                "continuation expired; repeat the initial read",
            )
        })?;
    if snapshot.expires < Instant::now()
        || snapshot.caller != options.caller
        || snapshot.policy_hash != policy_hash(options)
        || snapshot.arguments_hash != arguments_hash(name, args)
        || snapshot.generation != store.generation()
    {
        return Err(Issue::new(
            "CURSOR_STALE",
            "memory or access changed; repeat the initial read",
        ));
    }
    paginate_shared(
        store,
        snapshot.response,
        args,
        options,
        offset,
        Some(token.into()),
    )
}

#[derive(Clone)]
enum Item {
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

fn items(data: &ResponseData) -> Vec<Item> {
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

fn empty_data(data: &ResponseData) -> ResponseData {
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

fn push(data: &mut ResponseData, item: &Item) {
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

fn size(response: &Response) -> usize {
    to_string(response).unwrap().chars().count() + delivery_reserve(response)
}

fn delivery_reserve(response: &Response) -> usize {
    let mut size = 0;
    if response.operation == "context"
        && let ResponseData::Context(data) = &response.data
    {
        let pending = data
            .sections
            .iter()
            .filter(|s| s.name == "reminders")
            .flat_map(|s| s.records.iter())
            .filter(|n| n.body_range.is_none() && !n.tags.iter().any(|t| t.starts_with("seen:")))
            .map(|n| n.id.as_str())
            .collect::<BTreeSet<_>>();
        let tag = to_string(&format!("seen:{}", Utc::now().format("%Y-%m-%d")))
            .unwrap()
            .chars()
            .count()
            + 1;
        size += data
            .sections
            .iter()
            .flat_map(|s| s.records.iter())
            .filter(|n| {
                pending.contains(n.id.as_str()) && !n.tags.iter().any(|t| t.starts_with("seen:"))
            })
            .count()
            * tag;
    }
    size
}

pub(in crate::api) fn paginate(
    store: &Store,
    full: Response,
    args: &Args,
    options: &ApiOptions,
    offset: usize,
    token: Option<String>,
) -> ApiResult<Response> {
    paginate_shared(store, Arc::new(full), args, options, offset, token)
}

fn paginate_shared(
    store: &Store,
    mut full: Arc<Response>,
    args: &Args,
    options: &ApiOptions,
    offset: usize,
    token: Option<String>,
) -> ApiResult<Response> {
    checkpoint()?;
    if matches!(full.data, ResponseData::Graph(_)) {
        return graph::page(store, full, args, options, offset, token);
    }
    let budget = args
        .value("budget_chars")
        .as_u64()
        .unwrap_or(if full.operation == "read" {
            API_DEFAULT_READ_BUDGET_CHARS as u64
        } else {
            API_DEFAULT_CONTEXT_BUDGET_CHARS as u64
        }) as usize;
    let limit = if full.operation == "search" {
        args.value("limit")
            .as_u64()
            .unwrap_or(API_DEFAULT_SEARCH_LIMIT as u64) as usize
    } else {
        usize::MAX
    };
    if to_vec(full.as_ref()).unwrap().len() + delivery_reserve(&full) > API_MAX_SNAPSHOT_BYTES {
        return Err(Issue::new(
            "SNAPSHOT_TOO_LARGE",
            "snapshot exceeds 8 MiB; narrow the view or use an authorized export for large sources",
        ));
    }
    let entries = items(&full.data);
    if offset > entries.len() {
        return Err(Issue::new(
            "CURSOR_STALE",
            "continuation is outside the snapshot",
        ));
    }
    if offset == 0 && entries.len() <= limit && size(&full) <= budget {
        return delivered(store, Arc::unwrap_or_clone(full), options);
    }
    let mut page = (*full).clone();
    page.data = empty_data(&full.data);
    if let ResponseData::Record(data) = &mut page.data {
        let total = match &full.data {
            ResponseData::Record(d) => d.node.body.as_deref().unwrap_or("").chars().count(),
            _ => 0,
        };
        data.node.body_range = Some(ContentRange {
            start: total,
            end: total,
            total,
        });
    }
    if let Some(c) = page.coverage.as_mut() {
        c.complete = false;
        c.omitted.push(Omission {
            section: "continuation".into(),
            count: entries.len().saturating_sub(offset),
            reason: "budget_or_limit".into(),
        });
    }
    let token = token.unwrap_or_else(new_node_id);
    // Reserve continuation metadata while sizing the page.
    let mut next_args = args.0.clone();
    next_args
        .as_object_mut()
        .unwrap()
        .insert("cursor".into(), json!(format!("{token}:{}", entries.len())));
    page.continuation = Some(Continuation {
        cursor: format!("{token}:{}", entries.len()),
        next_arguments: next_args,
    });
    let mut end = offset;
    for entry in entries.iter().skip(offset).take(limit) {
        checkpoint()?;
        let previous = page.data.clone();
        push(&mut page.data, entry);
        if size(&page) > budget {
            page.data = previous;
            break;
        }
        end += 1;
    }
    if end == offset && end < entries.len() {
        return Err(Issue::new(
            "BUDGET_TOO_SMALL",
            "required metadata/item cannot fit; increase budget_chars",
        ));
    }
    if let ResponseData::Record(d) = &mut page.data {
        let start = entries[offset..end].iter().find_map(|i| {
            if let Item::Body(_, n) = i {
                Some(*n)
            } else {
                None
            }
        });
        if let Some(start) = start {
            d.node.body_range = Some(ContentRange {
                start,
                end: start + d.node.body.as_deref().unwrap_or("").chars().count(),
                total: match &full.data {
                    ResponseData::Record(f) => f.node.body.as_deref().unwrap_or("").chars().count(),
                    _ => 0,
                },
            });
        }
    }
    if let Some(c) = page.coverage.as_mut() {
        c.rules_complete = full.coverage.as_ref().is_some_and(|c| c.rules_complete)
            && !entries[end..].iter().any(|item| match item {
                Item::Section(i, _) => {
                    matches!(&full.data,ResponseData::Context(d) if d.sections[*i].name=="rules")
                }
                _ => false,
            });
        c.omitted.last_mut().unwrap().count = entries.len() - end;
        if end == entries.len() {
            c.complete = full.coverage.as_ref().is_some_and(|c| c.complete);
            c.omitted.pop();
            page.continuation = None;
        }
    }
    page = delivered(store, page, options)?;
    if let (ResponseData::Context(original), ResponseData::Context(emitted)) =
        (&mut Arc::make_mut(&mut full).data, &page.data)
    {
        let updated = emitted
            .sections
            .iter()
            .filter(|s| s.name == "reminders")
            .flat_map(|s| s.records.iter())
            .map(|n| (n.id.clone(), n.clone()))
            .collect::<BTreeMap<_, _>>();
        for section in &mut original.sections {
            if section.name == "maintenance" {
                section.details = Value::Null;
            }
            for record in &mut section.records {
                if let Some(current) = updated.get(&record.id) {
                    *record = current.clone();
                }
            }
        }
    }
    if end < entries.len() {
        let cursor = format!("{token}:{end}");
        let mut next_args = args.0.clone();
        next_args
            .as_object_mut()
            .unwrap()
            .insert("cursor".into(), json!(cursor));
        page.continuation = Some(Continuation {
            cursor,
            next_arguments: next_args,
        });
        let mut snapshots = store
            .api_snapshots
            .lock()
            .unwrap_or_else(|p| p.into_inner());
        snapshots.retain(|_, s| s.expires > Instant::now());
        if snapshots.len() >= API_MAX_CACHED_SNAPSHOTS {
            snapshots.clear();
        }
        snapshots.insert(
            token,
            Snapshot {
                caller: options.caller.clone(),
                arguments_hash: arguments_hash(&full.operation, args),
                policy_hash: policy_hash(options),
                generation: store.generation(),
                expires: Instant::now() + API_SNAPSHOT_TTL,
                response: full,
            },
        );
    }
    Ok(page)
}

fn delivered(store: &Store, mut response: Response, options: &ApiOptions) -> ApiResult<Response> {
    checkpoint()?;
    // Reminder stamping and pending-report consumption form one delivery phase.
    // Once begun, finish it even if the caller disconnects in the meantime.
    let _uninterrupted = suspend();
    if response.operation == "context"
        && let ResponseData::Context(data) = &mut response.data
    {
        let mut updated = BTreeMap::new();
        for section in &mut data.sections {
            if section.name == "maintenance"
                && section.details.get("memory_shifted").is_some()
                && let Some(scope) = store.current_initiative()
            {
                take_pending_report(store, &scope).map_err(issue_from_error)?;
            }
            if section.name == "reminders" {
                for n in &mut section.records {
                    if n.body_range.is_some() {
                        continue;
                    }
                    stamp_reminder_seen(store, &n.id).map_err(issue_from_error)?;
                    *n = node(store, &n.id, options, false)?;
                    updated.insert(n.id.clone(), n.clone());
                }
            }
        }
        for section in &mut data.sections {
            for record in &mut section.records {
                if let Some(current) = updated.get(&record.id) {
                    *record = current.clone();
                }
            }
        }
    }
    Ok(response)
}
