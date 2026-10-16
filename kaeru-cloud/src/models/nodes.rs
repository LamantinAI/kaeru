//! Cloud nodes exchange contracts.

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use utoipa::ToSchema;

/// A node being pushed up from a local vault. `id` is the local node's
/// UUIDv7, preserved verbatim so soft links resolve.
#[derive(Debug, Deserialize, ToSchema)]
pub struct NodeIngestReq {
    pub id: String,
    pub node_type: String,
    pub tier: String,
    pub name: String,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    /// Initiative this node belongs to — the shared scope on both sides.
    #[serde(default)]
    #[schema(required = true, value_type = String)]
    pub initiative: Option<String>,
    /// Memory layer (`core`/`hot`/`warm`/`cold`/`frozen`). Preserved across
    /// the cloud so recall priority survives share/pull. Defaults to `warm`.
    #[serde(default)]
    pub layer: Option<String>,
    /// The node's `properties` JSON — a `reference`'s URL, a `board`'s status
    /// registry. Omitted by an older client, in which case whatever the cloud
    /// already holds is kept rather than cleared (#85).
    #[serde(default)]
    #[schema(value_type = Option<Object>)]
    pub properties: Option<JsonValue>,
}

/// Full node view returned to the caller — the **untruncated** body and
/// tier/tags, so a puller can materialise the node locally verbatim.
#[derive(Debug, Serialize, ToSchema)]
pub struct NodeView {
    pub id: String,
    pub node_type: String,
    pub tier: String,
    pub name: String,
    pub body: Option<String>,
    pub tags: Vec<String>,
    pub visibility: String,
    pub layer: String,
    /// Always present, `null` when the node has none — a puller needs to be
    /// able to tell "no properties" from "this cloud is too old to send them".
    #[schema(value_type = Option<Object>)]
    pub properties: Option<JsonValue>,
}
