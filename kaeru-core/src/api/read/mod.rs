//! Scoped, bounded read dispatch shared by all adapters.

use paging::{continued, paginate};
use serde_json::to_string;

use super::write::{policy_read, retention_preview};
use super::{ApiOptions, ApiResult, Args, Issue, Response, issue_from_error};
use crate::api::control::checkpoint;
use crate::{Error, Store};
mod context;
mod history;
mod node;
mod paging;
mod reasoning;
mod record;
mod reflect;
mod search;
pub(in crate::api) mod session;
pub(super) mod trace;
pub(super) use context::{context, scope_nodes};
pub(super) use history::{history, initiatives, time_string};
pub(super) use node::{created_at, entity, filter_references, node, relations};
pub(crate) use paging::Snapshot;
pub(super) use reasoning::{content_hash, trails};
pub(super) use record::{historical_node, read};
pub(super) use reflect::reflect;
pub(super) use search::search;

pub(super) fn dispatch(
    store: &Store,
    name: &str,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    checkpoint()?;
    if args.has("cursor") {
        return continued(store, name, args, options);
    }
    let result = store
        .db_ref()
        .read_snapshot(|| {
            session::run(store, options, || match name {
                "context" => context(store, args, options),
                "search" => search(store, args, options),
                "read" => read(store, args, options),
                "history" => history(store, args, options),
                "reflect" => reflect(store, args, options),
                "initiatives" => initiatives(store, args, options),
                "policy" => policy_read(store, args),
                "retention" => retention_preview(store, args, options),
                _ => Err(Issue::new(
                    "UNKNOWN_TOOL",
                    "operation is not a canonical read",
                )),
            })
            .map_err(|issue| Error::Invalid(to_string(&issue).expect("issue serializes")))
        })
        .map_err(issue_from_error);
    checkpoint()?;
    paginate(store, result?, args, options, 0, None)
}

#[cfg(test)]
mod query_plan_tests;
