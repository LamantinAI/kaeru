#!/usr/bin/env python3
"""kaeru-first — summarise the decision log.

Reads `decisions.jsonl` from the state directory (or a path given as the first
argument) and answers the question the hook exists to answer: when it blocked,
did the agent go and read; when it let a question through, did the human say
"it was in memory"?

    python3 kaeru_first_report.py            # default state dir
    python3 kaeru_first_report.py path/to/decisions.jsonl
"""

from __future__ import annotations

import collections
import json
import os
import sys
from pathlib import Path


def state_dir() -> Path:
    explicit = os.environ.get("KAERU_FIRST_STATE_DIR")
    if explicit:
        return Path(explicit)
    base = os.environ.get("XDG_STATE_HOME") or str(Path.home() / ".local" / "state")
    return Path(base) / "kaeru-first"


def main(argv: list[str]) -> int:
    path = Path(argv[1]) if len(argv) > 1 else state_dir() / "decisions.jsonl"
    if not path.exists():
        print(f"no log at {path}")
        return 0
    rows = [json.loads(ln) for ln in path.read_text(encoding="utf-8").splitlines() if ln.strip()]
    asks = [r for r in rows if r.get("event") in ("PreToolUse", "Stop")]
    replies = [r for r in rows if r.get("event") == "UserPromptSubmit"]
    print(f"{len(asks)} ask(s) gated, {len(replies)} human reply(ies) classified, "
          f"{len({r.get('session') for r in rows})} session(s)\n")

    print("by shape:")
    for k, n in collections.Counter(r.get("shape") for r in asks).most_common():
        print(f"  {str(k):11s} {n:5d}")
    print("\nby outcome:")
    for k, n in collections.Counter(r.get("reason") for r in asks).most_common():
        print(f"  {str(k):30s} {n:5d}")

    denies = [r for r in asks if r.get("decision") == "deny"]
    print(f"\ndenied: {len(denies)} of {len(asks)} ({100 * len(denies) // max(1, len(asks))}%)")

    # After a deny, what did the human eventually say?
    after = collections.Counter()
    for r in replies:
        after[(r.get("after_decision"), r.get("outcome"))] += 1
    print("\nhuman reply after the gate's decision:")
    for (dec, out), n in sorted(after.items(), key=lambda x: -x[1]):
        print(f"  after {str(dec):5s} → {str(out):14s} {n:5d}")

    misses = [r for r in replies if r.get("outcome") == "miss"]
    passed_then_miss = [r for r in misses if r.get("after_decision") == "pass"]
    print(f"\nmisses (human said it was in memory): {len(misses)}; of those the gate had let through: {len(passed_then_miss)}")
    for k, n in collections.Counter(r.get("after_reason") for r in passed_then_miss).most_common():
        print(f"  let through because {str(k):28s} {n:4d}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
