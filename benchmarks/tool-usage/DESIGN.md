# Tool-usage suite — design

> Sibling to `../agent-memory/`, measuring the other axis. That suite asks
> **can memory produce the right answer** when the facts are messy. This one asks
> **does a working agent reach for memory at all, and does it get back the current
> answer** — in situations where a real user had to stop the agent and say so.

## Why this suite exists

Every case here is taken from a session that actually happened, and every one of
them cost something: an interrupted response, a repeated instruction, work redone.
They come from two passes over one vault's full history (13,110 session files,
June–September 2026): an inventory of tool calls, and a classification of all 136
unique interrupted responses by what the user had to say afterwards.

That second pass is what shaped this suite. The failures cluster into three
mechanisms, and the biggest is not the one anybody predicted:

| mechanism | share of memory-related interruptions | what it looks like |
|---|---|---|
| **asked and got the wrong answer** | 8 cases, the costliest | agent had made 15–143 memory calls before the user stepped in |
| **rule stated in conversation, never captured** | 12 cases, the most frequent | "don't upload until it's fixed", "pull the branches before opening PRs" |
| **never asked memory at all** | 8 cases, 5 with zero calls | "use kaeru, friend", "did you even use kaeru?" |

The first mechanism is why this suite cannot be a retrieval benchmark. In the worst
case the agent made **143** memory calls and still used a superseded method, because
the current one was unreachable from the initiative it was working in. Counting
calls would have scored that session as exemplary.

## The measurement problem, and how this suite solves it

A benchmark that prescribes a verb sequence measures obedience, not memory: if two
reasonable storage strategies exist and the agent picks the second, that is not a
failure. But "just grade the final answer" throws away the thing we actually need
to know — whether the agent *used its instrument*.

The resolution is in how an expectation is written:

- **Never**: "the agent must call `claim --verdict refuted`".
- **Instead**: "after this session, memory must hold a hypothesis whose status is
  refuted, attached to its evidence" — reachable by any of three verb paths, all
  scored equal.
- **Failure** is a state, not a missing call: a bare `episode` whose body says
  "refuted" leaves the next session unable to tell a closed question from an open
  one. That is what actually happened in the logs, repeatedly.

So every expectation in this suite is a property of **the memory left behind and of
what the next session can recover from it**. The tool-call trace is recorded, but as
diagnosis ("which path did it take"), never as the grade.

## Case anatomy

Each case is a directory:

```
cases/<case-id>/
├── case.yaml       # situation, prompts, expectations, trap, baseline
└── fixture.yaml    # the memory state the agent wakes up to
```

**`fixture.yaml`** declares nodes, edges, layers, tiers, tasks and chains to
create before the run — including, deliberately, the stale and competing material.
A case whose fixture is clean tests nothing that fails in practice.

**`case.yaml`** carries:

- `prompt` — an ordinary work request, phrased as the user actually phrases them.
- `mode` — `explicit` (the prompt mentions memory) or **`silent`** (it does not).
- `expect.memory_after` — what must be recoverable, with `any_of` paths where
  several storage strategies are legitimate.
- `expect.behaviour` — what the answer or the work must show.
- `fail_if` — the specific wrong outcome observed in the field.
- `trap` — the deliberate hazard, named.
- `baseline` — what the agent actually did when this happened, with the date and
  the user's own words. This is the part no invented case can have.

### `silent` mode is half the suite

The failures that hurt most were not questions about memory. The user asked for
ordinary work; the agent, not knowing what memory held, did it wrong. So half the
cases never mention memory in the prompt — the fixture contains a rule or a current
method, and the only way to succeed is to consult it unprompted. A suite where every
prompt says "look in memory" measures retrieval under laboratory conditions and
would have missed every one of the eight most expensive failures in the corpus.

## Scoring

Per case, three independent outcomes:

1. **Outcome** — did `memory_after` hold and `fail_if` stay clear? Pass/fail.
2. **Currency** — of the facts the agent used, how many were the current version
   rather than a superseded one? A confident stale answer scores **below** silence:
   in the field it is what produced the interruptions.
3. **Cost** — calls made before the answer was reached; and for `silent` cases,
   whether memory was consulted before the work started or only after being told.

Agents are stochastic, so a case is run N times and scored as a rate, not a verdict.
Fixed model and fixed fixture keep everything else deterministic; the fixture build
is pure I/O and reproducible byte-for-byte.

