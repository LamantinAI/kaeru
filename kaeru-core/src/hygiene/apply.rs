//! Atomic application of one bounded candidate batch.
use super::state::stamp_demotion;
use super::{HYGIENE_ACTOR, HygieneAction, HygieneCandidate, HygieneReport};
use crate::mutate::set_layer_as;
use crate::{Layer, Result, Store};

/// Applies `candidates` in batches, re-checking each node against the layer
/// it had at collection time. A node whose layer changed meanwhile is skipped
/// — an agent touched it, and an agent's decision outranks the sweep.
///
/// The caller owns batching policy: this function performs one batch per call
/// so the daemon can release the write transaction and pause in between.
/// Report changes are published only after the entire batch commits.
pub fn apply_batch(
    store: &Store,
    batch: &[HygieneCandidate],
    report: &mut HygieneReport,
) -> Result<()> {
    let mut committed_report = report.clone();
    store.atomic(|store| apply_batch_inner(store, batch, &mut committed_report))?;
    *report = committed_report;
    Ok(())
}

fn apply_batch_inner(
    store: &Store,
    batch: &[HygieneCandidate],
    report: &mut HygieneReport,
) -> Result<()> {
    for candidate in batch {
        let current = match crate::mutate::read_node_now(store, &candidate.node_id)? {
            Some(node) => node,
            // Retracted between collection and application — skip rather than
            // resurrect it through `set_layer`'s historical fallback.
            None => {
                report.skipped += 1;
                continue;
            }
        };
        let current_layer = current.layer.parse::<Layer>().unwrap_or(Layer::Warm);
        if current_layer != candidate.from {
            report.skipped += 1;
            continue;
        }

        set_layer_as(store, &candidate.node_id, candidate.to, HYGIENE_ACTOR)?;
        match candidate.action {
            HygieneAction::Archive => report.archived += 1,
            HygieneAction::DemoteFromCore => {
                stamp_demotion(store, &candidate.node_id)?;
                report.demoted += 1;
            }
            HygieneAction::Promote => report.promoted += 1,
        }
        report.lines.push(format!(
            "{} {} → {}: {} ({})",
            candidate.action.as_str(),
            candidate.from.as_str(),
            candidate.to.as_str(),
            candidate.name,
            candidate.reason
        ));
    }
    Ok(())
}
