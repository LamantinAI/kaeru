# Memory API

Kaeru exposes 35 named operations: 16 frequent operations, ten local maintenance
operations, and nine cloud operations. MCP, named HTTP routes and Rig consume
one catalogue and the same local dispatcher in `kaeru-core::api`.
There is no query DSL or generic execute tool. Old tool names and HTTP aliases
are removed. Existing vaults migrate on open; their recorded history remains. Older records
without initiative membership become discoverable under `legacy-unscoped`,
including retained identities. This changes current membership metadata only.

## Frequent operations

| Operation | Purpose |
| --- | --- |
| `context` | Restore rules, current slots, open work, reminders and saved reasoning. `view` selects work, map, board or diagnostics. |
| `search` | Literal lexical text and explicit filters across named initiatives. |
| `read` | Full record, currentness, conflicts and saved reasoning. Optional relations, provenance and directed path. |
| `note` | Observation, decision, constraint, plan or outcome. |
| `reference` | Exact supplied source text, an authorized artifact, or a URL citation without fetching. |
| `task` | Open work with an optional ISO deadline. |
| `task_update` | Status or deadline; `due: null` clears it. Status changes preserve deadlines. |
| `claim` | A hypothesis, optionally with its already reached status and evidence. |
| `verdict` | Decide supported/refuted/inconclusive, optionally recording the completed check atomically; `record_check` leaves status alone. |
| `revise` | Edit the same identity; immutable source bytes require replacement. |
| `replace` | Explicit successor and retirement, preserving lineage and reporting dependents. |
| `relate` | Connect, disconnect or reweight a typed directed edge. Weight is required for connect/set_weight. |
| `review` | Open a doubt, answer an open record, or selectively close review flags. Counterevidence remains. |
| `trail` | Save an ordered route with an authored summary; refresh or extend an existing route. |
| `pin` | Set or clear a persistent attention pin. |
| `reflect` | Inspect integrity, unfinished work, staleness, scope and sharing proposals without applying semantic changes. |

## Occasional local operations

| Operation | Purpose |
| --- | --- |
| `lifecycle` | Change operational/archival tier while preserving content and references; report any identity change. |
| `synthesise` | Create a synthesis with explicit sources and provenance. |
| `forget` | Retract a record and its edges; keep recorded history. |
| `history` | Versions, metadata audit descriptors and accessible lineage for one identity. |
| `retention` | Set a recall layer, page through hygiene proposals and reasons, or explicitly run a pass with a durable log. |
| `initiatives` | Discover permitted project names. |
| `initiative_manage` | Rename, merge, delete, attach membership or change a current slot pointer. |
| `board_config` | Configure columns, their order and explicit terminal flags. |
| `policy` | Read or change sharing policy and allowed clouds. |
| `export` | Write scoped graph JSON and Markdown into an authorized artifact root. |

## Cloud operations

`clouds`, `share`, `unshare`, `cloud_initiatives`, `cloud_recall`, `pull`,
`link_cloud`, `cloud_links`, `sync_review` retain their existing network contracts.
They are optional in embedded Rig deployments. Local reads do not search the
cloud. With several configured clouds, an operation must name its destination.

## A small workflow

An initiative is a project scope. These examples use `billing`, which is an
example project name. They are tool arguments; send the same object to the
matching HTTP route, or to the Rig tool with the `kaeru_` prefix.

First recover what matters now with `context`:

```json
{"initiative": "billing"}
```

This returns standing rules, current records, open work, reminders and saved
reasoning. `context` also offers `work`, `map`, `board` and `diagnostics` views.
Use `search` to find something outside that initial context:

```json
{"initiatives": ["billing"], "text": "rounding", "prefix": true}
```

Search is lexical. `prefix: true` helps find word inflections; the text is not a
query language. Search hits can be excerpts. Read a returned ID for full content,
its currentness, material conflicts and saved reasoning:

```json
{"initiative": "billing", "node": {"id": "<returned-id>"}}
```

Search hits already carry `currentness`: `superseded`, `contested`, `refuted`
or `current`. A superseded hit names accessible replacements in `successors`;
by default, search includes those records even when they do not match the
literal text. An ordinary `read` includes material counterevidence in
`conflicts`. These fields describe only records the caller may access.

To inspect all accessible links, in both directions and with their edge types,
read the selected record with `"include": ["relations"]`:

```json
{"initiative": "billing", "node": {"id": "<returned-id>"}, "include": ["relations"]}
```

To save a decision, call `note`:

```json
{
  "initiative": "billing",
  "kind": "decision",
  "name": "rounding-policy",
  "body": "Round only the final invoice total."
}
```

Keep the returned ID and revision for later calls. Use `task` for work to do,
`claim` for a hypothesis, and `reference` for a source whose supplied text must
remain exact. `note` also represents observations, constraints, plans and outcomes.
A URL citation records the URL without fetching its content.

## Choose the operation by what changed

| Situation | Operation and effect |
| --- | --- |
| Correct wording or extend the same record | `revise` appends a content version under the same ID. |
| A different fact replaces the old one | `replace` creates a successor, links new to old with `supersedes`, retires the predecessor and reports dependents. |
| Work has settled into durable knowledge | `lifecycle` changes the tier, preserves content and reports any successor ID. |
| Another record is now the current handoff | Assign `options.slot` on capture, or use `initiative_manage` to assign an existing record. |
| Keep something in attention | `pin` sets a persistent pin. |
| Save how a conclusion was reached | `trail` stores an ordered route and an authored summary. |

A slot selects one current record for a purpose, such as `handoff`. Changing its
holder does not refute, retire or move the previous holder. Slots have their own
revision and optional `expected_slot_revision`. Pins are independent of slots.

