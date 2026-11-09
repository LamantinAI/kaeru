//! Bounded synchronous database execution defaults.
use std::time::Duration;
pub const DEFAULT_READ_WORKERS: usize = 4;
pub const MAX_READ_WORKERS: usize = 64;
pub const READ_WORKER_ACQUIRE_TIMEOUT: Duration = Duration::from_secs(5);
pub const fn read_workers() -> usize {
    DEFAULT_READ_WORKERS
}
