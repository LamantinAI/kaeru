# MCP diagnostics

Diagnostics are disabled by default. Set
`KAERU_MCP_DIAGNOSTICS_INTERVAL_SECS` to a positive number to log periodic
logical-session counts and HTTP handler counters, plus individual HTTP handler
timings. The existing `KAERU_MCP_LOG_LEVEL` must allow `info` events.

```sh
KAERU_MCP_DIAGNOSTICS_INTERVAL_SECS=10 kaeru-mcp
```

The `kaeru_mcp::diagnostics` target logs:

- `logical_sessions`: retained streamable-HTTP MCP sessions, including clients
  that disconnected without closing their logical session. This excludes legacy
  SSE sessions and browser login sessions. If the manager is locked, the field
  is omitted rather than blocking the sampler.
- `sessions_issued`: session IDs issued in HTTP responses since startup.
- `session_slots_in_use` and `session_capacity`: admission capacity, including
  reservations and workers/handlers still closing. This can exceed the number
  currently visible in the logical-session lookup map.
- `http_started`, `http_returned_headers`, `http_abandoned_handlers` and
  `in_flight_http_handlers`: handler lifecycle counters. Cancellation releases
  the in-flight counter. These are independent atomic observations, not a single
  transactional snapshot.
- Per-response method, fixed route category, status, response-header latency
  and a session fingerprint for correlation. Custom mount paths use `other`.

Response-header latency is **not** the time until an SSE stream ends. The
counters are not counts of connected sockets, live streams or graph operations.
Requests and responses are not buffered or collected by this diagnostics layer;
bodies, query strings, cookies and authorization headers are not logged.
Memory measurements must be collected separately: RSS alone cannot distinguish
live objects from freed pages retained by the allocator. These diagnostics do
not expose allocator, internal replay-cache or response-send task counts.

For a parallel experiment, use a separate vault, config files and listener.
Never open a vault already held by another daemon. Explicitly point
`KAERU_MCP_CLOUDS_FILE`, `KAERU_MCP_WEB_CONFIG_FILE`, `KAERU_READ_TRACE_DIR`
and any artifact directory to the experiment's own paths. Disable update checks
with `KAERU_MCP_UPDATE_CHECK=0` when an isolated local run is required.
Use rotating log files and explicit CPU, memory and runtime limits for a soak
test. Enabling diagnostics does not alter session expiry or repair retention.

The [transport capacity settings](instance-ui.md#mcp-transport-capacity) limit
retained stateful sessions. Idle expiry defaults to five minutes and protects
pending operations and response delivery; zero disables it. Cleanup logs the
number of expired sessions. Exhausted admission returns 503 without evicting
active clients.
