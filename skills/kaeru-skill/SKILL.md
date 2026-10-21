---
name: kaeru
description: Recall, capture and inspect persistent project memory through the kaeru MCP server. Use for cross-session project continuity, durable decisions, standing constraints, open work and saved reasoning.
---

# Kaeru memory

Kaeru holds scoped records and typed graph relationships across sessions.
Use the tool schemas exposed by the connected server; the current API has 35
operations. Agent access has no query DSL; the offline `kaeru-mcp amg`
operator CLI handles knowledge transfer, not agent recall.

Respect memory-write exclusions before applying capture examples. Persist the
exclusion itself as a standing constraint, retaining its full meaning and scope
in the user's context rather than a narrower paraphrase. It covers indirect
representations and derived disclosures as well as verbatim values. A later
request to record an update does not revoke the exclusion. Check the proposed
body, source, description and metadata before writing; keep only an allowed
high-level outcome when the update contains excluded material. If the boundary
is unclear, ask before expanding it. Only an explicit change from the user
widens what may be persisted.

## Recover useful context

On returning to a project, `context(initiative="project")` restores standing
rules, current slots, open tasks/claims, reminders, pins and saved reasoning.
Use `initiatives()` when the established project name is unknown. A map or board
is `context(initiative="project", view="map")` or `view="board"`.
Choose the calls the task needs; these are available workflows, not a mandatory
protocol before every action.

Raise relevant overdue or blocking work before or alongside the requested task.
State its due date or age and the impact recorded in memory, rather than merely
listing it as open. Do not invent deadlines or blockers; continue the requested
work as well.

`context` is a scoped working set, not an exhaustive search of everything
known. Complete coverage describes that view, not the absence of other rules.
Before making a lasting choice, check governing decisions and constraints,
not only activity about the work item. A relevant authority must actually
address the choice; mentioning the same component is not enough. If context or
topic search supplies no governing source, search relevant reference/constraint
records. When vocabulary is uncertain, a bounded metadata-filtered search such
as `filters={"types":["reference"]}` can discover source names; read the relevant
source before treating the choice as unconstrained.
Search includes cold/frozen records unless a layer filter excludes them; do not
mistake a record omitted from context for missing knowledge. After a scoped
miss, discover accessible initiatives and search related project scopes before
concluding no decision exists. Stay within the host's authorization ceiling.

Search with `search(initiatives=["project"], text="entity", prefix=true)`.
Text is literal lexical content, not an expression. `prefix=true` helps word
inflections; `match="all"` requires all terms. Search in the language stored.
Do not translate notes or queries to English to improve matching. A miss is
scoped, not proof that another initiative or a cloud has no relevant memory.

Read returned IDs with `read(initiative="project", node="<uuid>")`.
Currentness, material conflicts and saved summaries accompany ordinary reads.
`include=["relations"]` adds typed edges; `include=["provenance"]` follows
sources; `include=["path"], to="<uuid>"` previews a directed route.

Follow `continuation.next_arguments` unchanged when present. To change a page
budget or search condition, restart without the old cursor. Inspect `coverage`, especially
`rules_complete`, and `body_range` before treating a response as complete.
A stale cursor means restart the read. A retired record describes the last
assertion and accessible successors; do not quote it as a current fact.

## Capture meaning

- `note`: observations, decisions, constraints, plans and outcomes. Set `kind`
  explicitly for a standing constraint or a plan.
- `reference`: exact supplied text with `source={"text":"..."}`, an authorized
  artifact, or a URL citation. URLs are not fetched. Use this for settled source
  documents; use notes for authored interpretations. When the user supplies a
  final document, preserve the supplied text verbatim as the reference source;
  a paraphrased note labeled "final" is a different artifact.
- `task`: work with an optional ISO date `due`. `task_update` changes status or
  deadline; `due=null` clears the deadline. Completion is the board's terminal
  status, not whichever column happens to be last.
