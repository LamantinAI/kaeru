//! MCP adapter request models.

mod api;
mod cloud;
pub use api::AuthorizedRead;

pub use cloud::{
    CloudLinksParams, CloudRecallParams, CloudScopeParams, LinkCloudParams, PullParams,
    ShareParams, SyncReviewParams, UnshareParams,
};

pub mod auth;
