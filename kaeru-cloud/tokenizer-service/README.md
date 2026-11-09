# Tokenizer service

Standalone PostgreSQL-backed credentials and effective authorization. Valkey is
an optional derived-facts cache. The service does not open Mnestic/TiKV or call
Control during authorization. Memory and Control integration is a separate step;
the existing memory-service instance bearer is not replaced by this executable.

## Run

Set these environment variables:

| Variable | Meaning |
| --- | --- |
| `TOKENIZER_DATABASE_URL` | Tokenizer-owned PostgreSQL connection; migrations run at startup |
| `TOKENIZER_CONTROL_KEY` | Distinct internal Control/Gateway credential, at least 32 bytes |
| `TOKENIZER_MEMORY_KEY` | Distinct internal Memory-service credential, at least 32 bytes |
| `TOKENIZER_VALKEY_URL` | Optional Redis-compatible cache URL |
| `TOKENIZER_LISTEN` | Defaults to `127.0.0.1:9880` |
| `TOKENIZER_CACHE_NAMESPACE` | Defaults to `kaeru:tokenizer:v1`; isolate installations |

```sh
cargo run --locked -p kaeru-tokenizer-service
```

The service starts uninitialized. Its trusted Control caller must bootstrap a
projection before issuing credentials. `/health/live` is process liveness;
`/health/ready` is 503 until PostgreSQL and the projection are ready. Cache failure
does not make the source of truth unavailable. Startup rejects PostgreSQL with
`fsync=off`; service connections set `synchronous_commit=on`.

Use a protected service network and authenticated TLS termination for deployment.
The executable supports HTTP/2 prior knowledge on its internal cleartext hop;
the client and any proxy must be configured to use HTTP/2. Reuse one client
instead of constructing a connection for each check. The network test verifies
HTTP/2 and a single reused connection, not just an HTTP/2 configuration flag.

## Compose

Set `TOKENIZER_DB_PASSWORD` and the two service keys, then start only the explicit
target (profiles are additive to the default memory stack):

```sh
docker compose --profile tokenizer up -d --build tokenizer-service
```

This starts tokenizer PostgreSQL, Valkey and the service. Only the service port is
published, on loopback. Database and cache volumes persist across restarts.
Valkey is configured without eviction and with AOF so its storage can also host
appropriately retained future event streams; token durability comes from
PostgreSQL. No Control consumer or Memory event publisher is simulated here.

The image uses the workspace lockfile. With a matching `cargo vendor` tree,
offline builds can supply the generated source-replacement configuration as a
BuildKit `cargo_config` secret; set its directory to `/build/vendor`. This optional
build input is not copied into the runtime image. Without it the build uses the
normal registry and Git sources. Registry and Git caches are reused by BuildKit.

## API

Internal requests and responses are `application/cbor`. DTOs come from
`contracts::models`; action IDs are numbers, not Rust declaration
ordinals. Unknown fields, unknown actions, invalid masks, trailing CBOR data,
oversized bodies and unbounded batches are rejected.

Every `/v1` call requires `Authorization: Bearer <service key>`. The presented
product credential for an authorization check belongs in the CBOR request body,
not in place of the service key. Public clients do not receive service keys.

| Route | Caller and behavior |
| --- | --- |
| `POST /v1/authorize` | Memory or Control; bounded checks in one snapshot, returns 200 `allowed=true/false` with principal, checked scope and revisions |
| `POST /v1/rights/bootstrap` | Control only; validated initial snapshot, source generation/watermark and operation ID; empty store only |
| `POST /v1/rights/apply` | Control only; typed changes, expected/current source revision and operation ID |
| `POST /v1/tokens` | Control/Gateway attesting a verified session actor; one-time secret on the original create response |
| `GET /v1/tokens` | Control with `actor_id`, optional `after` and `limit`; own metadata only |
| `GET /v1/tokens/{id}` | Control with `actor_id`; own metadata, never a verifier or secret |
| `POST /v1/tokens/{id}/revoke` | Control with verified actor/operation ID; durable, idempotent revoke |
| `GET /v1/operations/{id}` | Control with either `actor_id` or `source_generation`; safe operation result only |

A valid credential with insufficient rights returns `allowed=false`; malformed
requests use 422, invalid credentials 401, caller restrictions 403, unavailable
metadata 404, update conflicts 409, oversized bodies 413, wrong content type 415,
and unavailable authorization state/dependencies 503. Replies use `no-store`.

The session actor is trusted because the authenticated Control/Gateway verified
the UI session; a browser-supplied actor ID alone is not authentication. Product
tokens, including limited CONTROL tokens, cannot issue new unrestricted tokens.
There is no token-to-token delegation or secret retrieval endpoint.

## First-run workflow

Against an **empty isolated installation**, set `TOKENIZER_BASE_URL` plus the
two service-key variables and run:

```sh
cargo run --locked -p kaeru-tokenizer-service --example roundtrip
```

The typed Rust client bootstraps synthetic principals, an organization and its
all-projects member, creates a credential, checks inherited access, adds another
project/initiative and checks access through the same credential, then revokes
it. The secret is consumed in memory and never printed or saved. A nonempty
projection is rejected rather than overwritten. This exercises the tokenizer
API directly; it does not impersonate a working Control or event consumer.

## Latency probe

Use another **empty isolated installation**, with the same three client variables
as the first-run example. This creates a synthetic projection and credentials:

