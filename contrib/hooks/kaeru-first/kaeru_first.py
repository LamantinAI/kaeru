#!/usr/bin/env python3
"""kaeru-first — a harness hook for the moment an agent is about to ask the human.

Usage audit 4 (#89) found that the agent reads kaeru almost always, and almost
never at the moment it asks: 80% of its questions to the user came with no
kaeru read in the previous ten minutes. The first version of this hook gated
that moment on a timer — "no kaeru read in the window → deny once".

Usage audit 5 changed two things about it.

First, **the moment is wider than a question mark.** Over 2,608 turns where the
agent stopped and waited for the human, the detector below recognises 1,164
plain-text asks. The first version's `Stop` could see 201 of them — a reply
whose last line ends in `?`. The largest class it missed, 856, was an
imperative hand-off: "say «fix it»", "I need your answer about the provider",
"send me the report". The user's "it's in kaeru" replies landed on the
invisible shapes far more often than on the visible ones. So `Stop` now looks
at the whole tail of the reply, not its last character. (Two shapes were
counted at first and then dropped or narrowed — the enumerated list, and a
bare "waiting" — see `asking_shape` and the README: both turned out to be
reporting, not asking. Every count is an upper bound: the corpus is "turns the
human replied to", and the human replies to every final turn eventually.)

Second, **a timer is the wrong gate for a wider net, and so is a lexical one.**
Half of those hand-offs are procedural ("write «done»") and memory cannot
answer them; a timer would have blocked every other turn. The obvious
replacement — search kaeru for the question's terms and block only when memory
"has evidence" — was built and measured, and does not work: no lexical rule
separates the asks the human answered with "it's in kaeru" from the ones he
did not (see `rank_hits`). Relevance is semantic, and the hook cannot see it.

So the hook does the part it can do and leaves the judgement to the agent. It
skips what is procedural. It passes when the agent already read memory on the
same terms — relevance, not recency. Otherwise it runs one unscoped prefix
`search` itself and blocks once with the top hits, names first: the agent
glances at three excerpts and either reads one or asks. The block costs a
turn, not a round of tool calls. Nothing found, or the daemon down — pass.

Events, and what each does:

  PostToolUse   on a kaeru READ verb: remember when, what verb, and which
                terms it consulted; for `search`, which names came back.
  PreToolUse    on AskUserQuestion (Claude Code only — Codex asks in plain
                text): run the gate on the question text.
  Stop          if the reply asks in any recognised shape, run the gate on the
                asking lines. Honours `stop_hook_active`: block once, never
                loop.
  UserPromptSubmit
                if the previous turn asked: classify the human's reply. A
                substantive answer gets the capture reminder. A reply saying
                the answer was in memory is logged as a miss and gets a
                stronger one.

Every decision is appended to `decisions.jsonl` in the state directory, so
whether the gate earns its keep is a `jq` question, not a guess —
`kaeru_first_report.py` summarises it.

It never parses a transcript. It fails open: any error, an unreachable
daemon, an unknown event — exit 0, no output. A hook that breaks the harness
is worse than no hook.

Environment:

  KAERU_FIRST_URL        the daemon's MCP endpoint (default
                         http://127.0.0.1:9876/mcp).
  KAERU_FIRST_TOKEN      bearer token, when the daemon has one.
  KAERU_FIRST_SEARCH     `0` disables the evidence search and restores the
                         timer gate of the first version.
  KAERU_FIRST_WINDOW     seconds a kaeru read stays "recent" (default 600).
  KAERU_FIRST_LIMIT      hits to ask the daemon for (default 5, shows 3).
  KAERU_FIRST_LEXICON_DIR  an extra directory of `*.json` lexicons, merged with
                         the built-in English one and any `lexicon/` beside
                         the script — see README, "Lexicons".
  KAERU_FIRST_STATE_DIR  where per-session state and the decision log live
                         (default $XDG_STATE_HOME/kaeru-first, else
                         ~/.local/state/kaeru-first).
"""

from __future__ import annotations

import hashlib
import json
import os
import re
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path

# --------------------------------------------------------------- verbs ------

# kaeru verbs that CONSULT memory. `initiatives` is left out on purpose: it
# lists scopes, it does not read what is in them.
READ_VERBS = frozenset(
    {
        "at", "awake", "between", "board", "board_status", "cloud_initiatives",
        "cloud_links", "cloud_recall", "drill", "history", "ideas", "lint",
        "neighbours", "outcomes", "overview", "path", "recall", "recent",
        "reflect", "search", "slots", "surface", "tagged", "trace", "why",
    }
)
# Verbs that read ONE node in full — what "actually reading a hit" means.
NODE_READ_VERBS = frozenset({"at", "drill", "neighbours", "why", "trace", "history"})

KAERU_TOOL = re.compile(r"^mcp__(?:plugin_[^_]+_)?kaeru__(?P<verb>[a-z_]+)$")

SUBSTANTIAL_ANSWER = 30
STATE_TTL = 7 * 24 * 3600
SHOWN_HITS = 3

