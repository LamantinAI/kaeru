//! Service-owned configuration. Local home files are never discovered here.

use std::path::PathBuf;

use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[cfg(feature = "storage-tikv")]
use kaeru_core::config::storage::TiKvConfig;
use kaeru_core::{KaeruConfig, config::StorageConfig};
use kaeru_server::{
    ApiConfig, RuntimeOptions,
    auth::store::AccountStore,
    settings::{KaeruMcpConfig, web::AccessMode},
};

use crate::errors::StartError;

#[derive(Clone, Deserialize)]
pub struct MemoryServiceConfig {
    #[serde(flatten)]
    pub transport: KaeruMcpConfig,
    #[serde(default)]
    pub api_token: String,
    #[serde(default)]
    pub web_config_file: Option<PathBuf>,
    #[serde(default = "enabled")]
    pub ui_enable: bool,
    #[serde(default = "enabled")]
    pub api_enable: bool,
    #[serde(default = "enabled")]
    pub hygiene_enable: bool,
    #[serde(default)]
    pub api_allow: String,
    #[serde(default)]
    pub api_deny: String,
    #[serde(default)]
    pub api_include_local: bool,
    #[serde(default)]
    pub api_allow_origin: Option<String>,
}

fn enabled() -> bool {
    true
}
fn csv(text: &str) -> Vec<String> {
    text.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

impl MemoryServiceConfig {
    fn load(legacy: Environment, current: Environment) -> Result<Self, ConfigError> {
        let result: Self = Config::builder()
            .set_default("listen_port", 9877)?
            .add_source(legacy)
            .add_source(current)
            .build()?
            .try_deserialize()?;
        result.transport.validate("KAERU_MEMORY")?;
        Ok(result)
    }

    pub fn from_env() -> Result<Self, ConfigError> {
        Self::load(
            Environment::with_prefix("KAERU_CLOUD").try_parsing(true),
            Environment::with_prefix("KAERU_MEMORY").try_parsing(true),
        )
    }

    /// Runtime defaults belong to this host, independent of Cargo feature unification.
    #[cfg(feature = "storage-tikv")]
    pub fn storage_defaults() -> KaeruConfig {
        KaeruConfig {
            storage: StorageConfig::Tikv {
                connection: TiKvConfig::default(),
            },
            ..KaeruConfig::defaults()
        }
    }

    #[cfg(feature = "storage-tikv")]
    pub fn storage_from_env() -> kaeru_core::Result<KaeruConfig> {
        let config = KaeruConfig::from_env_with_defaults(Self::storage_defaults())?;
        Self::validate_storage(&config)?;
        Ok(config)
    }

    #[cfg(feature = "storage-tikv")]
    fn validate_storage(config: &KaeruConfig) -> kaeru_core::Result<()> {
        if let StorageConfig::Tikv { connection } = &config.storage {
            if connection
                .namespace
                .as_ref()
                .is_none_or(|s| s.trim().is_empty())
            {
                return Err(kaeru_core::Error::Invalid(
                    "KAERU_STORAGE__NAMESPACE is required for memory-service TiKV storage".into(),
                ));
            }
            if connection.end_points.is_empty()
                || connection.end_points.iter().any(|s| s.trim().is_empty())
            {
                return Err(kaeru_core::Error::Invalid(
                    "KAERU_STORAGE__END_POINTS must list PD endpoints for memory-service".into(),
                ));
            }
        }
        Ok(())
    }

    pub fn runtime_options(&self) -> Result<RuntimeOptions, StartError> {
        if !self.api_token.trim().is_empty()
            && !self.transport.auth_token.trim().is_empty()
            && self.api_token.trim() != self.transport.auth_token.trim()
        {
            return Err(ConfigError::Message(
                "conflicting API_TOKEN and AUTH_TOKEN; use one service bearer token".into(),
            )
            .into());
        }
        let account = self
            .web_config_file
            .as_ref()
            .map(|path| AccountStore::open(path.clone()))
            .transpose()?
            .flatten();
        if self.web_config_file.is_some() && account.is_none() {
            return Err(StartError::Account(
                "explicit instance account file does not exist".into(),
            ));
        }
        let web = account.as_ref().map(AccountStore::snapshot);
        if web.as_ref().is_some_and(|w| !w.enabled) {
            return Err(StartError::Account(
                "a disabled local account file cannot set memory-service access policy".into(),
            ));
        }
        let token = if !self.api_token.trim().is_empty() {
            self.api_token.trim().to_string()
        } else if !self.transport.auth_token.trim().is_empty() {
            self.transport.auth_token.trim().to_string()
        } else {
            web.as_ref()
                .map(|w| w.mcp_token.clone())
                .unwrap_or_default()
        };
        if token.is_empty() && !self.transport.listen_address.is_loopback() {
            return Err(StartError::InsecureBind(self.transport.listen_address));
        }
        let authenticated = !token.is_empty() || web.is_some();
        Ok(RuntimeOptions {
            access: AccessMode {
                enabled: authenticated,
                mcp_token: token,
            },
            ui_enabled: self.ui_enable && (!authenticated || web.is_some()),
            account,
            api_enabled: self.api_enable,
            api_config: ApiConfig {
                allow: csv(&self.api_allow),
                deny: csv(&self.api_deny),
                include_local: self.api_include_local,
                allow_origin: self.api_allow_origin.clone(),
            },
            hygiene_enabled: self.hygiene_enable,
            update_check_enabled: false,
        })
    }
}

#[cfg(test)]
mod tests;
