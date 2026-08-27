# kaeru — Documentation

Detailed reference for the design and internals of `kaeru`, a cognitive memory
layer for LLM agents. For a product overview and install/runbook, see the
top-level [`README.md`](../README.md) and [`QUICK_START.md`](../QUICK_START.md);
this folder is the deeper "how it's built and why" reference.

## Contents

- **[architecture.md](architecture.md)** — the design: the bi-temporal substrate,
  the two-tier (hippocampus / cortex) model, memory layers and layered re-entry,
  edges as operational semantics, per-initiative scoping via junction relations,
  the structural-first retrieval model, knowledge chains, the reflection pass,
  the daemon / adapter topology (MCP, rig, cloud), and the read-only HTTP
  surface where a route is a verb.
- **[structure.md](structure.md)** — the code map: the four crates, the
  `kaeru-core` module layout (graph / recall / mutate), and where a given
  concern lives.
- **[board.md](board.md)** — the task board: how a per-initiative status registry
  turns the existing `task` nodes into a customizable tracker, why a card's
  column is a tag while the column *order* is a registry, and the JSON contract a
  UI builds a board (and a time scrubber) on.
- **[offline-build.md](offline-build.md)** — building with no network access:
  where the vendored dependencies live (a separate repo, one tag per release),
  what the fetch script checks, the C++ toolchain kaeru needs regardless, and
  the Windows line-ending trap that fails every checksum at once.
- **[curator-api.md](curator-api.md)** — the verb taxonomy: the ~70 curator
  primitives grouped by what they do (re-entry, capture, link & chain, recall,
  time-travel, evolve, initiatives, sharing, maintenance), with the epistemic
  intent behind each group.

## One-paragraph mental model

`kaeru` is a typed property graph over CozoDB (RocksDB backend) whose substrate
is **bi-temporal**: every node and edge records when it was asserted and
retracted, so history and time-travel are native, not bolted on. The graph is
split into two **tiers** — *operational* (the hippocampus: fast, in-flight
thinking) and *archival* (the cortex: settled knowledge) — and orthogonally into
five **layers** (`core`/`hot`/`warm`/`cold`/`frozen`) that govern how eagerly a
node re-enters context. One substrate holds many **initiatives** (projects) via a
junction relation, so a node can belong to several at once. Retrieval is
**structural first** — exact lookup, typed traversal, saved reasoning chains,
layered re-entry — with full-text search as a fuzzy fallback (there is no
vector/embedding layer today). Agents
reach the graph through a **curator API** of ~70 verbs, exposed over MCP (a
one-daemon-per-machine HTTP service) and as native `rig` tools. The design stance
is **facilitator, not enforcer**: the verbs are available tools the agent chooses
to use; the daemon hints but never blocks.
