//! Local account setup and explicit authentication choice, before opening the vault.

use std::io::{self, BufRead};
use std::path::PathBuf;

use clap::{ArgGroup, Parser, ValueEnum};

use super::session::{hash_password, random_secret};
use super::store::{AccountStore, read_config};
use crate::settings::{
    KaeruMcpConfig,
    web::{self, WebConfig},
};

#[derive(Clone, Copy, ValueEnum)]
enum AuthChoice {
    Enabled,
    Disabled,
}

#[derive(Parser)]
#[command(version, about = "Kaeru local authentication setup")]
#[command(group(ArgGroup::new("action").args(["init_web_auth", "reset_web_password", "auth", "auth_status", "stdio"]).multiple(false)))]
struct Setup {
    #[arg(long)]
    init_web_auth: bool,
    #[arg(long)]
    reset_web_password: bool,
    /// Enable authentication or persist an explicitly open local installation.
    #[arg(long, value_enum)]
    auth: Option<AuthChoice>,
    /// Print enabled or disabled without changing configuration or opening the vault.
    #[arg(long)]
    auth_status: bool,
    #[arg(long)]
    stdio: bool,
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

fn password(args: &Setup) -> Result<String, Box<dyn std::error::Error>> {
    let value = if args.password_stdin {
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
    Ok(value)
}

pub fn handle() -> Result<bool, Box<dyn std::error::Error>> {
    if !std::env::args().any(|arg| {
        matches!(
            arg.as_str(),
            "--init-web-auth"
                | "--reset-web-password"
                | "--auth"
                | "--auth-status"
                | "--help"
                | "-h"
                | "--version"
                | "-V"
        )
    }) {
        return Ok(false);
    }
    let args = Setup::parse();
    let path = args
        .web_config
        .clone()
        .map(Ok)
        .unwrap_or_else(web::config_path)?;
    let path = if path.is_absolute() {
        path
    } else {
        std::env::current_dir()?.join(path)
    };
    let current = read_config(&path)?;
    let token = KaeruMcpConfig::new()?.auth_token;
    if args.auth_status {
        println!(
            "{}",
            if web::access_mode(current.as_ref(), &token).enabled {
                "enabled"
            } else {
                "disabled"
            }
        );
        return Ok(true);
    }
    if matches!(args.auth, Some(AuthChoice::Disabled)) {
        match AccountStore::open(path.clone())? {
            Some(account) => {
                account.update(account.snapshot().revision, |cfg| cfg.enabled = false)?
            }
            None => AccountStore::initialize(
                path.clone(),
                WebConfig {
                    enabled: false,
                    ..Default::default()
                },
            )?,
        }
        eprintln!(
            "Authentication disabled in {}. Restart kaeru-mcp to apply.",
            path.display()
        );
        return Ok(true);
    }
    if current.as_ref().is_some_and(|cfg| cfg.enabled) {
        if args.init_web_auth {
            return Err("Account already configured; use --reset-web-password".into());
        }
        let account = AccountStore::open(path.clone())?.ok_or("Account disappeared")?;
        if args.reset_web_password {
            let hash = hash_password(&password(&args)?)?;
            account.update(account.snapshot().revision, |cfg| cfg.password_hash = hash)?;
        } else if account.snapshot().mcp_token.is_empty() && token.is_empty() {
            account.update(account.snapshot().revision, |cfg| {
                cfg.mcp_token = random_secret()
            })?;
        }
    } else {
        if args.reset_web_password {
            return Err("Authentication is disabled; enable it first with --auth enabled".into());
        }
        let hash = hash_password(&password(&args)?)?;
        let next = WebConfig {
            enabled: true,
            mcp_token: random_secret(),
            username: args.username,
            password_hash: hash,
            jwt_secret: random_secret(),
            public_origin: args.public_origin,
            ..Default::default()
        };
        match AccountStore::open(path.clone())? {
            Some(account) => account.update(account.snapshot().revision, |cfg| *cfg = next)?,
            None => AccountStore::initialize(path.clone(), next)?,
        }
    }
    eprintln!(
        "Authentication configured in {}. Restart kaeru-mcp, then open /ui/. MCP clients use KAERU_MCP_AUTH_TOKEN or mcp_token from this owner-only file.",
        path.display()
    );
    Ok(true)
}
