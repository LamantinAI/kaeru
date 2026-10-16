//! Host-supplied access capabilities; never model arguments.

use std::path::PathBuf;

/// Host-owned capabilities, never accepted as model arguments.
#[derive(Clone, Debug, Default)]
pub struct ApiOptions {
    /// Reads and exports of files must stay within this configured root.
    pub artifact_root: Option<PathBuf>,
    /// None means the trusted local owner. HTTP adapters supply their ceiling.
    pub allowed_initiatives: Option<Vec<String>>,
    pub denied_initiatives: Vec<String>,
    pub include_local: bool,
    /// Identity used for persistent replay records, supplied by the adapter.
    pub caller: String,
}

impl ApiOptions {
    pub fn local() -> Self {
        Self {
            include_local: true,
            caller: "local-owner".into(),
            ..Self::default()
        }
    }
}
