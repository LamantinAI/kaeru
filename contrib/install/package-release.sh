#!/usr/bin/env bash
# Builds release tarballs for the targets we ship as prebuilts.
#
#   ./contrib/install/package-release.sh v0.1.0
#
# Output: dist/kaeru-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
#         dist/kaeru-v0.1.0-aarch64-apple-darwin.tar.gz
#         dist/SHA256SUMS
#
# Prerequisites (one-time):
#   cargo install cargo-zigbuild
#   rustup target add x86_64-unknown-linux-gnu aarch64-apple-darwin
#   zig 0.13+ in PATH
#   For darwin targets: a macOS SDK extracted somewhere. zigbuild handles
#   the compiler, but Apple frameworks (Security, CoreFoundation, …) live
#   in the SDK and ring/rustls need them. One-time setup:
#
#     mkdir -p ~/.local/share/macos-sdk && cd ~/.local/share/macos-sdk
#     curl -fL -O https://github.com/joseluisq/macosx-sdks/releases/download/12.3/MacOSX12.3.sdk.tar.xz
#     tar -xf MacOSX12.3.sdk.tar.xz && rm MacOSX12.3.sdk.tar.xz
#
#   The script auto-discovers MacOSX*.sdk under that dir; override with SDKROOT.
#
# Ships only the client daemon `kaeru-mcp`. The shared `kaeru-cloud` server is
# distributed via Docker (one per team), not as a per-user prebuilt.
#
# Upload everything in dist/ as release assets. install.sh expects this
# exact archive layout (top-level kaeru-mcp inside the tar).

set -euo pipefail

TAG="${1:-}"
[[ -n "$TAG" ]] || { echo "usage: $0 <tag, e.g. v0.1.0>" >&2; exit 1; }

TARGETS=(
    x86_64-unknown-linux-gnu
    aarch64-apple-darwin
)

# Resolve SDKROOT for darwin cross-compile. zigbuild uses zig clang for the
# linker, but darwin frameworks (Security, CoreFoundation, …) live in the
# Apple SDK. Without SDKROOT the link step fails with "unable to find
# framework 'Security'". Pick the first MacOSX*.sdk under the local cache
# unless the caller already exported SDKROOT.
if [[ -z "${SDKROOT:-}" ]]; then
    sdk_candidate=$(ls -d "$HOME/.local/share/macos-sdk/MacOSX"*.sdk 2>/dev/null | head -n1 || true)
    if [[ -n "$sdk_candidate" ]]; then
        export SDKROOT="$sdk_candidate"
        echo "==> using SDKROOT=$SDKROOT"
    else
        echo "!!  SDKROOT not set and no SDK found under ~/.local/share/macos-sdk/" >&2
        echo "!!  darwin builds will fail. Either export SDKROOT or place a MacOSX*.sdk there." >&2
    fi
fi

# Cross-linking RocksDB's C++ through zig does not survive LTO on the cross
# targets — the link dies on a thousand unmatched `std::__cxx11::` symbols.
# Set here rather than left to the caller, because passing it is exactly the
# sort of thing that gets dropped between attempts — it did, and cost an hour
# of chasing the wrong cause.
export CARGO_PROFILE_RELEASE_LTO=false

# MCPB names platforms the way node does; rust names them by triple.
mcpb_platform() {
    case "$1" in
        *-apple-darwin)  echo darwin ;;
        *-linux-*)       echo linux  ;;
        *-windows-*)     echo win32  ;;
        *) echo "unknown MCPB platform for target $1" >&2; exit 1 ;;
    esac
}

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT"

DIST="$ROOT/dist"
rm -rf "$DIST"
mkdir -p "$DIST"

