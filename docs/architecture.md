# Architecture

`kaeru` is a cognitive memory layer for LLM agents: a typed property graph over
CozoDB with a bi-temporal substrate, a two-tier cognitive design, and a curator
API that agents drive. This document explains the moving parts and the reasons
behind them.

---

## 1. Substrate and bi-temporality

The store is a single embedded [CozoDB](https://cozodb.org) instance with the
RocksDB backend, running **in-process** with the agent's daemon — no server, no
network hop to the data.

Every domain relation is **bi-temporal**. The primary key of the `node` and
`edge` relations includes a `Validity` — a `(timestamp, is_assert)` pair native
to Cozo. Writing a fact *asserts* it at a timestamp; changing it *retracts* the
old value and asserts a new one at a later timestamp. Nothing is deleted or
overwritten in place:

- **Time-travel is free.** A read `@ 'NOW'` sees the currently-valid version; a
  read as-of any past instant reconstructs what the graph looked like then. This
  is what `at` (full node read, optionally `when:`) and `history` expose.
- **Conflict resolution is non-destructive.** A `supersede` retracts the prior
  version through the substrate rather than clobbering it, so the old reasoning
  survives and remains inspectable.

The core node schema (simplified):

```
:create node {
    id: String,
    validity: Validity default [floor_to_second(now()), true] =>
    type: String,          # episode | reference | idea | outcome | chain | task | hypothesis | audit_event | ...
    tier: String,          # operational | archival
    name: String,
    body: String?,
    tags: [String]?,
    initiatives: [String]?,
    properties: Json?,
    visibility: String default 'local',   # local | shared
    layer: String default 'warm',         # core | hot | warm | cold | frozen
}
```

Edges live in a parallel `edge` relation (also `Validity`-keyed) carrying `src`,
`dst`, `edge_type`, and a `weight` (0..1). Junction relations (`node_initiative`,
`edge_initiative`, `chain_member`) deliberately **do not** put `Validity` in the
PK — membership is a set fact, not a versioned one.

> **Known edge case — whole-second resolution.** Validity is floored to the
> second. Two opposing mutations on the same node within one second (e.g. `link`
> then an immediate `unlink`) resolve ambiguously. Human-paced and cron-paced use
> always crosses the boundary; the test suite sleeps across it.

### Schema migrations

A forward-only `migration_journal` runs on open: a newer build applies additive
schema changes (new relations / columns) to an existing vault in place. There is
no down-migration or destructive path — pre-1.0, `export` before a major upgrade.

---

## 2. Two tiers — hippocampus and cortex

Orthogonal to layers, every node sits in one of two **tiers**, a direct nod to
the hippocampus / cortex split:

- **Operational (hippocampus).** High-velocity working memory: episodes,
  scratch, drafts, hypotheses under test, open questions, tasks, audit events.
  It churns and gets revisited.
- **Archival (cortex).** Settled knowledge: ideas, outcomes, summaries,
  references, persona/entity records. Mostly read; this is what survives.

Promotion across the boundary is a deliberate, logged operation
(`synthesise` → `settle`, or `consolidate_out`), and provenance (`derived_from`)
is preserved across the tier boundary rather than lost. `consolidate_in` mirrors
it back when settled knowledge needs reworking.

**Cortex on re-entry.** `awake` reads the two tiers *separately*: the operational
working set by layer (`core → hot → warm`), and a dedicated **cortex** slice of
the archival tier. Core-layer cortex is uncapped, so a project's standing facts
re-enter every session instead of waiting for an explicit recall. This is what
makes the cortex actually get *used* — it is surfaced by default, not on demand.

---

## 3. Memory layers and layered re-entry

The second axis is the **layer** — how eagerly a node should be surfaced to a
future agent:

| Layer | Meaning | Re-entry behaviour |
|---|---|---|
| `core` | Always-in-context keystone facts, standing rules | Loaded uncapped by `awake` |
| `hot` | Frequently needed | Loaded after core (bounded) |
| `warm` | Default; relevant | Loaded after hot (bounded) |
| `cold` | Archived | Only via explicit `surface` |
| `frozen` | Retained but not surfaced | Only via explicit `surface` |

Layer is stamped **at creation** — every capture verb (`episode`/`jot`/`cite`/
`task`/`claim`) takes an optional `layer` (default `warm`) — so a node is born
with its priority. Keeping `core` small and explicit is a discipline the tooling
encourages but does not enforce.

`awake` is the single re-entry call: it returns the layered operational working
set, the cortex slice, session pins, recent episodes, and the open-review queue
in one bundle. `surface` reaches the cold/frozen material `awake` intentionally
withholds.

---

## 4. Edges as operational semantics

Edges are **not** mere associations. Each edge type is something the curator API
*responds to*:

- `derived_from` — provenance / explainability; the chain that survives the tier
  boundary during consolidation.
- `contradicts` — triggers a non-destructive `under_review` flow (the target is
  flagged, not changed, until `resolve`/`refute`).
- `supersedes` — retracts the previous version through the bi-temporal substrate.
- `refers_to`, `part_of`, `causal`, `blocks`, `targets`, `verifies`,
  `falsifies`, `consolidated_to`, `temporal` — typed relations the traversal and
  export layers interpret.

Edges carry a `weight` (0..1). Strong links (`strong: true` = weight 1.0) make
shorter paths for knowledge chains (§7).

---

## 5. Per-initiative scoping via junction relations

One substrate holds many **initiatives** (projects). Membership is a
junction relation — `node_initiative(initiative, node_id)` — rather than a column
on the node. Two consequences:

- **Multi-membership is native.** The same `node_id` can appear under several
  initiatives (different rows, same node). `attach` grants a node a second home
  additively — the repair for fragmentation, where knowledge about one topic gets
  scattered across projects captured under different names.
- **Scoped reads are cheap.** RocksDB prefix-scan over the junction gives
  `O(log n + k)` on the active initiative rather than a full-table filter.

Scope is a property of the `Store` at call time (`use_initiative` / `scoped`).
When an initiative is active, reads and mutations are scoped to it; with none
active, reads are cross-initiative and mutations land un-tagged. Because the
daemon shares one `Store` across concurrent sessions, `Store::scoped` serializes
the set-scope-then-operate sequence behind an internal guard so two sessions
can't corrupt each other's scope. Initiative-level verbs (`rename_initiative`,
`delete_initiative`, `attach`) take explicit names and don't rely on the active
scope, so the cloud can call them too.

`delete_initiative` drops membership and `forget`s nodes *exclusive* to the
initiative; nodes shared with others only lose that one membership. Forgetting is
bi-temporal, so a delete is recoverable via `at(<past>)`.

---

## 6. Retrieval — structural first

`kaeru` is deliberately **not** a RAG vector store. Retrieval, in priority order:

1. **Exact lookup** — `recall` by name; `at` for the full node.
2. **Typed traversal** — `walk` / `drill` / `trace` over edge types; `between`
   for the edges linking two nodes; `overview` for a readable subgraph map.
3. **Layered re-entry** — `awake` / `surface` (§3), and `tagged` reads.
4. **Saved reasoning chains** — `chain` / `read_chain` (§7).
5. **Full-text fuzzy fallback** — Cozo FTS (`search`) when the exact name is
   forgotten.
6. **Vectors** — Cozo HNSW is available but kept as a *cold* fallback, not the
   primary mode.

The bet: an agent that curates a typed graph and re-enters it structurally beats
one that embeds everything and hopes cosine similarity surfaces the right passage.

---

## 7. Knowledge chains

A **chain** is a saved reasoning trail: the shortest *weighted* path between two
nodes (Dijkstra with `cost = 1 − weight`, so strong edges are short hops),
materialised as a first-class `chain` node plus an ordered `chain_member` list.

- `path` previews the trail; `chain(from, to)` saves it. An agent-authored
  `summary` explains *why* the trail matters and becomes the chain's body.
- `chains(node)` lists the chains a node is in **with their summaries**, so an
  agent triages the menu instead of reading every trail; `read_chain` reads one
  in full.
- **Dedup at creation.** The path is deterministic, so a repeated `chain(a, b)`
  reuses the existing identical chain instead of duplicating it (a repeat with a
  new summary refreshes the metadata).
- **Mutability.** `rechain` regenerates a chain between its current endpoints
  (picking up new edges / re-weights) or extends it to a new node — so a trail
  the graph outgrew can be refreshed rather than left stale.

Chains turn "an isolated, context-poor node" into "the connected story of how a
conclusion was reached".

---

## 8. Reflection — self-maintenance

`reflect` is a **computed maintenance work-list**, not a static reminder. The
store works out what actually needs tending in the active scope:

- **orphans** — nodes with no edges at NOW (link them or forget them);
- **open reviews** — nodes with an inbound `contradicts` (resolve / refute);
- **stale chains** — chains whose stored members no longer match the recomputed
  shortest path between their endpoints (`rechain`);
- **cortex candidates** — operational, linked nodes untouched past a threshold
  (`reflect_settle_age_secs`, default 14d) — settled work to promote into cortex;
- **shared / cloud** — shared nodes whose rebalancing is escalated to the user,
  never auto-applied.

Each item ships with *how* to act on it. It's built for a periodic (cron) tidy
pass and is deliberately **not** wired into the re-entry instructions — reflection
is a maintenance beat, not something to nag a coding agent about mid-task.

---

## 9. Audit events

`audit_event` is a first-class node type: every mutation primitive writes an
audit node automatically. Changes to memory themselves become reasoning surface.
Substrate history (`Validity`) and operational audit (audit nodes) stay separate:
the substrate tracks *what was*, the audit nodes track *who changed it and why*.
Audit nodes are excluded from ordinary reads and re-entry.

---

## 10. Topology — daemon, adapters, cloud

`kaeru-core` is the library (substrate, schema, primitives). Three consumers
wrap it:

- **`kaeru-mcp`** — a Model Context Protocol server (rmcp 1.6, streamable HTTP).
  **One daemon per machine** owns the RocksDB writer lock; any number of agent
  sessions (Claude Code, Cursor, Opencode, …) connect concurrently over HTTP.
  A stdio MCP that forked a subprocess per session would deadlock on the single
  writer — hence the long-lived shared daemon. Bearer-token auth and host
  allow-listing gate non-loopback binds.
- **`kaeru-rig`** — the [rig](https://github.com/0xPlaygrounds/rig) adapter: the
  full curator verb set as discrete `rig::tool::Tool`s over an embedded
  `Arc<Store>`, so a rig agent reads/writes one vault directly (no cloud HTTP).
  Store work runs on `spawn_blocking`; per-call scope is serialized through
  `Store::scoped`.
- **`kaeru-cloud`** — the optional shared tier: an Axum REST service over the
  same `kaeru-core`, one per team, bearer-token auth. Local daemons reach it
  **only** through `kaeru-mcp`'s cloud client. It adds no separate persistence
  path — it wraps the same core behind HTTP. Sharing is explicit and passes two
  gates: the initiative's `share_policy` and a deterministic secret guard.

All graph reads/writes go through `kaeru-core` primitives; adapters never issue
raw Cozo queries.

---

## 11. Design stance — facilitator, not enforcer

The curator API is a set of *available tools*, not a mandatory protocol. MCP
tools hint when context is missing (no active initiative, a fresh node with
nothing linked) but never block. There are no required call sequences. The verb
taxonomy (`awake`, `drill`, `claim`, `settle`, `reflect`, …) is meant to map to
natural agent thinking, not merely expose graph operations — the goal is that an
agent reaches for the memory because the verbs match how it already reasons.
