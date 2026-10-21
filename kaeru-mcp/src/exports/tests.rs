use std::fs;
use std::time::{Duration, Instant};

use base64::{Engine, engine::general_purpose::STANDARD};
use kaeru_core::models::{ArchiveFormat, Artifact};
use rmcp::model::ResourceContents;
use sha2::{Digest, Sha256};

use super::ExportResources;
use crate::settings::archive::ExportResourceLimits;

fn artifact(root: &std::path::Path, name: &str, bytes: &[u8]) -> Artifact {
    let path = root.join(name);
    fs::write(&path, bytes).unwrap();
    Artifact {
        path: path.to_string_lossy().into(),
        format: ArchiveFormat::Amg,
        bytes: bytes.len(),
        sha256: format!("{:x}", Sha256::digest(bytes)),
        nodes: 0,
        edges: 0,
        initiatives: vec![],
        omissions: vec![],
    }
}
#[test]
fn only_generated_verified_receipts_yield_downloads_and_immutable_bytes_survive_replacement() {
    let dir = tempfile::tempdir().unwrap();
    let registry = ExportResources::default();
    let a = artifact(dir.path(), "a.amg", b"original");
    let link = registry.register(&a, dir.path()).unwrap();
    assert!(registry.read("file:///etc/passwd").is_none());
    assert!(registry.read("kaeru://exports/../anything").is_none());
    fs::write(&a.path, b"replaced").unwrap();
    let ResourceContents::BlobResourceContents { blob, .. } = registry.read(&link.uri).unwrap()
    else {
        panic!()
    };
    assert_eq!(STANDARD.decode(blob).unwrap(), b"original");
    assert!(ExportResources::default().register(&a, dir.path()).is_err());
    fs::remove_file(&a.path).unwrap();
    assert!(ExportResources::default().register(&a, dir.path()).is_err());
}
#[test]
fn registry_expires_and_bounds_count_total_and_file_size_without_sleeping() {
    let dir = tempfile::tempdir().unwrap();
    let now = Instant::now();
    let limits = ExportResourceLimits {
        lifetime: Duration::from_secs(10),
        entries: 1,
        file_bytes: 4,
        total_bytes: 4,
    };
    let registry = ExportResources::new(limits);
    let a = artifact(dir.path(), "a.amg", b"1234");
    let first = registry.register_at(&a, dir.path(), now).unwrap();
    let b = artifact(dir.path(), "b.amg", b"5678");
    registry
        .register_at(&b, dir.path(), now + Duration::from_secs(1))
        .unwrap();
    assert!(
        registry
            .read_at(&first.uri, now + Duration::from_secs(1))
            .is_none()
    );
    let second = registry
        .register_at(&b, dir.path(), now + Duration::from_secs(1))
        .unwrap();
    assert!(
        registry
            .read_at(&second.uri, now + Duration::from_secs(12))
            .is_none()
    );
    let big = artifact(dir.path(), "big.amg", b"12345");
    assert!(registry.register_at(&big, dir.path(), now).is_err());
}
#[test]
fn receipt_cannot_open_an_outside_file_or_symlink() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("exports");
    fs::create_dir(&root).unwrap();
    let a = artifact(dir.path(), "outside.amg", b"outside");
    assert!(ExportResources::default().register(&a, &root).is_err());
    #[cfg(unix)]
    {
        let link = root.join("link.amg");
        std::os::unix::fs::symlink(&a.path, &link).unwrap();
        let mut fake = a;
        fake.path = link.to_string_lossy().into();
        assert!(ExportResources::default().register(&fake, &root).is_err());
    }
}
