---
name: kaeru
user-invocable: true
description: Cognitive memory layer for LLM agents — typed graph + bi-temporal substrate + curator API, reached through the kaeru MCP server. Use when the user wants to capture, recall, reason, or trace persistent thoughts across sessions; when re-entering a multi-session project; or when the user explicitly asks to "remember", "save", "note", "look up what I thought about X", "what's in project Y".
---

# kaeru — agent memory

`kaeru` is a typed-graph memory the user has spent time building.
Operational tier (cognitive / hippocampus) is fast working thought;
archival tier (recollection / cortex) is settled long-term knowledge.
Every node and edge is bi-temporal — assertions and retractions live
side-by-side, time-travel is native.

**There is no CLI.** kaeru is reached through its MCP server, as tools.
Everything below is written the way you actually call them —
`verb(param=value)` — not as shell commands. If you find yourself
reaching for a terminal to talk to kaeru, that is the wrong door.

The vault location comes from `KAERU_VAULT_PATH` (Linux default
`~/.local/share/kaeru`); platform defaults handle macOS / Windows. Call
`config()` to see what actually resolved.

**Cardinal rule (initiative):** pass `initiative` on every meaningful
call. Without it, mutations stay un-tagged and reads span every project
— almost never what you want. Use the repo / project / topic name when
in doubt.

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

## Memory of record (runtimes with built-in memory, e.g. Claude Code)

kaeru is meant to be the agent's **memory of record** — the one place
durable knowledge lives across sessions. Some runtimes ship their own
built-in memory that loads every session and competes with kaeru for
the agent's attention; **Claude Code** is the common case (an
auto-loaded `MEMORY.md` plus a file store under
`~/.claude/projects/<project>/memory/`). Left alone, the agent drifts
back to the built-in store and knowledge forks across two systems.

This is an integration gap, not a kaeru bug — a built-in store baked
into the runtime's system prompt can't be out-competed by an MCP
server's instructions alone. Close it from the **config** side, where
the user has the lever. In line with kaeru's facilitator stance these
are setup steps the user opts into, not anything kaeru enforces.

**1. Make the built-in store redirect to kaeru.** Rewrite the
auto-loaded memory file (Claude Code: `MEMORY.md`) from a neutral index
into a short directive: *source of truth is kaeru; on session start run
`initiatives` → `awake` → `overview`; write new facts to kaeru
(`jot`/`episode`/`cite`/`claim`/`task`), not to the file store.* The
same file that pulled the agent toward local notes now pushes it toward
kaeru, every session.

**2. Migrate existing notes, leave pointers.** For each note already in
the built-in store, `cite` it into kaeru (persona/project facts are
exactly what archival `cite` is for — `url` is optional), then reduce
the original file to a one-line pointer at the kaeru node so nothing is
lost and nothing is dual-maintained.

**3. Reinforce on session start (optional).** If the runtime supports
startup hooks, add one that prints a one-line reminder to consult kaeru
first. Claude Code example — a `SessionStart` hook in
`~/.claude/settings.json`:

```json
{
  "hooks": {
    "SessionStart": [
      {
        "matcher": "startup|resume|clear",
        "hooks": [
          {
            "type": "command",
            "command": "printf '%s\\n' 'MEMORY: source of truth is kaeru (MCP). On start call initiatives, then awake and overview. Write new facts/tasks to kaeru (jot/episode/cite/claim/task), not the local file store.'"
          }
        ]
      }
    ]
  }
}
```

A freshly-created `settings.json` may not be picked up until the
runtime reloads its hook config (in Claude Code: open `/hooks` once, or
restart). Keep the capture language native (see the cardinal rule
above) — write the reminder in the user's language.

## Personas — same primitives, different uses

The verb taxonomy looks research-flavoured (`claim` / `evidence` /
`confirm` / `synthesise`) but the underlying primitives are general.
Three example personas of how the same kaeru maps to different daily
workflows:

### Researcher / engineer

