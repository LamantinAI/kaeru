# kaeru-mcp

Model Context Protocol server exposing the kaeru curator API as
native MCP tools. Drop it into any MCP-aware agent runtime; the agent
gets 36 tools — `awake`, `overview`, `jot`, `drill`, `claim`, `flag`,
`settle`, `at`, `history`, … — without any markdown-parsing layer.

## Build

```bash
cargo install --path kaeru-mcp
```

The binary lands at `~/.cargo/bin/kaeru-mcp`. It uses stdio transport;
logs go to stderr only (stdout is the JSON-RPC channel).

## Configuration

`kaeru-mcp` reads the same env vars as `kaeru-cli`:

| Variable                       | Effect                                              |
|--------------------------------|-----------------------------------------------------|
| `KAERU_VAULT_PATH`             | Override vault location                             |
| `KAERU_ACTIVE_WINDOW_SIZE`     | Soft cap on `awake` pinned set (default 15)         |
| `KAERU_RECENT_EPISODES_CAP`    | Soft cap on `recent` results (default 15)           |
| `KAERU_AWAKE_DEFAULT_WINDOW_SECS` | Default `awake` window in seconds (default 86400) |
| `KAERU_SUMMARY_VIEW_CHILDREN_CAP` | Soft cap on `drill` children (default 12)        |
| `KAERU_BODY_EXCERPT_CHARS`     | Excerpt truncation (default 240)                    |
| `KAERU_PROVENANCE_MAX_HOPS`    | Max hops for `trace` (default 5)                    |
| `KAERU_DEFAULT_MAX_HOPS`       | Default walk depth (default 2)                      |
| `KAERU_MAX_HOPS_CAP`           | Walk hard cap (default 3)                           |
| `RUST_LOG`                     | Log level (`info`, `debug`, …). Logs go to stderr.  |

## Connecting to Claude Code

Add to Claude Code's MCP server registry. Either via CLI:

```bash
claude mcp add kaeru -- kaeru-mcp
```

…or directly in the config file (`~/.claude/claude_desktop_config.json`
or `claude.json`, depending on platform):

```json
{
  "mcpServers": {
    "kaeru": {
      "command": "kaeru-mcp",
      "env": {
        "KAERU_VAULT_PATH": "/home/you/.local/share/kaeru"
      }
    }
  }
}
```

After restart, Claude sees the 36 tools natively. No CLI subprocess
overhead, no markdown parsing — direct in-process tool calls.

## Connecting to other MCP runtimes

Anything that speaks MCP over stdio: Cursor, Continue, Goose, Cline,
mcp-inspector, etc. Format is the same — point the runtime at the
`kaeru-mcp` binary.

For debugging, the official inspector is the easiest:

```bash
npx @modelcontextprotocol/inspector kaeru-mcp
```

## Initiative discipline (read first)

Every tool call accepts an optional `initiative` parameter. Pass the
project name on each call once you know which project you're working
on. Without `initiative`, mutations are un-tagged and reads are
cross-initiative — almost never what you want.

The agent's standard re-entry ritual:

1. `initiatives` — list known projects.
2. `awake` with `initiative=<name>` — what was open last time.
3. `overview` with `initiative=<name>` — what the project knows.
4. Then capture / inquire / reason with `initiative=<name>` on each call.

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
diagnostics        : lint
snapshot           : export
```

Every tool has a `description` field that the agent reads via
`tools/list`; explore in the inspector to see the full schemas.

## Concurrency model

Stdio MCP processes one tool call at a time at the protocol level,
but `rmcp` may dispatch handlers concurrently inside the server. For
a single agent making sequential tool calls (the normal pattern),
this is fine — each call sees the substrate state at the moment its
handler runs. If you batch-fire many calls without waiting, expect
out-of-order responses and possible read-after-write surprises.

## Versioning

`kaeru-mcp` rides the workspace version. Tool descriptions and schemas
follow the curator API in `kaeru-core`; when a verb is added or
renamed there, this server picks it up at the next rebuild. There's
no MCP-side migration story yet — agents reading `tools/list` always
get the current shape.
