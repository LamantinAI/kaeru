//! Opt-in, loopback-only full-path lab, with a frozen synthetic scope registry.
mod cli;
mod client;
mod config;
mod models;
mod prepare;
mod router;
use axum::serve::ListenerExt;
use clap::Parser;
use kaeru_core::{Store, config::StorageConfig};
use kaeru_memory_service::settings::MemoryServiceConfig;
use kaeru_runtime::ApiConfig;
use std::{
    error::Error,
    fs::File,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;
#[tokio::main]
async fn main() -> Result<()> {
    match cli::Options::parse().command {
        cli::Command::Prepare(args) => prepare::prepare(args).await,
        cli::Command::Serve(args) => serve(args).await,
    }
}
async fn serve(args: cli::Serve) -> Result<()> {
    cli::loopback_url(&args.tokenizer_url)?;
    if !args.listen.ip().is_loopback() {
        return Err("lab listener must be loopback".into());
    }
    let prepared: models::MappingDocument = serde_json::from_reader(File::open(args.prepared)?)?;
    prepared.mapping.validate()?;
    let storage = MemoryServiceConfig::storage_from_env()?;
    let StorageConfig::Tikv { connection } = &storage.storage else {
        return Err("lab requires explicit disposable TiKV storage".into());
    };
    if !connection
        .namespace
        .as_deref()
        .is_some_and(|n| n.starts_with("known-read-"))
        || connection.end_points.iter().any(|e| {
            e.parse::<std::net::SocketAddr>()
                .map_or(true, |a| !a.ip().is_loopback())
        })
    {
        return Err("lab requires synthetic namespace and loopback PD".into());
    }
    let store = Arc::new(Store::open_with_config(storage)?);
    let authorizer = client::Authorizer::new(
        &args.tokenizer_url,
        contracts::SecretToken::new(args.memory_key),
    )?;
    let app = router::router(router::App {
        store,
        cfg: ApiConfig {
            allow: vec!["known-read-0".into()],
            include_local: args.include_local,
            ..Default::default()
        },
        authorizer: authorizer.clone(),
        mapping: prepared.mapping,
    })?;
    let listener = tokio::net::TcpListener::bind(args.listen).await?;
    let connections = Arc::new(AtomicUsize::new(0));
    let count = connections.clone();
    println!(
        "Disposable authorized-read adapter ready; operator local visibility={}",
        args.include_local
    );
    axum::serve(
        listener.tap_io(move |_| {
            count.fetch_add(1, Ordering::Relaxed);
        }),
        app,
    )
    .with_graceful_shutdown(shutdown())
    .await?;
    println!(
        "authorization_calls={} http2_responses={} incoming_connections={}",
        authorizer.calls.load(Ordering::Relaxed),
        authorizer.http2_responses.load(Ordering::Relaxed),
        connections.load(Ordering::Relaxed)
    );
    Ok(())
}
async fn shutdown() {
    #[cfg(unix)]
    {
        let mut signal = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("signal handler");
        tokio::select! {_=signal.recv()=>{},_=tokio::signal::ctrl_c()=>{}}
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}
