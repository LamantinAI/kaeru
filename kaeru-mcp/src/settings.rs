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
