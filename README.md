# 蛙 kaeru

`kaeru` is a cognitive memory layer for LLM agents — a typed graph that an agent can think in, plus a recollection layer for long-term ideas and outcomes.

Designed for **multi-session continuity**: when an agent opens a project, it has full context of what was being thought about, can follow provenance chains, and can consolidate outcomes into stable long-term knowledge.

Inspired by the LLM-wiki pattern (Karpathy, gist `442a6bf555914893e9891c11519de94f`), the bi-temporal knowledge graph approach of Graphiti / Zep, the curator-driven knowledge engine of Cognee, and the reasoning-based hierarchical-summary navigation pattern from PageIndex. Two-tier design grounded in the hippocampus / cortex split.

Name: 蛙 (*kaeru*, "frog"; homophonic with 帰る "to return" and 変える "to change") — the agent that returns, recalls, and reshapes.

## Overview

`kaeru` is built around a typed property graph stored in CozoDB. Two tiers, biological analogy:

- **cognitive (operational / hippocampus)** — high-velocity working graph where the agent actively thinks: episodes, scratch, drafts, hypotheses, experiments, audit events.
- **recollection (archival / cortex)** — settled ideas, outcomes, summaries, references; mostly read.

Every node and edge is **bi-temporal** — the substrate stores assertion / retraction history natively, so time-travel queries are out of the box and conflict resolution is non-destructive (the old version is invalidated, not deleted).

Per-initiative subgraphs through a junction-relation pattern: one substrate, many initiatives, multi-membership. An agent working on project A asks "what was I doing here last time?" and gets an answer scoped to A. The same node can belong to several initiatives at once.

`kaeru` is a **facilitator, not an enforcer**. The curator API exposes ~36 primitives (`awake`, `recall`, `drill`, `claim`, `synthesise`, `at`, `history`, `consolidate_out`, …) as available tools. The agent and user choose when to invoke them; the CLI hints but doesn't block.

## Architecture Notes

- **Substrate is CozoDB** with RocksDB backend; bi-temporal `Validity` is native to the substrate, not bolted on.
- **Edges carry operational semantics** — each edge type is something the curator API responds to. `derived_from` powers provenance and explainability; `contradicts` triggers a non-destructive `under_review` flow; `supersedes` retracts the previous version through the bi-temporal substrate. Edges are not just associations.
- **`audit_event` is a first-class node type** — every mutation writes an audit node, so changes to memory themselves become reasoning surface for the agent. Substrate-level history (`Validity`) and operational audit (audit-event nodes) stay separate: the substrate tracks *what was*, the audit nodes track *who did it and why*.
- **Per-initiative scope through junction relations** rather than column filtering — RocksDB prefix-scan gives O(log n + k) on the active initiative.
- **Retrieval is structural-first** — explicit name lookup, typed graph traversal, summary views. Cozo FTS for fuzzy fallback when an exact name is forgotten. Vector embeddings are not the primary mode.
- **Two-tier with explicit `consolidate_out`** — operational drafts get promoted to archival as a deliberate, logged operation. Provenance (`derived_from`) survives the tier boundary.
- **Single binary, embedded substrate** — `kaeru` runs in-process with the agent. No server, no network. Vault on disk under a platform-specific default (Linux `$XDG_DATA_HOME/kaeru`, macOS `~/Library/Application Support/ai.lamantin.kaeru`, Windows `%LOCALAPPDATA%\ai.lamantin.kaeru`); override with `KAERU_VAULT_PATH`.

## Layout

```
kaeru/
├── Cargo.toml                  ← workspace root
├── kaeru-core/                 ← library: substrate, schema, primitives
├── kaeru-mcp/                  ← binary `kaeru-mcp`: Model Context Protocol server (the agent's surface)
├── kaeru-cloud/                ← binary `kaeru-cloud`: shared cloud tier (Axum REST over kaeru-core)
└── skills/
    └── kaeru-skill/            ← portable agent skill (Claude Code / etc.)
```

Future integration crates: `kaeru-langchain` (Python bridge), `kaeru-rig` (Rig framework). Not yet started.

## Install

> **Pre-1.0 alpha.** Substrate schema may change between minor versions —
> export to markdown if you need to keep notes around.

See [QUICK_START.md](QUICK_START.md) for source builds, MCP daemon setup, and the re-entry ritual.

## Quick tour (MCP tools)

