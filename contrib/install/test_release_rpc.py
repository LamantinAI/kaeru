import json
import unittest

from check_release_rpc import validate


class ReleaseReplyTest(unittest.TestCase):
    def test_error_containing_expected_text_is_not_a_success(self):
        reply = {"error": {"message": "release check note 1"}}
        with self.assertRaises(ValueError):
            validate(json.dumps(reply), ["release check note 1"])

    def test_tool_and_memory_errors_are_not_successes(self):
        for result in [{"isError": True}, {"structuredContent": {"state": "error"}},
                       {"structuredContent": {"state": "partial"}}]:
            with self.assertRaises(ValueError):
                validate(json.dumps({"result": result}))

    def test_sse_success_requires_all_expected_records(self):
        reply = {"result": {"structuredContent": {"state": "ok", "data": ["first", "second"]}}}
        raw = ": heartbeat\n\nevent: message\ndata: " + json.dumps(reply) + "\n\n"
        validate(raw, ["first", "second"])
        with self.assertRaises(ValueError):
            validate(raw, ["missing"])


if __name__ == "__main__":
    unittest.main()
