# 蛙 kaeru

`kaeru` is a **cross-agent cognitive engine** for LLM agents — a typed graph that agents think in, plus a recollection layer for long-term ideas and outcomes. Local-first for each agent, with an optional shared cloud tier so a whole team of agents and people build on one another's memory.

Designed for **multi-session, multi-agent continuity**: when an agent opens a project, it has full context of what was being thought about, can follow provenance chains, can consolidate outcomes into stable long-term knowledge, and can pull in what the rest of the team has shared.

Inspired by the LLM-wiki pattern (Karpathy, gist `442a6bf555914893e9891c11519de94f`), the bi-temporal knowledge graph approach of Graphiti / Zep, the curator-driven knowledge engine of Cognee, and the reasoning-based hierarchical-summary navigation pattern from PageIndex. Two-tier design grounded in the hippocampus / cortex split.

Name: 蛙 (*kaeru*, "frog"; homophonic with 帰る "to return" and 変える "to change") — the agent that returns, recalls, and reshapes.

![A kaeru vault rendered as a live knowledge galaxy by kaeru-viz](kaeru-viz/galaxy.gif)

<sub>A kaeru vault rendered by [`kaeru-viz`](kaeru-viz/) — each project a constellation of stars around its core, sized by memory layer, with ochre cross-project bridges, reasoning-chain replay, and a time-lapse of how the knowledge grew. Hover a node to trace its neighbours; click to pin them.</sub>

## Overview

![A kaeru vault rendered as a knowledge galaxy by kaeru-viz](kaeru-viz/screenshot.png)

`kaeru` is built around a typed property graph stored in CozoDB. Two tiers, biological analogy:

- **cognitive (operational / hippocampus)** — high-velocity working graph where the agent actively thinks: episodes, scratch, drafts, hypotheses, experiments, audit events.
- **recollection (archival / cortex)** — settled ideas, outcomes, summaries, references; mostly read.

Nodes and edges retain recorded assertion / retraction history, so earlier content stays inspectable after a change. There is one recorded-time axis; administrative metadata such as layer and visibility is labeled as current.

Per-initiative subgraphs through a junction-relation pattern: one substrate, many initiatives, multi-membership. An agent working on project A asks "what was I doing here last time?" and gets an answer scoped to A. The same node can belong to several initiatives at once.

`kaeru` is a **facilitator, not an enforcer**. The memory API exposes 35 named operations (`context`, `search`, `read`, `claim`, `synthesise`, `history`, `lifecycle`, …) as available tools. The agent and user choose when to invoke them; the daemon hints but doesn't block.

## Features

The shared API has 35 named tools: 16 frequent operations, ten local maintenance
operations and nine optional cloud operations. MCP, HTTP and embedded Rig share
this contract. See [the API reference](docs/curator-api.md).

