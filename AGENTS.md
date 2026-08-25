# AGENTS.md — working agreements

How agents work *in* this repository. What the code **is** lives in
[CLAUDE.md](CLAUDE.md); this file is about process.

## Git: commit locally in a batch, push squashed

Commit as often as the work has natural steps — a passing test, a finished
module, a doc update. Local history is a workbench: it costs nothing, it makes
a mistake cheap to walk back, and it keeps a long task from ending in one
undifferentiated blob.

**The remote sees one commit per batch of work.** Before pushing, collapse the
local run onto the upstream tip and push that:

```sh
git fetch origin
git reset --soft origin/main     # keeps every change staged, drops the local commits
git commit                       # one message describing the whole batch
git push origin HEAD
```

Never push the intermediate commits and squash afterwards — rewriting pushed
history is the thing this avoids.

Two consequences worth stating:

- **The squashed message is the one that survives**, so it carries the whole
  batch: what changed, why, and what it was verified against. The local
  messages are scaffolding and can be terse.
- **Squash before pushing, not after.** If a batch is genuinely two unrelated
  changes, push it as two batches — squash the first, push, then the second.
  One commit per batch, not one commit per push session.

## Commit authorship

Commits are authored by `GrumpyChubbyCat <lamantin-ai@yandex.ru>`. No
assistant attribution — no `Co-Authored-By` for a model, no generated-with
trailer.

## Pull requests

Merged with `--squash`, and always with a thank-you comment. Issues are closed
with one too, saying what was actually done and what was deliberately left —
a closed issue should explain itself to whoever opens it next.

## Language

Release notes, issues, PR reviews and code comments are in **English**.
Telegram posts are in plain Russian — analogies over anglicisms.