# ------------------------------------------------------------- messages -----

RECIPE = (
    "Before you ask the user, check kaeru — the answer may already be in memory.\n"
    "1. `search` WITHOUT `initiative`: initiatives fragment, and a scoped search "
    "misses a node that lives in a sibling scope.\n"
    "2. Put a prefix wildcard on each entity in your question (`certif*`, "
    "`deploy*`), and search in the language the notes were written in.\n"
    "3. If the question is \"what next\", that is `awake` + `board`, not a "
    "question for the user.\n"
    "4. `at <name>` on any hit to read it in full.\n"
    "If nothing turns up, ask — and say in one line what you searched, so the "
    "user knows memory was tried first."
)

CAPTURE = (
    "The user just answered a question you asked. By construction that answer "
    "was not in kaeru — capture it now: `episode` (or `cite` for a settled "
    "fact) with the `initiative`, and `link` it to what it is about. Rules "
    "spoken in the dialogue and never captured are the most frequent class of "
    "interruption in the usage audits."
)

CAPTURE_MISS = (
    "The user says the answer to your last question was already in memory. "
    "Before anything else: `search` for it WITHOUT `initiative`, `at` the hit, "
    "and use it. Then capture where it lives under the initiative you are "
    "working in, so the next agent finds it without being told."
)


def hits_message(hits: list[tuple[str, str]], terms: list[str]) -> str:
    shown = hits[:SHOWN_HITS]
    lines = [
        "Before you ask the user — kaeru has nodes that may bear on this.",
        f"Searched `{' OR '.join(t + '*' for t in terms)}` without `initiative`; "
        f"top {len(shown)} of {len(hits)}:",
    ]
    for name, excerpt in shown:
        lines.append(f"  - `{name}`" + (f" — {excerpt}" if excerpt else ""))
    lines.append(
        "If one of them answers your question, `at <name>` and use it. If none "
        "does, ask — and say in one line that memory was checked. The hook cannot "
        "judge relevance; you can."
    )
    return "\n".join(lines)


def unread_message(names: list[str]) -> str:
    shown = ", ".join(f"`{n}`" for n in names[:SHOWN_HITS])
    more = f" and {len(names) - SHOWN_HITS} more" if len(names) > SHOWN_HITS else ""
    return (
        f"Your last `search` returned {len(names)} hit(s) and you read none of "
        f"them: {shown}{more}. `at` one before you ask the user — a search you "
        "do not read is not a search."
    )


# --------------------------------------------------------------- state ------


def state_dir() -> Path:
    explicit = os.environ.get("KAERU_FIRST_STATE_DIR")
    if explicit:
        return Path(explicit)
    base = os.environ.get("XDG_STATE_HOME") or str(Path.home() / ".local" / "state")
    return Path(base) / "kaeru-first"


def state_path(session_id: str) -> Path:
    safe = re.sub(r"[^A-Za-z0-9_.-]", "_", session_id) or "unknown"
    return state_dir() / f"{safe}.json"


def load_state(session_id: str) -> dict:
    try:
        return json.loads(state_path(session_id).read_text())
    except (OSError, ValueError):
        return {}


def save_state(session_id: str, state: dict) -> None:
    directory = state_dir()
    directory.mkdir(parents=True, exist_ok=True)
    state["updated"] = time.time()
    path = state_path(session_id)
    tmp = path.with_suffix(".tmp")
    tmp.write_text(json.dumps(state, ensure_ascii=False))
    tmp.replace(path)
    prune(directory)


def prune(directory: Path) -> None:
    cutoff = time.time() - STATE_TTL
    try:
        for f in directory.glob("*.json"):
            if f.stat().st_mtime < cutoff:
                f.unlink(missing_ok=True)
    except OSError:
        pass


def log_decision(record: dict) -> None:
    """One line per decision. Best-effort; the log must never fail the hook."""
    try:
        directory = state_dir()
        directory.mkdir(parents=True, exist_ok=True)
        record = {"ts": round(time.time(), 3), **record}
        with (directory / "decisions.jsonl").open("a", encoding="utf-8") as fh:
            fh.write(json.dumps(record, ensure_ascii=False) + "\n")
    except OSError:
        pass


# ------------------------------------------------------------- settings -----


def window() -> float:
    try:
        return float(os.environ.get("KAERU_FIRST_WINDOW", "600"))
    except ValueError:
        return 600.0


def search_enabled() -> bool:
    return os.environ.get("KAERU_FIRST_SEARCH", "1") not in ("0", "false", "no", "off")


def search_limit() -> int:
    try:
        return max(1, int(os.environ.get("KAERU_FIRST_LIMIT", "5")))
    except ValueError:
        return 5


def turn_key(event: dict, fallback: str = "") -> str:
    # Claude Code names a turn `prompt_id`, Codex `turn_id`.
    key = str(event.get("turn_id") or event.get("prompt_id") or "")
    if key:
        return key
    return "q:" + hashlib.sha1(fallback.encode("utf-8", "ignore")).hexdigest()[:12]


