//! Forward-only schema migrations.
//!
//! The substrate's schema evolves: relations get added (`chain_member`),
//! columns get added (`edge.weight`). A fresh vault is created at the latest
//! schema by [`crate::store`]'s `SCHEMA_STATEMENTS`, but an **existing** vault
//! opened by a newer binary would otherwise never pick those changes up —
//! `bootstrap_schema` only runs the full create when `node` is absent.
//!
//! This module closes that gap with a tiny migration runner modelled on the
//! classic "journal + ordered registry" pattern:
//!
//!   - `migration_journal { name => applied_at }` is the journal — one row per
//!     applied migration, exactly like a `__migrations__` collection.
//!   - [`MIGRATIONS`] is the ordered registry. Each entry has a unique,
//!     order-sorting `name` (zero-padded numeric prefix) and an `up` fn run
//!     once. **Append only; never reorder or rename** a shipped migration.
//!
//! Runner rules ([`run_migrations`]):
//!   - **Fresh vault** (`fresh = true`): the create-time schema is already
//!     current, so every registered migration is *baseline-stamped* as
//!     applied without running. This is what keeps schema-evolving
//!     migrations from firing against a schema that already includes them.
//!   - **Existing vault**: apply every registered migration whose name is not
//!     yet in the journal, in registry order, stamping each on success.
//!
//! A *legacy* vault — `node` present but `migration_journal` absent, created before
//! this runner existed — is not fresh, so all migrations run. To make that
//! safe, every migration `up` is itself **idempotent** (check-then-create):
//! the journal prevents needless re-runs, but correctness never depends on
//! it. Write new migrations the same way.
//!
//! Adding a column to an existing relation uses Cozo's `:replace` with the
//! new schema (the added column must carry a `default`, so stored rows
//! backfill): read the relation out, then `:replace <rel> { ...new schema }`
//! with the same rows — Cozo fills the added column from its default.

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use cozo::DbInstance;
use cozo::ScriptMutability;

use crate::errors::Error;
use crate::errors::Result;

/// One forward-only migration. `name` must be unique and sort in application
/// order; `up` must be idempotent (safe to run against a vault that already
/// carries the change).
struct Migration {
    name: &'static str,
    up: fn(&DbInstance) -> Result<()>,
}

/// The ordered migration registry. Append new migrations; never reorder or
/// rename existing entries (the `name` is the journal key forever).
const MIGRATIONS: &[Migration] = &[Migration {
    name: "0001_chain_member",
    up: m0001_chain_member,
}];

/// Applies pending migrations. `fresh` is `true` when the vault was just
/// created at the latest schema this build; in that case every migration is
/// stamped applied without running (baseline). Otherwise unapplied
/// migrations run in registry order.
pub(crate) fn run_migrations(db: &DbInstance, fresh: bool) -> Result<()> {
    ensure_journal(db)?;

    if fresh {
        for m in MIGRATIONS {
            stamp(db, m.name)?;
        }
        return Ok(());
    }

    let applied = applied_set(db)?;
    for m in MIGRATIONS {
        if applied.contains(m.name) {
            continue;
        }
        (m.up)(db).map_err(|e| {
            Error::SchemaBootstrap(format!("migration `{}` failed: {e:?}", m.name))
        })?;
        stamp(db, m.name)?;
    }
    Ok(())
}

// ── Journal helpers ────────────────────────────────────────────────────────

/// Creates the `migration_journal` relation if it does not yet exist.
fn ensure_journal(db: &DbInstance) -> Result<()> {
    if relation_exists(db, "migration_journal")? {
        return Ok(());
    }
    db.run_script(
        ":create migration_journal { name: String => applied_at: Float default now() }",
        BTreeMap::new(),
        ScriptMutability::Mutable,
    )
    .map_err(|e| Error::SchemaBootstrap(format!("create migration_journal: {e:?}")))?;
    Ok(())
}

/// Reads the set of already-applied migration names from the journal.
fn applied_set(db: &DbInstance) -> Result<BTreeSet<String>> {
    let rows = db.run_script(
        "?[name] := *migration_journal{name}",
        BTreeMap::new(),
        ScriptMutability::Immutable,
    )?;
    Ok(rows
        .rows
        .iter()
        .filter_map(|r| r.first().and_then(|v| v.get_str()).map(String::from))
        .collect())
}

/// Records a migration as applied. Idempotent — `:put` upserts.
fn stamp(db: &DbInstance, name: &str) -> Result<()> {
    let script = format!(
        r#"
        ?[name] <- [['{name}']]
        :put migration_journal {{name}}
        "#
    );
    db.run_script(&script, BTreeMap::new(), ScriptMutability::Mutable)
        .map_err(|e| Error::SchemaBootstrap(format!("stamp migration `{name}`: {e:?}")))?;
    Ok(())
}

// ── Introspection helpers (shared by idempotent migrations) ─────────────────

/// Whether a stored relation exists.
fn relation_exists(db: &DbInstance, name: &str) -> Result<bool> {
    let rows = db.run_script("::relations", BTreeMap::new(), ScriptMutability::Immutable)?;
    Ok(rows
        .rows
        .iter()
        .any(|r| r.first().and_then(|v| v.get_str()) == Some(name)))
}

