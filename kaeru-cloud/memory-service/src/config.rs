//! Service configuration for `kaeru-cloud`.
//!
//! `KaeruCloudConfig` covers the HTTP service itself — bind address, port,
//! bearer token, log level. Substrate tuning (vault path, caps) is the
//! separate `kaeru_core::KaeruConfig`, read from `KAERU_*` env vars, exactly
//! as the local daemon does. Defaults are merged with `KAERU_CLOUD_*` env
//! overrides through the `config` crate.

use std::net::{IpAddr, Ipv4Addr};

use config::{Config, ConfigError, Environment};
use serde::{Deserialize, Serialize};

/// Tunables for the cloud HTTP service. Env mapping:
/// `KAERU_CLOUD_<FIELD_UPPERCASE>` (e.g. `KAERU_CLOUD_LISTEN_PORT`,
/// `KAERU_CLOUD_API_TOKEN`).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KaeruCloudConfig {
    /// Address to bind. Defaults to loopback; set to `0.0.0.0` to expose.
    pub listen_address: IpAddr,
    /// Port to bind. Defaults to 9877 (the local MCP daemon uses 9876).
    pub listen_port: u16,
    /// Bearer token required on every request. Empty = auth disabled
    /// (loopback / dev only) — loudly warned about at startup.
    pub api_token: String,
    /// Tracing level: `error` | `warn` | `info` | `debug` | `trace`.
    pub log_level: String,
}

impl KaeruCloudConfig {
    /// Built-in defaults, ignoring the environment.
    pub fn defaults() -> Self {
        Self {
            listen_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            listen_port: 9877,
            api_token: String::new(),
            log_level: "info".to_string(),
        }
    }

    /// Reads `KAERU_CLOUD_*` env vars on top of [`Self::defaults`].
    pub fn from_env() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(Config::try_from(&Self::defaults())?)
            .add_source(Environment::with_prefix("KAERU_CLOUD").try_parsing(true))
            .build()?
            .try_deserialize()
    }
}

impl Default for KaeruCloudConfig {
    fn default() -> Self {
        Self::defaults()
    }
}
