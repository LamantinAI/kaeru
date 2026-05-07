---
name: kaeru
user-invocable: true
description: Cognitive memory layer for LLM agents — typed graph + bi-temporal substrate + curator API. Use when the user wants to capture, recall, reason, or trace persistent thoughts across sessions; when re-entering a multi-session project; or when the user explicitly asks to "remember", "save", "note", "look up what I thought about X", "what's in project Y".
allowed-tools: Bash
---

# kaeru — agent memory CLI

`kaeru` is a typed-graph memory the user has spent time building.
Operational tier (cognitive / hippocampus) is fast working thought;
archival tier (recollection / cortex) is settled long-term knowledge.
Every node and edge is bi-temporal — assertions and retractions live
side-by-side, time-travel is native.

You interact through `kaeru-cli` subprocesses. Substrate location is
read from `KAERU_VAULT_PATH` (or the Linux default
`~/.local/share/kaeru`); platform defaults handle macOS / Windows.

**Cardinal rule (initiative):** every meaningful action must pass
`--initiative <name>`. Without it, mutations stay un-tagged and reads
are cross-initiative — almost never what you want. Use the repo /
project / topic name as the initiative when in doubt.

**Cardinal rule (language):** the vault is in the user's native
language. If they capture in Russian, store and search in Russian;
if Japanese, in Japanese; if English, in English. **Do NOT translate
content into English on capture and do NOT translate queries into
English on lookup.** Translation creates a drift between what was
stored and what you query for, and nothing matches. Every node carries
a `lang:ru` / `lang:en` / `lang:mixed` / `lang:other` tag at write
time that signals which language to expect.

**Search idiom (multilingual):** `search` is FTS without stemming —
exact tokens after lowercasing. Russian morphology, English
plurals, German declensions — none of them are folded. For
inflection-tolerant matching, append `*` to the term:
`search "утечк*"` finds `утечка` / `утечку` / `утечке`,
`search "token*"` finds `token` / `tokens` / `tokenize`,
`search "verlier*"` finds `verlieren` / `verloren` / `Verlierer`.
This works for any script; do not "translate to English to be safe".

## When to use

Auto-trigger when the user:
- Says **"remember"** / **"save this"** / **"note that"** / **"keep this in memory"**.
- Asks **"what did I think about X"** / **"what's in project Y"** / **"trace this back"**.
- (Re-)enters a project and you want continuity from previous sessions.
- Closes a thought ("decided", "settled", "this is the answer").
- Flags doubt ("wait — this looks wrong").

User-invocable via `/kaeru`.

## Re-entry ritual (do this first when picking up a project)

```bash
kaeru initiatives                              # see existing projects
kaeru --initiative <name> awake                # what was open last time
kaeru --initiative <name> overview             # what does this project know
```

`awake` answers "what was I doing" (process state — pinned, recent,
under-review). `overview` answers "what does this project know"
(epistemic state — categorical breakdown, provenance forests, open
questions). Run both.

## Capture (write thoughts)

```bash
# Quick fleeting thought — auto-named, low-significance:
kaeru --initiative X jot "noticed token expiry differs across platforms"

# Load-bearing observation / decision — pick a deliberate name:
kaeru --initiative X episode 'auth-decision' 'platform-aware expiry policy'

# Connect two named nodes:
kaeru --initiative X link from-name to-name --type causal
# Edge types: refers-to (default), causal, derived-from, contradicts,
# part-of, blocks, targets, supersedes, verifies, falsifies,
# temporal, consolidated-to.
```

## Inquire (read)

```bash
kaeru --initiative X recall <name>            # name → id (exact match)
kaeru --initiative X drill <name>             # name + 1-hop drill-down
kaeru --initiative X search "<query>"         # FTS across name+body
kaeru --initiative X search "<query>*"        # prefix-match (handles word forms)
kaeru --initiative X trace <name>             # walk derived_from ancestors
kaeru --initiative X recent --since 3h        # episodes in last 3h
kaeru --initiative X ideas                    # archival ideas
kaeru --initiative X outcomes                 # archival outcomes
kaeru --initiative X overview                 # full subgraph map
kaeru --initiative X tagged "<tag>"           # slice by tag — see below
```

`drill` is the most-used: replaces `recall <name>` + `summary <id>`
with one round-trip.

**Search results are sorted newest-first within equal scores**, so a
recent capture beats a stale one when both match. Stale information
naturally falls down the list; if the agent doesn't see what it
expects in the top 3 results, it should refine the query rather than
keep scrolling.

## Slicing by tag

