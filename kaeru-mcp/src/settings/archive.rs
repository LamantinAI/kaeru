//! Bounded, temporary MCP archive downloads; files remain on disk independently.

use std::time::Duration;

#[derive(Clone, Copy)]
pub struct ExportResourceLimits {
    pub lifetime: Duration,
    pub entries: usize,
    pub file_bytes: usize,
    pub total_bytes: usize,
}
impl Default for ExportResourceLimits {
    fn default() -> Self {
        Self {
            lifetime: Duration::from_secs(15 * 60),
            entries: 32,
            file_bytes: 32 * 1024 * 1024,
            total_bytes: 128 * 1024 * 1024,
        }
    }
}
pub const EXPORT_RESOURCE_PREFIX: &str = "kaeru://exports/";
