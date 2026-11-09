//! Bounds for the isolated authorization integration adapter.
use std::time::Duration;
pub const AUTHORIZE_TIMEOUT: Duration = Duration::from_secs(5);
pub const MAX_DECISION_BYTES: usize = 16 * 1024;
pub const MAX_READ_REQUEST_BYTES: usize = 64 * 1024;
pub const MAX_CREDENTIAL_BYTES: usize = 4096;
