# AMG knowledge transfer

AMG (Agentic Memory Graph) transfers the current state of knowledge between
Kaeru vaults. It preserves record IDs, typed properties, memberships, local
edges, ordered trails, slots and version-bound hypothesis decisions. It can
contain every initiative, one initiative, or an explicit set of initiatives.
The current format is **AMG schema 1**, independent of Kaeru's software version
and database migration numbers.

## Operator commands

Stop the daemon using the source or destination vault before export/import.
These commands open the embedded store exclusively; a busy vault fails with a
lock error. Never remove a running daemon's lock.

```sh
# Entire current graph.
kaeru-mcp amg export --vault /path/to/source --all --output knowledge.amg

# One or several initiatives, selected by exact name.
kaeru-mcp amg export --vault /path/to/source \
  --initiative research --initiative infrastructure --output selected.amg

# Validate and inspect without opening a vault.
kaeru-mcp amg inspect selected.amg

# Preview the destination merge, then apply it.
kaeru-mcp amg import selected.amg --vault /path/to/destination --dry-run
kaeru-mcp amg import selected.amg --vault /path/to/destination

# Explicitly accept supplied conflicting current assertions.
kaeru-mcp amg import selected.amg --vault /path/to/destination --force
```

Without `--vault`, commands use the normal `KAERU_VAULT_PATH`/platform default.
Exports use an atomic file publication, refuse an existing output file, and set
owner-only permissions on Unix. Commands print JSON metadata or merge counts.
`--dry-run` checks the real destination and applies no knowledge changes; opening
the store can still perform normal bootstrap/schema migrations. Invalid files
and unsupported schemas are rejected before opening the destination.

This release exposes AMG through the local operator CLI and `kaeru_core::amg`.
It adds no MCP/HTTP agent operation or browser import/export screen.

## Container and metadata

The logical layout is `amg:meta:data`. Length prefixes keep colons inside text
or binary CBOR unambiguous:

| Section | Encoding |
| --- | --- |
| Magic | Four bytes: `amg:` |
| Metadata length | Unsigned 32-bit integer, big-endian |
| Metadata | One CBOR map of exactly that length |
| Separator | One byte: `:` |
| Graph length | Unsigned 64-bit integer, big-endian |
| Graph | One CBOR map of exactly that length |
| End | EOF; trailing bytes are rejected |

Metadata contains:

| Field | Meaning |
| --- | --- |
| `schema_version` | Portable schema version, currently `1`. |
| `exported_at` | UTC RFC3339 snapshot timestamp. |
| `software` | Producer name (`kaeru`) and software version. |
| `profile` | `knowledge-current`. |
| `scope` | `all` or `initiatives`. |
| `initiatives` | Exact selected names, or the initiative inventory for a full graph. |
| `omissions` | Counts and reasons for deliberately omitted trails/external-store edges. |

The graph map contains `nodes`, `edges`, `trails`, `slots` and `decisions`.
Exchange types live in `kaeru-core/src/models/amg.rs`; a fixed independently
encoded example lives in `kaeru-core/tests/fixtures/amg-v1-empty.amg`.

Schema 1 uses definite-length CBOR collections with unique text map keys and
JSON-compatible values. Tags, byte strings, non-finite numbers and extra fields
are rejected. The writer sorts map keys by UTF-8 byte length then bytes;
arrays retain their declared order. Do not split the file on colon characters.
Newer unsupported schemas fail before graph decoding or mutation. Older
unsupported schemas also fail until an explicit migration is implemented.

Default limits are 64 MiB per file, 64 KiB of metadata, 100,000 nodes, 500,000
edges and 500,000 auxiliary records, with bounded CBOR depth/item counts. The
library accepts explicit `AmgLimits`; the CLI uses these defaults. Limits on
bytes, nesting and collection sizes are checked before decoded graph allocation.

## Scope and closure

A selected document contains only records belonging to its selected initiatives.
Membership lists are intersected with that selection. Both endpoints of each
exported local edge must be present; adjacent outside records are never pulled
into the document. Explicit edge ownership is respected; legacy unscoped edges
use endpoint closure. With several initiatives, cross-links within their union
remain. A full export includes unscoped records and edge-only initiatives too.

A trail with any missing member is omitted as a whole and reported in metadata.
It is never shortened into a path its author did not assert. A mandatory typed
task reference or decision evidence outside the document causes an error:
select the required initiatives together instead of producing dangling meaning.
Trail freshness/fingerprints are preserved; summaries invalidated by a changed
member cannot remain fresh.

## Import and force

The importer validates the complete document and merged destination invariants,
then commits graph records and semantic metadata in one transaction. Errors roll
back the entire import, including failures after individual writes.

| Situation | Normal import | `--force` |
| --- | --- | --- |
| New identity | Create a local assertion. | Same. |
| Identical current state | No new version. | Same. |
| Conflicting current node/edge | Abort the import. | Append a new current assertion under the same ID. |
| Retracted identity supplied in file | Abort. | Restore through a new assertion; retain the retraction/history. |
| Occupied slot or conflicting trail/decision | Abort. | Accept supplied metadata; retain before-images in the import receipt. |
| Existing ID changes node type, invalid references, conflicting board identity | Abort. | Abort; force does not bypass invariants. |
| Destination record absent from file | Leave it intact. | Same. |

Source `exported_at` is provenance, not an ordering or overwrite instruction.
New assertions receive local recorded versions above prior local versions.
Node/edge history remains intact. Memberships are additive, including junctions
retained after retraction. No deletion is inferred from absence in a file.
Repeated identical imports are no-ops, with no additional receipt/audit event.

Incoming new/restored nodes are local; existing live node visibility and
initiative sharing policies remain destination-owned. Immutable source text
must match its digest even with force. A foreign `verified_artifact` becomes
`imported_artifact`; the importer neither reads its path nor claims local
verification. Force can append a validated source revision without changing the
ordinary API's immutable-source rules.

Imported assertions record producer timestamps, source visibility/origin and a
SHA-256 digest of the normalized document in version-bound provenance. The
provenance chain is bounded. Non-temporal metadata before-images and merge
counts are retained in an administrative import receipt; automatically
invalidated local trail summaries are included. These receipts are not a
complete historical reconstruction of every administrative state.

## Deliberate boundaries

AMG carries current knowledge, not historical versions, tombstones, audit
journals, accounts/tokens, session pins, delivery bookkeeping or cloud endpoint
configuration. External-store edges are omitted and counted. It is a knowledge
transfer format, not a replacement for a complete vault backup.

The `amg:` magic identifies the file. Cryptographic signatures, optional signing
keys and encryption are reserved for future work; schema 1 provides none of
these guarantees. A document digest identifies content, not a trusted author.