The "default" use case. Captures observations as `jot` /
`episode`, formalizes hunches via `claim`, records what was checked with `evidence` +
`confirm` / `refute`, settles findings with `synthesise` →
`settle` (operational draft → archival outcome). External sources
go through `cite(name=…, url=…)`. Initiative is the project name.

### Personal manager / assistant

The agent helps with daily life — todos, people, plans, journal.

- **Tasks** with deadlines: `task(body="купить молоко", due="2d")`,
  `task(body="позвонить маме", due="weekend")`. Mark complete with
  `done <task-name>`.
- **People / places / things** without URLs: `cite "Анна"
  body="врач семейной клиники, рекомендация Маши")` — same `cite`
  verb, `url` optional. Persona records live in the archival tier
  ("cortex"), so things like "who is my user" stick around forever.
- **Plans / intentions / decisions**: just `jot` (`role:jot`,
  `kind:observation`). Slice later with `tagged "topic:план"` etc.
- **Daily journal**: `jot` whatever's on the agent / user's mind;
  `recent(since="24h")` for "what happened today", `recent(since="7d")`
  for the week.

Initiative for personal use is typically a single name like
`personal` or `daily`, or split by life area (`work`, `home`,
`learning`).

### Long-term cortex (cross-initiative facts)

Things that should outlive any specific project — `who is my user`,
`my preferences`, `repeated correspondents`, persistent locations.
Capture as `cite(name="<name>", body="…")` (no URL) **without an
`initiative`**, or under a stable one like `cortex`. The
archival tier means these aren't surfaced by a project's `awake` /
`overview` and aren't crowded out by recent thoughts; they're
retrievable on demand via `drill <name>` / `tagged "kind:reference"`.

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
initiatives()                          # which projects exist
awake(initiative="<name>")             # what was open last time
overview(initiative="<name>")          # what this project knows
```

`awake` answers "what was I doing" (process state — pinned, recent,
under-review). `overview` answers "what does this project know"
(epistemic state — categorical breakdown, provenance forests, open
questions). Run both.

## Exit ritual (do this when a piece of work ends)

There is an entry ritual and, for a long time, nothing symmetric at the other
end. A session does not finish, it stops — so the moment "this is done, tidy
up" belongs to never arrives on its own, and the instruction to promote
knowledge *when it stops changing* points at something only visible between
sessions.

The fix is not discipline, it is picking a moment that actually occurs. Three
do:

- **A task closes** — `done <name>`
- **A review closes** — `close_review <target>`
- **A claim gets its verdict** — `confirm` / `refute` / `inconclusive`

All three now answer with what still converges on the thing that just closed,
and ask what the work concluded. When they do, spend one more call:

```
settle(source="<name>", initiative="X")                    # one node hardened in place
synthesise(from=["a","b","c"], new_name="<x>",
           new_body="…", initiative="X")                   # several → one outcome
