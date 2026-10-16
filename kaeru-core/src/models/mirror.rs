//! Committed edge changes delivered to cloud adapters.

use crate::graph::EdgeType;

/// A network adapter mirrors this committed local edit after the transaction.
#[derive(Clone, Copy, Debug)]
pub enum EdgeChange {
    Upsert(f64),
    Retract,
}
#[derive(Clone, Debug)]
pub struct EdgeMirror {
    pub from: String,
    pub to: String,
    pub edge_type: EdgeType,
    pub change: EdgeChange,
}
