# kaeru-mcp

One long-lived process owns an embedded Kaeru vault and serves MCP, the named
HTTP memory API and the embedded visualizer. Multiple clients share that owner;
separate vaults can run on separate ports. No Rust/Node.js runtime is needed
for an installed binary.

## Install and run

From the workspace root:

```sh
cargo install --path kaeru-mcp --locked
kaeru-mcp
```

Source builds need Rust 1.95+, a C/C++ toolchain and libclang. Cargo embeds the
checked-in `ui-dist/`; frontend changes need an explicit rebuild from
`kaeru-viz/`. See [Quick Start](../QUICK_START.md) for prebuilt releases.

The local [installer](../contrib/install/install.sh) generates a systemd user
unit or launchd agent, configures optional authentication and starts the service.
It does not copy pre-existing service templates. Managed locations are:

- Linux: `~/.config/systemd/user/kaeru-mcp.service`.
- macOS: `~/Library/LaunchAgents/ai.lamantin.kaeru-mcp.plist`.

```sh
systemctl --user status kaeru-mcp
journalctl --user -u kaeru-mcp -f
# After changing a binary or service environment:
systemctl --user restart kaeru-mcp
```

On macOS use launchd for `ai.lamantin.kaeru-mcp`; logs are written to
`~/Library/Logs/kaeru-mcp.log`. A source-only installation can run in the
foreground or use an operator-managed service.

## Endpoints

The default listener is `http://127.0.0.1:9876`.

| Path | Purpose |
| --- | --- |
| `/mcp` | Streamable HTTP MCP. |
| `/sse` and `/messages` | Legacy HTTP+SSE MCP for clients needing that transport. |
| `/v1/<operation>` | Named memory API: POST the same JSON arguments as its MCP tool. |
| `/ui/` | Embedded live graph visualizer. |
| `/ui/login`, `/ui/admin` | Browser account screens when authentication is enabled. |
| `/auth/*`, `/admin/*` | Browser session and account endpoints. |

The UI is enabled by default and also enables the named HTTP API. Its memory
scope defaults to empty; enable access explicitly for manual runs:

```sh
KAERU_MCP_VIZ_INITIATIVES='*' KAERU_MCP_VIZ_INCLUDE_LOCAL=1 kaeru-mcp
```