**What is deliberately not a metric:** the number of distinct verbs used. Under the
enrichment model in #79, knowledge delivered inside another answer is a *better*
outcome than a call — zero calls with the right result is a success, and a suite
that rewards verb variety would push the product the wrong way.

## Fixture requirements — learned from the first pilot

Two cases (A1, A2) were run against live fixtures before the rest were written.
Both "passed", and the pilot's value was in showing that one of those passes was
meaningless. Three rules follow, and they bind every case built from here on.

### 1. A case must be unpassable without the mechanism it tests

A2 was supposed to prove that an unreachable rule (#81) changes behaviour. The
agent did the right thing — it refused to publish — but the trace shows it never
found the rule and never looked for one. Three other paths led to the same
outcome: a sibling node in the fixture spelled out the constraint ("three reviewer
comments are still open"), a second one warned that re-uploading loses comment
anchors, and the agent's own standing policy is to confirm before publishing
anywhere. It said so itself: *"the note in memory describes the procedure but is
not permission to upload."*

A case whose expected behaviour has other sufficient causes measures nothing. So:

- **the rule must be arbitrary, not inferable** — "never re-upload whole, always
  patch blocks" is checkable and unguessable; "don't publish with open comments"
  is common sense and will be honoured for free;
- **no other fixture node may restate it**, in whole or in part;
- **the action must be routine**, not one that trips built-in caution. Publishing,
  deleting and spending money are all confirmed by default, so they cannot be
  used to detect whether memory was consulted.

Before a case is accepted, write down every path to the expected behaviour. If any
path avoids the mechanism, the case is not yet a test.

### 2. A fixture must reproduce scale, not only structure

A1 recreated the field failure faithfully in shape — a current method, two live
predecessors, supersession stated only in prose — and the agent got it right on the
first search, immediately naming the supersession it read in the text. In the field
the same agent had made 62 calls and still used the stale method.

The difference is size. Four nodes, with a `refers_to` edge pointing straight at the
current method and nothing else competing, is not the graph where this fails. The
failure needs a working set where the right node does not surface first: dozens of
nodes, several plausible hits per query, and the current one **not** the newest.

Fixtures therefore need a `noise` section — bulk nodes that share vocabulary with
the target — and cases like A1 need the target buried rather than adjacent. A case
that passes on a four-node fixture has proved nothing about a four-hundred-node one.

### 3. Failure criteria are deviations, not lists of wrong answers

A2's rewritten run produced neither the correct format (TSV) nor the one the case
warned against (CSV): it wrote a Markdown table, named by ISO week, with a page of
reasoning for why that naming was best. Two of three `fail_if` clauses named
specific wrong choices and missed it entirely.

Plausible-and-wrong is the *normal* shape of this failure, not an edge case — in
the field the agent's stale screenshot method was equally well argued. So failure
must be written as **deviation from the convention** ("the file is not TSV", "the
name does not match the pattern"), never as an enumeration of anticipated mistakes.
The corollary for grading: an answer's confidence and reasoning quality say nothing
about whether the convention was consulted, and must not be read as evidence that
it was.

### 4. Runs must be isolated

The A1 agent wrote its own node into the fixture and linked it — correct behaviour,
and it left the fixture different from how it started. Any second run would begin
from a mutated state. A runner must therefore build the fixture into a scratch vault
per run and discard it afterwards; never against a working vault. During this pilot
the cleanup was manual, which does not scale past a handful of runs.

## Coverage

Twenty-seven situations, in `CATALOG.md`, each with the field evidence behind it.
They are grouped by mechanism, and the proportions follow the corpus rather than a
feature list — so, for instance, retrospective capture (the verdict is already known
when memory is reached) outweighs the prospective claim→test→verdict cycle, because
that is the ratio observed in real sessions: 13 of 23 claims arrived with the answer
already in hand.

## Relationship to the golden dataset (#80)

`#80` builds cases as **Layer → Fill → Task**: give the agent material, let it store,
then ask something answerable only from memory. That format measures storage and
recall quality well, and this suite does not duplicate it.

It cannot, however, express the two largest mechanisms above: in it, knowledge is
always deliberately filed and then explicitly asked for. "Asked and got a stale
answer" needs a fixture with a live predecessor; "rule never captured" needs a prompt
where nothing tells the agent that a rule was even stated. Both are what `silent`
mode and hand-built fixtures are for. The suites are complementary and should share
a fixture format.
