#!/usr/bin/env bash
# kaeru installer — downloads prebuilt binaries from a GitHub release
# and drops them in $KAERU_INSTALL_DIR (default: ~/.local/bin).
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/LamantinAI/kaeru/main/contrib/install/install.sh | bash
#
# Env overrides:
#   KAERU_VERSION        version tag to install, e.g. "v0.1.0" (default: latest)
#   KAERU_INSTALL_DIR    where to put the binaries (default: ~/.local/bin)
#   KAERU_SETUP_DAEMON   yes (default) to install a user-level systemd /
#                        launchd unit and start kaeru-mcp; "no" to skip
#
# Supported targets in this release:
#   - linux  / x86_64  -> static musl binary, runs on any glibc or musl host
#   - macOS  / arm64   -> Apple Silicon (M1/M2/M3); UNSIGNED, see post-install note

set -euo pipefail

REPO="LamantinAI/kaeru"
VERSION="${KAERU_VERSION:-latest}"
INSTALL_DIR="${KAERU_INSTALL_DIR:-$HOME/.local/bin}"
SETUP_DAEMON="${KAERU_SETUP_DAEMON:-yes}"

say()  { printf '==> %s\n' "$*"; }
warn() { printf '!!  %s\n' "$*" >&2; }
die()  { printf 'xx  %s\n' "$*" >&2; exit 1; }

os=$(uname -s)
arch=$(uname -m)

case "$os/$arch" in
    Linux/x86_64)            target="x86_64-unknown-linux-musl" ;;
    Darwin/arm64|Darwin/aarch64) target="aarch64-apple-darwin" ;;
    Darwin/x86_64)
        die "Intel Mac (x86_64) is not yet shipped as a prebuilt; build from source — see README."
        ;;
    Linux/aarch64|Linux/arm64)
        die "Linux arm64 is not yet shipped as a prebuilt; build from source — see README."
        ;;
    *)
        die "unsupported platform: $os/$arch"
        ;;
esac

if [[ "$VERSION" == "latest" ]]; then
    say "resolving latest release of $REPO"
    # Split curl from grep|sed: with `set -o pipefail`, grep -m1 closing
    # the pipe early can SIGPIPE curl and abort the script before we
    # even use the result.
    api_response=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest")
    tag=$(printf '%s\n' "$api_response" | grep -m1 '"tag_name"' \
        | sed -E 's/.*"tag_name"[[:space:]]*:[[:space:]]*"([^"]+)".*/\1/')
    [[ -n "$tag" ]] || die "could not resolve latest tag from GitHub API"
else
    tag="$VERSION"
fi

archive="kaeru-${tag}-${target}.tar.gz"
url="https://github.com/${REPO}/releases/download/${tag}/${archive}"

say "downloading $url"
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
curl -fsSL "$url" -o "$tmp/$archive" \
    || die "download failed; check that release $tag and asset $archive exist"

say "extracting"
tar -xzf "$tmp/$archive" -C "$tmp"

mkdir -p "$INSTALL_DIR"
mv "$tmp/kaeru" "$tmp/kaeru-mcp" "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/kaeru" "$INSTALL_DIR/kaeru-mcp"

# Strip macOS Gatekeeper quarantine bit. Binaries are unsigned (we cross-build
# from Linux) so without this the user gets a "cannot be opened because the
# developer cannot be verified" dialog on first run.
if [[ "$os" == "Darwin" ]]; then
    xattr -d com.apple.quarantine "$INSTALL_DIR/kaeru"     2>/dev/null || true
    xattr -d com.apple.quarantine "$INSTALL_DIR/kaeru-mcp" 2>/dev/null || true
fi

say "installed kaeru $tag -> $INSTALL_DIR"

case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
        warn "$INSTALL_DIR is not in your PATH"
        warn "add this to ~/.bashrc or ~/.zshrc:"
        warn "    export PATH=\"$INSTALL_DIR:\$PATH\""
        ;;
esac

echo
"$INSTALL_DIR/kaeru" --version || true
echo