Every captured node automatically gets these tags:
- `kind:<type>` — `kind:observation`, `kind:reference`, `kind:experiment`, `kind:idea`, …
- `sig:<level>` — `sig:low` / `sig:medium` / `sig:high` (significance, only for episodes that have it).
- `role:<role>` — `role:jot` / `role:review` / `role:synthesise` / `role:revised` (when applicable).
- `lang:<code>` — `lang:ru` / `lang:en` / `lang:mixed` / `lang:other` (auto-detected from body script).
- `topic:<word>` — up to 5 content tokens auto-derived from the body.
  E.g. `jot "обнаружил утечку токена"` adds `topic:обнаружил`,
  `topic:утечку`, `topic:токена`.
- `status:<state>` — only for hypotheses (`status:open`, `status:supported`, `status:refuted`, `status:inconclusive`).

Examples:
```bash
kaeru --initiative X tagged "kind:experiment"     # all experiments
kaeru --initiative X tagged "sig:high"            # high-significance only
kaeru --initiative X tagged "topic:auth"          # everything mentioning "auth"
kaeru --initiative X tagged "lang:ru"             # only Russian-language nodes
kaeru --initiative X tagged "status:open"         # open hypotheses
```

Topic tags use the **exact form from the body** — same as `search`,
no stemming. If you stored "утечку", topic tag is `topic:утечку`,
not `topic:утечка`. For loose matching use `search "<root>*"`
instead of `tagged`.

## Reason (hypothesis cycle)

```bash
kaeru --initiative X claim "weekend deploys cause flaky tests" --about <related-name>
# → creates hypothesis, optionally linked via refers-to.

kaeru --initiative X test <hypothesis> --method "compared 100 runs each"
# → creates experiment with `targets` edge.

kaeru --initiative X confirm <hypothesis> --by <evidence-name>
# → status = Supported, edge `verifies`.
kaeru --initiative X refute <hypothesis> --by <counterexample-name>
# → status = Refuted, edge `falsifies`.
```

## Review-flow

```bash
# Flag a node you doubt — non-destructive, attaches a contradicts edge:
kaeru --initiative X flag <target> --reason "second look needed"

# Close an open question by recording the answer:
kaeru --initiative X resolve <question> --by <answer-name>
```

## Evolve (graph metabolism)

```bash
# Promote operational draft → archival (preserves provenance):
kaeru --initiative X settle <draft> --as idea --name <new> --body "..."

# Bring archival back to operational for revision:
kaeru --initiative X reopen <archival> --as draft --name <new> --body "..."

# Many-to-one consolidation:
kaeru --initiative X synthesise --from a,b,c --as summary \
  --name combined --body "..."

# Rewrite a node's body (and/or rename):
kaeru --initiative X revise <name> --body "<new body>" [--rename <new-name>]

# Bi-temporal forget — retracts node + edges, history preserved:
kaeru --initiative X forget <name>
```

## Time-travel (the killer feature)

```bash
# What did this look like at a moment?
kaeru --initiative X at <name> --when 5m              # 5 minutes ago
kaeru --initiative X at <name> --when 2h              # 2 hours ago
kaeru --initiative X at <name> --when 1746549601      # unix seconds
kaeru --initiative X at <name> --when 2026-05-06T12:00:00Z

# Every assertion / retraction recorded for a node:
kaeru --initiative X history <name>
```

## Snapshot / share

```bash
# Obsidian-friendly markdown vault (README + INDEX + LOG + pages):
kaeru --initiative X export /tmp/kaeru-snap
```

Useful when the user wants to read offline, share a frozen view, or
when you want a flat-file overview without doing many CLI calls.

## Conventions and gotchas

- **One initiative per project.** Mixing initiatives makes `awake`
  noisy. Prefer narrower scopes (`auth-rewrite`, not just `work`).
- **Names matter.** `recall` is exact-match. `search` is FTS but
  doesn't stem (search "token" doesn't find "tokens"). When in doubt
  use `search "<word>"`.
- **`jot` vs `episode`.** Use `jot` for stream-of-consciousness; the
  auto-name handles uniqueness via id-suffix. Use `episode` only when
  you'll want to recall by exact name later.
- **Prefer `drill` over `recall + summary`.** One round-trip.
- **Mutations are auto-tagged with the active initiative**, but reads
  are also scoped — searching under one initiative won't surface
  other initiatives' nodes.
- **`config` is your friend** — `kaeru config` shows resolved
  vault_path and caps. Run if anything feels off.
- **All commands return human-readable text now** — JSON output
  is a future addition. Parse the human text robustly (look for
  patterns, not exact whitespace).

## When NOT to use

- Single-shot factual lookups that don't need persistence.
- Code that the user is editing — those changes already live in git;
  don't duplicate into kaeru.
- Anything truly ephemeral that won't be read across sessions.

## Help

`kaeru --help` shows the typical workflow + ENVIRONMENT vars.
`kaeru <command> --help` shows full per-command docs.