```

And before you stop for the day, or when the conversation is about to be
compacted:

```
reflect(initiative="X")                # the work-list, with how to act on each part
```

`reflect` names overdue tasks, open reviews, claims whose text already answers
them, stale chains, and settled work still sitting in the operational tier. It
is a read — nothing changes until you act on it.

**`reflect` is yours to call**, by design — it reads and proposes, it never
acts, so the judgement about what to settle and what to leave stays with the
agent that has the context. Two moments deserve it: when a piece of work ends,
and before a session stops.

The one thing that does run by itself is layer tidying (`hygiene`), because
moving a node between recall bands is reversible and needs no judgement.
Anything that changes what the graph *says* stays a decision.

## Cadence — habits that keep the graph useful

These are the moves that turn kaeru from "saved markdown with
frontmatter" into an actual graph the agent can reason over. Each is
small per occurrence; together they're the difference between "I cited
something once" and "next session can find it via three different paths".

- **Capture the user's ask as a `task`.** When the user says
  "build X and report back" or "fix Y by tomorrow", that's literally
  what `task` was designed for: `task(body="…", due="1h")`, then `done`
  when finished. The task node is what survives into next session as
  "what was being worked on" — `board` shows it in its column, and
  `set_status` moves it as work progresses. (`awake` does *not* list
  tasks; it carries the pinned set, recent nodes and open reviews.) Findings
  you derive *while* doing the task go into separate
  `cite` / `episode` / `claim` nodes; the task is the operational arc
  connecting them. Single-shot factual lookups don't need a task.

- **Cite, then link.** When you `cite` a new node that's conceptually
  adjacent to one you saw earlier in this session (via `search` /
  `drill`), `link` them — `edge_type="causal"` if one causes the other,
  `"derived_from"` if one is a refinement, `"refers_to"`
  for a plain "see also". Edges are how recall walks the graph; without
  them every cite is an island and only exact-name lookups will find
  it. Costs one call per edge. Pays off every time someone
  navigates in.

- **Know the three read depths.** `recall <name>` returns just the id;
  `drill <name>` gives a short body **excerpt** plus 1-hop neighbors; and
  `at <name>` reads the node **in full** — the whole untruncated body and
  every field (type, tier, layer, visibility, tags). `drill` / `search` /
  `recall` all truncate the body, so when you actually need a node's
  complete content, reach for `at`. (Add `when="5m"` / `"2h"` / a date to
  see how it looked at a past moment.) Don't re-`search` words you just
  `recalled` — it queries a different index for the same answer.

- **Refine, don't stampede.** If `search "X"` doesn't surface what you
  want in the top 3 hits, the next call should be a *different shape* —
  `search "X*"` for inflection, `tagged "topic:X"` for exact-token
  slice, or `drill <related-name>` to walk in. Five variant phrasings
  in 20 seconds is almost always slower than reading the first three
  results carefully and then making one targeted call.

- **Re-`awake` after long gaps.** If your last `awake` / `recent` was
  more than ~30 minutes ago and there's any chance another agent or
  another teammate's session has written to the same vault, run them
  again before assuming your view is current. The vault is shared at
  the daemon level; sibling writes only become visible on read.

## Capture (write thoughts)

Match the verb to the **epistemic status** of the content, not its
length. This is the single most consequential choice on the write side:
a note captured as the wrong kind is findable but not usable.

```
# Fleeting thought — auto-named, low significance:
jot(body="noticed token expiry differs across platforms", initiative="X")

# Load-bearing observation or decision — a name you will recall by:
episode(name="auth-decision", body="platform-aware expiry policy", initiative="X")

# Todo with a deadline (auto-named; kind:task, status:open):
task(body="купить молоко", due="2d", initiative="X")
task(body="созвон с командой", due="2026-05-15", initiative="X")
done(name_or_id="<task-name>", initiative="X")

# Settled document kept verbatim — ADRs, specs, glossaries, persona
# records. `url` is OPTIONAL: this is for "my own settled doc" as much
# as for an external source. Goes straight to the archival tier.
cite(name="transformer-paper", url="https://…", body="…", initiative="X")
cite(name="Анна", body="врач, рекомендация Маши", initiative="X")

# A falsifiable claim. If you ALREADY know how it turned out — the
# usual case, since you reach memory after the check has run — say so
# in the same call and it lands settled:
claim(text="the cache pays for itself", verdict="refuted",
      by="<evidence-node>", initiative="X")
```

When using `url`, point at the canonical artifact — the actual PDF, the
release-asset download, the dashboard panel — not an API endpoint or a
metadata URL. A later `drill` hands that URL to the next agent, which
wants to fetch, not to introspect.

**Then connect it.** A node nobody linked is findable only by its exact
name, which means it is findable only by someone who already knows it
exists.

```
link(from="from-name", to="to-name", edge_type="causal", initiative="X")

