---
description: Recall what kaeru knows about a topic — search then drill the top hit
---

Recall what kaeru knows about: `$ARGUMENTS`

If `$ARGUMENTS` is empty, ask the user what to look up and stop here.

1. Pick the active initiative from the current project (cwd basename / git remote name). If you have no clear initiative, run cross-initiative (omit the `initiative` parameter).
2. Call `kaeru_search(query="$ARGUMENTS*", initiative=…)` — the trailing `*` is critical: kaeru FTS does not stem, so the prefix glob is what catches "tokens" vs "token", "утечку" vs "утечка", etc.
3. If 0 hits: try once more without the `*` (some exact-form queries miss with glob), then once more without `initiative` (the answer may live in `cortex` or another project). If still 0, report that to the user — do not invent.
4. If ≥ 1 hit: call `kaeru_drill(name="<top-hit-name>", initiative=…)` to get the brief + 1-hop neighbourhood in one round-trip. Do **not** re-search with reworded terms — change the query *shape* (try `kaeru_tagged(tag="topic:$ARGUMENTS")`) before reaching for variant phrasings.
5. Summarise the top hit + its neighbourhood for the user in 3–5 lines.

Working directory:
!`pwd`
