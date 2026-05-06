//! Runtime configuration for `kaeru-core`.
//!
//! `KaeruConfig` collects every tunable cap, default, and path the
//! curator API reads at runtime. Defaults are calibrated for an LLM
//! agent's working set (5–15 items "under microscope") and platform
//! conventions; overrides come from `KAERU_*` environment variables
//! ([`KaeruConfig::from_env`]) or by constructing a config explicitly
//! and passing it to [`crate::Store::open_with_config`].
//!
//! Sources are merged through the `config` crate: built-in defaults
//! first, environment variables on top. Adapter crates (`kaeru-cli`,
//! `kaeru-mcp`, …) can layer their own file sources on top by chaining
//! more `add_source` calls before `try_deserialize`.
//!
//! Keeping the resolved config inside `Store` (rather than as
//! `lazy_static` / `OnceLock` globals) means tests under `cargo test`'s
//! parallel runner can each construct their own `Store` with their own
//! caps without racing on shared state.

use config::Config;
use config::Environment;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

use crate::errors::Result;

/// Tunables read by curator-API primitives. Every field has a default
/// suitable for typical agent sessions; overriding a field changes the
/// behaviour of the corresponding primitive without changing its
/// signature.
///
/// Env-variable mapping: `KAERU_<FIELD_NAME_UPPERCASE>` (e.g.
/// `KAERU_ACTIVE_WINDOW_SIZE`, `KAERU_VAULT_PATH`). Values are parsed
/// into the field's underlying type; malformed values fail
/// [`KaeruConfig::from_env`] rather than silently falling back to the
/// default.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KaeruConfig {
    /// Filesystem path to the on-disk substrate (RocksDB-backed Cozo).
    /// Defaults are platform-specific — see [`default_vault_path`].
    /// Override via `KAERU_VAULT_PATH`.
    pub vault_path: PathBuf,
    /// Soft cap on the number of nodes [`crate::active_window`] returns.
    pub active_window_size: usize,
    /// Soft cap on [`crate::recent_episodes`] results.
    pub recent_episodes_cap: usize,
    /// Default `recent_episodes` window used by [`crate::awake`], in seconds.
    pub awake_default_window_secs: u64,
    /// Soft cap on children returned by [`crate::summary_view`].
    pub summary_view_children_cap: usize,
    /// Maximum body characters per [`crate::NodeBrief`] excerpt.
    pub body_excerpt_chars: usize,
    /// Maximum hops for [`crate::recollect_provenance`].
    pub provenance_max_hops: u8,
    /// Default `max_hops` recommended for [`crate::walk`] callers.
    pub default_max_hops: u8,
    /// Hard cap enforced by [`crate::walk`] on `max_hops`. Beyond this
    /// the walk is rejected with `Error::Invalid`.
    pub max_hops_cap: u8,
}

impl KaeruConfig {
    /// Returns the built-in defaults, ignoring environment variables.
    /// Useful for tests that want a known baseline.
    pub fn defaults() -> Self {
        Self {
            vault_path: default_vault_path(),
            active_window_size: 15,
            recent_episodes_cap: 15,
            awake_default_window_secs: 24 * 60 * 60,
            summary_view_children_cap: 12,
            body_excerpt_chars: 240,
            provenance_max_hops: 5,
            default_max_hops: 2,
            max_hops_cap: 3,
        }
    }

    /// Reads `KAERU_*` environment variables on top of [`Self::defaults`].
    ///
    /// Defaults provide every field; `Environment::with_prefix("KAERU")`
    /// then overlays whichever variables are set. `try_parsing(true)`
    /// makes the env source coerce `"15"` into `usize`, `"86400"` into
    /// `u64`, etc; a value that fails to parse surfaces as
    /// `Error::Config` rather than corrupting the cap.
    pub fn from_env() -> Result<Self> {
        let resolved = Config::builder()
            .add_source(Config::try_from(&Self::defaults())?)
            .add_source(
                Environment::with_prefix("KAERU")
                    .try_parsing(true),
            )
            .build()?
            .try_deserialize()?;
        Ok(resolved)
    }
}

