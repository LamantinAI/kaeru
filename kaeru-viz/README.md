# kaeru-viz — a knowledge galaxy

A 3D visualizer of a kaeru knowledge base, built for a conference talk. It turns
the substrate into a galaxy where every project is a constellation of stars
around its core, and **replays real reasoning chains** so an audience can see
*how* an insight was reached — not just what was learned.

It consumes the shared named HTTP memory API. `initiatives` and
`context(view="map")` select visible records; paged `read` supplies graph
relationships and saved trail members. Source bodies load only for requested
records. There is no separate graph endpoint or compatibility alias.

![kaeru-viz — a vault rendered as a knowledge galaxy](screenshot.png)

## Live instance

The bundled frontend is served directly by `kaeru-mcp` at `/ui/`. It uses the
same named `/v1/` operations as agents, with a local account and JWT session.
No export, separate Vite process or Node runtime is required on the instance.
[Instance setup and development](../docs/instance-ui.md) describe account setup,
operator scope, administration, embedded builds and browser tests.

The operator's initiative and visibility ceiling remains authoritative. Full
bodies are read only when needed; stale continuations restart coherently.
Authentication/network errors remain errors instead of loading a static graph.

All fonts are self-hosted: Spectral, IBM Plex Mono and the CJK seal glyph.
Source fonts are explicitly bundled; `public/graph.json` is never a build input.

## Development

```sh
npm ci
npm test
npm run build:embedded
npm run check:embedded
```

`npm run dev` is optional for frontend development against a configured daemon;
it preserves the browser-facing Host for same-origin authentication. Production
uses the binary's bundle.

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

- **Time-lapse** — scrub or ▶ to watch the graph grow over the weeks.
- **Hover** a node (settle on it briefly) to highlight its neighbours and the
  links between them; **click** to pin that selection until you pick another or
  click empty space. Drag to orbit — auto-rotate stops while you hover or drag.