# ------------------------------------------------------------------
# Daemon setup (user-level; no sudo).
# Linux  -> systemd user service in ~/.config/systemd/user/
# macOS  -> launchd LaunchAgent in ~/Library/LaunchAgents/
# Skipped if KAERU_SETUP_DAEMON=no.
# ------------------------------------------------------------------

install_systemd_user_unit() {
    local unit_dir="$HOME/.config/systemd/user"
    local unit_path="$unit_dir/kaeru-mcp.service"
    local bin_path="$INSTALL_DIR/kaeru-mcp"

    if ! command -v systemctl >/dev/null 2>&1; then
        warn "systemctl not found; skipping daemon setup"
        warn "you can run kaeru-mcp manually: $bin_path"
        return
    fi

    say "writing systemd user unit -> $unit_path"
    mkdir -p "$unit_dir"
    cat > "$unit_path" <<EOF
[Unit]
Description=kaeru-mcp — cognitive memory MCP server (HTTP daemon)
Documentation=https://github.com/${REPO}
After=default.target

[Service]
Type=simple
ExecStart=$bin_path
Restart=on-failure
RestartSec=2

# kaeru-mcp tunables — uncomment and edit, then \`systemctl --user daemon-reload\`.
#Environment=KAERU_MCP_LISTEN_ADDRESS=127.0.0.1
#Environment=KAERU_MCP_LISTEN_PORT=9876
#Environment=KAERU_MCP_MOUNT_PATH=/mcp
#Environment=KAERU_MCP_LOG_LEVEL=info
#Environment=KAERU_VAULT_PATH=%h/.local/share/kaeru

[Install]
WantedBy=default.target
EOF

    systemctl --user daemon-reload
    systemctl --user enable --now kaeru-mcp.service

    sleep 1
    if systemctl --user is-active --quiet kaeru-mcp.service; then
        say "kaeru-mcp daemon is running"
        say "    status:  systemctl --user status kaeru-mcp"
        say "    logs:    journalctl --user -u kaeru-mcp -f"
        return 0
    else
        warn "daemon failed to start; inspect: systemctl --user status kaeru-mcp"
        return 1
    fi
}

install_launchd_user_agent() {
    local agents_dir="$HOME/Library/LaunchAgents"
    local plist_path="$agents_dir/ai.lamantin.kaeru-mcp.plist"
    local bin_path="$INSTALL_DIR/kaeru-mcp"
    local log_path="$HOME/Library/Logs/kaeru-mcp.log"

    if ! command -v launchctl >/dev/null 2>&1; then
        warn "launchctl not found; skipping daemon setup"
        warn "you can run kaeru-mcp manually: $bin_path"
        return
    fi

    say "writing launchd user agent -> $plist_path"
    mkdir -p "$agents_dir" "$HOME/Library/Logs"

    # Idempotent re-install: drop any previous instance before rewriting.
    launchctl unload "$plist_path" 2>/dev/null || true

    cat > "$plist_path" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>ai.lamantin.kaeru-mcp</string>
    <key>ProgramArguments</key>
    <array>
        <string>$bin_path</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>EnvironmentVariables</key>
    <dict>
        <key>KAERU_MCP_LOG_LEVEL</key><string>info</string>
    </dict>
    <key>StandardOutPath</key>
    <string>$log_path</string>
    <key>StandardErrorPath</key>
    <string>$log_path</string>
</dict>
</plist>
EOF

    launchctl load "$plist_path"

    sleep 1
    if launchctl list 2>/dev/null | grep -q ai.lamantin.kaeru-mcp; then
        say "kaeru-mcp daemon is running"
        say "    status:  launchctl list | grep kaeru-mcp"
        say "    logs:    tail -f $log_path"
        return 0
    else
        warn "daemon failed to start; inspect log: $log_path"
        return 1
    fi
}

if [[ "$SETUP_DAEMON" != "yes" ]]; then
    say "skipping daemon setup (KAERU_SETUP_DAEMON=$SETUP_DAEMON)"
elif [[ "$os" == "Linux" ]]; then
    install_systemd_user_unit || true
elif [[ "$os" == "Darwin" ]]; then
    install_launchd_user_agent || true
fi

echo
say "final step — point your agent at the daemon:"
say "    claude mcp add --transport http kaeru http://127.0.0.1:9876/mcp"
