# Instance visualizer

`kaeru-mcp` serves `/ui/` on the same Axum listener as MCP and the named
`/v1/` memory API. The binary includes HTML, JavaScript, styles and fonts.
Installed instances need no Node.js, separate frontend process or exported graph.

## Local installation and authentication

The local installer asks whether to enable authentication. A new installation
chooses **no authentication** by default. It prints a warning to enable
protection before deploying on a hosted or publicly reachable server.
Unattended updates preserve the configured mode; interactive updates use the
existing mode as their default.

- Without authentication, `/ui/` opens the graph directly. There is no login,
  sign-out or account settings screen. MCP and memory HTTP requests do not need
  credentials; stale cookies and old environment tokens are ignored when the
  configuration explicitly disables authentication.
- With authentication, the browser requires a username/password and MCP clients
  require a bearer token. Existing account files without an `enabled` field
  remain protected. A legacy environment-only MCP token also remains active
  unless authentication is explicitly disabled.

Installer automation:

```sh
KAERU_SETUP_AUTH=no bash install.sh
KAERU_SETUP_AUTH=yes KAERU_WEB_PASSWORD_FILE=/private/password.txt bash install.sh
KAERU_SETUP_AUTH=keep bash install.sh
```

`ask` is the default. Interactive questions read `/dev/tty`, so the choice also
works with `curl ... | bash`. `KAERU_MCP_WEB_CONFIG_FILE` selects the config path.
Older releases before 0.8 are not passed setup flags they cannot support.

For source/Cargo installations or a manual change, stop the daemon and choose:

```sh
kaeru-mcp --auth disabled
kaeru-mcp --auth enabled
kaeru-mcp --auth-status
```

Restart the daemon after changing its mode. Enabling creates the account,
password hash, JWT key and random MCP token; an already protected account keeps
its credentials. Disabling removes the stored credentials and writes a minimal
`enabled = false` configuration. Re-enabling asks for a new password.
`KAERU_MCP_AUTH_TOKEN` overrides the generated `mcp_token` while protection is on.
The stdio bridge reads the same selection and token without taking the daemon's
writer lock. Setup prints the configuration path, never the token/password.

`--init-web-auth` remains available for explicit account creation. The password
prompt does not echo. `--password-stdin` supports automation, `--username`
selects the login, and `--reset-web-password` performs local recovery.
Initialization and recovery do not open the vault or network listener.

Browser requests use relative URLs. By default login and CSRF checks compare
Origin with Host, so localhost, IP addresses and domains work without pinning
one address. A reverse proxy should preserve the browser-facing Host.
`--public-origin https://memory.example` can optionally pin it during setup.

The owner-only file defaults to `$XDG_CONFIG_HOME/kaeru/web.toml`, otherwise
`~/.config/kaeru/web.toml`. Windows uses `%APPDATA%\kaeru\web.toml`, falling
back to `%USERPROFILE%\AppData\Roaming\kaeru\web.toml`.
`KAERU_MCP_WEB_CONFIG_FILE` overrides it per instance. A daemon without an account
also runs when no default configuration directory is available.
When protection is enabled it holds the username, Argon2id verifier, MCP token,
persistent JWT signing key, lifetime, origin and session revision. Restart after manual edits. Its generated comment
is an example, not an enabled account:

```toml
# basic-creds: admin:admin
# Example only; no account is enabled by this comment.
```

## Access and sessions

Authentication mode does not widen `KAERU_MCP_VIZ_INITIATIVES`, `KAERU_MCP_VIZ_DENY` or
`KAERU_MCP_VIZ_INCLUDE_LOCAL`. These variables constrain HTTP/UI access; MCP
sessions retain their local owner capabilities. An empty allowed set shows no
records. An operator
intentionally viewing their own entire vault can explicitly configure:

```sh
export KAERU_MCP_VIZ_INITIATIVES='*'
export KAERU_MCP_VIZ_INCLUDE_LOCAL=1
kaeru-mcp
```

With protection enabled, agents use `mcp_token` from the private config or
`KAERU_MCP_AUTH_TOKEN`. A protected configuration without a usable MCP token
rejects requests rather than silently reopening the service. Browser cookies
do not authorize MCP; static MCP credentials do not authorize administration.

