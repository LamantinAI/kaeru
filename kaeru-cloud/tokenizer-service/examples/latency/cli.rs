use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub(super) struct Args {
    #[arg(long, env = "TOKENIZER_BASE_URL")]
    pub(super) base_url: String,
    #[arg(long, env = "TOKENIZER_COMPARE_URL")]
    pub(super) compare_url: Option<String>,
    #[arg(long, env = "TOKENIZER_CONTROL_KEY", hide_env_values = true)]
    pub(super) control_key: String,
    #[arg(long, env = "TOKENIZER_MEMORY_KEY", hide_env_values = true)]
    pub(super) memory_key: String,
    #[arg(long, default_value_t = 1000)]
    pub(super) samples: usize,
    #[arg(long, default_value_t = 100)]
    pub(super) write_samples: usize,
    #[arg(long, default_value_t = 50)]
    pub(super) warmup: usize,
    #[arg(long, default_value_t = 16)]
    pub(super) concurrency: usize,
    #[arg(long, default_value_t = 100)]
    pub(super) projects: usize,
    #[arg(long, default_value_t = 10)]
    pub(super) per_project: usize,
    #[arg(long, default_value_t = 100)]
    pub(super) users: usize,
    #[arg(long)]
    pub(super) output: PathBuf,
}
