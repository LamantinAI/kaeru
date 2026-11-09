//! Context delivery side effects and their retained-snapshot bookkeeping.
use crate::api::control::{checkpoint, suspend};
use crate::api::read::node;
use crate::api::{ApiOptions, ApiResult, Response, issue_from_error};
use crate::hygiene::take_pending_report;
use crate::models::ResponseData;
use crate::{Store, stamp_reminder_seen};
use chrono::Utc;
use serde_json::{Value, to_string};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

pub(super) fn reserve_chars(response: &Response) -> usize {
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

pub(super) fn deliver(
    store: &Store,
    mut response: Response,
    options: &ApiOptions,
) -> ApiResult<Response> {
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

pub(super) fn patch_snapshot(full: &mut Arc<Response>, page: &Response) {
    if matches!(full.data, ResponseData::Context(_))
        && let (ResponseData::Context(original), ResponseData::Context(emitted)) =
            (&mut Arc::make_mut(full).data, &page.data)
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
}
