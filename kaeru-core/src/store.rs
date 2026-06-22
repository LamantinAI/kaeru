//! Store — wrapper around an embedded CozoDB instance.
//!
//! Owns the `DbInstance` and exposes a thin script-execution surface plus
//! schema bootstrap. Higher-level primitives (`write_episode`, `recall`,
//! `link`, `walk`, ...) sit in sibling modules and run scripts through here.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

use cozo::{DbInstance, NamedRows, ScriptMutability};

use crate::config::KaeruConfig;
use crate::errors::{Error, Result};

/// In-process handle to the kaeru substrate.
pub struct Store {
    db: DbInstance,
    /// Process-local current initiative. Set via `use_initiative`,
    /// read by primitives that scope by initiative (junction lookups).
    current_initiative: Mutex<Option<String>>,
    /// Serializes scope sessions ([`Store::scoped`]). The current initiative
    /// is process-local mutable state shared by every handle to this store;
    /// holding this guard across a set-initiative-then-operate sequence keeps
    /// two concurrent scoped callers (e.g. a `kaeru-rig` `spawn_blocking`
    /// pool) from interleaving each other's scope. Distinct from
    /// `current_initiative` so primitives can still read the scope without
    /// deadlocking against the guard.
    scope_guard: Mutex<()>,
    /// Tunable caps and defaults read by curator-API primitives.
    /// Captured at `Store` construction time so concurrent tests under
    /// `cargo test` can each pin their own config without racing on
    /// shared state.
    config: KaeruConfig,
}

impl Store {
    /// Opens an in-memory store with [`KaeruConfig::from_env`]. Useful
    /// for tests and ephemeral sessions; schema is bootstrapped on open.
    /// Returns `Err` if env-var parsing fails.
    pub fn open_in_memory() -> Result<Self> {
        Self::open_in_memory_with(KaeruConfig::from_env()?)
    }

    /// Opens an in-memory store with an explicit [`KaeruConfig`]. Used
    /// by tests that want deterministic caps regardless of environment.
    pub fn open_in_memory_with(config: KaeruConfig) -> Result<Self> {
        let db = DbInstance::new("mem", "", "")
            .map_err(|e| Error::Substrate(format!("failed to open cozo in-memory: {e:?}")))?;
        let store = Self {
            db,
            current_initiative: Mutex::new(None),
            scope_guard: Mutex::new(()),
            config,
        };
        store.bootstrap_schema()?;
        Ok(store)
    }

