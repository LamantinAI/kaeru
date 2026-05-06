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
├── kaeru-cli/                  ← binary `kaeru`: CLI surface
├── kaeru-mcp/                  ← binary `kaeru-mcp`: Model Context Protocol server
└── skills/
    └── kaeru-skill/            ← portable agent skill (Claude Code / etc.)
```

Future integration crates: `kaeru-langchain` (Python bridge), `kaeru-rig` (Rig framework). Not yet started.

## Building from source

Prerequisites: Rust 1.95+ (edition 2024). On Linux you'll also need `libclang-dev` for the RocksDB build.

```bash
git clone <repo>
cd kaeru

# Build everything; runs the test suite (43 tests).
cargo test --workspace

# Install the two binaries to ~/.cargo/bin
cargo install --path kaeru-cli
cargo install --path kaeru-mcp
```

## Quick tour (CLI)

```bash
# See what projects exist:
kaeru initiatives

# Re-entry ritual: process state + epistemic state.
kaeru --initiative auth-rewrite awake
kaeru --initiative auth-rewrite overview

# Capture (auto-named):
kaeru --initiative auth-rewrite jot 'noticed token expiry differs across platforms'

# Fuzzy lookup when you forgot the exact name:
kaeru --initiative auth-rewrite search "expiry"

# Drill into something:
kaeru --initiative auth-rewrite drill noticed-token-expiry-differs-across-...

# Hypothesis cycle:
kaeru --initiative auth-rewrite claim "platform-aware policy is correct" --about <node>
kaeru --initiative auth-rewrite test <hyp> --method "compared iOS / Android TTL"
kaeru --initiative auth-rewrite confirm <hyp> --by <experiment>

# Time-travel:
kaeru --initiative auth-rewrite at <name> --when 5m
kaeru --initiative auth-rewrite history <name>

# Snapshot to an Obsidian-friendly markdown vault:
kaeru --initiative auth-rewrite export /tmp/auth-snapshot
```

`kaeru --help` walks through the typical workflow; `kaeru <command> --help` has full per-command docs.

## Connecting to an MCP-aware agent

`kaeru-mcp` exposes the same 36 verbs as native MCP tools — no shell-out, no markdown-of-CLI-output parsing. See `kaeru-mcp/README.md` for full setup; quick version:

```bash
claude mcp add kaeru -- kaeru-mcp
```

After restart the agent sees tools like `awake`, `drill`, `claim`, `at` natively. Each tool takes an optional `initiative` parameter.

For agent runtimes without MCP, the portable skill at `skills/kaeru-skill/` teaches them how to shell out to `kaeru-cli` instead — same workflow, slower transport.

## Status

Pre-1.0. The substrate, curator API, CLI, MCP server, markdown export, and bi-temporal handle are implemented; the test suite is green. What still needs hardening:

- Concurrency story for the MCP server when an agent batch-fires many calls — currently each call is atomic but reads can race ahead of pending writes.
- Audit events are not yet attached to the initiative junction (their `LOG.md` filtering in export is by `affected_refs` intersection, which works but is a workaround).
- No integration adapters for LangChain / Rig yet.
- Whole-second `Validity` resolution means `link` followed by an immediate `unlink` against the same edge in the same second resolves ambiguously. Test code sleeps between operations; interactive use is fine because human pacing always crosses the boundary.

## Contributing

Discussion and design feedback through issues. PRs welcome on the open items above and anywhere the agent-facing surface feels rough — the verb taxonomy (`awake`, `drill`, `claim`, `flag`, `settle`, …) is meant to map to natural agent thinking, not just expose graph operations.

## License

MIT — see `LICENSE`.
