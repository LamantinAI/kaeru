//! `recent_episodes` — episodes whose latest assertion is within a time
//! window from now. Feeds the session-restoration `awake` composite.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::errors::Result;
use crate::graph::NodeId;
use crate::graph::temporal::parse_validity;
use crate::store::Store;

/// Returns episode node ids whose latest assertion timestamp is within
/// `window_seconds` from now, ordered by validity descending (newest first).
/// Capped at `config().recent_episodes_cap`.
///
/// Feeds session restoration: "what episodes did the agent write recently
/// in this initiative?" is the question this answers. Pair with
/// `active_window` for the pinned set; their union is the working-set
/// view `awake` returns.
pub fn recent_episodes(store: &Store, window_seconds: u64) -> Result<Vec<NodeId>> {
    // Anchor at NOW so retracted rows are skipped; bind validity so we can
    // compare its timestamp against the window cutoff in Rust. `:order
    // validity` is newest-first because Cozo wraps the Validity timestamp
    // in `Reverse<>` — smaller Validity sorts earlier, larger time later.
    //
    // When the store carries a current initiative, the query joins
    // `node_initiative` so only episodes attached to that initiative
    // surface; otherwise the read is cross-initiative.
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
                ?[id, validity] := *node{id, validity, type @ 'NOW'}, type = 'episode',
                                    *node_initiative{initiative, node_id: id},
                                    initiative = $init
                :order validity
            "#
        }
        None => {
            r#"
                ?[id, validity] := *node{id, validity, type @ 'NOW'}, type = 'episode'
                :order validity
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let cutoff = now_secs.saturating_sub(window_seconds) as f64;

    let cap = store.config().recent_episodes_cap;
    let mut out: Vec<NodeId> = Vec::new();
    for row in &rows.rows {
        let (secs, asserted) = parse_validity(row.get(1))?;
        if !asserted || secs < cutoff {
            continue;
        }
        if let Some(id) = row.first().and_then(|v| v.get_str()).map(String::from) {
            out.push(id);
            if out.len() >= cap {
                break;
            }
        }
    }
    Ok(out)
}