    /// Opens (or creates) a disk-backed store at `path`, using
    /// [`KaeruConfig::from_env`] for caps. The directory is created if
    /// it does not yet exist; Cozo's RocksDB engine then takes over.
    ///
    /// `path` here overrides whatever `vault_path` resolved to from env;
    /// callers that want the env-resolved vault should use
    /// [`Store::open_with_config`] with the result of `KaeruConfig::from_env`.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut config = KaeruConfig::from_env()?;
        config.vault_path = path.as_ref().to_path_buf();
        Self::open_with_config(config)
    }

    /// Opens (or creates) a disk-backed store at `config.vault_path`.
    /// The canonical disk constructor: adapters typically call
    /// `Store::open_with_config(KaeruConfig::from_env()?)`.
    pub fn open_with_config(config: KaeruConfig) -> Result<Self> {
        // Create the vault directory if missing. Cozo's RocksDB engine
        // expects the parent path to exist; we accept either an empty
        // directory or one with an existing Cozo dataset inside.
        if !config.vault_path.exists() {
            fs::create_dir_all(&config.vault_path).map_err(Error::Io)?;
        }

        let path_str = config.vault_path.to_string_lossy();
        let db = DbInstance::new("rocksdb", path_str.as_ref(), "").map_err(|e| {
            Error::Substrate(format!("failed to open cozo rocksdb at {path_str}: {e:?}"))
        })?;

        let store = Self {
            db,
            current_initiative: Mutex::new(None),
            scope_guard: Mutex::new(()),
            config,
        };
        store.bootstrap_schema()?;
        Ok(store)
    }

    /// Returns the runtime configuration this store was opened with.
    pub fn config(&self) -> &KaeruConfig {
        &self.config
    }

    /// Sets the current initiative for this Store handle. Subsequent
    /// session primitives default-filter to this initiative through the
    /// junction relations.
    pub fn use_initiative(&self, name: &str) {
        if let Ok(mut guard) = self.current_initiative.lock() {
            *guard = Some(name.to_string());
        }
    }

    /// Clears the current initiative (subsequent reads will need explicit
    /// `--initiative` arguments or fall back to cross-initiative).
    pub fn clear_initiative(&self) {
        if let Ok(mut guard) = self.current_initiative.lock() {
            *guard = None;
        }
    }

    /// Returns a copy of the current initiative name, if any.
    pub fn current_initiative(&self) -> Option<String> {
        self.current_initiative.lock().ok().and_then(|g| g.clone())
    }

    /// Runs `f` with the current initiative atomically set to `initiative`
    /// (`None` = cross-initiative), serialized against other `scoped` callers
    /// on this store.
    ///
    /// The current initiative is process-local state shared by every handle to
    /// the store, so a bare "set scope, then run a primitive" sequence races
    /// when two callers interleave (e.g. distinct `kaeru-rig` `KaeruMemory`
    /// handles with different initiatives sharing one `Arc<Store>` across the
    /// `spawn_blocking` pool). This holds an internal guard across the whole
    /// set-then-operate sequence so each scoped session is atomic. The guard
    /// is separate from `current_initiative`, so primitives inside `f` read the
    /// scope normally without deadlocking. Poisoning is recovered from — a
    /// panicked prior session does not wedge the store.
    pub fn scoped<T>(&self, initiative: Option<&str>, f: impl FnOnce(&Self) -> T) -> T {
        let _guard = self
            .scope_guard
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        match initiative {
            Some(name) => self.use_initiative(name),
            None => self.clear_initiative(),
        }
        f(self)
    }

    /// Runs a CozoScript that may mutate state.
    pub fn run(&self, script: &str) -> Result<NamedRows> {
        let rows = self
            .db
            .run_script(script, BTreeMap::new(), ScriptMutability::Mutable)?;
        Ok(rows)
    }

    /// Runs a read-only CozoScript.
    pub fn run_read(&self, script: &str) -> Result<NamedRows> {
        let rows = self
            .db
            .run_script(script, BTreeMap::new(), ScriptMutability::Immutable)?;
        Ok(rows)
    }

    /// Internal accessor for primitives that need to issue parametrised scripts.
    /// Restricted to crate-visibility so external code goes through the
    /// curator-API primitives, not raw scripts.
    pub(crate) fn db_ref(&self) -> &DbInstance {
        &self.db
    }

    /// Idempotent schema bootstrap.
    ///
    /// Passes: (1) create `node`/`edge`/junction relations and the regular
    /// indexes if `node` does not yet exist (a *fresh* vault — created at the
    /// latest schema); (2) ensure the FTS indexes exist regardless, so vaults
    /// opened before FTS landed pick them up; (3) run the migration runner,
    /// which baseline-stamps a fresh vault and forward-migrates an existing
    /// one (new relations / columns added since it was created). See
    /// [`crate::migrate`].
    fn bootstrap_schema(&self) -> Result<()> {
        let existing =
            self.db
                .run_script("::relations", BTreeMap::new(), ScriptMutability::Immutable)?;
        let node_present = existing.rows.iter().any(|row| {
            row.first()
                .and_then(|v| v.get_str())
                .is_some_and(|name| name == "node")
        });
        if !node_present {
            for stmt in SCHEMA_STATEMENTS {
                self.db
                    .run_script(stmt, BTreeMap::new(), ScriptMutability::Mutable)
                    .map_err(|e| {
                        Error::SchemaBootstrap(format!("statement: {stmt}\nerror: {e:?}"))
                    })?;
            }
        }
        self.ensure_fts_indexes()?;
        crate::migrate::run_migrations(&self.db, !node_present)?;
        Ok(())
    }

    /// Creates the FTS indexes on `node:fts_name` and `node:fts_body`
    /// if they don't yet exist. Idempotent.
    fn ensure_fts_indexes(&self) -> Result<()> {
        let listed = self.db.run_script(
            "::indices node",
            BTreeMap::new(),
            ScriptMutability::Immutable,
        )?;
        let mut existing: std::collections::HashSet<String> = std::collections::HashSet::new();
        for row in &listed.rows {
            if let Some(name) = row.first().and_then(|v| v.get_str()) {
                existing.insert(name.to_string());
            }
        }

        for (idx_name, stmt) in FTS_INDEX_STATEMENTS {
            if existing.contains(*idx_name) {
                continue;
            }
            self.db
                .run_script(stmt, BTreeMap::new(), ScriptMutability::Mutable)
                .map_err(|e| {
                    Error::SchemaBootstrap(format!("fts statement: {stmt}\nerror: {e:?}"))
                })?;
        }
        Ok(())
    }
}

