//! Local read-delivery journal. Content and request arguments never enter it.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReadTraceEvent {
    pub operation: String,
    pub node_ids: Vec<String>,
    pub initiatives: Vec<String>,
    pub timestamp: DateTime<Utc>,
}
