use kaeru_tokenizer_service::{
    api::{self, ApiState},
    cache::Cache,
    config::Settings,
    error::Result,
    storage::Store,
};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    if let Err(error) = run().await {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let settings = Settings::from_env()?;
    let cache = if let Some(url) = &settings.valkey_url {
        let cache = Cache::connect(
            url,
            settings.cache_ttl,
            settings.cache_timeout,
            settings.cache_max_bytes,
        )
        .await;
        if cache.is_none() {
            tracing::warn!("Derived-facts cache unavailable; using PostgreSQL");
        }
        cache
    } else {
        None
    };
    let store = Store::connect(
        &settings.database_url,
        settings.max_connections,
        cache,
        settings.cache_namespace,
    )
    .await?;
    let app = api::router(
        ApiState {
            store: store.clone(),
            keys: settings.keys,
            max_projection_records: settings.max_projection_records,
        },
        settings.max_body_bytes,
    );
    let listener = TcpListener::bind(settings.listen).await?;
    tracing::info!("Tokenizer listener started");
    let result = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown())
        .await;
    store.close().await;
    result?;
    Ok(())
}

async fn shutdown() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        if let Ok(mut terminate) = signal(SignalKind::terminate()) {
            tokio::select! {_=tokio::signal::ctrl_c()=>{},_=terminate.recv()=>{}}
            return;
        }
    }
    let _ = tokio::signal::ctrl_c().await;
}