```
# See what projects exist:
initiatives

# Re-entry ritual: process state + epistemic state.
awake (initiative: "auth-rewrite")
overview (initiative: "auth-rewrite")

# Capture (auto-named):
jot (initiative: "auth-rewrite", body: "noticed token expiry differs across platforms")

# Fuzzy lookup when you forgot the exact name:
search (initiative: "auth-rewrite", query: "expiry")

# Drill into something:
drill (initiative: "auth-rewrite", name: "noticed-token-expiry-differs-across-...")

# Hypothesis cycle:
claim (initiative: "auth-rewrite", body: "platform-aware policy is correct", about: "<node>")
test (initiative: "auth-rewrite", hypothesis: "<hyp>", method: "compared iOS / Android TTL")
confirm (initiative: "auth-rewrite", hypothesis: "<hyp>", by: "<experiment>")

# Time-travel:
at (initiative: "auth-rewrite", name: "<name>", when: "5m")
history (initiative: "auth-rewrite", name: "<name>")

# Snapshot to an Obsidian-friendly markdown vault:
export (initiative: "auth-rewrite", path: "/tmp/auth-snapshot")
```

## Connecting to an MCP-aware agent

`kaeru-mcp` is a long-lived HTTP service: **one daemon per machine** owns the substrate, any number of agent sessions (Claude Code, Opencode, Cursor, …) connect concurrently. This is intentional — RocksDB is single-writer, so a stdio MCP that forks a subprocess per session would hit lock contention. See `kaeru-mcp/README.md` for systemd / launchd unit templates and the full HTTP config.

Run the daemon (or set up the systemd user unit from `contrib/install/`):

```bash
kaeru-mcp                                            # foreground, Ctrl-C to stop
```

Then point your agent at it:

- **Claude Code**: `claude mcp add --transport http kaeru http://127.0.0.1:9876/mcp` — see `skills/kaeru-skill/` for the portable system-prompt rules.
- **Opencode**: `bash contrib/opencode/install-opencode.sh` — wires the daemon, drops `AGENTS.kaeru.md` rules into `~/.config/opencode/`, and installs `/kaeru` / `/lesson` / `/recall` slash commands. Designed to coexist with your existing OSS-model provider config (Qwen / DeepSeek / GLM / Ollama). See `contrib/opencode/README.md`.
- **Cursor and other runtimes**: paste the body of `skills/kaeru-skill/SKILL.md` into your agent's rules / system-prompt section. For MCP-aware clients the daemon URL above works directly.

After restart the agent sees tools like `awake`, `drill`, `claim`, `at` natively. Each tool takes an optional `initiative` parameter.

## Local & cloud — sharing memory across a team

`kaeru` runs **local-first**: your vault lives on your machine and nothing leaves it by default. A second, optional tier — `kaeru-cloud` — is a shared store for a trusted group (a team, a family). Each initiative carries a sticky `share_policy`:

- `private` (default) — nothing ever leaves; personal projects.
- `team` — nodes you explicitly mark `shared` may sync to the cloud.

Sharing is never automatic and passes two gates: the initiative policy, and a deterministic **pre-share secret guard** that blocks anything looking like an API key, token, or private key. The guard is silent on clean content and only interrupts on a real hit.

Verbs (over MCP): `policy` (mark an initiative `team`), `share` (push a node), `cloud_recall` (see what the team has), `pull` (bring a shared node into your local graph), `link_cloud` / `cloud_links` (reference cloud nodes without copying), and `sync_review` (batch-review still-local nodes). Capture verbs (`episode` / `jot` / `cite`) take `visibility: shared` to capture-and-share in one call.

Per-user / per-org isolation (multi-tenant) is a future addition; today the cloud is one shared space scoped by initiative. See [`kaeru-cloud/README.md`](kaeru-cloud/README.md).

## Status

Pre-1.0. The substrate, curator API, MCP server, shared cloud tier (sharing & recall), markdown export, and bi-temporal handle are implemented; the test suite is green. What still needs hardening:

- Cloud is one shared space scoped by initiative; per-user / per-org multi-tenant isolation is not built yet.

- Concurrency story for the MCP server when an agent batch-fires many calls — currently each call is atomic but reads can race ahead of pending writes.
- Audit events are not yet attached to the initiative junction (their `LOG.md` filtering in export is by `affected_refs` intersection, which works but is a workaround).
- No integration adapters for LangChain / Rig yet.
- Whole-second `Validity` resolution means `link` followed by an immediate `unlink` against the same edge in the same second resolves ambiguously. Test code sleeps between operations; interactive use is fine because human pacing always crosses the boundary.

## Contributing

Discussion and design feedback through issues. PRs welcome on the open items above and anywhere the agent-facing surface feels rough — the verb taxonomy (`awake`, `drill`, `claim`, `flag`, `settle`, …) is meant to map to natural agent thinking, not just expose graph operations.

## License

MIT — see `LICENSE`.
