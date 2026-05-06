//! Graph schema layer — node and edge type definitions, the audit
//! writer, and bi-temporal `at` / `history` reads.
//!
//! Everything in this module models the graph that lives in the
//! substrate: what kinds of nodes there are, what kinds of edges connect
//! them, how time-travel queries read them, and how every mutation
//! writes its own audit_event node. Curator-API primitives in `mutate/`,
//! `recall/`, and `session/` build on top of these types.

pub mod audit;
pub mod edge;
pub mod node;
pub mod temporal;

pub use edge::EdgeType;
pub use node::EpisodeKind;
pub use node::HypothesisStatus;
pub use node::NodeId;
pub use node::NodeType;
pub use node::Significance;
pub use node::Tier;
pub use node::new_node_id;
pub use temporal::NodeSnapshot;
pub use temporal::Revision;
pub use temporal::at;
pub use temporal::history;
