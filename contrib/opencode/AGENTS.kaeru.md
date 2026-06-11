# kaeru — agent memory

`kaeru` is a typed-graph memory the user has spent time building.
Operational tier (cognitive / hippocampus) is fast working thought;
archival tier (recollection / cortex) is settled long-term knowledge.
Every node and edge is bi-temporal — assertions and retractions live
side-by-side, time-travel is native.

## Transport — MCP daemon, not the CLI

A single long-lived `kaeru-mcp` daemon owns the substrate (RocksDB is
single-writer). You interact via the MCP tools the daemon exposes;
opencode names them `kaeru_<verb>` (so `kaeru_awake`, `kaeru_drill`,
`kaeru_jot`, …). All examples below use that form.

**Do NOT shell out to a `kaeru` CLI when the MCP daemon is running.**
The CLI tries to open the same on-disk vault and will fail with
`Resource temporarily unavailable` because the daemon already holds
the lock. If you see `kaeru: command not found`, that's fine — the
CLI isn't required when MCP is wired.

## Cardinal rules

- **Initiative on every call.** Pass `initiative=<name>` to every MCP
  tool that accepts it. Without it, mutations land un-tagged and reads
  go cross-initiative — almost never what you want. The right initiative
  is usually the project name (cwd basename or git remote name). If
  unsure, ask the user once and stick with the answer for the session.
- **Store and search in the user's native language.** If they capture
  in Russian, store and search in Russian; if Japanese, in Japanese;
  if English, in English. Do NOT translate content on capture and do
  NOT translate queries on lookup. Translation creates drift between
  what was stored and what you query for, and nothing matches. Every
  node carries a `lang:ru` / `lang:en` / `lang:mixed` / `lang:other`
  tag auto-detected from the body.
- **Search is FTS without stemming** — exact tokens after lowercasing.
  For inflection-tolerant matching, append `*` to the term:
  `search query="утечк*"` finds `утечка` / `утечку` / `утечке`,
  `search query="token*"` finds `token` / `tokens` / `tokenize`.
  Works for any script; do not "translate to English to be safe".

## Personas — same primitives, different uses

The verb taxonomy looks research-flavoured (`claim` / `test` /
`confirm` / `synthesise`) but the underlying primitives are general.

### Researcher / engineer

The "default" use case. Captures observations as `jot` / `episode`,
formalizes hunches via `claim`, validates with `test` + `confirm` /
`refute`, settles findings with `synthesise` → `settle` (operational
draft → archival outcome). External sources go through `cite` with a
URL. Initiative is the project name.

### Personal manager / assistant

The agent helps with daily life — todos, people, plans, journal.

- **Tasks with deadlines:** `kaeru_task(body="купить молоко", due="2d", initiative=…)`.
  Mark complete with `kaeru_done(name=<task-name>)`.
- **People / places / things without URLs:**
  `kaeru_cite(name="Анна", body="врач семейной клиники, рекомендация Маши", initiative=…)`
  — same `cite` verb, no `url` needed. Persona records live in archival
  tier ("cortex"), so things like "who is my user" stick around forever.
- **Plans / intentions / decisions:** just `jot`. Slice later with
  `tagged tag="topic:план"` etc.
- **Daily journal:** `jot` whatever's on the agent / user's mind;
  `recent since="24h"` for "what happened today", `recent since="7d"`
  for the week.

Initiative for personal use is typically a single name like `personal`
or `daily`, or split by life area (`work`, `home`, `learning`).

### Long-term cortex (cross-initiative facts)

Things that should outlive any specific project — "who is my user",
"my preferences", "repeated correspondents", persistent locations.
Capture with `cite` (no URL) under a stable initiative like `cortex`,
or omit `initiative` entirely. The archival tier means these aren't
surfaced by a project's `awake` / `overview` and aren't crowded out
by recent thoughts; they're retrievable on demand via
`drill name=<x>` or `tagged tag="kind:reference"`.

## When to use

Auto-trigger when the user:
- Says **"remember"** / **"save this"** / **"note that"** / **"keep this in memory"**.
- Asks **"what did I think about X"** / **"what's in project Y"** / **"trace this back"**.
- (Re-)enters a project and you want continuity from previous sessions.
- Closes a thought ("decided", "settled", "this is the answer").
- Flags doubt ("wait — this looks wrong").

User-invocable via `/kaeru`.

## Re-entry ritual (do this first when picking up a project)

```
kaeru_initiatives()                              # see existing projects
kaeru_awake(initiative="<name>")                 # what was open last time
kaeru_overview(initiative="<name>")              # what does this project know
```

`awake` answers "what was I doing" (process state — pinned, recent,
under-review). `overview` answers "what does this project know"
(epistemic state — categorical breakdown, provenance forests, open
questions). Run both.

## Cadence — habits that keep the graph useful

Small per occurrence, the difference between "I cited something once"
and "next session can find it via three different paths":