# Weight is YOUR judgement of how load-bearing the connection is
# (0.0–1.0, default 0.5) — not a semantic score. It steers knowledge
# chains: a chain threads through strong edges.
link(from="a", to="b", strong=true, initiative="X")     # = 1.0
link(from="a", to="b", weight=0.3, initiative="X")      # tentative
```

Edge types are a closed vocabulary: `refers_to` (default), `causal`,
`derived_from`, `contradicts`, `part_of`, `blocks`, `targets`,
`supersedes`, `verifies`, `falsifies`, `temporal`, `consolidated_to`.

## Inquire (read)

```
recall(name="<name>", initiative="X")          # exact name → id, nothing else
drill(name="<name>", initiative="X")           # the node + one hop of neighbours
at(name="<name>", initiative="X")              # the node IN FULL — whole body, all fields
at(name="<name>", when="2h", initiative="X")   # …as it stood two hours ago
search(query="<query>", initiative="X")        # full-text over name + body
search(query="<query>*", initiative="X")       # prefix — handles word forms
trace(name="<name>", initiative="X")           # walk derived_from to the sources
why(name_or_id="<name>", initiative="X")       # the saved reasoning trail
recent(since="3h", initiative="X")             # episodes in the last 3h
ideas(initiative="X")                          # archival ideas
outcomes(initiative="X")                       # archival outcomes — what the work concluded
overview(initiative="X")                       # the subgraph map
tagged(tag="<tag>", initiative="X")            # slice by tag — see below
board(initiative="X")                          # tasks by column
```

**`drill` and `search` return excerpts; `at` returns the text.** If a
body matters, `at` it — the excerpt is for deciding whether to.

`drill` is the most-used: replaces `recall <name>` + `summary <id>`
with one round-trip.

**Search results are sorted newest-first within equal scores**, so a
recent capture beats a stale one when both match. Stale information
naturally falls down the list; if the agent doesn't see what it
expects in the top 3 results, it should **change the shape of the
query** — `search "X*"` for inflection, `tagged "topic:X"` for exact
token, `drill <related>` to walk neighbors — not re-phrase the same
intent five times.

## Slicing by tag

Every captured node automatically gets these tags:
- `kind:<type>` — `kind:observation`, `kind:reference`, `kind:experiment`, `kind:idea`, …
- `sig:<level>` — `sig:low` / `sig:medium` / `sig:high` (significance, only for episodes that have it).
- `role:<role>` — `role:jot` / `role:review` / `role:synthesise` / `role:revised` (when applicable).
- `lang:<code>` — `lang:ru` / `lang:en` / `lang:mixed` / `lang:other` (auto-detected from body script).
- `topic:<word>` — up to 5 auto-derived tokens: the node's **most-mentioned**
  words, not its first ones. A name you chose counts triple, so
  `episode "figma-export-broken" "the pipeline drops layer names"` tags
  `topic:figma` and `topic:export` ahead of anything in the prose. A compound
  is tagged by its parts too — `figma-макет` also yields `topic:figma` and
  `topic:макет`, so the word inside it stays reachable.
- `status:<state>` — hypotheses (`status:open`, `status:supported`, `status:refuted`, `status:inconclusive`) and tasks (`status:open`, `status:done`, or the initiative's own board vocabulary).

Examples:
```
tagged(tag="kind:experiment", initiative="X")   # all experiments
tagged(tag="sig:high", initiative="X")          # high-significance only
tagged(tag="topic:auth", initiative="X")        # everything about auth
tagged(tag="lang:ru", initiative="X")           # Russian-language nodes
tagged(tag="status:open", initiative="X")       # claims still awaiting a verdict
```

Topic tags use the **exact form from the body** — same as `search`,
no stemming. If you stored "утечку", topic tag is `topic:утечку`,
not `topic:утечка`. For loose matching use `search "<root>*"`
instead of `tagged`.

A miss is not a dead end: `tagged` answers an unmatched tag with the near
tags that actually exist in scope (and their counts), so the empty answer
tells you what to ask for instead.

## Reason (hypothesis cycle)

```
# You normally reach memory AFTER the check has run, so record the answer
# with the claim — one call, and the status lands on the tag where every
# read surface can see it:
claim(text="weekend deploys cause flaky tests",
      verdict="refuted", by="<evidence-name>", initiative="X")

# Genuinely open question — no verdict yet. Keeps surfacing in `awake`:
claim(text="weekend deploys cause flaky tests", about="<related-name>", initiative="X")

evidence(hypothesis="<name>", method="compared 100 runs each", initiative="X")
# → writes the result up as an experiment node with a `targets` edge.
evidence(hypothesis="<name>", node="<existing-episode>", initiative="X")
# → registers something you already captured instead.

