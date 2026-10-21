# Opencode wiring for kaeru

Drop-in glue that gives [Opencode](https://opencode.ai) the same kaeru
experience Claude Code gets — MCP tools, behaviour rules in the system
prompt, and `/kaeru` / `/lesson` / `/recall` slash commands.

This directory does **not** install the `kaeru-mcp` daemon — that's
[`contrib/install/install.sh`](../install/install.sh)'s job. The daemon
needs to be running before opencode can connect.

## What's in here

| File | Lands at | Purpose |
| --- | --- | --- |
| `AGENTS.kaeru.md` | `~/.config/opencode/AGENTS.kaeru.md` | Behaviour rules — verb taxonomy, cardinal rules, cadence habits. Loaded into the system prompt via `instructions`. |
| `opencode.kaeru.json` | merged into `~/.config/opencode/opencode.json` | Two-key additive snippet: `mcp.kaeru` (points at the daemon’s legacy `/sse` endpoint; Kaeru also serves streamable HTTP at `/mcp`) + `instructions` (includes the AGENTS file). No providers, no API keys. |
| `commands/kaeru.md`  | `~/.config/opencode/commands/kaeru.md`  | `/kaeru`  — re-entry ritual (`initiatives` → `context`). |
| `commands/lesson.md` | `~/.config/opencode/commands/lesson.md` | `/lesson` — capture a settled lesson via `note` or `reference`. |
| `commands/recall.md` | `~/.config/opencode/commands/recall.md` | `/recall` — lexical lookup, read the top hit. |
| `install-opencode.sh` | runs from the repo | One-shot installer. |

## Install

From a kaeru repo checkout, with `jq` and the daemon already running:

```bash
bash contrib/opencode/install-opencode.sh
```

The installer:

1. Copies the AGENTS file and three command files into
   `~/.config/opencode/` (user-owned — no `sudo`).
2. Looks at `~/.config/opencode/opencode.json`:
   - **Symlink to `/etc/opencode/opencode.json`** (common on shared
     hosts): prints the exact `jq` + `sudo` merge command to run by
     hand. The installer does **not** elevate itself.
   - **Regular user-owned file**: jq-merges in place, saves the
     previous content as `opencode.json.bak.kaeru-add`.
   - **Missing**: writes the snippet as a new file.
3. Probes the daemon at `http://127.0.0.1:9876/mcp` and warns if it's
   not reachable.

`KAERU_MCP_URL=...` overrides the installer’s reachability probe only. For a
nonstandard endpoint, edit `mcp.kaeru.url` in the installed config; the copied
snippet itself uses `http://localhost:9876/sse`. Add a bearer header there if
MCP authentication is enabled. Existing `mcp.kaeru` entries are not overwritten.

## After install

Restart opencode. The agent should now have:

- The 35 memory operations, shown with the client’s server prefix, such as
  `kaeru_context`, `kaeru_read` and `kaeru_note`.
- The full kaeru behaviour rules in every session's system prompt.
- `/kaeru`, `/lesson`, `/recall` as native slash commands.

Smoke test in a kaeru project directory:

```
/kaeru                     # should fire initiatives → context
/recall daemon             # search + read the top hit
/lesson "kaeru is wired into opencode now"
```

## Design notes

- **No provider block ships here.** You already maintain your own
  `opencode.json` with API keys and model choices. The repo's job is
  to add kaeru wiring, not opine on which OSS model you run.
- **No plugin.** Opencode loads `AGENTS.kaeru.md` deterministically
  into every system prompt — that's the SessionStart-equivalent
  reliability we need, without a TS/Bun dependency. A
  `session.created` plugin for cwd→initiative auto-derivation is a
  possible follow-up.
- **MCP tool naming is `kaeru_<verb>`** in opencode (it prefixes with
  the server name). This differs from Claude Code's
  `mcp__kaeru__<verb>`; the AGENTS file uses the opencode convention.
- **Transport selection.** The shipped snippet selects legacy HTTP+SSE.
  Clients supporting streamable HTTP can use `/mcp` instead; clients needing
  stdio can run `kaeru-mcp --stdio`. All reach the same daemon/vault.
- **`localhost` instead of `127.0.0.1`.** Opencode's MCP client uses
  Node/Bun `fetch`, which honours `HTTP_PROXY` / `http_proxy` env vars.
  On hosts behind a corporate proxy, requests to `127.0.0.1` get
  intercepted by the proxy (which can't reach the loopback) and come
  back as `SSE error: Non-200 status code (500)`. `localhost` is
  conventionally in `NO_PROXY`, so the request bypasses the proxy and
  reaches the daemon directly. If your `NO_PROXY` doesn't include
  `localhost`, add it — or set the URL to `127.0.0.1` only after
  checking that `fetch` won't go through the proxy in your shell.
