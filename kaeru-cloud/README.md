# kaeru-cloud

The **shared cloud tier** of kaeru's local/cloud memory split. An Axum REST
service wrapping `kaeru-core`: the same substrate mechanics as a local vault
(Cozo + RocksDB), but reachable over HTTP and shared between users, gated by
a bearer token.

Agents normally reach the cloud through their local `kaeru-mcp` daemon or
the embedded Rig cloud adapter. For nodes an initiative chooses to share,
these adapters call this service. Users keep
personal initiatives `private` (never leave the machine) and mark team
knowledge `shared`; sharing is gated by an initiative `share_policy` and a
deterministic pre-share secret guard. See [the architecture guide](../docs/architecture.md#migrations-and-cloud).

## What it is / isn't

- **Is:** a single shared store for a trusted group (a team, a family). One
  process owns the RocksDB writer lock; many local daemons connect over HTTP.
- **Isn't (yet):** multi-tenant. Everyone with the token shares one space,
  scoped by initiative. Per-user / per-org isolation is a future addition.

## Build & run

```bash
cargo install --path kaeru-cloud --locked
KAERU_CLOUD_API_TOKEN=replace-with-a-long-random-secret kaeru-cloud
```

By default it listens on `http://127.0.0.1:9877` and stores its vault under
the platform default (`$XDG_DATA_HOME/kaeru` on Linux). Point
`KAERU_VAULT_PATH` at a dedicated cloud vault, distinct from any local one.

### Docker

```bash
KAERU_CLOUD_API_TOKEN=replace-with-a-long-random-secret docker compose up --build
```

Or just the cloud:

```bash
docker build -f docker/Dockerfile.cloud -t kaeru-cloud .
docker run -p 127.0.0.1:9877:9877 \
  -v kaeru-cloud-data:/home/kaeru/.local/share/kaeru \
  -e KAERU_CLOUD_LISTEN_ADDRESS=0.0.0.0 \
  -e KAERU_CLOUD_API_TOKEN=replace-with-a-long-random-secret kaeru-cloud
```

## Configuration

**Service** (`KAERU_CLOUD_*`):

| Variable                      | Default     | Effect                                   |
|-------------------------------|-------------|------------------------------------------|
| `KAERU_CLOUD_LISTEN_ADDRESS`  | `127.0.0.1` | Bind address. `0.0.0.0` to expose.       |
| `KAERU_CLOUD_LISTEN_PORT`     | `9877`      | TCP port.                                |
| `KAERU_CLOUD_API_TOKEN`       | *(empty)*   | Bearer token required on every request. Empty disables auth only on loopback; non-loopback startup refuses it. |
| `KAERU_CLOUD_LOG_LEVEL`       | `info`      | `error` / `warn` / `info` / `debug` / `trace`. |

**Substrate** (`KAERU_*`, shared with `kaeru-core`): `KAERU_VAULT_PATH` and
the curator-API caps.

## Endpoints

All under a bearer-token gate except `/health`, `/openapi.json` and `/docs`.

**The authoritative description is `/openapi.json`**, generated from the
handlers themselves — browse it at `/docs`. The table below is a summary; if the
two ever disagree, the generated one is right.

| Method | Path                                   | Purpose                                   |
|--------|----------------------------------------|-------------------------------------------|
| GET    | `/health`                              | Liveness and running kaeru-core version (unauthenticated). |
| GET    | `/openapi.json`                        | The OpenAPI document (unauthenticated).   |
| GET    | `/docs`                                | Swagger UI over it (unauthenticated).     |
| POST   | `/api/v1/nodes`                        | Ingest a shared node — an upsert under the preserved id. |
| GET    | `/api/v1/nodes/{id}`                   | Fetch a node's full record (soft-link / pull). |
| DELETE | `/api/v1/nodes/{id}`                   | Retract a node — recorded history retained, idempotent. |
| POST   | `/api/v1/edges`                        | Ingest an edge between shared nodes — an upsert. |
| DELETE | `/api/v1/edges`                        | Retract an edge, by `src` / `dst` / `edge_type` in the body. |
| GET    | `/api/v1/initiatives`                  | Every initiative, with live node counts.  |
| GET    | `/api/v1/initiatives/{name}/nodes`     | Page through an initiative's shared nodes; `?q=` searches. |
| GET    | `/api/v1/initiatives/{name}/edges`     | Edges whose both endpoints are in the initiative. |
| POST   | `/api/v1/initiatives/{name}/rename`    | Rename team-wide.                         |
| DELETE | `/api/v1/initiatives/{name}`           | Delete team-wide.                         |

## Auth & TLS

The token is a static shared secret — the minimal control for a trusted
group. It travels in plaintext over `http://`, so terminate TLS in front of
the service with a reverse proxy (nginx) for any deployment reachable beyond
a trusted network. The app deliberately does not speak HTTPS itself.

The local MCP browser account/JWT configuration does not apply to this service.
The cloud keeps its own bearer-token boundary and separate vault.

## Versioning

Rides the workspace version and tracks `kaeru-core`.
