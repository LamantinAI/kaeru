//! Internal lexical retrieval result, including exhaustion before scope filtering.

#[derive(Debug, Default)]
pub(crate) struct SearchCandidates {
    pub ids: Vec<String>,
    pub scan_truncated: bool,
    pub candidates_truncated: bool,
}
