//! Bounds for portable graph documents, independent of agent response budgets.

pub const SCHEMA_VERSION: u32 = 1;
pub const MAGIC: &[u8; 4] = b"amg:";
pub const PROFILE: &str = "knowledge-current";
pub const MAX_ORIGINS: usize = 64;

#[derive(Clone, Debug)]
pub struct AmgLimits {
    pub file_bytes: usize,
    pub metadata_bytes: usize,
    pub nodes: usize,
    pub edges: usize,
    pub auxiliary_records: usize,
    pub nesting: usize,
    pub cbor_items: usize,
}

impl Default for AmgLimits {
    fn default() -> Self {
        Self {
            file_bytes: 64 * 1024 * 1024,
            metadata_bytes: 64 * 1024,
            nodes: 100_000,
            edges: 500_000,
            auxiliary_records: 500_000,
            nesting: 64,
            cbor_items: 2_000_000,
        }
    }
}
