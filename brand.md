# kaeru — brand & palette

The visual identity comes from `kaeru-viz`: an **archive / old-document**
aesthetic — warm parchment ink on a greyish-black ground, a single vermillion
"seal" accent, and ochre/amber gold for the load-bearing marks. Not neon,
not pure black; aged paper and lamplight.

Source of truth: `kaeru-viz/src/style.css` (`:root`) and `kaeru-viz/src/main.js`
(palette constants).

## Core palette

| Role | Hex | Notes |
|---|---|---|
| **Ground** (bg) | `#2a2a2f` | greyish-black backdrop — *not* pure black; used for canvas + fog |
| **Panel / ink-black** | `#121215` | near-black surfaces (panels, HUD) |
| **Parchment** (fg) | `#ece8df` | warm off-white — primary text |
| **Warm white** (glow) | `#fff3df` | node-core highlight / star glow |
| **Seal** (vermillion) | `#c8402e` | the one red accent — headings, hot marks, seal |
| **Ochre / gold** | `#caa24a` | secondary accent — core marks, gilt lines |
| Dim text | `#8b887e` | muted labels |
| Faint text | `#5a5851` | least-emphasis |
| Hairline | `rgba(236,232,223,0.10)` → `0.18` | rules / borders (parchment at low alpha) |

## Galaxy accents (the graph itself)

| Role | Hex | Notes |
|---|---|---|
| Cross-project bridges | `#cf8b2e` | bright ochre lines between projects |
| Bridge nodes | `#e6a83c` | amber points where bridges land |
| Star dust | `#9a968c` | faint background particles |

## Functional colors (encode meaning, not brand)

**Memory layer** (node dots) — importance, warm→cool as it cools:

| Layer | Dot | Legend bar |
|---|---|---|
| core | `#ffce4d` (amber gold) | `#caa24a` (ochre) |
| hot | `#ff7a45` (orange) | `#c8402e` (vermillion) |
| warm | `#6f95ff` (blue) | `#7e96cf` |
| cold | `#5566a0` (slate) | `#6f7da0` |
| frozen | `#46506f` (deep slate) | `#566666` |

**Tier** — hippocampus vs cortex:

| Tier | Hex |
|---|---|
| operational (hippocampus) | `#ff6ad5` (magenta) |
| archival (cortex) | `#56d0ff` (cyan) |

**Edge types** are keyed by hue (teal `derived_from`, amber `causal`, red
`contradicts`, green `verifies`, …); **per-project** cluster hues are spread by
the golden angle (HSL, sat 0.95, light 0.6). These are data encodings — vary
them freely; they're not part of the brand.

## The three headline colors

- **Orange** — `#ff7a45` (hot) / seal `#c8402e`: energy, the "hot" and the seal.
- **Ochre / gold** — `#caa24a` / bridge `#cf8b2e`: the gilt, load-bearing marks and links.
- **Black** — panel `#121215` / ground `#2a2a2f`: aged, greyish ink-black, never flat `#000`.

On parchment `#ece8df`.

## Typography

- **Serif** — *Zen Old Mincho* (display / titles) — the archival, brush-cut feel.
- **Sans** — *IBM Plex Sans* (UI / body).
- **Mono** — *IBM Plex Mono* (ids, code, tabular numbers).
