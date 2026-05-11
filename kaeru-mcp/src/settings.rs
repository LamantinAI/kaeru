//! Runtime configuration for `kaeru-mcp`.
//!
//! Layered the same way as `kaeru-core::config::KaeruConfig` and
//! `widehabit`'s `WideConfig`: built-in defaults, then `KAERU_MCP_*`
//! environment variables on top through the `config` crate. No
//! hardcoded constants leak into the rest of the binary — every
//! tunable is a field with a default function here.

use std::net::Ipv4Addr;

use config::Config;
use config::ConfigError;
use config::Environment;
use serde::Deserialize;

/// Tunables for the kaeru-mcp daemon. All fields are env-overridable
/// via `KAERU_MCP_<FIELD_UPPERCASE>` (e.g. `KAERU_MCP_LISTEN_PORT`).
#[derive(Clone, Debug, Deserialize)]
pub struct KaeruMcpConfig {
    /// IPv4 the HTTP listener binds to. Defaults to loopback so the
    /// daemon is local-only out of the box; flip to `0.0.0.0` to
    /// expose on a LAN (and add auth — there is none right now).
    #[serde(default = "default_listen_address")]
    pub listen_address: Ipv4Addr,

    /// TCP port for the HTTP listener.
    #[serde(default = "default_listen_port")]
    pub listen_port: u16,

    /// Axum mount path for the MCP service. Must start with `/`. The
    /// full URL clients connect to is
    /// `http://<listen_address>:<listen_port><mount_path>`.
    #[serde(default = "default_mount_path")]
    pub mount_path: String,

    /// Tracing log level (`error`, `warn`, `info`, `debug`, `trace`).
    /// Logs go to stderr only because stdout is reserved when MCP
    /// servers run over stdio elsewhere — kaeru-mcp itself uses HTTP,
    /// but keeping the convention prevents surprises if anyone ever
    /// pipes the binary somewhere.
    #[serde(default = "default_log_level")]
    pub log_level: String,

    /// Idle timeout for rmcp MCP sessions, in seconds. After this
    /// many seconds without activity the session manager drops the
    /// session (rmcp's own default behaviour is 300s). Editor-attached
    /// clients like Claude Code sit idle between user prompts, so we
    /// disable reaping by default — set to non-zero to restore it
    /// (useful for proxy-style deployments where hung clients should
    /// free server-side state).
    #[serde(default)]
    pub keep_alive_secs: u64,
}

impl KaeruMcpConfig {
    /// Builds a config from `KAERU_MCP_*` environment variables on top
    /// of defaults. Numeric and IP fields are parsed automatically via
    /// `try_parsing(true)`.
    pub fn new() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(
                Environment::with_prefix("KAERU_MCP")
                    .try_parsing(true),
            )
            .build()?
            .try_deserialize()
    }
}

fn default_listen_address() -> Ipv4Addr {
    Ipv4Addr::LOCALHOST
}

fn default_listen_port() -> u16 {
    9876
}

fn default_mount_path() -> String {
    "/mcp".to_string()
}

fn default_log_level() -> String {
    "info".to_string()
}
