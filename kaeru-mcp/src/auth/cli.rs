//! Local-only account setup and recovery, before opening the vault.

use std::io::{self, BufRead};
use std::path::PathBuf;

use clap::Parser;

use super::session::{hash_password, random_secret};
use super::store::AccountStore;
use crate::settings::web::{self, WebConfig};

#[derive(Parser)]
#[command(version, about = "Kaeru instance and local browser-account setup")]
struct Setup {
    #[arg(long, conflicts_with = "reset_web_password")]
    init_web_auth: bool,
    #[arg(long, conflicts_with_all = ["init_web_auth", "reset_web_password"])]
    stdio: bool,
    #[arg(long)]
    reset_web_password: bool,
    #[arg(long)]
    web_config: Option<PathBuf>,
    #[arg(long, default_value = "admin")]
    username: String,
    /// Pin an origin; otherwise use the browser request Host.
    #[arg(long, default_value = "")]
    public_origin: String,
    /// Read one password line from stdin for explicit automation.
    #[arg(long)]
    password_stdin: bool,
}

pub fn handle() -> Result<bool, Box<dyn std::error::Error>> {
    if !std::env::args().any(|arg| {
        matches!(
            arg.as_str(),
            "--init-web-auth" | "--reset-web-password" | "--help" | "-h" | "--version" | "-V"
        )
    }) {
        return Ok(false);
    }
    let args = Setup::parse();
    let password = if args.password_stdin {
        let mut value = String::new();
        io::stdin().lock().read_line(&mut value)?;
        value.trim_end_matches(['\r', '\n']).to_string()
    } else {
        let first = rpassword::prompt_password("New password: ")?;
        if first != rpassword::prompt_password("Repeat password: ")? {
            return Err("Passwords do not match".into());
        }
        first
    };
    let hash = hash_password(&password)?;
    let path = args.web_config.map(Ok).unwrap_or_else(web::config_path)?;
    let path = if path.is_absolute() {
        path
    } else {
        std::env::current_dir()?.join(path)
    };
    if args.init_web_auth {
        AccountStore::initialize(
            path.clone(),
            WebConfig {
                username: args.username,
                password_hash: hash,
                jwt_secret: random_secret(),
                public_origin: args.public_origin,
                ttl_seconds: web::DEFAULT_TTL_SECONDS,
                issuer: web::SESSION_ISSUER.into(),
                audience: web::SESSION_AUDIENCE.into(),
                revision: 1,
            },
        )?;
    } else {
        let account = AccountStore::open(path.clone())?.ok_or("Account is not configured")?;
        account.update(account.snapshot().revision, |next| {
            next.password_hash = hash;
        })?;
    }
    eprintln!(
        "Browser account saved to {}. Start or restart kaeru-mcp, then open /ui/.",
        path.display()
    );
    Ok(true)
}