For a hypothesis whose check is already complete, `claim` can include
`initial_status` and evidence in the same call. For an existing claim, use
`verdict` with an explicit `supported`, `refuted` or `inconclusive` status:

```json
{
  "initiative": "billing",
  "hypothesis": {"id": "<claim-id>"},
  "status": "supported",
  "evidence": {"method": "Checked all rounding boundary cases; totals matched."}
}
```

Evidence can also reference an existing node. The less common
`verdict(action="record_check")` records a completed check without changing the
claim's status; it accepts no `status` argument. No separate evidence tool or
mandatory claim/check/verdict sequence is required.

## Scope and validation

Ordinary project calls require `initiative`; search requires an `initiatives`
list. Exact names resolve inside that scope. Duplicate names require an ID;
the API does not silently choose the newest record. Newly generated IDs are
UUIDs, and custom IDs from older vaults remain accepted.

An ID does not bypass transport authorization or reveal a record in a forbidden
scope. Operations that accept foreign endpoints can use an explicit scoped
reference. The trusted local owner can also recover an orphaned retired identity
by its known ID when it has no current memberships. Restricted viewers do not
receive this recovery exception.

Unknown fields are rejected. Dates use `YYYY-MM-DD`; historical reads use an
RFC3339 `recorded_at` with an explicit offset. Handlers apply the documented
schema defaults. Administrative namespace operations require reconciliation
before merging competing slot holders or two board registries.

## Read the result before retrying

The 26 local operations return a shared JSON envelope. This illustrative `note`
result includes all mutation collections; IDs and revisions are placeholders:

```json
{
  "api_version": 1,
  "operation": "note",
  "state": "applied",
  "request_id": null,
  "replayed": false,
  "data": {
    "primary": {"id": "<returned-id>", "revision": "<opaque-revision>"},
    "created": [{"id": "<returned-id>", "revision": "<opaque-revision>"}],
    "affected": [],
    "changes": [],
    "evidence": [],
    "failed_steps": [],
    "slots": [],
    "artifacts": []
  },
  "issues": []
}
```

| State | Meaning |
| --- | --- |
| `ok` | A read succeeded. Inspect its coverage for omissions. |
| `applied` | The operation changed state. Returned identities describe the committed outcome. |
| `unchanged` | The requested state already held. |
| `partial` | Some effects succeeded and another step failed. Inspect committed IDs and `failed_steps`. |
| `rejected` | The command was refused without a semantic commit. Inspect `issues`. |

For example, optional linking or sharing can fail after a successful local
capture. Retrying the whole capture without a stable key could duplicate it.
A host-supplied `request_id` binds the caller, operation and normalized arguments
for a 24-hour replay window. Reuse it to recover an uncertain outcome.
`replayed: true` describes the original result. Reusing the key with different
arguments is rejected. Use `expected_revision` for supported updates when a
concurrent edit should cause a conflict rather than overwrite it.

MCP structured content and text contain the same JSON. Local tools advertise
output schemas. Rig names tools `kaeru_<operation>`; HTTP uses
`POST /v1/<operation>` with the same arguments. API version 1 is independent of
the crate version and the MCP protocol version. The instance visualizer uses
these same operations, with optional browser authentication, preserving the
operator scope ceiling and egress rules. See [instance access](instance-ui.md).

## Complete content and incomplete coverage

Reads have a response budget. `coverage` reports omissions; `body_range` shows
which part of a body is present. When `continuation` exists, call the same
operation with `continuation.next_arguments` unchanged. Repeat until there is
no continuation. Source pages reconstruct the supplied UTF-8 text exactly.

Cursors belong to the caller, arguments, policy and store generation. They
expire after 15 minutes, and writes invalidate them. A stale cursor requires
restarting the read. Snapshots are limited to 8 MiB each and 16 concurrent reads
per store; larger sources remain available through authorized export.

Truncated standing rules explicitly report incomplete coverage. A reminder is
marked as seen only when fully delivered. Saved reasoning summaries are bounded
and report truncation or staleness; expanding a trail does not certify an outdated
explanation. Local reads do not imply that a cloud was searched.

## History and existing vaults

Use `history` to list recorded versions and available lineage. Long bodies in
that listing are excerpts with ranges. Use `read` with the selected
`recorded_at` to inspect a full historical version, following its continuation:

```json
{
  "initiative": "billing",
  "node": {"id": "<returned-id>"},
  "recorded_at": "2026-10-10T12:00:00Z"
}
```

Type, tier, name, body and tags retain substrate versions. Layer, visibility,
properties and junction memberships are current metadata. Historical reads label
that distinction; an audit entry can identify a metadata change without retaining
its old value. There is one recorded-time axis, with no independent valid-time
axis for when a fact was true in the world. Replacement and forgetting preserve
history; forgetting is a retraction, not physical erasure.

Existing vaults migrate on open. Migration 0010 adds API bookkeeping. Migration
0011 restores missing legacy membership junctions and assigns previously
unscoped domain identities to `legacy-unscoped`, including retired identities.
It preserves IDs, bodies and stored validity. Old tool names and HTTP aliases
have no compatibility dispatcher; update clients before using this API.

Intrinsic graph commands use one Cozo transaction for nodes, edges, memberships,
audit and replay. Storage failures, domain failures and panics roll it back.
Network supplements run after local commit. Filesystem exports and entire
hygiene passes are separate effects; their results do not promise rollback of
an external file, remote write or whole pass.

For the implementation, start with [the code map](structure.md). The shared
catalogue is in `kaeru-core/src/api/surface.rs`, and output types are in
`kaeru-core/src/api/types.rs`. MCP `tools/list` returns the active tool definitions.