/// Schema statements run in order during bootstrap.
///
/// `node` and `edge` carry `Validity` in their primary keys → bi-temporal
/// content history is native. Junction relations (`node_initiative`,
/// `edge_initiative`) deliberately do not — initiative membership is
/// append-only and read via prefix-scan.
const SCHEMA_STATEMENTS: &[&str] = &[
    r#"
    :create node {
        id: String,
        validity: Validity default [floor_to_second(now()), true] =>
        type: String,
        tier: String,
        name: String,
        body: String?,
        tags: [String]?,
        initiatives: [String]?,
        properties: Json?,
        visibility: String default 'local',
        layer: String default 'warm',
    }
    "#,
    r#"
    :create edge {
        src: String,
        dst: String,
        edge_type: String,
        validity: Validity default [floor_to_second(now()), true] =>
        weight: Float default 1.0,
        properties: Json?,
        dst_store: String default 'local',
    }
    "#,
    r#"
    :create node_initiative {
        initiative: String,
        node_id: String =>
        added_at: Float default now(),
    }
    "#,
    r#"
    :create edge_initiative {
        initiative: String,
        edge_pk: String =>
        added_at: Float default now(),
    }
    "#,
    // Initiatives carry a sticky `share_policy` (Gate 1). Not bi-temporal —
    // sharing governance is policy, not knowledge history (same rationale
    // as the junction relations).
    r#"
    :create initiative {
        name: String =>
        share_policy: String default 'private',
        set_at: Float default now(),
    }
    "#,
    // Knowledge chains: an ordered reasoning path. The chain itself is a
    // `Chain` node in `node`; this relation holds its ordered members. PK
    // (chain_id, position) gives ordered reads via prefix-scan; the by_node
    // index answers "which chains is this node in?".
    r#"
    :create chain_member {
        chain_id: String,
        position: Int =>
        node_id: String,
    }
    "#,
    // Session pins are persisted in the substrate so a process restart
    // restores the active window. Not bi-temporal: pin/unpin is a session-
    // level concern, not a knowledge-level one.
    r#"
    :create session_pin {
        node_id: String =>
        reason: String,
        pinned_at: Float default now(),
    }
    "#,
    "::index create node:by_name { name }",
    "::index create node:by_tier_type { tier, type }",
    "::index create node:by_layer { layer }",
    "::index create node:by_visibility { visibility }",
    "::index create edge:by_src { src }",
    "::index create edge:by_dst { dst }",
    "::index create edge:by_edge_type { edge_type }",
    "::index create edge:by_dst_store { dst_store }",
    "::index create chain_member:by_node { node_id }",
];

/// FTS indexes — created lazily after `SCHEMA_STATEMENTS` so vaults
/// opened before FTS landed pick them up on the next `Store::open`.
///
/// `Lowercase` is the only filter — `Stemmer('English')` is
/// language-specific and would mangle Russian / Japanese content. The
/// body index uses `extract_filter` to skip retraction rows where
/// `body` is null.
///
/// Tuple shape: `(index_name, create_statement)`. `index_name` is what
/// appears in `::indices node` and what `ensure_fts_indexes` checks
/// against to keep the operation idempotent.
const FTS_INDEX_STATEMENTS: &[(&str, &str)] = &[
    (
        "fts_name",
        r#"
        ::fts create node:fts_name {
            extractor: name,
            tokenizer: Simple,
            filters: [Lowercase]
        }
        "#,
    ),
    (
        "fts_body",
        r#"
        ::fts create node:fts_body {
            extractor: body,
            extract_filter: !is_null(body),
            tokenizer: Simple,
            filters: [Lowercase]
        }
        "#,
    ),
];

#[cfg(test)]
mod tests {
    use super::Store;

