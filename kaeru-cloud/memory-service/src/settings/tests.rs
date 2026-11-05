use std::collections::HashMap;

use super::MemoryServiceConfig;
use config::Environment;
use kaeru_core::{KaeruConfig, config::StorageConfig};

fn load(values: &[(&str, &str)]) -> MemoryServiceConfig {
    let values = values
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<HashMap<_, _>>();
    MemoryServiceConfig::load(
        Environment::with_prefix("KAERU_CLOUD")
            .try_parsing(true)
            .source(Some(values.clone())),
        Environment::with_prefix("KAERU_MEMORY")
            .try_parsing(true)
            .source(Some(values)),
    )
    .unwrap()
}

#[test]
fn service_environment_wins_over_legacy_values() {
    let config = load(&[
        ("KAERU_CLOUD_LISTEN_PORT", "18000"),
        ("KAERU_MEMORY_LISTEN_PORT", "18001"),
        ("KAERU_CLOUD_API_TOKEN", "old"),
        ("KAERU_MEMORY_API_TOKEN", "new"),
    ]);
    assert_eq!(config.transport.listen_port, 18001);
    assert_eq!(config.runtime_options().unwrap().access.mcp_token, "new");
    assert_eq!(load(&[]).transport.listen_port, 9877);
}

#[test]
fn local_disabled_account_and_cloud_file_are_not_service_policy() {
    let home = tempfile::tempdir().unwrap();
    let path = home.path().join("web.toml");
    std::fs::write(&path, "enabled=false").unwrap();
    let path = path.to_str().unwrap();
    let config = load(&[
        ("KAERU_MCP_WEB_CONFIG_FILE", path),
        ("KAERU_MCP_CLOUDS_FILE", path),
        ("KAERU_MCP_AUTH_TOKEN", "local"),
        ("KAERU_MEMORY_API_TOKEN", "service"),
    ]);
    assert!(config.web_config_file.is_none());
    assert!(config.transport.clouds.is_empty());
    let options = config.runtime_options().unwrap();
    assert!(options.access.enabled);
    assert_eq!(options.access.mcp_token, "service");
    assert!(!options.ui_enabled);
}

#[test]
fn service_rejects_disabled_explicit_account_and_insecure_bind() {
    let config = load(&[("KAERU_MEMORY_LISTEN_ADDRESS", "0.0.0.0")]);
    assert!(config.runtime_options().is_err());
    let home = tempfile::tempdir().unwrap();
    let path = home.path().join("web.toml");
    std::fs::write(&path, "enabled=false").unwrap();
    let config = load(&[
        ("KAERU_MEMORY_WEB_CONFIG_FILE", path.to_str().unwrap()),
        ("KAERU_MEMORY_API_TOKEN", "service"),
    ]);
    assert!(config.runtime_options().is_err());
}

#[cfg(feature = "storage-tikv")]
#[test]
fn backend_default_is_host_policy_not_feature_unification() {
    let mut service = MemoryServiceConfig::storage_defaults();
    assert!(matches!(service.storage, StorageConfig::Tikv { .. }));
    assert!(matches!(
        KaeruConfig::defaults().storage,
        StorageConfig::Rocksdb
    ));
    assert!(MemoryServiceConfig::validate_storage(&service).is_err());
    if let StorageConfig::Tikv { connection } = &mut service.storage {
        connection.namespace = Some("fixture".into());
        connection.end_points = vec!["pd.example:2379".into()];
    }
    assert!(MemoryServiceConfig::validate_storage(&service).is_ok());
    service.storage = StorageConfig::Memory;
    assert!(MemoryServiceConfig::validate_storage(&service).is_ok());
}
