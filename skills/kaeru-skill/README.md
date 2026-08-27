# kaeru-skill

A portable agent skill that teaches an LLM how to *use* kaeru: which verb
carries which kind of knowledge, the rituals at both ends of a session, and
the habits that turn a pile of notes into a graph worth reading.

There is no CLI — kaeru is the `kaeru-mcp` daemon, and the skill is written
in the form the verbs are actually called: `awake(initiative="x")`. An
MCP-aware runtime discovers the verbs themselves through `tools/list`; what
this skill adds is the *when and why*, which no tool schema can carry.

`SKILL.md` is the source of truth. The frontmatter is in Anthropic's
Claude Code skill format; the body is platform-neutral and can be
pasted as a system-prompt rule into any agent runtime.

## Prerequisites

- The `kaeru-mcp` daemon installed and running
  (`cargo install --path kaeru-mcp`, then register it with your agent —
  see the repo README). MCP-aware runtimes discover the verbs themselves;
  this skill adds the *when and why* on top.
- A vault location either at the platform default
  (`$XDG_DATA_HOME/kaeru` on Linux, `~/Library/Application Support/ai.lamantin.kaeru`
  on macOS, `%LOCALAPPDATA%\ai.lamantin.kaeru` on Windows) or set
  explicitly via `KAERU_VAULT_PATH`.

## Install per platform

### Claude Code

Symlink (so updates from the repo flow through automatically):

```bash
ln -s "$PWD/skills/kaeru-skill" ~/.claude/skills/kaeru-skill
```

Or copy if you prefer a snapshot:

```bash
cp -r skills/kaeru-skill ~/.claude/skills/kaeru-skill
```

The skill auto-triggers on memory-related phrases ("remember",
"save this", "what did I think about X", …) and is user-invocable
via `/kaeru`.

### Opencode

Opencode speaks MCP natively — see [`contrib/opencode/`](../../contrib/opencode/)
for a turn-key wiring: an `AGENTS.kaeru.md` system-prompt include,
slash commands (`/kaeru`, `/lesson`, `/recall`), an additive
`opencode.kaeru.json` snippet that adds the daemon as a remote MCP
server, and an installer that merges into your existing
`opencode.json` without touching your model providers or API keys.

### Cursor / Continue / other IDE-embedded agents

These don't currently support a SKILL-MD format directly. Paste the
**body of `SKILL.md`** (everything after the `---` frontmatter) into
your agent's system-prompt or "rules" section.

### Aider / generic CLI agents

Same as above — strip the frontmatter, treat the remaining markdown
as instructional context for the agent.

### MCP-aware runtimes

MCP-aware agents (Claude Code, Codex, Opencode, Cursor) discover the verbs
themselves through `tools/list`, and the daemon ships a compact ontology in
its server instructions — so the tools work without this skill.

What the skill still adds is judgement the tool list can't carry: which verb
matches which epistemic state, the rituals at both ends of a session, the
habit of linking and chaining after a capture. The daemon's own instructions
are capped at 2048 characters by the client — the skill is where the rest of
the reasoning lives. Install it if you want that discipline taught
explicitly; skip it if the built-in instructions are enough.

## Updating

The skill is the body of `SKILL.md`. When the curator-API surface
grows (new verbs, new conventions), update `SKILL.md` here and bump
the symlink-pointed copies on each install host. There's no version
field — every commit on `main` is the current canonical version.

## Why the skill exists

An agent that only sees the tool list knows *what exists*, not *when to
reach for what*. The skill supplies the re-entry ritual
(`initiatives → awake → overview`), its counterpart at the other end (the
terminal verbs, then `settle` / `synthesise`, then `reflect`), the verb
mental model (capture / inquire / reason / evolve / time-travel), and the
`initiative` discipline up front.

That is the difference between a graph that accumulates and one that stays
navigable — and it is mostly about the moments a tool schema cannot describe:
when a thought is worth a name, when work has stopped changing, when a local
answer is confidently incomplete.