- **Capture the user's ask — and your own intents — as a `task`.**
  When the user says "build X and report back" or "fix Y by tomorrow",
  that's literally what `task` was designed for: `task body="<…>"
  due="1h"`, `done name=<…>` when finished. The task node is what
  `awake` surfaces next session.
  **The same applies to your own promises.** Whenever you catch
  yourself saying "I should save X later" or "when Y comes back I'll
  do Z", capture that as a task *right then*. If MCP is reachable,
  `task` it. If kaeru is down, write a one-line entry to
  `~/road-notes.md` (or the equivalent local scratch file). Intent
  that lives only in conversation context is intent that evaporates
  the moment the session ends or you pivot to a new prompt — this is
  how findings get lost.
  Findings you derive *while* doing the task go into separate
  `cite` / `episode` / `claim` nodes; the task is the operational arc
  connecting them. Single-shot factual lookups don't need a task.
- **Drain before resume after a kaeru disconnect.** If kaeru tools
  went away mid-session and came back, your first action after the
  re-entry `awake` is to **drain the held queue**: scan recent
  conversation for findings or self-tasks you marked for-later, and
  `cite` / `episode` / `claim` each one before accepting new work
  from the user. Save first, talk second. The failure mode this
  fixes: agent makes a TODO ("save X when kaeru is back"), the
  session ends or pivots, X never gets saved because the intent lived
  only in conversation context.
- **Cite, then link.** When you `cite` a new node that's conceptually
  adjacent to one you saw earlier in this session (via `search` /
  `drill`), `link` them with `edge_type=causal` / `derived_from` /
  `refers_to`. Without edges every cite is an island; only exact-name
  lookups will find it. Costs one call per edge; pays off every time
  someone navigates in.
- **Don't `search` what you just `recalled`.** `recall name=<x>`
  returns the id; full content is `drill name=<x>`. Re-issuing
  `search query="<same words>"` after a successful `recall` is a
  redundant round-trip against a different index for the same answer.
- **Refine, don't stampede.** If `search query="X"` doesn't surface
  what you want in the top 3 hits, the next call should be a
  *different shape* — `search query="X*"` for inflection,
  `tagged tag="topic:X"` for exact-token slice, or
  `drill name=<related>` to walk in. Five variant phrasings in 20
  seconds is almost always slower than reading the first three
  results and making one targeted call.
- **Re-`awake` after long gaps.** If your last `awake` / `recent` was
  more than ~30 minutes ago and there's any chance another agent or
  another teammate's session has written to the same vault, run them
  again before assuming your view is current. The vault is shared at
  the daemon level; sibling writes only become visible on read.

## Capture (write thoughts)

```
# Quick fleeting thought — auto-named, low-significance:
kaeru_jot(body="noticed token expiry differs across platforms",
          initiative="X")

# Load-bearing observation / decision — pick a deliberate name:
kaeru_episode(name="auth-decision",
              body="platform-aware expiry policy",
              initiative="X")

# Todo with deadline (auto-named, kind:task, status:open):
kaeru_task(body="купить молоко", due="2d", initiative="X")
kaeru_task(body="созвон с командой", due="2026-05-15", initiative="X")
kaeru_done(name="<task-name>", initiative="X")

# External source OR persona/entity — both via `cite`:
kaeru_cite(name="transformer-paper",
           url="https://arxiv.org/abs/1706.03762",
           body="…",
           initiative="X")
kaeru_cite(name="Анна",
           body="врач, рекомендация Маши",
           initiative="X")
# When `url` is set, point at the canonical artifact (the actual PDF,
# the release-asset download URL, the dashboard panel) — not the API
# endpoint or metadata URL. Future `drill` exposes that URL to the next
# agent, which wants to fetch, not introspect.

# Connect two named nodes:
kaeru_link(from="from-name", to="to-name",
           edge_type="causal", initiative="X")
