#!/usr/bin/env bash
# kaeru installer — downloads prebuilt binaries from a GitHub release
# and drops them in $KAERU_INSTALL_DIR (default: ~/.local/bin).
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/GrumpyChubbyCat/kaeru/main/contrib/install/install.sh | bash
#
# Env overrides:
#   KAERU_VERSION        version tag to install, e.g. "v0.1.0" (default: latest)
#   KAERU_INSTALL_DIR    where to put the binaries (default: ~/.local/bin)
#
# Supported targets in this release:
#   - linux  / x86_64  -> static musl binary, runs on any glibc or musl host
#   - macOS  / arm64   -> Apple Silicon (M1/M2/M3); UNSIGNED, see post-install note

set -euo pipefail

REPO="GrumpyChubbyCat/kaeru"
VERSION="${KAERU_VERSION:-latest}"
INSTALL_DIR="${KAERU_INSTALL_DIR:-$HOME/.local/bin}"

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
    tag=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
        | grep -m1 '"tag_name"' \
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
say "next: set up the MCP daemon — see https://github.com/${REPO}#connecting-to-an-mcp-aware-agent"
