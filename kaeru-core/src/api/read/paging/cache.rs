//! Active cursors are admitted, never evicted to make room for another caller.
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Instant;

use super::{arguments_hash, policy_hash};
use crate::Store;
use crate::api::{ApiOptions, ApiResult, Args, Issue, Response};
use crate::config::api::{
    API_MAX_CACHED_SNAPSHOT_BYTES, API_MAX_CACHED_SNAPSHOTS, API_SNAPSHOT_TTL,
};

#[derive(Clone)]
pub(super) struct Snapshot {
    caller: String,
    arguments_hash: String,
    policy_hash: String,
    generation: u64,
    expires: Instant,
    pub response: Arc<Response>,
}
struct Entry {
    snapshot: Snapshot,
    offset: usize,
    bytes: usize,
    busy: bool,
}
#[derive(Default)]
pub(crate) struct SnapshotCache {
    entries: BTreeMap<String, Entry>,
    bytes: usize,
}

impl SnapshotCache {
    fn purge(&mut self, now: Instant, generation: u64) {
        self.entries.retain(|_, e| {
            e.busy || (e.snapshot.expires > now && e.snapshot.generation == generation)
        });
        self.bytes = self.entries.values().map(|e| e.bytes).sum();
    }
}

pub(super) struct Lease<'a> {
    store: &'a Store,
    pub token: String,
    pub snapshot: Snapshot,
    new: bool,
    active: bool,
}
impl Drop for Lease<'_> {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        let mut cache = self
            .store
            .api_snapshots
            .lock()
            .unwrap_or_else(|p| p.into_inner());
        if self.new {
            if let Some(entry) = cache.entries.remove(&self.token) {
                cache.bytes -= entry.bytes;
            }
        } else if let Some(entry) = cache.entries.get_mut(&self.token) {
            entry.busy = false;
        }
    }
}

pub(super) fn checkout<'a>(
    store: &'a Store,
    name: &str,
    token: String,
    offset: usize,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Lease<'a>> {
    if store.db_ref().write_transaction_active() {
        return Err(Issue::new(
            "INVALID_OPERATION",
            "continuations cannot run inside a write transaction",
        ));
    }
    let now = Instant::now();
    let generation = store.generation();
    let mut cache = store
        .api_snapshots
        .lock()
        .unwrap_or_else(|p| p.into_inner());
    cache.purge(now, generation);
    let entry = cache
        .entries
        .get_mut(&token)
        .ok_or_else(|| Issue::new("CURSOR_STALE", "continuation is no longer active"))?;
    let snapshot = &entry.snapshot;
    if snapshot.expires <= now
        || snapshot.caller != options.caller
        || snapshot.policy_hash != policy_hash(options)
        || snapshot.arguments_hash != arguments_hash(name, args)
        || snapshot.generation != generation
        || entry.offset != offset
    {
        return Err(Issue::new(
            "CURSOR_STALE",
            "continuation identity, arguments, offset or memory changed",
        ));
    }
    if entry.busy {
        return Err(Issue::new(
            "CURSOR_BUSY",
            "another request owns this continuation",
        ));
    }
    entry.busy = true;
    let snapshot = entry.snapshot.clone();
    Ok(Lease {
        store,
        token,
        snapshot,
        new: false,
        active: true,
    })
}

pub(super) fn reserve<'a>(
    store: &'a Store,
    token: String,
    full: Arc<Response>,
    args: &Args,
    options: &ApiOptions,
    generation: u64,
    bytes: usize,
) -> ApiResult<Lease<'a>> {
    if store.db_ref().write_transaction_active() {
        return Err(Issue::new(
            "INVALID_OPERATION",
            "a write transaction cannot publish a continuation",
        ));
    }
    if generation != store.generation() {
        return Err(Issue::new("CURSOR_STALE", "memory changed during assembly"));
    }
    let now = Instant::now();
    let mut cache = store
        .api_snapshots
        .lock()
        .unwrap_or_else(|p| p.into_inner());
    cache.purge(now, store.generation());
    if cache.entries.len() >= API_MAX_CACHED_SNAPSHOTS
        || bytes > API_MAX_CACHED_SNAPSHOT_BYTES.saturating_sub(cache.bytes)
    {
        return Err(Issue::new(
            "SNAPSHOT_CAPACITY",
            "complete or expire active continuations before starting another",
        ));
    }
    if cache.entries.contains_key(&token) {
        return Err(Issue::new(
            "CURSOR_STALE",
            "snapshot identity is already active",
        ));
    }
    let snapshot = Snapshot {
        caller: options.caller.clone(),
        arguments_hash: arguments_hash(&full.operation, args),
        policy_hash: policy_hash(options),
        generation,
        expires: now + API_SNAPSHOT_TTL,
        response: full,
    };
    cache.entries.insert(
        token.clone(),
        Entry {
            snapshot: snapshot.clone(),
            offset: 0,
            bytes,
            busy: true,
        },
    );
    cache.bytes += bytes;
    Ok(Lease {
        store,
        token,
        snapshot,
        new: true,
        active: true,
    })
}

impl Lease<'_> {
    pub fn generation(&self) -> u64 {
        self.snapshot.generation
    }
    pub fn publish(mut self, full: Arc<Response>, offset: usize, generation: u64) -> ApiResult<()> {
        let mut cache = self
            .store
            .api_snapshots
            .lock()
            .unwrap_or_else(|p| p.into_inner());
        let entry = cache
            .entries
            .get_mut(&self.token)
            .ok_or_else(|| Issue::new("CURSOR_STALE", "snapshot was released"))?;
        if !entry.busy
            || entry.snapshot.expires <= Instant::now()
            || (!self.new && offset <= entry.offset)
        {
            return Err(Issue::new("CURSOR_STALE", "continuation cannot be renewed"));
        }
        if generation != self.store.generation()
            || serde_json::to_vec(full.as_ref()).unwrap().len() > entry.bytes
        {
            return Err(Issue::new(
                "CURSOR_STALE",
                "snapshot changed outside its reservation",
            ));
        }
        entry.snapshot.response = full;
        entry.snapshot.generation = generation;
        entry.offset = offset;
        entry.busy = false;
        self.active = false;
        Ok(())
    }
    pub fn finish(mut self) -> ApiResult<()> {
        let mut cache = self
            .store
            .api_snapshots
            .lock()
            .unwrap_or_else(|p| p.into_inner());
        let entry = cache
            .entries
            .remove(&self.token)
            .ok_or_else(|| Issue::new("CURSOR_STALE", "continuation was released"))?;
        cache.bytes -= entry.bytes;
        self.active = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
