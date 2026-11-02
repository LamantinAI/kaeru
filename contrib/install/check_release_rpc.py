"""Validate a release smoke-test reply instead of accepting error text as data."""
import argparse
import json
import sys


def validate(raw, expected=()):
    try:
        reply = json.loads(raw)
    except json.JSONDecodeError:
        frames = [json.loads(line[5:].strip()) for line in raw.splitlines()
                  if line.startswith("data:") and line[5:].strip()]
        reply = next((frame for frame in reversed(frames)
                      if "result" in frame or "error" in frame), None)
    if not isinstance(reply, dict) or "error" in reply or "result" not in reply:
        raise ValueError("MCP request did not return a successful result")
    result = reply["result"]
    if not isinstance(result, dict) or result.get("isError"):
        raise ValueError("MCP tool returned an error")
    payload = result.get("structuredContent")
    if isinstance(payload, dict) and payload.get("state") not in ("ok", "applied", "unchanged"):
        raise ValueError("Memory operation did not complete successfully")
    text = json.dumps(result)
    for value in expected:
        if value not in text:
            raise ValueError(f"Expected content was not returned: {value}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--expect", action="append", default=[])
    args = parser.parse_args()
    try:
        validate(sys.stdin.read(), args.expect)
    except (ValueError, KeyError) as error:
        parser.exit(1, f"Release smoke check failed: {error}\n")


if __name__ == "__main__":
    main()
