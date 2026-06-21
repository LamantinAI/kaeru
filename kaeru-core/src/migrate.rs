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

use std::collections::{BTreeMap, BTreeSet};

use cozo::{DbInstance, ScriptMutability};

use crate::errors::{Error, Result};

/// One forward-only migration. `name` must be unique and sort in application
/// order; `up` must be idempotent (safe to run against a vault that already
/// carries the change).
struct Migration {
    name: &'static str,
    up: fn(&DbInstance) -> Result<()>,
}

/// The ordered migration registry. Append new migrations; never reorder or
/// rename existing entries (the `name` is the journal key forever).
const MIGRATIONS: &[Migration] = &[
    Migration {
        name: "0001_chain_member",
        up: m0001_chain_member,
    },
    Migration {
        name: "0002_node_layer_visibility",
        up: m0002_node_layer_visibility,
    },
    Migration {
        name: "0003_edge_dst_store",
        up: m0003_edge_dst_store,
    },
    Migration {
        name: "0004_initiative",
        up: m0004_initiative,
    },
];

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
        (m.up)(db)
            .map_err(|e| Error::SchemaBootstrap(format!("migration `{}` failed: {e:?}", m.name)))?;
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

/// Whether a stored `relation` carries a column named `column`. `::columns`
/// lists one row per column with the column name in the first field.
fn column_exists(db: &DbInstance, relation: &str, column: &str) -> Result<bool> {
    let script = format!("::columns {relation}");
    let rows = db.run_script(&script, BTreeMap::new(), ScriptMutability::Immutable)?;
    Ok(rows
        .rows
        .iter()
        .any(|r| r.first().and_then(|v| v.get_str()) == Some(column)))
}

