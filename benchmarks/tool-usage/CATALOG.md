# Situation catalogue

Twenty-seven situations, each observed in real sessions. `mode` is `silent` when
the prompt does not mention memory — the agent must consult it unprompted.

Field evidence is quoted from the user's own words at the moment they had to stop
the agent. Identifying details are removed; the structure is unchanged.

---

## A. Asked, and got the wrong answer

*The costliest mechanism. In every case below the agent had already made memory
calls — 15 to 143 of them — before the user intervened. Counting calls scores these
sessions as exemplary; they were failures.*

| id | situation | must be recoverable after | mode | field evidence |
|---|---|---|---|---|
| `A1-stale-method` | Two methods for the same task exist; the newer one says in prose that it replaces the older. Agent must use the current one. | the current method, and no use of the superseded one | silent | 62 calls, then: *"you seem to have forgotten how we take screenshots now — look in kaeru, we have a **current** method"* |
| `A2-unreachable-core` | A rule was stored with `layer=core` but no initiative, so it loads in no session (#81). Agent works inside an initiative. | the rule applied, or an explicit report that memory holds nothing on it | silent | 85 calls and two `awake`s, then: *"you forgot **again**… put it in core already, you're driving me mad"* |
| `A3-fragmented-project` | One project's knowledge is spread across several initiatives; the needed fact is in a sibling of the one being worked in. | the fact found across the boundary, not recreated | silent | *"look in kaeru, in some initiative"* — user had to name the search space |
| `A4-name-misremembered` | The agent recalls a node name imprecisely; exact lookup fails. | the node found by search rather than abandoned or duplicated | silent | 26 name-misses in four days; agents retry guesses instead of searching |
| `A5-current-vs-history` | Several dated versions of a procedure exist; the question is about the one in force now. | the newest version, with the older ones not presented as current | explicit | *"you can look in kaeru for the **current** information on how we uploaded"* |
| `A6-answer-in-own-artifact` | The answer is in a document the project already produced, not in a search index. | the answer taken from the existing artifact | silent | *"why are you searching — the flows are described in our own lesson"* |
| `A7-duplicate-before-write` | About to store something already stored under a different name. | one node, not two; existing one found first | silent | duplicate hub nodes observed in the live graph |

## B. A rule was stated in conversation and never captured

*The most frequent mechanism — 12 interruptions. A constraint is spoken once, holds
for weeks, and nothing writes it down. The next session breaks it again.*

| id | situation | must be recoverable after | mode | field evidence |
|---|---|---|---|---|
| `B1-constraint-spoken` | User states a hard constraint mid-work ("never set X, only Y"). Session ends. | the constraint stored as a rule, scoped so it loads in this project | explicit | *"we absolutely must not set medium, only low"* — said twice in one day, in two sessions; memory held nothing |
| `B2-constraint-after-compaction` | Same, but the session is compacted before the work continues. | the constraint survives compaction via memory, not via context | silent | both sessions above began with a context compaction |
| `B3-scope-limit` | User narrows scope ("research only, no code changes"). | the limit respected for the rest of the arc | explicit | *"on the icons, research only for now, no code changes"* |
| `B4-precondition` | User states a precondition for a routine action ("pull all branches before opening PRs"). | precondition applied next time the action comes up | silent | *"before making PRs, pull the current branches — we might be working on a stale one"* |
| `B5-write-restriction` | User restricts what may be written **to memory** ("don't record the money side"). | the restriction honoured, and itself remembered | explicit | *"just don't write about the money in kaeru"* |
| `B6-do-not-reopen` | User closes a topic ("don't raise this again"). | the topic not reopened in later sessions | silent | *"don't worry about that bot, I checked it works — don't raise this for now"* |

## C. Memory was never consulted

*Eight interruptions, five with zero memory calls before the user intervened.*

| id | situation | must be recoverable after | mode | field evidence |
|---|---|---|---|---|
| `C1-reentry-cold` | Returning to a project after a break, with open debts waiting. | debts named before work starts | silent | *"first do awake in kaeru, then continue"* |
| `C2-deployment-target` | A fact about where things live (server, environment) is in memory; agent assumes instead. | the stored target used | silent | zero calls, then: *"what are you doing — that's deployed on a different server, look in kaeru"* |
| `C3-prior-decision` | A settled decision exists; agent proposes reopening it. | the decision found and respected | silent | *"no, we're definitely not changing the stack"* — zero calls |
| `C4-roadmap-not-written` | A plan is agreed in conversation; nothing records it. | plan stored and usable next session | explicit | *"did you write the roadmap into memory so we can follow it?"* |

## D. Capture dispatch — the verb matched to the epistemic status

*Not interruptions but graph damage: the wrong verb leaves the next session unable
to tell settled from open.*

| id | situation | must be recoverable after | mode | field evidence |
|---|---|---|---|---|
| `D1-retrospective-verdict` | A suspicion was checked before memory was reached; the verdict is already known. | a hypothesis carrying its verdict and evidence | explicit | 13 of 23 real claims arrived with the answer already known; several still sit open with "REFUTED" in the prose |
| `D2-partial-verdict` | The check ran and did not decide. | the third verdict recorded, not prose | explicit | "VERDICT: PARTIAL" written into bodies, status left open |
| `D3-arc-closed` | A line of work ends in verified delivery. | the outcome promoted to the archival tier | silent | five-episode arc ending "deployed and verified in production" — zero consolidation |
| `D4-settled-doc` | A durable document is produced (spec, rule, glossary). | stored as a reference, not a journal entry | explicit | "capturing everything as episode" — 545 episodes against 4 outcomes |
| `D5-trail-worth-saving` | Work runs from observation to decision across sessions. | the trail saved and readable later | silent | 22 chains written, zero read in the whole history |

## E. Debts and lifecycle

| id | situation | must be recoverable after | mode | field evidence |
|---|---|---|---|---|
| `E1-overdue-task` | A task with a deadline passed while work continued elsewhere in the same project. | the overdue task surfaced before new work | silent | a task eight days overdue while the agent closed a neighbouring one |
| `E2-task-closed` | Work completing a stored task is finished. | the task moved out of open | silent | 69 tasks created against 14 closed |
| `E3-contradiction` | New evidence contradicts a stored fact. | the conflict marked, not silently duplicated | silent | agent wrote "refuted-…" as a fresh episode, left the original unmarked |
| `E4-archived-needed` | The needed knowledge was demoted to a cold layer. | brought back into view | explicit | 141 demotions to cold, zero reads back |
| `E5-team-tier` | Working in a shared initiative where the team has published relevant nodes. | cloud tier consulted before answering locally | silent | 23 of 24 cloud sessions happened only because the user asked |

---

## Proportions

The suite follows the corpus, not a feature list. Group A is the smallest by count
and the largest by cost, so it carries the most cases per situation. Group B is the
most frequent mechanism in the field and is under-served by every existing memory
benchmark, because it requires a prompt in which nobody says a rule was stated.

Eighteen of twenty-seven are `silent`. That ratio is deliberate: in the corpus, the
prompts that preceded the most expensive failures were ordinary work requests.
