use clap::{Parser, Subcommand};
use std::{net::SocketAddr, path::PathBuf};
#[derive(Parser)]
#[command(
    about = "Disposable Memory → HTTP/2 Tokenizer → TiKV integration, not production Control wiring"
)]
pub struct Options {
    #[command(subcommand)]
    pub command: Command,
}
#[derive(Subcommand)]
pub enum Command {
    Prepare(Prepare),
    Serve(Serve),
}
#[derive(clap::Args)]
pub struct Prepare {
    #[arg(long, env = "TOKENIZER_BASE_URL")]
    pub tokenizer_url: String,
    #[arg(long, env = "TOKENIZER_CONTROL_KEY", hide_env_values = true)]
    pub control_key: String,
    #[arg(long)]
    pub output: PathBuf,
}
#[derive(clap::Args)]
pub struct Serve {
    #[arg(long, env = "TOKENIZER_BASE_URL")]
    pub tokenizer_url: String,
    #[arg(long, env = "TOKENIZER_MEMORY_KEY", hide_env_values = true)]
    pub memory_key: String,
    #[arg(long)]
    pub prepared: PathBuf,
    #[arg(long)]
    pub listen: SocketAddr,
    #[arg(long)]
    pub include_local: bool,
}
pub fn loopback_url(base: &str) -> Result<(), &'static str> {
    let url = reqwest::Url::parse(base).map_err(|_| "invalid tokenizer URL")?;
    if url.scheme() != "http"
        || url
            .host_str()
            .and_then(|h| h.parse::<std::net::IpAddr>().ok())
            .is_none_or(|a| !a.is_loopback())
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(
            "this lab requires an explicit loopback HTTP tokenizer URL without credentials or query",
        );
    }
    Ok(())
}
