# Architecture

Kaeru gives agents persistent project memory through a typed graph. Records
hold observations, decisions, sources and work; relationships explain how those
records depend on or challenge each other. The memory API provides ordinary
named operations over that graph.

## Storage and recorded history

The daemon owns one embedded CozoDB instance with a RocksDB backend. Agent
sessions connect to the daemon rather than opening the same vault independently.
Embedded Rig applications instead own their Store in process.

Node and edge keys include Cozo's `Validity`, a timestamp plus an assertion or
retraction marker. Content changes retain earlier assertions and retract the
old state. `history` lists versions; `read(recorded_at=...)` reads a version at
a specified recorded moment. Replacement and forgetting preserve earlier data.

This is one recorded-time axis: when Kaeru recorded a state. The API does not
store a second axis for when a fact was true in the outside world. Consequently
it does not promise full bi-temporal reconstruction.

| Stored information | What a historical read means |
| --- | --- |
| Node type, tier, name, body and tags | The recorded version at the requested moment. |
| Node layer, visibility and properties | Current metadata, identified as such. |
| Initiative and trail membership | Current junction membership; no earlier membership snapshot is invented. |
| In-place edge weight updates | The available current value, not a reconstructed previous weight. |
| Metadata audit entries | Evidence that an operation occurred; the previous value is not always stored. |

`created_at` comes from stored versions, not an ID timestamp. `recorded_at`
identifies the returned version. A revision is an opaque token for concurrency
checks; callers should not derive it from either timestamp.

The substrate currently records whole seconds. When successive changes affect
the same key within one second, mutation helpers advance that key's timestamp
to preserve order. A burst can therefore put its recorded timestamp slightly
ahead of wall time. Cozo's native clock uses microseconds while these stored
values use seconds, so its `NOW` read sees the highest committed version
immediately. This existing representation must be preserved when changing
storage or interpreting historical timestamps.

## Local commands and separate effects

An intrinsic graph command runs in one Cozo transaction. Its nodes, edges,
memberships, audit entries and replay record commit together. Storage errors,
domain errors, nested failures and panics roll back that command. Independent
writers serialize. A dedicated transaction thread avoids occupying Cozo's Rayon
query workers while waiting for transaction messages.

Some operations deliberately have supplementary effects. An optional relation
can fail after a capture, and a cloud share runs after local commit. The result
reports committed identities and failed steps, so a caller can recover without
assuming that everything rolled back.

Filesystem export and an entire hygiene pass are separate effects. Their results
are recorded afterward; a database transaction cannot undo an external file,
a remote write or a whole already applied pass.

A host-provided `request_id` binds caller, operation and normalized arguments
for a 24-hour replay window. Concurrent duplicate calls recover one outcome.
`replayed: true` means the returned result describes that original execution.
Reusing the key with different arguments is rejected. Supported updates accept
`expected_revision` to detect a competing change.

## What enters context

Operational memory holds work in progress; archival memory holds settled
knowledge. These are tiers. Recall layers (`core`, `hot`, `warm`, `cold`,
`frozen`) are independent and control how eagerly records are recalled.

`context` restores standing rules, current slots, open work, reminders, pins,
knowledge and saved reasoning. It reports incomplete coverage when a response
budget prevents delivery. A reminder is marked as seen only when fully emitted.
Hygiene proposes or applies layer changes and supplies a pending report;
context consumes that report only when returning it. Rig hygiene is host opt-in.

Search is lexical, with explicit filters and a prefix option. There is no
embedding index or agent-authored query language. `read` includes currentness,
material conflicts and bounded saved reasoning by default. Relations, provenance
and directed weighted paths can be expanded explicitly. Traversal checks access
before reading or following an endpoint, so filtering an output cannot expose
a forbidden record through graph shape.

Saved trails retain an authored summary and fingerprints of their premises.
Content-preserving promotion keeps summary freshness; editing a premise can make
it stale. Refreshing a route alone does not certify that its explanation is still
correct.

Read continuations use bounded snapshots tied to the caller, arguments, policy
and store generation. Snapshots expire after 15 minutes, and writes invalidate
them. They are limited to 8 MiB each and 16 cached reads per store. Ranges and
coverage distinguish an excerpt from complete content. Following source pages
reconstructs the supplied UTF-8 text exactly; authorized export is available for
sources too large for a cached snapshot.

## Scope, identity and current records

Initiatives are project scopes. Additive `node_initiative` and `edge_initiative`
junctions allow a record to belong to more than one project. Exact names resolve
inside the requested scope; duplicates require an ID. A per-call Store guard
restores the previous scope after execution, including failures.

HTTP also applies the operator's initiative ceiling, visibility policy and
redaction. IDs do not bypass these restrictions. The trusted owner can recover
a retired identity by known ID when it has no current memberships; this narrow
historical recovery exception is unavailable to restricted viewers.

`revise` keeps the identity and appends content history. `replace` creates a
successor with a new-to-old `supersedes` edge, preserving lineage and reporting
dependents. `lifecycle` moves between tiers and reports any identity change.
Identity transfer preserves memberships, pins, pointers and incident weighted
references. A changed fact cannot inherit evidence endorsing different content.

A slot selects one current record for a purpose such as handoff. It has its own
revision. Assigning another holder does not supersede, retire or move the old
record. Initiative rename moves slot state, sharing restrictions and hygiene
state. Merge refuses competing holders or two board registries until reconciled.

Board completion follows explicit terminal flags, independently of column order.
For legacy registries lacking flags, the former final column retains its old
terminal meaning. Status changes preserve deadlines, and due dates use UTC day
boundaries. See [the board guide](board.md).

## One local API, three adapters

`kaeru-core::api` owns the 35-operation catalogue, validation, shared result types,
authorization, retrieval and local command dispatch. Sixteen operations are
frequent, ten are occasional local operations and nine retain existing cloud
contracts. These groups express priority; they do not hide tools through lazy
loading.

| Adapter | Contract |
| --- | --- |
| MCP | Registers the shared definitions; local structured content and text carry the same JSON. |
| HTTP | Literal `POST /v1/<operation>` routes with the same arguments, a principal and egress checks. |
| Rig | Distinct `kaeru_<operation>` tools; local work runs on a blocking pool with per-call scope. |

Local responses use `api_version: 1`, independently of crate and MCP protocol
versions. Old tool names and HTTP aliases have no compatibility dispatcher.
The [code map](structure.md) shows the implementation boundaries.

## Migrations and cloud

Forward migrations run when a vault opens. Migration 0010 adds API bookkeeping;
0011 restores missing legacy membership junctions and makes previously unscoped
domain identities discoverable as `legacy-unscoped`, including retired identities.
IDs, bodies and recorded validity remain intact. This preserves stored data,
while clients must adopt the new API vocabulary.

The existing cloud REST service and nine bridge operations remain in this phase.
A registry resolves a single configured cloud implicitly; several require a
named destination. Sharing checks initiative policy, allowed destinations and
the secret guard. Pull creates a local copy; soft links retain remote references.
Local reads do not claim to have searched a cloud.

Remote supplements run after local commit, including capture/share, explicitly
requested namespace propagation and edits between already shared endpoints.
Accounts, a cloud-primary redesign and background synchronization are separate
work.

## What the tests establish

Integration tests exercise transactional rollback, replay, history, old-vault
migration, board semantics, scoped traversal, source preservation and bounded
reads. Adapter tests check transport policy and matching definitions. These
checks establish implementation behavior. Whether a model consults memory and
uses the result well requires a separate agent benchmark with its API and host
instructions recorded.
