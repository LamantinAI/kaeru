//! Shared memory exchange models. Execution and transports consume these contracts.

mod access;
mod catalogue;
mod mirror;
mod read_trace;
mod records;
mod references;
mod response;
mod search;

pub use access::ApiOptions;
pub use catalogue::{ToolAnnotations, ToolDefinition};
pub use mirror::{EdgeChange, EdgeMirror};
pub use read_trace::ReadTraceEvent;
pub use records::{
    Artifact, Change, ContentRange, ContextData, Entity, EvidenceResult, FailedStep, HistoryData,
    HistoryEvent, InitiativesData, MutationData, NodeData, RecordData, RelationData,
    RetentionProposal, SearchData, Section, SlotChange, TrailData,
};
pub use references::{
    ArtifactSource, CheckEvidence, Evidence, ExistingEvidence, IdRef, NodeRef, ScopedRef, Source,
    TextSource,
};
pub use response::{Continuation, Coverage, Issue, Omission, Response, ResponseData, State};
pub(crate) use search::SearchCandidates;

mod export;
pub use export::{
    ChainExport, EdgeExport, ExportOpts, ExportSummary, GraphExport, GraphMeta, InitiativeStat,
    NodeExport, ProjectLink,
};
