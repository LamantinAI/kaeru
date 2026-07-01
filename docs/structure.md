# Code structure

A map of the workspace and where each concern lives. `kaeru-core` is the source
of truth for shared types; the adapter crates consume it and never duplicate
types or issue raw Cozo queries.

## Workspace

```
kaeru/
├── Cargo.toml            ← workspace root; shared deps under [workspace.dependencies],
│                           single [workspace.package] version inherited by all crates
├── kaeru-core/           ← library: substrate, schema, curator primitives
├── kaeru-mcp/            ← binary `kaeru-mcp`: MCP server (the agent's surface)
├── kaeru-cloud/          ← binary `kaeru-cloud`: shared cloud tier (Axum REST)
├── kaeru-rig/            ← library: rig framework adapter (embedded, no cloud HTTP)
├── kaeru-viz/            ← (non-crate) browser knowledge-graph visualizer (Vite + three.js)
└── skills/kaeru-skill/   ← (non-crate) portable agent skill for distribution
```

Targets Rust edition 2024. `kaeru-viz` and `skills/` are not built by Cargo.

## kaeru-core — the library

The one place graph logic lives. Read/write always goes through these
primitives.

```
kaeru-core/src/
├── config.rs             ← KaeruConfig (env-driven, KAERU_* overlay)
├── store.rs              ← Store: in-memory + on-disk constructors; use_initiative / scoped (scope guard)
├── session.rs            ← awake, pin / unpin, active_window  (re-entry bundle: layered + cortex)
├── export.rs             ← Obsidian-friendly markdown snapshot
├── export_json.rs        ← whole-graph JSON export (feeds kaeru-viz / /graph.json)
├── migrate.rs            ← forward-only migration_journal
├── guard.rs              ← deterministic pre-share secret guard
├── graph/                ← SCHEMA layer
│   ├── node.rs           ← NodeId, NodeType, Tier, Layer, EpisodeKind, Significance, Visibility …
│   ├── edge.rs           ← EdgeType (+ FromStr)
│   ├── temporal.rs       ← at, history, parse_validity, validity_seconds
│   └── audit.rs          ← write_audit (private to mutations)
├── recall/               ← READ side
│   ├── by_name.rs        ← recall_id_by_name (+ _global), node_brief_by_id, read_node_full
│   ├── layered.rs        ← recall_by_layer / _in_tier  (the layer + tier split awake uses)
│   ├── path.rs           ← shortest_path, chains_of, read_chain
│   ├── reflect.rs        ← reflect → ReflectionReport (the maintenance work-list)
│   ├── lint.rs           ← orphans + unresolved reviews
│   ├── overview.rs       ← terminal-readable subgraph map
│   ├── fts.rs            ← fuzzy_recall via Cozo FTS
│   ├── between.rs, walk.rs, recent.rs, tagged.rs, recollect.rs,
│   ├── summary_view.rs, under_review.rs, initiatives.rs
│   └── mod.rs            ← NodeBrief / NodeFull + parse_brief
└── mutate/               ← WRITE side (each writes an audit_event)
    ├── episode.rs        ← write_episode, jot
    ├── cite.rs           ← Reference node (URL in properties)
    ├── edge.rs           ← link / unlink / set_edge_weight
    ├── chain.rs          ← create_chain, regenerate_chain, extend_chain (+ dedup)
    ├── initiative.rs     ← rename_initiative, delete_initiative, attach_node
    ├── hypothesis.rs     ← formulate / run_experiment / update_status
    ├── review.rs         ← mark_resolved / mark_under_review
    ├── synthesise.rs     ← synthesise (converge seeds → durable insight)
    ├── consolidate.rs    ← consolidate_out / consolidate_in (tier promotion)
    ├── supersedes.rs, metabolism.rs (forget / improve), layer.rs, task.rs
    └── mod.rs            ← shared helpers (now_validity_seconds, attach_node_to_initiative, …)
```

Module-organisation rule of thumb: a flat file becomes a `mod.rs`-with-submodules
directory once it passes ~300–400 lines or accumulates more than three cohesive
primitives. Shared cross-submodule types live in the parent `mod.rs`.

## kaeru-mcp — the daemon

Each curator verb is one `#[tool]` method on `KaeruServer`; output is
`Content::text(...)`. Cloud-facing tools call `kaeru-cloud` via `cloud_client.rs`.

```
kaeru-mcp/src/
├── main.rs               ← tokio + tracing; builds Store + CloudRegistry; Accept-normalizing layer
├── settings.rs           ← KaeruMcpConfig (KAERU_MCP_* env + clouds.toml)
├── server.rs             ← KaeruServer + #[tool_router]; one #[tool] per verb + the agent instructions
├── params.rs             ← Parameters<T> structs the tools deserialize
├── utils.rs              ← output builders + input parsing (with_initiative, ts_suffix, CAPTURE_NUDGE …)
├── cloud_client.rs       ← async reqwest client + CloudRegistry (named multi-cloud)
└── tools/                ← one module per verb group: capture, chain, session, lookup,
                             cloud, initiative, hypothesis, lint (lint + reflect), review, …
```

## kaeru-cloud — the shared tier

```
kaeru-cloud/src/
├── main.rs               ← thin entrypoint (config + tracing → run)
├── lib.rs                ← run(): build state, bind, serve
├── config.rs             ← KaeruCloudConfig (KAERU_CLOUD_* env)
└── api/                  ← state.rs, extractors.rs (bearer), errors.rs (IntoResponse), router/
```

TLS is terminated by a reverse proxy — the service speaks plain HTTP. It wraps
the same `kaeru-core`; no separate persistence path.

## kaeru-rig — the framework adapter

`lib.rs` defines a `KaeruMemory` handle over an embedded `Arc<Store>` and the
`mem_tool!` macro that generates each verb as a `rig::tool::Tool`. Verb bodies
are split across `capture.rs`, `chains.rs`, `lookup.rs`, `manage.rs` (session /
initiative / diagnostics), `reason.rs`, `evolve.rs`. Every call runs inside
`Store::scoped(<memory initiative>)` on a `spawn_blocking` pool.

## Error model

No `anyhow`. `kaeru-core` defines an `Error` thiserror enum (`Substrate`,
`SchemaBootstrap`, `Invalid`, `NotFound`, `Io`, `Config`) and `Result<T>`.
Substrate errors funnel through `From<cozo::Error>`; adapters surface
`kaeru-core::Result` directly without re-wrapping.
