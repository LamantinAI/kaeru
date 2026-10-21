# Kaeru in Opencode

Tools are named `kaeru_<operation>` in Opencode. Use that prefix for the
operation examples below. The connected schemas are authoritative.

# Kaeru memory

Kaeru holds scoped records and typed graph relationships across sessions.
Use the tool schemas exposed by the connected server; the current API has 35
operations. Agent access has no query DSL; the offline `kaeru-mcp amg`
operator CLI handles knowledge transfer, not agent recall.

## Recover useful context

On returning to a project, `context(initiative="project")` restores standing
rules, current slots, open tasks/claims, reminders, pins and saved reasoning.
Use `initiatives()` when the established project name is unknown. A map or board
is `context(initiative="project", view="map")` or `view="board"`.
Choose the calls the task needs; these are available workflows, not a mandatory
protocol before every action.

Search with `search(initiatives=["project"], text="entity", prefix=true)`.
Text is literal lexical content, not an expression. `prefix=true` helps word
inflections; `match="all"` requires all terms. Search in the language stored.
Do not translate notes or queries to English to improve matching. A miss is
scoped, not proof that another initiative or a cloud has no relevant memory.

Read returned IDs with `read(initiative="project", node="<uuid>")`.
Currentness, material conflicts and saved summaries accompany ordinary reads.
`include=["relations"]` adds typed edges; `include=["provenance"]` follows
sources; `include=["path"], to="<uuid>"` previews a directed route.

Follow `continuation.next_arguments` when present. Inspect `coverage`, especially
`rules_complete`, and `body_range` before treating a response as complete.
A stale cursor means restart the read. A retired record describes the last
assertion and accessible successors; do not quote it as a current fact.

## Capture meaning

- `note`: observations, decisions, constraints, plans and outcomes. Set `kind`
  explicitly for a standing constraint or a plan.
- `reference`: exact supplied text with `source={"text":"..."}`, an authorized
  artifact, or a URL citation. URLs are not fetched. Use this for settled source
  documents; use notes for authored interpretations.
- `task`: work with an optional ISO date `due`. `task_update` changes status or
  deadline; `due=null` clears the deadline. Completion is the board's terminal
  status, not whichever column happens to be last.
- `claim`: a hypothesis. An already reached `initial_status` and evidence can
  be recorded in the same call.
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
`cloud_initiatives` and `cloud_recall`; `pull` makes a local copy. `link_cloud`
and `cloud_links` retain a remote reference. `policy` controls whether and where
an initiative may share. `share` and `unshare` are explicit; sharing also runs a
secret guard. With several configured clouds, name the one you mean.


## Export when asked

Use `export(format="markdown", initiative="project", destination="project.zip")`
for an archive of readable Markdown pages, or `format="amg"` with a `.amg`
destination for portable current knowledge. Choose exactly one of `initiative`,
`initiatives=[...]` or `all=true`. Export runs against the live service and needs
an authorized host artifact root. Return the artifact/resource link to the user;
MCP clients retrieve the file with `resources/read`. No history backup or agent
import is implied. Do not put archive bytes or base64 into a memory note.
