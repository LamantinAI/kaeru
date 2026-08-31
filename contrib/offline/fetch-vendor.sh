#!/usr/bin/env bash
# Fetches the vendored dependencies for this checkout's version and wires the
# workspace to build from them, with no network access.
#
#   ./contrib/offline/fetch-vendor.sh            # the version in Cargo.toml
#   ./contrib/offline/fetch-vendor.sh v0.7.1     # a specific release
#
# Run this on a machine that still has network. Afterwards the whole directory
# is self-contained: copy it to the machine that does not, and build there.
#
# It writes two things and touches nothing else:
#   vendor/              the crate sources
#   .cargo/config.toml   the source-replacement config, taken from the vendor
#                        repo rather than written here, with only `directory`
#                        repointed at the tree unpacked above
#
# Both are gitignored in the main repository — a vendor tree is fetched, never
# committed alongside the source.

set -euo pipefail

VENDOR_REPO="${KAERU_VENDOR_REPO:-https://github.com/LamantinAI/kaeru-vendor.git}"

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

VERSION="${1:-}"
if [[ -z "$VERSION" ]]; then
  VERSION="v$(grep -m1 '^version = ' Cargo.toml | cut -d'"' -f2)"
  echo "==> no version given, using the workspace's: $VERSION"
fi

if [[ -e vendor && ! -w vendor ]]; then
  echo "error: vendor/ exists and is not writable" >&2
  exit 1
fi

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

echo "==> fetching $VERSION from $VENDOR_REPO"
# --depth 1 on the tag: one release's tree, not the history of every release.
if ! git clone --quiet --depth 1 --branch "$VERSION" "$VENDOR_REPO" "$tmp/vendor-repo" 2>"$tmp/err"; then
  echo "error: could not fetch $VERSION" >&2
  sed 's/^/       /' "$tmp/err" >&2
  echo >&2
  echo "       Tags that exist:" >&2
  git ls-remote --tags "$VENDOR_REPO" 2>/dev/null \
    | sed 's|.*refs/tags/|       |' | grep -v '\^{}' | sort -V | tail -10 >&2
  exit 1
fi

# The lock in the vendor tag and the lock here must agree, or the build fails
# on a checksum mismatch that names a crate nobody touched. Say so now, in
# terms of the actual mistake, rather than letting cargo say it later.
if ! cmp -s Cargo.lock "$tmp/vendor-repo/Cargo.lock"; then
  echo "error: Cargo.lock here does not match the one $VERSION was vendored from." >&2
  # Same content, different line endings is a different mistake with a
  # different fix, and on Windows it is by far the likely one. Say which.
  if cmp -s <(tr -d '\r' < Cargo.lock) \
            <(tr -d '\r' < "$tmp/vendor-repo/Cargo.lock"); then
    echo "       The two locks are the same apart from line endings, so this" >&2
    echo "       checkout was converted to CRLF — Git for Windows does that by" >&2
    echo "       default. Re-clone with core.autocrlf=false:" >&2
    echo >&2
    echo "         git -c core.autocrlf=false clone <url>" >&2
    exit 1
  fi
  echo "       The vendor tree is only valid for the exact lock it was built" >&2
  echo "       against. Either check out $VERSION, or publish a vendor for" >&2
  echo "       this lock with contrib/offline/publish-vendor.sh." >&2
  exit 1
fi

rm -rf vendor
mv "$tmp/vendor-repo/vendor" vendor
mkdir -p .cargo

# Everything in the published config is taken as cargo printed it, except
# `directory`: point it at the vendor tree that was just unpacked here.
#
# Vendor trees published before this was fixed — v0.7.0 among them — carry the
# publishing machine's absolute staging path, so copying the file verbatim
# makes the build fail with
#
#   error: failed to read root of directory source: …/staging/vendor
#
# against a path the user never chose. Published tags are immutable by design
# (publish-vendor.sh refuses to overwrite one), so rewriting it here is what
# keeps those releases usable. For trees published since, this is a no-op.
sed 's|^directory = .*|directory = "vendor"|' \
  "$tmp/vendor-repo/config.toml" > .cargo/config.toml

if ! grep -q '^directory = "vendor"$' .cargo/config.toml; then
  echo "error: the vendored config has no 'directory' line to point at the" >&2
  echo "       tree just unpacked — refusing to leave a config that would" >&2
  echo "       silently build against crates.io instead." >&2
  exit 1
fi

# Make offline the config's own property rather than something every caller
# has to remember to pass. `cargo vendor` does not print this stanza, so
# without it `--offline` on the command line is the only thing standing
# between a build and the network.
cat >> .cargo/config.toml <<'CONFIG'

[net]
offline = true
CONFIG

crates="$(find vendor -mindepth 1 -maxdepth 1 -type d | wc -l)"
size="$(du -sh vendor | cut -f1)"

cat <<EOF

ready — $crates crates, $size

  vendor/            the sources
  .cargo/config.toml source replacement + offline mode

This directory now builds with no network:

  cargo build --release -p kaeru-mcp

kaeru still needs a C++ toolchain: cozo builds RocksDB from source, and zstd
and lz4 from C. That is a property of kaeru, not of building offline — see
docs/offline-build.md for what each platform needs.
EOF
