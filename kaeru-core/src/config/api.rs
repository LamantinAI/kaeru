//! Stable protocol identifiers and bounded-read defaults.

use std::time::Duration;

pub const API_VERSION: u32 = 1;
pub const API_OPERATION_PREFIX: &str = "/v1/";
pub const API_EXCERPT_CHARS: usize = 400;
pub const API_BODY_CHUNK_CHARS: usize = 256;
pub const API_TRAVERSAL_NODE_LIMIT: usize = 5_000;
pub const API_SEARCH_CANDIDATE_LIMIT: usize = 5_000;
pub const API_DEFAULT_SEARCH_LIMIT: usize = 10;
pub const API_DEFAULT_READ_BUDGET_CHARS: usize = 12_000;
pub const API_DEFAULT_CONTEXT_BUDGET_CHARS: usize = 8_000;
pub const API_MAX_SNAPSHOT_BYTES: usize = 8 * 1024 * 1024;
pub const API_MAX_CACHED_SNAPSHOTS: usize = 16;
pub const API_SNAPSHOT_TTL: Duration = Duration::from_secs(15 * 60);
pub const API_REPLAY_TTL_SECONDS: u64 = 24 * 60 * 60;
