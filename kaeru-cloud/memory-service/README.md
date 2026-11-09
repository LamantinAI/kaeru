# Memory service

`kaeru-memory-service` is the central MCP/HTTP memory server. It replaces the
previous cloud server and supports direct agent access without a local install.
It composes the shared `kaeru-runtime` runtime around one Store, defaulting to
transactional TiKV. `kaeru-local` remains an optional private embedded host.

```sh
cargo build -p kaeru-memory-service --locked
KAERU_STORAGE__END_POINTS=pd.example:2379 \
KAERU_STORAGE__NAMESPACE=team-memory \
KAERU_MEMORY_API_TOKEN=replace-with-a-long-random-secret \
  target/debug/kaeru-memory-service
```

TiKV support is a default package feature. The binary requires `storage-tikv`;
removing default features never silently creates an embedded-default service.
Core/local/Rig defaults remain RocksDB independently of workspace feature
unification. An explicit `KAERU_STORAGE__BACKEND=memory` or `rocksdb` override
is available for deliberate isolated development; set a separate vault path
when choosing disk storage. There is no fallback after a TiKV connection error.

## Configuration and access

Service defaults < legacy `KAERU_CLOUD_*` values < `KAERU_MEMORY_*` values.
Storage/caps still use `KAERU_*`; nested storage fields use double underscores.
Local `KAERU_MCP_*`, home `clouds.toml` and home `web.toml` are not discovered.

| Variable | Purpose |
| --- | --- |
| `KAERU_MEMORY_LISTEN_ADDRESS`, `LISTEN_PORT` | Default127.0.0.1:9877. |
| `KAERU_MEMORY_API_TOKEN` | Instance bearer for MCP, canonical HTTP and legacy sharing. Required off-host. |
| `KAERU_MEMORY_ALLOWED_HOSTS` | Additional authorities accepted by the MCP DNS-rebinding guard. |
| `KAERU_STORAGE__END_POINTS`, `NAMESPACE` | Required PD endpoint list and independent physical namespace. |
| `KAERU_STORAGE__TLS__CA`, `TLS__CERT`, `TLS__KEY` | Explicit credentials for PD/TiKV; use mounted container paths. |
| `KAERU_MEMORY_API_ALLOW`, `API_DENY` | Comma-separated canonical HTTP initiative ceiling/exclusions; empty allow grants none. |
| `KAERU_MEMORY_API_INCLUDE_LOCAL` | Explicitly permit local visibility in canonical HTTP responses. |
| `KAERU_MEMORY_WEB_CONFIG_FILE` | Optional explicit instance account file; disabled files are rejected. |
| `KAERU_MEMORY_UI_ENABLE`, `API_ENABLE`, `HYGIENE_ENABLE` | Independent runtime capabilities. |

MCP is `/mcp`, with `/sse` and `/messages` for retained SSE transport. Named
HTTP operations use `/v1/<operation>`. Compatibility routes remain `/api/v1`,
with `/health`, `/openapi.json` and `/docs`. That Swagger document describes
only the compatibility adapter. Every route uses the same Store; no second
TiKV owner is opened for sharing.

An instance token is bootstrap access to one trusted space. It does not provide
platform users, organizations, grants, revocation or tenant isolation. Control
and Auth are separate work. The legacy adapter retains its independent bearer
gate; a browser cookie does not authorize it.

The current UI uses password/cookie sessions rather than a bearer-token input.
Bearer-only service mode therefore disables browser routes. To use the instance
UI, prepare an account explicitly with the local operator command
`kaeru-mcp --init-web-auth --web-config <file>`, then pass that file through
`KAERU_MEMORY_WEB_CONFIG_FILE`. This is an instance account, not platform login.
No local account can implicitly disable service authentication.

## Deployment boundaries

Each namespace admits one active engine runtime; several clients share it.
TiKV replication does not authorize several independent memory-service owners.
Normal shutdown cancels handlers/background tasks before releasing the Store.
Crash/partition recovery, coordinated GC and production backup sizing remain
separate qualification. AMG transfers current knowledge, not full history.

The default Compose stack bundles PD, TiKV and memory-service:

```sh
export KAERU_MEMORY_API_TOKEN=replace-with-a-long-random-secret
docker compose up --build -d
```

PD advertises `pd:2379` and TiKV advertises `tikv:20160` on the Compose network;
only the memory-service port is published. The dedicated namespace defaults to
`memory`, overridable with `KAERU_STORAGE__NAMESPACE`. Persistent PD/TiKV volumes
survive normal stop/start. This one-PD/one-store development stack is not HA.

For only the private local host, with RocksDB and no cloud sharing:

```sh
docker compose --profile local up --build -d kaeru-local
```

