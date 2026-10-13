# Task board

A board is a per-initiative registry of status keys, labels, order and explicit
terminal flags. Tasks retain status and deadline tags on versioned nodes. The
built-in statuses are open, in-progress and done; done is terminal. Reordering
columns never changes whether a task is complete. On an older registry without
terminal flags, its former final column keeps the legacy terminal meaning.

Use `context(initiative="project", view="board")` to read columns and cards.
It returns the shared JSON envelope; `data.sections` contains the board section,
its records and `details.columns`. Follow continuation arguments when needed.
With `recorded_at`, both card content/status and versioned registry state are
read as of that moment. Layer, visibility and memberships remain current metadata.

Create work with `task(initiative="project", body="Ship the fix", due="2026-10-20")`.
`task_update(initiative="project", task="<uuid>", status="done")` preserves its
deadline. `due=null` deliberately clears it. Status keys must exist in the board.
Body changes use `revise` with a patch.

`board_config` supports add, remove, relabel, reorder and set_terminal. Use the
returned board revision for `expected_revision` when editing concurrently. Open
must remain nonterminal and the final terminal column cannot be removed. Move tasks to another status before removing an occupied column.

HTTP uses `POST /v1/context`, `/v1/task_update` and `/v1/board_config` with those
same arguments. Each handler applies Principal, operator scope/visibility and
redaction; old GET board/column routes are removed.
