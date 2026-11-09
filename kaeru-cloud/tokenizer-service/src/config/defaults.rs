use std::time::Duration;

pub const LISTEN: &str = "127.0.0.1:9880";
pub const MAX_CONNECTIONS: u32 = 16;
pub const MAX_BODY_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_PROJECTION_RECORDS: usize = 10_000;
pub const MAX_BATCH: usize = 128;
pub const MAX_TOKEN_NAME: usize = 256;
pub const DEFAULT_TOKEN_MONTHS: u32 = 6;
pub const DB_ACQUIRE_TIMEOUT: Duration = Duration::from_secs(5);
pub const DB_STATEMENT_TIMEOUT: &str = "5s";
pub const CACHE_TIMEOUT: Duration = Duration::from_millis(100);
pub const CACHE_TTL: Duration = Duration::from_secs(60);
pub const CACHE_MAX_BYTES: usize = 2 * 1024 * 1024;
pub const CBOR_NESTING: usize = 32;
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);
pub const DEFAULT_PAGE_SIZE: usize = 50;
