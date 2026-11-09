use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "Known-ID canonical card read against disposable synthetic memory")]
pub struct Options {
    #[command(subcommand)]
    pub command: Command,
}
#[derive(Subcommand)]
pub enum Command {
    Seed(Seed),
    Run(Run),
}

#[derive(Args)]
pub struct Seed {
    #[arg(long, value_delimiter = ',')]
    pub pd_endpoints: Vec<String>,
    #[arg(long,default_value_t=1000,value_parser=clap::value_parser!(u32).range(30..=10000))]
    pub records: u32,
    #[arg(long)]
    pub output: PathBuf,
}
#[derive(Args)]
pub struct Run {
    #[arg(long, env = "CARD_READ_BASE_URL")]
    pub base_url: String,
    #[arg(long, env = "CARD_READ_TOKEN", hide_env_values = true)]
    pub token: String,
    #[arg(long)]
    pub fixture: PathBuf,
    #[arg(long)]
    pub output: PathBuf,
    #[arg(long, default_value_t = 200)]
    pub samples: usize,
    #[arg(long, default_value_t = 50)]
    pub paging_samples: usize,
    #[arg(long, default_value_t = 20)]
    pub warmup: usize,
    #[arg(long, default_value_t = 16)]
    pub concurrency: usize,
    #[arg(long, default_value_t = 2)]
    pub rounds: usize,
    #[arg(long, default_value_t = 30)]
    pub timeout_seconds: u64,
}