- `claim`: a testable suspicion or hypothesis, including one previously captured
  as an ordinary note. An already reached `initial_status` and evidence can be
  recorded in the same call. When evidence decides such a question, use a
  structured claim/verdict and connect the evidence and earlier suspicion.
  Rewriting a note's body to say "resolved" or "refuted" does not record a
  machine-readable verdict. Keep content revisions and epistemic status distinct.
- `verdict`: `status="supported"`, `"refuted"` or `"inconclusive"`, optionally
  `evidence={"method":"what was actually checked"}` or `{"node":"<uuid>"}`.
  `action="record_check"` records evidence without deciding status.

Pass the established initiative. New names are accepted but reported; check
that a transliteration or accidental alias did not split the project.
Names resolve exactly inside that initiative. Prefer returned IDs when names
repeat. An ID is not permission to reach a different initiative.

A capture can include `related={"to":"<uuid>","type":"derived_from",
"weight":0.8}`. Separate graph edits use `relate(action="connect", from=...,
to=..., type=..., weight=...)`, `disconnect` or `set_weight`. Choose weights
explicitly; they express the connection's importance, not a measured similarity.
`supersedes` runs from the replacement to what it replaces. A relation alone
does not retire a record; `replace` performs a semantic replacement.

## Maintain inspectable memory

`revise` edits an identity using a patch. Exact source bytes are immutable;
change a source with `replace`. `lifecycle(target_tier="archival")` promotes
settled knowledge while preserving references and reporting the resulting ID.
Tier and recall layer are independent; `retention(action="set_layer", ...)`
controls how eagerly a record returns to context.

When work is completed or a discussion is settled, inspect the older records
that still present it as pending. Revise ordinary work notes to state the
resolution, or explicitly replace assertions that are no longer current.
Keep their provenance and recorded history. A new completion summary or a move
to archival does not by itself resolve an older "still pending" assertion.
Read back the affected records before claiming the memory is consistent.
After a mutation, use the returned current ID and revision for subsequent
operations rather than an identifier that the mutation retired.

`review` opens a doubt, answers an open record, or closes selected review flags.
Closing flags leaves substantive counterevidence. Record the actual conclusion,
not only a change to a status field.

`trail` saves `route={"from":"<uuid>","to":"<uuid>"}` or ordered `members`
with an authored `summary`. Refresh/extend can change the route; a summary stays
stale until an updated explanation is supplied. Ordinary reads surface saved
summaries without needing a special lookup call.

`options={"slot":"handoff"}` on capture selects the current handoff pointer.
It does not retire the previous holder or change a role. `context` returns slot
revisions; `initiative_manage` explicitly sets or clears them.
`reflect` proposes findings; it does not apply semantic cleanup automatically.
`forget` retracts content and edges while preserving history.

`history(initiative="project", node="<uuid>")` returns recorded versions and
metadata audit descriptors. `read(..., recorded_at="2026-10-01T12:00:00Z")`
shows an earlier assertion. Current layer/visibility/properties are labeled as
current metadata. There is no separate valid-time query axis.

## Read mutation outcomes

Local tools return JSON with `api_version`, `operation`, `state`, `data` and
`issues`. Trust the returned committed IDs and revisions. `partial` can mean a
local capture succeeded but a relation or remote supplement failed; inspect
`failed_steps` before retrying. A host-provided `request_id` safely recovers an
uncertain outcome. `replayed=true` describes that original operation.

## Optional team cloud

Local reads do not include cloud memory. Discover with `clouds`,
`cloud_initiatives` and `cloud_recall`; pass `initiative` and `id` to
`cloud_recall` for the full shared record without importing it. `pull` makes a
local copy and restores its incident edges, preserving local edge decisions.
An error with `node_imported=true` means the node is already local but the
edge import is incomplete; retry rather than assuming nothing was written. `link_cloud`
and `cloud_links` retain a remote reference. `policy` controls whether and where
an initiative may share. `share` and `unshare` are explicit; sharing also runs a
secret guard. With several configured clouds, name the one you mean.
