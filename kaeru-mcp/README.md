# kaeru-mcp

Long-lived MCP service that exposes the kaeru curator API over HTTP.
**One daemon per machine** owns the substrate; any number of agent
sessions (Claude Code, Cursor, Continue, …) connect concurrently.

This is **not** a stdio MCP server you spawn from each agent session.
The substrate is single-writer (RocksDB under Cozo), so each subprocess
would race for the lock — the second one to start fails. Service-mode
solves that by putting one writer in front of the vault and letting
many readers/writers connect over HTTP.

## Build & install

```bash
cargo install --path kaeru-mcp
```

The binary lands at `~/.cargo/bin/kaeru-mcp`.

## Run as a service

### Linux (systemd, user-mode)

```bash
mkdir -p ~/.config/systemd/user
cp contrib/systemd/kaeru-mcp.service ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable --now kaeru-mcp
```

Check status / logs:

```bash
systemctl --user status kaeru-mcp
journalctl --user -u kaeru-mcp -f
```

Override env vars via `systemctl --user edit kaeru-mcp` (creates a
drop-in `override.conf`).

### macOS (launchd)

```bash
cp contrib/launchd/ai.lamantin.kaeru-mcp.plist ~/Library/LaunchAgents/
# Edit the file: replace REPLACE_ME with your username (3 spots).
launchctl load ~/Library/LaunchAgents/ai.lamantin.kaeru-mcp.plist
```

Logs land in `~/Library/Logs/kaeru-mcp.log`.

### Quick foreground run (for testing)

```bash
kaeru-mcp
# stops on Ctrl-C
```

By default it listens on `http://127.0.0.1:9876` and exposes two MCP
transports on the same port:

- `http://127.0.0.1:9876/mcp` — **streamable HTTP** (current MCP spec, used
  by Claude Code and most clients).
