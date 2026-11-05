# Memory service

`kaeru-memory-service` is the central MCP/HTTP memory server. It replaces the
previous cloud server and supports direct agent access without a local install.
It composes the shared `kaeru-server` runtime around one Store, defaulting to
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
