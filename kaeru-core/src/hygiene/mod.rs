//! Hygiene — the pass that keeps an initiative's memory from silting up.
//!
//! Memory rots in a specific way: the journal of "deployed this, fixed that"
//! accumulates in the layers that get injected into context, `core` grows
//! because nothing bounds it, and nodes that everything references sit in
//! `warm` where they are easy to miss. Cleaning that by hand is a weekly
//! chore, so it doesn't happen, so the graph lies.
//!
//! This module does it automatically, and stays safe by construction:
//!
//! * **It only moves layers.** No deletion, no rewriting of names or bodies,
//!   no merging, no touching `visibility` — the cloud fate of a node stays a
//!   human decision. Every action reverses with one `layer` call.
//! * **It never demotes `core` more than one step** — and the promise holds
//!   across passes, not only within one (#94). A misjudged `core` node lands
//!   in `hot`, still inside the window of recent nodes, not in an archive
//!   nobody opens, and it stays there for a full journal age before the
//!   archive rule may look at it: a layer move does not refresh the node's
//!   timestamp, so without that grace the very next pass took it the rest of
//!   the way.
//! * **It weighs a node by support, never by contact.** An inbound
//!   `supersedes`, `contradicts` or `falsifies` cancels a node; counting it
//!   as a reference made the edge that says "obsolete" the edge that kept it
//!   loaded (#92). A verdict — superseded or refuted — is a demotion reason
//!   in its own right, at any age.
//! * **It never promotes into `core`.** Growth of the uncapped, always-injected
//!   layer stays a deliberate act; the pass can only suggest by promoting to
//!   `hot`.
//! * **It skips pinned nodes.** A pin is a louder, more explicit "keep this in
//!   view" than any layer.
//! * **It re-checks before every write.** Candidates are collected once and
//!   applied in batches; an agent may have touched a node in between, and the
//!   agent's decision wins.
//!
//! Collection owns a read snapshot; each application batch owns a separate
//! write transaction. Reporting is published after commit, and callers can
//! make progress during the pause between batches.

mod apply;
mod collect;
mod report;
mod state;
#[cfg(test)]
mod tests;

use crate::{Result, Store};
pub use apply::apply_batch;
pub use collect::{HygieneAction, HygieneCandidate, collect};
pub use report::HygieneReport;
use state::read_pending_report;
pub use state::{
    HygieneState, core_count, due, node_count, record_run, state, take_pending_report,
};

/// Audit actor stamped on every move the pass makes.
pub const HYGIENE_ACTOR: &str = "hygiene";

/// Name prefix of the durable episode each pass writes (`hygiene-<init>-<ts>`).
///
/// The episode is written by the adapters, but the prefix lives here so the
/// read side can recognise the tools' own bookkeeping without hard-coding a
/// string an adapter owns — `reflect` was recommending that the agent settle
/// and link the sweeper's diary (#76).
pub const HYGIENE_EPISODE_PREFIX: &str = "hygiene-";

/// Runs a full pass for `initiative` when one is [`due`]: collect, apply in
/// batches of `hygiene_batch_size`, record the result.
///
/// Collection uses a read snapshot; each batch and final report use separate
/// write transactions. Other callers can proceed between batches. Request-local
/// scope frames can be nested; they do not provide mutation serialization.
///
/// `pause` runs between batches, outside a transaction. Returning `false` stops
/// cleanly at a batch boundary; the daemon wires this to its cancellation token
/// and yields between batches so waiting clients can make progress.
///
/// A pass stopped early is not a problem: candidates are predicates over the
/// current graph, not a stored to-do list, so the next pass simply re-derives
/// whatever is left.
///
/// Returns `None` when no pass was due.
pub fn run_pass(
    store: &Store,
    initiative: &str,
    pause: impl FnMut() -> bool,
) -> Result<Option<HygieneReport>> {
    run_pass_inner(store, initiative, pause, false)
}

/// [`run_pass`] without the due-check — the `hygiene force=true` path. Used
/// when a human asks for a sweep now rather than waiting for a trigger.
/// Everything else is identical, including transaction and batching discipline.
pub fn force_pass(
    store: &Store,
    initiative: &str,
    pause: impl FnMut() -> bool,
) -> Result<Option<HygieneReport>> {
    run_pass_inner(store, initiative, pause, true)
}

fn run_pass_inner(
    store: &Store,
    initiative: &str,
    mut pause: impl FnMut() -> bool,
    force: bool,
) -> Result<Option<HygieneReport>> {
    let Some((trigger, candidates, core_before)) = store.scoped(Some(initiative), |s| {
        s.db_ref().read_snapshot(|| {
            let trigger = match due(s, initiative)? {
                Some(reason) => reason,
                None if force => "forced".to_string(),
                None => return Ok(None),
            };
            Ok(Some((
                trigger,
                collect(s, initiative)?,
                core_count(s, initiative)?,
            )))
        })
    })?
    else {
        return Ok(None);
    };

    let mut report = HygieneReport {
        initiative: initiative.to_string(),
        trigger,
        core_before,
        core_ceiling: store.config().hygiene_core_trigger,
        ..Default::default()
    };

    let batch_size = store.config().hygiene_batch_size.max(1);
    for (index, batch) in candidates.chunks(batch_size).enumerate() {
        if index > 0 && !pause() {
            report.stopped_early = true;
            break;
        }
        // One short critical section per batch: this is the whole reason the
        // pass is batched at all.
        store.scoped(Some(initiative), |s| apply_batch(s, batch, &mut report))?;
    }

    store.scoped(Some(initiative), |s| {
        s.atomic(|s| -> Result<()> {
            report.core_after = core_count(s, initiative)?;
            let nodes_seen = node_count(s, initiative)?;
            // A pass that moved nothing has nothing to announce — and must not
            // clear a cue an earlier pass left that the agent hasn't collected yet.
            let cue = if report.applied() > 0 {
                Some(report.headline())
            } else {
                read_pending_report(s, initiative)?
            };
            record_run(s, initiative, nodes_seen, cue.as_deref())
        })
    })?;
    Ok(Some(report))
}
