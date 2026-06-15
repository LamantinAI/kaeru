# Quick Start

> **Pre-1.0 alpha.** The substrate schema may change between minor versions.
> Until 1.0 stabilises, keep a markdown export before upgrades or large
> migrations.

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

Restart Claude Code. The agent should see MCP tools such as `awake`, `drill`,
`jot`, `cite`, `claim`, `task`, `export`, and `overview`.

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

See `contrib/opencode/README.md` for the exact files and merge behaviour.

### Remote Or Docker Daemons

If the daemon binds to anything other than loopback, remember:

- kaeru has no built-in auth; put it behind your own trusted network or proxy;
- set `KAERU_MCP_ALLOWED_HOSTS` to the hostnames or `host:port` values clients
  use, otherwise the streamable HTTP transport rejects non-loopback `Host`
  headers.

Example:

```bash
KAERU_MCP_LISTEN_ADDRESS=0.0.0.0
KAERU_MCP_ALLOWED_HOSTS=192.0.2.10:9876,kaeru.lan
```

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
- `search`, `drill`, `trace`, `between`, `tagged` for recall;
- `synthesise`, `settle`, `reopen`, `supersede` when knowledge changes shape.

## 4. Memory Layers

Current kaeru memory has two orthogonal axes:

- **Tier:** `operational` is working memory: observations, drafts, hypotheses,
  open questions, and tasks. `archival` is settled memory: references, ideas,
  outcomes, summaries, persona/entity records.
- **Layer:** `core`, `hot`, `warm`, `cold`, `frozen` describe how aggressively
  an item should be surfaced to future agents. New captures default to `warm`;
  the agent should keep truly central material small and explicit, and let stale
  or low-value material cool down instead of carrying everything into context.

When capturing, choose the verb by epistemic status, not by length. If the fact
is already settled, use `cite`; if it is still unfolding, use `episode` or
`claim`. After capturing, search for related nodes and link them, otherwise the
new node is easy to lose.

## 5. Migration To The Layered Model

The safe migration path is semantic, not a blind database rewrite:

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
old memory into the new tier/layer model and drop stale operational noise while
preserving settled knowledge and active work.
