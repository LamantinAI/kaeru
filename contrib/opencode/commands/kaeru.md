---
description: kaeru re-entry ritual — load process state + epistemic state for an initiative
---

Run the kaeru re-entry ritual.

Initiative: `$ARGUMENTS` (if empty, derive from the current working directory's basename or git remote name — ask the user if neither yields a clear name).

1. Call `kaeru_initiatives()` to see existing projects.
2. If the chosen initiative is not in the list, tell the user and propose creating fresh state by starting to capture into it; if it is, continue.
3. Call `kaeru_awake(initiative="<chosen>")` — surfaces pinned set, recent episodes (24h), and open reviews.
4. Call `kaeru_overview(initiative="<chosen>")` — surfaces counts by tier/type, provenance forests, open questions.
5. Summarise back to the user in two sentences: what was open, what the project knows. Then ask what they want to do next.

Working directory context:
!`pwd`
!`git -C "$PWD" remote get-url origin 2>/dev/null || echo "(no git remote)"`
