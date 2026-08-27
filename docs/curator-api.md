# The curator API

The agent-facing surface: ~40 verbs, grouped by what they do. The taxonomy is
meant to map to how an agent already reasons — capture what you notice, link it,
test claims, settle what holds, recall structurally, tidy up — rather than expose
raw graph operations. Every verb takes an optional `initiative` scope.

Names below are the MCP tool names; the `rig` adapter exposes the same set as
`kaeru_*` tools.

## Re-entry & session

The ritual at the start of a session: process state, then epistemic state.

| Verb | Does |
|---|---|
| `initiatives` | List every project the substrate knows. |
| `awake` | Restore a project's context: layered operational working set (`core→hot→warm`), the archival **cortex** slice, session pins, recent episodes, open reviews, plus the read-back of unfinished work — open tasks (overdue first), claims awaiting a verdict, and the saved trails. |
| `overview` | A readable map of what the project's memory knows (subgraph). |
| `recent` | Episodes asserted within a recent window. |
| `pin` / `unpin` | Persist / release a node in the active-window set across restarts. |
| `config` | Read effective daemon configuration — vault path, the configured clouds and which is default, and every cap. |

## Capture — match the verb to epistemic status

Not by length; by what the content *is*.

| Verb | For |
|---|---|
| `jot` | A fleeting note, no name needed (auto-named). Operational. |
| `episode` | An observation tied to current work, may evolve. Operational. |
| `cite` | A settled document kept verbatim — specs, ADRs, persona/entity records, glossaries, external sources (URL optional). Straight to archival. |
| `task` / `done` | Actionable todos with a `due`; close with `done`. |
| `claim` | A hypothesis to test (see the hypothesis cycle). Operational. |

All capture verbs take `layer` (default `warm`) and `visibility: shared` (capture-and-share in one call).

## Link & chain — turn nodes into structure

| Verb | Does |
|---|---|
| `link` / `unlink` | Create / retract a typed edge (`derived_from`, `contradicts`, `causal`, `refers_to`, `part_of`, `blocks`, `targets`, …); `weight` / `strong=true` sets connection strength. |
| `reweight` | Change an existing edge's weight. |
| `path` | Preview the strongest weighted path between two nodes (no save). |
| `chain` | Save that path as a recallable trail, with an agent-authored `summary`. Idempotent (dedups identical trails). |
| `why` | The saved reasoning leading to a node — a chain's ordered steps, or the chain a node belongs to. Replaces the former `chains` + `read_chain`. |
| `rechain` | Regenerate a chain between its endpoints (picks up graph changes) or extend it to a new node. |

## Recall & lookup — structural first

| Verb | Does |
|---|---|
| `recall` | Exact name → id. |
| `search` | Full-text fuzzy fallback when the exact name is forgotten. |
| `drill` | A node plus its drill-down children (excerpts). |
| `trace` | Follow provenance (`derived_from`) back to sources. |
| `between` | The edges linking two nodes, both directions. |
| `tagged` | Read by tag, exact match. `topic:` tags are a node's most-mentioned words (a chosen name counts triple, compounds are also tagged by their parts). A miss returns the near tags that exist in scope rather than a bare empty list. |
| `at` | Read a node **in full** (whole body + every field), at NOW or as-of a past `when:`. |
| `history` | The assertion / retraction timeline of a node. |
| `surface` | Pull archived `cold` / `frozen` layers that `awake` withholds. |
| `ideas` / `outcomes` | List archival ideas / settled outcomes (cortex reads). |

## Hypothesis cycle

| Verb | Does |
|---|---|
| `claim` | Record a hypothesis. With `--verdict` (+ optional `--by`) it lands settled in one call — the usual case, since you reach memory after the check has run. Without one it stays `open` and keeps surfacing in `awake`. |
| `evidence` | Record what was actually checked: `--method` writes it up as an experiment node, `--node` registers one you already captured. Past tense. |
| `confirm` / `refute` / `inconclusive` | Mark it supported / refuted / undecided. `--by` (the evidence) is optional — a verdict with no citation still beats one buried in the body. |

## Evolve knowledge — when it changes shape

| Verb | Does |
|---|---|
| `synthesise` | Converge several operational seeds into one durable insight. |
| `settle` | Promote a node that stopped changing into the archival tier (`derived_from` preserved). `settle <name>` alone is enough — name, body and manual tags carry over, the type is derived. |
| `unsettle` | Bring an archival node back to operational for rework (mirror of settle; same in-place defaults). |
| `supersede` | Replace a node with a new version (bi-temporal retraction of the old). |
| `revise` | Amend a node's content. |
| `flag` / `resolve` | Raise / clear an `under_review` (a `contradicts` edge). |
| `layer` | Re-file a node's memory layer after creation. |
| `forget` / `improve` | Retract a node (bi-temporal, recoverable) / refine it. |

## Initiatives

| Verb | Does |
|---|---|
| `rename_initiative` | Move a whole project to a new name (fails if the name is taken). |
| `delete_initiative` | Drop scoping; forget nodes exclusive to it (recoverable via `at`). |
| `attach` | Give a node a second home in another initiative — additive multi-membership, the repair for fragmentation. |

Local by default; `cloud="<name>"` on rename/delete also applies it in that cloud, team-wide. The cloud is never defaulted for these two — they affect everyone and the cloud has no undo.

## Sharing — the team cloud

Explicit, gated (initiative policy + secret guard); nothing leaves automatically.

| Verb | Does |
|---|---|
| `policy` | Read / set an initiative's `share_policy` (`private` / `team`). |
| `share` | Push a node to the shared cloud (runs the two gates). Re-sharing a corrected node updates the cloud copy in place — the push is an upsert under the same id. |
| `unshare` | Withdraw a node from a cloud: retracts the cloud copy (bi-temporally — history survives) and marks it `local` again. `share`'s inverse. |
| `cloud_recall` | List what the team has shared. |
| `pull` | Bring a shared node into the local graph. |
| `link_cloud` / `cloud_links` | Reference a cloud node from a local one without copying, resolved on demand. |
| `clouds` | What this daemon can reach: names, endpoints, which is default. |
| `sync_review` | Batch-split still-local nodes into propose-share vs keep-local. |

A single daemon can target several named clouds via `clouds.toml`; cloud verbs take a `cloud:` argument.

**With one cloud configured it is optional** — there is nothing to disambiguate. **With several it is required**: an unnamed call is refused rather than routed to the default, and the refusal lists the choices. The default was invisible in both directions — a read answered by one cloud while the nodes lived in another looked exactly like an empty answer, and the same silence sat under `delete_initiative`. A refusal costs one retry; a cloud write cannot be undone in the wrong place.

Every cloud result and error names the cloud it touched, and an empty `cloud_recall` distinguishes "this initiative is empty here" from "this cloud has never heard of it".

`clouds.toml` is read once, at daemon startup. An edited file does not take effect until the process restarts — a client-side reconnect does not respawn it.

## Maintenance & export

| Verb | Does |
|---|---|
| `lint` | Orphan nodes (no edges) + unresolved reviews — the raw hygiene list. |
| `reflect` | The computed maintenance work-list: orphans to link, stale chains to `rechain`, settled work to promote into cortex, and shared/cloud items escalated to the user. Built for a periodic (cron) pass. |
| `export` | Snapshot an initiative to an Obsidian-friendly markdown vault. |