Sign-in sets an HttpOnly, SameSite=Strict JWT cookie, also Secure for HTTPS.
Cookie-authenticated changes and memory requests require a matching Origin/Host
and CSRF header. An explicitly configured public origin adds a fixed restriction.
JWT verification fixes the algorithm and checks issuer,
audience, time, identity and revision. Keys survive restarts. Login and password
work are bounded; credentials never enter graph journals or browser storage.

`/ui/admin` changes the account/password and session lifetime with password
confirmation. Changes and logout revoke all browser sessions. Effective memory
permissions are displayed read-only, without hashes, signing keys or tokens.
Expired sessions return to login; permission errors, failures and empty graphs
remain visible. None request an offline snapshot.

`KAERU_MCP_UI_ENABLE=0` hides the bundled pages while preserving account
endpoints. Named `/v1/` routes remain active if a protected browser account
exists or `KAERU_MCP_VIZ_ENABLE=1`; otherwise hiding the UI also removes those
routes. This does not disable MCP or change its authentication mode. A platform
frontend behind nginx can use the same `/auth/`, `/admin/` and `/v1/` routes.

## Building

Source: `kaeru-viz/`; generated bundle: `kaeru-mcp/ui-dist/`.

```sh
cd kaeru-viz
npm ci
npm test
npm run build:embedded
npm run check:embedded
```

CI verifies source/lockfile and generated output. Cargo embeds the checked-in
bundle and tracks changes; it never runs npm. The bundle excludes graph dumps,
source maps and unexpected files. Static routes have no filesystem/SPA fallback
outside the explicit UI pages. The source UI is replaceable; memory semantics
and JWT verification live outside its presentation layer.

The MCP image builds in three stages: Node frontend, Rust binary, runtime.
Only the executable reaches runtime; installers still deliver one executable.
The local systemd/launchd installer configures the UI to view the owner’s vault
with `KAERU_MCP_VIZ_INITIATIVES=*` and `KAERU_MCP_VIZ_INCLUDE_LOCAL=1`. Explicit
API scope, visibility and validation rules still apply in either auth mode.

For Vite development, set `KAERU_VIZ_URL` to the daemon; auth/admin/API routes
are proxied. If an origin is explicitly pinned, use the Vite browser origin.
End-to-end tests use a real embedded binary and temporary private accounts/vaults:

```sh
cargo build -p kaeru-mcp
cd kaeru-viz
npx playwright install chromium
npm run test:browser
```

`KAERU_MCP_TEST_BINARY` and `KAERU_BROWSER_PATH` select existing test executables.
`KAERU_SCREENSHOT_DIR` retains screenshots. Tests never use the live daemon.

## Containers

Build from the workspace root, then persist the vault **and** authentication
configuration. This example first initializes a protected account without
opening the vault/listener, then starts an owner-access UI bound to host loopback:

```sh
docker build -f docker/Dockerfile.mcp -t kaeru-mcp .
docker run --rm -it \
  -v kaeru-config:/home/kaeru/.config/kaeru \
  kaeru-mcp --auth enabled

docker run --name kaeru-mcp \
  -p 127.0.0.1:9876:9876 \
  -v kaeru-data:/home/kaeru/.local/share/kaeru \
  -v kaeru-config:/home/kaeru/.config/kaeru \
  -e KAERU_MCP_LISTEN_ADDRESS=0.0.0.0 \
  -e KAERU_MCP_VIZ_INITIATIVES='*' \
  -e KAERU_MCP_VIZ_INCLUDE_LOCAL=1 \
  kaeru-mcp
```

For intentional local open mode, choose `--auth disabled` during initialization.
For access through a public host, configure the MCP allowed Host authorities and
HTTPS reverse proxy; preserve the browser-facing Host and optionally pin its
HTTPS origin during account setup.

The repository `docker-compose.yaml` demonstrates the local/cloud bridge. Its
base definition persists the two vaults but does **not** persist MCP account
configuration, select an auth mode, grant HTTP/UI owner scope or add routable
MCP Host authorities. Supply those explicitly through your deployment/override;
the local installer's account/scope setup does not run inside an image.
