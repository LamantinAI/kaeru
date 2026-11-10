//! Cookie parsing and issuance with explicit scope and legacy-root cleanup.
use crate::settings::{ui::UiMount, web};
use axum::{
    http::{HeaderMap, header},
    response::Response,
};
fn values(headers: &HeaderMap) -> impl Iterator<Item = &str> {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .filter_map(|v| v.trim().split_once('='))
        .filter(|(name, _)| *name == web::SESSION_COOKIE)
        .map(|(_, value)| value)
}
pub(super) fn present(headers: &HeaderMap) -> bool {
    values(headers).next().is_some()
}
pub(super) fn token(headers: &HeaderMap) -> Option<&str> {
    let mut tokens = values(headers);
    let token = tokens.next()?;
    if tokens.next().is_some() {
        None
    } else {
        Some(token)
    }
}
fn append(response: &mut Response, path: &str, token: &str, ttl: u64, secure: bool) {
    let value = format!(
        "{}={token}; Path={path}; HttpOnly; SameSite=Strict; Max-Age={ttl}{}",
        web::SESSION_COOKIE,
        if secure { "; Secure" } else { "" }
    );
    response.headers_mut().append(
        header::SET_COOKIE,
        value
            .parse()
            .expect("validated cookie path and URL-safe JWT"),
    );
}
pub(super) fn issue(response: &mut Response, mount: &UiMount, token: &str, ttl: u64, secure: bool) {
    if mount.prefixed() {
        append(response, "/", "", 0, secure);
    }
    append(response, mount.cookie_path(), token, ttl, secure);
}
