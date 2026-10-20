# 蛙 kaeru

**Cognitive memory for agents.** Kaeru gives LLM agents persistent project
context through a typed graph: observations, decisions, sources, open work and
the relationships explaining how they fit together. A local vault works across
sessions and agents; an optional team cloud holds explicitly shared knowledge.

The name 蛙 (*kaeru*, “frog”) echoes 帰る (“to return”) and 変える (“to change”):
return to a project, recall its context, and reshape what you know.

![A Kaeru vault in the knowledge galaxy visualizer](kaeru-viz/galaxy.gif)

The visualizer is embedded in `kaeru-mcp` at `/ui/`. No graph dump, separate
frontend server or Node.js runtime is needed on an installed instance.

## What it does

- **Scoped memory:** initiatives separate projects; a record may belong to several.
- **Two tiers:** operational work and archival knowledge, independent of the
  `core`, `hot`, `warm`, `cold` and `frozen` recall layers.
- **Connected reasoning:** typed links, source provenance, hypotheses with
  evidence, and authored reasoning trails that report stale premises.
- **Recorded history:** edits and retractions preserve prior node/edge states.
  There is one recorded-time axis; current administrative metadata is labeled.
- **One agent contract:** 35 named operations over MCP, HTTP and embedded Rig:
  26 local operations and nine cloud operations. No query DSL.
- **Inspectable access:** optional local account/JWT sessions for the UI and
  separate bearer credentials for MCP; local installation defaults to no auth.
- **Knowledge transfer:** AMG/CBOR export and atomic import of the current graph
  or selected initiatives, with explicit force revisions preserving local history.

Kaeru supplies tools rather than imposing a workflow. `context` restores a
bounded working set, and `search`/`read` recover additional knowledge on demand.
Recall uses graph structure and lexical search; embeddings are not implemented.

## Get started

Install a published binary and a user-level daemon on Linux or macOS:

```sh
curl -fsSL https://raw.githubusercontent.com/LamantinAI/kaeru/main/contrib/install/install.sh | bash
```

The installer asks whether to enable authentication and defaults to no auth for
a fresh local installation. Existing configured modes are preserved on unattended
upgrades. Features described here follow this source checkout; an older published
binary may require upgrading. See [Quick Start](QUICK_START.md) for source builds,
platforms and client setup.

Connect an MCP client to `http://127.0.0.1:9876/mcp`. Clients needing stdio can
run `kaeru-mcp --stdio`: it relays to the daemon instead of opening another vault.
The browser UI is `http://127.0.0.1:9876/ui/`. Installer-managed local services
allow it to view their owner's vault; manual runs require an explicit
[HTTP/UI scope configuration](docs/instance-ui.md#access-and-sessions).

```text
context(initiative="billing")
note(initiative="billing", kind="observation", body="Rounding differs by platform")
search(initiatives=["billing"], text="rounding", prefix=true)
read(initiative="billing", node="<returned uuid>")
claim(initiative="billing", text="The platform-specific rounding rule is correct")
verdict(initiative="billing", hypothesis="<returned uuid>",
        status="supported", evidence={"method":"Compared the documented examples"})
history(initiative="billing", node="<returned uuid>")
```

Read `coverage` and follow `continuation.next_arguments` for bounded responses.
A `partial` mutation may already have committed a local record; inspect its
returned IDs and failed steps before retrying. Hosts can supply `request_id` to
recover an uncertain outcome.

## Local service and embedded use

One daemon owns each CozoDB/RocksDB vault. Concurrent agent sessions use that
process over HTTP; separate vaults can use separate daemons. The stdio relay
auto-starts a daemon if needed. Embedded Rust applications can instead use
`kaeru-rig` over an owned `Arc<Store>`; they must not open a vault already held
by a daemon.

| Component | Responsibility |
| --- | --- |
| `kaeru-core/` | Graph, shared memory API, transactions, recorded history and AMG. |
| `kaeru-mcp/` | MCP transports, named HTTP API, embedded UI, local auth and operator CLI. |
| `kaeru-rig/` | Rig tools over the same local API, with optional cloud access. |
| `kaeru-cloud/` | Shared store for a trusted team, using its own vault and bearer token. |
| `kaeru-viz/` | Frontend source; generated assets are embedded in the MCP binary. |
| `skills/kaeru-skill/` | Portable instructions teaching agents how to use memory. |

See [architecture](docs/architecture.md) and [code structure](docs/structure.md).

## Optional team cloud

Nothing is shared automatically. Initiative policy, allowed cloud destinations
and a pre-share secret guard govern explicit sharing. Local reads do not search
the cloud. A single configured cloud resolves implicitly; with several, an
operation must name its destination even when the config declares a default.

The cloud is a shared space for a trusted group, without per-user tenancy.
Browser accounts belong to the local MCP service; they do not add cloud tenant
isolation. See [cloud setup](kaeru-cloud/README.md) and
[daemon configuration](kaeru-mcp/README.md#configuration).

## Export, transfer and backups

AMG transfers current knowledge, including typed properties and semantic
metadata. `--force` appends supplied conflicting node/edge versions under the
same IDs; it does not erase prior local versions or infer deletions.

```sh
# Stop the daemon using each vault before offline export/import.
kaeru-mcp amg export --vault /path/to/source --all --output knowledge.amg
kaeru-mcp amg inspect knowledge.amg
kaeru-mcp amg import knowledge.amg --vault /path/to/destination --dry-run
kaeru-mcp amg import knowledge.amg --vault /path/to/destination
```

AMG does not contain source history, accounts, journals or runtime configuration.
For a complete pre-upgrade backup, stop the owner and copy its vault and config.
The agent `export` operation produces a scoped JSON/Markdown view, not an AMG
file or a complete backup. See [AMG](docs/amg.md) and
[upgrade guidance](QUICK_START.md#5-upgrades-and-backups).

## Development and documentation

Rust builds need a C/C++ toolchain and libclang for RocksDB. Cargo embeds the
checked-in frontend bundle; Node.js is needed only to change/build the UI.

```sh
cargo check --workspace --locked
cargo test --workspace --all-targets --locked
cargo fmt --all -- --check
```

- [Quick Start](QUICK_START.md): installation, auth, clients, upgrades and cloud.
- [Documentation index](docs/README.md): API, architecture and implementation guides.
- [MCP service](kaeru-mcp/README.md): transports and operator configuration.
- [UI and authentication](docs/instance-ui.md): access modes, accounts and builds.
- [Offline builds](docs/offline-build.md): matching vendor trees and native prerequisites.
- [Portable skill](skills/kaeru-skill/): agent instructions; optional client wiring lives in `contrib/`.

An optional daily update check sends version/platform/channel information, never
vault content. Disable it with `KAERU_MCP_UPDATE_CHECK=0`; see the
[update Worker](contrib/update-worker/). Optional local
[read tracing](docs/architecture.md#local-read-delivery-journal) records delivered
IDs with 72-hour raw retention followed by compression and 14-day total retention.

## Status and license

Pre-1.0. Existing vaults migrate forward on open; old agent verb names and HTTP
aliases have no compatibility dispatcher. API version 1 and AMG schema 1 are
independent of the software version. PostgreSQL storage, embeddings, cloud tenant
isolation and AMG cryptographic signatures/encryption are not implemented.

[Business Source License 1.1](LICENSE), with the Additional Use Grant and
conversion terms described in [LICENSING.md](LICENSING.md).
