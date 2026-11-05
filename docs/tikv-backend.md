# TiKV backend and development stand

The default Compose stack runs one Placement Driver (PD) and one TiKV server using
pinned official images. PD supplies transaction timestamps and region metadata;
application values travel directly between the client and TiKV.

This stand is for synthetic backend integration tests. Starting it does not
change the storage backend of an existing Kaeru daemon or migrate a vault.

## Enable the backend

Memory-service defaults to TiKV. The optional local host keeps RocksDB and can
explicitly enable the same backend for controlled use:

```sh
cargo build -p kaeru-local --features storage-tikv --locked
```

For an explicitly remote local-host image, pass `--build-arg CARGO_FEATURES=storage-tikv`
to `docker/Dockerfile.local`. The memory-service image includes TiKV by default.

The same feature is available in `kaeru-core`, `kaeru-server`, `kaeru-memory-service` and `kaeru-rig`.
The engine remains in-process; only its transactional KV storage is remote.
Select the backend explicitly before starting the binary:

```sh
export KAERU_STORAGE__BACKEND=tikv
export KAERU_STORAGE__END_POINTS=pd.example:2379
export KAERU_STORAGE__NAMESPACE=development-memory
cargo run -p kaeru-local --features storage-tikv --locked
```

Use a comma-separated endpoint list for several PD members. Nested storage
settings use `__`; existing variables such as `KAERU_VAULT_PATH` retain their
single-underscore spelling. `Store::open(path)` explicitly selects RocksDB;
`Store::open_in_memory*` explicitly selects memory. Adapters use
`Store::open_with_config` to honor backend selection. A build without the
feature rejects `tikv` configuration instead of falling back to local storage.

A nonempty namespace is mandatory in Core. Every key is prefixed; whole-store
scans and backups exclude lease metadata. One namespace permits **one active
engine runtime**, shared by its client sessions. A heartbeat renews ownership;
write transactions validate a separate token so a former owner cannot commit
after takeover. Lease loss requires reopening the Store and its caches.
Opening a second Store for the same namespace fails. Use distinct namespaces
for independent Stores. A namespace is not a tenant security boundary.

For a secured cluster, provide all three client credentials:

```sh
export KAERU_STORAGE__TLS__CA=/run/secrets/tikv-ca.pem
export KAERU_STORAGE__TLS__CERT=/run/secrets/tikv-client.pem
export KAERU_STORAGE__TLS__KEY=/run/secrets/tikv-client.key
```

They apply to both PD and TiKV. `KAERU_STORAGE__KEYSPACE` optionally selects an
existing PD keyspace on an APIv2-enabled cluster; the application does not
provision it. `KAERU_STORAGE__OPTIMISTIC=true` selects optimistic transactions;
the default is pessimistic.

RPC defaults: 3-second request timeout, 100 rows per scan batch, 16MiB decoding
limit, 6MiB maximum value and a 60-second ownership lease. Scans and batch reads
reduce their batch size if a response exceeds the decoding limit. Encoded keys
must fit TiKV's 8192-byte limit, including namespace and keyspace prefixes.
These settings live in `TiKvConfig` and have matching nested environment keys.

## Execution and recovery

Standalone scripts and multi-script transactions share Core's persistent
blocking worker. Initialization also runs outside Tokio. API version 1,
recorded seconds and AMG schema 1 stay unchanged. Validity scans, transactional
FTS, rollback and namespace isolation use the same memory contracts.

The application never retries an accepted transaction automatically. If a
primary commit acknowledgement is lost, the engine reports an **unknown commit
outcome** and refuses further use of that runtime. Reopen and reconcile the
operation; an error is not proof that the write failed. For canonical API
commands, preserve the original `request_id` when checking their stored outcome.

Mnestic's optional `storage-sqlite` feature provides a full raw-KV backup and
restore path, including recorded versions and FTS data. Core does not enable
SQLite by default. Bulk restore stays atomic and can fail on server transaction
limits; it does not publish partial batches. Large production backup/restore
and failover need separate qualification. AMG transfers current knowledge and
is not a history backup.

Standalone TiKV does not automatically advance its MVCC GC safepoint. An
operator must coordinate GC with active snapshots, restores and backups;
physical MVCC versions are separate from Kaeru's explicit recorded versions.
This integration never advances a shared cluster's safepoint itself.

## Default Compose stack

`docker compose up --build -d` starts the bundled PD, TiKV and memory-service.
Set `KAERU_MEMORY_API_TOKEN` first. Internal advertised addresses are `pd:2379`
and `tikv:20160`; PD/TiKV ports are not published. The stack owns persistent
PD/TiKV volumes and uses namespace `memory` unless explicitly overridden.
It is a one-store development topology, not a production HA cluster.

`docker compose --profile local up --build -d kaeru-local` selects only the
optional private host: RocksDB, no dependency on PD/TiKV and no cloud sharing.
An explicit target matters because profiles add to the unprofiled default stack.

## Inspect and stop the stack

```sh
docker compose ps
docker compose logs --tail 100 memory-service
docker compose exec -T pd curl -fsS http://pd:2379/health
docker compose exec -T pd curl -fsS http://pd:2379/pd/api/v1/stores
docker compose restart memory-service
docker compose stop
```

The PD/store addresses are Docker service names, reachable by memory-service
on the same network. They are not published or advertised as host loopback.
There is one Compose file for both central and local profiles.

Normal stop/start retains named data volumes. `docker compose down -v` removes
the stack **and its data volumes**; use it only for disposable data.

## Stand limits

- One PD and one TiKV provide no high availability. Resource limits and the
  reduced RocksDB cache suit development, not production sizing.
- Storage uses an internal Docker network without TLS; its ports are not
  published. A secured deployment needs explicit server/client TLS settings.
- TiKV provides transactional KV, not SQL. Do not mix RawKV writes with the
  transactional API in the same key range.
- Standalone TiKV needs a coordinated MVCC GC and backup policy. These
  containers never advance a shared cluster's safepoint automatically.
- AMG exports current knowledge; they are not full-history storage backups.

## Integration checks

Native Core integration tests require a test runner on the same Docker
network, or an external test cluster whose PD and advertised store addresses
are reachable by that runner. Set `MNESTIC_TIKV_PD_ENDPOINT` accordingly, then:

```sh
cargo test -p kaeru-core --features storage-tikv --test tikv_backend -- --include-ignored --test-threads=1
```

The checks exercise a Tokio host, public mutations, history, rollback, FTS,
ownership and reopen in a unique synthetic namespace. Optional
`MNESTIC_TIKV_CA`, `MNESTIC_TIKV_CERT` and `MNESTIC_TIKV_KEY` select client
credentials for a TLS test cluster. The normal workspace gates remain
self-contained; live tests are ignored unless explicitly selected.
