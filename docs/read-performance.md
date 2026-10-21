# Read performance and cancellation

The shared memory API authorizes records before returning their contents. Bulk
reads keep this contract without repeating a full set of database queries for
each record and each incident endpoint.

- Keyed temporal queries bind their lookup key before reading the relation.
  Reverse initiative membership uses `node_initiative:by_node`. Fresh stores
  create it; migration `0013_node_initiative_by_node` repairs existing stores.
- Response assembly owns a read-only Cozo transaction. All `@ 'NOW'` queries in
  that assembly share one temporal instant and database snapshot, including
  access checks and scalar fallback reads. Writes are rejected inside it.
- A request-local loader batches 64 records. It loads access metadata before
  bodies, reuses endpoint checks, and discards heavy body/edge caches between
  batches. Policy-sensitive results cannot be reused under another policy.
  Generation changes invalidate cached values; an old result cannot populate
  a newer generation. There is no persistent cross-request content cache.
- The transaction and loader end before pagination delivery stamps reminders
  or consumes a maintenance report. These writes and returned revisions retain
  their ordinary semantics.

Hosts may call `execute_with_control` with an `ExecutionControl` cancellation
probe; ordinary `execute` remains supported. MCP passes its request token, HTTP
cancels when its handler is dropped, and Rig cancels when its call future is
dropped. Cancelled reads return `CANCELLED`, never a successful truncated view.
Checks occur at entry, after acquiring the scope guard, between database
batches/traversal steps, and before delivery. A running Cozo statement is allowed
to finish; cancellation neither kills unrelated queries nor interrupts an
already started command or delivery bookkeeping.

The store still serializes scoped API operations. A cancelled request waiting
for that guard skips its storage work after the guard becomes available. This
change does not remove the mutex or provide preemptive database cancellation.

## Reproduce the kernel measurements

The example creates synthetic data in memory or a **new temporary RocksDB
directory**. It never accepts an existing vault path or loads configuration
from the environment.

```sh
cargo run --release -p kaeru-core --example read_perf --locked -- \
  --backend mem --nodes 2300 --repeats 3
cargo run --release -p kaeru-core --example read_perf --locked -- \
  --backend rocksdb --nodes 2300 --repeats 3
```

Each node has six versions and two outgoing edges, spread across 38 initiatives.
The measured map/work scope therefore contains 61 records at 2300 total nodes.
JSONL output includes the backend, build profile, fixture size, elapsed time,
page and record counts, and a fingerprint of delivered IDs, revisions, bodies,
currentness and memberships. All continuation pages are consumed. Compare the
same example and fixture against the baseline and fixed builds, using identical
profiles/backends and no competing builds during measurement.

These are kernel read measurements, not a complete visualizer load or a
prediction for every real vault. `context` still materializes its selected view
before pagination. The visualizer uses a separate compact graph projection
through `context(view=graph)`. Ordinary continuation pages retain their existing
generation,
policy and expiry checks; the assembly snapshot does not turn them into a
long-lived database transaction.

## Compare whole-graph request patterns

```sh
cargo run --release -p kaeru-core --example read_perf --locked -- \
  --graph --backend rocksdb --nodes 2300 --repeats 3
```

This mode compares per-initiative maps followed by per-node reads with the
compact paged graph projection. Both use the same disposable fixture and kernel;
all graph pages are consumed. Output reports elapsed time, API call count,
response bytes and structural fingerprints, and checks that the node/edge
projections match. It includes JSON serialization and excludes HTTP latency,
egress redaction and browser rendering.
