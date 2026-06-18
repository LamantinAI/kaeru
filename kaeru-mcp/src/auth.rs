//! Static bearer-token authentication for inbound MCP requests.
//!
//! kaeru-mcp binds to loopback by default and ships with no auth — fine
//! when only local agent sessions reach it. Once the daemon is exposed on
//! a routable address (e.g. the central `feynman` stand), the open port is
//! full curator access to the vault for anyone who can reach it. This
//! module closes that gap with the simplest credible control: a single
//! shared secret.
//!
//! When `KAERU_MCP_AUTH_TOKEN` is set, [`require_bearer`] is layered over
//! the whole axum router, so **both** transports — streamable HTTP
//! (`/mcp`) and the legacy SSE pair (`/sse`, `/messages`) — demand
//! `Authorization: Bearer <token>` on every request. An empty token (the
//! default) leaves the middleware off entirely; that decision is made in
//! `main.rs`, not here.
//!
//! This is intentionally not OAuth: the MCP spec's auth flow is overkill
//! for a single-operator stand. A static token is opt-in, works with
//! `claude mcp add --header`, and is a clean upgrade path if richer auth
//! is ever needed.

use std::sync::Arc;

use axum::extract::{Request, State};
use axum::http::header::{AUTHORIZATION, WWW_AUTHENTICATE};
use axum::http::{HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

/// axum middleware that rejects any request lacking a matching bearer
/// token. The expected token is injected as state via
/// `from_fn_with_state`; `main.rs` only installs this layer when a
/// non-empty token is configured, so `expected` is never empty here.
pub async fn require_bearer(
    State(expected): State<Arc<str>>,
    request: Request,
    next: Next,
) -> Response {
    // Extract an owned copy first so the borrow on the request's headers
    // ends before `next.run` takes ownership of the request.
    let presented = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(bearer_token)
        .map(str::to_owned);

    match presented {
        Some(token) if constant_time_eq(token.as_bytes(), expected.as_bytes()) => {
            next.run(request).await
        }
        _ => {
            tracing::warn!("rejected MCP request: missing or invalid bearer token");
            unauthorized()
        }
    }
}

/// Parses the token out of an `Authorization` header value, accepting any
/// case of the `Bearer` scheme per RFC 7235. Returns `None` for malformed
/// or non-bearer values.
fn bearer_token(header: &HeaderValue) -> Option<&str> {
    let value = header.to_str().ok()?;
    let (scheme, token) = value.split_once(' ')?;
    scheme
        .eq_ignore_ascii_case("bearer")
        .then(|| token.trim())
        .filter(|t| !t.is_empty())
}

/// Length-checked constant-time byte comparison. Avoids leaking the
/// matched-prefix length through timing — cheap insurance for a secret
/// compared on every request.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

fn unauthorized() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        [(WWW_AUTHENTICATE, HeaderValue::from_static("Bearer"))],
        "Unauthorized: a valid bearer token is required\n",
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use axum::http::HeaderValue;

    use super::{bearer_token, constant_time_eq};

    fn parse(raw: &str) -> Option<String> {
        bearer_token(&HeaderValue::from_str(raw).unwrap()).map(str::to_owned)
    }

    #[test]
    fn extracts_token_case_insensitive_scheme() {
        assert_eq!(parse("Bearer s3cret").as_deref(), Some("s3cret"));
        assert_eq!(parse("bearer s3cret").as_deref(), Some("s3cret"));
        assert_eq!(parse("BEARER s3cret").as_deref(), Some("s3cret"));
    }

    #[test]
    fn rejects_malformed_or_non_bearer() {
        assert_eq!(parse("s3cret"), None); // no scheme
        assert_eq!(parse("Basic s3cret"), None); // wrong scheme
        assert_eq!(parse("Bearer "), None); // empty token
        assert_eq!(parse("Bearer    "), None); // whitespace-only token
    }

    #[test]
    fn constant_time_eq_matches_only_identical_bytes() {
        assert!(constant_time_eq(b"token", b"token"));
        assert!(!constant_time_eq(b"token", b"toked"));
        assert!(!constant_time_eq(b"token", b"token-longer"));
        assert!(!constant_time_eq(b"", b"x"));
        assert!(constant_time_eq(b"", b""));
    }
}
