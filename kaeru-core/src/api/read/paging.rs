//! Bounded read paging: assemble, reserve, deliver, then publish or release.
use crate::api::control::checkpoint;
use crate::api::{ApiOptions, ApiResult, Args, hash};
use crate::config::api::API_MAX_SNAPSHOT_BYTES;
use crate::models::{Issue, Response, ResponseData};
use crate::{Store, new_node_id};
use serde_json::{json, to_vec};
use std::sync::Arc;
mod assembly;
mod cache;
mod delivery;
mod graph;
mod items;
pub(crate) use cache::SnapshotCache;

struct PageCursor<'a> {
    offset: usize,
    token: String,
    generation: u64,
    lease: Option<cache::Lease<'a>>,
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
    let lease = cache::checkout(store, name, token.into(), offset, args, options)?;
    let full = lease.snapshot.response.clone();
    let generation = lease.generation();
    paginate_shared(
        store,
        full,
        args,
        options,
        PageCursor {
            offset,
            token: token.into(),
            generation,
            lease: Some(lease),
        },
    )
}

pub(in crate::api) fn paginate(
    store: &Store,
    full: Response,
    args: &Args,
    options: &ApiOptions,
    generation: u64,
) -> ApiResult<Response> {
    let token = match &full.data {
        ResponseData::Graph(data) => data.graph.snapshot_id.clone(),
        _ => new_node_id(),
    };
    paginate_shared(
        store,
        Arc::new(full),
        args,
        options,
        PageCursor {
            offset: 0,
            token,
            generation,
            lease: None,
        },
    )
}

fn paginate_shared<'a>(
    store: &'a Store,
    mut full: Arc<Response>,
    args: &Args,
    options: &ApiOptions,
    cursor: PageCursor<'a>,
) -> ApiResult<Response> {
    checkpoint()?;
    if matches!(full.data, ResponseData::Graph(_)) {
        return graph::page(store, full, args, options, cursor);
    }
    let PageCursor {
        offset,
        token,
        generation,
        mut lease,
    } = cursor;
    let bytes = to_vec(full.as_ref()).unwrap().len() + delivery::reserve_chars(&full);
    if bytes > API_MAX_SNAPSHOT_BYTES {
        return Err(Issue::new(
            "SNAPSHOT_TOO_LARGE",
            "snapshot exceeds 8 MiB; narrow the view or use an authorized export for large sources",
        ));
    }
    let Some(page) = assembly::assemble(&full, args, offset, &token)? else {
        return delivery::deliver(store, Arc::unwrap_or_clone(full), options);
    };
    let has_more = page.has_more();
    if has_more && lease.is_none() {
        lease = Some(cache::reserve(
            store,
            token,
            full.clone(),
            args,
            options,
            generation,
            bytes,
        )?);
    }
    let response = delivery::deliver(store, page.response, options)?;
    delivery::patch_snapshot(&mut full, &response);
    if has_more {
        let current_generation = store.generation();
        let own_delivery_write =
            matches!(full.data, ResponseData::Context(_)) && store.db_ref().exclusive_operation();
        if current_generation != generation && !own_delivery_write {
            return Err(Issue::new("CURSOR_STALE", "memory changed during assembly"));
        }
        lease.expect("partial page has a reservation").publish(
            full,
            page.next_offset,
            current_generation,
        )?;
    } else if let Some(lease) = lease {
        lease.finish()?;
    }
    Ok(response)
}
