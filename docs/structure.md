# Code structure

The four Rust crates share dependencies through the workspace manifest. Core
owns local memory semantics. MCP and Rig expose those semantics to clients;
cloud provides the existing shared-memory REST service. The browser visualizer
is compiled separately, then embedded into the MCP binary;
Cargo-only installation uses the checked-in generated bundle. The portable skill
remains a separate distribution.

## Follow one call

A local call starts at an adapter, validates against the catalogue in
`kaeru-core::api`, resolves authorized references, then reads the graph or runs
a command. The adapter returns the shared result. Optional network effects run
after the local commit and report their own failures.

| Concern | Start here |
| --- | --- |
| Names, input shapes and descriptions | `kaeru-core/src/api/surface.rs` |
| Result types and host capabilities | `kaeru-core/src/api/types.rs` |
| Validation, dispatch and replay | `kaeru-core/src/api/mod.rs` |
| Scope, visibility and reference resolution | `kaeru-core/src/api/access.rs` |
| Context, search, paging and history | `kaeru-core/src/api/read/` |
| Capture, updates and administration | `kaeru-core/src/api/write/` |
| Transaction ownership and rollback | `kaeru-core/src/store/database.rs` |
| Identity transfer and preserved references | `kaeru-core/src/mutate/identity.rs` |
| Stored schema and forward migrations | `kaeru-core/src/store.rs`, `kaeru-core/src/migrate.rs` |
| Domain storage primitives | `kaeru-core/src/graph/`, `mutate/`, `recall/` |

The domain primitives remain available to core and cloud code. The retired MCP
and Rig tool modules are removed; they are not a compatibility API.

## Adapters and clients

| Component | Responsibility |
| --- | --- |
| `kaeru-mcp/src/server.rs` | MCP registration, discovery and server instructions. |
| `kaeru-mcp/src/canonical.rs` | Local dispatch and asynchronous supplements. |
| `kaeru-mcp/src/cloud.rs` | The nine existing cloud bridge operations. |
| `kaeru-mcp/src/api/` | Named HTTP routes, principal and egress policy. |
| `kaeru-mcp/src/ui/` | Embedded assets, browser routes and thin account screens. |
| `kaeru-mcp/src/auth/` | JWT/password verification and atomic local account persistence. |
| `kaeru-mcp/ui-dist/` | Generated, verified static frontend bundled by Cargo. |
| `kaeru-rig/src/tools.rs` | Distinct `kaeru_<operation>` tools over the shared catalogue. |
| `kaeru-rig/src/cloud/` | Embedded asynchronous cloud bridge. |
| `kaeru-cloud/src/api/` | Standalone cloud REST handlers. |
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
| `vault_migration.rs` | Opening older disk vaults without losing recorded data. |

MCP tests include transport authorization and parity with Rig. Rig tests exercise
embedded calls and cloud supplements. Browser transport tests are in
`kaeru-viz/test/`; hook tests are beside the hook implementation. Passing these
checks establishes implementation behavior, not whether an agent remembers to
consult memory. That requires a separate quality benchmark.
