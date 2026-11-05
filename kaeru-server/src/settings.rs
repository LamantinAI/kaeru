//! Transport configuration and defaults shared by explicit host profiles.
//! Local environment/file discovery lives in kaeru-local; service policy
//! lives in memory-service. The runtime receives resolved values.

use std::net::{IpAddr, Ipv4Addr};

use config::{Config, ConfigError};
use serde::Deserialize;

/// Product identity advertised during MCP initialization.
/// Protocol ceiling retained from Kaeru 0.8.0; SDK updates must not opt into new wire semantics.
pub const MCP_PROTOCOL_VERSION: rmcp::model::ProtocolVersion =
    rmcp::model::ProtocolVersion::V_2025_11_25;
pub const MCP_SESSION_HEADER: &str = "mcp-session-id";
pub const MCP_PROTOCOL_HEADER: &str = "mcp-protocol-version";
pub const SESSION_RETRY_AFTER_SECS: u64 = 60;
pub const STDIO_SESSION_CLOSE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(2);
pub const SESSION_REAP_INTERVAL: std::time::Duration = std::time::Duration::from_secs(5);

pub const MCP_SERVER_NAME: &str = "kaeru";
pub const MCP_SERVER_TITLE: &str = "kaeru memory";
pub const MCP_SERVER_WEBSITE_URL: &str = "https://lamantin-ai.com/products/kaeru/";

/// One named cloud endpoint: a `kaeru-cloud` base URL plus its bearer token.
/// Deserialized from a `[clouds.<name>]` section of the clouds TOML file.
#[derive(Clone, Debug, Deserialize)]
pub struct CloudEndpoint {
    /// Base URL of the memory-service compatibility API (e.g. `https://team.example/`).
    pub url: String,
    /// Bearer token matching that cloud's `KAERU_CLOUD_API_TOKEN`.
    #[serde(default)]
    pub token: String,
}

/// Tunables for the kaeru-mcp daemon. All fields are env-overridable
/// via `KAERU_MCP_<FIELD_UPPERCASE>` (e.g. `KAERU_MCP_LISTEN_PORT`).
#[derive(Clone, Debug, Deserialize)]
pub struct KaeruMcpConfig {
    /// IPv4 the HTTP listener binds to. Defaults to loopback so the
    /// daemon is local-only out of the box; flip to `0.0.0.0` to
    /// expose on a LAN — and set `auth_token` when you do, since a
    /// routable port is otherwise open curator access to the vault.
    #[serde(default = "default_listen_address")]
    pub listen_address: IpAddr,

    /// TCP port for the HTTP listener.
    #[serde(default = "default_listen_port")]
    pub listen_port: u16,

    /// Axum mount path for the streamable HTTP MCP transport. Must
    /// start with `/`. The full URL clients connect to is
    /// `http://<listen_address>:<listen_port><mount_path>`. This is the
    /// canonical, spec-current endpoint that Claude Code and any
    /// streamable-HTTP-aware client should use.
    #[serde(default = "default_mount_path")]
    pub mount_path: String,

    /// Axum mount path for the legacy HTTP+SSE transport — the
    /// `text/event-stream` endpoint a client opens with GET. Defaults
    /// to `/sse`. Paired with `messages_path`. Kept on by default so
    /// older / lagging clients (e.g. Opencode 1.15.x, see
    /// opencode-ai/opencode#8058) work out of the box.
    #[serde(default = "default_sse_path")]
    pub sse_path: String,

    /// Axum mount path for the legacy HTTP+SSE POST endpoint — the URI
    /// the SSE `endpoint` event tells the client to POST JSON-RPC
    /// requests to. Defaults to `/messages`.
    #[serde(default = "default_messages_path")]
    pub messages_path: String,

    /// Extra `Host` header authorities to accept, on top of the
    /// loopback set (`localhost`, `127.0.0.1`, `::1`) that rmcp's
    /// Streamable HTTP transport allows by default as a DNS-rebinding
    /// guard. When binding to a routable address (`0.0.0.0`) you MUST
    /// list the host(s) clients put in their `Host` header here —
    /// otherwise rmcp answers `403 Forbidden: Host header is not
    /// allowed` and clients like Claude Code surface it as
    /// "Needs authentication". Comma-separated, with or without port,
    /// e.g. `KAERU_MCP_ALLOWED_HOSTS=192.0.2.10:9876,kaeru.lan`.
    /// Empty (default) keeps the loopback-only behaviour.
    #[serde(default = "default_allowed_hosts")]
    pub allowed_hosts: String,

    /// Shared secret required on every inbound MCP request. When
    /// non-empty, clients must send `Authorization: Bearer <token>` and
    /// the middleware in `auth/` rejects anything else with `401`;
    /// the check covers both the streamable HTTP (`/mcp`) and legacy SSE
    /// (`/sse`, `/messages`) transports. When empty, a configured account's
    /// persisted `mcp_token` is used. An explicit `enabled=false` in the local
    /// web configuration disables authentication even if an old token remains.
    /// With no configuration and no token, authentication is disabled.
    /// Configure a client with e.g.
    /// `claude mcp add --transport http --header "Authorization: Bearer <token>" kaeru <url>`.
    /// Env: `KAERU_MCP_AUTH_TOKEN`.
    #[serde(default = "default_auth_token")]
    pub auth_token: String,

