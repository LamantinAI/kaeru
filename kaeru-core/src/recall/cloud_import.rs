//! Compatibility paths for the cloud import primitives.
//! Mutation implementations live in `mutate`; batched lookups live in `recall`.
pub use super::lookup::existing_node_ids;
pub use crate::mutate::cloud_import::{restore_cloud_edges, restore_cloud_edges_cancellable};
