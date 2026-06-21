#!/usr/bin/env bash
# Bakes a curated, redacted graph snapshot from the live kaeru-mcp daemon into
# kaeru-viz/public/graph.json — an offline copy the app falls back to when no
# daemon is reachable. The snapshot can contain real vault content, so it is
# git-ignored; never commit it.
#
# The daemon must have the viz endpoint enabled (KAERU_MCP_VIZ_ENABLE=1) with an
# allow-list configured (KAERU_MCP_VIZ_INITIATIVES). `initiatives_csv` here only
# NARROWS within that configured allow-list — it can't widen the export.
#
# Usage:  scripts/bake-graph-snapshot.sh [initiatives_csv]
#   KAERU_VIZ_URL   daemon base url (default http://127.0.0.1:9876)
#   KAERU_VIZ_DENY  CSV of initiative-name substrings that must NOT appear in
#                   the baked snapshot — a fail-closed guard against an
#                   accidentally-too-broad export. Configure it locally; no
#                   names are baked into this script.
set -euo pipefail

url="${KAERU_VIZ_URL:-http://127.0.0.1:9876}"
inits="${1:-}"
here="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
out="$here/kaeru-viz/public/graph.json"
mkdir -p "$(dirname "$out")"

q="$url/graph.json"
[[ -n "$inits" ]] && q="$q?initiatives=$inits"

echo "==> baking $q"
curl -fsS "$q" -o "$out"

# Public-safety gate: fail loudly if anything sensitive slipped through.
KAERU_VIZ_DENY="${KAERU_VIZ_DENY:-}" python3 - "$out" <<'PY'
import json, os, sys, re
g = json.load(open(sys.argv[1]))
m = g["meta"]
names = [i["name"] for i in g["initiatives"]]
deny = [d.strip().lower() for d in os.environ.get("KAERU_VIZ_DENY", "").split(",") if d.strip()]
bad = [n for n in names if any(d in n.lower() for d in deny)]
assert not bad, f"DENY LEAK (matched KAERU_VIZ_DENY): {bad}"
# no structured secrets in any exported body
secret = re.compile(r"sk-[A-Za-z0-9]{20}|ghp_[A-Za-z0-9]{20}|AKIA[A-Z0-9]{16}|-----BEGIN")
hits = [n["name"] for n in g["nodes"] if n.get("body") and secret.search(n["body"])]
assert not hits, f"SECRET LEAK in bodies: {hits}"
print(f"==> ok: {m['node_count']} nodes / {m['edge_count']} edges / "
      f"{m['initiative_count']} projects / {m['chain_count']} chains / "
      f"{m['redacted_count']} redacted — clean")
PY
echo "==> wrote $out"
