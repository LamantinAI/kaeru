//! Session primitives: pin / unpin / active_window / awake.
//!
//! Pins are persisted in the `session_pin` substrate relation so that a
//! process restart restores the active window — sessions outlive process
//! lifetime, just like the rest of the graph. `awake` is the single call
//! an agent makes when re-entering a project: it returns the pinned set,
//! recently-written episodes, and the open-review queue in one bundle.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::graph::audit::write_audit;
use crate::errors::Result;
use crate::graph::NodeId;
use crate::recall::recent_episodes;
use crate::recall::under_review_pinned;
use crate::store::Store;

fn now_secs_f64() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as f64)
        .unwrap_or(0.0)
}

/// Adds `node_id` to the session pin set with a `reason` justifying its
/// place in the active window. Idempotent: re-pinning the same node
/// updates the reason and timestamp.
pub fn pin(store: &Store, node_id: &NodeId, reason: &str) -> Result<()> {
    let now = now_secs_f64();
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("nid".to_string(), DataValue::Str(node_id.clone().into()));
    params.insert("reason".to_string(), DataValue::Str(reason.into()));

    let script = format!(
        r#"
        ?[node_id, reason, pinned_at] <- [[$nid, $reason, {now}]]
        :put session_pin {{node_id => reason, pinned_at}}
        "#
    );
    store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Mutable)?;

    write_audit(store.db_ref(), "pin", "system", &[node_id.clone()])?;
    Ok(())
}

/// Removes `node_id` from the session pin set. No-op if it wasn't pinned.
pub fn unpin(store: &Store, node_id: &NodeId) -> Result<()> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("nid".to_string(), DataValue::Str(node_id.clone().into()));

    let script = r#"
        ?[node_id] <- [[$nid]]
        :rm session_pin {node_id}
    "#;
    store
        .db_ref()
        .run_script(script, params, ScriptMutability::Mutable)?;

    write_audit(store.db_ref(), "unpin", "system", &[node_id.clone()])?;
    Ok(())
}

/// Bundle returned by [`awake`] — the agent's restored working context
/// when re-entering a project.
#[derive(Debug, Clone)]
pub struct AwakenedContext {
    /// Initiative active on the `Store` at the moment `awake` was called.
    /// `None` if no initiative was selected via `use_initiative`.
    pub initiative: Option<String>,
    /// Persisted session pins, newest-first. See [`active_window`].
    pub pinned: Vec<NodeId>,
    /// Episodes whose latest assertion is within
    /// `config().awake_default_window_secs`, newest-first.
    pub recent: Vec<NodeId>,
    /// Nodes with inbound `contradicts` edges valid at NOW —
    /// the open-review queue from `mark_under_review`.
    pub under_review: Vec<NodeId>,
}

/// Composite session-restoration primitive. Single call an agent makes
/// when re-entering a project: returns the pinned set, recently-written
/// episodes (last 24h), and the open-review queue.
///
/// Read-only by design — `awake` does not write an audit event. The
/// agent's reaction to the returned context (e.g. pinning new nodes,
/// resolving reviews) writes its own audit trail through the underlying
/// mutation primitives.
pub fn awake(store: &Store) -> Result<AwakenedContext> {
    let window = store.config().awake_default_window_secs;
    Ok(AwakenedContext {
        initiative: store.current_initiative(),
        pinned: active_window(store)?,
        recent: recent_episodes(store, window)?,
        under_review: under_review_pinned(store)?,
    })
}

/// Returns currently-pinned node ids, ordered by `pinned_at` descending
/// (most-recently pinned first). Capped at `config().active_window_size`.
///
/// Currently pin-only — recently-touched nodes (e.g. derived from
/// audit-event affected_refs in the last few minutes) could be folded
/// in here later as a richer "active context" view.
pub fn active_window(store: &Store) -> Result<Vec<NodeId>> {
    let limit = store.config().active_window_size;
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            format!(
                r#"
                ?[node_id, pinned_at] := *session_pin{{node_id, pinned_at}},
                                          *node_initiative{{initiative, node_id}},
                                          initiative = $init
                :order -pinned_at
                :limit {limit}
                "#
            )
        }
        None => format!(
            r#"
            ?[node_id, pinned_at] := *session_pin{{node_id, pinned_at}}
            :order -pinned_at
            :limit {limit}
            "#
        ),
    };
    let rows = store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Immutable)?;
    let pins: Vec<NodeId> = rows
        .rows
        .iter()
        .filter_map(|row| row.first().and_then(|v| v.get_str()).map(String::from))
        .collect();
    Ok(pins)
}