for target in "${TARGETS[@]}"; do
    echo "==> building $target"

    # darwin needs `-p kaeru-mcp`: without it cargo unifies features across
    # the workspace, `kaeru-rig`'s rig-core pulls in aws-lc-rs, and aws-lc's
    # assembly makes zig's Mach-O linker fail with no message at all — only
    # "exit status 1" and an empty note. Scoped to the package we ship, that
    # dependency is not in the graph and kaeru-mcp reaches rustls via `ring`.
    # It is not elegant, and the alternative is discovering it again next
    # release.
    #
    # linux-gnu builds workspace-wide (no scope) and links cleanly against the
    # GNU C++ ABI that RocksDB is compiled for. The old linux-musl target is
    # gone on purpose: even once it linked, the static-musl binary segfaulted
    # opening an EXISTING vault — musl's small default pthread stack overflows
    # in RocksDB recovery, while a fresh vault (no recovery) ran fine. The
    # Linux prebuilt is glibc/gnu now; it needs a glibc host (not Alpine/musl).
    scope=()
    [[ "$target" == aarch64-apple-darwin ]] && scope=(-p kaeru-mcp)

    cargo zigbuild --release --target "$target" "${scope[@]}" --bin kaeru-mcp

    stage=$(mktemp -d)
    cp "target/$target/release/kaeru-mcp" "$stage/"

    archive="kaeru-${TAG}-${target}.tar.gz"
    tar -C "$stage" -czf "$DIST/$archive" kaeru-mcp

    # …and the same binary as an MCP Bundle. A .mcpb is a zip carrying the
    # server plus a manifest, which is what lets a client install kaeru with
    # nothing to download and nothing to build.
    #
    # The manifest runs the binary with `--stdio`, never bare. Bare would start
    # a second daemon over whichever one already owns the vault, and the
    # substrate is single-writer — the loser fails on the RocksDB lock. With
    # `--stdio` each client gets a relay of its own and the daemon stays one.
    bundle="kaeru-mcp-${TAG}-${target}.mcpb"
    mkdir -p "$stage/server"
    mv "$stage/kaeru-mcp" "$stage/server/kaeru-mcp"
    sed -e "s/__VERSION__/${TAG#v}/" -e "s/__PLATFORM__/$(mcpb_platform "$target")/" \
        "$ROOT/contrib/mcpb/manifest.json" > "$stage/manifest.json"
    ( cd "$stage" && zip -qr "$DIST/$bundle" manifest.json server )
    rm -rf "$stage"

    echo "    -> dist/$archive"
    echo "    -> dist/$bundle"
done

echo "==> SHA256SUMS"
# Bundles are summed alongside the tarballs: server.json carries a
# `fileSha256` for the .mcpb, and MCP clients verify it before installing.
( cd "$DIST" && sha256sum kaeru-*.tar.gz kaeru-*.mcpb | tee SHA256SUMS )

# The registry entry, generated rather than kept by hand: it carries a
# download URL and a SHA-256 per bundle, and both change every release. A
# hand-edited server.json is a server.json that publishes last release's hash.
echo "==> server.json"
REL="https://github.com/LamantinAI/kaeru/releases/download/$TAG"
packages=""
for target in "${TARGETS[@]}"; do
    bundle="kaeru-mcp-${TAG}-${target}.mcpb"
    sum=$(cd "$DIST" && sha256sum "$bundle" | cut -d' ' -f1)
    [[ -n "$packages" ]] && packages="$packages,"
    packages="$packages
    {
      \"registryType\": \"mcpb\",
      \"identifier\": \"$REL/$bundle\",
      \"fileSha256\": \"$sum\",
      \"transport\": { \"type\": \"streamable-http\", \"url\": \"http://127.0.0.1:9876/mcp\" }
    }"
done
sed -e "s/__VERSION__/${TAG#v}/" "$ROOT/contrib/mcpb/server.json.template" \
    | python3 -c "import sys; sys.stdout.write(sys.stdin.read().replace('__PACKAGES__', '''$packages'''))" \
    > "$DIST/server.json"
echo "    -> dist/server.json"

echo
echo "Publish to the MCP registry (maintainer, once per release):"
echo "    mcp-publisher login github && mcp-publisher publish dist/server.json"
echo
echo "Done. Upload contents of dist/ to the GitHub release for $TAG:"
ls -lh "$DIST"
