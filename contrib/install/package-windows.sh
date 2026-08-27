#!/usr/bin/env bash
# Builds the Windows prebuilt, which the main packaging script cannot: it
# needs cargo-xwin and the MSVC CRT, and both are easiest inside a container.
#
#   ./contrib/install/package-windows.sh v0.7.0
#
# Output: dist/kaeru-v0.7.0-x86_64-pc-windows-msvc.zip
#
# Three things here are not obvious and each has cost a release:
#
#   1. `--network host`. Inside a bridged container, 127.0.0.1 is the
#      container — so a proxy listening on the host's loopback is
#      unreachable, and cargo-xwin's MSVC CRT download fails.
#
#   2. A persistent XWIN_CACHE_DIR. Without it every run re-downloads the
#      CRT, which is exactly the fetch most likely to fail.
#
#   3. The mtime check at the end. **A failed cargo-xwin run leaves the
#      previous release's kaeru-mcp.exe in target/**, so packaging succeeds
#      and ships a stale binary. v0.6.0 nearly went out that way. The check
#      is not paranoia; it is the reason this script exists as a script.

set -euo pipefail

TAG="${1:-}"
[[ -n "$TAG" ]] || { echo "usage: $0 <tag, e.g. v0.7.0>" >&2; exit 1; }

TARGET=x86_64-pc-windows-msvc
IMAGE="${KAERU_XWIN_IMAGE:-rust:latest}"
XWIN_CACHE="${XWIN_CACHE_DIR:-$HOME/.cache/cargo-xwin}"

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT"
mkdir -p dist "$XWIN_CACHE"

# Record where we start, so a build that silently does nothing is caught.
before=$(stat -c %Y "target/$TARGET/release/kaeru-mcp.exe" 2>/dev/null || echo 0)
started=$(date +%s)

echo "==> building $TARGET in $IMAGE"
docker run --rm \
    --network host \
    -e HTTP_PROXY="${HTTP_PROXY:-}" -e HTTPS_PROXY="${HTTPS_PROXY:-}" \
    -e http_proxy="${http_proxy:-}" -e https_proxy="${https_proxy:-}" \
    -e NO_PROXY="${NO_PROXY:-localhost,127.0.0.1}" \
    -e XWIN_CACHE_DIR=/xwin \
    -e CARGO_TERM_COLOR=never \
    -v "$ROOT:/src" -v "$XWIN_CACHE:/xwin" \
    -v "$HOME/.cargo/registry:/usr/local/cargo/registry" \
    -w /src \
    "$IMAGE" \
    bash -c '
        set -e
        rustup target add x86_64-pc-windows-msvc
        cargo install cargo-xwin --locked 2>/dev/null || true
        # -p, for the same reason the other targets use it: without it cargo
        # unifies features across the workspace and builds kaeru-mcp with
        # dependencies it does not use.
        cargo xwin build --release --target x86_64-pc-windows-msvc -p kaeru-mcp --bin kaeru-mcp
    '

exe="target/$TARGET/release/kaeru-mcp.exe"
[[ -f "$exe" ]] || { echo "!!  $exe does not exist — the build produced nothing" >&2; exit 1; }

after=$(stat -c %Y "$exe")
if [[ "$after" == "$before" || "$after" -lt "$started" ]]; then
    echo "!!  $exe was not rebuilt by this run — it is left over from an" >&2
    echo "!!  earlier build. Something failed inside the container without" >&2
    echo "!!  stopping the script. Do NOT ship this." >&2
    exit 1
fi

stage=$(mktemp -d)
cp "$exe" "$stage/"
archive="kaeru-${TAG}-${TARGET}.zip"
(cd "$stage" && zip -q "$OLDPWD/dist/$archive" kaeru-mcp.exe)
rm -rf "$stage"

echo "==> dist/$archive"
echo "    built $(date -d "@$after" '+%Y-%m-%d %H:%M')"
echo
echo "Add it to SHA256SUMS alongside the other targets:"
echo "    (cd dist && sha256sum $archive >> SHA256SUMS)"