# ---------------------------------------------------------------- text ------


# ------------------------------------------------------------- lexicon ------
#
# Everything below that depends on a human language is DATA, not code. The
# script carries an English lexicon so it works as a single file; any
# `lexicon/*.json` beside it (and in $KAERU_FIRST_LEXICON_DIR) is merged in,
# so a vault written in another language needs a file, not a fork. Each key is
# a list — of words for `stop`, of regex fragments for the rest — and the
# languages are simply alternated together.

EN = {
    # Function words plus the procedural vocabulary an ask is made of. None of
    # it says WHAT is being asked, only that something is — and in a vault of
    # hundreds of nodes a term like "start" hits everything.
    "stop": """
    the this that these those with from into onto about after before
    should would could which what when where whether there their they them
    then than also just only still want need make made take next
    step option options please lets start run now here some more most
    very have has had will does doing done your yours ours mine
    sure okay ready first last other another same each both either neither
    every any all none something anything the and for are but not
    you can her was one our out day get has him his
    how its may new now old see two way who did let
    put say she too use yes
    ask asks asked tell told say said give gave send sent know knew think
    like look looks see seen show shown put get got goes going come came
    once while until since because though although however whatever whichever
    it's i'll i'm i've you'll we'll that's there's let's don't doesn't didn't
    can't won't isn't aren't wasn't shouldn't couldn't wouldn't
    file files folder repo repository branch commit push merge script command
    code test tests version release project task tasks work list result results
    change changes current new old
    """.split(),
    # The agent is waiting for a go-word, not for knowledge: say "ship it".
    "quoted_go": ["say", "write", "reply", "type", "answer", "respond with", "the word"],
    # A short yes/no-shaped stem. Only exempts when the whole line is short.
    "confirm_stem": [
        "go ahead", "proceed", "start", "ship", "merge", "push", "commit", "run it", "shall i",
        "should i", "deploy", "delete", "continue", "apply", "open the pr", "create the pr", "ok to",
    ],
    # Option labels that make a question a confirmation, not a lookup.
    "yesno_label": [
        r"yes\b", r"no\b", r"ok\b", "okay", r"go\b", r"wait\b", r"skip\b", r"later\b", "not now",
        "proceed", "continue", "hold off", r"first\b", "leave it", "don't",
    ],
    # How an ask shows up in a plain-text reply (the Stop path).
    # Addressed forms only. A bare "confirm" also matches "I can confirm the
    # tests pass", and a bare "decide" matches "you can decide later" — both
    # statements, both caught by an earlier revision of this list.
    "imperative": [
        "let me know", "tell me", "your call", "which one", "please confirm", "can you confirm",
        "could you confirm", "confirm whether", "confirm which", "please choose", "please pick",
        "please decide", "you decide", "waiting for your", "waiting on you", "need your", "over to you",
        "up to you", "send me", "give me", "share the", "paste the", "point me to",
    ],
    # "I can …", "I could …" and "Happy to …" open as many reports as offers
    # ("I could not reproduce it", "Happy to report the build is fixed"), so an
    # offer has to say it is one.
    "offer": [
        "want me to", "shall i", "if you want", "if you'd like", "would you like", "say the word",
        "i can also", "i could also", r"happy to (?:do|take|run|write|fix|add|draft|dig|look|help)",
    ],
    # A line that opens like this hands the human an ACTION — "Next: run the
    # tests", "Next step: open the PR" — not a question. Memory cannot open a
    # PR for anyone. Agents that close every reply with a next action (a common
    # house style) would otherwise be blocked on every single turn.
    "next_step": [
        r"next\s*:", "next step", "next action", r"now\s*:", r"then\s*:", "to do now", r"your move\s*:",
        r"action\s*:", r"do this\s*:",
    ],
    # A word that turns a confirmation into a choice.
    "alternative": [r"or\b", "versus", r"vs\b", "either"],
    # What the human says when the agent should have looked first.
    "miss": [
        r"(?:check|look in|search|read|it'?s in|that'?s in) (?:kaeru|memory|your notes)",
        r"already (?:told|said|decided|discussed|gave|sent|did)",
        r"i (?:already )?(?:told|gave|sent|showed) you",
        r"we (?:already |have already )?(?:did|discussed|decided|set (?:this|it|that) up|fixed)",
        r"as i said", r"like i said",
    ],
    "complaint": [r"use (?:kaeru|memory|your memory)", "stop asking", "how many times", r"you keep (?:asking|forgetting)"],
    "capture": [
        r"(?:write|put|save|record|store|note) .{0,25}(?:kaeru|memory)", "remember this", r"save (?:this|it)\b",
        r"record (?:this|it)\b", r"did you (?:write|save|record|capture)",
    ],
}


