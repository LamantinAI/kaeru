//! Replaceable instance frontend over the shared authenticated memory API.

mod assets;
mod handlers;

pub(crate) use handlers::{BrowserAuth, api_access};

pub fn router(state: BrowserAuth) -> axum::Router {
    assets::router().merge(handlers::router(state))
}

pub fn auth_routes(state: BrowserAuth) -> axum::Router {
    handlers::router(state)
}
