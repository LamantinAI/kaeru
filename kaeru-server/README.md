# Shared server runtime

`kaeru-server` is a library, not another deployed service. It owns MCP and
canonical HTTP dispatch, session admission, exports, the embedded instance UI
and reusable instance-authentication mechanisms.

Hosts pass an already opened `Arc<Store>`, resolved transport settings and
explicit `RuntimeOptions`. The runtime neither selects storage nor discovers
home configuration. Local startup/CLI/stdio live in `kaeru-local`; the central
TiKV profile and compatibility REST adapter live in
`kaeru-cloud/memory-service`.

The frontend source is `kaeru-viz`; `npm run build:embedded` writes the single
checked-in `ui-dist` bundle. Both executables consume it. Run workspace gates
from the repository root; see `docs/structure.md` for module ownership.
