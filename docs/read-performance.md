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
before pagination. Compact graph projection and visualizer request reduction
are separate work. Existing continuation pages retain their existing generation,
policy and expiry checks; the assembly snapshot does not turn them into a
long-lived database transaction.

## Recorded comparison

On 2026-10-21, release builds of the baseline (`93f8254`) and these kernel
changes were measured serially with the identical fixture. At 2300 total nodes,
61 records in the selected initiative, median of three runs:

| Backend | Operation | Before, ms | After, ms |
| --- | --- | ---: | ---: |
| mem | map | 1930.960 | 3.367 |
| mem | work | 1936.727 | 8.350 |
| mem | read | 53.584 | 1.154 |
| rocksdb | map | 2187.694 | 5.189 |
| rocksdb | work | 2185.843 | 8.984 |
| rocksdb | read | 62.031 | 1.341 |

The same comparison covers 500 nodes (three runs) and 5000 nodes (one run).
All 18 backend/size/operation pairs matched the delivered fingerprints, page
counts and record counts. See [raw JSONL](measurements/read-performance-2026-10-21/results.jsonl)
and [measurement metadata](measurements/read-performance-2026-10-21/metadata.json).
The single 5000-node runs show direction, not a latency distribution.
