# CLAUDE.md — kaeru

## Project Summary

`kaeru` is a cognitive memory layer for LLM agents — a typed property graph stored in CozoDB with a bi-temporal substrate, a curator API as the operational layer, and a two-tier (cognitive / recollection) design grounded in the hippocampus / cortex split.

The workspace targets Rust `1.95+` with edition `2024` and uses a shared dependency setup from the root `Cargo.toml`.

Workspace members:

- `kaeru-core/` — library crate: substrate, schema, primitives.
- `kaeru-cli/` — binary crate `kaeru`: CLI surface adapter.
- `kaeru-mcp/` — binary crate `kaeru-mcp`: Model Context Protocol server (rmcp 1.6, stdio transport).

There is also a non-crate `skills/kaeru-skill/` directory with a portable
agent skill (SKILL.md frontmatter + body). It's source for distribution
(install instructions in its README); not built by Cargo.

## How To Work In This Repository

Before changing code, orient yourself by crate and responsibility:

1. Identify the correct workspace member first — `kaeru-core`, `kaeru-cli`, or `kaeru-mcp`.
2. Substrate, schema, primitives, curator API logic — `kaeru-core/`.
3. CLI argument parsing, terminal output, command dispatch — `kaeru-cli/`.
4. MCP tool definitions and rmcp wiring — `kaeru-mcp/`. Each verb is one `#[tool]` method on `KaeruServer`, output is `Content::text(...)` of a CLI-equivalent rendering.
5. Shared dependencies live in the root `Cargo.toml` under `[workspace.dependencies]`. Add new deps there first; pull them into a crate with `dep.workspace = true`.
6. Treat `kaeru-core` as the source of truth for shared types. Adapter crates (`kaeru-cli`, `kaeru-mcp`, future `kaeru-langchain`, `kaeru-rig`) consume `kaeru-core`; do not duplicate types.
7. When adding or renaming a curator-API verb, update both `kaeru-cli`'s subcommand + handler AND the matching `#[tool]` in `kaeru-mcp/src/server.rs`. They expose the same surface.

## Local Runbook

- `cargo check --workspace` — quick type-check after changes that touch more than one crate.
- `cargo test --workspace` — run tests across the workspace.
- `cargo run --bin kaeru` — run the CLI.

The substrate stores its data under a platform-specific default path resolved at compile time (see `config::default_vault_path` for the cfg-gated branches): Linux `$XDG_DATA_HOME/kaeru` (fallback `$HOME/.local/share/kaeru`), macOS `$HOME/Library/Application Support/ai.lamantin.kaeru`, Windows `%LOCALAPPDATA%\ai.lamantin.kaeru`. Override via the `KAERU_VAULT_PATH` env var (auto-routed through `KaeruConfig::from_env`).

## Module Organisation

`kaeru-core` modules already use the `mod.rs`-with-submodules layout where they grew past flat:

```
kaeru-core/src/
├── config.rs               ← KaeruConfig (env-driven via `config` crate)
├── errors.rs
├── store.rs                ← Store: in-memory + disk constructors
├── session.rs              ← pin / unpin / active_window / awake
├── export.rs               ← Obsidian-friendly markdown snapshot
├── graph/                  ← schema layer
│   ├── mod.rs
│   ├── audit.rs            ← write_audit (private to mutations)
│   ├── edge.rs             ← EdgeType (+ FromStr)
│   ├── node.rs             ← NodeId, NodeType, Tier, EpisodeKind, …
│   └── temporal.rs         ← at, history, parse_validity
├── recall/                 ← read-side primitives
│   ├── mod.rs              ← NodeBrief, parse_brief, truncate_excerpt
│   ├── by_name.rs          ← recall_id_by_name, count_by_type, node_brief_by_id
│   ├── walk.rs
│   ├── recent.rs
│   ├── under_review.rs
│   ├── recollect.rs        ← recollect_idea / outcome / provenance
│   ├── summary_view.rs
│   ├── lint.rs
│   ├── overview.rs         ← terminal-readable subgraph map
│   ├── fts.rs              ← fuzzy_recall via Cozo FTS
│   ├── initiatives.rs      ← list_initiatives
│   ├── between.rs          ← edges between two nodes (both directions)
│   └── tagged.rs           ← read by tag
└── mutate/                 ← write-side primitives
    ├── mod.rs              ← shared helpers (now_validity_seconds, RMW reads, attach_node_to_initiative)
    ├── episode.rs          ← write_episode + jot
    ├── edge.rs             ← link / unlink
    ├── supersedes.rs
    ├── synthesise.rs
    ├── review.rs           ← mark_resolved / mark_under_review
    ├── hypothesis.rs       ← formulate / run_experiment / update_status
    ├── consolidate.rs      ← consolidate_out / consolidate_in
    ├── metabolism.rs       ← forget / improve
    └── cite.rs             ← Reference node with URL in properties JSON
```

