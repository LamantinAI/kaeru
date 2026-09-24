#!/usr/bin/env python3
"""A semantic judge for kaeru-first — optional, off, and not wired in yet (#100).

The hook has three decisions that are questions about *meaning*, and it makes
all three lexically: is this an ask, is it procedural, and does a hit answer
it. The third one was measured and failed — no lexical rule separated the 49
asks the human answered with "it's in kaeru" from the 300 he did not (lift
1.0–1.2, precision ~4%), so the hook stopped judging and shows the top three
hits instead. That is honest and it costs a turn every time.

This module is the *candidate* replacement: TypeSafe's Jev, a decision model
that returns a calibrated probability for a yes/no question rather than text.
It is here so the question "does it actually beat the lexicon?" can be
answered with a number, by `eval_judge.py`, on a corpus, before a single line
of the hook changes. If it does not beat the lexicon, that is a second
negative result and this file stays unused.

**Nothing here runs unless someone opts in.** The judge is active only when
`KAERU_FIRST_JUDGE=jev` and `TYPESAFE_API_KEY` are both set. Absent either, or
on any error or timeout, every entry point returns `None` and the caller keeps
its existing behaviour.

**What leaves the machine when it is on.** The tail of the agent's reply and
the *name and excerpt* of each candidate node — that is, pieces of the user's
memory — go to a third-party API over HTTPS. Say so at opt-in, and never
enable it on a vault whose contents may not leave the building. Vault text is
always passed as `state`, never as `instructions`: the question is ours, the
content is data, and a node whose body says "ignore your instructions" must
not be able to change what is being asked.
"""

from __future__ import annotations

import json
import os
import urllib.error
import urllib.request

# The vendor's own endpoint. Several third-party pages document a
# `jevmodel.org` host with its own key name; it is not TypeSafe's, and a hook
# that carries somebody's API key has no business pointing anywhere but the
# service that issued it.
DEFAULT_ENDPOINT = "https://api.typesafe.ai/v1/systemone"
DEFAULT_MODEL = "jev-latest"

# The whole exchange has to fit inside the hook's own budget, which is already
# spending time on a kaeru search. Jev's published latency is 70–500 ms.
TIMEOUT_SECONDS = 3.0

# One call answers up to this many questions in parallel, so the hit list is
# capped rather than chunked: past twenty candidates the ask is too vague for
# the answer to be in any of them.
MAX_QUESTIONS = 20


def enabled() -> bool:
    """Both the switch and the key, or nothing happens."""
    return os.environ.get("KAERU_FIRST_JUDGE", "").lower() == "jev" and bool(
        os.environ.get("TYPESAFE_API_KEY")
    )


class Jev:
    """A minimal Jev client: one POST, typed answers, no text generation."""

    def __init__(self, endpoint: str | None = None, key: str | None = None) -> None:
        self.endpoint = endpoint or os.environ.get(
            "KAERU_FIRST_JUDGE_URL", DEFAULT_ENDPOINT
        )
        self.key = key or os.environ.get("TYPESAFE_API_KEY", "")
        self.model = os.environ.get("KAERU_FIRST_JUDGE_MODEL", DEFAULT_MODEL)

    def ask(self, state: str, questions: dict[str, dict]) -> dict | None:
        """Answers `questions` about `state`. `None` means "could not ask"."""
        if not self.key or not questions:
            return None
        body = json.dumps(
            {"state": state, "model": self.model, "questions": questions}
        ).encode("utf-8")
        req = urllib.request.Request(
            self.endpoint,
            data=body,
            headers={
                "Content-Type": "application/json",
                "Authorization": f"Bearer {self.key}",
            },
            method="POST",
        )
        try:
            with urllib.request.urlopen(req, timeout=TIMEOUT_SECONDS) as resp:
                return json.loads(resp.read().decode("utf-8", "replace"))
        except (urllib.error.URLError, OSError, ValueError):
            return None

    def nouls(self, state: str, questions: dict[str, str]) -> dict[str, float] | None:
        """`{key: instruction}` → `{key: probability}`, or `None` on failure."""
        typed = {
            key: {
                "type": "noul",
                "instructions": instruction,
                "criteria": {"true": "yes", "false": "no"},
            }
            for key, instruction in list(questions.items())[:MAX_QUESTIONS]
        }
        answer = self.ask(state, typed)
        if not isinstance(answer, dict):
            return None
        return probabilities(answer, typed.keys())