    #[test]
    fn open_and_query_trivial() {
        let store = Store::open_in_memory().expect("open mem store");
        let rows = store.run_read("?[a] := a = 1").expect("trivial query");
        assert_eq!(rows.rows.len(), 1, "expected one result row");
    }

    #[test]
    fn schema_creates_all_relations() {
        let store = Store::open_in_memory().expect("open mem store");
        let rows = store.run_read("::relations").expect("list relations");
        let names: Vec<String> = rows
            .rows
            .iter()
            .filter_map(|row| row.first().and_then(|v| v.get_str()).map(String::from))
            .collect();
        for expected in [
            "node",
            "edge",
            "node_initiative",
            "edge_initiative",
            "session_pin",
            "initiative",
            "chain_member",
        ] {
            assert!(
                names.iter().any(|n| n == expected),
                "{expected} relation must be present"
            );
        }
    }

    #[test]
    fn use_initiative_round_trip() {
        let store = Store::open_in_memory().expect("open");
        assert!(store.current_initiative().is_none());
        store.use_initiative("kaeru");
        assert_eq!(store.current_initiative().as_deref(), Some("kaeru"));
        store.clear_initiative();
        assert!(store.current_initiative().is_none());
    }

    /// `scoped` sets the active initiative for the closure (and clears it for
    /// `None`). Serialization is exercised implicitly by every scoped caller;
    /// here we assert the scope is what the closure observes.
    #[test]
    fn scoped_sets_initiative_for_the_closure() {
        let store = Store::open_in_memory().expect("open");
        let seen = store.scoped(Some("alpha"), |s| s.current_initiative());
        assert_eq!(seen.as_deref(), Some("alpha"));
        let cleared = store.scoped(None, |s| s.current_initiative());
        assert!(cleared.is_none(), "None scope clears the initiative");
    }

    /// Empirical experiment: does `Validity` + `[String]?` column compose
    /// cleanly under retraction + re-insertion?
    ///
    /// Procedure:
    ///   t1 = 1000 — assert n1 with initiatives = ['a', 'b']
    ///   t2 = 2000 — retract n1, then re-assert with initiatives = ['a', 'c']
    /// Expectations:
    ///   `is_in('b', initiatives)` @ 1500 → 1 row
    ///   `is_in('b', initiatives)` @ 2500 → 0 rows
    ///   `is_in('c', initiatives)` @ 2500 → 1 row
    #[test]
    fn validity_with_list_column_empirical() {
        let store = Store::open_in_memory().expect("open");

        let insert_t1 = r#"
            ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
                [['n1', [1000.0, true], 'concept', 'archival', 'Test', null, null, ['a', 'b'], null]]
            :put node {id, validity => type, tier, name, body, tags, initiatives, properties}
        "#;
        store.run(insert_t1).expect("insert at t1");

        let retract_t2 = r#"
            ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
                [['n1', [2000.0, false], 'concept', 'archival', 'Test', null, null, null, null]]
            :put node {id, validity => type, tier, name, body, tags, initiatives, properties}
        "#;
        store.run(retract_t2).expect("retract at t2");

        let reinsert_t2 = r#"
            ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
                [['n1', [2000.0, true], 'concept', 'archival', 'Test', null, null, ['a', 'c'], null]]
            :put node {id, validity => type, tier, name, body, tags, initiatives, properties}
        "#;
        store.run(reinsert_t2).expect("re-insert at t2");

        let q_b_at_1500 = r#"
            ?[id] := *node{id, initiatives @ 1500.0}, is_in('b', initiatives)
        "#;
        let r1 = store.run_read(q_b_at_1500).expect("query b @ 1500");
        assert_eq!(r1.rows.len(), 1, "is_in('b', initiatives) @ 1500 → 1 row");

        let q_b_at_2500 = r#"
            ?[id] := *node{id, initiatives @ 2500.0}, is_in('b', initiatives)
        "#;
        let r2 = store.run_read(q_b_at_2500).expect("query b @ 2500");
        assert_eq!(r2.rows.len(), 0, "is_in('b', initiatives) @ 2500 → 0 rows");

        let q_c_at_2500 = r#"
            ?[id] := *node{id, initiatives @ 2500.0}, is_in('c', initiatives)
        "#;
        let r3 = store.run_read(q_c_at_2500).expect("query c @ 2500");
        assert_eq!(r3.rows.len(), 1, "is_in('c', initiatives) @ 2500 → 1 row");
    }
}
