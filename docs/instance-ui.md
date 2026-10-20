# Instance visualizer

`kaeru-mcp` serves `/ui/` on the same Axum listener as MCP and the named
`/v1/` memory API. The binary includes HTML, JavaScript, styles and fonts.
Installed instances need no Node.js, separate frontend process or exported graph.

## Setup

Create a local account before starting the daemon:

```sh
kaeru-mcp --init-web-auth
```

The command prompts without echoing the password. Browser requests use relative
URLs; by default, login and CSRF checks compare Origin with the request Host.
Localhost, IP addresses and domains work without pinning one address. A reverse
proxy should preserve the browser-facing Host. To optionally pin an origin:

```sh
kaeru-mcp --init-web-auth --public-origin https://memory.example
```

`--username` chooses the login, `--web-config` selects the file, and
`--password-stdin` enables explicit automation. Passwords are not command-line
arguments. Local recovery uses `--reset-web-password`; stop the daemon first.
Initialization/recovery do not open the vault or network listener.

The owner-only file defaults to `$XDG_CONFIG_HOME/kaeru/web.toml`, otherwise
`~/.config/kaeru/web.toml`. Windows uses `%APPDATA%\kaeru\web.toml`, falling
back to `%USERPROFILE%\AppData\Roaming\kaeru\web.toml`.
`KAERU_MCP_WEB_CONFIG_FILE` overrides it per instance. A daemon without an account
also runs when no default configuration directory is available.
It holds the username, Argon2id verifier, persistent JWT signing key, lifetime,
origin and session revision. Restart after manual edits. Its generated comment
is an example, not an enabled account:

```toml
# basic-creds: admin:admin
# Example only; no account is enabled by this comment.
```

## Access and sessions

Login does not widen `KAERU_MCP_VIZ_INITIATIVES`, `KAERU_MCP_VIZ_DENY` or
`KAERU_MCP_VIZ_INCLUDE_LOCAL`. An empty allowed set shows no records. An operator
intentionally viewing their own entire vault can explicitly configure:

```sh
export KAERU_MCP_VIZ_INITIATIVES='*'
export KAERU_MCP_VIZ_INCLUDE_LOCAL=1
kaeru-mcp
```

Agents retain the separate `KAERU_MCP_AUTH_TOKEN`. Browser-account activation
closes anonymous MCP HTTP access even when that token is absent; configure it
for MCP clients and the stdio bridge. Browser cookies do not authorize MCP;
static MCP credentials do not authorize administration.

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

`KAERU_MCP_UI_ENABLE=0` hides the bundled pages while preserving configured
API authentication and account endpoints. A platform frontend behind nginx can
use the same `/auth/`, `/admin/` and `/v1/` routes.

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
Mount both the vault and `/home/kaeru/.config/kaeru` persistently in containers.

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
