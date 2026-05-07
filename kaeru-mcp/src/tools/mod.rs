//! Per-domain tool handler functions. Each `tools::<group>::<fn>`
//! takes `&Store` plus typed args and returns
//! `Result<CallToolResult, McpError>`. The `#[tool_router]` impl in
//! `server.rs` is one thin wrapper per tool that just calls into
//! these.
//!
//! Mirrors the layout of `kaeru-cli/src/commands/`.

pub mod capture;
pub mod consolidate;
pub mod hypothesis;
pub mod lint;
pub mod lookup;
pub mod metabolism;
pub mod review;
pub mod session;
pub mod task;
pub mod temporal;
pub mod vault;
