//! Persisted pass bookkeeping, triggers and delivery acknowledgements.
use crate::mutate::now_validity_seconds;
use crate::{NodeId, Result, Store};
use cozo::{DataValue, ScriptMutability};
use std::collections::BTreeMap;

/// Bookkeeping row for an initiative: when the pass last ran and how many
/// nodes existed then.
#[derive(Debug, Clone, Default)]
pub struct HygieneState {
    pub last_run_at: f64,
    pub nodes_at_last_run: usize,
}

/// Reads the bookkeeping row, defaulting to "never ran".
pub fn state(store: &Store, initiative: &str) -> Result<HygieneState> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    let script = r#"
        ?[last_run_at, nodes_at_last_run] :=
            *initiative_hygiene{initiative, last_run_at, nodes_at_last_run},
            initiative = $init
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    Ok(rows
        .rows
        .first()
        .map(|r| HygieneState {
            last_run_at: r.first().and_then(|v| v.get_float()).unwrap_or(0.0),
            nodes_at_last_run: r.get(1).and_then(|v| v.get_int()).unwrap_or(0).max(0) as usize,
        })
        .unwrap_or_default())
}

/// Counts nodes belonging to `initiative`, excluding audit events (they are
/// bookkeeping, not knowledge, and would swamp the write-count trigger).
pub fn node_count(store: &Store, initiative: &str) -> Result<usize> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    let script = r#"
        ?[id] := *node{id, type @ 'NOW'}, type != 'audit_event',
                 *node_initiative{initiative, node_id: id}, initiative = $init
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    Ok(rows.rows.len())
}

/// Counts live `core` nodes in `initiative`.
pub fn core_count(store: &Store, initiative: &str) -> Result<usize> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    let script = r#"
        ?[id] := *node{id, layer, type @ 'NOW'}, layer = 'core', type != 'audit_event',
                 *node_initiative{initiative, node_id: id}, initiative = $init
    "#;
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    Ok(rows.rows.len())
}

/// Decides whether a pass is due, and says why. Any one of three conditions
/// fires it: enough writes since the last pass, a `core` layer grown past its
/// threshold, or simply enough time.
///
/// Read-only — safe to call on every `awake` and on every write.
pub fn due(store: &Store, initiative: &str) -> Result<Option<String>> {
    let cfg = store.config();
    let state = state(store, initiative)?;
    let nodes = node_count(store, initiative)?;

    let written = nodes.saturating_sub(state.nodes_at_last_run);
    if written >= cfg.hygiene_writes_trigger {
        return Ok(Some(format!("{written} writes since the last pass")));
    }

    // `>` rather than `>=`: the threshold is the ceiling the pass brings core
    // back to, so a core sitting exactly on it is at its target, not over it.
    // With `>=` a satisfied pass still met its own trigger and re-fired on
    // every write forever (#75).
    let core = core_count(store, initiative)?;
    if core > cfg.hygiene_core_trigger {
        return Ok(Some(format!(
            "core holds {core} nodes (threshold {})",
            cfg.hygiene_core_trigger
        )));
    }

    // A never-run initiative with too little in it to trip either threshold
    // is not "stale" — leave it alone until it has something to clean.
    if state.last_run_at > 0.0 {
        let age = now_validity_seconds() as f64 - state.last_run_at;
        if age >= cfg.hygiene_stale_after_secs as f64 {
            return Ok(Some(format!(
                "{} days since the last pass",
                (age / 86_400.0) as u64
            )));
        }
    }

    Ok(None)
}

/// Records that the pass unloaded `node_id` from `core`, so the archive rule
/// can leave it in the working set for a while (#94).
///
/// Overwrites any earlier stamp: what matters is the most recent move, and a
/// node demoted a second time starts its grace period again.
pub(super) fn stamp_demotion(store: &Store, node_id: &NodeId) -> Result<()> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(node_id.clone().into()));
    params.insert(
        "at".to_string(),
        DataValue::from(now_validity_seconds() as f64),
    );
    store.db_ref().run_script(
        r#"
        ?[node_id, at] <- [[$id, $at]]
        :put hygiene_demotion {node_id => at}
        "#,
        params,
        ScriptMutability::Mutable,
    )?;
    Ok(())
}

