#!/usr/bin/env bash
# Builds release tarballs for the targets we ship as prebuilts.
#
#   ./contrib/install/package-release.sh v0.1.0
#
# Output: dist/kaeru-v0.1.0-x86_64-unknown-linux-musl.tar.gz
#         dist/kaeru-v0.1.0-aarch64-apple-darwin.tar.gz
#         dist/SHA256SUMS
#
# Prerequisites (one-time):
#   cargo install cargo-zigbuild
#   rustup target add x86_64-unknown-linux-musl aarch64-apple-darwin
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
    x86_64-unknown-linux-musl
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

# Cross-linking RocksDB's C++ with zig's libc++ does not survive LTO: the
# musl link dies on a thousand `std::__cxx11::` symbols that are present but
# unmatched. Set here rather than left to the caller, because passing it is
# exactly the sort of thing that gets dropped between attempts — it did,
# and cost an hour of chasing the wrong cause.
export CARGO_PROFILE_RELEASE_LTO=false

ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$ROOT"

DIST="$ROOT/dist"
rm -rf "$DIST"
mkdir -p "$DIST"

for target in "${TARGETS[@]}"; do
    echo "==> building $target"

    # Scope differs per target, and both halves are load-bearing.
    #
    # darwin needs `-p kaeru-mcp`: without it cargo unifies features across
    # the workspace, `kaeru-rig`'s rig-core pulls in aws-lc-rs, and aws-lc's
    # assembly makes zig's Mach-O linker fail with no message at all — only
    # "exit status 1" and an empty note. Scoped to the package we ship, that
    # dependency is not in the graph and kaeru-mcp reaches rustls via `ring`.
    #
    # musl needs the opposite: scoped, the C++ side of cozorocks links
    # against libc++ while RocksDB was compiled for the GNU ABI, and the
    # link dies on a thousand `std::__cxx11::` symbols. The workspace-wide
    # feature set is what pulls libstdc++ in.
    #
    # Neither is elegant. Both are the difference between a binary and no
    # binary, and the alternative is discovering it again next release.
    scope=()
    [[ "$target" == aarch64-apple-darwin ]] && scope=(-p kaeru-mcp)

    cargo zigbuild --release --target "$target" "${scope[@]}" --bin kaeru-mcp

    stage=$(mktemp -d)
    cp "target/$target/release/kaeru-mcp" "$stage/"

    archive="kaeru-${TAG}-${target}.tar.gz"
    tar -C "$stage" -czf "$DIST/$archive" kaeru-mcp
    rm -rf "$stage"

    echo "    -> dist/$archive"
done

echo "==> SHA256SUMS"
( cd "$DIST" && sha256sum kaeru-*.tar.gz | tee SHA256SUMS )

echo
echo "Done. Upload contents of dist/ to the GitHub release for $TAG:"
ls -lh "$DIST"
