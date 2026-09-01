# tool-usage — does the agent reach for memory, and get the current answer?

A regression instrument for the curator API. Sibling to `../agent-memory/`:
that suite asks whether memory can produce the right answer; this one asks
whether a working agent uses memory at all, and whether what comes back is the
version in force.

- **[DESIGN.md](DESIGN.md)** — method, why expectations are states rather than
  verb sequences, scoring, and how this differs from the golden dataset (#80).
- **[CATALOG.md](CATALOG.md)** — 27 situations, grouped by failure mechanism,
  each with the field evidence behind it.
- **[cases/](cases/)** — worked cases: a fixture, a prompt, and expectations.

## Status

**All 27 catalogued situations are built.** The suite is a dataset; no runner
yet — see DESIGN.md for what a runner must compute.

| group | cases | what the group isolates |
|---|---|---|
| **A · asked, got the wrong answer** | A1 A2 A3 A4 A5 A6 A7 | the costliest mechanism: memory was consulted and returned the wrong thing — stale versions, unreachable nodes, sibling initiatives, misremembered names, answers living in artifacts |
| **B · rule stated, never captured** | B1 B2 B3 B4 B5 B6 | the most frequent mechanism: a constraint spoken once, holding for weeks, that nothing writes down |
| **C · memory never consulted** | C1 C2 C3 C4 | zero-call failures: cold re-entry, assumed topology, reopened decisions, unrecorded plans |
| **D · capture dispatch** | D1 D2 D3 D4 D5 | the verb matched to epistemic status, so the next session can tell settled from open |
| **E · debts and lifecycle** | E1 E2 E3 E4 E5 | overdue work, unclosed tasks, unmarked contradictions, cold knowledge, the team tier |

**Modes.** 13 single-turn `silent`, 6 `explicit`, 8 multi-turn (a rule is stated
in one session and must survive into the next). Every case carries a `baseline`
recording what actually happened in the field, and every fixture pulls a noise
bank so the target does not sit on the surface.

**Regression guards.** Individual cases are pinned to known defects and fixes, so
a change that breaks one names itself: A2 → #81 (fails while open, by design),
D2 → the `inconclusive` verdict added in v0.7.0, D5 → #71 chain membership across
promotion, C1 and E1 → the awake read-back sections, A5 → structural vs prose
supersession (paired with variant A5b to measure what the edges buy).

**Piloted so far:** A1 and A2 only, on 2026-09-01. The remaining 25 are written to
the post-pilot template but have not been run. Expect some to need the same
treatment A2 did — the pilot's first lesson was that a case can pass without
touching its mechanism, and only a run reveals it.

## Where the cases come from

Two audits of one vault's complete history — 13,110 session files, June to
September 2026. The first inventoried every `mcp__kaeru__*` call and every
error. The second classified all 136 unique interrupted responses by what the
user said after stopping the agent.

That classification is why the suite is shaped the way it is. The most
expensive failure was not "the agent never asked memory" — in the worst case it
had made **143** memory calls before the user intervened, and still acted on a
superseded method. Any metric built on call counts would have scored that
session as exemplary. So the cases measure what was left in memory and whether
the answer used was current, never which verbs were called.

Eighteen of the twenty-seven situations are `silent`: the prompt is an ordinary
work request that never mentions memory, because that is how the real failures
arrived.