/// Whether `relation` carries an index named `index` (short name, e.g.
/// `by_node` — Cozo lists it as `relation:by_node`).
fn index_exists(db: &DbInstance, relation: &str, index: &str) -> Result<bool> {
    let script = format!("::indices {relation}");
    let rows = db.run_script(&script, BTreeMap::new(), ScriptMutability::Immutable)?;
    let qualified = format!("{relation}:{index}");
    Ok(rows.rows.iter().any(|r| {
        r.first()
            .and_then(|v| v.get_str())
            .is_some_and(|n| n == index || n == qualified)
    }))
}

// ── Migrations ──────────────────────────────────────────────────────────────

/// `0001` — knowledge chains. Adds the `chain_member` relation and its
/// `by_node` index for vaults created before chains landed. Idempotent.
fn m0001_chain_member(db: &DbInstance) -> Result<()> {
    if !relation_exists(db, "chain_member")? {
        db.run_script(
            ":create chain_member { chain_id: String, position: Int => node_id: String }",
            BTreeMap::new(),
            ScriptMutability::Mutable,
        )?;
    }
    if !index_exists(db, "chain_member", "by_node")? {
        db.run_script(
            "::index create chain_member:by_node { node_id }",
            BTreeMap::new(),
            ScriptMutability::Mutable,
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::index_exists;
    use super::relation_exists;
    use super::run_migrations;
    use crate::store::Store;
    use cozo::DbInstance;
    use cozo::ScriptMutability;
    use std::collections::BTreeMap;

    /// A fresh `Store` open stamps every registered migration as applied
    /// (baseline) — none should be left pending.
    #[test]
    fn fresh_open_baseline_stamps_all_migrations() {
        let store = Store::open_in_memory().expect("open");
        let rows = store
            .run_read("?[name] := *migration_journal{name}")
            .expect("read journal");
        let names: Vec<String> = rows
            .rows
            .iter()
            .filter_map(|r| r.first().and_then(|v| v.get_str()).map(String::from))
            .collect();
        assert!(
            names.iter().any(|n| n == "0001_chain_member"),
            "fresh vault baseline-stamps 0001; journal = {names:?}"
        );
        // chain_member already exists from the create-time schema.
        assert!(relation_exists(store.db_ref(), "chain_member").unwrap());
    }

    /// Simulate a legacy vault: a bare Cozo db with only `node`, no
    /// `migration_journal` and no `chain_member`. Running migrations against
    /// it (fresh = false) must create the missing relation + index and stamp.
    #[test]
    fn legacy_vault_runs_pending_migration() {
        let db = DbInstance::new("mem", "", "").expect("mem db");
        // Minimal pre-chains schema: just enough to look non-fresh.
        db.run_script(
            ":create node { id: String => name: String }",
            BTreeMap::new(),
            ScriptMutability::Mutable,
        )
        .unwrap();
        assert!(!relation_exists(&db, "chain_member").unwrap(), "absent before");

        run_migrations(&db, false).expect("migrate legacy");

        assert!(relation_exists(&db, "chain_member").unwrap(), "created by 0001");
        assert!(index_exists(&db, "chain_member", "by_node").unwrap(), "index created");

        let journal = db
            .run_script(
                "?[name] := *migration_journal{name}",
                BTreeMap::new(),
                ScriptMutability::Immutable,
            )
            .unwrap();
        assert_eq!(journal.rows.len(), 1, "0001 stamped exactly once");

        // Idempotent: a second pass is a no-op and does not error.
        run_migrations(&db, false).expect("re-run is safe");
        let journal2 = db
            .run_script(
                "?[name] := *migration_journal{name}",
                BTreeMap::new(),
                ScriptMutability::Immutable,
            )
            .unwrap();
        assert_eq!(journal2.rows.len(), 1, "still stamped once");
    }

    /// The real upgrade scenario through RocksDB: a disk vault is created with
    /// a pre-chains schema (a `node` relation but no `chain_member` /
    /// `migration_journal`), closed, then reopened via [`Store::open`]. The
    /// reopen must detect the existing-but-legacy vault, run `0001`, and leave
    /// `chain_member` present — without wiping the row that was already there.
    #[test]
    fn disk_legacy_vault_upgrades_on_reopen() {
        use crate::new_node_id;
        use std::env;
        use std::fs;

        let path = env::temp_dir().join(format!("kaeru-mig-disk-{}", new_node_id()));

        // First open: hand-build a legacy-shaped vault directly on the engine,
        // bypassing the full bootstrap so `chain_member` is genuinely absent.
        {
            let db = DbInstance::new("rocksdb", path.to_string_lossy().as_ref(), "")
                .expect("open rocksdb");
            db.run_script(
                ":create node { id: String => name: String, body: String? }",
                BTreeMap::new(),
                ScriptMutability::Mutable,
            )
            .unwrap();
            db.run_script(
                "?[id, name, body] <- [['n1', 'legacy', null]] :put node {id => name, body}",
                BTreeMap::new(),
                ScriptMutability::Mutable,
            )
            .unwrap();
            assert!(!relation_exists(&db, "chain_member").unwrap(), "absent at create");
        }

        // Reopen through Store::open — bootstrap sees `node` present (legacy),
        // skips the create pass, and the migration runner upgrades it.
        {
            let store = Store::open(&path).expect("reopen disk vault");
            assert!(
                relation_exists(store.db_ref(), "chain_member").unwrap(),
                "0001 created chain_member on reopen"
            );
            let preserved = store
                .run_read("?[name] := *node{id, name}, id = 'n1'")
                .unwrap();
            assert_eq!(preserved.rows.len(), 1, "pre-existing node row survived");
        }

        let _ = fs::remove_dir_all(&path);
    }
}
