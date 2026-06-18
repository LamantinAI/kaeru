//! Runtime configuration for `kaeru-mcp`.
//!
//! Layered the same way as `kaeru-core::config::KaeruConfig` and
//! `widehabit`'s `WideConfig`: built-in defaults, then `KAERU_MCP_*`
//! environment variables on top through the `config` crate. No
//! hardcoded constants leak into the rest of the binary — every
//! tunable is a field with a default function here.

use std::net::Ipv4Addr;
use std::path::PathBuf;

use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;

/// One named cloud endpoint: a `kaeru-cloud` base URL plus its bearer token.
/// Deserialized from a `[clouds.<name>]` section of the clouds TOML file.
#[derive(Clone, Debug, Deserialize)]
pub struct CloudEndpoint {
    /// Base URL of the `kaeru-cloud` service (e.g. `https://team.example/`).
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
    pub listen_address: Ipv4Addr,

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
    /// the middleware in `auth.rs` rejects anything else with `401`;
    /// the check covers both the streamable HTTP (`/mcp`) and legacy SSE
    /// (`/sse`, `/messages`) transports. Empty (default) disables auth
    /// entirely — acceptable for the loopback-only default bind, but you
    /// should set this whenever `listen_address` is routable. Configure
    /// a client with e.g.
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

    /// Base URL of the shared `kaeru-cloud` service this daemon proxies to
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
    /// Builds a config from the clouds TOML file (if present) overlaid with
    /// `KAERU_MCP_*` environment variables. The file supplies `clouds` /
    /// `default`; env supplies scalar tunables and the legacy
    /// `cloud_url`/`cloud_token`. Numeric and IP fields parse automatically
    /// via `try_parsing(true)`.
    pub fn new() -> Result<Self, ConfigError> {
        let mut builder = Config::builder();
        if let Some(path) = clouds_file_path() {
            // `required(false)`: a missing file is the common single-cloud
            // case, not an error.
            builder = builder.add_source(File::from(path).format(FileFormat::Toml).required(false));
        }
        builder
            .add_source(Environment::with_prefix("KAERU_MCP").try_parsing(true))
            .build()?
            .try_deserialize()
    }
}

/// Resolves the clouds TOML file path: `KAERU_MCP_CLOUDS_FILE` when set,
/// else `$XDG_CONFIG_HOME/kaeru/clouds.toml` (fallback
/// `$HOME/.config/kaeru/clouds.toml`). Returns `None` when no home/config
/// dir can be resolved (then only env config applies).
fn clouds_file_path() -> Option<PathBuf> {
    if let Ok(explicit) = std::env::var("KAERU_MCP_CLOUDS_FILE")
        && !explicit.is_empty()
    {
        return Some(PathBuf::from(explicit));
    }
    let base = std::env::var("XDG_CONFIG_HOME")
        .ok()
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var("HOME")
                .ok()
                .filter(|s| !s.is_empty())
                .map(|h| PathBuf::from(h).join(".config"))
        })?;
    Some(base.join("kaeru").join("clouds.toml"))
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
    use std::fs;
    use std::io::Write;

    use super::KaeruMcpConfig;

    /// A clouds TOML file is parsed into the `clouds` map and `default`,
    /// proving the config-crate File source is wired correctly.
    #[test]
    fn clouds_toml_loads_named_endpoints() {
        // Unique temp path; point the loader at it via the override env var.
        let dir = std::env::temp_dir();
        let path = dir.join(format!("kaeru-clouds-{}.toml", std::process::id()));
        let mut f = fs::File::create(&path).expect("create toml");
        write!(
            f,
            r#"
default = "family"

[clouds.family]
url = "https://home.example/"
token = "fam-xxx"

[clouds.work]
url = "https://team.corp/"
token = "work-yyy"
"#
        )
        .expect("write toml");

        // SAFETY: single-threaded within this test; removed before returning.
        unsafe {
            std::env::set_var("KAERU_MCP_CLOUDS_FILE", &path);
        }
        let cfg = KaeruMcpConfig::new().expect("load config");
        unsafe {
            std::env::remove_var("KAERU_MCP_CLOUDS_FILE");
        }
        let _ = fs::remove_file(&path);

        assert_eq!(cfg.default_cloud, "family");
        assert_eq!(cfg.clouds.len(), 2, "two clouds parsed");
        assert_eq!(cfg.clouds["family"].url, "https://home.example/");
        assert_eq!(cfg.clouds["family"].token, "fam-xxx");
        assert_eq!(cfg.clouds["work"].url, "https://team.corp/");
    }
}
