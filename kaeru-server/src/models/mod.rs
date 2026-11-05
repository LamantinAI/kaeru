//! MCP adapter request models.

mod cloud;

pub use cloud::{
    CloudLinksParams, CloudRecallParams, CloudScopeParams, LinkCloudParams, PullParams,
    ShareParams, SyncReviewParams, UnshareParams,
};

pub mod auth;
