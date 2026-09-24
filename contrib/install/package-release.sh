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

# Everything that ships, for packaging / checksums / server.json.
ALL_TARGETS=(
    x86_64-unknown-linux-gnu
    aarch64-apple-darwin
)

# Only darwin is cross-compiled. linux-gnu is built NATIVELY inside a
# container — see `build_linux_in_container`.
TARGETS=(
    aarch64-apple-darwin
)

# Debian 12. Chosen for the glibc floor its toolchain produces (2.34), which
# covers Ubuntu 22.04 and everything newer. Bullseye would floor at 2.31 but
# cannot build `zstd-sys` with its compiler.
LINUX_IMAGE="${KAERU_LINUX_IMAGE:-rust:bookworm}"
LINUX_TARGET=x86_64-unknown-linux-gnu

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

# Packages one built binary: a tarball, and the same binary as an MCP Bundle.
#
# A .mcpb is a zip carrying the server plus a manifest, which is what lets a
# client install kaeru with nothing to download and nothing to build. The
# manifest runs the binary with `--stdio`, never bare: bare would start a second
# daemon over whichever one already owns the vault, and the substrate is
# single-writer — the loser fails on the RocksDB lock.
package_target() {
    local target="$1" binary="$2" stage archive bundle
    stage=$(mktemp -d)
    cp "$binary" "$stage/kaeru-mcp"

    archive="kaeru-${TAG}-${target}.tar.gz"
    tar -C "$stage" -czf "$DIST/$archive" kaeru-mcp

    bundle="kaeru-mcp-${TAG}-${target}.mcpb"
    mkdir -p "$stage/server"
    mv "$stage/kaeru-mcp" "$stage/server/kaeru-mcp"
    sed -e "s/__VERSION__/${TAG#v}/" -e "s/__PLATFORM__/$(mcpb_platform "$target")/" \
        "$ROOT/contrib/mcpb/manifest.json" > "$stage/manifest.json"
    ( cd "$stage" && zip -qr "$DIST/$bundle" manifest.json server )
    rm -rf "$stage"

    echo "    -> dist/$archive"
    echo "    -> dist/$bundle"
}

# Builds the Linux binary natively, in a container, and NOT with zigbuild.
#
# This is the fix for a shipped regression, and the reasoning is worth keeping.
# 0.7.1 and 0.7.2 published a linux-gnu binary cross-compiled by cargo-zigbuild.
# It SIGSEGVs on startup when opening a vault that already has content — a NULL
# write deep in RocksDB's C++, no Rust panic, before `substrate ready`. A fresh
# vault opens fine, which is what made it survive testing. Verified on one
# machine, one vault: the published 0.7.1/0.7.2 binaries crash where a native
# build of the very same commit opens the vault and serves.
#
# The same symptom was seen once before and mis-attributed. The comment this
# replaces blamed "musl's small default pthread stack" for a segfault opening an
# existing vault, and the cure was to switch musl → gnu — while keeping the
# cross-compiler. The crash came back with glibc, so the cross-compiler was
# always the common factor, not the libc.
#
# Native build, old distro, no cross-linking of RocksDB's C++. The container is
# the same shape the Windows build already uses.
# The check that 0.7.1 and 0.7.2 did not have, and shipped a SIGSEGV for.
#
# Both of those binaries opened an empty directory happily, opened a vault
# they had created themselves happily, and died on a vault written by an
# earlier release — which is every user's vault. Testing the build against
# nothing is testing the one case that always worked.
#
# So: the PREVIOUS release creates and fills a vault, and the binary about to
# ship has to open it and read from it, in a plain Debian container with no
# toolchain in it. A crash here is exit 139 and a release that does not
# happen.
#
# `KAERU_SKIP_VAULT_CHECK=1` skips it, loudly. Do not use it to ship.
verify_linux_opens_an_existing_vault() {
    local new_binary="target-linux-release/release/kaeru-mcp"
    local work prev_tag prev_binary vault port
    port=9977

    if [[ "${KAERU_SKIP_VAULT_CHECK:-0}" == "1" ]]; then
        echo "!!  SKIPPING the existing-vault check — the binary is UNVERIFIED." >&2
        return 0
    fi

    prev_tag=$(git tag -l 'v*' --sort=-v:refname | grep -v "^${TAG}$" | head -n1)
    [[ -n "$prev_tag" ]] || { echo "!!  no previous tag to build a vault with" >&2; exit 1; }

    work=$(mktemp -d)
    vault="$work/vault"
    mkdir -p "$vault"
    echo "==> verifying $LINUX_TARGET against a vault written by $prev_tag"

    # The previous release's own linux asset, which is known to work.
    if ! gh release download "$prev_tag" \
            --pattern "kaeru-${prev_tag}-${LINUX_TARGET}.tar.gz" \
            --dir "$work" >/dev/null 2>&1; then
        echo "!!  could not download $prev_tag's linux asset — cannot verify" >&2
        exit 1
    fi
    tar -xzf "$work/kaeru-${prev_tag}-${LINUX_TARGET}.tar.gz" -C "$work"
    prev_binary=$(find "$work" -name kaeru-mcp -type f | head -n1)
    chmod +x "$prev_binary"

    # Fill the vault with the old binary: schema, a few nodes, an audit trail.
    _serve_and_write "$prev_binary" "$vault" "$port" "$prev_tag" write

    # Then open the same vault with what we are about to ship.
    _serve_and_write "$new_binary" "$vault" "$port" "$TAG" read

    echo "    ✓ $TAG opens and reads a vault written by $prev_tag"
    # After the verdict, never before it: a cleanup failure is not a reason
    # to fail a check that passed.
    rm -rf "$work" 2>/dev/null || true
}