def load_lexicons() -> dict[str, list[str]]:
    lex = {key: list(values) for key, values in EN.items()}
    dirs = [Path(__file__).resolve().parent / "lexicon"]
    if os.environ.get("KAERU_FIRST_LEXICON_DIR"):
        dirs.append(Path(os.environ["KAERU_FIRST_LEXICON_DIR"]))
    for directory in dirs:
        try:
            files = sorted(directory.glob("*.json")) if directory.is_dir() else []
        except OSError:
            files = []
        for f in files:
            try:
                data = json.loads(f.read_text(encoding="utf-8"))
                if not isinstance(data, dict):
                    continue
                extra = {key: [str(v) for v in values if str(v).strip()]
                         for key, values in data.items() if key in lex and isinstance(values, list)}
                for key, values in extra.items():
                    if key != "stop":
                        for fragment in values:
                            re.compile(fragment)      # one bad fragment disqualifies ITS file…
            except (OSError, ValueError, re.error) as exc:
                print(f"kaeru-first: lexicon {f.name} skipped: {exc}", file=sys.stderr)
                continue                              # …and only its file
            for key, values in extra.items():
                lex[key].extend(values)
    return lex


def _alt(parts: list[str]) -> str:
    return "(?:" + "|".join(parts) + ")"


def build(lex: dict[str, list[str]]) -> dict:
    flags = re.I
    return {
        "stop": frozenset(w.lower() for w in lex["stop"]),
        "quoted_go": re.compile(_alt(lex["quoted_go"]) + r"\s*[«\"“'‹]", flags),
        "confirm_stem": re.compile(r"^(?:\*\*)?" + _alt(lex["confirm_stem"]) + r"\b", flags),
        "yesno_label": re.compile(r"^(?:\*\*)?" + _alt(lex["yesno_label"]), flags),
        "imperative": re.compile(r"\b" + _alt(lex["imperative"]) + r"\b", flags),
        "offer": re.compile(r"^(?:[-*•>]\s*)?(?:\*\*)?" + _alt(lex["offer"]), flags),
        "alternative": re.compile(r"\b" + _alt(lex["alternative"]), flags),
        "next_step": re.compile(r"^[\s>#*_\-•\d.)]*" + _alt(lex["next_step"]), flags),
        "miss": re.compile(_alt(lex["miss"]), flags),
        "complaint": re.compile(_alt(lex["complaint"]), flags),
        "capture": re.compile(_alt(lex["capture"]), flags),
    }


try:
    LEX = build(load_lexicons())
except re.error:
    # Files are validated one by one above; this is the belt to those braces.
    LEX = build({key: list(values) for key, values in EN.items()})

STOP = LEX["stop"]
QUOTED_GO, CONFIRM_STEM, YESNO_LABEL = LEX["quoted_go"], LEX["confirm_stem"], LEX["yesno_label"]
IMPERATIVE, OFFER, ALTERNATIVE, NEXT_STEP = LEX["imperative"], LEX["offer"], LEX["alternative"], LEX["next_step"]
MISS_MARKERS, COMPLAINT_MARKERS, CAPTURE_MARKERS = LEX["miss"], LEX["complaint"], LEX["capture"]
QMARK = ("?", "？")


def strip_md(line: str) -> str:
    return line.rstrip("*_`)\"'»”’ ").strip()


TOKEN = re.compile(r"[^\W_][\w-]*|[.!?:;\n]")


def scan_terms(text: str) -> dict[str, bool]:
    """Every searchable term in `text`, flagged by whether it looks like an entity.

    An entity is a token with a digit in it (`k8s`), a hyphenated name, a Latin
    token inside prose written in another script, or a word capitalised
    anywhere but the start of a sentence.
    """
    mixed = any(ord(c) > 127 and c.isalpha() for c in text)
    entity: dict[str, bool] = {}
    sentence_start = True
    for m in TOKEN.finditer(text):
        tok = m.group(0)
        if tok in ".!?:;\n":
            sentence_start = True
            continue
        w = tok.lower().strip("_-")
        # Three characters is enough for an ASCII token — api, vpn, ssh — while
        # in most other scripts a word that short is a function word.
        short_ok = w.isascii() or any(c.isdigit() for c in w)
        if len(w) >= (3 if short_ok else 4) and w not in STOP and not w.isdigit():
            looks_like_one = (
                any(c.isdigit() for c in w) or "-" in w or (mixed and w.isascii())
                or (tok[0].isupper() and not sentence_start)
            )
            entity[w] = entity.get(w, False) or looks_like_one
        sentence_start = False
    return entity


def has_entity(text: str) -> bool:
    return any(scan_terms(text).values())


def terms_of(text: str, cap: int = 8) -> list[str]:
    """The terms worth searching for — entities first, then by length.

    Length is a poor proxy for what matters: in "once you tell me about the
    key, I'll run generation and bring the second column" the word the human's
    answer turned on is the shortest one. So what looks like an entity goes
    first, and only then length.
    """
    entity = scan_terms(text)
    return sorted(entity, key=lambda w: (not entity[w], -len(w), w))[:cap]


