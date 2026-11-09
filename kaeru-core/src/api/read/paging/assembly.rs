//! Budget-aware page construction without storage or delivery side effects.
use super::delivery::reserve_chars;
use super::items::{Item, empty_data, items, push};
use crate::api::control::checkpoint;
use crate::api::{ApiResult, Args};
use crate::config::api::{
    API_DEFAULT_CONTEXT_BUDGET_CHARS, API_DEFAULT_READ_BUDGET_CHARS, API_DEFAULT_SEARCH_LIMIT,
};
use crate::models::{ContentRange, Continuation, Issue, Omission, Response, ResponseData};
use serde_json::{json, to_string};

pub(super) struct Page {
    pub response: Response,
    pub next_offset: usize,
    pub total: usize,
}
impl Page {
    pub fn has_more(&self) -> bool {
        self.next_offset < self.total
    }
}
fn size(response: &Response) -> usize {
    to_string(response).unwrap().chars().count() + reserve_chars(response)
}

/// None keeps the original complete response, without cloning its body.
pub(super) fn assemble(
    full: &Response,
    args: &Args,
    offset: usize,
    token: &str,
) -> ApiResult<Option<Page>> {
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
    let entries = items(&full.data);
    if offset > entries.len() {
        return Err(Issue::new(
            "CURSOR_STALE",
            "continuation is outside the snapshot",
        ));
    }
    if offset == 0 && entries.len() <= limit && size(full) <= budget {
        return Ok(None);
    }
    let mut page = full.clone();
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
    }
    Ok(Some(Page {
        response: page,
        next_offset: end,
        total: entries.len(),
    }))
}
