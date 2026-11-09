use std::fmt::{Debug, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};

/// A credential may cross the wire, but must not appear in debug output.
#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SecretToken(String);

impl SecretToken {
    pub fn new(value: String) -> Self {
        Self(value)
    }
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl Debug for SecretToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("[REDACTED]")
    }
}
