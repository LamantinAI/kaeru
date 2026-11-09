use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub scope: String,
    pub body: String,
    pub expected_conflicts: usize,
    pub expected_reasoning: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Fixture {
    pub schema_version: u16,
    pub backend: String,
    pub namespace: String,
    pub allowed_scope: String,
    pub records: usize,
    pub versions_per_record: usize,
    pub edges: usize,
    pub cards: Vec<Card>,
    pub denied: Card,
    pub seed_seconds: f64,
}

pub struct Sample {
    pub case: String,
    pub body_bytes: usize,
    pub budget: usize,
    pub concurrency: usize,
    pub index: usize,
    pub pages: usize,
    pub response_bytes: usize,
    pub latency_ms: f64,
    pub first_page_ms: f64,
    pub status: u16,
    pub correct: bool,
    pub error: String,
    pub returned_chars: usize,
    pub conflicts: usize,
    pub reasoning: usize,
    pub http_version: String,
    pub phase_seconds: f64,
}
