#!/usr/bin/env bash
# kaeru opencode wiring — installs AGENTS.kaeru.md + slash commands into
# ~/.config/opencode/, and helps you merge the kaeru MCP block into your
# existing opencode.json.
#
# Does NOT install the kaeru-mcp daemon — that's contrib/install/install.sh.
# Does NOT touch your model providers, API keys, or any other config —
# only adds `mcp.kaeru` and `instructions`.
#
# Usage:
#   bash contrib/opencode/install-opencode.sh
#
# Idempotent. Re-running overwrites the AGENTS file and command files
# but never touches your opencode.json — the merge is always your call.

set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
agents_src="$here/AGENTS.kaeru.md"
snippet_src="$here/opencode.kaeru.json"
commands_src="$here/commands"

cfg_dir="${XDG_CONFIG_HOME:-$HOME/.config}/opencode"
agents_dst="$cfg_dir/AGENTS.kaeru.md"
commands_dst="$cfg_dir/commands"
opencode_json="$cfg_dir/opencode.json"

mcp_url="${KAERU_MCP_URL:-http://127.0.0.1:9876/mcp}"

say()  { printf '==> %s\n' "$*"; }
warn() { printf '!!  %s\n' "$*" >&2; }
die()  { printf 'xx  %s\n' "$*" >&2; exit 1; }

# ------------------------------------------------------------------
# Sanity: the three source files must exist (we're running from a
# checked-out repo, not a curl|bash one-liner).
# ------------------------------------------------------------------
[[ -f "$agents_src"  ]] || die "missing $agents_src — run this from a kaeru repo checkout"
[[ -f "$snippet_src" ]] || die "missing $snippet_src"
[[ -d "$commands_src" ]] || die "missing $commands_src/"

command -v jq >/dev/null 2>&1 || die "jq not found; install jq first (it's needed for the config merge)"

# ------------------------------------------------------------------
# 1. AGENTS.kaeru.md + commands — user-owned files, no sudo.
# ------------------------------------------------------------------
mkdir -p "$cfg_dir" "$commands_dst"

say "installing AGENTS.kaeru.md -> $agents_dst"
cp "$agents_src" "$agents_dst"

for f in "$commands_src"/*.md; do
    name=$(basename "$f")
    say "installing command -> $commands_dst/$name"
    cp "$f" "$commands_dst/$name"
done

# ------------------------------------------------------------------
# 2. opencode.json — three cases.
# ------------------------------------------------------------------
echo
if [[ ! -e "$opencode_json" ]]; then
    say "no opencode.json yet — writing fresh one with kaeru wiring"
    cp "$snippet_src" "$opencode_json"
    say "wrote $opencode_json"

elif [[ -L "$opencode_json" ]]; then
    target=$(readlink -f "$opencode_json")
    warn "$opencode_json is a symlink -> $target"
    warn "we won't edit that file directly. Run this merge yourself:"
    echo
    echo "    jq -s '.[0] * .[1]' \\"
    echo "        $target \\"
    echo "        $snippet_src \\"
    echo "      | sudo tee $target.new >/dev/null \\"
    echo "      && sudo cp $target $target.bak.kaeru-add \\"
    echo "      && sudo mv $target.new $target"
    echo

else
    if jq -e '.mcp.kaeru' "$opencode_json" >/dev/null 2>&1; then
        say "$opencode_json already has mcp.kaeru — skipping merge"
    else
        backup="$opencode_json.bak.kaeru-add"
        say "merging kaeru wiring into $opencode_json"
        say "backup -> $backup"
        cp "$opencode_json" "$backup"
        jq -s '.[0] * .[1]' "$opencode_json" "$snippet_src" > "$opencode_json.new"
        mv "$opencode_json.new" "$opencode_json"
        say "merged. Diff against backup:"
        diff -u "$backup" "$opencode_json" || true
    fi
fi

# ------------------------------------------------------------------
# 3. Probe the daemon. Don't block — just inform.
# ------------------------------------------------------------------
echo
say "probing kaeru-mcp at $mcp_url"
if command -v curl >/dev/null 2>&1; then
    # Streamable HTTP MCP returns 406 on a bare GET (no Accept header) —
    # that's the "alive" signal we look for, not an error.
    code=$(curl -s -o /dev/null -w '%{http_code}' --max-time 3 "$mcp_url" || echo "000")
    case "$code" in
        2*|4*) say "daemon reachable (HTTP $code)" ;;
        000)   warn "daemon not reachable at $mcp_url — is kaeru-mcp running?" ;;
        *)     warn "unexpected response from $mcp_url (HTTP $code)" ;;
    esac
else
    warn "curl not found; skipping daemon probe"
fi

# ------------------------------------------------------------------
# 4. Next steps.
# ------------------------------------------------------------------
cat <<EOF

==> done.

Restart your opencode session. The agent will see:

  - kaeru MCP tools (kaeru_awake, kaeru_drill, kaeru_jot, …)
  - The AGENTS.kaeru.md guidance loaded into every system prompt
  - Slash commands /kaeru, /lesson, /recall

Try in opencode:

    /kaeru                          # re-entry ritual for the current project
    /recall <topic>                 # fuzzy lookup
    /lesson <body>                  # capture a settled lesson

If the daemon was not reachable above, start it:

    systemctl --user start kaeru-mcp           # Linux (installed via contrib/install/install.sh)
    launchctl start ai.lamantin.kaeru-mcp      # macOS
    kaeru-mcp                                  # foreground, anywhere

EOF