- **Two-tier graph** — operational (cognitive / hippocampus) for active thinking; archival (recollection / cortex) for settled knowledge. `lifecycle` promotes across the boundary, preserving content and provenance.
- **Recorded history** — native assertion / retraction history. `read` retrieves a full record or its state at `recorded_at`; `history` shows earlier versions. Changes preserve the previous assertions instead of deleting their history.
- **Per-initiative scoping + layered re-entry** — one substrate, many projects. `context` restores a project's working set by recall layer and surfaces settled knowledge alongside open work. `search` reaches Cold / Frozen records too unless a layer filter excludes them.
- **Reasoning chains** — `trail` saves a weighted route or an explicit ordered sequence with an agent-authored summary. Ordinary `read` includes saved reasoning and reports stale premises; `include=["reasoning"]` expands the members. A trail can be refreshed or extended after the graph changes.
- **Self-maintenance** — `reflect` produces findings about integrity, unfinished work, staleness and sharing. The agent chooses what to apply; automatic hygiene separately maintains recall layers.
- **Role slots** — `initiative_manage(action="set_slot")` gives an initiative one current pointer per role (`handoff`, `entrypoint`, `queue`). `context` returns the holder and slot revision. Replacing the pointer does not archive or retire the previous record; those remain explicit lifecycle decisions.
- **It tells you when you are behind** — the daemon checks for newer releases daily and surfaces an update notice to the agent. It uses `updates.lamantin-ai.com` with a GitHub fallback; version, platform, installation channel and environment are sent, never vault content. The endpoint keeps daily aggregate counts; see [the update Worker](contrib/update-worker/) for retention details. `KAERU_MCP_UPDATE_CHECK=0` disables the check.
- **Automatic hygiene** — a background pass moves old unreferenced journal entries to Cold, lowers untouched unreferenced Core nodes and raises heavily referenced nodes. It triggers on accumulated writes, Core growth and elapsed time, runs off the reactor in batches, and changes only recall layers. On by default in the daemon: `KAERU_MCP_HYGIENE_ENABLE=0` disables it. Embedded `kaeru-rig` hygiene remains opt-in through `KaeruMemory::with_hygiene()`. `retention(action="preview_hygiene")` shows proposed moves without applying them.
- **Cross-agent sharing** — local-first by default; an optional `kaeru-cloud` tier lets a trusted team share knowledge through initiative policy, allowed cloud destinations and a deterministic secret guard. `cloud_recall` searches the shared tier, `unshare` withdraws a shared record, and several configured clouds require an explicit destination. See [Local & cloud](#local--cloud--sharing-memory-across-a-team).
- **Structural recall** — scoped name / ID lookup, literal lexical search and typed graph traversal. `read` expands relations, provenance and paths on demand; returned records carry currentness, material conflicts and saved reasoning. Source references preserve the supplied text exactly.
- **Read-back on re-entry** — `context` restores what was *touched* and also what is still *owed*: project rules, current slots, open tasks, claims, reminders and saved reasoning. Bounded results report coverage and provide continuation arguments.
- **Initiative management** — `initiative_manage` renames, merges or deletes an initiative, or attaches an existing record to another initiative to repair fragmentation.
- **Markdown export** — an Obsidian-friendly snapshot of an initiative through `export`. AMG additionally transfers structured current knowledge between vaults.

## Architecture Notes

- **Substrate is CozoDB** with the embedded RocksDB backend. `Validity` stores recorded assertions and retractions natively; current metadata is not presented as a second historical axis.
- **Edges carry operational semantics** — `derived_from` powers provenance, `contradicts` records counterevidence, and `supersedes` runs from the replacement to what it replaced. `replace` creates a successor and retires the previous identity; `revise` updates the same identity. Edges are not just associations.
- **`audit_event` is a first-class node type** — mutations record operational audit alongside domain changes. Substrate history tracks prior assertions; audit records explain the operation and actor. Intrinsic mutations, audit and replay records commit together.
- **Per-initiative scope through junction relations** rather than column filtering — a record can belong to several initiatives. The shared API applies scope and visibility checks before returning content or following links.
- **Retrieval is structural-first** — explicit name / ID lookup, lexical search, typed graph traversal and saved summaries. No embedding index is wired into the current retrieval path.
- **Two-tier with explicit promotion** — `lifecycle(target_tier="archival")` moves settled knowledge into the archival tier while preserving content and references; promotion can report an identity change. Recall layers remain independent of the tier.
- **Single binary, embedded substrate** — the local service owns the vault and serves MCP, named HTTP operations and the visualizer. The vault lives under a platform-specific default (Linux `$XDG_DATA_HOME/kaeru`, macOS `~/Library/Application Support/ai.lamantin.kaeru`, Windows `%LOCALAPPDATA%\ai.lamantin.kaeru`); override with `KAERU_VAULT_PATH`.

## Layout

```
kaeru/
├── Cargo.toml                  ← workspace root
├── kaeru-core/                 ← library: substrate, shared API, memory primitives and models
├── kaeru-mcp/                  ← binary `kaeru-mcp`: MCP, HTTP API, instance UI and local auth
├── kaeru-cloud/                ← binary `kaeru-cloud`: shared cloud tier (Axum REST over kaeru-core)
├── kaeru-rig/                  ← library: `rig` Tools that give a rig agent kaeru memory
├── kaeru-viz/                  ← visualizer source and embedded frontend build
└── skills/
    └── kaeru-skill/            ← portable agent skill (Claude Code / etc.)
```

`kaeru-rig` is the [rig](https://github.com/0xPlaygrounds/rig) framework adapter — the shared memory operations as discrete rig `Tool`s over an embedded `Arc<Store>`, so a rig agent reads and writes one vault. Future: `kaeru-langchain` (Python bridge), not yet started.

## Install

> **Pre-1.0 alpha.** Take a vault and configuration backup before upgrading;
> see [upgrades and backups](QUICK_START.md#5-upgrades-and-backups).

```sh
curl -fsSL https://raw.githubusercontent.com/LamantinAI/kaeru/main/contrib/install/install.sh | bash
```

The installer sets up a user-level daemon on Linux or macOS and asks whether to
enable authentication. A fresh local installation defaults to no auth; updates
preserve the configured choice. See [QUICK_START.md](QUICK_START.md) for source
builds, client configuration and platform requirements.

## Quick tour (MCP tools)

```text
# See what projects exist, then recover the project's context:
initiatives()
context(initiative="auth-rewrite")

# Capture an observation:
note(initiative="auth-rewrite", body="Token expiry differs across platforms")

# Search and read a record with its connections:
search(initiatives=["auth-rewrite"], text="expiry", prefix=true)
read(initiative="auth-rewrite", node="<returned uuid>", include=["relations"])

# Record a hypothesis, then its checked verdict:
claim(initiative="auth-rewrite", text="The platform-aware policy is correct")
verdict(initiative="auth-rewrite", hypothesis="<hypothesis uuid>",
        status="supported", evidence={"method":"Compared iOS and Android TTL"})

# Inspect recorded history:
history(initiative="auth-rewrite", node="<uuid>")
read(initiative="auth-rewrite", node="<uuid>", recorded_at="2026-10-01T12:00:00Z")

# Save the reasoning route with an authored summary:
trail(initiative="auth-rewrite", route={"from":"<a>","to":"<b>"},
      summary="The evidence supports the platform-specific policy")
read(initiative="auth-rewrite", node="<a>", include=["reasoning"])

# Point the project at its current handoff:
initiative_manage(action="set_slot", initiative="auth-rewrite",
                  slot="handoff", node="<handoff uuid>")

# Preview the next layer-maintenance pass:
retention(initiative="auth-rewrite", action="preview_hygiene")

# Export an initiative into the host's configured artifact root:
export(initiative="auth-rewrite", destination="/path/to/artifacts/auth-snapshot")
```

Follow `continuation.next_arguments` and inspect `coverage` on bounded reads.
A `partial` mutation may already contain committed changes; inspect its IDs and
failed steps before retrying. Hosts can provide `request_id` to recover an
uncertain outcome. See [all operations and arguments](docs/curator-api.md).

## Building without network access

Some machines cannot reach crates.io. The dependencies for each release are
vendored into a separate repository, [`LamantinAI/kaeru-vendor`](https://github.com/LamantinAI/kaeru-vendor),
one tag per release:

```bash
git checkout <release-tag>
./contrib/offline/fetch-vendor.sh     # ~600 MB, needs network once
# carry the whole directory across, then:
cargo build --release --offline -p kaeru-mcp
```

Rust alone is not enough on either side of that line — cozo builds RocksDB from
C++ and zstd/lz4 from C, so a C++ toolchain and `libclang` are needed whether or
not you are offline. See [docs/offline-build.md](docs/offline-build.md) for the
per-platform requirements and the Windows line-ending trap that fails every
checksum at once.

## Connecting to an MCP-aware agent

`kaeru-mcp` is a long-lived HTTP service: **one daemon per vault** owns the substrate, any number of agent sessions (Claude Code, Opencode, Cursor, …) connect concurrently. This is intentional — RocksDB is single-writer, so a stdio MCP that forks a subprocess per session would hit lock contention. See `kaeru-mcp/README.md` for systemd / launchd unit templates and the full HTTP config.

Run the daemon (or set up the systemd user unit from `contrib/install/`):

```bash
kaeru-mcp                                            # foreground, Ctrl-C to stop
```

Then point your agent at it:

- **Claude Code**: `claude mcp add --transport http kaeru http://127.0.0.1:9876/mcp` — see `skills/kaeru-skill/` for the portable system-prompt rules.
- **Check memory before asking** (Claude Code and Codex): `contrib/hooks/kaeru-first/` is a harness hook for the moment the agent is about to ask the user something — if kaeru was not read recently, it sends the agent to search first, once, and then asks it to capture the user's answer. Stdlib Python, fails open. See its README.
- **Opencode**: `bash contrib/opencode/install-opencode.sh` — wires the daemon, drops `AGENTS.kaeru.md` rules into `~/.config/opencode/`, and installs `/kaeru` / `/lesson` / `/recall` slash commands. Designed to coexist with your existing OSS-model provider config (Qwen / DeepSeek / GLM / Ollama). See `contrib/opencode/README.md`.
- **Cursor and other runtimes**: paste the body of `skills/kaeru-skill/SKILL.md` into your agent's rules / system-prompt section. For MCP-aware clients the daemon URL above works directly.

After restart the agent sees `context`, `search`, `read`, `note`, `claim` and the other named tools. Project calls pass `initiative`; search uses an `initiatives` list. Clients that need stdio can run `kaeru-mcp --stdio`, a relay to the same daemon.

## Local & cloud — sharing memory across a team

`kaeru` runs **local-first**: your vault lives on your machine and nothing leaves it by default. A second, optional tier — `kaeru-cloud` — is a shared store for a trusted group (a team, a family). Each initiative carries a sticky `share_policy`:

- `private` (default) — nothing ever leaves; personal projects.
- `team` — nodes you explicitly mark `shared` may sync to the cloud.

Sharing is never automatic and passes two gates: the initiative policy, and a deterministic **pre-share secret guard** that blocks anything looking like an API key, token, or private key. The guard is silent on clean content and only interrupts on a real hit.

Verbs (over MCP): `policy` (mark an initiative `team`), `share` (push a node), `cloud_recall` (see what the team has), `pull` (bring a shared node into your local graph), `link_cloud` / `cloud_links` (reference cloud nodes without copying), and `sync_review` (batch-review still-local nodes). `note` and `reference` accept `sharing: {visibility: "shared", cloud: "team"}` to capture-and-share in one call.

`cloud_recall(initiative="project", id="<uuid>")` reads a complete shared record without importing it. `pull` copies it locally and restores its incident edges while preserving local weights and explicit unlinks. Local reads do not search the cloud automatically.

A single daemon can also reach **several named clouds** (e.g. a `family` and a `work` cloud) via a `clouds.toml` file; the cloud verbs then take an optional `cloud: <name>` and soft links remember which cloud they point at. See [`kaeru-mcp/README.md`](kaeru-mcp/README.md#configuration) for the file format.

Per-user / per-org isolation (multi-tenant) is a future addition; today each cloud is one shared space scoped by initiative. See [`kaeru-cloud/README.md`](kaeru-cloud/README.md).

## Instance visualizer

Open `http://127.0.0.1:9876/ui/` for the live knowledge galaxy, chain replay,
timeline, reader and task board. It uses the same scoped memory API as agents;
the graph loads as compact structure and record texts load when opened.

Authentication is optional. `kaeru-mcp --auth enabled` configures a local account
and JWT browser sessions; agent MCP bearer credentials remain separate. See
[UI, access and account configuration](docs/instance-ui.md).

## Knowledge transfer (AMG)

AMG is a CBOR-based format for transferring current knowledge between vaults.
Export the whole graph or selected initiatives, then inspect and import it:

```sh
# Stop each vault's daemon before offline export or import.
kaeru-mcp amg export --vault /path/to/source --all --output knowledge.amg
kaeru-mcp amg inspect knowledge.amg
kaeru-mcp amg import knowledge.amg --vault /path/to/destination --dry-run
kaeru-mcp amg import knowledge.amg --vault /path/to/destination
```

Metadata records the export time, schema version, software version and initiative
selection. `--force` appends conflicting imported versions under the same IDs
while preserving local history. For a complete backup, stop the owner and copy
the vault and configuration. See [AMG format and import behavior](docs/amg.md).

## Roadmap

- **Initiative onboarding** — a generated "what is this initiative, and what's in it" briefing the first time an agent enters a project, an instruction over the collected context, not graph machinery.
- **PostgreSQL backend** — a server-mode substrate alongside the embedded RocksDB default.
- **Multi-tenant + isolation** — per-user / per-org separation in the shared cloud.

## Store verification and read tracing

Store tests use an independent temporal oracle to check recorded state against the substrate.
Optional local read tracing records delivered IDs and operation metadata, with
72 hours of raw JSONL followed by gzip and 14 days total retention. Enable it
with `KAERU_READ_TRACE_ENABLED=true`; no journal is written by default. See
[storage verification and read tracing](docs/architecture.md#local-read-delivery-journal).

## Status

Pre-1.0. Implemented: the typed graph and shared 35-operation memory API,
independent tiers and recall layers, recorded history, scoped lexical / graph
recall, authored reasoning trails, atomic commands, forward migrations, optional
team clouds, the Rig adapter, the instance visualizer with optional local auth,
and AMG knowledge transfer. Current boundaries:

- **Multi-tenant.** A cloud is one trusted shared space scoped by initiative; per-user / per-org isolation is not built yet.
- **MCP concurrency.** Concurrent sessions share one `Store`; scoped calls are serialized. Cancellation is cooperative, and concurrently submitted calls do not imply a user-specified read-after-write order.
- **Historical metadata.** Earlier content is inspectable; current administrative metadata is explicitly labeled as current. There is one recorded-time axis.
- **Adapters and storage.** Rig is supported; the Python / LangChain bridge, PostgreSQL backend and embedding retrieval are still future work.
- **Migrations and transfer.** Vault migrations are forward-only. AMG transfers current knowledge, not the source vault's full history or runtime configuration; cryptographic signatures and encryption are not implemented.

See [architecture](docs/architecture.md), [the API reference](docs/curator-api.md)
and [read-performance measurements](docs/read-performance.md) for the contracts
and measured behavior.

## Contributing

Discussion and design feedback through issues. PRs welcome on the open items
above and anywhere the agent-facing surface feels rough — the operations
(`context`, `read`, `claim`, `verdict`, `trail`, …) are meant to map to natural
agent thinking, not just expose graph operations. See [the documentation index](docs/README.md)
for the implementation map and development checks.

## License

**Business Source License 1.1** — see [`LICENSE`](LICENSE), with a plain-language
summary in [`LICENSING.md`](LICENSING.md).

kaeru is *source-available*: read it, modify it, run it in production, and embed
it as a component or tool inside your own systems and products — including what
you sell. The one thing you may not do is operate kaeru itself as a hosted /
managed / SaaS platform for third parties. Each released version converts to the
**Apache License 2.0** on its configured Change Date or four years after its first
public distribution, whichever comes first. Contributions are accepted under
the [Contributor License Agreement](CLA.md).
