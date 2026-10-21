# Quick Start

This guide follows the current source checkout. The installer downloads a
published release; if that binary predates a feature described here, upgrade or
build from source. See [the documentation index](docs/README.md).

One daemon owns each embedded RocksDB vault. Agents connect over HTTP or use
`kaeru-mcp --stdio` as a relay to that daemon; the relay does not open a second
copy of the vault.

## 1. Install And Start The Daemon

### Prebuilt binary

```bash
curl -fsSL https://raw.githubusercontent.com/LamantinAI/kaeru/main/contrib/install/install.sh | bash
```

The installer downloads the latest release, installs `kaeru-mcp` into
`~/.local/bin`, creates a user-level daemon, and starts it:

- Linux: `~/.config/systemd/user/kaeru-mcp.service`
- macOS: `~/Library/LaunchAgents/ai.lamantin.kaeru-mcp.plist`

Useful install knobs:

- `KAERU_INSTALL_DIR=/usr/local/bin` changes the binary destination.
- `KAERU_VERSION` pins a published release tag instead of `latest`.
- `KAERU_SETUP_AUTH=ask|yes|no|keep` controls optional local authentication.
  Fresh installs default to no auth; unattended updates preserve the configured mode.
- `KAERU_WEB_PASSWORD_FILE=/private/password.txt` supplies a password for
  unattended `KAERU_SETUP_AUTH=yes` setup.
- `KAERU_SETUP_DAEMON=no` installs the binary but skips daemon setup.
- `KAERU_SETUP_CLAUDE_MEMORY=yes` also configures Claude Code to treat kaeru
  as the memory of record: disables Claude's built-in auto-memory and adds a
  `SessionStart` reminder hook. This is opt-in because it edits
  `~/.claude/settings.json`.

If `~/.local/bin` is not on your `PATH`, add it:

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

Supported prebuilt targets:

| OS | Arch | Notes |
| --- | --- | --- |
| Linux | x86_64 | static musl binary |
| macOS | aarch64 | Apple Silicon; unsigned, installer clears quarantine |
| Windows | x86_64 | MSVC build, static CRT — no VC++ redistributable needed |