/// Drops demotion stamps whose grace period has run out — they answer
/// nothing after that, and the relation would otherwise grow one row per
/// node the pass has ever unloaded.
fn prune_demotions(store: &Store) -> Result<()> {
    let cutoff = now_validity_seconds() as f64 - store.config().hygiene_journal_age_secs as f64;
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("cutoff".to_string(), DataValue::from(cutoff));
    store.db_ref().run_script(
        r#"
        ?[node_id] := *hygiene_demotion{node_id, at}, at < $cutoff
        :rm hygiene_demotion {node_id}
        "#,
        params,
        ScriptMutability::Mutable,
    )?;
    Ok(())
}

/// Records that a pass ran: stamps the time, the node count it saw, and the
/// headline waiting for delivery.
pub fn record_run(
    store: &Store,
    initiative: &str,
    nodes_seen: usize,
    pending_report: Option<&str>,
) -> Result<()> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("init".to_string(), DataValue::Str(initiative.into()));
    params.insert(
        "at".to_string(),
        DataValue::from(now_validity_seconds() as f64),
    );
    params.insert("nodes".to_string(), DataValue::from(nodes_seen as i64));
    params.insert(
        "report".to_string(),
        match pending_report {
            Some(text) => DataValue::Str(text.into()),
            None => DataValue::Null,
        },
    );
    prune_demotions(store)?;
    let script = r#"
        ?[initiative, last_run_at, nodes_at_last_run, pending_report] <-
            [[$init, $at, $nodes, $report]]
        :put initiative_hygiene {initiative => last_run_at, nodes_at_last_run, pending_report}
    "#;
    store
        .db_ref()
        .run_script(script, params, ScriptMutability::Mutable)?;
    Ok(())
}

/// Reads the pending cue **without** clearing it, so an empty pass can carry an
/// undelivered one forward instead of overwriting it.
pub(super) fn read_pending_report(store: &Store, initiative: &str) -> Result<Option<String>> {
    let mut read = BTreeMap::new();
    read.insert("init".to_string(), DataValue::Str(initiative.into()));
    let script = r#"
        ?[pending_report] := *initiative_hygiene{initiative, pending_report},
            initiative = $init
    "#;
    let rows = store
        .db_ref()
        .run_script(script, read, ScriptMutability::Immutable)?;
    Ok(rows
        .rows
        .first()
        .and_then(|r| r.first())
        .and_then(|v| v.get_str())
        .map(String::from))
}

/// Takes the pending headline for `initiative`, clearing it. Called on the
/// next tool response — the one delivery channel that reliably reaches both
/// Claude Code and Codex (MCP notifications are received but not surfaced by
/// either client).
pub fn take_pending_report(store: &Store, initiative: &str) -> Result<Option<String>> {
    store.atomic(|store| take_pending_report_inner(store, initiative))
}

fn take_pending_report_inner(store: &Store, initiative: &str) -> Result<Option<String>> {
    let pending = read_pending_report(store, initiative)?;

    if pending.is_some() {
        // Clear ONLY the report. Reading it is not a pass: rewriting
        // `last_run_at` here would push the "N days since the last pass"
        // trigger forward every time an agent picked up a headline.
        let state = state(store, initiative)?;
        let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
        params.insert("init".to_string(), DataValue::Str(initiative.into()));
        params.insert("at".to_string(), DataValue::from(state.last_run_at));
        params.insert(
            "nodes".to_string(),
            DataValue::from(state.nodes_at_last_run as i64),
        );
        let clear = r#"
            ?[initiative, last_run_at, nodes_at_last_run, pending_report] <-
                [[$init, $at, $nodes, null]]
            :put initiative_hygiene {initiative => last_run_at, nodes_at_last_run, pending_report}
        "#;
        store
            .db_ref()
            .run_script(clear, params, ScriptMutability::Mutable)?;
    }
    Ok(pending)
}
