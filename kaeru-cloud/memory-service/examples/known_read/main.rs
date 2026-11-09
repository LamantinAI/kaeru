//! Canonical HTTP known-card measurements; no search or user vault access.
mod cli;
mod client;
mod fixture;
mod measurement;
mod models;

use clap::Parser;
use cli::{Command, Options, Run};
use client::Api;
use measurement::save;
use models::{Card, Fixture, Sample};
use reqwest::Client;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};
use tokio::task::JoinSet;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    match Options::parse().command {
        Command::Seed(args) => tokio::task::spawn_blocking(move || fixture::seed(args)).await?,
        Command::Run(args) => run(args).await,
    }
}

async fn phase(
    api: &Api,
    card: &Card,
    budget: usize,
    concurrency: usize,
    samples: usize,
    case: &str,
) -> Result<Vec<Sample>> {
    let started = Instant::now();
    let mut tasks = JoinSet::new();
    let mut next = 0;
    let mut rows = Vec::with_capacity(samples);
    while next < samples || !tasks.is_empty() {
        while next < samples && tasks.len() < concurrency {
            let api = api.clone();
            let card = card.clone();
            let case = case.to_string();
            let index = next;
            next += 1;
            tasks.spawn(async move { api.read(&card, budget, concurrency, index, &case).await });
        }
        if let Some(result) = tasks.join_next().await {
            rows.push(result?);
        }
    }
    let seconds = started.elapsed().as_secs_f64();
    for row in &mut rows {
        row.phase_seconds = seconds;
    }
    Ok(rows)
}

async fn run(args: Run) -> Result<()> {
    if args.samples == 0
        || args.rounds == 0
        || args.concurrency == 0
        || args.concurrency > 128
        || args.timeout_seconds == 0
    {
        return Err("invalid benchmark dimensions".into());
    }
    let fixture: Fixture = serde_json::from_reader(File::open(&args.fixture)?)?;
    if fixture.schema_version != 1 || fixture.backend != "tikv" {
        return Err("benchmark requires the isolated TiKV fixture".into());
    }
    let mut file = File::create_new(&args.output)?;
    writeln!(
        file,
        "round,case,body_bytes,budget,concurrency,index,pages,latency_ms,first_page_ms,http_status,correct,error_kind,response_bytes,returned_chars,conflicts,reasoning,http_version,phase_seconds"
    )?;
    let api = Api {
        client: Client::builder()
            .http1_only()
            .pool_max_idle_per_host(args.concurrency)
            .timeout(Duration::from_secs(args.timeout_seconds))
            .build()?,
        base: args.base_url.trim_end_matches('/').into(),
        token: args.token,
    };
    let mut errors = 0;
    for card in &fixture.cards {
        errors += save(
            &mut file,
            &[api.read(card, 64000, 1, 0, "first_request").await],
            0,
        )?;
    }
    for n in 0..10 {
        errors += save(
            &mut file,
            &[
                api.reject(&fixture.cards[0], true, n).await,
                api.reject(&fixture.denied, false, n).await,
            ],
            0,
        )?;
    }
    if errors > 0 {
        return Err("first-card/authentication controls failed; raw rows preserved".into());
    }
    for round in 1..=args.rounds {
        let mut concurrency = vec![1, 4.min(args.concurrency), args.concurrency];
        concurrency.sort_unstable();
        concurrency.dedup();
        if round % 2 == 0 {
            concurrency.reverse();
        }
        for card in &fixture.cards {
            for _ in 0..args.warmup {
                if !api.read(card, 64000, 1, 0, "warmup").await.correct {
                    return Err("warmup failed".into());
                }
            }
            for &c in &concurrency {
                let rows = phase(&api, card, 64000, c, args.samples, "known_card").await?;
                errors += save(&mut file, &rows, round)?;
            }
        }
    }
    if args.paging_samples > 0 {
        let card = fixture
            .cards
            .iter()
            .max_by_key(|c| c.body.len())
            .ok_or("fixture is empty")?;
        for c in [1, args.concurrency] {
            let rows = phase(
                &api,
                card,
                12000,
                c,
                args.paging_samples,
                "default_budget_paging",
            )
            .await?;
            let diagnostic_errors = save(&mut file, &rows, 1)?;
            errors += diagnostic_errors;
            println!("paging errors={diagnostic_errors}; no retries applied");
        }
    }
    println!("main_read_errors={errors}; raw requests contain no credentials or payloads");
    if errors > 0 {
        return Err("main read correctness errors; inspect raw CSV".into());
    }
    Ok(())
}