def probabilities(answer: dict, keys) -> dict[str, float] | None:
    """Pulls a probability per question out of a response.

    Deliberately forgiving about the envelope: the published shapes differ
    between the vendor's own docs and the SDKs, and a judge that cannot read
    one field of an answer should fall back to the lexicon, not crash a hook.
    """
    answers = answer.get("answers") or answer.get("questions") or answer
    if not isinstance(answers, dict):
        return None
    out: dict[str, float] = {}
    for key in keys:
        value = answers.get(key)
        if isinstance(value, (int, float)):
            out[key] = float(value)
        elif isinstance(value, dict):
            for field in ("probability", "p", "true", "value", "score"):
                candidate = value.get(field)
                if isinstance(candidate, (int, float)):
                    out[key] = float(candidate)
                    break
    return out or None


# ------------------------------------------------------------ the questions --
#
# One phrasing per decision, kept here rather than at the call sites so the
# eval measures what the hook would actually ask.

ANSWERS_THE_ASK = (
    "The state holds a question an AI agent is about to put to its human, and "
    "one note from the human's own long-term memory. Would reading this note "
    "give the agent the answer, so that asking is unnecessary?"
)

IS_A_KNOWLEDGE_ASK = (
    "The state holds the closing lines of an AI agent's reply to its human. Is "
    "the agent asking the human for a FACT or a DECISION that could already be "
    "written down somewhere — as opposed to asking permission to act, asking "
    "for a go-ahead, or telling the human what to do next?"
)

IS_DOUBTING = (
    "The state holds the closing lines of an AI agent's reply. Is the agent "
    "acting on a guess about a specific fact — hedging, assuming, saying "
    "'probably' or 'I think' about something that could be looked up — rather "
    "than stating what it knows or has checked?"
)


def state_for_hit(ask: str, name: str, excerpt: str) -> str:
    """The `state` for one relevance question.

    Both halves are labelled and both are content, never instruction. The
    question lives in `instructions`, where the vault cannot reach it.
    """
    return json.dumps(
        {"agent_is_asking": ask, "note": {"name": name, "excerpt": excerpt}},
        ensure_ascii=False,
    )


def relevance(jev: Jev, ask: str, hits: list[tuple[str, str]]) -> dict[str, float] | None:
    """`{node name: probability it answers the ask}` for up to `MAX_QUESTIONS` hits.

    One call per hit's state is what the API shape costs us: questions in a
    request share one state, and each hit is a different state. Batched by
    hand would mean one state holding every note, which reads as a single
    blob and loses which note answered.
    """
    if not hits:
        return None
    out: dict[str, float] = {}
    for name, excerpt in hits[:MAX_QUESTIONS]:
        answer = jev.nouls(
            state_for_hit(ask, name, excerpt), {"answers": ANSWERS_THE_ASK}
        )
        if answer is None:
            return None
        out[name] = answer.get("answers", 0.0)
    return out


def shape_of_ask(jev: Jev, tail: str) -> dict[str, float] | None:
    """The two judgements about the agent's own reply, in one call.

    `knowledge_ask` replaces what `is_procedural` decides lexically;
    `doubting` is the one the hook cannot see at all today — the agent that
    does not ask, guesses, and acts on the guess.
    """
    return jev.nouls(
        json.dumps({"agent_reply_tail": tail}, ensure_ascii=False),
        {"knowledge_ask": IS_A_KNOWLEDGE_ASK, "doubting": IS_DOUBTING},
    )
