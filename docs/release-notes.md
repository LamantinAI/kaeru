# Release notes — how they are written

Notes live outside this repository, one directory per version:
`marketing/v<x.y.z>/release-v<x.y.z>.md`. They are written in English, and
they are the only artefact most users read about a release — so they are
written for someone deciding **whether to upgrade today**, not for someone
admiring the work.

The shape depends on which digit moved. The rules below the templates apply
to all three.

---

## PATCH — `0.7.3 → 0.7.4`

Things that were broken are less broken. Nobody has to relearn anything.

```markdown
# kaeru <version>

<One or two sentences naming what this release is for. If it is mostly one
thing, say "Mostly one thing:" and name it. If it is a sweep, say how many
fixes and what they have in common.>

## <What the reader was hitting, as a statement>

<The defect in the user's terms, then the fix. Lead with the symptom they
would recognise, not the module that held it.>

## <Next one>

…

## Also

- <Fixes too small for a section — one line each.>

## Breaking changes

<Only if something changed that a caller, a stored vault, or a script can
trip over. Numbered, each with what to do about it. A patch release SHOULD
NOT have this section; if it does, say why the change could not wait.>

## Upgrade

<Commands. State plainly whether the vault upgrades itself, and whether the
upgrade is reversible.>

## Prebuilt binaries

<Table: OS, arch, asset name. Any asset that is not verified says so.>

<N> tests green across the workspace.
```

**A patch release that replaces an already-published binary says so in its
own paragraph**, with the date and what was wrong with the old one. Two of
ours did (0.7.1 and 0.7.2, after the cross-compiled Linux binary SIGSEGV'd),
and a reader who downloaded last week has no other way to find out.

---

## MINOR — `0.7.4 → 0.8.0`

New capability, or old capability that now works differently. The reader has
to change something — a habit, a call, a prompt.

Everything from the patch template, plus:

```markdown
## What this release is about

<Two or three paragraphs. A minor release usually has a theme; name it.
If it does not have one, that is worth knowing before publishing.>

## <Feature, in the words of the problem it solves>

<What was impossible or annoying before. Then the new thing. Then an example
that fits on a screen.>

## Breaking changes

<Mandatory section if ANY of these changed, even when the vault needs no
migration:
  1. a parameter's name, type, or whether it is required;
  2. a default;
  3. the shape of output a script might parse;
  4. what a verb does when given the same input as before;
  5. stored data, in a way an older build cannot read.
Number them, and for each say who breaks and what to do. Do not describe a
breaking change anywhere else instead — 0.7.2 buried "`link` now requires
`weight`" in a feature paragraph and had to be corrected after release.>

## Deprecations

<What still works but will not. When it goes. What replaces it.>
```

---

## MAJOR — `0.x → 1.0`, `1.x → 2.0`

The shape of the thing changed. Assume the reader has a working setup they
are afraid of losing.

Everything from the minor template, plus:

```markdown
## Why the shape changed

<The argument, honestly. What was learned that made the old shape wrong.
Cite the evidence — an audit, a measurement, a number of sessions.>

## What was removed

<Each removal with its replacement and a before/after. "Removed" without
"instead, do this" is how a major release loses its users.>

## Migrating

<Step by step, in the order a person will do them. Include the rollback:
what to export first, and what cannot be undone.>

## What did not change

<The promises that still hold. A major release is frightening; the list of
things a reader does not need to re-learn is load-bearing.>
```

---

## Rules that hold for every release

**Lead with the symptom, not the subsystem.** "An obsolete fact no longer
outranks the one that replaced it" tells a reader whether it affects them.
"Fixed hygiene in-degree calculation" does not.

**Carry the evidence.** Where a number exists, quote it: *6,003 calls in 206
sessions*, *23 nodes and 0 edges after seven nudges*, *1,262 of 1,262 `link`
calls omitted `weight`*. A claim with a number behind it survives scrutiny;
one without reads as marketing, and we do not write marketing.

**Name what is still wrong.** Unverified binaries, known gaps, things a fix
did not cover. 0.7.3 shipped with "nobody has confirmed a released macOS
build opens an existing vault" and that sentence is worth more than another
paragraph of good news.

**Say what a fix costs.** A grace period that delays archiving, a budget that
truncates a window, a required argument — each has a downside, and the reader
should hear it from us.

**No release number in the first sentence.** They know which release they are
reading; they do not know whether to care.

**One line at the end: `<N> tests green across the workspace.`** It is the
only quality claim we make, and it is checkable.

---

## The tag

The annotated tag carries the same summary in one screen: what closed, what
breaks, the test count. Two mechanical traps, both of which have bitten:

- **Write it with `-F <file>`, never as a shell string.** Backticks in the
  message become command substitution, and the words inside them vanish.
- **Add `--cleanup=verbatim`.** `git tag` strips lines that *start* with `#`
  as comments, so a line beginning `#98 (…)` disappears from the tag without
  a word of warning.

```sh
git tag -a -F /tmp/tag-v0.7.4.txt --cleanup=verbatim v0.7.4
```

## The GitHub release

Body is the notes file verbatim. Assets: the per-platform archives, the
`.mcpb` bundles, and `SHA256SUMS`. The release is published **after** the
binaries are attached, so nobody downloads a release that has nothing in it.
