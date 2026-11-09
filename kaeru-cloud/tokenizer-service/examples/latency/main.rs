//! Closed-loop HTTP/2 latency probe against an empty, isolated tokenizer.

use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

use clap::Parser;
use contracts::{
    Action, ActionMask, ApplyRightsRequest, ApplyRightsResponse, BootstrapRequest, Membership,
    MembershipState, OrganizationRole, POLICY_VERSION, ProjectionChange, RevokeTokenRequest,
    SCHEMA_VERSION, TokenOperationResponse,
};
use reqwest::Client;
use uuid::Uuid;

use cli::Args;
use client::{Api, Point, reads, save};
use fixture::{create_request, fixture, id};

mod cli;
mod client;
mod fixture;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let total = args
        .projects
        .checked_mul(args.per_project)
        .ok_or("dataset size overflow")?;
    let records = total
        .checked_add(args.projects)
        .and_then(|n| n.checked_add(args.users.checked_add(1)?.checked_mul(2)?))
        .and_then(|n| n.checked_add(1))
        .ok_or("dataset size overflow")?;
    if args.projects == 0
        || args.per_project == 0
        || args.users == 0
        || args.samples == 0
        || args.write_samples == 0
        || args.concurrency == 0
        || args.concurrency > 128
        || args.samples > total
        || records > 10_000
    {
        return Err("invalid dimensions; distinct samples require enough initiatives and the projection must fit the service limit".into());
    }
    let mut file = File::create_new(&args.output)?;
    writeln!(
        file,
        "backend,scenario,concurrency,index,latency_us,http_status,correct,request_bytes,response_bytes"
    )?;
    let client = Client::builder()
        .http2_prior_knowledge()
        .timeout(Duration::from_secs(15))
        .build()?;
    let api = Api {
        client: client.clone(),
        base: args.base_url.trim_end_matches('/').into(),
        control: args.control_key.clone(),
        memory: args.memory_key.clone(),
        label: "cached",
    };
    let generation = Uuid::now_v7();
    let snapshot = fixture(&args);
    println!(
        "projection principals={} projects={} initiatives={}",
        snapshot.principals.len(),
        snapshot.projects.len(),
        snapshot.initiatives.len()
    );
    let _: (ApplyRightsResponse, _, _, _) = api
        .call(
            "/v1/rights/bootstrap",
            &api.control,
            &BootstrapRequest {
                schema_version: SCHEMA_VERSION,
                policy_version: POLICY_VERSION,
                operation_id: Uuid::now_v7(),
                source_generation: generation,
                source_revision: 0,
                snapshot,
            },
        )
        .await?;
    let (issued, _, _, _): (TokenOperationResponse, _, _, _) = api
        .call("/v1/tokens", &api.control, &create_request())
        .await?;
    let secret = issued
        .secret
        .ok_or("create did not return a one-time secret")?;
    let mut backends = vec![api.clone()];
    if let Some(url) = &args.compare_url {
        backends.push(Api {
            base: url.trim_end_matches('/').into(),
            label: "postgres_only",
            ..api.clone()
        });
    }
    let mut errors = 0;
    let mut revision = 0u64;
    let mut concurrencies = vec![1, args.concurrency];
    concurrencies.sort_unstable();
    concurrencies.dedup();
    for backend in &backends {
        for &concurrency in &concurrencies {
            for scenario in [
                "authorize_allow",
                "authorize_deny",
                "authorize_batch16",
                "authorize_distinct",
            ] {
                if scenario == "authorize_distinct" {
                    // Publish an unchanged projection revision outside timing:
                    // PostgreSQL is warm, but all derived-facts keys are new.
                    let _: (ApplyRightsResponse, _, _, _) = api
                        .call(
                            "/v1/rights/apply",
                            &api.control,
                            &ApplyRightsRequest {
                                schema_version: SCHEMA_VERSION,
                                policy_version: POLICY_VERSION,
                                operation_id: Uuid::now_v7(),
                                source_generation: generation,
                                expected_revision: revision,
                                source_revision: revision + 1,
                                changes: vec![],
                            },
                        )
                        .await?;
                    revision += 1;
                }
                let points = reads(backend, &args, &secret, scenario, concurrency).await?;
                errors += save(&mut file, &points)?;
            }
        }
    }
    let mut created = Vec::new();
    let mut points = Vec::new();
    for n in 0..args.write_samples {
        let started = Instant::now();
        let request = create_request();
        let (result, status, input, output): (TokenOperationResponse, _, _, _) =
            api.call("/v1/tokens", &api.control, &request).await?;
        points.push(Point::new(
            "cached",
            "create",
            1,
            n,
            started,
            status,
            result.secret.is_some(),
            input,
            output,
        ));
        created.push((request, result));
    }
    errors += save(&mut file, &points)?;
    let mut points = Vec::new();
    for n in 0..args.write_samples {
        let started = Instant::now();
        let request = &created[0].0;
        let (result, status, input, output): (TokenOperationResponse, _, _, _) =
            api.call("/v1/tokens", &api.control, request).await?;
        points.push(Point::new(
            "cached",
            "create_replay",
            1,
            n,
            started,
            status,
            result.replayed && result.secret.is_none(),
            input,
            output,
        ));
    }
    errors += save(&mut file, &points)?;
    let mut points = Vec::new();
    for (n, (_, token)) in created.iter().enumerate() {
        let started = Instant::now();
        let request = RevokeTokenRequest {
            schema_version: SCHEMA_VERSION,
            operation_id: Uuid::now_v7(),
            actor_id: id(1, 1),
        };
        let (result, status, input, output): (TokenOperationResponse, _, _, _) = api
            .call(
                &format!("/v1/tokens/{}/revoke", token.token.id),
                &api.control,
                &request,
            )
            .await?;
        points.push(Point::new(
            "cached",
            "revoke",
            1,
            n,
            started,
            status,
            result.token.revoked_at.is_some(),
            input,
            output,
        ));
    }
    errors += save(&mut file, &points)?;
    let mut points = Vec::new();
    for n in 0..args.write_samples {
        let started = Instant::now();
        let member = Membership {
            id: id(3, 1),
            principal_id: id(1, 1),
            organization_id: id(2, 0),
            role: OrganizationRole::Member,
            state: MembershipState::Active,
            all_projects: true,
            all_actions: if n % 2 == 0 {
                ActionMask::of(&[Action::Read, Action::Write])
            } else {
                ActionMask::of(&[Action::Read])
            },
        };
        let request = ApplyRightsRequest {
            schema_version: SCHEMA_VERSION,
            policy_version: POLICY_VERSION,
            operation_id: Uuid::now_v7(),
            source_generation: generation,
            expected_revision: revision,
            source_revision: revision + 1,
            changes: vec![ProjectionChange::Membership(member)],
        };
        let (result, status, input, output): (ApplyRightsResponse, _, _, _) =
            api.call("/v1/rights/apply", &api.control, &request).await?;
        revision += 1;
        points.push(Point::new(
            "cached",
            "rights_apply",
            1,
            n,
            started,
            status,
            result.source_revision == revision,
            input,
            output,
        ));
    }
    errors += save(&mut file, &points)?;
    for (_, token) in &created {
        if token.secret.is_none() {
            errors += 1;
        }
    }
    if errors > 0 {
        return Err(format!("{errors} failed checks; inspect raw CSV").into());
    }
    println!(
        "raw latency samples saved; request/response payloads and credentials were not recorded"
    );
    Ok(())
}
