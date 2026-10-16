//! Independent bounds for lexical candidate scans and displayed briefs.

/// Maximum displayed briefs per `fuzzy_recall` call.
pub const FUZZY_RECALL_LIMIT_CAP: usize = 50;
/// Maximum global hits examined per FTS index, before initiative filtering.
/// One additional hit is requested to distinguish exhaustion from truncation.
pub const FTS_SCAN_LIMIT: usize = 50_000;
