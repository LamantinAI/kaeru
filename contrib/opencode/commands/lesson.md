---
description: Capture a lesson learnt as durable archival memory in kaeru
---

Capture a lesson learnt — durable, settled, archival-tier knowledge.

Lesson body: `$ARGUMENTS`

1. If no body was provided, ask the user what the lesson is and stop here.
2. Pick the active initiative — the current project (cwd basename / git remote name). If unclear from context, ask the user once.
3. Derive a short kebab-case name from the body (≤ 6 words, lowercase, no punctuation). If the body already starts with a clear noun-phrase, use that.
4. Call `kaeru_cite(name="<derived-name>", body="<full body>", initiative="<initiative>")` — no URL. `cite` without a URL is the right verb for "a settled internal fact / lesson / decision"; it lands in the archival tier (cortex) so it survives the operational decay cycle.
5. If the lesson is conceptually adjacent to something the user just discussed, also `kaeru_link(from="<derived-name>", to="<related>", edge_type="refers_to", initiative=…)` — cited islands are findable only by exact name.
6. Confirm to the user with the node name and a one-line summary.

Working directory:
!`pwd`
