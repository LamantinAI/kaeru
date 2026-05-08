# Quick Start

> **Pre-1.0 alpha.** The substrate schema may change between minor versions.
> Until 0.x → 1.0 stabilises, treat your vault as disposable — export to
> markdown if you want to keep notes around (`kaeru export <dir>`).

## 1. Install

### Prebuilt binary (recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/LamantinAI/kaeru/main/contrib/install/install.sh | bash
```

What this does:

1. Detects OS / arch, downloads the matching tarball from the latest GitHub release, unpacks `kaeru` and `kaeru-mcp` into `~/.local/bin`.
2. On macOS clears the Gatekeeper quarantine bit on both binaries (they're cross-built from Linux and unsigned).
3. Installs a **user-level** daemon — `~/.config/systemd/user/kaeru-mcp.service` on Linux, `~/Library/LaunchAgents/ai.lamantin.kaeru-mcp.plist` on macOS — and starts it. No `sudo` involved; no system-wide files touched.

Env knobs:
- `KAERU_INSTALL_DIR=/usr/local/bin` — change the binary destination.
- `KAERU_VERSION=v0.1.0` — pin a specific tag instead of `latest`.
- `KAERU_SETUP_DAEMON=no` — skip the daemon step (you'll run `kaeru-mcp` manually).

Currently shipped prebuilt targets:

| OS    | Arch    | Notes                                                           |
|-------|---------|-----------------------------------------------------------------|
| Linux | x86_64  | static (musl), runs on any glibc/musl host                      |
| macOS | aarch64 | Apple Silicon (M1/M2/M3); unsigned, installer clears Gatekeeper |

For Intel Mac or Linux ARM, build from source (below).

If `~/.local/bin` is not on your `PATH`, the installer reminds you:

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

### From source

Prerequisites: Rust 1.95+ (edition 2024). On Linux you also need `libclang-dev` for the RocksDB build.

```bash
git clone https://github.com/LamantinAI/kaeru.git
cd kaeru
cargo test --workspace
cargo install --path kaeru-cli
cargo install --path kaeru-mcp
```

## 2. Verify the CLI

```bash
kaeru --version
kaeru initiatives          # empty list on a fresh vault — that's fine
```

The substrate lives at a platform-specific default: Linux `$XDG_DATA_HOME/kaeru` (typically `~/.local/share/kaeru`), macOS `~/Library/Application Support/ai.lamantin.kaeru`. Override with `KAERU_VAULT_PATH=/path/to/vault`.

## 3. Daemon

`kaeru-mcp` is a long-lived HTTP service. **One daemon per machine** owns the substrate; any number of agent sessions (Claude Code, Cursor, …) connect concurrently. RocksDB is single-writer, so a stdio MCP that forks a subprocess per session would lose the lock race.

If you ran the prebuilt installer, the daemon is already up — verify:

```bash
# Linux
systemctl --user status kaeru-mcp
journalctl --user -u kaeru-mcp -f

# macOS
launchctl list | grep kaeru-mcp
tail -f ~/Library/Logs/kaeru-mcp.log
```

If you skipped daemon setup (`KAERU_SETUP_DAEMON=no`) or built from source, run it in the foreground:

```bash
kaeru-mcp
# kaeru-mcp listening — point MCP clients here   url=http://127.0.0.1:9876/mcp
```

Ctrl-C to stop. Manual unit-file recipes live in `kaeru-mcp/contrib/systemd/` and `kaeru-mcp/contrib/launchd/`.

## 4. Wire into Claude Code

```bash
claude mcp add --transport http kaeru http://127.0.0.1:9876/mcp
```

Restart your Claude Code session. The agent will see ~38 tools (`awake`, `drill`, `claim`, `at`, `cite`, …) — each takes an optional `initiative` parameter. Tool descriptions and the server's `instructions` field map out when to use what.

## 5. Re-entry ritual (every session)

```bash
# pick a project
kaeru initiatives

# process state — what was open
kaeru --initiative <name> awake

# epistemic state — what the project knows
kaeru --initiative <name> overview
```

From there: `jot` / `episode` for working observations, `cite <name> --body "..."` (URL optional) for settled documents (ADRs, specs, persona records), `claim` → `test` → `confirm`/`refute` for hypotheses, `task` / `done` for actionable todos. Inquire with `drill`, `trace`, `search`, `tagged`. Time-travel with `at`, `history`.

`kaeru --help` walks the typical workflow; `kaeru <command> --help` has full per-command docs.
