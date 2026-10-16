//! References, evidence and source inputs for the memory API.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NodeRef {
    NameOrId(String),
    Id(IdRef),
    Scoped(ScopedRef),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IdRef {
    pub id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScopedRef {
    pub name: String,
    pub initiative: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Evidence {
    Existing(ExistingEvidence),
    Check(CheckEvidence),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExistingEvidence {
    pub node: NodeRef,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckEvidence {
    pub method: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Source {
    Text(TextSource),
    Artifact(ArtifactSource),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextSource {
    pub text: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactSource {
    pub artifact: String,
}
