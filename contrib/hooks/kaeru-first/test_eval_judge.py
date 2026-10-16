"""Offline regressions for pairing journal questions with their actual replies."""

from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from eval_judge import from_journal


def ask(question="Which database?", session="s", turn="t", **fields):
    return {
        "event": "Stop", "session": session, "turn": turn,
        "shape": "question", "q": question, "hits": ["database-choice"],
        **fields,
    }


def reply(outcome="answered", session="s", **fields):
    return {
        "event": "UserPromptSubmit", "session": session,
        "after_shape": "question", "outcome": outcome, **fields,
    }


class JournalCase(unittest.TestCase):
    def corpus(self, *rows):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "decisions.jsonl"
            path.write_text("\n".join(json.dumps(row) for row in rows), encoding="utf-8")
            return from_journal(path)

    def test_a_later_unasked_miss_does_not_relabel_an_answered_question(self):
        rows = self.corpus(ask(), reply(), reply("unasked:miss", after_shape=None))
        self.assertEqual(rows, [{
            "ask": "Which database?",
            "hits": [{"name": "database-choice", "excerpt": ""}],
            "label": "other",
        }])

    def test_a_question_is_consumed_by_its_first_reply(self):
        rows = self.corpus(ask(), reply("miss"), reply())
        self.assertEqual(len(rows), 1)
        self.assertEqual(rows[0]["label"], "miss")

    def test_reusing_a_turn_key_keeps_the_last_question_in_log_order(self):
        rows = self.corpus(
            ask("Old question?", turn="a"),
            ask("Intermediate question?", turn="b"),
            ask("Actual question?", turn="a"),
            reply("miss"),
        )
        self.assertEqual([row["ask"] for row in rows], ["Actual question?"])

    def test_interleaved_sessions_have_independent_pending_questions(self):
        rows = self.corpus(
            ask("First session?", session="one"),
            ask("Second session?", session="two", event="PreToolUse", shape="tool"),
            reply("miss", session="one"),
            reply(session="two", after_shape="tool"),
        )
        self.assertEqual([(row["ask"], row["label"]) for row in rows], [
            ("First session?", "miss"), ("Second session?", "other"),
        ])

    def test_an_unpaired_reply_clears_stale_questions_instead_of_leaving_them_for_later(self):
        for fields in (
            {"after_shape": None},
            {"after_shape": "tool"},
            {"outcome": "unasked:miss"},
            {"outcome": "unknown"},
        ):
            with self.subTest(fields=fields):
                self.assertEqual(self.corpus(ask(), reply(**fields), reply("miss")), [])

    def test_a_loop_guard_stop_without_a_turn_id_replaces_the_pending_question(self):
        guard = ask("Loop guard question?", reason="stop_hook_active")
        guard.pop("turn")
        rows = self.corpus(ask("Earlier question?"), guard, reply())
        self.assertEqual([row["ask"] for row in rows], ["Loop guard question?"])

    def test_other_recorded_outcomes_keep_their_non_miss_label(self):
        for outcome in ("answered", "complaint", "capture_nudge"):
            with self.subTest(outcome=outcome):
                self.assertEqual(self.corpus(ask(), reply(outcome))[0]["label"], "other")

    def test_a_new_empty_question_and_missing_sessions_cannot_reuse_an_older_ask(self):
        self.assertEqual(self.corpus(ask(), ask(""), reply("miss")), [])
        self.assertEqual(self.corpus(ask(session=None), reply("miss", session=None)), [])


if __name__ == "__main__":
    unittest.main()