impl Default for KaeruConfig {
    fn default() -> Self {
        Self::defaults()
    }
}

/// Platform-specific default vault directory. Resolved at compile time
/// per `target_os`; the host's `$HOME` / `$XDG_DATA_HOME` /
/// `%LOCALAPPDATA%` are looked up at runtime.
///
/// Layout:
///   - Linux:   `$XDG_DATA_HOME/kaeru` (fallback `$HOME/.local/share/kaeru`)
///   - macOS:   `$HOME/Library/Application Support/ai.lamantin.kaeru`
///   - Windows: `%LOCALAPPDATA%\ai.lamantin.kaeru`
///                (fallback `%APPDATA%\ai.lamantin.kaeru`)
///   - other:   `./kaeru-vault` (BSDs, exotic targets — best-effort)
#[cfg(target_os = "linux")]
fn default_vault_path() -> PathBuf {
    let xdg = std::env::var("XDG_DATA_HOME")
        .ok()
        .filter(|s| !s.is_empty())
        .map(PathBuf::from);
    let base = xdg.unwrap_or_else(|| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        PathBuf::from(home).join(".local").join("share")
    });
    base.join("kaeru")
}

#[cfg(target_os = "macos")]
fn default_vault_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home)
        .join("Library")
        .join("Application Support")
        .join("ai.lamantin.kaeru")
}

#[cfg(target_os = "windows")]
fn default_vault_path() -> PathBuf {
    let base = std::env::var("LOCALAPPDATA")
        .ok()
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var("APPDATA")
                .ok()
                .filter(|s| !s.is_empty())
                .map(PathBuf::from)
        })
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("ai.lamantin.kaeru")
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn default_vault_path() -> PathBuf {
    PathBuf::from("./kaeru-vault")
}

#[cfg(test)]
mod tests {
    use super::KaeruConfig;
    use super::default_vault_path;

    #[test]
    fn defaults_are_known_constants() {
        let d = KaeruConfig::defaults();
        assert_eq!(d.active_window_size, 15);
        assert_eq!(d.recent_episodes_cap, 15);
        assert_eq!(d.awake_default_window_secs, 86_400);
        assert_eq!(d.summary_view_children_cap, 12);
        assert_eq!(d.body_excerpt_chars, 240);
        assert_eq!(d.provenance_max_hops, 5);
        assert_eq!(d.default_max_hops, 2);
        assert_eq!(d.max_hops_cap, 3);
        assert!(!d.vault_path.as_os_str().is_empty(), "vault_path must resolve");
    }

    #[test]
    fn from_env_with_no_overrides_matches_defaults() {
        // No KAERU_* vars set → result equals defaults. (Other tests in
        // the same process may set them; keep this test sensitive only
        // to fields we know are unset by checking equality with the
        // post-merge defaults rather than with hardcoded values.)
        let d = KaeruConfig::defaults();
        let from_env = KaeruConfig::from_env().expect("from_env");
        assert_eq!(d.active_window_size, from_env.active_window_size);
        assert_eq!(d.max_hops_cap, from_env.max_hops_cap);
    }

    /// Sanity: the platform-specific default path ends with the
    /// expected leaf ("kaeru" on Linux, "ai.lamantin.kaeru" on
    /// macOS/Windows, "kaeru-vault" on the fallback branch).
    #[test]
    fn default_vault_path_has_expected_leaf() {
        let p = default_vault_path();
        let leaf = p.file_name().and_then(|s| s.to_str()).unwrap_or("");

        #[cfg(target_os = "linux")]
        assert_eq!(leaf, "kaeru");
        #[cfg(any(target_os = "macos", target_os = "windows"))]
        assert_eq!(leaf, "ai.lamantin.kaeru");
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        assert_eq!(leaf, "kaeru-vault");
    }
}