Compose profiles are additive; the explicit service target excludes default
services. It also avoids requiring a central service token for local-only use.
Both modes live in the same Compose file. TLS/account configuration needs explicit mounts or an
override for a secured deployment. Existing local/artifact named volumes are
retained; no RocksDB vault is automatically migrated to TiKV.

## Known-card latency probe

The `known_read` Cargo example measures canonical `POST /v1/read` with an already
known ID, not search or the legacy raw-node endpoint. Use a disposable,
loopback-reachable PD/TiKV lab whose advertised addresses are reachable by both
the seeder and the service. The seeder creates a random fresh namespace; it does
not open an existing vault or accept a destination namespace.

```sh
cargo run --release --locked -p kaeru-memory-service --example known_read -- \
  seed --pd-endpoints 127.0.0.1:12379 --records 1000 --output fixture.json
```

After seeding exits and releases its single-owner Store, start the real
memory-service against `fixture.json`'s namespace and the same PD endpoint.
Enable the canonical API with `API_ALLOW=known-read-0`, `API_INCLUDE_LOCAL=true`,
and an explicit synthetic instance bearer; disable hygiene/UI for the probe.
Use the `KAERU_MEMORY_*`/`KAERU_STORAGE__*` variable names documented above.
Never open the namespace with a second engine while the service owns it.

Set `CARD_READ_BASE_URL` and `CARD_READ_TOKEN` in the client environment:

```sh
cargo run --release --locked -p kaeru-memory-service --example known_read -- \
  run --fixture fixture.json --samples 200 --rounds 2 --concurrency 16 \
  --paging-samples 50 --output reads.csv
```

The synthetic fixture has 1/4/16-KiB ASCII bodies, two recorded versions per
regular node, scoped edges, a conflict and saved reasoning on each target.
Timing includes the actual HTTP response, JSON decoding and full-body/context
validation. Connections are reused. The current canonical service path is
HTTP/1.1 JSON with instance authentication, not integrated product-tokenizer
authorization. Do not add independent authorization percentiles to these values.

The runner exercises C1, C4 and the selected concurrency in reversed round order.
Set `KAERU_READ_WORKERS` on the service to bound concurrent read transactions
(default 4; 1–64). Ordinary full reads use a 64,000-character budget; the separate default-budget
case follows continuations without retry and records real paging errors.
Control cases verify 401 for a wrong bearer and 404 for an inaccessible scope.
CSV files contain timings/status/bytes, never tokens, card bodies or cursor
values. Keep first-request observations, main read results and paging diagnostics
separate. These closed-loop local measurements do not establish a production SLO.


## Authorized-read integration probe

The `authorized_read` example adds a real Memory → Tokenizer HTTP/2 + CBOR hop
before canonical reads. It is a loopback-only integration harness, separate from
the deployed instance-auth runtime. Control's authoritative UUID-to-initiative
registry is not implemented by this example: a frozen, bijective synthetic map
binds five known-card scopes to disposable platform resource IDs.

Start an empty isolated tokenizer PostgreSQL with optional Valkey, using its
normal trusted Control and Memory service keys. Set `TOKENIZER_BASE_URL`,
`TOKENIZER_CONTROL_KEY` and `TOKENIZER_MEMORY_KEY` in the environment. Prepare
synthetic personal-space rights and one-time credentials in a private directory:

```sh
cargo run --release --locked -p kaeru-memory-service --example authorized_read -- \
  prepare --output "$LAB_DIR/authorization.json"
```

Stop the ordinary memory-service before opening the fixture namespace. Configure
the same loopback PD, `known-read-*` namespace and reader limit as for the
instance-auth probe, then run:

```sh
cargo run --release --locked -p kaeru-memory-service --example authorized_read -- \
  serve --prepared "$LAB_DIR/authorization.json" --listen 127.0.0.1:19878 \
  --include-local
```

The fixture deliberately uses local nodes; `--include-local` is the operator's
explicit visibility ceiling for this disposable lab. Production visibility is
not granted by a tokenizer decision alone. Point `known_read run` at this
listener with the issued memory credential. The external measured operation
includes authorization, the TiKV read, canonical egress, every continuation and
body/metadata validation. Compare cache-enabled and PostgreSQL-only tokenizer
instances against the same authority database; keep resource limits identical.

Each page reauthorizes the exact initiative. Caller identity binds principal,
credential fingerprint, authority revisions and map revision; raw tokens never
enter cursors or logs. Wrong or revoked credentials, denied scopes, mismatched
CBOR responses and tokenizer outages fail closed. The adapter shares canonical
cancellation, visibility checks and redaction, exposes only `/v1/read`, and has no
instance-bearer fallback. Remove disposable volumes and private credentials when
finished. This probe does not complete production Control/Memory wiring.