# Edge types: refers_to (default), causal, derived_from, contradicts,
# part_of, blocks, targets, supersedes, verifies, falsifies,
# temporal, consolidated_to.
```

## Inquire (read)

```
kaeru_recall(name="<x>", initiative="X")              # name → id (exact match)
kaeru_drill(name="<x>", initiative="X")               # brief + 1-hop neighborhood
kaeru_search(query="<q>", initiative="X")             # FTS across name + body
kaeru_search(query="<q>*", initiative="X")            # prefix glob (handles word forms)
kaeru_trace(name="<x>", initiative="X")               # walk derived_from ancestors
kaeru_recent(since="3h", initiative="X")              # episodes in last 3h
kaeru_ideas(initiative="X")                           # archival ideas
kaeru_outcomes(initiative="X")                        # archival outcomes
kaeru_overview(initiative="X")                        # full subgraph map
kaeru_tagged(tag="<t>", initiative="X")               # slice by tag — see below
kaeru_between(a="<n1>", b="<n2>", initiative="X")     # all edges between two nodes
```

`drill` is the most-used: returns a brief + 1-hop neighbors in one
round-trip. **Search results are sorted newest-first within equal
scores** — a recent capture beats a stale one when both match. If the
agent doesn't see what it expects in the top 3 results, **change the
shape of the query** (prefix glob, tagged, drill a neighbor), don't
re-phrase the same intent five times.

## Slicing by tag

Every captured node automatically gets these tags:
- `kind:<type>` — `kind:observation`, `kind:reference`, `kind:experiment`, `kind:idea`, …
- `sig:<level>` — `sig:low` / `sig:medium` / `sig:high` (significance, only for episodes that have it).
- `role:<role>` — `role:jot` / `role:review` / `role:synthesise` / `role:revised` (when applicable).
- `lang:<code>` — `lang:ru` / `lang:en` / `lang:mixed` / `lang:other` (auto-detected from body script).
- `topic:<word>` — up to 5 content tokens auto-derived from the body
  (lowercased, stop-words removed; same form as in the body, no stemming).
- `status:<state>` — only for hypotheses (`status:open`, `status:supported`, `status:refuted`, `status:inconclusive`).

```
kaeru_tagged(tag="kind:experiment", initiative="X")  # all experiments
kaeru_tagged(tag="sig:high",        initiative="X")  # high-significance only
kaeru_tagged(tag="topic:auth",      initiative="X")  # everything mentioning "auth"
kaeru_tagged(tag="lang:ru",         initiative="X")  # only Russian-language nodes
kaeru_tagged(tag="status:open",     initiative="X")  # open hypotheses
```

Topic tags use the **exact form from the body** — no stemming. If you
stored "утечку", the topic tag is `topic:утечку`, not `topic:утечка`.
For loose matching use `search query="<root>*"` instead of `tagged`.

## Reason (hypothesis cycle)

```
kaeru_claim(body="weekend deploys cause flaky tests",
            about="<related-name>", initiative="X")
# → creates hypothesis, optionally linked via refers_to.

kaeru_test(hypothesis="<name>",
           method="compared 100 runs each", initiative="X")
# → creates experiment with `targets` edge.

kaeru_confirm(hypothesis="<name>", by="<evidence-name>", initiative="X")
# → status = Supported, edge `verifies`.
kaeru_refute(hypothesis="<name>", by="<counterexample-name>", initiative="X")
# → status = Refuted, edge `falsifies`.
```

## Review-flow

```
# Flag a node you doubt — non-destructive, attaches a contradicts edge:
kaeru_flag(name="<target>", reason="second look needed", initiative="X")

# Close an open question by recording the answer:
kaeru_resolve(question="<name>", by="<answer-name>", initiative="X")
```

## Evolve (graph metabolism)

```
# Promote operational draft → archival (preserves provenance):
kaeru_settle(source="<draft>", as_type="idea",
             name="<new>", body="…", initiative="X")

# Bring archival back to operational for revision:
kaeru_reopen(source="<archival>", as_type="draft",
             name="<new>", body="…", initiative="X")

# Many-to-one consolidation:
kaeru_synthesise(sources=["a","b","c"], as_type="summary",
                 name="combined", body="…", initiative="X")

# Rewrite a node's body (and/or rename):
kaeru_revise(name="<x>", body="<new body>", initiative="X")

# Bi-temporal forget — retracts node + edges, history preserved:
kaeru_forget(name="<x>", initiative="X")
```

## Time-travel

```
# What did this look like at a moment?
kaeru_at(name="<x>", when="5m",   initiative="X")   # 5 minutes ago
kaeru_at(name="<x>", when="2h",   initiative="X")   # 2 hours ago
kaeru_at(name="<x>", when="2026-05-06T12:00:00Z", initiative="X")

# Every assertion / retraction recorded for a node:
kaeru_history(name="<x>", initiative="X")
```

## Snapshot / share

```
kaeru_export(path="/tmp/kaeru-snap", initiative="X")
# Obsidian-friendly markdown vault (README + INDEX + LOG + pages).
```

Useful when the user wants to read offline, share a frozen view, or
when you want a flat-file overview without doing many tool calls.

## Conventions and gotchas

- **One initiative per project.** Mixing initiatives makes `awake`
  noisy. Prefer narrower scopes (`auth-rewrite`, not just `work`).
- **Names matter.** `recall` is exact-match. `search` is FTS but
  doesn't stem (`search query="token"` doesn't find "tokens"). Use
  the `*` suffix for inflection-tolerant matching.
- **`jot` vs `episode`.** Use `jot` for stream-of-consciousness; the
  auto-name handles uniqueness via id-suffix. Use `episode` only when
  you'll want to recall by exact name later.
- **Prefer `drill` over `recall` + a second call.** One round-trip.
- **Mutations are auto-tagged with the active initiative**, but reads
  are also scoped — searching under one initiative won't surface
  other initiatives' nodes.
- **`kaeru_config`** shows resolved vault_path and caps. Run if
  anything feels off.
- **Tool output is human-readable text** (not JSON). Parse robustly —
  look for patterns, not exact whitespace.

## When NOT to use

- Single-shot factual lookups that don't need persistence.
- Code that the user is editing — those changes already live in git;
  don't duplicate into kaeru.
- Anything truly ephemeral that won't be read across sessions.