# Runs a kaeru-mcp binary against `vault` in a clean container and either
# writes a few nodes into it or reads them back. Anything other than a clean
# start is fatal — a SIGSEGV shows up here as a container exit of 139.
_serve_and_write() {
    local binary="$1" vault="$2" port="$3" label="$4" mode="$5"
    local log cid
    log=$(mktemp)

    # As us, not as root: the container writes RocksDB files into a
    # directory this script has to clean up afterwards, and a release that
    # passes its own check must not then die on `rm`.
    cid=$(docker run -d --network host --user "$(id -u):$(id -g)" \
        -e KAERU_VAULT_PATH=/vault -e KAERU_MCP_LISTEN_PORT="$port" \
        -e RUST_LOG=info \
        -v "$(realpath "$binary")":/usr/local/bin/kaeru-mcp:ro \
        -v "$vault":/vault \
        debian:bookworm-slim /usr/local/bin/kaeru-mcp)

    local ready=0 i
    for i in $(seq 1 40); do
        if docker logs "$cid" 2>&1 | grep -q "substrate ready"; then ready=1; break; fi
        if [[ "$(docker inspect -f '{{.State.Running}}' "$cid")" != "true" ]]; then break; fi
        sleep 0.5
    done

    if [[ "$ready" != "1" ]]; then
        local code
        code=$(docker inspect -f '{{.State.ExitCode}}' "$cid")
        echo "!!  $label never reached 'substrate ready' (container exit $code):" >&2
        docker logs "$cid" 2>&1 | tail -20 >&2
        [[ "$code" == "139" ]] && echo "!!  exit 139 is a SIGSEGV — this is the 0.7.1 defect." >&2
        docker rm -f "$cid" >/dev/null 2>&1 || true
        exit 1
    fi

    local body result
    if [[ "$mode" == "write" ]]; then
        for n in 1 2 3; do
            body='{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"jot","arguments":{"body":"release check note '"$n"'","initiative":"release-check"}}}'
            _mcp_call "$port" "$body" >/dev/null
        done
    else
        body='{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"search","arguments":{"query":"release*","initiative":"release-check"}}}'
        result=$(_mcp_call "$port" "$body")
        if ! grep -q "release check note" <<<"$result"; then
            echo "!!  $label started but could not read the vault's existing content:" >&2
            echo "$result" | head -5 >&2
            docker rm -f "$cid" >/dev/null 2>&1 || true
            exit 1
        fi
    fi

    docker rm -f "$cid" >/dev/null 2>&1 || true
    rm -f "$log"
}

