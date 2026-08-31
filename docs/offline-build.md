# Building kaeru without network access

Some machines cannot reach crates.io: an air-gapped environment, a locked-down
corporate network, a customer site. This is how to build kaeru there.

The short version: on a machine that **has** network, fetch the dependencies
once; carry the directory across; build.

```sh
git clone https://github.com/LamantinAI/kaeru.git && cd kaeru
git checkout v0.7.1
./contrib/offline/fetch-vendor.sh          # ~600 MB, needs network
# …carry the whole directory to the offline machine…
cargo build --release --offline -p kaeru-mcp
```

---

## Where the dependencies live

In a separate repository: [`LamantinAI/kaeru-vendor`](https://github.com/LamantinAI/kaeru-vendor),
**one tag per kaeru release**, the vendor tree at the root.

Separate, rather than committed next to the source, for one reason: the tree is
about 600 MB and 460-odd crates. A repository carrying that for every release
would grow past the point where cloning it to fix a typo is reasonable, and
almost nobody building kaeru needs it. Fetching it is a deliberate act by the
few who do.

Tagged, rather than a directory per release, so that fetching one release costs
one release. `git clone --depth 1 --branch v0.7.1` downloads that tree and
nothing else; a directory layout would hand every builder the entire history of
every release forever.

## What `fetch-vendor.sh` does

1. Clones the vendor repo at the tag matching your checkout (or the tag you
   name).
2. **Checks that `Cargo.lock` matches.** A vendor tree is only valid for the
   exact lock it was built against — a mismatch fails later as a checksum error
   naming a crate nobody touched, so the script says it now, in terms of the
   actual mistake.
3. Puts `vendor/` in place and writes `.cargo/config.toml`.

Both are gitignored. A vendor tree is fetched, never committed alongside the
source.

## What you still need on the build machine

Rust alone is **not enough**, and this is a property of kaeru rather than of
building offline: cozo compiles RocksDB from C++ sources, and zstd and lz4 from
C. Offline just means you meet those requirements without a package manager to
help.

| | why | check |
|---|---|---|
| `rustc` / `cargo` 1.95+ | edition 2024 | `rustc -vV` |
| A C++ toolchain | RocksDB, zstd, lz4 | `cc --version` |
| `libclang` | `zstd-sys` generates bindings with bindgen | `clang --version`, or set `LIBCLANG_PATH` |

**Linux** — `build-essential` and `libclang-dev`, or the equivalent.
**macOS** — the Xcode command line tools carry all three.
**Windows** — MSVC Build Tools with the "Desktop development with C++"
workload, plus LLVM. `link.exe` from the same workload is what Rust itself
needs on an `*-msvc` target. `fetch-vendor.sh` is a shell script: run it from
Git Bash, which ships with Git for Windows, not from `cmd` or PowerShell.

## Windows: the one that will bite you

Git on Windows checks out text files with CRLF line endings unless told
otherwise. That changes the bytes of every vendored source, and every checksum
fails — with an error naming a crate you have never heard of and did not touch.

The vendor repository carries a `.gitattributes` with `* -text` for exactly
this reason. **Do not remove it**, and if you copy the tree around by some
means other than git, copy it verbatim.

If you see `checksum mismatch` on a fresh clone, this is why, before you
suspect anything else.

## Verifying an offline build is really offline

`.cargo/config.toml` sets `offline = true`, so cargo refuses to reach the
network rather than quietly succeeding because a cache happened to be warm.
To prove it end to end, build with an empty `CARGO_HOME`:

```sh
CARGO_HOME="$(mktemp -d)" cargo build --release --offline -p kaeru-mcp
```

If that succeeds, nothing came from the network.

---

## Publishing a vendor tree (maintainers)

Part of cutting a release, after the tag exists and from a clean checkout of
it:

```sh
git checkout v0.7.1
./contrib/offline/publish-vendor.sh v0.7.1
```

The script refuses to run against a dirty tree or a mismatched version,
because a vendor tag has to mean "the dependencies for exactly this source".

Two details it gets right that are easy to get wrong by hand:

- **`config.toml` is captured from `cargo vendor`'s own stdout**, not written
  by hand. Cargo prints the replacement stanzas it needs — including the one
  for kaeru's single git dependency (`graph_builder`, pinned to a fork). A
  hand-maintained config drifts silently the next time a dependency changes.
- **A published tag is immutable.** The script refuses to overwrite one. If a
  vendor tree is wrong, the fix is a new version, not a moved tag — someone has
  already built against the old one.
