# kaeru-cloud

The **shared cloud tier** of kaeru's local/cloud memory split. An Axum REST
service wrapping `kaeru-core`: the same substrate mechanics as a local vault
(Cozo + RocksDB), but reachable over HTTP and shared between users, gated by
a bearer token.

The local `kaeru-mcp` daemon is the agent's only surface; for the nodes an
initiative chooses to share, it proxies into this service. Users keep
personal initiatives `private` (never leave the machine) and mark team
knowledge `shared`; sharing is gated by an initiative `share_policy` and a
deterministic pre-share secret guard. See the design overview in
`research/context/design/local_cloud_split` (design vault).

## What it is / isn't

- **Is:** a single shared store for a trusted group (a team, a family). One
  process owns the RocksDB writer lock; many local daemons connect over HTTP.
- **Isn't (yet):** multi-tenant. Everyone with the token shares one space,
  scoped by initiative. Per-user / per-org isolation is a future addition.

## Build & run

```bash
cargo install --path kaeru-cloud
KAERU_CLOUD_API_TOKEN=replace-with-a-long-random-secret kaeru-cloud
```

By default it listens on `http://127.0.0.1:9877` and stores its vault under
the platform default (`$XDG_DATA_HOME/kaeru` on Linux). Point
`KAERU_VAULT_PATH` at a dedicated cloud vault, distinct from any local one.

### Docker

```bash
docker compose up --build        # starts kaeru-cloud + kaeru-mcp wired together
```

Or just the cloud:

```bash
docker build -f docker/Dockerfile.cloud -t kaeru-cloud .
docker run -p 9877:9877 -e KAERU_CLOUD_API_TOKEN=... kaeru-cloud
```

## Configuration

**Service** (`KAERU_CLOUD_*`):

| Variable                      | Default     | Effect                                   |
|-------------------------------|-------------|------------------------------------------|
| `KAERU_CLOUD_LISTEN_ADDRESS`  | `127.0.0.1` | Bind address. `0.0.0.0` to expose.       |
| `KAERU_CLOUD_LISTEN_PORT`     | `9877`      | TCP port.                                |
| `KAERU_CLOUD_API_TOKEN`       | *(empty)*   | Bearer token required on every request. Empty = auth disabled (dev / loopback). |
| `KAERU_CLOUD_LOG_LEVEL`       | `info`      | `error` / `warn` / `info` / `debug` / `trace`. |

**Substrate** (`KAERU_*`, shared with `kaeru-core`): `KAERU_VAULT_PATH` and
the curator-API caps.

## Endpoints

All under a bearer-token gate except `/health`:

| Method | Path                                   | Purpose                                   |
|--------|----------------------------------------|-------------------------------------------|
| GET    | `/health`                              | Liveness (unauthenticated).               |
| POST   | `/api/v1/nodes`                        | Ingest a shared node (id preserved).      |
| GET    | `/api/v1/nodes/{id}`                   | Fetch a node's full record (soft-link / pull). |
| GET    | `/api/v1/initiatives/{name}/nodes`     | List an initiative's shared nodes (discovery). |

## Auth & TLS

The token is a static shared secret — the minimal control for a trusted
group. It travels in plaintext over `http://`, so terminate TLS in front of
the service with a reverse proxy (nginx) for any deployment reachable beyond
a trusted network. The app deliberately does not speak HTTPS itself.

## Versioning

Rides the workspace version and tracks `kaeru-core`.
