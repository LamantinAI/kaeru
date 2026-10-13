//! Bounded retrieval with currentness and stored reasoning on the ordinary path.

use chrono::{DateTime, Utc};
use serde_json::json;

use crate::api::access::{check_scope, resolve};
use crate::api::types::{
    ContentRange, HistoryData, HistoryEvent, InitiativesData, Response, ResponseData,
};
use crate::api::{ApiOptions, ApiResult, Args, issue_from_error};
use crate::{Store, history as core_history, list_initiatives, unversioned_changes};

use super::relations;

pub(in crate::api) fn history(
    store: &Store,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    let id = resolve(store, &args.parse("node")?, options, None, true, false)?;
    let versions = core_history(store, &id)
        .map_err(issue_from_error)?
        .into_iter()
        .map(|r| {
            let body_range = r
                .body
                .as_ref()
                .filter(|b| b.chars().count() > 400)
                .map(|b| ContentRange {
                    start: 0,
                    end: 400,
                    total: b.chars().count(),
                });
            HistoryEvent {
                recorded_at: time_string(r.seconds),
                asserted: r.asserted,
                name: r.name,
                body: r.body.map(|b| b.chars().take(400).collect()),
                body_range,
            }
        })
        .collect();
    let metadata=unversioned_changes(store,&id).map_err(issue_from_error)?.into_iter().map(|e|json!({"operation":e.op,"recorded_at":time_string(e.seconds as f64),"actor":e.actor,"previous_value_stored":false})).collect();
    Ok(Response::read(
        "history",
        ResponseData::History(HistoryData {
            lineage: relations(store, &id, options)?
                .into_iter()
                .filter(|e| matches!(e.edge_type.as_str(), "supersedes" | "consolidated_to"))
                .collect(),
            id,
            versions,
            metadata_events: metadata,
        }),
        store.current_initiative().into_iter().collect(),
    ))
}

pub(in crate::api) fn time_string(seconds: f64) -> String {
    DateTime::<Utc>::from_timestamp(seconds as i64, 0)
        .map(|t| t.to_rfc3339())
        .unwrap_or_else(|| seconds.to_string())
}

pub(in crate::api) fn initiatives(
    store: &Store,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    let filter = args.text("filter");
    let scopes = list_initiatives(store)
        .map_err(issue_from_error)?
        .into_iter()
        .filter(|s| check_scope(s, options).is_ok())
        .filter(|s| filter.as_ref().is_none_or(|f| s.contains(f)))
        .collect();
    Ok(Response::read(
        "initiatives",
        ResponseData::Initiatives(InitiativesData {
            initiatives: scopes,
        }),
        vec![],
    ))
}
