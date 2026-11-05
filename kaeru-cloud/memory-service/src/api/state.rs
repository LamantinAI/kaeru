//! `AppState` — the shared application state, cloned per request. Holds the
//! cloud substrate handle and the expected bearer token. Each field has a
//! `FromRef` impl so handlers (`State<Arc<Store>>`) and the auth extractor
//! (`Arc<str>` via `FromRef`) can pull just the piece they need.

use std::sync::Arc;

use axum::extract::FromRef;
use kaeru_core::Store;

#[derive(Clone)]
pub struct AppState {
    /// Expected bearer token. Empty = auth disabled (dev / loopback).
    pub api_token: Arc<str>,
    /// The cloud substrate. `Store` is internally synchronised (Cozo /
    /// RocksDB), so a shared `Arc` serves concurrent requests safely.
    pub store: Arc<Store>,
}

impl FromRef<AppState> for Arc<str> {
    fn from_ref(state: &AppState) -> Self {
        state.api_token.clone()
    }
}

impl FromRef<AppState> for Arc<Store> {
    fn from_ref(state: &AppState) -> Self {
        state.store.clone()
    }
}