# One MCP tool call over the streamable-HTTP transport: initialize, then the
# call itself. rmcp refuses a request whose Accept does not include
# text/event-stream, which is why it is spelled out here.
_mcp_call() {
    local port="$1" body="$2" sid
    sid=$(curl -sS -D - -o /dev/null -X POST "http://127.0.0.1:$port/mcp" \
        -H 'Content-Type: application/json' \
        -H 'Accept: application/json, text/event-stream' \
        -d '{"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"release-check","version":"1"}}}' \
        | tr -d '\r' | awk -F': ' 'tolower($1)=="mcp-session-id"{print $2}')
    curl -sS -X POST "http://127.0.0.1:$port/mcp" \
        -H 'Content-Type: application/json' \
        -H 'Accept: application/json, text/event-stream' \
        -H "Mcp-Session-Id: $sid" \
        -d "$body"
}

build_linux_in_container() {
    echo "==> building $LINUX_TARGET natively in $LINUX_IMAGE"
    local out="target-linux-release/release/kaeru-mcp"
    local status=0

    # The old binary is removed INSIDE the container, which owns it: the
    # build runs as root (it apt-installs clang), so the host cannot delete
    # what it produced.
    docker run --rm --network host \
        -e HTTP_PROXY="${HTTP_PROXY:-}" -e HTTPS_PROXY="${HTTPS_PROXY:-}" \
        -e http_proxy="${http_proxy:-}" -e https_proxy="${https_proxy:-}" \
        -e NO_PROXY="${NO_PROXY:-localhost,127.0.0.1}" \
        -e CARGO_TERM_COLOR=never \
        -v "$ROOT:/io" -v "$HOME/.cargo/registry:/root/.cargo/registry" \
        -w /io \
        "$LINUX_IMAGE" \
        bash -c 'rm -f /io/target-linux-release/release/kaeru-mcp &&
                 apt-get update -qq && apt-get install -y -qq clang libclang-dev >/dev/null &&
                 cargo build --release --target-dir /io/target-linux-release -p kaeru-mcp --bin kaeru-mcp' \
        || status=$?

    # Two questions, and the container's own exit code is the honest answer
    # to the first. Judging freshness by mtime was tried and does not work:
    # cargo HARD-LINKS `release/kaeru-mcp` to `release/deps/kaeru-mcp-<hash>`,
    # so a relink after the file is deleted carries the old timestamp and a
    # perfectly good build reads as stale. It stopped two releases that were
    # fine before anyone noticed why.
    if [[ "$status" != "0" ]]; then
        echo "!!  the build container exited $status — do NOT ship this." >&2
        exit 1
    fi
    if [[ ! -f "$out" ]]; then
        echo "!!  $out does not exist — the container produced nothing." >&2
        exit 1
    fi

    # The glibc floor is a property of the toolchain, not a promise — read it
    # off the binary so the release notes can state the truth.
    local floor
    floor=$(objdump -T "$out" | grep -o 'GLIBC_[0-9.]*' | sort -V | tail -1)
    echo "    glibc floor: ${floor#GLIBC_}"

    package_target "$LINUX_TARGET" "$out"
}

build_linux_in_container
verify_linux_opens_an_existing_vault

for target in "${TARGETS[@]}"; do
    echo "==> building $target"

    # darwin needs `-p kaeru-mcp`: without it cargo unifies features across
    # the workspace, `kaeru-rig`'s rig-core pulls in aws-lc-rs, and aws-lc's
    # assembly makes zig's Mach-O linker fail with no message at all — only
    # "exit status 1" and an empty note. Scoped to the package we ship, that
    # dependency is not in the graph and kaeru-mcp reaches rustls via `ring`.
    #
    # NOTE: this target is still CROSS-COMPILED, which is what broke the Linux
    # binary in 0.7.1/0.7.2 (see `build_linux_in_container`). Nobody has opened
    # an existing vault with a released darwin build and confirmed it survives.
    # Until someone does, treat it as unverified.
    cargo zigbuild --release --target "$target" -p kaeru-mcp --bin kaeru-mcp

    package_target "$target" "target/$target/release/kaeru-mcp"
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
for target in "${ALL_TARGETS[@]}"; do
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
