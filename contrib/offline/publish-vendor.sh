#!/usr/bin/env bash
# Publishes this workspace's vendored dependencies to LamantinAI/kaeru-vendor
# as a tag matching the release.
#
#   ./contrib/offline/publish-vendor.sh v0.7.0
#
# Run it from a clean checkout of the tag you are releasing: the vendor tree
# must match that commit's Cargo.lock byte for byte, or an offline build fails
# on a checksum mismatch with no useful explanation.
#
# What lands in the vendor repo, at the root:
#   vendor/            every crate source `cargo vendor` produced
#   config.toml        the source-replacement stanza cargo PRINTED for it
#   Cargo.lock         the lock this vendor tree corresponds to
#   .gitattributes     `* -text` — see below, this one is not optional
#
# `config.toml` is captured from cargo's own stdout rather than written by
# hand. It carries the crates.io replacement *and* the stanza for the one git
# dependency (`graph_builder`, a pinned fork). Hand-maintaining that is how it
# silently drifts the next time a dependency changes.

set -euo pipefail

VERSION="${1:-}"
VENDOR_REPO="${KAERU_VENDOR_REPO:-https://github.com/LamantinAI/kaeru-vendor.git}"
WORKDIR="${KAERU_VENDOR_WORKDIR:-/tmp/kaeru-vendor-publish}"

if [[ -z "$VERSION" ]]; then
  echo "usage: $0 vX.Y.Z" >&2
  exit 2
fi
if [[ ! "$VERSION" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "error: version must look like v0.7.0, got '$VERSION'" >&2
  exit 2
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

# The tag in the vendor repo has to mean "the vendor for exactly this source",
# so publishing from a dirty tree would produce a lock nobody can reproduce.
if [[ -n "$(git status --porcelain)" ]]; then
  echo "error: working tree is dirty — publish from a clean checkout of $VERSION" >&2
  exit 1
fi

manifest_version="$(grep -m1 '^version = ' Cargo.toml | cut -d'"' -f2)"
if [[ "v$manifest_version" != "$VERSION" ]]; then
  echo "error: workspace is at v$manifest_version but you asked for $VERSION" >&2
  exit 1
fi

echo "==> vendoring dependencies (this takes a few minutes and ~600 MB)"
rm -rf "$WORKDIR"
mkdir -p "$WORKDIR/staging"
# Capture the config cargo prints; it is the only correct source of the
# replacement stanzas.
cargo vendor --versioned-dirs "$WORKDIR/staging/vendor" > "$WORKDIR/staging/config.toml"

cp Cargo.lock "$WORKDIR/staging/Cargo.lock"

# Windows checks out text files with CRLF unless told otherwise, which changes
# the bytes of every vendored source and fails every checksum. The colleague
# who first built kaeru offline lost real time to this; it is the single
# highest-value line in the repository.
cat > "$WORKDIR/staging/.gitattributes" <<'EOF'
# Vendored crate sources are checksummed byte-for-byte against Cargo.lock.
# Any line-ending translation changes those bytes and every build fails with
# "checksum mismatch" pointing at a crate nobody touched. Do not remove.
* -text
EOF

cat > "$WORKDIR/staging/README.md" <<EOF
# kaeru-vendor

Vendored Rust dependencies for [kaeru](https://github.com/LamantinAI/kaeru),
one tag per release, for building with no network access.

**This is not a fork.** There is no kaeru source here — only its dependencies.
The source lives in the main repository; this repo exists so that an offline
build does not need a 600 MB directory committed alongside it forever.

## Using it

From a kaeru checkout, on a machine that still has network:

\`\`\`sh
./contrib/offline/fetch-vendor.sh
\`\`\`

Then carry the whole directory to the machine that does not, and build there.
See \`docs/offline-build.md\` in the main repository for the full procedure,
including the C++ toolchain kaeru needs for RocksDB.

## Tags

Each tag is the complete vendor tree for the release of the same name, and
matches that release's \`Cargo.lock\` byte for byte. Fetch one release without
paying for the rest:

\`\`\`sh
git clone --depth 1 --branch $VERSION \\
  https://github.com/LamantinAI/kaeru-vendor.git
\`\`\`

## Contents

| | |
|---|---|
| \`vendor/\` | every crate source, including the one git dependency |
| \`config.toml\` | the source-replacement config, captured from \`cargo vendor\`'s own output |
| \`Cargo.lock\` | the lock this tree corresponds to |
| \`.gitattributes\` | \`* -text\` — without it Windows rewrites line endings and every checksum fails |

## Updating

Do not commit here by hand. The tree is produced by
\`contrib/offline/publish-vendor.sh\` in the main repository, from a clean
checkout of the tag being released.
EOF

echo "==> preparing the vendor repository"
git clone --quiet "$VENDOR_REPO" "$WORKDIR/repo" 2>/dev/null || {
  # A repository with no commits cannot be cloned; start one.
  mkdir -p "$WORKDIR/repo"
  git -C "$WORKDIR/repo" init --quiet -b main
  git -C "$WORKDIR/repo" remote add origin "$VENDOR_REPO"
}

if git -C "$WORKDIR/repo" rev-parse "$VERSION" >/dev/null 2>&1; then
  echo "error: tag $VERSION already exists in the vendor repo — a published" >&2
  echo "       vendor tree is immutable. Bump the version instead." >&2
  exit 1
fi

# Replace the tree wholesale: a vendor is a snapshot, not an accumulation.
find "$WORKDIR/repo" -mindepth 1 -maxdepth 1 ! -name .git -exec rm -rf {} +
cp -a "$WORKDIR/staging/." "$WORKDIR/repo/"

crates="$(find "$WORKDIR/repo/vendor" -mindepth 1 -maxdepth 1 -type d | wc -l)"
size="$(du -sh "$WORKDIR/repo/vendor" | cut -f1)"
echo "==> $crates crates, $size"

cd "$WORKDIR/repo"
git add -A
git -c user.name="GrumpyChubbyCat" -c user.email="lamantin-ai@yandex.ru" \
  commit --quiet --author="GrumpyChubbyCat <lamantin-ai@yandex.ru>" \
  -m "vendor: kaeru $VERSION — $crates crates" \
  -m "Produced by contrib/offline/publish-vendor.sh from a clean checkout of $VERSION. Matches that release's Cargo.lock byte for byte."
git tag -a "$VERSION" -m "kaeru $VERSION vendored dependencies"

echo "==> pushing"
git push --quiet origin main
git push --quiet origin "$VERSION"

echo
echo "published $VERSION → $VENDOR_REPO"
echo "  $crates crates, $size"
echo "  verify: ./contrib/offline/fetch-vendor.sh $VERSION && cargo build --offline --release"
