//! HTTP layer: state, extractors, error responses, and the router.
//!
//! Layering mirrors the widehabit backend: handlers/routers under
//! `router/`, shared state in `state.rs`, request extractors in
//! `extractors.rs`, and the `IntoResponse` mapping in `errors.rs`. The
//! "service / domain layer" the handlers delegate to is simply
//! `kaeru-core` — there is no separate persistence layer in this crate.

pub mod errors;
pub mod extractors;
pub mod router;
pub mod state;
