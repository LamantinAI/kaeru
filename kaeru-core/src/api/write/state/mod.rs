//! Intrinsic commands grouped by status, content, graph edits and trails.

use crate::Store;
use crate::api::{ApiOptions, ApiResult, Args, Response};
mod content;
mod relations;
mod status;
mod trail;
use content::{replacement, revise, synthesise};
use relations::{forget, pin, relate};
use status::{review, task_update, verdict};
use trail::trail;

pub(super) fn dispatch(
    store: &Store,
    name: &str,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    match name {
        "verdict" => verdict(store, args, options),
        "task_update" => task_update(store, args, options),
        "revise" => revise(store, args, options),
        "replace" | "lifecycle" => replacement(store, name, args, options),
        "relate" => relate(store, args, options),
        "review" => review(store, args, options),
        "trail" => trail(store, args, options),
        "pin" => pin(store, args, options),
        "forget" => forget(store, args, options),
        "synthesise" => synthesise(store, args, options),
        _ => unreachable!(),
    }
}
