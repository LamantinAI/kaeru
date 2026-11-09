# Code structure

The Rust crates share dependencies through the workspace manifest. Core
owns local memory semantics. MCP and Rig expose those semantics to clients;
memory-service hosts the shared runtime plus the compatibility REST adapter. The browser visualizer
is compiled separately, then embedded through kaeru-runtime into both executables;
Cargo-only installation uses the checked-in generated bundle. The portable skill
remains a separate distribution.

Platform authorization is a separate boundary: `kaeru-cloud/contracts` owns neutral
exchange models and stable action IDs; `kaeru-cloud/tokenizer-service` owns its
PostgreSQL schema, pure role evaluator, optional Valkey cache and HTTP/2+CBOR
adapter. It does not link Core or open a knowledge store. See its README for
one-time credential and consistency contracts.

## Follow one call

A local call starts at an adapter, validates against the catalogue in
`kaeru-core::api`, resolves authorized references, then reads the graph or runs
a command. The adapter returns the shared result. Optional network effects run
after the local commit and report their own failures.

| Concern | Start here |
| --- | --- |
| Names, input shapes and descriptions | `kaeru-core/src/api/surface.rs` |
| Result types and host capabilities | `kaeru-core/src/models/response.rs`, `records.rs`, `access.rs` |
| Validation, dispatch and replay | `kaeru-core/src/api/mod.rs` |
| Scope, visibility and reference resolution | `kaeru-core/src/api/access.rs` |
| Context, search, paging and history | `kaeru-core/src/api/read/` |
| Capture, updates and administration | `kaeru-core/src/api/write/` |
| Transaction ownership and rollback | `kaeru-core/src/store/database.rs` |
| Identity transfer and preserved references | `kaeru-core/src/mutate/identity.rs` |
| Stored schema and forward migrations | `kaeru-core/src/store.rs`, `kaeru-core/src/migrate.rs` |
| Archive rendering | `kaeru-core/src/archive/`, `kaeru-core/src/models/archive.rs` |
| Portable knowledge export/import | `kaeru-core/src/amg/`, `kaeru-core/src/models/amg.rs`, `kaeru-core/src/config/amg.rs` |
| Domain storage primitives | `kaeru-core/src/graph/`, `mutate/`, `recall/` |

The domain primitives remain available to core and cloud code. The retired MCP
and Rig tool modules are removed; they are not a compatibility API.

## Adapters and clients

| Component | Responsibility |
| --- | --- |
| `kaeru-runtime/src/server.rs` | MCP registration, discovery and server instructions. |
| `kaeru-runtime/src/canonical.rs` | Local dispatch and asynchronous supplements. |
| `kaeru-runtime/src/cloud.rs` | The nine existing cloud bridge operations. |
| `kaeru-runtime/src/api/` | Named HTTP routes, principal and egress policy. |
| `kaeru-runtime/src/ui/` | Embedded assets, browser routes and thin account screens. |
| `kaeru-runtime/src/exports/` | Bounded generated-only MCP archive resources. |
| `kaeru-local/src/cli/` | Offline AMG and local authentication setup commands. |
| `kaeru-runtime/src/auth/` | JWT/password verification and atomic local account persistence. |
| `kaeru-runtime/ui-dist/` | Generated, verified static frontend bundled by Cargo. |
| `kaeru-rig/src/tools.rs` | Distinct `kaeru_<operation>` tools over the shared catalogue. |
| `kaeru-rig/src/cloud/` | Embedded asynchronous cloud bridge. |
| `kaeru-cloud/memory-service/src/api/` | Memory-service compatibility REST handlers. |
| `kaeru-viz/src/api.js` | Browser transport and continuation handling. |
| `skills/kaeru-skill/` | Portable instructions for agents using memory. |
| `contrib/hooks/kaeru-first/` | Optional hook that checks memory before asking a user. |

Adapters construct host capabilities and handle transport or network concerns.
Local validation, result types and mutations belong in core, so adding a second
transport does not create a second implementation of memory behavior.

## Where to review the guarantees

The new integration tests call the public API and check complete workflows.
Unit tests beside domain code cover individual storage and adapter behaviors.
`tests/invariants.rs` predates the API redesign and remains in place.

| Test file in `kaeru-core/tests/` | What it checks |
| --- | --- |
| `api_workflows.rs` | Board/deadline semantics, review evidence, scoped reads, paging and delivery. |
| `atomic_commands.rs` | Command rollback and transactional behavior. |
| `canonical_api.rs` | Shared definitions, validation and canonical results. |
| `identity_and_history.rs` | Content preservation, identity transfer and historical access. |
| `agent_exports.rs` | Live two-format export, egress, no-clobber filesystem behavior and replay. |
| `amg.rs` | Scoped export, atomic conflict/force import, semantic metadata and recorded history. |
| `vault_migration.rs` | Opening older disk vaults without losing recorded data. |

MCP tests include transport authorization and parity with Rig. Rig tests exercise
embedded calls and cloud supplements. Browser transport tests are in
`kaeru-viz/test/`; hook tests are beside the hook implementation. Passing these
checks establishes implementation behavior, not whether an agent remembers to
consult memory. That requires a separate quality benchmark.
