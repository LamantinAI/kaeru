//! Agent-facing archive format and artifact metadata.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ArchiveFormat {
    Markdown,
    Amg,
}

impl ArchiveFormat {
    pub fn extension(self) -> &'static str {
        match self {
            Self::Markdown => "zip",
            Self::Amg => "amg",
        }
    }
    pub fn media_type(self) -> &'static str {
        match self {
            Self::Markdown => "application/zip",
            Self::Amg => "application/octet-stream",
        }
    }
}
