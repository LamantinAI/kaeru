# kaeru-skill

A portable agent skill that teaches an LLM how to use `kaeru-cli` for
persistent memory — capture, recall, reasoning, time-travel.

`SKILL.md` is the source of truth. The frontmatter is in Anthropic's
Claude Code skill format; the body is platform-neutral and can be
pasted as a system-prompt rule into any agent runtime.

## Prerequisites

- `kaeru-cli` installed and on `$PATH` (built from this repo:
  `cargo install --path kaeru-cli`).
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

### Cursor / Continue / OpenCode / other IDE-embedded agents

These don't currently support a SKILL-MD format directly. Paste the
**body of `SKILL.md`** (everything after the `---` frontmatter) into
your agent's system-prompt or "rules" section.

### Aider / generic CLI agents

Same as above — strip the frontmatter, treat the remaining markdown
as instructional context for the agent.

### MCP-based runtimes (future)

When `kaeru-mcp` lands, this skill won't be needed for MCP-aware
agents — they'll discover the curator-API tools natively through MCP's
`tools/list`. The skill's value persists for shell-out CLI use cases
and for agent runtimes that don't speak MCP.

## Updating

The skill is the body of `SKILL.md`. When the curator-API surface
grows (new verbs, new conventions), update `SKILL.md` here and bump
the symlink-pointed copies on each install host. There's no version
field — every commit on `main` is the current canonical version.

## Why the skill exists

Without it, an agent re-entering a project either ignores kaeru
entirely (loses continuity) or has to discover the verb taxonomy via
`kaeru --help`-style trawling each session. The skill gives the agent
the re-entry ritual (`initiatives → awake → overview`), the verb
mental model (capture / inquire / reason / evolve / time-travel), and
the `--initiative` discipline up front.
