//! Read-journal defaults, independent of memory retention layers.

use std::env::var_os;
use std::path::PathBuf;

pub const RAW_TTL_SECS: u64 = 72 * 60 * 60;
pub const RETENTION_SECS: u64 = 14 * 24 * 60 * 60;
pub const QUEUE_CAPACITY: usize = 256;
pub const SEGMENT_BYTES: u64 = 4 * 1024 * 1024;
pub const SEGMENT_SECS: u64 = 60 * 60;
pub const MAINTENANCE_SECS: u64 = 60;
pub const MAX_QUEUE_CAPACITY: usize = 65_536;
pub const MAX_SEGMENT_BYTES: u64 = 64 * 1024 * 1024;

pub fn default_directory() -> PathBuf {
    #[cfg(target_os = "linux")]
    let base = var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .unwrap_or_else(|| {
            PathBuf::from(var_os("HOME").unwrap_or_else(|| ".".into())).join(".local/state")
        });
    #[cfg(target_os = "macos")]
    let base = PathBuf::from(var_os("HOME").unwrap_or_else(|| ".".into())).join("Library/Logs");
    #[cfg(target_os = "windows")]
    let base = PathBuf::from(var_os("LOCALAPPDATA").unwrap_or_else(|| ".".into()));
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    let base = PathBuf::from(".kaeru-state");
    base.join("kaeru/read-trace")
}
