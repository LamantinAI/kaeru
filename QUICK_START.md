# Quick Start

> **Pre-1.0 alpha.** Additive schema changes migrate automatically on start
> (see §5); destructive changes can still need a semantic rebuild. Until 1.0
> stabilises, keep a markdown export before major upgrades.

`kaeru` runs as one long-lived MCP daemon per machine. Agents do not spawn it
over stdio: the vault is backed by RocksDB, and only one process can own the
writer lock safely.

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
- `KAERU_VERSION=v0.1.0` pins a release tag instead of `latest`.
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

For Intel Mac or Linux ARM, build from source.

### From source

Prerequisites: Rust 1.95+. On Linux you also need `libclang-dev` for the
RocksDB build.

```bash
git clone https://github.com/LamantinAI/kaeru.git
cd kaeru
cargo test --workspace
cargo install --path kaeru-mcp
kaeru-mcp
```

By default the daemon listens on `http://127.0.0.1:9876` and stores the vault
under the platform default:

- Linux: `$XDG_DATA_HOME/kaeru`, usually `~/.local/share/kaeru`
- macOS: `~/Library/Application Support/ai.lamantin.kaeru`

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

Restart Claude Code. The agent should see MCP tools such as `awake`, `drill`,
`at`, `jot`, `cite`, `claim`, `task`, `surface`, `export`, and `overview`.

For best results, make kaeru the only durable memory store. Either install with:

```bash
curl -fsSL https://raw.githubusercontent.com/LamantinAI/kaeru/main/contrib/install/install.sh \
  | KAERU_SETUP_CLAUDE_MEMORY=yes bash
```

or configure the same idea manually in Claude settings:

- set `autoMemoryEnabled` to `false`;
- add a session-start reminder telling the agent: source of truth is kaeru,
  start with `initiatives` -> `awake` -> `overview`, and write durable facts back
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

Opencode 1.15.x still uses legacy HTTP+SSE, so the contrib config points at:

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

### Remote Or Docker Daemons

If the daemon binds to anything other than loopback, remember:

- set `KAERU_MCP_AUTH_TOKEN` to a shared secret. Once set, every request to
  both transports (`/mcp` and `/sse` + `/messages`) must carry
  `Authorization: Bearer <token>`; anything else gets `401`. Left unset on a
  non-loopback bind, the port is open curator access to the vault — the daemon
  logs a warning to that effect at startup;
- set `KAERU_MCP_ALLOWED_HOSTS` to the hostnames or `host:port` values clients
  use, otherwise the streamable HTTP transport rejects non-loopback `Host`
  headers.

Example:

```bash
KAERU_MCP_LISTEN_ADDRESS=0.0.0.0
KAERU_MCP_AUTH_TOKEN=replace-with-a-long-random-secret
KAERU_MCP_ALLOWED_HOSTS=192.0.2.10:9876,kaeru.example
```

The token is a static shared secret, not OAuth — the minimal control for a
single-operator daemon. It travels in plaintext over `http://`, so if the
daemon is reachable beyond a trusted network, terminate TLS in front of it
(reverse proxy) so the bearer token cannot be sniffed.

## 3. Re-Entry Ritual

Do this at the start of every agent session:

```text
initiatives
awake(initiative: "<project>")
overview(initiative: "<project>")
```

After that, keep passing `initiative` on meaningful calls. Without it, reads are
cross-initiative and writes become untagged.

Common moves:

- `jot` / `episode` for working observations;
- `cite` for settled facts, specs, references, persona records, and decisions;
- `claim` -> `test` -> `confirm` / `refute` for hypotheses;
- `task` / `done` for actionable todos;
- `search`, `drill`, `trace`, `between`, `tagged` for recall; `at <name>` to read a
  node IN FULL (whole body + every field — `drill` / `search` only show excerpts);
  `surface` to pull archived `cold` / `frozen` layers that `awake` doesn't load;
- `synthesise`, `settle`, `reopen`, `supersede` when knowledge changes shape;
- `policy`, `share`, `cloud_recall`, `pull`, `sync_review` for team sharing (see §6);
- `rename_initiative` / `delete_initiative` to reorganise or drop a project — local
  by default, `cloud=true` to apply it team-wide.

## 4. Memory Layers

Current kaeru memory has two orthogonal axes:

- **Tier:** `operational` is working memory: observations, drafts, hypotheses,
  open questions, and tasks. `archival` is settled memory: references, ideas,
  outcomes, summaries, persona/entity records.
- **Layer:** `core`, `hot`, `warm`, `cold`, `frozen` describe how aggressively
  an item should be surfaced to future agents. Stamp it **at creation** —
  `episode`/`jot`/`cite`/`task`/`claim` all take an optional `layer` (default
  `warm`) — so a node is born with its priority. Keep truly central material
  (`core`) small and explicit, and let stale material cool down. `awake` loads
  `core → hot → warm`; reach `cold` / `frozen` on demand with `surface`.

