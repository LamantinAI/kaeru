#!/usr/bin/env bash
# Builds the Windows prebuilt, which the main packaging script cannot: it
# needs cargo-xwin and the MSVC CRT, and both are easiest inside a container.
#
#   ./contrib/install/package-windows.sh v0.7.3
#
# Output: dist/kaeru-v0.7.3-x86_64-pc-windows-msvc.zip
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
#   Static CRT (`+crt-static`) is what lets the .exe import only system DLLs,
#   so users need no VC++ redistributable — which the release notes promise.
#
#   3. Remove the old output before building and require a successful build.
#      Cargo hard-links artifacts, so mtime cannot prove a binary is fresh.

set -euo pipefail

TAG="${1:-}"
[[ -n "$TAG" ]] || { echo "usage: $0 <tag, e.g. v0.7.3>" >&2; exit 1; }

TARGET=x86_64-pc-windows-msvc
# messense/cargo-xwin, not a plain rust image: cargo-xwin needs clang-cl and
# lld-link, and `rust:latest` has neither — the build dies on
# `failed to find tool "clang-cl"` after pulling half a gigabyte first.
IMAGE="${KAERU_XWIN_IMAGE:-messense/cargo-xwin}"
XWIN_CACHE="${XWIN_CACHE_DIR:-$HOME/.cache/cargo-xwin}"

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT"
mkdir -p dist "$XWIN_CACHE"

# Keep the Windows cache separate from native Linux and Darwin builds.
exe="target-windows-release/$TARGET/release/kaeru-mcp.exe"

echo "==> building $TARGET in $IMAGE"
docker run --rm \
    --network host \
    -e HTTP_PROXY="${HTTP_PROXY:-}" -e HTTPS_PROXY="${HTTPS_PROXY:-}" \
    -e http_proxy="${http_proxy:-}" -e https_proxy="${https_proxy:-}" \
    -e NO_PROXY="${NO_PROXY:-localhost,127.0.0.1}" \
    -e XWIN_CACHE_DIR=/xwin \
    -e CARGO_HOME=/root/.cargo -e RUSTUP_HOME=/rustup \
    -e CARGO_TERM_COLOR=never -e CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-3}" \
    -e RUSTFLAGS="-C target-feature=+crt-static" \
    -v "$ROOT:/io" -v "$XWIN_CACHE:/xwin" -v "$HOME/.rustup:/rustup:ro" \
    -v "$HOME/.cargo/registry:/root/.cargo/registry" -v "$HOME/.cargo/git:/root/.cargo/git" \
    -w /io \
    "$IMAGE" \
    bash -c 'rm -f /io/target-windows-release/x86_64-pc-windows-msvc/release/kaeru-mcp.exe &&
             cargo xwin build --locked --release --target-dir /io/target-windows-release --target x86_64-pc-windows-msvc -p kaeru-mcp --bin kaeru-mcp'

[[ -f "$exe" ]] || { echo "!!  $exe does not exist — the build produced nothing" >&2; exit 1; }

stage=$(mktemp -d)
cp "$exe" "$stage/"
archive="kaeru-${TAG}-${TARGET}.zip"
(cd "$stage" && zip -q "$OLDPWD/dist/$archive" kaeru-mcp.exe)
rm -rf "$stage"

echo "==> dist/$archive"
echo
echo "Add it to SHA256SUMS alongside the other targets:"
echo "    (cd dist && sha256sum $archive >> SHA256SUMS)"
