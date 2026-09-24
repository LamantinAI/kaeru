#!/usr/bin/env python3
"""Does a semantic judge beat the lexicon? Measure it before believing it (#100).

The hook's relevance gate was already built once, lexically, and measured: no
rule separated the asks the human answered with "it's in kaeru" from the ones
he did not. Every rule caught the misses about as often as it blocked
everything else — lift 1.0–1.2, precision about 4% — and that table is in the
README so nobody builds the same gate a second time.

A semantic judge is a different method, not a better hunch, and it earns its
place the same way: on the same corpus, against the same baseline, with the
same table. This script is that comparison. It changes nothing in the hook.

## The corpus

One JSON object per line:

    {"ask": "<the agent's asking text>",
     "hits": [{"name": "...", "excerpt": "..."}, ...],
     "label": "miss" | "other"}

`miss` means the human's reply to that ask said the answer was already in
memory. `other` is any ask he answered normally. The hook's own
`decisions.jsonl` carries the asks, the terms and the hit names it showed, and
its `UserPromptSubmit` rows carry the outcome — but it does not store hit
excerpts, so a corpus built from it alone judges on names. Pass
`--from-journal` to build the thin version and see how far that gets.

## Running it

    python3 eval_judge.py corpus.jsonl                    # lexical baseline only
    KAERU_FIRST_JUDGE=jev TYPESAFE_API_KEY=... \
        python3 eval_judge.py corpus.jsonl --judge        # and the judge

The second form sends the asks and the hit excerpts to a third-party API.
That is somebody's memory leaving their machine: run it on a corpus you are
allowed to send, and never on a vault by reflex.

## The bar

From the issue: **lift ≥ 2 and precision well clear of the lexical ~4%.**
Under that, the honest outcome is a second negative result, recorded next to
the first.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

import judge as judge_mod
from kaeru_first import name_matches, stem, terms_of


def load(path: Path) -> list[dict]:
    rows = []
    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            rows.append(json.loads(line))
        except ValueError:
            continue
    return rows


def from_journal(path: Path) -> list[dict]:
    """A thin corpus out of `decisions.jsonl`: asks, hit NAMES, and outcomes.

    The journal pairs an ask with the reply that followed it, so the label is
    whatever the reply was classified as. Excerpts are not stored, so every
    hit here has an empty one — which is itself worth measuring: if the judge
    does well on names alone, the hook never has to send bodies.
    """
    rows = load(path)
    asks: dict[str, dict] = {}
    out: list[dict] = []
    for row in rows:
        if row.get("event") in ("Stop", "PreToolUse"):
            key = (row.get("session"), row.get("turn"))
            asks[key] = row
        elif row.get("event") == "UserPromptSubmit":
            outcome = str(row.get("outcome") or "")
            # The journal does not carry the turn id on the reply row, so the
            # ask is the last one logged for that session.
            ask = next(
                (
                    a
                    for (session, _), a in reversed(list(asks.items()))
                    if session == row.get("session")
                ),
                None,
            )
            if not ask or not ask.get("q"):
                continue
            out.append(
                {
                    "ask": ask.get("q", ""),
                    "hits": [{"name": n, "excerpt": ""} for n in ask.get("hits", [])],
                    "label": "miss" if outcome.endswith("miss") else "other",
                }
            )
    return out


# --------------------------------------------------------------- baselines --


def lexical_says_relevant(ask: str, hits: list[dict]) -> bool:
    """The best of the lexical rules from #91: a question term in a hit's NAME.

    It caught 83% of the misses — and blocked 71% of everything else, which is
    why it is a baseline rather than a gate.
    """
    stems = [stem(t) for t in terms_of(ask)]
    return any(name_matches((h.get("name", ""), h.get("excerpt", "")), stems) for h in hits)


def judge_says_relevant(jev: judge_mod.Jev, ask: str, hits: list[dict], threshold: float):
    pairs = [(h.get("name", ""), h.get("excerpt", "")) for h in hits]
    scored = judge_mod.relevance(jev, ask, pairs)
    if scored is None:
        return None
    return max(scored.values(), default=0.0) >= threshold


# ----------------------------------------------------------------- scoring --


def report(name: str, rows: list[dict], decision) -> None:
    """The #91 table for one rule: catch, block, lift, precision."""
    misses = [r for r in rows if r.get("label") == "miss"]
    others = [r for r in rows if r.get("label") != "miss"]
    unavailable = 0

    caught = 0
    for r in misses:
        verdict = decision(r)
        if verdict is None:
            unavailable += 1
        elif verdict:
            caught += 1
    blocked = 0
    for r in others:
        verdict = decision(r)
        if verdict is None:
            unavailable += 1
        elif verdict:
            blocked += 1

    catch_rate = caught / len(misses) if misses else 0.0
    block_rate = blocked / len(others) if others else 0.0
    # Lift: how much likelier a flagged ask is to be a real miss than an
    # unflagged one would be. 1.0 means the rule knows nothing.
    base = len(misses) / max(1, len(misses) + len(others))
    flagged = caught + blocked
    precision = caught / flagged if flagged else 0.0
    lift = (precision / base) if base else 0.0

    print(f"{name:28s} catch {catch_rate:5.0%}  block {block_rate:5.0%}  "
          f"precision {precision:5.1%}  lift {lift:4.2f}")
    if unavailable:
        print(f"{'':28s} ({unavailable} judgement(s) unavailable — counted as no)")


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("corpus", type=Path)
    parser.add_argument("--from-journal", action="store_true",
                        help="read a kaeru-first decisions.jsonl instead of a labelled corpus")
    parser.add_argument("--judge", action="store_true",
                        help="also run the semantic judge (sends data to a third party)")
    parser.add_argument("--threshold", type=float, default=0.5,
                        help="probability at or above which the judge says 'answers it'")
    args = parser.parse_args(argv[1:])

    rows = from_journal(args.corpus) if args.from_journal else load(args.corpus)
    misses = sum(1 for r in rows if r.get("label") == "miss")
    print(f"{len(rows)} ask(s): {misses} miss, {len(rows) - misses} other\n")
    if not rows:
        print("nothing to measure")
        return 0

    report("lexical: term in a name", rows,
           lambda r: lexical_says_relevant(r.get("ask", ""), r.get("hits", [])))
    report("lexical: any hit at all", rows, lambda r: bool(r.get("hits")))

    if args.judge:
        if not judge_mod.enabled():
            print("\n--judge needs KAERU_FIRST_JUDGE=jev and TYPESAFE_API_KEY")
            return 2
        jev = judge_mod.Jev()
        print(f"\nsending {len(rows)} ask(s) and their hit excerpts to "
              f"{jev.endpoint} — this is memory leaving the machine")
        report(f"jev: p >= {args.threshold}", rows,
               lambda r: judge_says_relevant(jev, r.get("ask", ""), r.get("hits", []),
                                             args.threshold))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