```
kaeru-cli/src/
├── main.rs                 ← Cli + Command enum + dispatch
├── format.rs               ← print helpers
├── parse.rs                ← parse_duration_secs, parse_tier, derive_auto_name, resolve_name(_or_id)
└── commands/               ← one file per logical group, one fn per subcommand
    ├── mod.rs
    ├── session.rs          ← awake, overview, initiatives, recent, pin, unpin, config
    ├── capture.rs          ← episode, jot, link, unlink, cite, supersede
    ├── lookup.rs           ← recall, drill, trace, search, summary, ideas, outcomes, tagged, between
    ├── temporal.rs         ← at, history
    ├── hypothesis.rs       ← claim, test, confirm, refute
    ├── review.rs           ← flag, resolve
    ├── consolidate.rs      ← settle, reopen, synthesise
    ├── metabolism.rs       ← forget, revise
    ├── lint.rs
    └── vault.rs            ← export
```

```
kaeru-mcp/src/
├── main.rs                 ← stdio + tokio + tracing init
└── server.rs               ← KaeruServer + #[tool_router] with one #[tool] per verb (~36 tools)
```

Triggers to refactor a flat file into a `mod.rs`-with-submodules layout:

- the single file passes ~300–400 lines, or
- more than three cohesive primitives accumulate in one file, or
- shared internal types start appearing across what was one file.

Shared cross-submodule types live in the parent `mod.rs` (or in a `base.rs` if the helper surface is wide enough to deserve its own file).

## Important Import Rule

This repository has a strict import style:

- **Prefer direct imports** for structs, enums, functions, traits, and types at the top of the file. Import each used item by name.
- **Avoid fully qualified paths inside implementation code.** `Foo::method(...)` is fine when `Foo` is imported; `crate::module::Foo::method(...)` deep inside a function body is not.
- **No glob imports** (`use foo::*`) in implementation code. The only acceptable glob is from a deliberately curated `prelude` re-export — and even then, prefer named imports unless the prelude is genuinely the right interface.
- **No half-imports** — don't import `module` and then write `module::Type::method` throughout. Import `Type` directly.
- **If names collide**, import the parent modules and disambiguate with short module aliases (e.g. `use std::io; use tokio::io as tokio_io;`).
- **Group imports logically**: standard library, third-party crates, then local modules. Blank line between groups.
- **Inline paths are acceptable only** when normal imports cannot resolve the situation (rare).

In short: keep logic readable, keep imports explicit, and do not scatter long module paths through the code body.

## Development Notes

- `cargo check --workspace` after changes that span crates.
- When a type is used by both `kaeru-core` and `kaeru-cli` (or future adapter crates), define it in `kaeru-core` and re-export. Do not duplicate.
- Bi-temporal `Validity` is core to the design — when introducing a new stored relation in `kaeru-core`, decide explicitly whether `Validity` belongs in the PK. Most domain relations do (`node`, `edge`); junction relations do not.
- `audit_event` nodes are written automatically by every mutation primitive in `kaeru-core`. Do not bypass mutation primitives by writing to substrate directly from `kaeru-cli`.
- The project is a **facilitator, not an enforcer**. CLI commands hint when context is missing (e.g. no active initiative); they do not block. Cognitive primitives are available tools, not mandatory protocol. Do not introduce required call sequences.

## Backend Rules

- All graph reads and writes go through `kaeru-core` primitives — never raw Cozo queries from `kaeru-cli`.
- **No `anyhow`.** Errors are explicit: `kaeru-core` defines `Error` (thiserror enum) and `Result<T>`. Variants describe the failure mode (`Substrate`, `SchemaBootstrap`, `Invalid`, `NotFound`, `Io`, `Config`). Add new variants when a new failure mode arises; do not stuff context strings into existing ones.
- Substrate errors funnel through the `From<cozo::Error> for Error` impl in `errors.rs`. Don't `format!("{e}")` cozo errors at call sites — let `?` propagate.
- The CLI surfaces errors directly from `kaeru-core::Result`. No re-wrapping.
- The substrate is single-process embedded — no server, no network. Adapters wrap the in-process API; they do not expose a separate persistence path.

## Out Of Scope

- Vector embeddings as the primary recall mode — Cozo HNSW is available but kept as fallback for cold queries; structural retrieval is the main mode.
- Server / network mode — `kaeru` runs in-process. gRPC server-mode is a future concern, not part of the current architecture.
- Replacing PKM tools — `kaeru` is for **agent** memory; humans interact through CLI + derived markdown export, not a competing UI layer.
