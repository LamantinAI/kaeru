#!/usr/bin/env bash
# Fetches the vendored dependencies for this checkout's version and wires the
# workspace to build from them, with no network access.
#
#   ./contrib/offline/fetch-vendor.sh            # the version in Cargo.toml
#   ./contrib/offline/fetch-vendor.sh v0.7.0     # a specific release
#
# Run this on a machine that still has network. Afterwards the whole directory
# is self-contained: copy it to the machine that does not, and build there.
#
# It writes two things and touches nothing else:
#   vendor/              the crate sources
#   .cargo/config.toml   the source-replacement config, taken from the vendor
#                        repo rather than written here
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
  echo "       The vendor tree is only valid for the exact lock it was built" >&2
  echo "       against. Either check out $VERSION, or publish a vendor for" >&2
  echo "       this lock with contrib/offline/publish-vendor.sh." >&2
  exit 1
fi

rm -rf vendor
mv "$tmp/vendor-repo/vendor" vendor
mkdir -p .cargo

# The vendor repo's config.toml is `cargo vendor`'s own stdout, so its
# `directory` is the absolute path the *publishing* machine vendored into —
# a path that exists nowhere else. Point it at this checkout instead. Every
# other line is copied through untouched, including the replacement stanza
# for the pinned `graph_builder` git dependency, which is the reason the file
# is captured rather than written by hand.
#
# Rewriting here rather than only at publish time is deliberate: a published
# vendor tag is immutable, so this is also what makes already-released tags
# usable.
awk '/^directory = / { print "directory = \"vendor\""; next } { print }' \
  "$tmp/vendor-repo/config.toml" > .cargo/config.toml

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

  cargo build --release --offline -p kaeru-mcp

kaeru still needs a C++ toolchain: cozo builds RocksDB from source, and zstd
and lz4 from C. That is a property of kaeru, not of building offline — see
docs/offline-build.md for what each platform needs.
EOF
