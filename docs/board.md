# The task board

A simple, customizable tracker over kaeru's existing `task` nodes — and the
contract a UI (e.g. `kaeru-viz`) builds a board on.

Nothing new in the substrate beyond one node type; no migration. Existing tasks
are already on the board the moment you look.

## The model

Two things that are easy to conflate, deliberately kept apart:

| Thing | Where it lives | Why |
|---|---|---|
| A task's **current column** | a `status:<key>` tag **on the task node** | it's a property of the task, not a separate entity. Moving a card is one rewrite of that tag — so the move history is free |
| The **set and order of columns** | one `Board` node per initiative, ordered `{key, label}` list in `properties.statuses` | columns are *ordered*, and tags are an unordered set — so this belongs in `properties` |

- **`key`** is stable — it's literally the tag on the task (`status:in-progress`).
- **`label`** is the human caption. Re-editing it touches **no task**.

A board is **per-initiative**: one initiative = one board. "The tracker for
project X" is just X's tasks plus X's column registry.

### Defaults, and when the Board node appears

The built-in vocabulary is `open` → `in-progress` → `done`, with `open` first —
which is exactly what `write_task` already stamps. So:

- existing tasks are already in column one;
- **no `Board` node exists until an initiative actually customizes its columns**
  (a read never writes);
- `effective_statuses` = the customized registry if there is one, else the
  built-in defaults.

### Time travel comes for free

A card's column is a bi-temporal tag, so reading the substrate at a past moment
rewinds the *whole board*: the columns of that day, and each card in the column
it was in then. A UI with a time scrubber gets sprint replay without storing a
single extra event.

## Verbs

Same surface in `kaeru-core`, over MCP, and in `kaeru-rig`.

| Verb | Does |
|---|---|
| `board` | read the board — columns in registry order (**empties included**) with tasks bucketed in. Optional `when` rewinds it |
| `set_status` | move a task to a column |
| `board_status` | customize the registry: `add` / `remove` / `relabel` / `reorder` |

### `board`

MCP: `board(initiative, when?)` — `when` accepts unix seconds, RFC-3339, or
`5m` / `2h` ago. rig: `kaeru_board(initiative?, when?)` with `when` in unix
seconds; it returns JSON:

```json
{
  "initiative": "proj-x",
  "columns": [
    {
      "key": "open",
      "label": "Open",
      "tasks": [
        { "id": "0198…", "name": "write the spec", "excerpt": "…", "due": "2026-08-01", "ts": 1786... }
      ]
    },
    { "key": "in-progress", "label": "In Progress", "tasks": [] },
    { "key": "done", "label": "Done", "tasks": [] }
  ]
}
```

- Columns arrive **in registry order** — render left to right as given.
- Empty columns **are** included, so the board doesn't flicker as it empties.
- `due` is the task's `due:` date when it has one; `ts` is the assertion time
  (when the card last changed) — useful for sorting and for the scrubber.
- A task whose status isn't a known column (drift / a removed column) falls into
  the **first** column rather than disappearing.

### `set_status` — the drag-and-drop target

`set_status(initiative, task, status)`; `task` is a name or id.

**Strict**: `status` must be a key in the initiative's registry, else the call is
refused with the known keys listed. This is the one place kaeru enforces rather
than hints, and it gates only the *vocabulary* — a typo must not silently spawn a
phantom column. It does **not** gate the workflow: any task may move to any
column, in any order (no "can't go from open straight to done").

So a UI can move a card anywhere, but should offer only the columns `board`
returned.

### `board_status` — the column editor

`board_status(initiative, action, …)`:

| action | args | note |
|---|---|---|
| `add` | `key`, `label?` | appended last; `label` defaults to `key` |
| `remove` | `key` | refuses to remove the last column. Tasks still tagged with it aren't touched — they show in column one until re-statused |
| `relabel` | `key`, `label` | cheap: registry only, no task touched |
| `reorder` | `order` | must be exactly the existing keys, permuted |

The first edit materializes the `Board` node, seeded from the defaults.

Returns the new list: `{"statuses": [{"key": "...", "label": "..."}, …]}`.

## Wiring a UI

1. **Render** — call `board(initiative)`; draw one column per entry, in order.
2. **Drag a card** — `set_status(initiative, task=<id>, status=<target key>)`,
   then re-read (or move optimistically and reconcile).
3. **Edit columns** — `board_status(...)`, then re-read.
4. **Scrub time** — call `board(initiative, when=<t>)` per scrubber position.
   Nothing else to wire; every position is a real historical board.

## Deliberate non-goals (for now)

- **One board per initiative.** Multiple boards inside one initiative would need
  a board id on every task; use a separate initiative per board instead.
- **No transition rules / WIP limits.** The board describes columns; it doesn't
  police movement. Such policy belongs in the app layer, not the substrate.
- **No assignees or priority fields.** Tasks carry `due:` today; anything more is
  a tag or a `properties` field away, but isn't modelled yet.