    /// Tracing log level (`error`, `warn`, `info`, `debug`, `trace`).
    /// Logs go to stderr only because stdout is reserved when MCP
    /// servers run over stdio elsewhere — kaeru-mcp itself uses HTTP,
    /// but keeping the convention prevents surprises if anyone ever
    /// pipes the binary somewhere.
    #[serde(default = "default_log_level")]
    pub log_level: String,

    /// Base URL of the shared memory-service compatibility API this daemon proxies to
    /// for sharing / recall (e.g. `http://127.0.0.1:9877`). Empty (default)
    /// disables cloud tools — `share` / `pull` / `cloud_recall` then report
    /// that the cloud is not configured. Env: `KAERU_MCP_CLOUD_URL`.
    #[serde(default = "default_cloud_url")]
    pub cloud_url: String,

    /// Bearer token sent to `kaeru-cloud` on every request. Must match the
    /// cloud's `KAERU_CLOUD_API_TOKEN`. Env: `KAERU_MCP_CLOUD_TOKEN`.
    #[serde(default = "default_cloud_token")]
    pub cloud_token: String,

    /// Named clouds for multi-cloud setups, loaded from the clouds TOML file
    /// (default `$XDG_CONFIG_HOME/kaeru/clouds.toml`, override via
    /// `KAERU_MCP_CLOUDS_FILE`). Each is a `[clouds.<name>]` section with
    /// `url` + `token`. The legacy single `cloud_url`/`cloud_token` pair, if
    /// set, is folded in as an additional named cloud at startup. Empty (the
    /// default) means single-cloud / no-cloud as before.
    #[serde(default)]
    pub clouds: std::collections::HashMap<String, CloudEndpoint>,

    /// Name of the cloud used when a tool is called without an explicit
    /// `cloud` argument. From the `default` key of the clouds TOML. When
    /// unset and exactly one cloud is configured, that one is the default.
    #[serde(default, rename = "default")]
    pub default_cloud: String,

    /// Idle timeout for managed MCP sessions, in seconds. Defaults to 300;
    /// zero disables expiry. Pending operations and response delivery are
    /// protected, then start a new full idle interval on completion.
    #[serde(default = "default_keep_alive_secs")]
    pub keep_alive_secs: u64,

    /// Maximum admitted, initializing or closing streamable-HTTP sessions.
    /// Existing sessions remain usable when new admission is full.
    #[serde(default = "default_max_sessions")]
    pub max_sessions: usize,

    /// Optional POST body limit for the SDK; zero preserves the previous
    /// unlimited transport behavior (including large reference captures).
    #[serde(default)]
    pub max_request_body_bytes: usize,

    /// Opt-in HTTP/session diagnostic sampling interval; zero disables it.
    /// Request logs contain timings and session fingerprints, never payloads
    /// or authorization headers. Env: `KAERU_MCP_DIAGNOSTICS_INTERVAL_SECS`.
    #[serde(default)]
    pub diagnostics_interval_secs: u64,
}

impl KaeruMcpConfig {
    /// Transport defaults without reading environment or local files.
    pub fn defaults() -> Self {
        Config::default()
            .try_deserialize()
            .expect("transport defaults are valid")
    }

    pub fn validate(&self, prefix: &str) -> Result<(), ConfigError> {
        if self.max_sessions == 0 || self.max_sessions > tokio::sync::Semaphore::MAX_PERMITS {
            return Err(ConfigError::Message(format!(
                "{prefix}_MAX_SESSIONS must be positive and within semaphore capacity"
            )));
        }
        Ok(())
    }
}

fn default_listen_address() -> IpAddr {
    IpAddr::V4(Ipv4Addr::LOCALHOST)
}

fn default_max_sessions() -> usize {
    256
}

fn default_keep_alive_secs() -> u64 {
    300
}

fn default_listen_port() -> u16 {
    9876
}

fn default_mount_path() -> String {
    "/mcp".to_string()
}

fn default_sse_path() -> String {
    "/sse".to_string()
}

fn default_messages_path() -> String {
    "/messages".to_string()
}

fn default_allowed_hosts() -> String {
    String::new()
}

fn default_auth_token() -> String {
    String::new()
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_cloud_url() -> String {
    String::new()
}

fn default_cloud_token() -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::KaeruMcpConfig;

    #[test]
    fn session_expiry_defaults_to_five_minutes_and_zero_disables_it() {
        let defaults: KaeruMcpConfig = serde_json::from_value(serde_json::json!({})).unwrap();
        assert_eq!(defaults.keep_alive_secs, 300);
        let disabled: KaeruMcpConfig =
            serde_json::from_value(serde_json::json!({"keep_alive_secs":0})).unwrap();
        assert_eq!(disabled.keep_alive_secs, 0);
    }
}

pub mod archive;
pub mod web;

pub mod runtime;
pub use runtime::RuntimeOptions;