When capturing, choose the verb by epistemic status, not by length. If the fact
is already settled, use `cite`; if it is still unfolding, use `episode` or
`claim`. After capturing, search for related nodes and link them, otherwise the
new node is easy to lose.

## 5. Schema Changes & Rebuilding A Vault

**Additive schema changes migrate automatically.** On every start the daemon
runs a forward-only migration journal (`migration_journal`): new relations and
columns added by a newer build are applied to an existing vault in place, so a
routine upgrade needs no action. Migrations are add-only — there is no
down-migration or destructive-change path.

### Routine upgrade (seamless — but export first)

A normal version bump is **seamless**: the new binary migrates your existing
vault in place on start and your data is preserved. Still, **export every
initiative first** as a one-command safety net before upgrading — cheap
insurance, and the markdown is a clean fallback if anything looks off.

1. **Back up — export each initiative** (ask the agent, or call directly):

   ```text
   initiatives                      # list them
   export(output_dir: "~/kaeru-backup/<initiative>", initiative: "<initiative>")
   ```

2. **Upgrade the binary and restart:**

   ```bash
   cd kaeru && git pull
   cargo install --path kaeru-mcp --force
   systemctl --user restart kaeru-mcp        # or restart your manual process
   ```

3. **Verify:** `awake` / `overview` per initiative — counts should match what
   you had. Migrations auto-applied; nothing else to do.

4. **Rollback (only if needed):** your pre-upgrade markdown export is the
   fallback — see the semantic rebuild below. Keep the export until you've
   confirmed the upgrade is healthy.

If a release note flags a **non-additive** change (rare, pre-1.0), follow the
semantic rebuild instead of relying on auto-migration.

When an upgrade involves a change migrations can't cover (a destructive or
incompatible schema shift, flagged in the release notes), rebuild the vault
semantically rather than with a blind database rewrite:

1. Upgrade and start the new `kaeru-mcp`.
2. Ask the agent to list initiatives with `initiatives`.
3. Export every current initiative to markdown:

```text
export(output_dir: "/tmp/kaeru-export/<initiative>", initiative: "<initiative>")
```

Repeat for each initiative. The export contains `README.md`, `INDEX.md`,
`LOG.md`, and node pages grouped by tier/type.

4. If you are rebuilding into a fresh vault, stop the daemon, point
   `KAERU_VAULT_PATH` at a new empty directory, and start it again.
5. Ask the agent to resynchronise from the exports. A useful prompt:

```text
For each exported kaeru initiative under /tmp/kaeru-export:
- read README.md, INDEX.md, LOG.md, and the node pages;
- recreate durable settled material with cite/synthesise/settle under the same initiative;
- recreate only still-relevant open work as task, claim, or episode;
- preserve important relationships with link;
- keep routine stale observations out unless they are needed for provenance;
- finish each initiative with awake, overview, and lint, then fix obvious orphaned important nodes.
```

Do not re-import everything mechanically. The goal is to let the agent classify
old memory into the tier/layer model and drop stale operational noise while
preserving settled knowledge and active work.

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
default = "family"          # used when a tool omits `cloud`

[clouds.family]
url   = "https://home.example/"
token = "fam-xxx"

[clouds.work]
url   = "https://team.corp/"
token = "work-yyy"
```

The cloud verbs below then take an optional `cloud: "<name>"`; a soft link
remembers which cloud it points at, and `cloud_links` resolves each against the
right one. The single `KAERU_MCP_CLOUD_URL`/`_TOKEN` pair still works on its own
(folded in as the `default` cloud) — no file needed for one cloud.

### Sharing flow

Sharing is explicit and gated; nothing leaves automatically.

1. Mark an initiative shareable, once: `policy(initiative: "<proj>", policy: "team")`.
   Default is `private` — personal initiatives never leave.
2. Share a node: `share(name: "<node>", initiative: "<proj>")`. Two gates run —
   the initiative policy, and a secret guard that blocks API keys / tokens /
   private keys. The local node is marked `shared` only after the cloud accepts it.
3. Or capture-and-share in one call: `episode(..., visibility: "shared", initiative: "<proj>")`
   — also `jot` and `cite`.
4. Batch review: `sync_review(initiative: "<proj>")` splits still-local nodes into
   PROPOSE SHARE (guard-clean) vs KEEP LOCAL (guard-flagged). Review once, then
   `share` the approved ones.

### Recall flow

- `cloud_recall(initiative: "<proj>")` — list what the team has shared.
- `pull(id: "<id>", initiative: "<proj>")` — bring a shared node into your local graph.
- `link_cloud(name: "<local>", cloud_id: "<id>")` then `cloud_links(name: "<local>")` —
  reference a cloud node from a local one without copying, and resolve it on demand.

Per-user / per-org isolation is a future addition; today the cloud is one shared
space scoped by initiative.