/// Runs a mutating script with no params — a brevity wrapper for the many
/// schema statements the migrations issue.
fn run_mut(db: &DbInstance, script: &str) -> Result<()> {
    db.run_script(script, BTreeMap::new(), ScriptMutability::Mutable)?;
    Ok(())
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

/// `0002` — memory layers + local/cloud visibility. Adds the `visibility` and
/// `layer` columns (defaults `'local'` / `'warm'`) to `node` for vaults
/// created before those features landed, plus the `by_layer` / `by_visibility`
/// indexes.
///
/// Column backfill uses Cozo's `:replace`: read every stored row with the
/// *old* column set, then `:replace` with the full v0.2.0 schema so the two
/// new columns take their defaults. But Cozo refuses `:replace` on a relation
/// that has indices attached, and `node` always carries some (`by_name`,
/// `by_tier_type`, and the FTS indexes `ensure_fts_indexes` creates on every
/// open). So the order is: drop every node index → `:replace` → recreate the
/// full v0.2.0 index set. The `column_exists` guard is load-bearing: a second
/// `:replace` against a relation that already has the columns would reset live
/// values to the defaults. Idempotent: the drops/creates are existence-guarded.
fn m0002_node_layer_visibility(db: &DbInstance) -> Result<()> {
    if !column_exists(db, "node", "layer")? || !column_exists(db, "node", "visibility")? {
        for idx in ["by_name", "by_tier_type", "by_layer", "by_visibility"] {
            if index_exists(db, "node", idx)? {
                run_mut(db, &format!("::index drop node:{idx}"))?;
            }
        }
        for idx in ["fts_name", "fts_body"] {
            if index_exists(db, "node", idx)? {
                run_mut(db, &format!("::fts drop node:{idx}"))?;
            }
        }
        run_mut(
            db,
            r#"
            ?[id, validity, type, tier, name, body, tags, initiatives, properties] :=
                *node{id, validity, type, tier, name, body, tags, initiatives, properties}
            :replace node {
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
        )?;
    }
    ensure_node_indices(db)?;
    Ok(())
}

/// Recreates / ensures the full v0.2.0 index set on `node` (idempotent). The
/// FTS statements mirror `store::FTS_INDEX_STATEMENTS` — duplicated here so the
/// migration is self-contained and frozen at the schema it targets.
fn ensure_node_indices(db: &DbInstance) -> Result<()> {
    let regular = [
        ("by_name", "::index create node:by_name { name }"),
        (
            "by_tier_type",
            "::index create node:by_tier_type { tier, type }",
        ),
        ("by_layer", "::index create node:by_layer { layer }"),
        (
            "by_visibility",
            "::index create node:by_visibility { visibility }",
        ),
    ];
    for (name, stmt) in regular {
        if !index_exists(db, "node", name)? {
            run_mut(db, stmt)?;
        }
    }
    let fts = [
        (
            "fts_name",
            "::fts create node:fts_name { extractor: name, tokenizer: Simple, filters: [Lowercase] }",
        ),
        (
            "fts_body",
            "::fts create node:fts_body { extractor: body, extract_filter: !is_null(body), tokenizer: Simple, filters: [Lowercase] }",
        ),
    ];
    for (name, stmt) in fts {
        if !index_exists(db, "node", name)? {
            run_mut(db, stmt)?;
        }
    }
    Ok(())
}

/// `0003` — cloud soft-links. Adds the `dst_store` column (default `'local'`)
/// to `edge` for vaults created before the local/cloud split, plus its index.
/// Same drop-indices → `:replace` → recreate dance as
/// [`m0002_node_layer_visibility`]; `edge` carries no FTS indexes.
fn m0003_edge_dst_store(db: &DbInstance) -> Result<()> {
    if !column_exists(db, "edge", "dst_store")? {
        for idx in ["by_src", "by_dst", "by_edge_type", "by_dst_store"] {
            if index_exists(db, "edge", idx)? {
                run_mut(db, &format!("::index drop edge:{idx}"))?;
            }
        }
        run_mut(
            db,
            r#"
            ?[src, dst, edge_type, validity, weight, properties] :=
                *edge{src, dst, edge_type, validity, weight, properties}
            :replace edge {
                src: String,
                dst: String,
                edge_type: String,
                validity: Validity default [floor_to_second(now()), true] =>
                weight: Float default 1.0,
                properties: Json?,
                dst_store: String default 'local',
            }
            "#,
        )?;
    }
    let edge_indices = [
        ("by_src", "::index create edge:by_src { src }"),
        ("by_dst", "::index create edge:by_dst { dst }"),
        (
            "by_edge_type",
            "::index create edge:by_edge_type { edge_type }",
        ),
        (
            "by_dst_store",
            "::index create edge:by_dst_store { dst_store }",
        ),
    ];
    for (name, stmt) in edge_indices {
        if !index_exists(db, "edge", name)? {
            run_mut(db, stmt)?;
        }
    }
    Ok(())
}

/// `0004` — sticky per-initiative share policy. Creates the `initiative`
/// relation for vaults created before cloud sharing landed. Idempotent.
fn m0004_initiative(db: &DbInstance) -> Result<()> {
    if !relation_exists(db, "initiative")? {
        db.run_script(
            ":create initiative { name: String => share_policy: String default 'private', set_at: Float default now() }",
            BTreeMap::new(),
            ScriptMutability::Mutable,
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use cozo::{DbInstance, ScriptMutability};

    use super::{column_exists, index_exists, relation_exists, run_migrations};
    use crate::store::Store;

    /// A v0.1.0-shaped `node` (no `visibility` / `layer`), as a real legacy
    /// vault carries it. The column-backfill migrations read this exact set.
    const LEGACY_NODE: &str = r#"
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
        }
    "#;

    /// A v0.1.0-shaped `edge` (no `dst_store`).
    const LEGACY_EDGE: &str = r#"
        :create edge {
            src: String,
            dst: String,
            edge_type: String,
            validity: Validity default [floor_to_second(now()), true] =>
            weight: Float default 1.0,
            properties: Json?,
        }
    "#;

    fn run(db: &DbInstance, script: &str) {
        db.run_script(script, BTreeMap::new(), ScriptMutability::Mutable)
            .unwrap();
    }

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
        for expected in [
            "0001_chain_member",
            "0002_node_layer_visibility",
            "0003_edge_dst_store",
            "0004_initiative",
        ] {
            assert!(
                names.iter().any(|n| n == expected),
                "fresh vault baseline-stamps {expected}; journal = {names:?}"
            );
        }
        // The create-time schema already carries everything the migrations add.
        assert!(relation_exists(store.db_ref(), "chain_member").unwrap());
        assert!(relation_exists(store.db_ref(), "initiative").unwrap());
        assert!(column_exists(store.db_ref(), "node", "layer").unwrap());
    }

    /// Simulate a v0.1.0 vault: `node` + `edge` in their pre-v0.2.0 shape, no
    /// `migration_journal` / `chain_member` / `initiative`, and no
    /// `layer`/`visibility`/`dst_store` columns. Running migrations (fresh =
    /// false) must create the missing relations + indexes, backfill the new
    /// columns from their defaults, preserve the existing rows, and stamp every
    /// migration exactly once.
    #[test]
    fn legacy_vault_runs_pending_migrations() {
        let db = DbInstance::new("mem", "", "").expect("mem db");
        run(&db, LEGACY_NODE);
        run(&db, LEGACY_EDGE);
        // One real node + one real edge so backfill operates on actual rows.
        // Validity is supplied explicitly (as kaeru's own writes do) — omitting
        // it would force the `now()`-based default into a constant rule.
        run(
            &db,
            "?[id, validity, type, tier, name, body, tags, initiatives, properties] <- \
             [['n1', [1700000000.0, true], 'reference', 'archival', 'legacy', null, null, null, null]] \
             :put node {id, validity => type, tier, name, body, tags, initiatives, properties}",
        );
        run(
            &db,
            "?[src, dst, edge_type, validity, weight, properties] <- \
             [['n1', 'n1', 'relates_to', [1700000000.0, true], 1.0, null]] \
             :put edge {src, dst, edge_type, validity => weight, properties}",
        );

        assert!(
            !relation_exists(&db, "chain_member").unwrap(),
            "absent before"
        );
        assert!(
            !column_exists(&db, "node", "layer").unwrap(),
            "no layer before"
        );
        assert!(
            !column_exists(&db, "edge", "dst_store").unwrap(),
            "no dst_store before"
        );

        run_migrations(&db, false).expect("migrate legacy");

        // 0001 / 0004: new relations + indexes.
        assert!(
            relation_exists(&db, "chain_member").unwrap(),
            "0001 chain_member"
        );
        assert!(
            index_exists(&db, "chain_member", "by_node").unwrap(),
            "0001 index"
        );
        assert!(
            relation_exists(&db, "initiative").unwrap(),
            "0004 initiative"
        );
        // 0002 / 0003: backfilled columns + indexes.
        assert!(
            column_exists(&db, "node", "layer").unwrap(),
            "0002 node.layer"
        );
        assert!(
            column_exists(&db, "node", "visibility").unwrap(),
            "0002 node.visibility"
        );
        assert!(
            index_exists(&db, "node", "by_layer").unwrap(),
            "0002 by_layer"
        );
        assert!(
            column_exists(&db, "edge", "dst_store").unwrap(),
            "0003 edge.dst_store"
        );

        // Existing row survived and picked up the schema defaults.
        let node = db
            .run_script(
                "?[name, layer, visibility] := *node{id, name, layer, visibility @ 'NOW'}, id = 'n1'",
                BTreeMap::new(),
                ScriptMutability::Immutable,
            )
            .unwrap();
        assert_eq!(node.rows.len(), 1, "n1 preserved");
        assert_eq!(
            node.rows[0][1].get_str(),
            Some("warm"),
            "layer default backfilled"
        );
        assert_eq!(
            node.rows[0][2].get_str(),
            Some("local"),
            "visibility default backfilled"
        );
        let edge = db
            .run_script(
                "?[dst_store] := *edge{src, dst_store @ 'NOW'}, src = 'n1'",
                BTreeMap::new(),
                ScriptMutability::Immutable,
            )
            .unwrap();
        assert_eq!(
            edge.rows[0][0].get_str(),
            Some("local"),
            "dst_store default backfilled"
        );

        let count = |db: &DbInstance| {
            db.run_script(
                "?[name] := *migration_journal{name}",
                BTreeMap::new(),
                ScriptMutability::Immutable,
            )
            .unwrap()
            .rows
            .len()
        };
        assert_eq!(count(&db), 4, "all four migrations stamped once");

        // Idempotent: a second pass is a no-op that must NOT reset the
        // backfilled values (the column_exists guard prevents a re-`:replace`).
        run_migrations(&db, false).expect("re-run is safe");
        assert_eq!(count(&db), 4, "still four after re-run");
        let again = db
            .run_script(
                "?[layer] := *node{id, layer @ 'NOW'}, id = 'n1'",
                BTreeMap::new(),
                ScriptMutability::Immutable,
            )
            .unwrap();
        assert_eq!(
            again.rows[0][0].get_str(),
            Some("warm"),
            "value preserved across re-run"
        );
    }

    /// The real upgrade scenario through RocksDB: a disk vault is created with
    /// a pre-chains schema (a `node` relation but no `chain_member` /
    /// `migration_journal`), closed, then reopened via [`Store::open`]. The
    /// reopen must detect the existing-but-legacy vault, run `0001`, and leave
    /// `chain_member` present — without wiping the row that was already there.
    #[test]
    fn disk_legacy_vault_upgrades_on_reopen() {
        use std::{env, fs};

        use crate::new_node_id;

        let path = env::temp_dir().join(format!("kaeru-mig-disk-{}", new_node_id()));

        // First open: hand-build a v0.1.0-shaped vault directly on the engine,
        // bypassing the full bootstrap so the v0.2.0 schema is genuinely absent.
        {
            let db = DbInstance::new("rocksdb", path.to_string_lossy().as_ref(), "")
                .expect("open rocksdb");
            run(&db, LEGACY_NODE);
            run(&db, LEGACY_EDGE);
            run(
                &db,
                "?[id, validity, type, tier, name, body, tags, initiatives, properties] <- \
                 [['n1', [1700000000.0, true], 'reference', 'archival', 'legacy', null, null, null, null]] \
                 :put node {id, validity => type, tier, name, body, tags, initiatives, properties}",
            );
            assert!(
                !relation_exists(&db, "chain_member").unwrap(),
                "absent at create"
            );
            assert!(
                !column_exists(&db, "node", "layer").unwrap(),
                "no layer at create"
            );
        }

        // Reopen through Store::open — bootstrap sees `node` present (legacy),
        // skips the create pass, and the migration runner upgrades it.
        {
            let store = Store::open(&path).expect("reopen disk vault");
            assert!(
                relation_exists(store.db_ref(), "chain_member").unwrap(),
                "0001 created chain_member on reopen"
            );
            assert!(
                column_exists(store.db_ref(), "node", "layer").unwrap(),
                "0002 backfilled node.layer on reopen"
            );
            let preserved = store
                .run_read("?[name, layer] := *node{id, name, layer @ 'NOW'}, id = 'n1'")
                .unwrap();
            assert_eq!(preserved.rows.len(), 1, "pre-existing node row survived");
            assert_eq!(
                preserved.rows[0][1].get_str(),
                Some("warm"),
                "row backfilled with default layer"
            );
        }

        let _ = fs::remove_dir_all(&path);
    }
}
