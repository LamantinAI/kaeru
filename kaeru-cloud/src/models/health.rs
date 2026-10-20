//! Cloud health exchange contracts.

use serde::Serialize;
use utoipa::ToSchema;

/// What `/health` answers. A local daemon reads `core_version` to warn when it
/// and the cloud run different kaeru-core versions.
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthView {
    /// Always `"ok"` when the service answers at all.
    pub status: String,
    /// Always `"kaeru-cloud"`.
    pub service: String,
    /// The kaeru-core version this cloud was built from.
    pub core_version: String,
}
