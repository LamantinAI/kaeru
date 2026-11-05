//! Storage selection. Embedded RocksDB remains the default.

use serde::{Deserialize, Serialize};

#[cfg(feature = "storage-tikv")]
pub use cozo::{TiKvConfig, TiKvTlsConfig};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(tag = "backend", rename_all = "lowercase")]
pub enum StorageConfig {
    #[default]
    Rocksdb,
    Memory,
    /// Requires an explicit namespace; the engine enforces one active owner.
    #[cfg(feature = "storage-tikv")]
    Tikv {
        #[serde(flatten)]
        connection: TiKvConfig,
    },
}

impl StorageConfig {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Rocksdb => "rocksdb",
            Self::Memory => "mem",
            #[cfg(feature = "storage-tikv")]
            Self::Tikv { .. } => "tikv",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StorageConfig;
    use crate::KaeruConfig;
    use config::{Config, Environment};
    use std::collections::HashMap;

    #[test]
    fn legacy_environment_keys_keep_their_meaning() {
        let config: KaeruConfig = Config::builder()
            .add_source(Config::try_from(&KaeruConfig::defaults()).unwrap())
            .add_source(
                Environment::with_prefix("KAERU")
                    .prefix_separator("_")
                    .separator("__")
                    .try_parsing(true)
                    .source(Some(HashMap::from([
                        ("KAERU_VAULT_PATH".into(), "fixture-vault".into()),
                        ("KAERU_ACTIVE_WINDOW_SIZE".into(), "7".into()),
                    ]))),
            )
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap();
        assert_eq!(config.vault_path.to_str(), Some("fixture-vault"));
        assert_eq!(config.active_window_size, 7);
        assert!(matches!(config.storage, StorageConfig::Rocksdb));
    }

    #[cfg(feature = "storage-tikv")]
    #[test]
    fn nested_environment_selects_tikv_and_parses_endpoint_list() {
        let config: KaeruConfig = Config::builder()
            .add_source(Config::try_from(&KaeruConfig::defaults()).unwrap())
            .add_source(
                Environment::with_prefix("KAERU")
                    .prefix_separator("_")
                    .separator("__")
                    .try_parsing(true)
                    .list_separator(",")
                    .with_list_parse_key("storage.end_points")
                    .source(Some(HashMap::from([
                        ("KAERU_STORAGE__BACKEND".into(), "tikv".into()),
                        (
                            "KAERU_STORAGE__END_POINTS".into(),
                            "pd.example:2379,pd2.example:2379".into(),
                        ),
                        ("KAERU_STORAGE__NAMESPACE".into(), "fixture".into()),
                    ]))),
            )
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap();
        let StorageConfig::Tikv { connection } = config.storage else {
            panic!("wrong backend")
        };
        assert_eq!(connection.namespace.as_deref(), Some("fixture"));
        assert_eq!(
            connection.end_points,
            vec!["pd.example:2379", "pd2.example:2379"]
        );
    }
}
