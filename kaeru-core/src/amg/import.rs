//! Validate first, then plan and apply under one transaction and writer gate.

use super::{codec::digest, plan::build, storage::check_legacy_keys, validate, write::apply};
use crate::config::amg::AmgLimits;
use crate::models::amg::{AmgDocument, AmgImportReport};
use crate::{Result, Store};

pub fn import(
    store: &Store,
    document: &AmgDocument,
    force: bool,
    dry_run: bool,
    limits: &AmgLimits,
) -> Result<AmgImportReport> {
    validate(document, limits)?;
    let digest = digest(document, limits)?;
    store.atomic(|store| {
        check_legacy_keys(store, &document.data.edges)?;
        let plan = build(store, document, force, dry_run, digest)?;
        let report = &plan.report;
        if !dry_run
            && (report.created_nodes
                + report.updated_nodes
                + report.created_edges
                + report.updated_edges
                + report.updated_metadata
                > 0)
        {
            apply(store, document, &plan)?;
        }
        Ok(plan.report)
    })
}