def ends_in_question(text: str | None) -> bool:
    if not text:
        return False
    lines = [ln.strip() for ln in text.splitlines() if ln.strip()]
    return bool(lines) and strip_md(lines[-1]).endswith(QMARK)


# Text inside quotes or a code span is MENTIONED, not used. A report about this
# very hook — "the list now holds only addressed forms such as «please
# confirm»" — quotes an imperative without being one, and a line that ends in
# a quoted question is not a question.
MENTION = re.compile(r"«[^»\n]*»|“[^”\n]*”|\"[^\"\n]*\"|`[^`\n]*`")


def used(line: str) -> str:
    return MENTION.sub(" ", line)


NUMBERED = re.compile(r"^\s*(?:\d+[.)]|\w[.)])\s")


def with_choice_context(tail: list[str], idx: int) -> str:
    """The question line, plus the numbered list it refers to when one sits right above it.

    "Which do we take?" names nothing — every word of it is a stopword. The
    subject is in the list above and the line that introduces it, so those are
    what gets searched. Only numbered lines count: bold bullets are how a
    status report is formatted, not how a choice is offered.
    """
    j = idx
    while j - 1 >= 0 and NUMBERED.match(tail[j - 1]):
        j -= 1
    if idx - j >= 2:
        return " ".join(tail[max(0, j - 1): idx + 1])
    return tail[idx]


def asking_shape(text: str | None) -> tuple[str, str] | None:
    """Does this reply hand the turn to the human, and how?

    Returns (shape, asking_text) or None. Looks at the last eight non-empty
    lines: the question mark anywhere, an imperative addressed to the user,
    or an offer waiting for a yes.
    """
    if not text:
        return None
    lines = [ln.strip() for ln in text.splitlines() if ln.strip()]
    if not lines:
        return None
    tail = lines[-8:]
    if strip_md(used(tail[-1])).endswith(QMARK):
        return "q_last", with_choice_context(tail, len(tail) - 1)
    q_idx = [i for i, ln in enumerate(tail) if strip_md(used(ln)).endswith(QMARK)]
    if q_idx:
        return "q_any", " ".join(with_choice_context(tail, i) for i in q_idx)
    # "Next: open a session and give it the brief" tells the human what to DO;
    # it asks for nothing memory could hold. Only a question mark overrides that.
    imp = [ln for ln in tail if IMPERATIVE.search(used(ln)) and not NEXT_STEP.match(ln)]
    if imp:
        return "imperative", " ".join(imp)
    off = [ln for ln in tail if OFFER.search(used(ln)) and not NEXT_STEP.match(ln)]
    if off:
        return "offer", " ".join(off)
    # An enumerated list is NOT a shape of its own. It was tried: over the same
    # corpus, 133 replies matched "two option-like lines plus a choice cue", and
    # on inspection none of a sample of twenty was a question memory could
    # answer — they were status reports with bold bullets and "next: do X"
    # hand-offs. A real choice comes with a question mark or an imperative and
    # is caught above; what is left over is formatting.
    return None


def is_procedural(question: str, options: list[str] | None = None) -> str | None:
    """A go-word, a yes/no, a bare confirmation — memory has no answer to these."""
    q = question.strip()
    if QUOTED_GO.search(q):
        return "quoted_go"
    if options is not None and 0 < len(options) <= 3:
        if any(YESNO_LABEL.match(o.strip()) for o in options):
            return "yesno_options"
    first = strip_md(q.splitlines()[0]) if q else ""
    # "Ship it?" is a confirmation. "Should I use the staging token or the prod
    # one for the Acme deploy?" starts the same way and is exactly the question
    # memory tends to answer — so the stem exempts only a short line that
    # offers no alternative and names no entity.
    if len(first) <= 90 and CONFIRM_STEM.match(first) and not ALTERNATIVE.search(first) and not has_entity(q):
        return "confirm_stem"
    return None


# --------------------------------------------------------------- kaeru ------


