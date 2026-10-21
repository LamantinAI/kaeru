# Documentation

For installation and client configuration, start with [QUICK_START.md](../QUICK_START.md).
For the product overview, see [README.md](../README.md).

## Reading the API and implementation

1. [Memory API](curator-api.md) explains the 35 operations, shows an ordinary
   workflow, and describes scope, partial results, paging and history.
2. [Architecture](architecture.md) explains storage, retrieval, transactions and
   the boundaries between core, transports and cloud.
3. [Code structure](structure.md) points to the modules and tests to inspect
   when reviewing a particular behavior.
4. [Task board](board.md) describes status columns, completion and deadlines.
5. [Instance UI and authentication](instance-ui.md) covers local accounts, access modes and containers.
6. [AMG knowledge transfer](amg.md) covers scoped CBOR export, atomic import and force.
7. [Offline builds](offline-build.md) covers vendored dependencies and toolchains.
8. [Read performance](read-performance.md) explains batching, cancellation and disposable benchmarks.

Kaeru stores typed records and relationships in an embedded CozoDB graph. Each
project is an initiative, and a record can belong to several initiatives.
Operational and archival tiers separate ongoing work from settled knowledge;
recall layers control how eagerly records return to context. Recorded versions
preserve earlier content. Some metadata remains current, so historical reads do
not reconstruct every past administrative state.

Agents use named operations over MCP, HTTP or embedded Rig tools. Retrieval
combines project context, lexical search, typed links and saved reasoning trails.
There is currently no embedding index or query language for agents to learn.
The API makes these capabilities available without prescribing a mandatory
sequence of calls.