```sh
cargo run --release --locked -p kaeru-tokenizer-service --example latency -- \
  --samples 1000 --write-samples 100 --concurrency 16 \
  --projects 100 --per-project 10 --users 100 --output latency.csv
```

The probe checks allow/deny results, a batch of 16 resources, new facts-cache
keys, token creation/replay/revocation and rights updates. Reads run sequentially
and with bounded concurrency; writes run sequentially. Warm scenarios discard
50 requests first. New-key scenarios publish an unchanged projection revision
outside timing, then check each initiative once, with the database already warm.
`--samples` must not exceed the number of initiatives.

Timing covers CBOR encoding, a reused HTTP/2 client, the network round trip,
response read and decoding. Percentiles include successful checks only; failures
are recorded and make the process exit unsuccessfully. The CSV contains timing,
status, correctness and byte counts, never credentials or payloads. Its path must
not already exist. Output percentiles are per RPC, including the batch case.

Optionally set `TOKENIZER_COMPARE_URL` to a second tokenizer using the **same
PostgreSQL database and service keys, without Valkey**. It receives the same
checks after the primary endpoint. The output labels these endpoints `cached`
and `postgres_only`; configure the primary with a working cache for that comparison.
Results describe the measured host, resource limits and transport, not a
production SLO or a saturation-throughput test.

## Rights and inheritance

Control owns business facts; the tokenizer persists a versioned projection.
Stable initiative IDs refer to one personal owner or one project. Projects refer
to organizations. Pending, moving and deleted initiatives are closed to normal
access; blocked organizations close ordinary data access.

An active organizational membership is required for grants inherited through it.
The all-projects grant, project grant and current-placement initiative grant are
additive. All-projects rights cover future projects. Unchecking the flag preserves
explicit grants. A new membership ID is required after leaving; old assignments
cannot revive on rejoin. Initiative moves must advance placement revision, and
old direct grants do not revive after a move back.

Organization operators can manage ordinary members' scopes without automatically
reading all organization data. They cannot manage peer operators or assign
administrative bits through data grants. Platform roles and organizational roles
are independent. Effective access intersects current account rights, purpose,
the token's current-placement project ceiling and resource state. MEMORY tokens
cannot administer users or organizations. Deleted inspection is an explicit
CONTROL action for platform operators/administrators and does not grant writing.

Multi-check operations are all-or-nothing. A move checks source and destination
in one snapshot. The policy engine returns eligibility; Control must still
enforce transactional business workflows such as an owner's successor.

## Consistency and credentials

Each authorize reads the current credential and revision from primary PostgreSQL
in a read-only repeatable-read transaction. Resource queries are scoped to the
requested IDs, ancestors and relevant grants. Valkey contains only version-keyed
facts; it never contains a bearer or a final reusable allow. Cache miss/failure
falls back to PostgreSQL. This correctness baseline intentionally retains a
database read per authorize; it is not a cache-only latency claim.

Security changes atomically increment `authz_revision`; `source_revision` orders
Control updates separately. Mutations serialize on the state row. Apply rejects
gaps, stale generations and operation IDs reused with different normalized
content. Array ordering and CBOR map ordering do not create new operations.
The first implementation validates the full projection during cold apply;
optimizing this write-side work is separate from the scoped authorization path.

After a revoke or rights update returns success, a newly started authorization
sees the new state. Already admitted work may finish using its old context;
strict cancellation/fencing is outside this service. Expiry is checked explicitly.

Credentials contain an ID and a 32-byte CSPRNG secret. Only a domain-separated
SHA-256 verifier is persisted and comparison is constant-time. Default expiry is
six calendar months UTC, clamped at the end of the month. Create/revoke wait for
PostgreSQL commit. One winning create response contains the secret; retries and
operation lookup never do. A lost create response requires a new issuance, not
retrieval of the old secret. No source secret or recoverable copy is stored.

## Implementation boundaries

`codec` owns the common CBOR representation used by HTTP, durable operation
results and optional cached facts. `api/codec` handles Axum extraction and HTTP
errors; `cache` owns only Valkey behavior. Its former codec paths remain
compatibility re-exports. Private `storage/operations` owns operation identities,
stable kinds, digests and token replay/persistence. Storage transactions remain
internal; callers use the operation-oriented `Store` methods.

## Tests

```sh
cargo test --locked -p contracts -p kaeru-tokenizer-service
```

Database/network tests are explicitly ignored without infrastructure. Point
`TOKENIZER_TEST_DATABASE_URL` to an **isolated test PostgreSQL** whose role can
create databases, and set `TOKENIZER_TEST_VALKEY_URL` for the cache integration tests:

```sh
cargo test --locked -p kaeru-tokenizer-service --test postgres --test http2 -- --include-ignored
```

Each test creates a uniquely named database and drops it afterward. Cache tests
use a unique namespace and only delete their own keys. CI runs these tests with
PostgreSQL and Valkey services. No working vault, installed daemon or real user
credentials are used. See `tests/policy.rs`, `tests/postgres.rs` and
`tests/http2.rs` for inheritance, replay/races, cache invalidation and wire checks.

The executable is the standalone milestone. Memory Authorizer wiring, stable
platform-ID to graph-scope mapping, lifecycle outbox, Control consumer and shared
mutable-node policy remain integration work. The default memory API continues to
use its existing instance authentication until those routes are integrated.
