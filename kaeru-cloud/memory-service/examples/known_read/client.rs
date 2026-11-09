use super::models::{Card, Sample};
use reqwest::Client;
use serde_json::{Value, json};
use std::time::Instant;

#[derive(Clone)]
pub struct Api {
    pub client: Client,
    pub base: String,
    pub token: String,
}

struct Accumulator {
    body: String,
    revision: Option<String>,
    conflicts: usize,
    reasoning: usize,
    metadata_seen: bool,
}
impl Accumulator {
    fn new() -> Self {
        Self {
            body: String::new(),
            revision: None,
            conflicts: 0,
            reasoning: 0,
            metadata_seen: false,
        }
    }
    fn accept(&mut self, value: &Value, card: &Card) -> Result<(), String> {
        if value.get("api_version").and_then(Value::as_u64) != Some(1)
            || value.get("operation").and_then(Value::as_str) != Some("read")
            || value.get("state").and_then(Value::as_str) != Some("ok")
        {
            return Err(value
                .pointer("/issues/0/code")
                .and_then(Value::as_str)
                .unwrap_or("api_state")
                .into());
        }
        let node = value.pointer("/data/node").ok_or("missing_node")?;
        if node.get("id").and_then(Value::as_str) != Some(&card.id) {
            return Err("wrong_id".into());
        }
        let revision = node
            .get("revision")
            .and_then(Value::as_str)
            .filter(|v| !v.is_empty())
            .ok_or("missing_revision")?;
        if self.revision.as_deref().is_some_and(|old| old != revision) {
            return Err("revision_changed".into());
        }
        self.revision = Some(revision.into());
        if node.get("name").and_then(Value::as_str) == Some(&card.name)
            && node
                .get("initiatives")
                .and_then(Value::as_array)
                .is_some_and(|v| v.iter().any(|s| s.as_str() == Some(&card.scope)))
            && node
                .get("currentness")
                .and_then(Value::as_str)
                .is_some_and(|v| !v.is_empty())
            && node.get("metadata_is_current").and_then(Value::as_bool) == Some(true)
        {
            self.metadata_seen = true;
        }
        self.conflicts = self.conflicts.max(
            value
                .pointer("/data/conflicts")
                .and_then(Value::as_array)
                .map_or(0, Vec::len),
        );
        self.reasoning = self.reasoning.max(
            value
                .pointer("/data/reasoning")
                .and_then(Value::as_array)
                .map_or(0, Vec::len),
        );
        if let Some(body) = node.get("body").and_then(Value::as_str) {
            let (start, end, total) =
                if let Some(range) = node.get("body_range").filter(|v| !v.is_null()) {
                    (
                        range
                            .get("start")
                            .and_then(Value::as_u64)
                            .ok_or("body_range")? as usize,
                        range
                            .get("end")
                            .and_then(Value::as_u64)
                            .ok_or("body_range")? as usize,
                        range
                            .get("total")
                            .and_then(Value::as_u64)
                            .ok_or("body_range")? as usize,
                    )
                } else {
                    (0, body.len(), body.len())
                };
            if start != self.body.len()
                || end < start
                || end - start != body.len()
                || total != card.body.len()
                || card.body.get(start..end) != Some(body)
            {
                return Err("body_range_or_content".into());
            }
            self.body.push_str(body);
        }
        Ok(())
    }
    fn complete(&self, card: &Card) -> bool {
        self.metadata_seen
            && self.body == card.body
            && self.conflicts == card.expected_conflicts
            && (!card.expected_reasoning || self.reasoning > 0)
    }
}

