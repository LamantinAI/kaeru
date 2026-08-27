//! Who is asking.
//!
//! Today there is exactly one answer — the operator running this daemon — and
//! [`Principal`] therefore carries no information. It exists anyway, because
//! the expensive part of introducing a caller identity is not writing the
//! type: it is changing every handler that never asked for one. A handler that
//! already takes a `Principal` gains a second variant for free.
//!
//! Deliberately **not** a tenant id. kaeru's unit of isolation is the `Store`,
//! not a column in the schema, so a hosted deployment gives a subscriber their
//! own store rather than a share of a common one. Identity here answers "may
//! this caller act", never "whose data is this" — nothing downstream of the
//! auth layer needs to know a tenant exists.

use std::convert::Infallible;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;

/// The authenticated caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Principal {
    /// The operator of this daemon. The bearer-token layer in `auth.rs` has
    /// already run by the time a handler sees this, so reaching a handler at
    /// all is the proof — there is nothing further to check.
    Local,
}

impl<S: Send + Sync> FromRequestParts<S> for Principal {
    type Rejection = Infallible;

    async fn from_request_parts(_: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        Ok(Principal::Local)
    }
}
