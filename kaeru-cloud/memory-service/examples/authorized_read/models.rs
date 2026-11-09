//! Lab-only mapping; a production Control registry is intentionally absent.
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mapping {
    pub revision: Uuid,
    pub initiatives: BTreeMap<String, Uuid>,
}
impl Mapping {
    pub fn validate(&self) -> Result<(), &'static str> {
        let mut ids = std::collections::BTreeSet::new();
        if self.revision.is_nil() || self.initiatives.is_empty() || self.initiatives.len() > 128 {
            return Err("invalid mapping size or revision");
        }
        for (name, id) in &self.initiatives {
            if name.trim() != name
                || name.is_empty()
                || name.contains(['*', '?', '[', ']', '{', '}', '\\'])
                || id.is_nil()
                || !ids.insert(id)
            {
                return Err("mapping requires unique exact names and UUIDs");
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Prepared {
    pub mapping: Mapping,
    pub principal_id: Uuid,
    pub token_id: Uuid,
    pub credential: contracts::SecretToken,
    pub second_credential: contracts::SecretToken,
    pub control_credential: contracts::SecretToken,
}

/// The serving process needs the mapping only, never issued product secrets.
#[derive(Deserialize)]
pub struct MappingDocument {
    pub mapping: Mapping,
}