impl Api {
    pub async fn read(
        &self,
        card: &Card,
        budget: usize,
        concurrency: usize,
        index: usize,
        case: &str,
    ) -> Sample {
        let started = Instant::now();
        let mut sample = Sample {
            case: case.into(),
            body_bytes: card.body.len(),
            budget,
            concurrency,
            index,
            pages: 0,
            response_bytes: 0,
            latency_ms: 0.0,
            first_page_ms: 0.0,
            status: 0,
            correct: false,
            error: "incomplete".into(),
            returned_chars: 0,
            conflicts: 0,
            reasoning: 0,
            http_version: String::new(),
            phase_seconds: 0.0,
        };
        let mut args = json!({"initiative":card.scope,"node":{"id":card.id},"budget_chars":budget});
        let mut data = Accumulator::new();
        for _ in 0..128 {
            let response = match self
                .client
                .post(format!("{}/v1/read", self.base))
                .bearer_auth(&self.token)
                .json(&args)
                .send()
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    sample.error = if e.is_timeout() {
                        "timeout"
                    } else {
                        "transport"
                    }
                    .into();
                    break;
                }
            };
            sample.status = response.status().as_u16();
            sample.http_version = format!("{:?}", response.version());
            let bytes = match response.bytes().await {
                Ok(v) => v,
                Err(_) => {
                    sample.error = "response_body".into();
                    break;
                }
            };
            sample.pages += 1;
            sample.response_bytes += bytes.len();
            let value: Value = match serde_json::from_slice(&bytes) {
                Ok(v) => v,
                Err(_) => {
                    sample.error = "decode".into();
                    break;
                }
            };
            if sample.first_page_ms == 0.0 {
                sample.first_page_ms = started.elapsed().as_secs_f64() * 1000.0;
            }
            if sample.status != 200 {
                sample.error = value
                    .pointer("/issues/0/code")
                    .and_then(Value::as_str)
                    .unwrap_or("http_status")
                    .into();
                break;
            }
            if let Err(error) = data.accept(&value, card) {
                sample.error = error;
                break;
            }
            if let Some(next) = value.pointer("/continuation/next_arguments") {
                args = next.clone();
                continue;
            }
            sample.correct = data.complete(card);
            sample.error = if sample.correct {
                "none"
            } else {
                "incomplete_or_metadata"
            }
            .into();
            break;
        }
        sample.latency_ms = started.elapsed().as_secs_f64() * 1000.0;
        sample.returned_chars = data.body.len();
        sample.conflicts = data.conflicts;
        sample.reasoning = data.reasoning;
        sample
    }

    pub async fn reject(&self, card: &Card, bad_token: bool, index: usize) -> Sample {
        let started = Instant::now();
        let case = if bad_token {
            "invalid_bearer"
        } else {
            "scope_denied"
        };
        let mut sample = Sample {
            case: case.into(),
            body_bytes: 0,
            budget: 64000,
            concurrency: 1,
            index,
            pages: 0,
            response_bytes: 0,
            latency_ms: 0.0,
            first_page_ms: 0.0,
            status: 0,
            correct: false,
            error: "transport".into(),
            returned_chars: 0,
            conflicts: 0,
            reasoning: 0,
            http_version: String::new(),
            phase_seconds: 0.0,
        };
        let token = if bad_token {
            "synthetic-invalid-bearer"
        } else {
            &self.token
        };
        if let Ok(response) = self
            .client
            .post(format!("{}/v1/read", self.base))
            .bearer_auth(token)
            .json(&json!({"initiative":card.scope,"node":{"id":card.id},"budget_chars":64000}))
            .send()
            .await
        {
            sample.status = response.status().as_u16();
            sample.http_version = format!("{:?}", response.version());
            if let Ok(bytes) = response.bytes().await {
                sample.pages = 1;
                sample.response_bytes = bytes.len();
                let value: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
                sample.correct = sample.status == if bad_token { 401 } else { 404 }
                    && value.pointer("/data/node").is_none();
                sample.error = if sample.correct {
                    "none"
                } else {
                    "denial_contract"
                }
                .into();
            }
        }
        sample.latency_ms = started.elapsed().as_secs_f64() * 1000.0;
        sample.first_page_ms = sample.latency_ms;
        sample
    }
}