confirm(hypothesis="<name>", by="<evidence-name>", initiative="X")
# → status = Supported, edge `verifies` when `by` is given.
refute(hypothesis="<name>", by="<counterexample-name>", initiative="X")
# → status = Refuted, edge `falsifies`.
inconclusive(hypothesis="<name>", initiative="X")
# → the check ran and did not decide. A verdict, not a missing answer.
```

## Knowledge chains (strongest reasoning trail between two nodes)

A chain is the **strongest weighted path** from one node to another —
Dijkstra over `link` weights, where a strong edge is a short hop. Use it
when two ideas are connected through several intermediate steps and you
want the whole trail, not an isolated endpoint.

```
# Preview the path without saving anything:
path(from="from-name", to="to-name", initiative="X")   # preview, writes nothing

# Materialize that path as a first-class `chain` node:
chain(from="from-name", to="to-name", name="auth-trail",
      summary="why this line of work went the way it did", initiative="X")

# `why` takes either, and dispatches:
why(name_or_id="<chain-name>", initiative="X")   # → the chain's ordered steps
why(name_or_id="<node>", initiative="X")         # → its chain, read directly when
                                                 #   there is one; a menu when several
```

Weights are what make this useful: `link(strong=true)` on the edges that
genuinely carry reasoning, leave incidental links at the default, and
`path`/`chain` will thread the load-bearing route rather than the
shortest hop-count. Chains are initiative-scoped and `local`.

## Review-flow

```
# Flag a node you doubt — non-destructive, attaches a contradicts edge:
flag(target="<name>", reason="second look needed", initiative="X")

# Close an open question by recording the answer:
resolve(question="<name>", by="<answer-name>", initiative="X")
```

## Evolve (graph metabolism)

```
# Promote a node that stopped changing → archival (provenance survives).
# The name ALONE is enough: name, body and manual tags carry over, and
# the type is derived (episode/task/experiment/hypothesis → outcome,
# draft/scratch → idea). Everything it defaulted is printed back.
settle(source="<name>", initiative="X")
settle(source="<name>", new_type="idea", new_name="<new>", initiative="X")

# Back to operational when settled knowledge turns out to still be in
# flight — same in-place defaults, no type heuristic:
unsettle(source="<name>", initiative="X")

# Many-to-one: several seeds converge into one durable node, each
# joined by derived_from so `trace` walks back to them.
synthesise(from=["a","b","c"], new_type="summary",
           new_name="combined", new_body="…", initiative="X")

# Rewrite a node's body (and/or rename):
revise(name="<name>", body="<new body>", rename="<new-name>", initiative="X")

# Bi-temporal forget — retracts node + edges, history preserved:
forget(name_or_id="<name>", initiative="X")
```

## Time-travel (the killer feature)

```
# What did this look like at a moment?
at(name="<name>", when="5m", initiative="X")                    # 5 minutes ago
at(name="<name>", when="2h", initiative="X")                    # 2 hours ago
at(name="<name>", when="1746549601", initiative="X")            # unix seconds
at(name="<name>", when="2026-05-06T12:00:00Z", initiative="X")

# Every assertion / retraction recorded for a node:
history(name="<name>", initiative="X")
```

## Snapshot / share

```
# Obsidian-friendly markdown vault (README + INDEX + LOG + pages):
export(path="/tmp/kaeru-snap", initiative="X")
```

Useful when the user wants to read offline, share a frozen view, or
when you want a flat-file overview without walking the graph call by call.

## Local vs cloud (team sharing)

Two tiers of a different kind: your **local** vault, which is the
default and where everything starts, and an optional **team cloud**.
Nothing syncs in the background — every crossing is a call you make.

**The cloud is not visible to a local read.** `awake`, `search` and
`overview` answer from the vault on this machine, so on a shared
initiative a complete-looking answer can be quietly missing whatever
the team put in the cloud. `awake` says so when the initiative permits
sharing; believe it.

```
clouds()                                        # which clouds this daemon can reach
cloud_initiatives(cloud="team")                 # what one of them holds
cloud_recall(initiative="X", query="proxy",
             cloud="team")                      # SEARCH the team's tier
