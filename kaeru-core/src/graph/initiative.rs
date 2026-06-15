//! Initiative-level sharing policy.
//!
//! The `initiative` relation stores a sticky `share_policy` per initiative
//! name. It is Gate 1 of the local/cloud split: before any node in an
//! initiative may be promoted to the shared cloud, its initiative's policy
//! must permit it. The policy is asked once and persists — not re-asked per
//! capture.

use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;

use crate::errors::Error;

/// Sticky per-initiative sharing policy (Gate 1).
///
/// `Private` is the default: nothing from the initiative ever leaves.
/// `Team` lets `Shared`-marked nodes sync. `Ask` defers to a one-time
/// human classification and behaves as `Private` until answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SharePolicy {
    Private,
    Team,
    Ask,
}

impl SharePolicy {
    pub fn as_str(&self) -> &'static str {
        match self {
            SharePolicy::Private => "private",
            SharePolicy::Team => "team",
            SharePolicy::Ask => "ask",
        }
    }

    /// Whether this policy currently permits a `Shared` node to sync.
    /// `Ask` is treated as not-yet-permitted until it is resolved to
    /// `Team` — fail-safe, like the `Private` default.
    pub fn permits_share(&self) -> bool {
        matches!(self, SharePolicy::Team)
    }
}

impl Default for SharePolicy {
    fn default() -> Self {
        SharePolicy::Private
    }
}

impl FromStr for SharePolicy {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "private" => Ok(SharePolicy::Private),
            "team" => Ok(SharePolicy::Team),
            "ask" => Ok(SharePolicy::Ask),
            _ => Err(Error::Invalid(format!("unknown share policy: {s}"))),
        }
    }
}