class Kaeru:
    """A minimal MCP client over urllib: open a session, search once, CLOSE it.

    Closing is not optional. The daemon runs with idle reaping off on purpose —
    a five-minute reaper used to kill editor sessions during ordinary pauses —
    so a session nobody closes lives until the daemon restarts. A hook that
    opens one per gated ask and walks away leaks thousands of them.
    """

    # The harness gives the hook ten seconds. The whole exchange — open,
    # search, close — has to fit inside that with room to spare, or the hook
    # is killed mid-flight and the session it opened is never closed.
    BUDGET = 6.0

    def __init__(self) -> None:
        self.url = os.environ.get("KAERU_FIRST_URL", "http://127.0.0.1:9876/mcp")
        self.token = os.environ.get("KAERU_FIRST_TOKEN")
        self.deadline = time.monotonic() + self.BUDGET

    def _request(self, method: str, body: dict | None = None, sid: str | None = None,
                 timeout: float | None = None) -> tuple[str | None, str]:
        headers = {
            "Content-Type": "application/json",
            "Accept": "application/json, text/event-stream",
        }
        if sid:
            headers["Mcp-Session-Id"] = sid
        if self.token:
            headers["Authorization"] = f"Bearer {self.token}"
        if timeout is None:
            timeout = max(0.2, min(3.0, self.deadline - time.monotonic()))
        data = json.dumps(body).encode() if body is not None else None
        req = urllib.request.Request(self.url, data=data, headers=headers, method=method)
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            new_sid = resp.headers.get("Mcp-Session-Id") or resp.headers.get("mcp-session-id")
            return new_sid, resp.read().decode("utf-8", "replace")

    def _close(self, sid: str | None) -> None:
        """End the session. Best-effort, own short timeout, never raises."""
        if not sid:
            return
        try:
            self._request("DELETE", sid=sid, timeout=1.5)
        except Exception:  # noqa: BLE001 — closing must not fail the hook
            pass

    @staticmethod
    def _result_text(raw: str) -> str:
        """The tool's text out of a plain-JSON or SSE response."""
        payloads = []
        stripped = raw.strip()
        if stripped.startswith("{"):
            payloads.append(stripped)
        for line in raw.splitlines():
            if line.startswith("data:"):
                payloads.append(line[5:].strip())
        for p in payloads:
            try:
                d = json.loads(p)
            except ValueError:
                continue
            if isinstance(d, dict) and "result" in d:
                content = d["result"].get("content") or []
                return "".join(c.get("text", "") for c in content if isinstance(c, dict))
        return ""

    def search(self, terms: list[str], limit: int) -> list[tuple[str, str]] | None:
        """Unscoped prefix search. None means the daemon could not be asked."""
        if not terms:
            return []
        return self.query(" OR ".join(f"{t}*" for t in terms), limit)

    def query(self, query: str, limit: int) -> list[tuple[str, str]] | None:
        sid = None
        try:
            sid, _ = self._request("POST", {
                "jsonrpc": "2.0", "id": 1, "method": "initialize",
                "params": {"protocolVersion": "2025-06-18", "capabilities": {},
                           "clientInfo": {"name": "kaeru-first", "version": "2"}},
            })
            self._request("POST", {"jsonrpc": "2.0", "method": "notifications/initialized"}, sid)
            _, raw = self._request("POST", {
                "jsonrpc": "2.0", "id": 2, "method": "tools/call",
                "params": {"name": "search", "arguments": {"query": query, "limit": limit}},
            }, sid)
        except (urllib.error.URLError, OSError, ValueError):
            return None
        finally:
            self._close(sid)   # whatever happened above — success, error, timeout
        return parse_hits(self._result_text(raw))


HIT_LINE = re.compile(r"^\s{2}- (?P<name>.+?) \((?P<type>[a-z_]+)\) — ")


def parse_hits(text: str) -> list[tuple[str, str]]:
    """`matches (N):` blocks → [(name, excerpt)]. A miss or anything odd → []."""
    hits: list[tuple[str, str]] = []
    lines = text.splitlines()
    for i, line in enumerate(lines):
        m = HIT_LINE.match(line)
        if not m:
            continue
        excerpt = ""
        if i + 1 < len(lines) and lines[i + 1].startswith("    ") and not HIT_LINE.match(lines[i + 1]):
            excerpt = lines[i + 1].strip()
            if len(excerpt) > 140:
                excerpt = excerpt[:137].rstrip() + "…"
        hits.append((m.group("name"), excerpt))
    return hits


def stem(term: str) -> str:
    """Trim an inflection, so a term still matches the same word in another case or number."""
    return term[: max(5, len(term) - 2)]


def name_matches(hit: tuple[str, str], stems: list[str]) -> int:
    name = hit[0].lower()
    return sum(1 for st in stems if st in name)


def rank_hits(hits: list[tuple[str, str]], terms: list[str]) -> list[tuple[str, str]]:
    """Order hits by how much of the question they echo — a name match first.

    This ranks; it does not judge. Every lexical rule for deciding "memory has
    the answer" was tried against 49 asks the human answered with "it's in
    kaeru" and 300 he did not, and none separates them: two terms in name and
    excerpt catches 24% of the first and blocks 22% of the second; one term in
    the node NAME, 83% and 71%; an `AND` of the two longest terms over the full
    body, 22% and 26%. Lift of about one, everywhere. Whether memory answers a
    question is semantic, the vault is dense (98% of asks hit something), and
    the entity that matters is often the shortest word in the ask. So the hook
    shows the candidates and leaves relevance to the agent, who can tell.
    """
    stems = [stem(t) for t in terms]

    def key(hit: tuple[str, str]) -> tuple[int, int]:
        blob = f"{hit[0]} {hit[1]}".lower()
        return (-name_matches(hit, stems), -sum(1 for st in stems if st in blob))

    return sorted(hits, key=key)


# ---------------------------------------------------------------- gate ------


def recent_reads(state: dict) -> list[dict]:
    cutoff = time.time() - window()
    reads = [r for r in state.get("reads", []) if isinstance(r, dict) and r.get("t", 0) >= cutoff]
    state["reads"] = reads[-40:]
    return state["reads"]


