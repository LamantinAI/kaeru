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

use cozo::{DataValue, DbInstance, ScriptMutability};

use crate::errors::{Error, Result};
use crate::graph::temporal::parse_validity;

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
    Migration {
        name: "0005_slot_occupant",
        up: m0005_slot_occupant,
    },
    Migration {
        name: "0006_initiative_hygiene",
        up: m0006_initiative_hygiene,
    },
    Migration {
        name: "0007_initiative_cloud",
        up: m0007_initiative_cloud,
    },
    Migration {
        name: "0008_supersedes_orientation",
        up: m0008_supersedes_orientation,
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

/// Adds `slot_occupant` — the per-initiative role registry that keeps a slot
/// (`handoff`, `entrypoint`, …) to exactly one live node.
fn m0005_slot_occupant(db: &DbInstance) -> Result<()> {
    if !relation_exists(db, "slot_occupant")? {
        db.run_script(
            ":create slot_occupant { initiative: String, slot: String => node_id: String, set_at: Float default now() }",
            BTreeMap::new(),
            ScriptMutability::Mutable,
        )?;
    }
    Ok(())
}

/// Adds `initiative_hygiene` — bookkeeping for the hygiene pass (when it last
/// ran, the node count it saw, and the report awaiting delivery).
fn m0006_initiative_hygiene(db: &DbInstance) -> Result<()> {
    if !relation_exists(db, "initiative_hygiene")? {
        db.run_script(
            ":create initiative_hygiene { initiative: String => last_run_at: Float default 0.0, nodes_at_last_run: Int default 0, pending_report: String? default null }",
            BTreeMap::new(),
            ScriptMutability::Mutable,
        )?;
    }
    Ok(())
}

/// Adds `initiative_cloud` — which clouds an initiative may be shared into.
///
/// Purely additive: an existing vault gains an empty relation, and an empty
/// set means "no restriction", so every initiative keeps behaving exactly as
/// it did. No existing row is read or rewritten.
fn m0007_initiative_cloud(db: &DbInstance) -> Result<()> {
    if !relation_exists(db, "initiative_cloud")? {
        db.run_script(
            ":create initiative_cloud { initiative: String, cloud: String }",
            BTreeMap::new(),
            ScriptMutability::Mutable,
        )?;
    }
    Ok(())
}

/// `0008` — one direction for `supersedes` (#93).
///
/// The type meant opposite things depending on who wrote it: `supersedes()`
/// and `occupy_slot` wrote old → new, while `mark_resolved`, `resolve_review`
/// and every agent calling `link a b supersedes` wrote new → old. The
/// surviving orientation is **`src` supersedes `dst`** — the reading of the
/// verb, and what live vaults are already full of — so this migration turns
/// the two primitive-written kinds around.
///
/// Only edges it can **attribute to a primitive** are touched, because an
/// agent-written edge is already right and flipping it would invert its
/// meaning. Attribution comes from the audit trail:
///
///   * `op = "supersedes"` carries `affected_refs = [old, new]`, which names
///     the stored edge `old → new` exactly;
///   * `op = "occupy_slot"` carries only the new holder, so the succession
///     edge is the one ending at that node written in the seconds before the
///     audit — `occupy_slot` writes the link, then the layer move, then the
///     audit, with nothing in between that waits on anything.
///
/// A pair that already has BOTH directions stored is left alone: something
/// wrote the other one deliberately, and `lint` reports it for a human.
///
/// The rows are rewritten in place rather than retracted and re-asserted.
/// The edge never changed — only the way it was encoded — so a read at a past
/// moment should see the correct direction too, and a retraction here would
/// instead tell every such reader that the succession stopped being true
/// today.
///
/// Idempotent: a second run finds no stored row under the old orientation.
fn m0008_supersedes_orientation(db: &DbInstance) -> Result<()> {
    // How long after its edge the `occupy_slot` audit may land. Generous: the
    // writes in between are two local Cozo scripts.
    const SLOT_AUDIT_WINDOW_SECS: f64 = 5.0;

    if !relation_exists(db, "edge")? || !relation_exists(db, "node")? {
        return Ok(());
    }

    // Every `supersedes` edge live at NOW, with the moment it was asserted.
    let live = db.run_script(
        r#"
        ?[src, dst, validity] := *edge{src, dst, edge_type, validity @ 'NOW'},
                                 edge_type = 'supersedes'
        "#,
        BTreeMap::new(),
        ScriptMutability::Immutable,
    )?;
    let mut edges: Vec<(String, String, f64)> = Vec::new();
    for row in &live.rows {
        let (Some(src), Some(dst)) = (
            row.first().and_then(|v| v.get_str()),
            row.get(1).and_then(|v| v.get_str()),
        ) else {
            continue;
        };
        let secs = parse_validity(row.get(2)).map(|(s, _)| s).unwrap_or(0.0);
        edges.push((src.to_string(), dst.to_string(), secs));
    }
    if edges.is_empty() {
        return Ok(());
    }
    let stored: BTreeSet<(String, String)> = edges
        .iter()
        .map(|(src, dst, _)| (src.clone(), dst.clone()))
        .collect();

    // The audit trail, which is what says who wrote an edge.
    let audits = db.run_script(
        r#"
        ?[validity, properties] := *node{id, type, validity, properties @ 'NOW'},
                                   type = 'audit_event'
        "#,
        BTreeMap::new(),
        ScriptMutability::Immutable,
    )?;
    let mut backwards: BTreeSet<(String, String)> = BTreeSet::new();
    let mut slot_events: Vec<(String, f64)> = Vec::new();
    for row in &audits.rows {
        let Some(DataValue::Json(payload)) = row.get(1) else {
            continue;
        };
        let op = payload.0.get("op").and_then(|v| v.as_str()).unwrap_or("");
        let refs: Vec<String> = payload
            .0
            .get("affected_refs")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        match op {
            "supersedes" if refs.len() == 2 => {
                backwards.insert((refs[0].clone(), refs[1].clone()));
            }
            "occupy_slot" if refs.len() == 1 => {
                let secs = parse_validity(row.first()).map(|(s, _)| s).unwrap_or(0.0);
                slot_events.push((refs[0].clone(), secs));
            }
            _ => {}
        }
    }
    for (holder, audited_at) in &slot_events {
        for (src, dst, secs) in &edges {
            if dst == holder && *secs <= *audited_at && audited_at - secs <= SLOT_AUDIT_WINDOW_SECS
            {
                backwards.insert((src.clone(), dst.clone()));
            }
        }
    }

    for (src, dst) in backwards {
        if !stored.contains(&(src.clone(), dst.clone())) {
            continue; // already flipped by an earlier run, or long retracted
        }
        if stored.contains(&(dst.clone(), src.clone())) {
            continue; // both directions exist — a human decides, `lint` says so
        }
        flip_edge(db, &src, &dst)?;
    }
    Ok(())
}

/// Rewrites every stored row of the `supersedes` edge `src → dst` as
/// `dst → src`, history included, and moves its initiative membership with
/// it. Used only by `0008`.
fn flip_edge(db: &DbInstance, src: &str, dst: &str) -> Result<()> {
    let mut read: BTreeMap<String, DataValue> = BTreeMap::new();
    read.insert("s".to_string(), DataValue::Str(src.into()));
    read.insert("d".to_string(), DataValue::Str(dst.into()));
    let rows = db.run_script(
        r#"
        ?[validity, weight, properties] :=
            *edge{src: $s, dst: $d, edge_type, validity, weight, properties},
            edge_type = 'supersedes'
        "#,
        read,
        ScriptMutability::Immutable,
    )?;

    for row in &rows.rows {
        let (secs, asserted) = parse_validity(row.first())?;
        let validity = DataValue::List(vec![DataValue::from(secs), DataValue::Bool(asserted)]);

        let mut rm: BTreeMap<String, DataValue> = BTreeMap::new();
        rm.insert("s".to_string(), DataValue::Str(src.into()));
        rm.insert("d".to_string(), DataValue::Str(dst.into()));
        rm.insert("v".to_string(), validity.clone());
        db.run_script(
            r#"
            ?[src, dst, edge_type, validity] <- [[$s, $d, 'supersedes', $v]]
            :rm edge {src, dst, edge_type, validity}
            "#,
            rm,
            ScriptMutability::Mutable,
        )?;

        let mut put: BTreeMap<String, DataValue> = BTreeMap::new();
        put.insert("s".to_string(), DataValue::Str(dst.into()));
        put.insert("d".to_string(), DataValue::Str(src.into()));
        put.insert("v".to_string(), validity);
        put.insert(
            "w".to_string(),
            row.get(1).cloned().unwrap_or(DataValue::from(1.0)),
        );
        put.insert(
            "p".to_string(),
            row.get(2).cloned().unwrap_or(DataValue::Null),
        );
        db.run_script(
            r#"
            ?[src, dst, edge_type, validity, weight, properties] <-
                [[$s, $d, 'supersedes', $v, $w, $p]]
            :put edge {src, dst, edge_type, validity => weight, properties}
            "#,
            put,
            ScriptMutability::Mutable,
        )?;
    }

    // The junction keys an edge by `src|dst|type`, so it has to turn too.
    if relation_exists(db, "edge_initiative")? {
        let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
        params.insert(
            "old".to_string(),
            DataValue::Str(format!("{src}|{dst}|supersedes").into()),
        );
        let memberships = db.run_script(
            "?[initiative] := *edge_initiative{initiative, edge_pk}, edge_pk = $old",
            params,
            ScriptMutability::Immutable,
        )?;
        for row in &memberships.rows {
            let Some(initiative) = row.first().and_then(|v| v.get_str()) else {
                continue;
            };
            let mut move_params: BTreeMap<String, DataValue> = BTreeMap::new();
            move_params.insert("init".to_string(), DataValue::Str(initiative.into()));
            move_params.insert(
                "old".to_string(),
                DataValue::Str(format!("{src}|{dst}|supersedes").into()),
            );
            move_params.insert(
                "new".to_string(),
                DataValue::Str(format!("{dst}|{src}|supersedes").into()),
            );
            db.run_script(
                r#"
                ?[initiative, edge_pk] <- [[$init, $new]]
                :put edge_initiative {initiative, edge_pk}
                "#,
                move_params.clone(),
                ScriptMutability::Mutable,
            )?;
            db.run_script(
                r#"
                ?[initiative, edge_pk] <- [[$init, $old]]
                :rm edge_initiative {initiative, edge_pk}
                "#,
                move_params,
                ScriptMutability::Mutable,
            )?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use cozo::{DbInstance, ScriptMutability};

    use super::{column_exists, index_exists, relation_exists, run_migrations};
    use crate::graph::EdgeType;
    use crate::graph::audit::write_audit;
    use crate::store::Store;
    use crate::{jot, link};

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
        // Counted against the registry, not a literal: appending a migration
        // is routine, and this assertion is about "each stamped exactly once".
        let registered = super::MIGRATIONS.len();
        assert_eq!(
            count(&db),
            registered,
            "every registered migration stamped once"
        );

        // Idempotent: a second pass is a no-op that must NOT reset the
        // backfilled values (the column_exists guard prevents a re-`:replace`).
        run_migrations(&db, false).expect("re-run is safe");
        assert_eq!(
            count(&db),
            registered,
            "still one stamp per migration after a re-run"
        );
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

    // ── 0008: one direction for `supersedes` (#93) ─────────────────────────

    /// Edges live at NOW as `(src, dst)` pairs of a given type.
    fn supersedes_edges(store: &Store) -> Vec<(String, String)> {
        let rows = store
            .run_read(
                r#"
                ?[src, dst] := *edge{src, dst, edge_type @ 'NOW'}, edge_type = 'supersedes'
                "#,
            )
            .expect("read edges");
        rows.rows
            .iter()
            .filter_map(|r| {
                Some((
                    r.first()?.get_str()?.to_string(),
                    r.get(1)?.get_str()?.to_string(),
                ))
            })
            .collect()
    }

    /// The legacy shape: `supersedes()` wrote old → new and left an audit
    /// event naming both. That edge turns around; an agent's own edge, which
    /// no audit attributes to a primitive, must not.
    #[test]
    fn m0008_flips_what_a_primitive_wrote_and_nothing_else() {
        let store = Store::open_in_memory().expect("open");
        let old = jot(&store, "the old truth").expect("jot");
        let new = jot(&store, "the new truth").expect("jot");
        link(&store, &old, &new, EdgeType::Supersedes).expect("legacy edge");
        write_audit(
            store.db_ref(),
            "supersedes",
            "system",
            &[old.clone(), new.clone()],
        )
        .expect("audit");

        let answer = jot(&store, "the answer").expect("jot");
        let question = jot(&store, "the question").expect("jot");
        link(&store, &answer, &question, EdgeType::Supersedes).expect("agent edge");

        super::m0008_supersedes_orientation(store.db_ref()).expect("migrate");

        let edges = supersedes_edges(&store);
        assert!(
            edges.contains(&(new.clone(), old.clone())),
            "the successor now points at what it replaced: {edges:?}"
        );
        assert!(
            !edges.contains(&(old.clone(), new.clone())),
            "and the old orientation is gone: {edges:?}"
        );
        assert!(
            edges.contains(&(answer, question)),
            "an edge no primitive wrote is already right: {edges:?}"
        );
    }

    /// `occupy_slot` wrote the succession edge and then an audit naming only
    /// the new holder, so the edge is identified by ending at that holder a
    /// moment earlier.
    #[test]
    fn m0008_flips_a_slot_succession() {
        let store = Store::open_in_memory().expect("open");
        let prev = jot(&store, "handoff-one").expect("jot");
        let next = jot(&store, "handoff-two").expect("jot");
        link(&store, &prev, &next, EdgeType::Supersedes).expect("legacy edge");
        write_audit(store.db_ref(), "occupy_slot", "system", &[next.clone()]).expect("audit");

        super::m0008_supersedes_orientation(store.db_ref()).expect("migrate");

        assert_eq!(
            supersedes_edges(&store),
            vec![(next, prev)],
            "the new holder supersedes the old one"
        );
    }

    /// Both orientations stored means somebody wrote the second one
    /// deliberately. The migration does not choose between them — `lint` puts
    /// the pair in front of a human.
    #[test]
    fn m0008_leaves_a_pair_that_has_both_directions() {
        let store = Store::open_in_memory().expect("open");
        let old = jot(&store, "v1").expect("jot");
        let new = jot(&store, "v2").expect("jot");
        link(&store, &old, &new, EdgeType::Supersedes).expect("legacy");
        link(&store, &new, &old, EdgeType::Supersedes).expect("the other way");
        write_audit(
            store.db_ref(),
            "supersedes",
            "system",
            &[old.clone(), new.clone()],
        )
        .expect("audit");

        super::m0008_supersedes_orientation(store.db_ref()).expect("migrate");

        let mut edges = supersedes_edges(&store);
        edges.sort();
        let mut expected = vec![(old.clone(), new.clone()), (new, old)];
        expected.sort();
        assert_eq!(edges, expected, "both survive, untouched");
    }

    /// A vault written by THIS build already writes new → old, so the pass
    /// has nothing to do — and running it twice must not undo its own work.
    #[test]
    fn m0008_is_a_no_op_on_a_current_vault_and_idempotent() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("proj");
        let old = jot(&store, "the old truth").expect("jot");
        std::thread::sleep(std::time::Duration::from_millis(1100));
        let new = crate::supersedes(
            &store,
            &old,
            crate::NodeType::Episode,
            crate::Tier::Operational,
            "v2",
            "the new truth",
        )
        .expect("supersedes");
        let before = supersedes_edges(&store);
        assert_eq!(
            before,
            vec![(new.clone(), old.clone())],
            "written new → old"
        );

        super::m0008_supersedes_orientation(store.db_ref()).expect("first run");
        super::m0008_supersedes_orientation(store.db_ref()).expect("second run");

        assert_eq!(supersedes_edges(&store), before, "nothing moved");
    }

    /// The junction keys an edge by `src|dst|type`, so a flipped edge has to
    /// take its initiative membership with it or it leaves the scope.
    #[test]
    fn m0008_moves_the_initiative_membership_with_the_edge() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("proj");
        let old = jot(&store, "v1").expect("jot");
        let new = jot(&store, "v2").expect("jot");
        link(&store, &old, &new, EdgeType::Supersedes).expect("legacy");
        write_audit(
            store.db_ref(),
            "supersedes",
            "system",
            &[old.clone(), new.clone()],
        )
        .expect("audit");

        super::m0008_supersedes_orientation(store.db_ref()).expect("migrate");

        let rows = store
            .run_read("?[edge_pk] := *edge_initiative{initiative, edge_pk}, initiative = 'proj'")
            .expect("read junction");
        let keys: Vec<String> = rows
            .rows
            .iter()
            .filter_map(|r| r.first().and_then(|v| v.get_str()).map(String::from))
            .collect();
        assert!(
            keys.contains(&format!("{new}|{old}|supersedes")),
            "membership followed the edge: {keys:?}"
        );
        assert!(
            !keys.contains(&format!("{old}|{new}|supersedes")),
            "and the old key is gone: {keys:?}"
        );
    }
}
