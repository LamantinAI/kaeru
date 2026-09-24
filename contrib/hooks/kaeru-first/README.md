# kaeru-first — check memory before asking the human

A harness hook for **Claude Code** and **Codex**. It serves one moment: the
agent is about to hand the turn to the human. It takes the entities out of
what the agent is asking, asks kaeru whether memory holds anything on them,
and — only if it does — sends the agent to read those nodes first. Once per
turn, never in a loop, and it fails open.

## Why

Usage audit 4 (#89): the agent reads kaeru almost always, just never at the
moment it asks. **80% of its questions to the user came with no kaeru read in
the previous ten minutes**; the median gap was 74 minutes. The first version
of this hook gated that moment on a timer: no read in the window → deny once,
with a search recipe.

Usage audit 5 replayed it over every turn where the agent stopped and waited
for the human — 2,608 of them in one user's logs — and changed two things.

**The moment is wider than a question mark.** In plain text, the hook's
detector now recognises 1,164 of those turns as an ask. The first version's
`Stop` could see 201 of them (17%) — a reply whose last line ends in `?`. The
largest class it missed, 856, was an imperative hand-off: *"say «fix it»"*,
*"I need your answer about the provider"*, *"send me the report"*. Then a
question above the last line (72) and an offer that says it is one (35).
(`AskUserQuestion` is a separate path; both versions see it.) The human's
*"it's in kaeru"* replies landed on the invisible shapes far more often than
on the visible ones.

Two corrections are worth keeping, because both came from the hook's own first
days rather than from the design. An **enumerated list** with a "choice cue"
was a fourth shape: it matched 133 replies, and of a sample of twenty none was
a question memory could answer — status reports with bold bullets and *"next:
do X"* hand-offs. The hook's first live firing was a false positive on exactly
that, and the shape is gone. And a bare *"waiting"* was an imperative: 143
replies hung on it alone, nearly all of them the agent waiting for an artefact
or an event — *"waiting for the run to finish"*, *"waiting for the photo"* —
not for knowledge; only the addressed form (*"waiting for your answer"*)
counts now. The general caution: the corpus is "turns the human replied to",
and the human eventually replies to every final turn, so every count here is
an upper bound on asking.

**A timer is the wrong gate for a wider net — and so is a lexical one.** Half
of those hand-offs are procedural — *"write «done»"* — and memory cannot
answer them. A timer would have blocked every other turn. The obvious
replacement was built and measured: search kaeru for the question's terms and
block only when memory *has evidence*. It does not work.

49 asks the human answered with some form of *"it's in kaeru"*, against 300 he
did not, live searches on the same vault:

| rule for "memory has evidence" | catches the 49 | blocks the 300 |
|---|---|---|
| any hit at all | 100% | 98% |
| ≥ 2 of the question's terms in name + excerpt | 24% | 22% |
| ≥ 1 term in the node **name**, or ≥ 2 anywhere | 83% | 71% |
| ≥ 50% of the terms covered | 4% | 6% |
| `AND` of the two longest terms, over the full body | 22% | 26% |

No rule separates the two groups — each catches misses about as often as it
blocks everything else. The vault is dense (98% of asks hit *something*), the
ask is mostly procedural words, and the entity the human's answer turned on is
often the shortest word in it (*"once you tell me about the **key**, I'll run
generation and bring the second column"*). Whether memory answers a question is semantic, and
kaeru has no embeddings by design. Documented here so nobody builds that gate
a second time.

So the hook does the part it can do and leaves the judgement to the agent. It
runs the search **itself** and blocks once with the top hits, names first. The
agent glances at three excerpts and either reads one or asks. The block costs
a turn, not a round of tool calls — and an agent that already searched on the
same terms never sees the hook at all.

## What it does

| event | condition | action |
|---|---|---|
| `PostToolUse` on a kaeru read verb | — | remembers when, what verb, which terms it consulted, and (for `search`) which names came back |
| `PreToolUse` on `AskUserQuestion` *(Claude Code)* | gate says so | denies once per turn, naming the hits |
| `Stop` | the reply asks in any recognised shape, gate says so | blocks once, same message |
| `UserPromptSubmit` | previous turn asked | classifies the reply: a substantive answer gets the capture reminder; *"it was in memory"* is logged as a miss and gets a stronger one |

**Shapes `Stop` recognises**, looking at the last eight lines: a line ending
in `?` (last or not), an imperative addressed to the human (*tell me, let me
know, please confirm, send me, I need your …, your call*), an offer waiting for a
yes (*Want me to …, If you want …, Would you like …*). Codex has no `AskUserQuestion` — it asks in plain
text — so there this is the whole mechanism.

**The gate**, in order:

1. **Procedural → pass, no search.** A quoted go-word (*say «fix it»*),
   yes/no-shaped options (*Yes, go / No, later*), a bare confirmation stem
   (*Ship it?* — but only a short line that offers no alternative and names no
   entity: *"Should I use the staging token or the prod one for the Acme
   deploy?"* starts the same way and is searched), or nothing left after the
   stopwords. Memory has no
   answer to these, and blocking a *"push?"* to force a search is worse than
   not asking. 14% of real asks.
2. **A relevant read → pass.** A recent read (`search`, `at`, `neighbours`,
   `awake`, …) whose terms overlap the question's. The agent already looked.
   A read about something else does not count — recency is not relevance.
3. **Hits the agent never read → deny.** The last `search` returned names and
   this session has read none of them — at any point: a node read two hours
   ago is still in context, and asking for it again would be nagging. A search you do not read is not
   a search; the message names what was skipped.
4. **Show the hits → deny once.** One unscoped `search` of the question's
   terms as `OR`-ed prefixes — entities first (a Latin or digit-bearing token,
   a hyphenated name, a word capitalised mid-sentence), then by length. Hits
   are **ranked, not judged**: a name match first. The message shows the top
   three with their excerpts and says so plainly — *the hook cannot judge
   relevance; you can.* No hits at all → pass. Daemon unreachable → pass; a
   search the agent could not run either is not worth blocking for.

**What counts as a read:** `search`, `at`, `drill`, `awake`, `recall`,
`neighbours`, `why`, `board` and the other verbs that consult memory.
**Does not:** writes, and `initiatives` — it lists scopes without reading
them.

## The decision log

Every decision is appended to `decisions.jsonl` in the state directory: the
shape, the terms, the reason, the hits — and for each human reply after an
ask, how it read (*answered*, *miss*, *complaint*, *capture nudge*). Whether
the gate earns its keep is then a measurement, not an impression:

```sh
python3 kaeru_first_report.py
```

prints denials by reason, what the human said after each decision, and the
number that matters — misses the gate had let through, and why. The log also
carries `name_hits` for every shown block, so whether a name match predicts a
useful block is one more thing that can be measured rather than argued.

A reply whose markers fire with **no recognised ask** before it is logged too,
as `unasked:miss` and friends, and counted apart. Nothing is injected for it —
the markers are loose, and a statement can trip them on its own — but it is
the only trace an ask the detector never saw leaves behind, so it stands as an
upper bound on the detector's blind spot. The reply's own text is never
logged, only its length: an answer to *"which key is it?"* is exactly the
thing a log should not hold.

## Lexicons

Everything the hook knows about a human language is **data**: the stopwords,
the verbs that make a hand-off, the shape of a yes/no label, what *"it was in
memory"* sounds like. The script carries an English lexicon so it works as a
single file. Any `lexicon/*.json` beside it — and any in
`$KAERU_FIRST_LEXICON_DIR` — is merged in, so a vault written in another
language needs a file, not a fork. A Russian one ships in `lexicon/ru.json`;
copy the directory along with the script.

```json
{
  "stop":        ["…function words and procedural vocabulary…"],
  "quoted_go":   ["…verbs that precede a quoted go-word…"],
  "confirm_stem":["…"], "yesno_label": ["…"], "alternative": ["…"],
  "imperative":  ["…"], "offer": ["…"],
  "miss":        ["…regex fragments…"], "complaint": ["…"], "capture": ["…"]
}
```

`stop` is a list of words; every other key is a list of regex fragments,
alternated with the English ones. Each file is validated on its own: one that
does not parse, or carries a fragment that does not compile, is skipped with a
line on stderr, and the others still apply. kaeru's own rule holds here too:
store and search in the user's language — so the net that catches an ask has
to speak it as well. A personal lexicon — the phrases *you* use when the agent
should have looked first — belongs in your own directory, not in the repo.

Precision matters more than recall in these lists. A bare *"confirm"* also
matches *"I can confirm the tests pass"*; *"I could …"* and *"Happy to …"* open
as many reports as offers. An entry should be the addressed form.

The tokenizer is script-agnostic, and treats a Latin token inside prose in
another script as an entity.

## An optional semantic judge (unproven, off)

Three of the hook's decisions are questions about meaning — is this an ask, is
it procedural, and does a hit answer it — and all three are made lexically.
The third was measured and failed: the table above is why the hook shows
candidates instead of judging them.

`judge.py` is a candidate second attempt with a different method: TypeSafe's
[Jev](https://typesafe.ai/), a decision model that answers a yes/no question
with a calibrated probability instead of text. **It is not wired into the
hook.** It exists so the question "does it beat the lexicon?" can be answered
with a number before anything changes, which is what `eval_judge.py` is for:

```sh
# the lexical baselines alone — no network, no key
python3 eval_judge.py corpus.jsonl

# and the judge, on a corpus you are allowed to send
KAERU_FIRST_JUDGE=jev TYPESAFE_API_KEY=... python3 eval_judge.py corpus.jsonl --judge
```

It prints the same four numbers for each rule — catch, block, precision,
lift — so the comparison is like for like. The bar to ship it: **lift ≥ 2 and
precision well clear of the lexical ~4%.** Below that, it is a second negative
result and the judge stays where it is.

**If it ever does get wired in, it stays opt-in.** Two environment variables,
both of them absent by default:

| variable | meaning |
|---|---|
| `KAERU_FIRST_JUDGE` | `jev` turns the judge on; anything else, or unset, is off |
| `TYPESAFE_API_KEY` | the key; without it the judge does nothing at all |

Missing either, or a timeout, an error, an answer shape it does not
recognise — the hook behaves exactly as it does today. A memory tool that
needs a third-party account to work would not be a memory tool.

**What would leave your machine.** The tail of the agent's reply, and the name
and excerpt of each candidate node — pieces of your vault, over HTTPS, to a
company that is not you. Decide that per vault, not per habit. Vault text is
always sent as data (`state`), never as part of the question
(`instructions`), so a node whose body says "ignore the above" cannot change
what is being asked; a test pins that.

**Unverified:** the corpus this hook was tuned on is mostly Russian, and
TypeSafe does not document non-English input. Measuring that is step one, not
an afterthought.

## Design notes

- **No transcript parsing.** Both harnesses pass everything needed as
  documented hook fields (`tool_input`, `tool_response`,
  `last_assistant_message`, `stop_hook_active`, `prompt`). Transcript formats
  are internal and differ between the two.
- **The hook's own search is not a read.** It goes to the daemon over HTTP,
  not through the harness, so it never fires `PostToolUse` and never opens
  its own gate. Only the agent's reads count. It does reach the daemon like
  any other call, though, so a usage audit built from daemon logs will see
  it: it identifies itself as `clientInfo: kaeru-first` — filter on that.
- **It closes every session it opens.** The daemon runs with idle reaping off
  on purpose (a five-minute reaper used to kill editor sessions during
  ordinary pauses), so a session nobody closes lives until the daemon
  restarts. Open, search and close share one six-second budget, inside the
  harness's ten, and the close runs in a `finally`.
- **The human's reply is classified, never stored.** Only its outcome and its
  length reach the log — someone answering a question sometimes pastes the
  key the agent asked for. And a reply is only classified when it follows an
  ask: the markers are loose on purpose, and *"we decided on postgres last
  week, now write the migration"* is a task, not a complaint.
- **Blocks once per turn per reason, never loops.** `Stop` honours
  `stop_hook_active`. An agent that searched, read, and still needs to ask,
  asks.
- **Fails open.** Unreadable input, an unwritable state directory, a dead
  daemon, an unknown event: exit 0, no output. A hook must never break the
  harness.
- State is one small JSON file per session in `$XDG_STATE_HOME/kaeru-first`
  (or `~/.local/state/kaeru-first`), pruned after a week; the decision log
  lives beside it.

## Install

Needs `python3` (stdlib only). Copy the scripts somewhere stable:

```sh
mkdir -p ~/.local/share/kaeru-first
cp -R kaeru_first.py kaeru_first_report.py lexicon ~/.local/share/kaeru-first/
chmod +x ~/.local/share/kaeru-first/*.py
```

### Claude Code — `~/.claude/settings.json`

```json
{
  "hooks": {
    "PostToolUse": [
      { "matcher": "mcp__kaeru__.*",
        "hooks": [{ "type": "command", "command": "~/.local/share/kaeru-first/kaeru_first.py", "timeout": 10 }] }
    ],
    "PreToolUse": [
      { "matcher": "AskUserQuestion",
        "hooks": [{ "type": "command", "command": "~/.local/share/kaeru-first/kaeru_first.py", "timeout": 10 }] }
    ],
    "Stop": [
      { "hooks": [{ "type": "command", "command": "~/.local/share/kaeru-first/kaeru_first.py", "timeout": 10 }] }
    ],
    "UserPromptSubmit": [
      { "hooks": [{ "type": "command", "command": "~/.local/share/kaeru-first/kaeru_first.py", "timeout": 10 }] }
    ]
  }
}
```

### Codex — `~/.codex/config.toml`

```toml
[[hooks.PostToolUse]]
matcher = "^mcp__kaeru__"
[[hooks.PostToolUse.hooks]]
type = "command"
command = "~/.local/share/kaeru-first/kaeru_first.py"
timeout = 10

[[hooks.Stop]]
[[hooks.Stop.hooks]]
type = "command"
command = "~/.local/share/kaeru-first/kaeru_first.py"
timeout = 10

[[hooks.UserPromptSubmit]]
[[hooks.UserPromptSubmit.hooks]]
type = "command"
command = "~/.local/share/kaeru-first/kaeru_first.py"
timeout = 10
```

Hooks in Codex are listed among its feature flags (`codex_hooks`). If the
hooks above never fire on your version, enable them:

```toml
[features]
codex_hooks = true
```

## Tuning

| variable | default | meaning |
|---|---|---|
| `KAERU_FIRST_URL` | `http://127.0.0.1:9876/mcp` | the daemon's MCP endpoint the gate searches |
| `KAERU_FIRST_TOKEN` | — | bearer token, when the daemon has one |
| `KAERU_FIRST_SEARCH` | `1` | `0` disables the evidence search and restores the first version's timer gate |
| `KAERU_FIRST_WINDOW` | `600` | seconds a kaeru read stays "recent" for the relevant-read and unread-hits rules |
| `KAERU_FIRST_LIMIT` | `5` | hits to ask the daemon for; three are shown |
| `KAERU_FIRST_LEXICON_DIR` | — | an extra directory of `*.json` lexicons, merged with the built-in English one |
| `KAERU_FIRST_STATE_DIR` | `$XDG_STATE_HOME/kaeru-first` | where per-session state and `decisions.jsonl` live |

## Tests

```sh
python3 -m unittest -v
```

Each scenario runs the script as a harness does — a subprocess fed one JSON
event — against a scratch state directory and a fake kaeru: a small MCP
server that answers `search` with hits for the terms a test declares known,
and `(no matches)` otherwise.
