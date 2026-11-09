use std::fs::File;
use std::io::Write;
use std::time::Instant;

use contracts::{AuthorizeRequest, AuthorizeResponse, SecretToken};
use kaeru_tokenizer_service::codec::{decode, encode};
use reqwest::{Client, Version};
use serde::{Serialize, de::DeserializeOwned};
use tokio::task::JoinSet;

use super::Result;
use super::cli::Args;
use super::fixture::{id, read_request};

#[derive(Clone)]
pub(super) struct Api {
    pub(super) client: Client,
    pub(super) base: String,
    pub(super) control: String,
    pub(super) memory: String,
    pub(super) label: &'static str,
}

impl Api {
    pub(super) async fn call<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        key: &str,
        body: &T,
    ) -> Result<(R, u16, usize, usize)> {
        let bytes = encode(body)?;
        let input = bytes.len();
        let response = self
            .client
            .post(format!("{}{}", self.base, path))
            .bearer_auth(key)
            .header("content-type", "application/cbor")
            .body(bytes)
            .send()
            .await?;
        if response.version() != Version::HTTP_2 {
            return Err("HTTP/2 was not used".into());
        }
        let status = response.status().as_u16();
        let bytes = response.bytes().await?;
        let output = bytes.len();
        if status != 200 {
            return Err(format!("unexpected HTTP status {status}").into());
        }
        Ok((
            decode(&bytes, 32).map_err(|_| "invalid CBOR response")?,
            status,
            input,
            output,
        ))
    }

    async fn read(
        &self,
        request: AuthorizeRequest,
        expected: bool,
        index: usize,
        scenario: &str,
        concurrency: usize,
    ) -> Point {
        let started = Instant::now();
        let result = self
            .call::<_, AuthorizeResponse>("/v1/authorize", &self.memory, &request)
            .await;
        match result {
            Ok((r, status, input, output)) => Point::new(
                self.label,
                scenario,
                concurrency,
                index,
                started,
                status,
                r.allowed == expected
                    && r.request_id == request.request_id
                    && r.checks == request.checks
                    && r.principal_id == id(1, 1),
                input,
                output,
            ),
            Err(_) => Point::new(
                self.label,
                scenario,
                concurrency,
                index,
                started,
                0,
                false,
                0,
                0,
            ),
        }
    }
}

pub(super) struct Point {
    pub(super) backend: String,
    pub(super) scenario: String,
    pub(super) concurrency: usize,
    pub(super) index: usize,
    pub(super) us: f64,
    pub(super) status: u16,
    pub(super) correct: bool,
    pub(super) input: usize,
    pub(super) output: usize,
}
impl Point {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        backend: &str,
        scenario: &str,
        concurrency: usize,
        index: usize,
        start: Instant,
        status: u16,
        correct: bool,
        input: usize,
        output: usize,
    ) -> Self {
        Self {
            backend: backend.into(),
            scenario: scenario.into(),
            concurrency,
            index,
            us: start.elapsed().as_secs_f64() * 1e6,
            status,
            correct,
            input,
            output,
        }
    }
}

pub(super) async fn reads(
    api: &Api,
    args: &Args,
    secret: &SecretToken,
    scenario: &str,
    concurrency: usize,
) -> Result<Vec<Point>> {
    let total = args.projects * args.per_project;
    if scenario != "authorize_distinct" {
        for n in 0..args.warmup {
            let point = api
                .read(
                    read_request(secret, scenario, n, total),
                    scenario != "authorize_deny",
                    n,
                    scenario,
                    1,
                )
                .await;
            if !point.correct {
                return Err("warmup did not produce the expected decision".into());
            }
        }
    }
    let mut tasks = JoinSet::new();
    let mut next = 0;
    let mut points = Vec::with_capacity(args.samples);
    while next < args.samples || !tasks.is_empty() {
        while next < args.samples && tasks.len() < concurrency {
            let api = api.clone();
            let request = read_request(secret, scenario, next, total);
            let scenario = scenario.to_string();
            let index = next;
            next += 1;
            tasks.spawn(async move {
                api.read(
                    request,
                    scenario != "authorize_deny",
                    index,
                    &scenario,
                    concurrency,
                )
                .await
            });
        }
        if let Some(point) = tasks.join_next().await {
            points.push(point?);
        }
    }
    Ok(points)
}

fn report(points: &[Point]) {
    let mut values = points
        .iter()
        .filter(|p| p.correct)
        .map(|p| p.us / 1000.0)
        .collect::<Vec<_>>();
    values.sort_by(f64::total_cmp);
    let p = |q: f64| {
        if values.is_empty() {
            f64::NAN
        } else {
            values[((values.len() as f64 * q).ceil() as usize).saturating_sub(1)]
        }
    };
    if let Some(first) = points.first() {
        println!(
            "{} {} c={} n={} errors={} p50={:.3}ms p95={:.3}ms p99={:.3}ms",
            first.backend,
            first.scenario,
            first.concurrency,
            points.len(),
            points.len() - values.len(),
            p(0.50),
            p(0.95),
            p(0.99)
        );
    }
}

pub(super) fn save(file: &mut File, points: &[Point]) -> Result<usize> {
    for p in points {
        writeln!(
            file,
            "{},{},{},{},{:.3},{},{},{},{}",
            p.backend,
            p.scenario,
            p.concurrency,
            p.index,
            p.us,
            p.status,
            p.correct,
            p.input,
            p.output
        )?;
    }
    file.flush()?;
    report(points);
    Ok(points.iter().filter(|p| !p.correct).count())
}