def relevant_read(state: dict, terms: list[str]) -> dict | None:
    """A recent read whose terms overlap the question's — the agent already looked."""
    if not terms:
        return None
    want = {t[:5] for t in terms}  # stem-ish, so an inflected form still overlaps
    for r in reversed(recent_reads(state)):
        have = {t[:5] for t in r.get("terms", [])} | {t[:5] for t in terms_of(" ".join(r.get("hits", [])))}
        if want & have:
            return r
    return None


def unread_hits(state: dict) -> list[str]:
    """Names the last recent search returned, none of which this SESSION has read.

    The search has to be recent — that is what makes it "the search you just
    ran". The reading does not: a node read two hours ago is still in the
    agent's context, and asking for it to be read again is nagging.
    """
    reads = recent_reads(state)
    last = next((r for r in reversed(reads) if r.get("verb") == "search"), None)
    names = list((last or {}).get("hits", []))
    if not names:
        return []
    if set(names) & set(state.get("read_names", [])):
        return []
    return names


def denied(state: dict, turn: str, reason: str) -> bool:
    key = f"{turn}:{reason}"
    log = state.setdefault("denied", [])
    if key in log:
        return True
    log.append(key)
    del log[:-40]
    return False


def gate(state: dict, event: str, turn: str, shape: str, question: str, options: list[str] | None) -> tuple[str | None, str | None, dict]:
    """(decision, message, log_fields). decision ∈ {None (pass), 'deny'}."""
    terms = terms_of(question)
    fields: dict = {"shape": shape, "terms": terms, "q": question.strip()[:160]}

    why = is_procedural(question, options)
    if why:
        fields.update(decision="pass", reason=f"exempt:{why}")
        return None, None, fields
    if not terms:
        fields.update(decision="pass", reason="exempt:no_terms")
        return None, None, fields

    r = relevant_read(state, terms)
    if r:
        fields.update(decision="pass", reason="relevant_read", read_verb=r.get("verb"))
        return None, None, fields

    pending = unread_hits(state)
    if pending:
        if denied(state, turn, "hits_unread"):
            fields.update(decision="pass", reason="already_denied:hits_unread")
            return None, None, fields
        fields.update(decision="deny", reason="hits_unread", hits=pending[:SHOWN_HITS])
        return "deny", unread_message(pending), fields

    if not search_enabled():
        # The first version's rule: any recent read opens the gate.
        if recent_reads(state):
            fields.update(decision="pass", reason="timer:recent_read")
            return None, None, fields
        if denied(state, turn, "no_recent_read"):
            fields.update(decision="pass", reason="already_denied:no_recent_read")
            return None, None, fields
        fields.update(decision="deny", reason="timer:no_recent_read")
        return "deny", RECIPE, fields

    raw = Kaeru().search(terms, search_limit())
    if raw is None:
        fields.update(decision="pass", reason="daemon_unreachable")
        return None, None, fields
    if not raw:
        fields.update(decision="pass", reason="no_hits", raw_hits=0)
        return None, None, fields
    hits = rank_hits(raw, terms)
    stems = [stem(t) for t in terms]
    fields["raw_hits"] = len(raw)
    fields["name_hits"] = sum(1 for h in hits if name_matches(h, stems))
    fields["hits"] = [h[0] for h in hits[:SHOWN_HITS]]
    if denied(state, turn, "hits_shown"):
        fields.update(decision="pass", reason="already_denied:hits_shown")
        return None, None, fields
    fields.update(decision="deny", reason="hits_shown")
    return "deny", hits_message(hits, terms), fields


# ------------------------------------------------------------- outputs ------


def deny_tool(reason: str) -> dict:
    return {"hookSpecificOutput": {"hookEventName": "PreToolUse", "permissionDecision": "deny", "permissionDecisionReason": reason}}


def add_context(text: str) -> dict:
    return {"hookSpecificOutput": {"hookEventName": "UserPromptSubmit", "additionalContext": text}}


# -------------------------------------------------------------- events ------


def on_post_tool_use(event: dict, state: dict, session: str) -> dict | None:
    match = KAERU_TOOL.match(event.get("tool_name") or "")
    if not match or match.group("verb") not in READ_VERBS:
        return None
    verb = match.group("verb")
    inp = event.get("tool_input") if isinstance(event.get("tool_input"), dict) else {}
    consulted = " ".join(str(inp.get(k, "")) for k in ("query", "name", "a", "b", "from", "to", "tag") if inp.get(k))
    record = {"t": time.time(), "verb": verb, "terms": terms_of(consulted), "name": str(inp.get("name") or "")[:120]}
    if verb == "search":
        resp = event.get("tool_response")
        text = ""
        if isinstance(resp, str):
            text = resp
        elif isinstance(resp, dict):
            text = resp.get("text") or "".join(c.get("text", "") for c in resp.get("content", []) if isinstance(c, dict))
        elif isinstance(resp, list):
            text = "".join(c.get("text", "") for c in resp if isinstance(c, dict))
        record["hits"] = [h[0] for h in parse_hits(text)][:10]
    state.setdefault("reads", []).append(record)
    state["last_read"] = record["t"]
    if verb in NODE_READ_VERBS and record["name"]:
        seen = state.setdefault("read_names", [])
        if record["name"] not in seen:
            seen.append(record["name"])
            del seen[:-500]
    return None


