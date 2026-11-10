//! Local account setup and explicit authentication choice, before opening the vault.

use std::io::{self, BufRead};
use std::path::PathBuf;

use clap::{Args, ValueEnum};

use crate::auth::session::{hash_password, random_secret};
use crate::auth::store::{AccountStore, read_config};
use crate::settings::{
    KaeruMcpConfig,
    web::{self, WebConfig},
};

#[derive(Clone, Copy, ValueEnum)]
pub(super) enum AuthChoice {
    Enabled,
    Disabled,
}

#[derive(Args)]
#[group(skip)]
pub(super) struct AuthOptions {
    #[arg(long)]
    pub init_web_auth: bool,
    #[arg(long)]
    pub reset_web_password: bool,
    /// Enable authentication or persist an explicitly open local installation.
    #[arg(long, value_enum)]
    pub auth: Option<AuthChoice>,
    /// Print enabled or disabled without changing configuration or opening the vault.
    #[arg(long)]
    pub auth_status: bool,
    /// Atomically update an existing account origin without changing credentials.
    /// Stop the daemon first; pass an exact origin or an empty string to unpin.
    #[arg(long, allow_hyphen_values = true)]
    pub set_web_origin: Option<String>,
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

fn password(args: &AuthOptions) -> Result<String, Box<dyn std::error::Error>> {
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

pub(super) fn handle(args: AuthOptions) -> Result<(), Box<dyn std::error::Error>> {
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
    if let Some(origin) = args.set_web_origin {
        let account = AccountStore::open(path.clone())?.ok_or("Account not configured")?;
        if !account.snapshot().enabled {
            return Err("Authentication is disabled".into());
        }
        account.update(account.snapshot().revision, |cfg| {
            cfg.public_origin = origin
        })?;
        eprintln!(
            "Browser origin updated. Restart kaeru-mcp to apply; existing sessions are revoked."
        );
        return Ok(());
    }
    let current = read_config(&path)?;
    let transport = KaeruMcpConfig::new()?;
    let token = transport.auth_token;
    let ui_root = transport.ui_base_path.page_root();
    if args.auth_status {
        println!(
            "{}",
            if web::access_mode(current.as_ref(), &token).enabled {
                "enabled"
            } else {
                "disabled"
            }
        );
        return Ok(());
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
        return Ok(());
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
        "Authentication configured in {}. Restart kaeru-mcp, then open {ui_root}/. MCP clients use KAERU_MCP_AUTH_TOKEN or mcp_token from this owner-only file.",
        path.display()
    );
    Ok(())
}

#[cfg(test)]
mod migration_tests {
    use super::*;
    fn options(path: PathBuf, origin: &str) -> AuthOptions {
        AuthOptions {
            init_web_auth: false,
            reset_web_password: false,
            auth: None,
            auth_status: false,
            set_web_origin: Some(origin.into()),
            web_config: Some(path),
            username: "unused".into(),
            public_origin: String::new(),
            password_stdin: false,
        }
    }
    #[test]
    fn origin_only_migration_is_validated_atomic_and_preserves_credentials() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("account.toml");
        let before = WebConfig {
            username: "operator".into(),
            password_hash: hash_password("synthetic-password").unwrap(),
            jwt_secret: "ab".repeat(32),
            mcp_token: "synthetic-service-key".into(),
            public_origin: "https://old.example".into(),
            ..Default::default()
        };
        AccountStore::initialize(path.clone(), before.clone()).unwrap();
        {
            let _writer = AccountStore::open(path.clone()).unwrap().unwrap();
            assert!(handle(options(path.clone(), "https://new.example")).is_err());
        }
        handle(options(path.clone(), "https://new.example:8443")).unwrap();
        let after = read_config(&path).unwrap().unwrap();
        assert_eq!(after.public_origin, "https://new.example:8443");
        assert_eq!(after.revision, before.revision + 1);
        assert_eq!(after.password_hash, before.password_hash);
        assert_eq!(after.jwt_secret, before.jwt_secret);
        assert_eq!(after.mcp_token, before.mcp_token);
        assert_eq!(after.username, before.username);
        let bytes = std::fs::read(&path).unwrap();
        assert!(handle(options(path.clone(), "https://new.example/path")).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
    }
}
