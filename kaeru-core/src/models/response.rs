//! Response envelopes, coverage and continuation metadata.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{ContextData, HistoryData, InitiativesData, MutationData, RecordData, SearchData};
use crate::config::API_VERSION;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Issue {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    pub retryable: bool,
}

impl Issue {
    pub fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            field: None,
            retryable: false,
        }
    }
    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Ok,
    Applied,
    Unchanged,
    Partial,
    Rejected,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ResponseData {
    Mutation(MutationData),
    Record(Box<RecordData>),
    Search(SearchData),
    Context(ContextData),
    History(HistoryData),
    Initiatives(InitiativesData),
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Omission {
    pub section: String,
    pub count: usize,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Coverage {
    pub initiatives: Vec<String>,
    pub complete: bool,
    pub rules_complete: bool,
    pub omitted: Vec<Omission>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Continuation {
    pub cursor: String,
    pub next_arguments: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Response {
    pub api_version: u32,
    pub operation: String,
    pub state: State,
    pub request_id: Option<String>,
    pub replayed: bool,
    pub data: ResponseData,
    pub issues: Vec<Issue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coverage: Option<Coverage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation: Option<Continuation>,
}

impl Response {
    pub fn mutation(operation: &str, data: MutationData) -> Self {
        Self {
            api_version: API_VERSION,
            operation: operation.into(),
            state: State::Applied,
            request_id: None,
            replayed: false,
            data: ResponseData::Mutation(data),
            issues: vec![],
            coverage: None,
            continuation: None,
        }
    }
    pub fn read(operation: &str, data: ResponseData, scopes: Vec<String>) -> Self {
        Self {
            api_version: API_VERSION,
            operation: operation.into(),
            state: State::Ok,
            request_id: None,
            replayed: false,
            data,
            issues: vec![],
            coverage: Some(Coverage {
                initiatives: scopes,
                complete: true,
                rules_complete: true,
                omitted: vec![],
            }),
            continuation: None,
        }
    }
    pub fn rejected(operation: &str, issue: Issue) -> Self {
        let mut result = Self::mutation(operation, MutationData::default());
        result.state = State::Rejected;
        result.issues.push(issue);
        result
    }
    pub fn is_error(&self) -> bool {
        matches!(self.state, State::Rejected | State::Partial)
    }
}