The `curl … | bash` installer covers Linux and macOS. On Windows, download the
`…-x86_64-pc-windows-msvc.zip` from the [Releases](https://github.com/LamantinAI/kaeru/releases)
page and run `kaeru-mcp.exe` yourself. For Intel Mac or Linux ARM, build from source.

### From source

Prerequisites: Rust 1.95+, a C/C++ toolchain and libclang for RocksDB.
On Debian/Ubuntu, install `build-essential` and `libclang-dev`.
Node.js is unnecessary unless rebuilding the frontend: Cargo uses the checked-in bundle.

```bash
git clone https://github.com/LamantinAI/kaeru.git
cd kaeru
cargo test --workspace --all-targets --locked
cargo install --path kaeru-mcp --locked
kaeru-mcp
```

By default the daemon listens on `http://127.0.0.1:9876` and stores the vault
under the platform default:

- Linux: `$XDG_DATA_HOME/kaeru`, usually `~/.local/share/kaeru`
- macOS: `~/Library/Application Support/ai.lamantin.kaeru`
- Windows: `%LOCALAPPDATA%\ai.lamantin.kaeru`

Override the vault with `KAERU_VAULT_PATH=/path/to/vault`.

Check the daemon:

```bash
# Linux
systemctl --user status kaeru-mcp
journalctl --user -u kaeru-mcp -f

# macOS
launchctl list | grep kaeru-mcp
tail -f ~/Library/Logs/kaeru-mcp.log
```

## 2. Connect An Agent

### Claude Code

```bash
claude mcp add --transport http kaeru http://127.0.0.1:9876/mcp
```

If the daemon enforces a token (see [Remote Or Docker Daemons](#remote-or-docker-daemons)),
pass it as a header:

```bash
claude mcp add --transport http \
  --header "Authorization: Bearer <token>" \
  kaeru http://<host>:9876/mcp
```

Restart Claude Code. The agent should see MCP tools such as `context`, `search`, `read`, `note`, `reference`, `claim`, `verdict`, and `task`.

For best results, make kaeru the only durable memory store. Either install with:

```bash
curl -fsSL https://raw.githubusercontent.com/LamantinAI/kaeru/main/contrib/install/install.sh \
  | KAERU_SETUP_CLAUDE_MEMORY=yes bash
```

or configure the same idea manually in Claude settings:

- set `autoMemoryEnabled` to `false`;
- add a session-start reminder telling the agent: source of truth is kaeru,
  start with `initiatives` -> `context`, and write durable facts back
  to kaeru.

The installer implementation is in `contrib/install/install.sh`.

### Opencode

First install and start `kaeru-mcp`, then from this repository run:

```bash
bash contrib/opencode/install-opencode.sh
```

This copies `AGENTS.kaeru.md`, installs `/kaeru`, `/lesson`, and `/recall`
commands, and merges a small `mcp.kaeru` block into
`~/.config/opencode/opencode.json`.

The bundled Opencode snippet uses the legacy HTTP+SSE endpoint:

```text
http://localhost:9876/sse
```

Other modern MCP clients usually want streamable HTTP:

```text
http://127.0.0.1:9876/mcp
```

If the daemon enforces a token, add an `Authorization: Bearer <token>` header
to the `mcp.kaeru` block in `~/.config/opencode/opencode.json` — the gate
covers the SSE transport too.

See `contrib/opencode/README.md` for the exact files and merge behaviour.

### Stdio-only clients

Configure the client command as `kaeru-mcp` with argument `--stdio`. It reuses
the configured daemon and its saved MCP token, starting the daemon if necessary.

### Local UI and authentication

Open `http://127.0.0.1:9876/ui/`. The local installer grants the UI access to its
owner’s vault. For a manual/source run, set the HTTP/UI scope explicitly:

```sh
KAERU_MCP_VIZ_INITIATIVES='*' KAERU_MCP_VIZ_INCLUDE_LOCAL=1 kaeru-mcp
```

To change authentication, stop the daemon, run `kaeru-mcp --auth enabled` or
`kaeru-mcp --auth disabled`, then restart it. Enabling prompts for a password
and persists a separate MCP token in the private `web.toml`; there is no active
`admin:admin` account. `kaeru-mcp --auth-status` prints the selected mode.
See [UI/account setup](docs/instance-ui.md) for recovery and automation.

### Remote Or Docker Daemons

If the daemon binds to anything other than loopback, remember:

- enable authentication with `kaeru-mcp --auth enabled` and configure MCP
  clients with `Authorization: Bearer <mcp_token>` from the private config.
  `KAERU_MCP_AUTH_TOKEN` can override that token. An explicit `--auth disabled`
  takes precedence over leftover environment tokens, so setting a token alone
  does not override a deliberately disabled configuration;
- set `KAERU_MCP_ALLOWED_HOSTS` to the hostnames or `host:port` values clients
  use, otherwise the streamable HTTP transport rejects non-loopback `Host`
  headers.

Example:

```bash
# After enabling authentication; use the saved MCP token in clients.
KAERU_MCP_LISTEN_ADDRESS=0.0.0.0 \
KAERU_MCP_ALLOWED_HOSTS=192.0.2.10:9876,kaeru.example \
  kaeru-mcp
```

The token is a static shared secret, not OAuth — the minimal control for a
single-operator daemon. It travels in plaintext over `http://`, so if the
daemon is reachable beyond a trusted network, terminate TLS in front of it
(reverse proxy) so the bearer token cannot be sniffed.

## 3. Recover project context

```text
initiatives()
context(initiative="<established project>")
search(initiatives=["<project>"], text="entity", prefix=true)
read(initiative="<project>", node="<returned uuid>")
```

Project calls require initiative. Capture observations/decisions with `note`,
exact source documents with `reference(source={"text":"..."})`, and work with
`task`. `verdict` can record the check and status together. See
[the 35-operation reference](docs/curator-api.md) for the full contract.
Follow continuation arguments and inspect coverage; excerpts are not full records.

## 4. Tiers and recall layers

Operational versus archival tier describes whether knowledge is in flight or
settled. `lifecycle` changes tier explicitly. Recall layers are independent:
core, hot, warm, cold and frozen. Capture `options.layer` selects a layer;
`retention(action="set_layer", ...)` changes it. Context restores standing rules
and unfinished work; search filters reach retained material on demand. Slots
are current pointers, not supersession or role changes.

## 5. Upgrades and backups

Vault opening runs forward migrations automatically. Migrations may add
relations, normalize stored metadata or recover old memberships; they are not
limited to adding columns. Downgrading a migrated vault is not a supported
rollback procedure.

Before upgrading:

1. Stop the daemon or embedded application owning the vault.
2. Copy the complete vault directory to a separate backup location. Back up
   `web.toml`, `clouds.toml` and relevant service configuration separately;
   preserve their permissions. A cold vault copy retains recorded history.
3. Optionally export a portable current-knowledge snapshot with a binary that
   supports AMG:

   ```sh
   kaeru-mcp amg export --vault /path/to/vault --all --output before-upgrade.amg
   ```

4. Upgrade the binary, then restart the owner. For a source installation:

   ```sh
   git pull --ff-only
   cargo install --path kaeru-mcp --locked --force
   systemctl --user restart kaeru-mcp
   ```

5. Verify expected initiatives and records with `context`, `read` and `history`.
   If rollback is needed, stop the service and restore the **pre-upgrade copy**
   with its matching binary/configuration. Keep the newer vault separately.

AMG transfers current knowledge into another vault and preserves IDs, properties,
local edges and semantic metadata. It does not transfer the source's old versions,
authentication or journals. Use `amg inspect`, then `amg import --dry-run` before
applying a merge. Conflicts abort by default; `--force` appends supplied versions
while retaining the destination's existing history. See [AMG](docs/amg.md).

The agent-facing `export` operation is a readable view, separate from AMG:

```text
export(initiative="project", destination="project-snapshot")
```

It writes `graph.json` and `README.md` beneath the host's authorized artifact
root (`KAERU_MCP_ARTIFACT_ROOT` for the daemon). Choose a new destination; its
parent must already exist inside that root. It is not a complete backup, and
there is no automatic Markdown re-import operation.

## 6. Team Cloud (Sharing & Recall)

By default everything stays on your machine. To share settled knowledge with a
trusted group (a team, a family), run the shared `kaeru-cloud` service and point
the daemon at it.

### Run the stack

The simplest way is docker-compose, which starts `kaeru-cloud` and a `kaeru-mcp`
already wired to it:

```bash
KAERU_CLOUD_API_TOKEN=replace-with-a-long-random-secret docker compose up --build
```

Or run the cloud yourself and wire each daemon by env:

```bash
KAERU_CLOUD_API_TOKEN=<secret> kaeru-cloud        # the shared service (port 9877)

# on each machine, point the local daemon at it:
KAERU_MCP_CLOUD_URL=http://<cloud-host>:9877 \
KAERU_MCP_CLOUD_TOKEN=<secret> \
  kaeru-mcp
```

`KAERU_CLOUD_API_TOKEN` is **mandatory** for any non-loopback bind: with an
empty token on a routable address `kaeru-cloud` refuses to start (an empty
token disables auth, which would leave the shared store open). Empty is allowed
only on `127.0.0.1` for local dev.

If the cloud is reachable beyond a trusted network, terminate TLS with a reverse
proxy in front of it — the service speaks plain HTTP. The proxy must forward all
paths (`/health`, `/api/v1/*`), not just one prefix.

### Multiple clouds (optional)

One daemon can reach several clouds (e.g. a `family` and a `work` cloud). List
them in `$XDG_CONFIG_HOME/kaeru/clouds.toml` (override via
`KAERU_MCP_CLOUDS_FILE`):

```toml
default = "family"          # config metadata; multiple-cloud calls still name cloud

[clouds.family]
url   = "https://home.example/"
token = "fam-xxx"

[clouds.work]
url   = "https://team.corp/"
token = "work-yyy"
```

With several clouds configured, cloud operations require `cloud: "<name>"`,
even when `default` is set. One configured cloud resolves implicitly. A soft link
remembers which cloud it points at, and `cloud_links` resolves each against the
right one. The single `KAERU_MCP_CLOUD_URL`/`_TOKEN` pair still works on its own
(folded in as the `default` cloud) — no file needed for one cloud.

### Sharing flow

Sharing is explicit and gated; nothing leaves automatically.

1. Mark an initiative shareable, once: `policy(initiative="<proj>", policy="team")`.
   Default is `private` — personal initiatives never leave.
2. Share a node: `share(name="<node>", initiative="<proj>")`. Two gates run —
   the initiative policy, and a secret guard that blocks API keys / tokens /
   private keys. The local node is marked `shared` only after the cloud accepts it.
3. Or capture-and-share in one call: `note(initiative="<proj>", body="...", sharing={"visibility":"shared"})`
   — also `reference`.
4. Batch review: `sync_review(initiative="<proj>")` splits still-local nodes into
   PROPOSE SHARE (guard-clean) vs KEEP LOCAL (guard-flagged). Review once, then
   `share` the approved ones.

### Recall flow

- `cloud_recall(initiative="<proj>")` — list what the team has shared.
- `pull(id="<id>", initiative="<proj>")` — bring a shared node into your local graph.
- `link_cloud(initiative="<proj>", name="<local>", cloud_id="<id>")` then
  `cloud_links(initiative="<proj>", name="<local>")` —
  reference a cloud node from a local one without copying, and resolve it on demand.

Per-user / per-org isolation is a future addition; today the cloud is one shared
space scoped by initiative.