def on_pre_tool_use(event: dict, state: dict, session: str) -> dict | None:
    if event.get("tool_name") != "AskUserQuestion":
        return None
    inp = event.get("tool_input") if isinstance(event.get("tool_input"), dict) else {}
    questions = inp.get("questions") if isinstance(inp.get("questions"), list) else []
    texts, options = [], []
    for q in questions:
        if isinstance(q, dict):
            texts.append(str(q.get("question", "")))
            options.extend(str(o.get("label", "")) for o in (q.get("options") or []) if isinstance(o, dict))
    question = " ".join(texts).strip()
    turn = turn_key(event, question)
    decision, message, fields = gate(state, "PreToolUse", turn, "tool", question, options or None)
    state["last_ask"] = {"t": time.time(), "shape": "tool", "q": question[:200], "decision": fields.get("decision"), "reason": fields.get("reason")}
    log_decision({"session": session, "event": "PreToolUse", "turn": turn, **fields})
    return deny_tool(message) if decision == "deny" else None


def on_stop(event: dict, state: dict, session: str) -> dict | None:
    text = event.get("last_assistant_message")
    found = asking_shape(text)
    # Recorded on every Stop so the next prompt knows whether it answers an ask.
    state["last_reply_question"] = bool(found)
    if not found:
        state.pop("last_ask", None)
        return None
    shape, asking = found
    if event.get("stop_hook_active"):
        # Already continued once this turn — the loop guard. Log it, let it through.
        state["last_ask"] = {"t": time.time(), "shape": shape, "q": asking[:200], "decision": "pass", "reason": "stop_hook_active"}
        log_decision({"session": session, "event": "Stop", "shape": shape, "decision": "pass", "reason": "stop_hook_active", "q": asking[:160]})
        return None
    turn = turn_key(event, asking)
    decision, message, fields = gate(state, "Stop", turn, shape, asking, None)
    state["last_ask"] = {"t": time.time(), "shape": shape, "q": asking[:200], "decision": fields.get("decision"), "reason": fields.get("reason")}
    log_decision({"session": session, "event": "Stop", "turn": turn, **fields})
    return {"decision": "block", "reason": message} if decision == "deny" else None


def on_user_prompt_submit(event: dict, state: dict, session: str) -> dict | None:
    last_ask = state.pop("last_ask", None)
    state["last_reply_question"] = False
    prompt = (event.get("prompt") or "").strip()
    # A reply is only a reply if something was asked. Without that, "we decided
    # on postgres last week, now write the migration" reads as "it was in
    # memory" — the markers are loose on purpose, and only an ask gives them
    # something to be about.
    if not prompt or not last_ask:
        return None
    if MISS_MARKERS.search(prompt):
        outcome = "miss"
    elif COMPLAINT_MARKERS.search(prompt):
        outcome = "complaint"
    elif CAPTURE_MARKERS.search(prompt):
        outcome = "capture_nudge"
    else:
        outcome = "answered"
    # The reply's text is NOT logged: a human answering a question sometimes
    # pastes the key the agent asked for. Its length is enough to measure with.
    log_decision({
        "session": session, "event": "UserPromptSubmit",
        "after_shape": last_ask.get("shape"), "after_decision": last_ask.get("decision"),
        "after_reason": last_ask.get("reason"), "outcome": outcome, "reply_len": len(prompt),
    })
    if outcome in ("miss", "complaint"):
        return add_context(CAPTURE_MISS)
    # A go-word after "say «ship it»" is not knowledge, and neither is whatever
    # the human says next after a hand-off the gate itself called procedural.
    procedural = str(last_ask.get("reason") or "").startswith("exempt:")
    if outcome != "answered" or procedural or len(prompt) < SUBSTANTIAL_ANSWER:
        return None
    return add_context(CAPTURE)


HANDLERS = {
    "PostToolUse": on_post_tool_use,
    "PreToolUse": on_pre_tool_use,
    "Stop": on_stop,
    "UserPromptSubmit": on_user_prompt_submit,
}


def main() -> int:
    try:
        event = json.loads(sys.stdin.read() or "{}")
        handler = HANDLERS.get(event.get("hook_event_name", ""))
        session = event.get("session_id")
        if handler is None or not session:
            return 0
        state = load_state(session)
        out = handler(event, state, str(session))
        save_state(str(session), state)
        if out is not None:
            sys.stdout.write(json.dumps(out, ensure_ascii=False))
    except Exception as exc:  # noqa: BLE001 — fail open, always
        print(f"kaeru-first: {exc}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