- `http://127.0.0.1:9876/sse` + `http://127.0.0.1:9876/messages` — **legacy
  HTTP+SSE** (2024-11-05 spec). Kept for clients that haven't migrated to
  streamable HTTP yet — currently Opencode 1.15.x
  ([opencode-ai/opencode#8058](https://github.com/anomalyco/opencode/issues/8058)).
  Point such clients at the `/sse` URL.

## Configuration

Two layers, both env-driven:

**Daemon transport** (`KAERU_MCP_*` — see `src/settings.rs`):

| Variable                     | Default       | Effect                                |
|------------------------------|---------------|---------------------------------------|
| `KAERU_MCP_LISTEN_ADDRESS`   | `127.0.0.1`   | Bind IPv4. `0.0.0.0` = LAN-exposed (no auth!). |
| `KAERU_MCP_ALLOWED_HOSTS`    | *(empty)*     | Extra `Host` authorities to accept beyond loopback, comma-separated. **Required when binding to `0.0.0.0`** — see note below. |
| `KAERU_MCP_LISTEN_PORT`      | `9876`        | TCP port.                             |
| `KAERU_MCP_MOUNT_PATH`       | `/mcp`        | Streamable HTTP mount path (must start with `/`). |
| `KAERU_MCP_SSE_PATH`         | `/sse`        | Legacy HTTP+SSE GET mount path.       |
| `KAERU_MCP_MESSAGES_PATH`    | `/messages`   | Legacy HTTP+SSE POST mount path.      |
| `KAERU_MCP_LOG_LEVEL`        | `info`        | `error` / `warn` / `info` / `debug` / `trace`. |
| `KAERU_MCP_AUTH_TOKEN`       | *(empty)*     | Bearer token required from inbound clients (covers `/mcp` + `/sse`). Empty = no inbound auth (fine on loopback). |

**Cloud bridge** (`KAERU_MCP_CLOUD_*` — sharing / recall via `kaeru-cloud`):

| Variable                  | Default     | Effect                                |
|---------------------------|-------------|---------------------------------------|
| `KAERU_MCP_CLOUD_URL`     | *(empty)*   | Base URL of the shared `kaeru-cloud` service (e.g. `http://cloud-host:9877`). Empty = cloud tools (`share` / `pull` / …) disabled. |
| `KAERU_MCP_CLOUD_TOKEN`   | *(empty)*   | Bearer token sent to the cloud; must match its `KAERU_CLOUD_API_TOKEN`. |

**Substrate / curator-API caps** (`KAERU_*` — see
`kaeru-core/src/config.rs`): `KAERU_VAULT_PATH`,
`KAERU_ACTIVE_WINDOW_SIZE`, `KAERU_RECENT_EPISODES_CAP`,
`KAERU_AWAKE_DEFAULT_WINDOW_SECS`, `KAERU_SUMMARY_VIEW_CHILDREN_CAP`,
`KAERU_BODY_EXCERPT_CHARS`, `KAERU_PROVENANCE_MAX_HOPS`,
`KAERU_DEFAULT_MAX_HOPS`, `KAERU_MAX_HOPS_CAP`.

Call the `config` MCP tool to see resolved values.

## Connecting an MCP client

### Claude Code

Once the daemon is running, register it as an HTTP-transport MCP
server. Either via CLI:

```bash
claude mcp add --transport http kaeru http://127.0.0.1:9876/mcp
```

…or directly in `~/.claude/claude_desktop_config.json` (or
`~/.config/claude/claude_code_settings.json`, depending on platform):

```json
{
  "mcpServers": {
    "kaeru": {
      "transport": "http",
      "url": "http://127.0.0.1:9876/mcp"
    }
  }
}
```

After restart, Claude sees the curator-API tools (`awake`, `drill`,
`claim`, `at`, `history`, …). Each tool accepts an optional
`initiative` parameter; pass it on every call once you've picked a
project.

#### Make kaeru your memory of record

Claude Code ships its own built-in auto-memory (an auto-loaded
`MEMORY.md` plus a file store under
`~/.claude/projects/<project>/memory/`) that competes with kaeru — left
alone, the agent keeps writing there and knowledge forks across two
systems. This is an integration gap, not a kaeru bug: a store baked into
the runtime's system prompt can't be out-competed by an MCP server's
instructions alone. Close it from the config side — rewrite the
auto-loaded `MEMORY.md` into a directive pointing at kaeru, migrate
existing notes in via `cite`, and optionally add a `SessionStart` hook
that reminds the agent to consult kaeru first. Step-by-step recipe (with
the hook JSON) is in
[`skills/kaeru-skill/SKILL.md`](../skills/kaeru-skill/SKILL.md) under
**"Memory of record"**.

### Opencode

Opencode 1.15.x speaks only the legacy HTTP+SSE transport (see
[opencode-ai/opencode#8058](https://github.com/anomalyco/opencode/issues/8058)).
Point its `mcp.kaeru.url` at `http://127.0.0.1:9876/sse` instead of
`/mcp`. The `contrib/opencode/install-opencode.sh` installer ships a
ready-made config snippet.

### Other MCP runtimes

Anything that speaks streamable HTTP MCP — Cursor, Continue, Goose,
mcp-inspector, etc. Format is the same; point the runtime at `/mcp`.
For clients still on legacy SSE, point them at `/sse`.

For poking at it interactively, the official inspector handles HTTP:

```bash
npx @modelcontextprotocol/inspector --transport http http://127.0.0.1:9876/mcp
```

## Tool catalogue

```
re-entry / session : awake, overview, initiatives, recent, pin, unpin, config
capture            : episode, jot, link, unlink, cite
lookup             : recall, drill, trace, search, ideas, outcomes, tagged, between
bi-temporal        : at, history
hypothesis         : claim, test, confirm, refute
review             : flag, resolve
consolidation      : settle, reopen, synthesise, supersede
metabolism         : forget, revise
cloud (sharing)    : policy, share, cloud_recall, pull, link_cloud, cloud_links, sync_review
diagnostics        : lint
snapshot           : export
```

`tools/list` returns descriptions and JsonSchema for each. Drill in
with the inspector to see full param shapes.

## Operational notes

- **Single writer.** Only one `kaeru-mcp` should run per machine
  per vault. If you start a second instance pointing at the same
  vault path it will fail at startup with a RocksDB `LOCK` error —
  this is the substrate refusing to corrupt itself, not a kaeru bug.
- **Auth.** Optional bearer token via `KAERU_MCP_AUTH_TOKEN`. Token-less
  on `127.0.0.1` is fine for personal use; when binding to `0.0.0.0`, set a
  token (and `KAERU_MCP_ALLOWED_HOSTS`) — otherwise the port is open curator
  access, and the daemon warns about it at startup. For exposure beyond a
  trusted network, terminate TLS with a reverse proxy so the token isn't
  sniffable.
- **LAN exposure needs `KAERU_MCP_ALLOWED_HOSTS`.** rmcp's Streamable
  HTTP transport carries a DNS-rebinding guard that validates the
  inbound `Host` header against an allow-list defaulting to loopback
  only (`localhost`, `127.0.0.1`, `::1`). So `KAERU_MCP_LISTEN_ADDRESS=0.0.0.0`
  on its own is **not enough** — a client connecting via the machine's
  routable address gets `403 Forbidden: Host header is not allowed`,
  and Claude Code mislabels that failed handshake as
  *"Needs authentication"*. List the authority clients use (host or
  `host:port`) so the guard lets it through:
  `KAERU_MCP_ALLOWED_HOSTS=192.0.2.10:9876,kaeru.lan`. Loopback stays
  allowed automatically.
- **Updates.** After `cargo install --path kaeru-mcp`, restart the
  service so the new binary takes over: `systemctl --user restart
  kaeru-mcp` / `launchctl unload+load`.
- **Schema migrations.** kaeru-core's bootstrap is idempotent — new
  indexes and FTS catalogues self-install on next start of the
  daemon. No manual migration step.
- **Concurrency model.** rmcp dispatches incoming tool calls onto
  tokio tasks; sequential requests within one MCP session are well-
  ordered. If a single client batch-fires many calls without waiting,
  responses can come back out of order, and read-after-write within
  the batch may race. Real agents wait for each response.

## Versioning

Rides the workspace version. The tool surface tracks `kaeru-core`'s
curator API; new verbs there get exposed automatically on the next
rebuild.
