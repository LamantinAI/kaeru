//! `recent_writes` — what was captured within a time window from now. Feeds
//! the session-restoration `awake` composite.
//!
//! It used to list `episode` nodes only, and that is the verb people reach
//! for to answer "did what I just write land?". A user who captured seven
//! `cite` nodes was told `0`, concluded the memory was not recording, and
//! went back to files (#101). The description was accurate and the answer
//! was still a lie, because nothing else answers that question.

use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use cozo::{DataValue, ScriptMutability};

use crate::errors::Result;
use crate::graph::NodeId;
use crate::graph::temporal::parse_validity;
use crate::hygiene::HYGIENE_EPISODE_PREFIX;
use crate::store::Store;

/// Returns the ids of nodes asserted within `window_seconds` of now,
/// ordered by validity descending (newest first). Capped at
/// `config().recent_episodes_cap`.
///
/// **Every captured type**, not just episodes: a reference, a claim, a task
/// and a jot are all things somebody wrote, and the question this verb
/// answers is "what landed?" (#101). Two kinds are excluded, and neither is
/// a capture: `audit_event` rows, which every mutation writes, and the
/// durable episode the hygiene pass leaves behind on each run — kaeru's own
/// bookkeeping, which would otherwise crowd out the user's work in the
/// window they are looking at.
///
/// Feeds session restoration. Pair with `active_window` for the pinned set;
/// their union is the working-set view `awake` returns.
pub fn recent_writes(store: &Store, window_seconds: u64) -> Result<Vec<NodeId>> {
    // Anchor at NOW so retracted rows are skipped; bind validity so we can
    // compare its timestamp against the window cutoff in Rust. `:order
    // validity` is newest-first because Cozo wraps the Validity timestamp
    // in `Reverse<>` — smaller Validity sorts earlier, larger time later.
    //
    // When the store carries a current initiative, the query joins
    // `node_initiative` so only episodes attached to that initiative
    // surface; otherwise the read is cross-initiative.
    // Cap the scan in the query, not just in Rust: rows are newest-first, and
    // the loop below keeps at most `cap` within the window — so the `cap`
    // newest rows are the only candidates. Without `:limit` this sorted the
    // whole episode set on every `awake`.
    let cap = store.config().recent_episodes_cap;
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            format!(
                r#"
                ?[id, validity, name] := *node{{id, validity, type, name @ 'NOW'}},
                                    type != 'audit_event',
                                    *node_initiative{{initiative, node_id: id}},
                                    initiative = $init
                :order validity
                :limit {cap}
            "#
            )
        }
        None => format!(
            r#"
                ?[id, validity, name] := *node{{id, validity, type, name @ 'NOW'}},
                                    type != 'audit_event'
                :order validity
                :limit {cap}
            "#
        ),
    };
    let rows = store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Immutable)?;

    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let cutoff = now_secs.saturating_sub(window_seconds) as f64;

    let mut out: Vec<NodeId> = Vec::new();
    for row in &rows.rows {
        let (secs, asserted) = parse_validity(row.get(1))?;
        if !asserted || secs < cutoff {
            continue;
        }
        // The sweep's own diary is not something the user wrote.
        if row
            .get(2)
            .and_then(|v| v.get_str())
            .is_some_and(|n| n.starts_with(HYGIENE_EPISODE_PREFIX))
        {
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

#[cfg(test)]
mod tests {
    use super::recent_writes;
    use crate::store::Store;

    /// The defect a user reported: seven `cite` captures and `recent`
    /// answering nothing, because it listed `episode` rows only (#101). The
    /// verb answers "did what I write land?", and nothing else does.
    #[test]
    fn every_kind_of_capture_counts_as_recent() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("proj");

        let cited =
            crate::cite(&store, "a-source", Some("https://example.invalid"), "why").expect("cite");
        let jotted = crate::jot(&store, "a thought").expect("jot");
        let tasked = crate::write_task(&store, "something to do", None).expect("task");

        let recent = recent_writes(&store, 3600).expect("recent");
        for (what, id) in [
            ("reference", &cited),
            ("episode", &jotted),
            ("task", &tasked),
        ] {
            assert!(
                recent.contains(id),
                "the {what} is in the window: {recent:?}"
            );
        }
    }

    /// kaeru's own bookkeeping is not somebody's work: the hygiene pass
    /// writes a durable episode per run, and a window full of those hides
    /// the writes a user is looking for.
    #[test]
    fn the_sweeps_own_diary_is_not_a_capture() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("proj");
        let mine = crate::jot(&store, "my note").expect("jot");
        let diary = crate::write_episode(
            &store,
            crate::EpisodeKind::Observation,
            crate::Significance::Low,
            &format!("{}proj-1234", crate::hygiene::HYGIENE_EPISODE_PREFIX),
            "moved 3 nodes",
        )
        .expect("diary");
        crate::attach_node(&store, &diary, "proj").expect("attach");

        let recent = recent_writes(&store, 3600).expect("recent");
        assert!(recent.contains(&mine));
        assert!(
            !recent.contains(&diary),
            "the sweep's diary stays out: {recent:?}"
        );
    }
}
