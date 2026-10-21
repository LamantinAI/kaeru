//! One scoped read owns this cache. It ends before delivery mutates reminders.
mod bulk;
mod fields;
#[cfg(test)]
mod tests;

use std::cell::RefCell;
use std::collections::BTreeMap;

pub(in crate::api) use bulk::prefetch;
use cozo::DataValue;
pub(in crate::api) use fields::{
    created, edge_rows, memberships, pin, query, raw_node, recorded, visibility,
};
use serde_json::Value;

use crate::api::{ApiOptions, ApiResult};
use crate::config::api::{API_READ_BATCH_SIZE, API_READ_CACHE_RECORDS};
use crate::models::RelationData;
use crate::{NodeFull, Store};
type CheckKey = (String, Option<String>, bool);
struct Frame {
    store: usize,
    generation: u64,
    policy: ApiOptions,
    members: BTreeMap<String, Vec<String>>,
    visible: BTreeMap<String, Option<String>>,
    historical_visible: BTreeMap<String, Option<String>>,
    recorded: BTreeMap<String, Option<f64>>,
    created: BTreeMap<String, Option<f64>>,
    pins: BTreeMap<String, Option<Value>>,
    raw: BTreeMap<String, Option<NodeFull>>,
    edges: BTreeMap<String, Vec<Vec<DataValue>>>,
    relations: BTreeMap<String, Vec<RelationData>>,
    checks: BTreeMap<CheckKey, ApiResult<()>>,
}
impl Frame {
    fn new(store: &Store, policy: &ApiOptions) -> Self {
        Self {
            store: store as *const Store as usize,
            generation: store.generation(),
            policy: policy.clone(),
            members: BTreeMap::new(),
            visible: BTreeMap::new(),
            historical_visible: BTreeMap::new(),
            recorded: BTreeMap::new(),
            created: BTreeMap::new(),
            pins: BTreeMap::new(),
            raw: BTreeMap::new(),
            edges: BTreeMap::new(),
            relations: BTreeMap::new(),
            checks: BTreeMap::new(),
        }
    }
    fn same_policy(&self, options: &ApiOptions) -> bool {
        self.policy.allowed_initiatives == options.allowed_initiatives
            && self.policy.denied_initiatives == options.denied_initiatives
            && self.policy.include_local == options.include_local
            && self.policy.caller == options.caller
    }
}
thread_local! {static ACTIVE:RefCell<Vec<Frame>>=const{RefCell::new(vec![])};}
struct Guard;
impl Drop for Guard {
    fn drop(&mut self) {
        ACTIVE.with_borrow_mut(|frames| {
            frames.pop();
        });
    }
}
pub(in crate::api) fn run<T>(store: &Store, options: &ApiOptions, f: impl FnOnce() -> T) -> T {
    ACTIVE.with_borrow_mut(|frames| frames.push(Frame::new(store, options)));
    let _guard = Guard;
    f()
}
fn current<T>(
    store: &Store,
    options: Option<&ApiOptions>,
    f: impl FnOnce(&mut Frame) -> T,
) -> Option<T> {
    ACTIVE.with_borrow_mut(|frames| {
        let frame = frames.last_mut()?;
        if frame.store != store as *const Store as usize
            || options.is_some_and(|opts| !frame.same_policy(opts))
        {
            return None;
        }
        if frame.generation != store.generation()
            || frame.visible.len() > API_READ_CACHE_RECORDS
            || frame.checks.len() > API_READ_CACHE_RECORDS * 4
        {
            *frame = Frame::new(store, &frame.policy);
        }
        Some(f(frame))
    })
}
fn current_at<T>(
    store: &Store,
    options: Option<&ApiOptions>,
    generation: u64,
    f: impl FnOnce(&mut Frame) -> T,
) -> Option<T> {
    current(store, options, |frame| {
        if frame.generation == generation {
            Some(f(frame))
        } else {
            None
        }
    })
    .flatten()
}
fn memo<T: Clone, E>(
    store: &Store,
    options: Option<&ApiOptions>,
    get: impl FnOnce(&Frame) -> Option<T>,
    set: impl FnOnce(&mut Frame, T),
    load: impl FnOnce() -> Result<T, E>,
) -> Result<T, E> {
    if let Some(Some(value)) = current(store, options, |frame| get(frame)) {
        return Ok(value);
    }
    let generation = store.generation();
    let value = load()?;
    current_at(store, options, generation, |frame| {
        set(frame, value.clone())
    });
    Ok(value)
}
pub(in crate::api) fn check(
    store: &Store,
    id: &str,
    scope: Option<&str>,
    options: &ApiOptions,
    historical: bool,
    f: impl FnOnce() -> ApiResult<()>,
) -> ApiResult<()> {
    let key = (id.into(), scope.map(str::to_string), historical);
    if let Some(Some(value)) = current(store, Some(options), |frame| {
        frame.checks.get(&key).cloned()
    }) {
        return value;
    }
    let generation = store.generation();
    let result = f();
    if result
        .as_ref()
        .is_err_and(|issue| issue.code == "CANCELLED")
    {
        return result;
    }
    current_at(store, Some(options), generation, |frame| {
        frame.checks.insert(key, result.clone());
    });
    result
}
pub(in crate::api) fn relations(
    store: &Store,
    id: &str,
    options: &ApiOptions,
    f: impl FnOnce() -> ApiResult<Vec<RelationData>>,
) -> ApiResult<Vec<RelationData>> {
    memo(
        store,
        Some(options),
        |s| s.relations.get(id).cloned(),
        |s, v| {
            if s.relations.len() >= API_READ_BATCH_SIZE * 2 {
                s.relations.clear();
            }
            s.relations.insert(id.into(), v);
        },
        f,
    )
}