pull(id="<uuid>", initiative="X", cloud="team") # bring one node local
```

`cloud_recall` is the cloud's `search`. It is paged (25 by default) and
tells you the true total and the exact call for the next page.

**Writing.** Local by default — personal, exploratory and half-formed
thoughts stay on the machine. Share what is settled and useful to
someone else:

```
policy(initiative="X", policy="team", cloud="team")   # once per initiative
share(name="<node>", initiative="X", cloud="team")
unshare(name="<node>", initiative="X", cloud="team")  # withdraw a mistake
```

`policy` asks two questions and both must pass: **whether** an
initiative may leave (`private` / `team` / `ask`) and **where to** (the
`clouds` list; an initiative with no list may go to any configured
cloud). A capture can share in one call with `visibility="shared"`,
which also takes `cloud`.

**Correcting and withdrawing.** A share is not permanent. Re-sharing a
corrected node updates the cloud copy in place — the push is an upsert
under the same id, so `revise` then `share` again. `unshare` retracts
it: the node leaves `cloud_recall` and the listings while its history
survives, which is the same "mark, don't delete" model the local graph
uses.

**Naming the cloud.** With one cloud configured you can omit `cloud`
entirely. With **several**, name it — an unnamed call is refused rather
than routed to a default, because a write that reaches the wrong cloud
cannot be undone there. The refusal lists your choices.

**Fail-safe by design.** Default visibility is `local`; `share` runs
both gates — the initiative's policy and a secret scanner — so a wrong
call errors safe. Worst case you fail to pull something; secrets and
personal notes do not leak by accident.

**Soft links, when a copy would be wrong.** `pull` copies a node into
your vault, and the copy goes stale silently when its owner revises it.
`link_cloud` points at it instead, and `cloud_links` resolves the
pointer live. Pull when you need the content; link when you need the
citation.

## Conventions and gotchas

- **One initiative per project.** Mixing initiatives makes `awake`
  noisy. Prefer narrower scopes (`auth-rewrite`, not just `work`).
- **Names matter.** `recall` is exact-match, nothing else. `search` is
  full-text but does not stem — `"token"` does not find `"tokens"`, so
  reach for `search "token*"`. A name that fails to resolve tells you
  which of three things happened: it is in another initiative (named),
  something close exists (listed), or it is nowhere.
- **`jot` vs `episode`.** Use `jot` for stream-of-consciousness; the
  auto-name handles uniqueness via id-suffix. Use `episode` only when
  you'll want to recall by exact name later.
- **Prefer `drill` over `recall` then a read.** One round-trip, and it
  says when the node sits in a saved trail.
- **Mutations are auto-tagged with the active initiative**, but reads
  are also scoped — searching under one initiative won't surface
  other initiatives' nodes.
- **`config()` is your friend** — resolved vault path, the configured
  clouds and which is default, and every cap. Run it when anything
  feels off.
- **Settle in place.** `settle(source=…)` needs nothing else: the node
  keeps its name, its full body and its manual tags, and only the tier
  moves. Do not demote finished work to a `cold` layer instead — a
  layer is how eagerly a node loads, a tier is whether it is still in
  flight.
- **Record the verdict with the claim.** You almost always reach memory
  *after* the check has run, so `claim(text=…, verdict=…, by=…)` in one
  call beats an open claim whose body says "REFUTED" in prose that
  nothing can query.
- **Every verb answers in human-readable text**, not JSON. Read it for
  meaning rather than parsing exact whitespace — and read the `↳` lines:
  they are where a result tells you the verb that goes further.

## When NOT to use

- Single-shot factual lookups that don't need persistence.
- Code that the user is editing — those changes already live in git;
  don't duplicate into kaeru.
- Anything truly ephemeral that won't be read across sessions.

## Help

There is no `--help`: the tool descriptions in the MCP server are the
reference, and they are not truncated the way the server instructions
are. `config()` shows the resolved vault path, the configured clouds
and every cap. When something feels off, start there.
