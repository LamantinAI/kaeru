"""kaeru-first — tests.

Each scenario runs the script the way a harness does: a subprocess fed one
JSON event on stdin, against a scratch state directory. The evidence gate
talks to a fake kaeru: a tiny MCP server that answers `search` with hits for
the terms a test declares as "known", and `(no matches)` otherwise.
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
import threading
import unittest
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

SCRIPT = Path(__file__).with_name("kaeru_first.py")
sys.path.insert(0, str(SCRIPT.parent))
import kaeru_first as kf  # noqa: E402

# A port nothing listens on — "daemon down".
DEAD_URL = "http://127.0.0.1:9/mcp"


class FakeKaeru(BaseHTTPRequestHandler):
    """The three MCP requests the hook makes, and a canned `search`."""

    known: set[str] = set()
    calls: list[str] = []
    opened: list[str] = []     # session ids handed out by `initialize`
    closed: list[str] = []     # session ids the client ended with DELETE
    fail_call: bool = False    # answer the search itself with a 500
    echo_terms: bool = True   # excerpt repeats the query terms, so a known-term question scores

    def log_message(self, *_):  # silence
        pass

    def do_DELETE(self):
        FakeKaeru.closed.append(self.headers.get("Mcp-Session-Id") or "")
        self.send_response(202)
        self.send_header("Content-Length", "0")
        self.end_headers()

    def do_POST(self):
        n = int(self.headers.get("Content-Length") or 0)
        body = json.loads(self.rfile.read(n) or b"{}")
        method = body.get("method")
        if method == "initialize":
            out = {"jsonrpc": "2.0", "id": body.get("id"), "result": {"protocolVersion": "2025-06-18", "capabilities": {}}}
            data = json.dumps(out).encode()
            sid = f"fake-session-{len(FakeKaeru.opened) + 1}"
            FakeKaeru.opened.append(sid)
            self.send_response(200)
            self.send_header("Mcp-Session-Id", sid)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(data)))
            self.end_headers()
            self.wfile.write(data)
            return
        if method == "notifications/initialized":
            self.send_response(202)
            self.send_header("Content-Length", "0")
            self.end_headers()
            return
        query = (body.get("params") or {}).get("arguments", {}).get("query", "")
        FakeKaeru.calls.append(query)
        if FakeKaeru.fail_call:
            self.send_response(500)
            self.send_header("Content-Length", "0")
            self.end_headers()
            return
        terms = [t.rstrip("*") for t in query.split(" OR ")]
        if any(t.startswith(k) or k.startswith(t) for t in terms for k in FakeKaeru.known):
            echo = (" · " + " ".join(terms)) if FakeKaeru.echo_terms else ""
            text = (
                "matches (2):\n"
                "  - provider-switch-note (episode) — 01a0aaaa\n"
                f"    we moved to acme-cloud, the setup guide lives here{echo}\n"
                "  - second-note (reference) — 01a0bbbb\n"
                "    one more excerpt\n\n"
                "↳ excerpts only — read one in full with `at <name>`."
            )
        else:
            text = "(no matches)\n↳ widen it: `search \"x*\"` (prefix match)."
        out = {"jsonrpc": "2.0", "id": body.get("id"), "result": {"content": [{"type": "text", "text": text}]}}
        data = json.dumps(out, ensure_ascii=False).encode()
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)


class HookCase(unittest.TestCase):
    server: ThreadingHTTPServer
    url: str

    @classmethod
    def setUpClass(cls):
        cls.server = ThreadingHTTPServer(("127.0.0.1", 0), FakeKaeru)
        cls.url = f"http://127.0.0.1:{cls.server.server_address[1]}/mcp"
        threading.Thread(target=cls.server.serve_forever, daemon=True).start()

    @classmethod
    def tearDownClass(cls):
        cls.server.shutdown()

    def setUp(self):
        self.state = tempfile.mkdtemp(prefix="kaeru-first-test-")
        FakeKaeru.known = set()
        FakeKaeru.calls = []
        FakeKaeru.echo_terms = True
        FakeKaeru.opened, FakeKaeru.closed, FakeKaeru.fail_call = [], [], False
        self.env_extra: dict[str, str] = {}

    def run_hook(self, event: dict, url: str | None = None, **env_over) -> dict | None:
        env = dict(os.environ)
        env.pop("KAERU_FIRST_SEARCH", None)
        env["KAERU_FIRST_STATE_DIR"] = self.state
        env["KAERU_FIRST_URL"] = url or self.url
        env.update(self.env_extra)
        env.update(env_over)
        proc = subprocess.run(
            [sys.executable, str(SCRIPT)], input=json.dumps(event, ensure_ascii=False),
            capture_output=True, text=True, env=env, timeout=30,
        )
        self.assertEqual(proc.returncode, 0, proc.stderr)
        return json.loads(proc.stdout) if proc.stdout.strip() else None

    def decisions(self) -> list[dict]:
        p = Path(self.state) / "decisions.jsonl"
        if not p.exists():
            return []
        return [json.loads(line) for line in p.read_text().splitlines() if line.strip()]

    # ----- event builders -----

    @staticmethod
    def ask(question: str, options: list[str] | None = None, session="s1", prompt_id="turn-1") -> dict:
        q: dict = {"question": question, "header": "h"}
        if options:
            q["options"] = [{"label": o, "description": ""} for o in options]
        return {"hook_event_name": "PreToolUse", "tool_name": "AskUserQuestion", "session_id": session,
                "prompt_id": prompt_id, "tool_input": {"questions": [q]}}

    @staticmethod
    def read(verb: str, session="s1", **inp) -> dict:
        resp = inp.pop("_response", None)
        ev = {"hook_event_name": "PostToolUse", "tool_name": f"mcp__kaeru__{verb}", "session_id": session, "tool_input": inp}
        if resp is not None:
            ev["tool_response"] = resp
        return ev

    @staticmethod
    def stop(text: str, session="s1", active=False, prompt_id="turn-1") -> dict:
        return {"hook_event_name": "Stop", "session_id": session, "last_assistant_message": text,
                "stop_hook_active": active, "prompt_id": prompt_id}

    @staticmethod
    def prompt(text: str, session="s1") -> dict:
        return {"hook_event_name": "UserPromptSubmit", "session_id": session, "prompt": text}


def denied_reason(out: dict | None) -> str:
    return (out or {}).get("hookSpecificOutput", {}).get("permissionDecisionReason", "") or (out or {}).get("reason", "")


# ----------------------------------------------------------------- fail open


class FailOpenTests(HookCase):
    def test_garbage_input_exits_zero_silently(self):
        proc = subprocess.run([sys.executable, str(SCRIPT)], input="not json", capture_output=True, text=True,
                              env={**os.environ, "KAERU_FIRST_STATE_DIR": self.state})
        self.assertEqual(proc.returncode, 0)
        self.assertEqual(proc.stdout, "")

    def test_an_unknown_event_is_ignored(self):
        self.assertIsNone(self.run_hook({"hook_event_name": "Whatever", "session_id": "s1"}))

    def test_an_unwritable_state_dir_fails_open(self):
        self.state = "/proc/definitely/not/writable"
        FakeKaeru.known = {"provider"}
        # It cannot persist, but it must not crash; a deny is still fine, a crash is not.
        self.run_hook(self.ask("Which provider are we on?"))

    def test_a_dead_daemon_lets_the_question_through(self):
        out = self.run_hook(self.ask("Which provider are we on?"), url=DEAD_URL)
        self.assertIsNone(out)
        self.assertEqual(self.decisions()[-1]["reason"], "daemon_unreachable")


# --------------------------------------------------------- what counts as read


class ReadTests(HookCase):
    def read_state(self) -> dict:
        return json.loads((Path(self.state) / "s1.json").read_text())

    def test_a_search_records_its_terms_and_hits(self):
        self.run_hook(self.read("search", query="provider* OR plan*", _response={"content": [{"type": "text", "text":
            "matches (1):\n  - provider-switch-note (episode) — 01a0\n    excerpt\n"}]}))
        r = self.read_state()["reads"][-1]
        self.assertEqual(r["verb"], "search")
        self.assertIn("provider", r["terms"])
        self.assertEqual(r["hits"], ["provider-switch-note"])

    def test_a_write_is_not_a_read(self):
        self.run_hook(self.read("episode", name="x", body="y"))
        self.assertNotIn("reads", self.read_state())

    def test_initiatives_alone_is_not_a_read(self):
        self.run_hook(self.read("initiatives"))
        self.assertNotIn("reads", self.read_state())

    def test_at_records_the_node_name(self):
        self.run_hook(self.read("at", name="provider-switch-note"))
        self.assertEqual(self.read_state()["reads"][-1]["name"], "provider-switch-note")


# ------------------------------------------------------------- the exemptions


class ProceduralTests(HookCase):
    """Memory has no answer to a go-word or a yes/no — never even ask it."""

    def test_a_quoted_go_word_passes_without_a_search(self):
        FakeKaeru.known = {"provider"}
        out = self.run_hook(self.ask("Say «fix the provider» and I will assemble the stage."))
        self.assertIsNone(out)
        self.assertEqual(self.decisions()[-1]["reason"], "exempt:quoted_go")
        self.assertEqual(FakeKaeru.calls, [])

    def test_yes_no_options_pass(self):
        FakeKaeru.known = {"provider"}
        out = self.run_hook(self.ask("Switch the provider to the backup right away?", ["Yes, switch", "No, later"]))
        self.assertIsNone(out)
        self.assertEqual(self.decisions()[-1]["reason"], "exempt:yesno_options")

    def test_a_bare_confirmation_stem_passes(self):
        out = self.run_hook(self.stop("Everything is in.\n\nShip it?"))
        self.assertIsNone(out)
        self.assertEqual(self.decisions()[-1]["reason"], "exempt:confirm_stem")

    def test_a_stem_does_not_exempt_a_question_with_an_alternative_or_an_entity(self):
        FakeKaeru.known = {"token", "migration"}
        for q in ("Should I use the staging token or the prod one for the Acme deploy?",
                  "Start with the ledger migration or with the cloud auth first?",
                  "Should I rotate the token for Acme?"):
            FakeKaeru.calls.clear()
            self.run_hook(self.ask(q, prompt_id=q))
            self.assertEqual(self.decisions()[-1]["reason"], "hits_shown", q)
            self.assertTrue(FakeKaeru.calls, q)

    def test_a_plain_confirmation_is_still_exempt(self):
        for q in ("Should I push to staging?", "Ship it?", "Go ahead with the merge?"):
            self.assertIsNone(self.run_hook(self.ask(q, prompt_id=q)))
            self.assertEqual(self.decisions()[-1]["reason"], "exempt:confirm_stem", q)

    def test_a_question_with_no_entities_passes(self):
        out = self.run_hook(self.ask("What next?"))
        self.assertIsNone(out)
        self.assertEqual(self.decisions()[-1]["reason"], "exempt:no_terms")
        self.assertEqual(FakeKaeru.calls, [])

    def test_choice_options_are_not_a_confirmation(self):
        FakeKaeru.known = {"provider"}
        out = self.run_hook(self.ask("Which plan do we take with the provider?", ["Basic", "Pro", "Enterprise"]))
        self.assertIsNotNone(out)
        self.assertEqual(self.decisions()[-1]["reason"], "hits_shown")


# --------------------------------------------------------------- showing hits


class ShowHitsTests(HookCase):
    def test_a_question_memory_has_something_on_is_denied_with_the_hits(self):
        FakeKaeru.known = {"provider"}
        out = self.run_hook(self.ask("Which provider are we on, and which plan?"))
        reason = denied_reason(out)
        self.assertIn("provider-switch-note", reason)
        self.assertIn("acme-cloud", reason)
        self.assertIn("`at <name>`", reason)
        self.assertIn("cannot judge relevance", reason)
        self.assertEqual(self.decisions()[-1]["reason"], "hits_shown")

    def test_a_question_memory_has_nothing_on_passes(self):
        out = self.run_hook(self.ask("Which provider are we on, and which plan?"))
        self.assertIsNone(out)
        self.assertEqual(self.decisions()[-1]["reason"], "no_hits")
        self.assertTrue(FakeKaeru.calls)

    def test_the_search_is_unscoped_and_prefixed(self):
        self.run_hook(self.ask("Which provider are we on, and which plan?"))
        self.assertEqual(FakeKaeru.calls[-1], "provider* OR plan*")

    def test_denied_once_per_turn(self):
        FakeKaeru.known = {"provider"}
        self.assertIsNotNone(self.run_hook(self.ask("Which provider are we on?", prompt_id="t7")))
        self.assertIsNone(self.run_hook(self.ask("Which provider are we on?", prompt_id="t7")))
        self.assertIsNotNone(self.run_hook(self.ask("Which provider are we on?", prompt_id="t8")))

    def test_a_relevant_read_opens_the_gate(self):
        FakeKaeru.known = {"provider"}
        self.run_hook(self.read("search", query="provider*", _response="(no matches)"))
        out = self.run_hook(self.ask("Which provider are we on?"))
        self.assertIsNone(out)
        self.assertEqual(self.decisions()[-1]["reason"], "relevant_read")

    def test_an_irrelevant_read_does_not(self):
        FakeKaeru.known = {"provider"}
        self.run_hook(self.read("search", query="linux* OR glibc*", _response="(no matches)"))
        out = self.run_hook(self.ask("Which provider are we on?"))
        self.assertIsNotNone(out)
        self.assertEqual(self.decisions()[-1]["reason"], "hits_shown")

    def test_hits_the_agent_never_read_are_named(self):
        self.run_hook(self.read("search", query="provider*", _response=
            "matches (2):\n  - provider-switch-note (episode) — 01a0\n    excerpt\n  - second-note (reference) — 01a1\n    more\n"))
        # A different question, so the relevant-read shortcut does not apply.
        out = self.run_hook(self.ask("Which certificate belongs on the server?"))
        reason = denied_reason(out)
        self.assertIn("read none of them", reason)
        self.assertIn("provider-switch-note", reason)
        self.assertEqual(self.decisions()[-1]["reason"], "hits_unread")

    def test_reading_a_hit_clears_it(self):
        self.run_hook(self.read("search", query="provider*", _response=
            "matches (1):\n  - provider-switch-note (episode) — 01a0\n    excerpt\n"))
        self.run_hook(self.read("at", name="provider-switch-note"))
        out = self.run_hook(self.ask("Which certificate belongs on the server?"))
        self.assertIsNone(out)
        self.assertEqual(self.decisions()[-1]["reason"], "no_hits")

    def test_a_node_read_hours_ago_is_still_read(self):
        """The search must be recent; the reading need not be — it is still in context."""
        self.run_hook(self.read("at", name="provider-switch-note"))
        state_file = Path(self.state) / "s1.json"
        state = json.loads(state_file.read_text())
        for r in state["reads"]:
            r["t"] -= 7200                       # two hours ago: outside the window
        state_file.write_text(json.dumps(state))
        self.run_hook(self.read("search", query="provider*", _response=
            "matches (1):\n  - provider-switch-note (episode) — 01a0\n    excerpt\n"))
        out = self.run_hook(self.ask("Which certificate belongs on the server?"))
        self.assertIsNone(out)
        self.assertEqual(self.decisions()[-1]["reason"], "no_hits")

    def test_the_hook_shows_hits_it_cannot_vouch_for(self):
        """It ranks, it does not judge: a loose hit is still shown, and logged as loose."""
        FakeKaeru.known = {"screen"}
        FakeKaeru.echo_terms = False
        out = self.run_hook(self.ask("What background colour does the xylophone loading screen have?"))
        self.assertIn("cannot judge relevance", denied_reason(out))
        d = self.decisions()[-1]
        self.assertEqual(d["reason"], "hits_shown")
        self.assertEqual(d["raw_hits"], 2)
        self.assertEqual(d["name_hits"], 0)

    def test_a_name_match_is_counted_in_the_log(self):
        FakeKaeru.known = {"provider"}
        FakeKaeru.echo_terms = False
        self.run_hook(self.ask("Which provider are we on?"))
        self.assertEqual(self.decisions()[-1]["name_hits"], 1)

    def test_search_can_be_disabled_back_to_the_timer(self):
        # No server needed: with KAERU_FIRST_SEARCH=0 the first version's rule applies.
        out = self.run_hook(self.ask("Which provider are we on?"), url=DEAD_URL, KAERU_FIRST_SEARCH="0")
        self.assertIn("Before you ask the user", denied_reason(out))
        self.run_hook(self.read("overview"), url=DEAD_URL, KAERU_FIRST_SEARCH="0")
        self.assertIsNone(self.run_hook(self.ask("Which provider are we on?", prompt_id="t2"), url=DEAD_URL, KAERU_FIRST_SEARCH="0"))


# ------------------------------------------------------- it cleans up after itself


class SessionHygieneTests(HookCase):
    """The daemon never reaps an idle session, so every one the hook opens it must close."""

    def test_every_search_closes_the_session_it_opened(self):
        FakeKaeru.known = {"provider"}
        for turn in range(5):
            self.run_hook(self.ask("Which provider are we on?", prompt_id=f"t{turn}"))
        self.assertEqual(len(FakeKaeru.opened), 5)
        self.assertEqual(FakeKaeru.closed, FakeKaeru.opened)

    def test_a_search_with_no_hits_closes_it_too(self):
        self.run_hook(self.ask("Which provider are we on?"))
        self.assertEqual(FakeKaeru.closed, FakeKaeru.opened)
        self.assertEqual(len(FakeKaeru.opened), 1)

    def test_a_failed_search_still_closes_the_session(self):
        FakeKaeru.known = {"provider"}
        FakeKaeru.fail_call = True
        out = self.run_hook(self.ask("Which provider are we on?"))
        self.assertIsNone(out)                                   # fails open…
        self.assertEqual(self.decisions()[-1]["reason"], "daemon_unreachable")
        self.assertEqual(FakeKaeru.closed, FakeKaeru.opened)     # …and still cleans up
        self.assertEqual(len(FakeKaeru.opened), 1)

    def test_an_exempt_ask_opens_nothing(self):
        self.run_hook(self.ask("Say «fix it» and I will go."))
        self.assertEqual(FakeKaeru.opened, [])


# ------------------------------------------------------------- the Stop path


class StopShapeTests(HookCase):
    """The wider net: an ask is not only a trailing question mark."""

    def setUp(self):
        super().setUp()
        FakeKaeru.known = {"provider"}

    def test_a_trailing_question_blocks(self):
        out = self.run_hook(self.stop("Changes are in.\n\nWhich provider do we set up?"))
        self.assertEqual((out or {}).get("decision"), "block")
        self.assertEqual(self.decisions()[-1]["shape"], "q_last")

    def test_a_question_above_the_last_line_blocks(self):
        out = self.run_hook(self.stop("Which provider do we set up?\nI left it untouched for the moment."))
        self.assertEqual((out or {}).get("decision"), "block")
        self.assertEqual(self.decisions()[-1]["shape"], "q_any")

    def test_an_imperative_hand_off_blocks(self):
        out = self.run_hook(self.stop("Gates are green.\n**I need your answer about the provider.**"))
        self.assertEqual((out or {}).get("decision"), "block")
        self.assertEqual(self.decisions()[-1]["shape"], "imperative")

    def test_an_offer_blocks(self):
        out = self.run_hook(self.stop("Finished.\nWant me to switch the provider to the backup one."))
        self.assertEqual((out or {}).get("decision"), "block")
        self.assertEqual(self.decisions()[-1]["shape"], "offer")

    def test_a_formatted_status_report_is_not_an_ask(self):
        """The first live firing of this hook was a false positive on exactly this:
        a report with bold bullets and an "or" somewhere in its tail."""
        report = ("Installed in both places.\n"
                  "- **The log is empty.** Nothing ended in a question or a request yet.\n"
                  "- **The provider side is untested.** It shows up after a real session.\n"
                  "Next: the provider plan gets checked after the first run.")
        self.assertIsNone(self.run_hook(self.stop(report)))
        self.assertEqual(self.decisions(), [])

    def test_a_numbered_next_step_is_not_an_ask(self):
        self.assertIsNone(self.run_hook(self.stop("Done with the provider.\n1. Keep\n2. Switch\nNext step: open the PR for the provider.")))
        self.assertEqual(self.decisions(), [])

    def test_an_enumerated_choice_with_a_question_is_still_caught(self):
        out = self.run_hook(self.stop("Paths for the provider:\n1. Keep\n2. Switch\n3. Postpone\nWhich do we take?"))
        self.assertEqual((out or {}).get("decision"), "block")
        self.assertEqual(self.decisions()[-1]["shape"], "q_last")

    def test_a_next_action_is_a_hand_off_not_an_ask(self):
        """The third live false positive: an agent that closes every reply with a next action."""
        for text in ("Session leak fixed, 68 tests green.\n\nNext: open a new session and give me the log path.",
                     "Done.\n**Next step (1 minute):** tell me when the run is finished.",
                     "All pushed.\n- Now: send me nothing, just open the PR page."):
            self.assertIsNone(kf.asking_shape(text), text)
        # A question survives the marker — that one IS an ask.
        self.assertEqual(kf.asking_shape("Done.\nNext step: which provider do we set up?")[0], "q_last")
        # And an imperative elsewhere in the tail still counts.
        self.assertEqual(kf.asking_shape("I need your answer about the provider.\nNext: run the suite.")[0], "imperative")

    def test_a_quoted_phrase_is_mentioned_not_used(self):
        """The second live false positive: a report ABOUT the lexicon, quoting its entries."""
        for text in ("Narrowed the list. It now holds only addressed forms such as «please confirm».",
                     'The bare form is gone, the addressed one stays ("waiting for your answer").',
                     "The matcher keys on `tell me` and `let me know`, nothing looser.",
                     "A short stem still passes untouched, for instance «Ship it?»"):
            self.assertIsNone(kf.asking_shape(text), text)
        # …while an imperative that merely carries a quote is still one, and still a go-word.
        shape, asking = kf.asking_shape("Tell me when — just write «done» and the run begins.")
        self.assertEqual(shape, "imperative")
        self.assertEqual(kf.is_procedural(asking), "quoted_go")

    def test_reports_that_open_like_offers_are_not_asks(self):
        for text in ("All green. I can confirm the tests pass on both platforms.",
                     "The migration is done; you can decide later whether to keep the old table.",
                     "I could not reproduce it locally.",
                     "Happy to report the build is fixed."):
            self.assertIsNone(kf.asking_shape(text), text)
        self.assertEqual(kf.asking_shape("Please confirm which provider we use.")[0], "imperative")
        self.assertEqual(kf.asking_shape("Done.\nIf you want, the provider can move to the backup one.")[0], "offer")

    def test_a_statement_passes(self):
        self.assertIsNone(self.run_hook(self.stop("The provider is configured and the suite is green.")))
        self.assertEqual(self.decisions(), [])

    def test_a_procedural_hand_off_passes(self):
        self.assertIsNone(self.run_hook(self.stop("The provider is set. Tell me when — just write «done» and the run begins.")))
        self.assertEqual(self.decisions()[-1]["reason"], "exempt:quoted_go")

    def test_the_loop_guard_is_honoured(self):
        out = self.run_hook(self.stop("Which provider do we set up?", active=True))
        self.assertIsNone(out)
        self.assertEqual(self.decisions()[-1]["reason"], "stop_hook_active")

    def test_a_full_width_question_mark_counts(self):
        self.assertEqual(kf.asking_shape("Which provider？")[0], "q_last")

    def test_markdown_around_the_question_mark_is_seen_through(self):
        self.assertEqual(kf.asking_shape("**Which provider?**")[0], "q_last")


# ---------------------------------------------------------- the human's reply


class ReplyTests(HookCase):
    def test_a_substantive_answer_gets_the_capture_reminder(self):
        self.run_hook(self.stop("Which certificate belongs on the server?"))
        out = self.run_hook(self.prompt("The wildcard one from the internal CA, it already sits under /etc/ssl there."))
        self.assertIn("capture it now", out["hookSpecificOutput"]["additionalContext"])
        self.assertEqual(self.decisions()[-1]["outcome"], "answered")

    def test_a_short_acknowledgement_gets_nothing(self):
        self.run_hook(self.stop("Which certificate belongs on the server?"))
        self.assertIsNone(self.run_hook(self.prompt("ok")))

    def test_it_was_in_memory_is_logged_as_a_miss(self):
        self.run_hook(self.stop("Which certificate belongs on the server?"))
        out = self.run_hook(self.prompt("I already gave you that guide, check kaeru"))
        self.assertIn("already in memory", out["hookSpecificOutput"]["additionalContext"])
        d = self.decisions()[-1]
        self.assertEqual(d["outcome"], "miss")
        self.assertEqual(d["after_shape"], "q_last")

    def test_a_use_memory_complaint_is_logged(self):
        self.run_hook(self.stop("Which certificate belongs on the server?"))
        self.run_hook(self.prompt("come on, use memory for once"))
        self.assertEqual(self.decisions()[-1]["outcome"], "complaint")

    def test_a_capture_nudge_is_logged(self):
        self.run_hook(self.stop("Which certificate belongs on the server?"))
        self.assertIsNone(self.run_hook(self.prompt("put that into kaeru")))
        self.assertEqual(self.decisions()[-1]["outcome"], "capture_nudge")

    def test_a_prompt_that_answers_nothing_is_not_classified(self):
        """The markers are loose; only an ask gives them something to be about."""
        for text in ("we decided on postgres last week, now implement the migration",
                     "as I said, keep it simple — write the parser",
                     "we did it! now let's go to the next issue"):
            self.assertIsNone(self.run_hook(self.prompt(text)), text)
        self.assertEqual(self.decisions(), [])

    def test_a_statement_in_between_clears_the_ask(self):
        self.run_hook(self.stop("Which certificate belongs on the server?"))
        self.run_hook(self.stop("Never mind, found it: the wildcard one.", prompt_id="t2"))
        self.assertIsNone(self.run_hook(self.prompt("we decided on postgres last week, now implement the migration")))

    def test_a_go_word_is_not_knowledge_to_capture(self):
        """After «say "ship it"» whatever comes next is a go-word or a new task, not an answer."""
        self.run_hook(self.stop("The provider is set. Tell me when — just write «done» and the run begins."))
        self.assertIsNone(self.run_hook(self.prompt("the review came back, have a look at what they are asking for")))
        self.assertEqual(self.decisions()[-1]["outcome"], "answered")

    def test_the_reply_text_never_reaches_the_log(self):
        self.run_hook(self.stop("Which certificate belongs on the server?"))
        reply = "use this one: sk-live-0123456789abcdef0123456789abcdef, it sits under /etc/ssl"
        self.run_hook(self.prompt(reply))
        raw = (Path(self.state) / "decisions.jsonl").read_text()
        self.assertNotIn("sk-live", raw)
        self.assertEqual(self.decisions()[-1]["reply_len"], len(reply))


# ------------------------------------------------------------------ lexicons


class LexiconTests(HookCase):
    """Language is data: a vault in another language needs a file, not a fork."""

    def lexicon(self, files: dict[str, str]) -> str:
        d = tempfile.mkdtemp(prefix="kaeru-first-lex-")
        for name, text in files.items():
            (Path(d) / name).write_text(text, encoding="utf-8")
        return d

    def test_an_extra_lexicon_widens_the_net(self):
        FakeKaeru.known = {"provider"}
        text = "The provider is configured. Zzask the plan of the provider"
        self.assertIsNone(self.run_hook(self.stop(text)))          # English alone sees no ask
        d = self.lexicon({"xx.json": json.dumps({"imperative": ["zzask"], "stop": ["configured"]})})
        out = self.run_hook(self.stop(text, prompt_id="t2"), KAERU_FIRST_LEXICON_DIR=d)
        self.assertEqual((out or {}).get("decision"), "block")
        self.assertEqual(self.decisions()[-1]["shape"], "imperative")
        self.assertNotIn("configured", self.decisions()[-1]["terms"])

    def test_a_broken_lexicon_file_is_skipped(self):
        FakeKaeru.known = {"provider"}
        d = self.lexicon({"bad.json": "{ not json"})
        out = self.run_hook(self.ask("Which provider are we on?"), KAERU_FIRST_LEXICON_DIR=d)
        self.assertIsNotNone(out)

    def test_a_bad_fragment_skips_its_file_and_only_its_file(self):
        FakeKaeru.known = {"provider"}
        d = self.lexicon({"bad.json": json.dumps({"imperative": ["(unclosed"]}),
                          "good.json": json.dumps({"imperative": ["zzask"]})})
        out = self.run_hook(self.stop("The provider is configured. Zzask the plan of the provider"), KAERU_FIRST_LEXICON_DIR=d)
        self.assertEqual((out or {}).get("decision"), "block")          # good.json still applied
        out = self.run_hook(self.stop("Gates are green.\n**I need your answer about the provider.**", prompt_id="t2"),
                            KAERU_FIRST_LEXICON_DIR=d)
        self.assertEqual((out or {}).get("decision"), "block")          # and English is intact

    def test_every_shipped_lexicon_is_well_formed(self):
        import re as _re
        shipped = sorted((SCRIPT.parent / "lexicon").glob("*.json"))
        self.assertTrue(shipped, "a lexicon directory ships with the hook")
        for f in shipped:
            data = json.loads(f.read_text(encoding="utf-8"))
            self.assertLessEqual(set(data), set(kf.EN), f"{f.name}: unknown key")
            for key, values in data.items():
                self.assertIsInstance(values, list, f"{f.name}:{key}")
                if key != "stop":
                    for fragment in values:
                        _re.compile(fragment)


# -------------------------------------------------------------- pure helpers


class HelperTests(unittest.TestCase):
    def test_terms_drop_procedure_and_keep_entities(self):
        self.assertEqual(kf.terms_of("Should I start 4.2–4.4 now or after your review of the pilot?"), ["review", "pilot"])

    def test_three_letter_ascii_tokens_survive(self):
        t = kf.terms_of("I need your login for the vpn and the api key")
        self.assertEqual(t, ["login", "api", "key", "vpn"])

    def test_the_entity_goes_first_even_when_it_is_the_shortest_word(self):
        t = kf.terms_of("Once you tell me about the key for AcmeCloud I will run generation.")
        self.assertEqual(t[0], "acmecloud")        # capitalised mid-sentence
        self.assertEqual(kf.terms_of("Left to consult Dana about billing.")[0], "dana")
        # …while a capital at the START of a sentence is just a capital.
        self.assertEqual(kf.terms_of("Generation is pending.\nDana is away, billing stays.")[0], "generation")
        self.assertEqual(kf.terms_of("Which bucket holds backup-2024 now?")[0], "backup-2024")

    def test_a_latin_token_inside_another_script_is_an_entity(self):
        self.assertEqual(kf.terms_of("Χρειάζομαι την απάντησή σου για τον πάροχο acme")[0], "acme")

    def test_terms_are_capped(self):
        t = kf.terms_of("alpha bravo charlie delta echoes foxtrot golfing hotels indigo juliet kilos")
        self.assertEqual(len(t), 8)
        self.assertGreaterEqual(len(t[0]), len(t[-1]))

    def test_parse_hits_reads_the_daemon_format(self):
        text = "matches (2):\n  - a-b (episode) — 01\n    first\n  - c-d (reference) — 02\n    second\n\n↳ tail"
        self.assertEqual(kf.parse_hits(text), [("a-b", "first"), ("c-d", "second")])
        self.assertEqual(kf.parse_hits("(no matches)\n↳ widen"), [])

    def test_rank_hits_puts_a_name_match_first_and_drops_nothing(self):
        hits = [("loading-screen", "mentions the provider and acme-cloud"), ("provider-switch-note", "empty")]
        ranked = kf.rank_hits(hits, ["provider", "acme-cloud"])
        self.assertEqual([h[0] for h in ranked], ["provider-switch-note", "loading-screen"])
        self.assertEqual(len(kf.rank_hits(hits, ["nothing"])), 2)

    def test_stem_survives_an_inflection(self):
        self.assertIn(kf.stem("providers"), "provider-switch-note")

    def test_a_bare_question_under_a_numbered_list_borrows_its_subject(self):
        shape, asking = kf.asking_shape("Paths for the provider:\n1. Keep\n2. Switch\nWhich do we take?")
        self.assertEqual(shape, "q_last")
        self.assertIn("provider", kf.terms_of(asking))
        # …but bold bullets are report formatting, and lend nothing.
        _, asking = kf.asking_shape("- **Provider:** kept\n- **Plan:** unchanged\nWhat next?")
        self.assertEqual(kf.terms_of(asking), [])

    def test_only_the_last_line_counts_for_q_last(self):
        self.assertEqual(kf.asking_shape("A question?\nAn answer.")[0], "q_any")


if __name__ == "__main__":
    unittest.main(verbosity=2)
