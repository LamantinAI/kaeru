//! Trusted bootstrap for an empty disposable tokenizer, never a live Control.
use super::{
    Result, cli,
    models::{Mapping, Prepared},
};
use contracts::{
    ApplyRightsResponse, BootstrapRequest, CreateTokenRequest, Initiative, POLICY_VERSION,
    PlatformRole, Principal, PrincipalState, ProjectionSnapshot, ResourceState, SCHEMA_VERSION,
    TokenCeiling, TokenOperationResponse, TokenPurpose,
};
use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};
use std::fs::OpenOptions;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use uuid::Uuid;

pub async fn call<T: Serialize, R: DeserializeOwned>(
    client: &Client,
    base: &str,
    path: &str,
    key: &str,
    request: &T,
) -> Result<R> {
    let mut body = Vec::new();
    ciborium::into_writer(request, &mut body)?;
    let response = client
        .post(format!("{}{path}", base.trim_end_matches('/')))
        .bearer_auth(key)
        .header("content-type", "application/cbor")
        .body(body)
        .send()
        .await?;
    if response.version() != reqwest::Version::HTTP_2 || !response.status().is_success() {
        return Err(format!("bootstrap request failed with HTTP {}", response.status()).into());
    }
    let bytes = response.bytes().await?;
    if bytes.len() > 64 * 1024 {
        return Err("bootstrap response too large".into());
    }
    Ok(ciborium::from_reader(bytes.as_ref())?)
}

pub async fn prepare(args: cli::Prepare) -> Result<()> {
    cli::loopback_url(&args.tokenizer_url)?;
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);
    let file = options.open(&args.output)?;
    let client = Client::builder()
        .http2_prior_knowledge()
        .timeout(super::config::AUTHORIZE_TIMEOUT)
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let principal_id = Uuid::now_v7();
    let other = Uuid::now_v7();
    let mapping = Mapping {
        revision: Uuid::now_v7(),
        initiatives: (0..5)
            .map(|n| (format!("known-read-{n}"), Uuid::now_v7()))
            .collect(),
    };
    let snapshot = ProjectionSnapshot {
        principals: vec![principal_id, other]
            .into_iter()
            .map(|id| Principal {
                id,
                role: PlatformRole::User,
                state: PrincipalState::Active,
            })
            .collect(),
        initiatives: mapping
            .initiatives
            .iter()
            .map(|(name, id)| Initiative {
                id: *id,
                personal_owner_id: Some(if name == "known-read-0" {
                    principal_id
                } else {
                    other
                }),
                project_id: None,
                state: ResourceState::Active,
                placement_revision: 1,
            })
            .collect(),
        ..ProjectionSnapshot::default()
    };
    let _: ApplyRightsResponse = call(
        &client,
        &args.tokenizer_url,
        "/v1/rights/bootstrap",
        &args.control_key,
        &BootstrapRequest {
            schema_version: SCHEMA_VERSION,
            policy_version: POLICY_VERSION,
            operation_id: Uuid::now_v7(),
            source_generation: Uuid::now_v7(),
            source_revision: 0,
            snapshot,
        },
    )
    .await?;
    let mut issued = Vec::new();
    for purpose in [
        TokenPurpose::Memory,
        TokenPurpose::Memory,
        TokenPurpose::Control,
    ] {
        let token: TokenOperationResponse = call(
            &client,
            &args.tokenizer_url,
            "/v1/tokens",
            &args.control_key,
            &CreateTokenRequest {
                schema_version: SCHEMA_VERSION,
                operation_id: Uuid::now_v7(),
                actor_id: principal_id,
                name: "synthetic-read-probe".into(),
                purpose,
                ceiling: TokenCeiling::AllCurrent,
                expires_at: None,
            },
        )
        .await?;
        issued.push(token);
    }
    let mut issued = issued.into_iter();
    let first = issued.next().unwrap();
    let prepared = Prepared {
        mapping,
        principal_id,
        token_id: first.token.id,
        credential: first.secret.ok_or("one-time secret missing")?,
        second_credential: issued
            .next()
            .unwrap()
            .secret
            .ok_or("one-time secret missing")?,
        control_credential: issued
            .next()
            .unwrap()
            .secret
            .ok_or("one-time secret missing")?,
    };
    serde_json::to_writer_pretty(file, &prepared)?;
    println!("Synthetic rights and three one-time credentials saved privately; no secrets printed");
    Ok(())
}
