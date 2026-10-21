//! Only published export receipts can create temporary MCP download capabilities.

use std::collections::BTreeMap;
use std::fs::{File, symlink_metadata};
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use base64::{Engine, engine::general_purpose::STANDARD};
use kaeru_core::{models::Artifact, new_node_id};
use rmcp::model::{RawResource, ResourceContents};
use sha2::{Digest, Sha256};

use crate::settings::archive::{EXPORT_RESOURCE_PREFIX, ExportResourceLimits};

#[derive(Clone)]
struct Entry {
    resource: RawResource,
    bytes: Arc<[u8]>,
    path: String,
    digest: String,
    created: Instant,
}
#[derive(Clone)]
pub struct ExportResources {
    entries: Arc<Mutex<BTreeMap<String, Entry>>>,
    limits: ExportResourceLimits,
}
impl Default for ExportResources {
    fn default() -> Self {
        Self::new(ExportResourceLimits::default())
    }
}
impl ExportResources {
    fn new(limits: ExportResourceLimits) -> Self {
        Self {
            entries: Arc::new(Mutex::new(BTreeMap::new())),
            limits,
        }
    }
    fn prune(&self, entries: &mut BTreeMap<String, Entry>, now: Instant) {
        entries.retain(|_, e| now.saturating_duration_since(e.created) < self.limits.lifetime);
    }
    pub fn register(&self, artifact: &Artifact, root: &Path) -> Result<RawResource, String> {
        self.register_at(artifact, root, Instant::now())
    }
    fn register_at(
        &self,
        artifact: &Artifact,
        root: &Path,
        now: Instant,
    ) -> Result<RawResource, String> {
        if artifact.bytes > self.limits.file_bytes
            || artifact.bytes > self.limits.total_bytes
            || self.limits.entries == 0
        {
            return Err("archive exists on the server but exceeds the MCP download limit; export fewer initiatives".into());
        }
        {
            let mut entries = self.entries.lock().unwrap_or_else(|p| p.into_inner());
            self.prune(&mut entries, now);
            if let Some(entry) = entries.values().find(|e| {
                e.path == artifact.path
                    && e.digest == artifact.sha256
                    && e.bytes.len() == artifact.bytes
            }) {
                return Ok(entry.resource.clone());
            }
        }
        // Never interpret a URI or client path as filesystem access. The receipt
        // supplies a generated artifact's expected path, exact size and digest.
        let path = Path::new(&artifact.path);
        let metadata =
            symlink_metadata(path).map_err(|_| "exported file is no longer available")?;
        if !metadata.file_type().is_file() || metadata.len() != artifact.bytes as u64 {
            return Err("exported file no longer matches its receipt".into());
        }
        let root = root
            .canonicalize()
            .map_err(|_| "export artifact root is unavailable")?;
        let actual = path
            .canonicalize()
            .map_err(|_| "exported file is unavailable")?;
        if !actual.starts_with(&root) {
            return Err("exported file is outside its authorized artifact root".into());
        }
        let mut bytes = Vec::with_capacity(artifact.bytes);
        File::open(&actual)
            .map_err(|_| "exported file is unavailable")?
            .take(artifact.bytes as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| "exported file cannot be read")?;
        if bytes.len() != artifact.bytes
            || format!("{:x}", Sha256::digest(&bytes)) != artifact.sha256
        {
            return Err("exported file no longer matches its receipt".into());
        }
        let mut resource = RawResource::new(
            format!("{EXPORT_RESOURCE_PREFIX}{}", new_node_id()),
            actual.file_name().unwrap_or_default().to_string_lossy(),
        );
        resource.mime_type = Some(artifact.format.media_type().into());
        resource.size = Some(artifact.bytes as u32);
        resource.description=Some("Generated knowledge archive; download expires after 15 minutes or daemon restart. Read with resources/read.".into());
        let entry = Entry {
            resource: resource.clone(),
            bytes: bytes.into(),
            path: artifact.path.clone(),
            digest: artifact.sha256.clone(),
            created: now,
        };
        let mut entries = self.entries.lock().unwrap_or_else(|p| p.into_inner());
        self.prune(&mut entries, now);
        while entries.len() >= self.limits.entries
            || entries
                .values()
                .map(|e| e.bytes.len())
                .sum::<usize>()
                .saturating_add(artifact.bytes)
                > self.limits.total_bytes
        {
            let Some(old) = entries
                .iter()
                .min_by_key(|(_, e)| e.created)
                .map(|(key, _)| key.clone())
            else {
                break;
            };
            entries.remove(&old);
        }
        entries.insert(resource.uri.clone(), entry);
        Ok(resource)
    }
    pub fn read(&self, uri: &str) -> Option<ResourceContents> {
        self.read_at(uri, Instant::now())
    }
    fn read_at(&self, uri: &str, now: Instant) -> Option<ResourceContents> {
        let entry = {
            let mut entries = self.entries.lock().unwrap_or_else(|p| p.into_inner());
            self.prune(&mut entries, now);
            entries.get(uri).cloned()
        }?;
        Some(
            ResourceContents::blob(STANDARD.encode(&entry.bytes), uri)
                .with_mime_type(entry.resource.mime_type.unwrap_or_default()),
        )
    }
}

#[cfg(test)]
mod tests;
