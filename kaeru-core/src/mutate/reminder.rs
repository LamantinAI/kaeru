//! `stamp_reminder_seen` — the one write behind scheduled surfacing (#90).
//!
//! A reminder is otherwise a pure read: [`due_reminders`] computes what is due
//! from `after:` / `for:` / `seen:` tags, exactly as `open_tasks` computes
//! `overdue`. The single thing that cannot be derived is *when it was first
//! actually delivered* — and the window has to run from that, not from the
//! date, or a fortnight away from the keyboard eats the reminder that mattered.
//!
//! So the caller that renders a reminder stamps it. That is a read with a
//! write, which is uncomfortable exactly once and then has a precedent:
//! `hygiene::take_pending_report` already clears the headline it hands over,
//! in the same `awake` path, for the same reason — delivery is a fact only the
//! deliverer knows.
//!
//! [`due_reminders`]: crate::recall::due_reminders

use chrono::Utc;

use super::{
    ReassertRow, merge_tags, node_version_seconds, now_validity_seconds, read_node_now,
    reassert_node_now, retract_node_at,
};
use crate::errors::{Error, Result};
use crate::graph::NodeId;
use crate::graph::audit::write_audit;
use crate::store::Store;

/// Records that a reminder has been delivered, starting its insistence window.
///
/// Idempotent by intent: a node that already carries a `seen:` tag is left
/// alone, so the clock starts once and re-delivery inside the window does not
/// push the end of it further out. Callers may simply stamp everything they
/// rendered.
///
/// Returns `true` when a stamp was actually written.
pub fn stamp_reminder_seen(store: &Store, node_id: &NodeId) -> Result<bool> {
    let Some(current) = read_node_now(store, node_id)? else {
        // Delivered from a snapshot and gone since — nothing to stamp, and
        // nothing worth failing a re-entry over.
        return Ok(false);
    };
    if current.tags.iter().any(|t| t.starts_with("seen:")) {
        return Ok(false);
    }

    let today = Utc::now().format("%Y-%m-%d").to_string();
    // Only the `seen:` family is replaced. `after:` and `for:` are the
    // author's statement and stay verbatim; the read rule stops matching on
    // its own once the window closes, so nothing here needs to remove them.
    let tags = merge_tags(&current.tags, &["seen:"], vec![format!("seen:{today}")]);

    // Re-assert first, retract second, same timestamp — the ordering
    // invariant every RMW rewrite in this module shares.
    let secs = node_version_seconds(store, node_id)?;
    reassert_node_now(
        store,
        node_id,
        ReassertRow {
            secs,
            type_: &current.type_,
            tier: &current.tier,
            name: &current.name,
            body: current.body.as_deref(),
            tags,
            visibility: &current.visibility,
            layer: &current.layer,
        },
    )?;
    retract_node_at(store, node_id, secs)?;

    write_audit(
        store.db_ref(),
        "reminder_seen",
        "system",
        std::slice::from_ref(node_id),
    )?;
    Ok(true)
}

/// Marks a node as not relevant until `after`, then insistent for
/// `window_days` — the write behind a capture that names a future moment.
///
/// Both arguments are required and there is no default for either, which is
/// the deliberate part. The precedent is `link`'s weight: across 6,003 calls
/// of real work the optional `strong=true` was passed in 0 of 1,262 links, so
/// every edge sat on its default and every weighted path ranked on nothing. A
/// default window would guarantee nobody ever chooses one, and "re-measure
/// this before quoting it" and "the certificate expires" want entirely
/// different answers.
///
/// Setting a reminder on a node that already has one replaces it — the tags
/// are the author's statement and the latest statement wins. The `seen:` stamp
/// is cleared with it, so re-scheduling gives the reminder a fresh window
/// rather than one already half spent.
///
/// A date in the past is accepted: it means "due now", which is a legitimate
/// thing to say and the only sensible reading of it.
pub fn set_reminder(store: &Store, node_id: &NodeId, after: &str, window_days: i64) -> Result<()> {
    let after = after.trim();
    if chrono::NaiveDate::parse_from_str(after, "%Y-%m-%d").is_err() {
        return Err(Error::Invalid(format!(
            "reminder date must be YYYY-MM-DD, got `{after}`"
        )));
    }
    if window_days <= 0 {
        return Err(Error::Invalid(format!(
            "reminder window must be at least one day, got {window_days} — it is how long the \
             reminder keeps appearing once it surfaces, and there is no default on purpose"
        )));
    }

    let current = read_node_now(store, node_id)?
        .ok_or_else(|| Error::NotFound(format!("node {node_id} not found at NOW")))?;
    let tags = merge_tags(
        &current.tags,
        &["after:", "for:", "seen:"],
        vec![format!("after:{after}"), format!("for:{window_days}d")],
    );

    let secs = now_validity_seconds();
    reassert_node_now(
        store,
        node_id,
        ReassertRow {
            secs,
            type_: &current.type_,
            tier: &current.tier,
            name: &current.name,
            body: current.body.as_deref(),
            tags,
            visibility: &current.visibility,
            layer: &current.layer,
        },
    )?;
    retract_node_at(store, node_id, secs)?;

    write_audit(
        store.db_ref(),
        "set_reminder",
        "system",
        std::slice::from_ref(node_id),
    )?;
    Ok(())
}
