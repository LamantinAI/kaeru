//! Replaceable browser surface with a mount independent of MCP transports.
mod assets;
mod cookies;
mod handlers;
use crate::settings::ui::UiMount;
use axum::{Router, http::Uri, response::Redirect, routing::get};
pub(crate) use handlers::{BrowserAuth, api_access};

#[cfg(test)]
pub fn router(state: BrowserAuth) -> Router {
    router_with_reads(state, None)
}
pub fn router_with_reads(state: BrowserAuth, reads: Option<Router>) -> Router {
    let mount = state.mount.clone();
    let mut routes = assets::router_at(mount.clone()).merge(handlers::router(state));
    if mount.prefixed()
        && let Some(reads) = reads
    {
        routes = routes.merge(reads);
    }
    mounted(mount, routes)
}
pub fn auth_routes(state: BrowserAuth) -> Router {
    mounted(state.mount.clone(), handlers::router(state))
}
fn mounted(mount: UiMount, routes: Router) -> Router {
    if !mount.prefixed() {
        return routes;
    }
    let root = mount.page_root().to_string();
    let canonical = root.clone();
    Router::new()
        .route(
            &root,
            get(move |uri: Uri| {
                let path = format!(
                    "{canonical}/{}",
                    uri.query().map(|q| format!("?{q}")).unwrap_or_default()
                );
                async move { Redirect::temporary(&path) }
            }),
        )
        .nest(&format!("{root}/"), routes)
}
