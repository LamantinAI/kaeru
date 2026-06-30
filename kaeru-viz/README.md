# kaeru-viz — a knowledge galaxy

A 3D visualizer of a kaeru knowledge base, built for a conference talk. It turns
the substrate into a galaxy where every project is a constellation of stars
around its core, and **replays real reasoning chains** so an audience can see
*how* an insight was reached — not just what was learned.

It renders the JSON served by the kaeru-mcp daemon at `GET /graph.json`
(`kaeru_core::export_graph_json`): nodes coloured by initiative / tier / layer
and sized by memory layer (Core largest), faint membership spokes from each node
to its project core, ochre **cross-project bridges** between related projects,
and a time-lapse of the months of accumulation. Hover a node to trace its
neighbours and the links between them; click to pin that selection.

![kaeru-viz — a vault rendered as a knowledge galaxy](screenshot.png)

## Data

The export is **curated + redacted by default** (safe OSS initiatives only;
credential bodies dropped) so it is safe for a public talk. See the daemon's
`KAERU_MCP_VIZ_INITIATIVES` env / `?initiatives=…` override and `kaeru-core/src/guard.rs`.

- **Live (dev):** the dev server proxies `/graph.json` to the daemon
  (`KAERU_VIZ_URL`, default `http://127.0.0.1:9876`) — always fresh, and you can
  mutate the vault on stage and reload.
- **Baked (talk safety net):** `scripts/bake-graph-snapshot.sh [initiatives_csv]`
  writes `public/graph.json` (with a fail-closed leak gate). The built app falls
  back to this bundled copy, so it works with **no daemon and no network**.

`public/graph.json`, `node_modules/`, and `dist/` are git-ignored (the snapshot
holds real content — never commit it).

Fonts (IBM Plex Sans/Mono, Zen Old Mincho) are **self-hosted** under
`public/fonts/` — Latin + Cyrillic for the UI, and a single CJK chunk for the 蛙
seal — so the bundle pulls nothing from the network, including offline.

## Run

```bash
npm install

# dev against the live daemon
npm run dev                       # http://localhost:5173

# bake an offline snapshot, then build a self-contained bundle for the talk
../scripts/bake-graph-snapshot.sh
npm run build && npm run preview  # serves dist/ with the bundled snapshot
```

## Controls

- **Reasoning chain → ▶ replay** — the hero: flies to a saved chain and animates
  it node-by-node, surfacing each step.
- **Color by** — initiative (galaxy), tier (hippocampus/cortex), or layer (importance).
- **layer glow** — Core/Hot stars enlarge; **Focus** — isolate one project's
  subgraph and frame the camera on it.
- **Guided tour** — walks the wow-moments in sequence (a slowly rotating galaxy →
  chain replay → one project up close → memory layers → the two tiers), with
  prev/next + arrow-key navigation for a presenter remote. The opening scene
  spins the galaxy to invite the obvious parallel — a small universe of one
  mind's work.

The ochre **cross-project bridges** draw a constellation between projects, derived
from shared `topic:` tags weighted by inverse frequency (specific topics count far
more than generic words) — computed server-side by `kaeru_core::project_affinity`
and served in the export's `project_links`. The relationships are real (projects
in a vault are often interconnected) but may never have been captured as edges;
this surfaces them from content. Strong ones can also be recorded as real
cross-initiative `refers_to` edges, so they become first-class graph structure
(visible as inter-cluster lines in the galaxy).
- **Time-lapse** — scrub or ▶ to watch the graph grow over the weeks.
- **Hover** a node (settle on it briefly) to highlight its neighbours and the
  links between them; **click** to pin that selection until you pick another or
  click empty space. Drag to orbit — auto-rotate stops while you hover or drag.