Installer-managed local services set that owner scope. HTTP/UI scope is an
operator ceiling; request arguments cannot widen it. MCP sessions use the local
owner API rather than inheriting this HTTP/UI ceiling. See
[access and sessions](../docs/instance-ui.md#access-and-sessions).

`KAERU_MCP_UI_ENABLE=0` hides pages. The HTTP API remains enabled if a protected
browser account exists or `KAERU_MCP_VIZ_ENABLE=1` is set. Hiding the UI does
not disable account authentication endpoints or turn off MCP.

## Clients

Point streamable-HTTP clients at `/mcp`. For example:

```sh
claude mcp add --transport http kaeru http://127.0.0.1:9876/mcp
```

With authentication enabled, send `Authorization: Bearer <mcp_token>` using
that client's header configuration. The browser username/password is not an
MCP credential.

Stdio-only clients run:

```sh
kaeru-mcp --stdio
```

The relay connects to the daemon, starting it if necessary, without opening a
second vault. It reads the same configured MCP token. Logs go to stderr; stdout
carries protocol frames. Release `.mcpb` bundles also invoke this relay.

The [Opencode integration](../contrib/opencode/README.md) installs a legacy-SSE
snippet and agent instructions. Other clients may connect directly over HTTP
or through the stdio relay; use the format supported by their installed version.
The [portable skill](../skills/kaeru-skill/README.md) supplies memory-use guidance.
The optional Claude memory-of-record setup is documented in
[Quick Start](../QUICK_START.md#claude-code).

## Configuration

### Listener and authentication

| Variable | Default | Meaning |
| --- | --- | --- |
| `KAERU_MCP_LISTEN_ADDRESS` | `127.0.0.1` | IPv4 bind address. |
| `KAERU_MCP_LISTEN_PORT` | `9876` | Listener port. |
| `KAERU_MCP_ALLOWED_HOSTS` | Empty extra list | Additional authorities accepted by the streamable-HTTP Host guard; comma-separated. Loopback remains allowed. |
| `KAERU_MCP_MOUNT_PATH` | `/mcp` | Streamable HTTP mount. |
| `KAERU_MCP_SSE_PATH` | `/sse` | Legacy GET endpoint. |
| `KAERU_MCP_MESSAGES_PATH` | `/messages` | Legacy POST endpoint. |
| `KAERU_MCP_KEEP_ALIVE_SECS` | `0` | MCP idle reaping; zero disables it. |
| `KAERU_MCP_LOG_LEVEL` | `info` | Tracing level. |
| `KAERU_MCP_WEB_CONFIG_FILE` | Platform config path | Local account/authentication-mode file. |
| `KAERU_MCP_AUTH_TOKEN` | Empty | Overrides the stored MCP token while authentication is enabled. |

Authentication is optional. Stop the daemon, run `kaeru-mcp --auth enabled` or
`kaeru-mcp --auth disabled`, then restart. Enabled mode creates a local account,
JWT signing key and separate random MCP token in private `web.toml`. There are
no active default credentials. `--auth-status` reports the chosen mode.

Explicit `enabled=false` overrides stale tokens and cookies. Without an auth
config, a nonempty environment token enables the legacy token-only mode; that
mode does not create a browser account. See [account setup and recovery](../docs/instance-ui.md).

For a remote bind, enable authentication, configure `KAERU_MCP_ALLOWED_HOSTS`
with the authority clients use, and terminate HTTPS at the reverse proxy. The
Host guard is separate from authentication; a rejected Host can fail the MCP
handshake even with a valid token.

### HTTP/UI and host capabilities

| Variable | Default | Meaning |
| --- | --- | --- |
| `KAERU_MCP_UI_ENABLE` | Enabled | Serve bundled UI pages. |
| `KAERU_MCP_VIZ_ENABLE` | Unset | Explicitly enable named HTTP routes when UI is hidden. |
| `KAERU_MCP_VIZ_INITIATIVES` | Empty | HTTP/UI allowed initiatives; `*` explicitly allows all. |
| `KAERU_MCP_VIZ_DENY` | Empty | Initiatives always denied to HTTP/UI. |
| `KAERU_MCP_VIZ_INCLUDE_LOCAL` | False | Permit local as well as shared records to HTTP/UI. |
| `KAERU_MCP_VIZ_ALLOW_ORIGIN` | Unset | Optional response CORS origin; does not bypass browser Origin/CSRF checks. |
| `KAERU_MCP_ARTIFACT_ROOT` | Unset | Authorize source artifact reads and agent `export` destinations. |
| `KAERU_MCP_HYGIENE_ENABLE` | Enabled | Background recall-layer maintenance; `0` disables it. |
| `KAERU_MCP_UPDATE_CHECK` | Enabled | Optional daily release check; `0` disables it. |

### Cloud bridge

`KAERU_MCP_CLOUD_URL` and `KAERU_MCP_CLOUD_TOKEN` configure a single cloud.
Without any cloud endpoints the tools remain discoverable but report that no
cloud is configured. Multiple endpoints come from `clouds.toml`:

```toml
[clouds.family]
url = "https://home.example"
token = "replace-with-family-token"

[clouds.work]
url = "https://team.example"
token = "replace-with-work-token"
```

The file defaults to `$XDG_CONFIG_HOME/kaeru/clouds.toml`, otherwise
`~/.config/kaeru/clouds.toml`; override with `KAERU_MCP_CLOUDS_FILE`.
Configuration is read at daemon startup. **One cloud resolves implicitly;
several require an explicit `cloud` argument.** A `default` key does not bypass
that ambiguity check. Soft links retain their named cloud. The environment URL
is folded into the registry without replacing an identically named file entry.

### Vault and read tracing

`KAERU_VAULT_PATH` selects the vault; platform defaults and upgrade guidance are
in [Quick Start](../QUICK_START.md). Other substrate tunables are defined in
`kaeru-core/src/config.rs`. Use `context(view="diagnostics")` for diagnostics.
`KAERU_READ_TRACE_ENABLED=true` enables the optional
[local read journal](../docs/architecture.md#local-read-delivery-journal), outside the vault.

## Operator commands and knowledge transfer

`kaeru-mcp --help` lists daemon, stdio, authentication and AMG commands.
`kaeru-mcp amg --help` lists offline inspect/export/import. Stop the process
owning the selected vault before export/import; the lock is never bypassed.

```sh
kaeru-mcp amg export --vault /path/to/vault --initiative project --output project.amg
kaeru-mcp amg inspect project.amg
kaeru-mcp amg import project.amg --vault /path/to/other-vault --dry-run
```

See [AMG](../docs/amg.md) for force/conflict semantics and backup limits.
Agent `export` is a separate JSON/Markdown artifact operation, not AMG.

## Containers and releases

The MCP image builds frontend → Rust binary → runtime in three stages. Only
the embedded binary reaches runtime. Persist both the vault and configuration
directory; an account file inside an ephemeral container will not survive
replacement. See [container setup](../docs/instance-ui.md#containers).

`contrib/install/package-release.sh` builds release archives/bundles and the MCP
Registry manifest. Agent definitions come from the shared catalogue; update
core schemas and parity checks when changing that contract.
