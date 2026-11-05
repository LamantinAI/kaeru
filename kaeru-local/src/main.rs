//! Optional local memory host. The installed kaeru-mcp command stays stable.

mod cli;
mod settings;
mod stdio_bridge;

use std::{error::Error, net::SocketAddr, str::FromStr, sync::Arc};

use kaeru_core::{KaeruConfig, Store};
use kaeru_server::shutdown_signal as shutdown;
#[cfg(test)]
use kaeru_server::transport as session;
use kaeru_server::{ApiConfig, RuntimeOptions, ServerRuntime, auth};
use tracing::Level;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let startup = cli::run()?;
    if startup == cli::Startup::Handled {
        return Ok(());
    }
    let mcp_config = settings::load()?;
    let web_path = settings::web::optional_config_path()?;

    // `--stdio` is the bridge, not a second server: it forwards to the daemon
    // that owns the vault rather than opening the vault itself. The flag
    // remains independent of local browser-account setup commands.
    if startup == cli::Startup::Stdio {
        let url = format!(
            "http://{}{}",
            SocketAddr::new(mcp_config.listen_address, mcp_config.listen_port),
            mcp_config.mount_path
        );
        // stdout carries the protocol, so every log line has to go to stderr —
        // one stray byte on stdout and the client sees a malformed frame.
        tracing_subscriber::registry()
            .with(EnvFilter::from(mcp_config.log_level.as_str()))
            .with(fmt::layer().with_writer(std::io::stderr))
            .init();
        let web_config = web_path
            .as_ref()
            .map(|path| auth::store::read_config(path))
            .transpose()?
            .flatten();
        let access = settings::web::access_mode(web_config.as_ref(), &mcp_config.auth_token);
        let bridge = stdio_bridge::Bridge::new(url, mcp_config.listen_port, Some(access.mcp_token));
        bridge.ensure_daemon().await?;
        bridge.run().await?;
        return Ok(());
    }

    let level = Level::from_str(&mcp_config.log_level)?;
    tracing_subscriber::registry()
        .with(EnvFilter::from(level.as_str()))
        .with(
            fmt::layer()
                .with_writer(std::io::stderr)
                .with_ansi(false)
                .compact(),
        )
        .init();

    tracing::info!(
        listen_address = %mcp_config.listen_address,
        listen_port    = mcp_config.listen_port,
        mount_path     = %mcp_config.mount_path,
        "kaeru-mcp starting"
    );

    let store_config = KaeruConfig::from_env()?;
    let store = Store::open_with_config(store_config)?;
    tracing::info!(
        backend = store.config().storage.kind(),
        "local substrate ready"
    );

    let web_enabled = !matches!(
        std::env::var("KAERU_MCP_UI_ENABLE").ok().as_deref(),
        Some("0" | "false" | "no")
    );
    let account = web_path
        .map(auth::store::AccountStore::open)
        .transpose()?
        .flatten();
    let web_config = account.as_ref().map(auth::store::AccountStore::snapshot);
    let access = settings::web::access_mode(web_config.as_ref(), &mcp_config.auth_token);
    let viz_enabled = web_enabled
        || web_config.as_ref().is_some_and(|cfg| cfg.enabled)
        || matches!(
            std::env::var("KAERU_MCP_VIZ_ENABLE").ok().as_deref(),
            Some("1" | "true" | "yes")
        );
    let viz_csv = |key: &str| -> Vec<String> {
        std::env::var(key)
            .ok()
            .map(|s| {
                s.split(',')
                    .map(str::trim)
                    .filter(|x| !x.is_empty())
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default()
    };
    let api_config = ApiConfig {
        allow: viz_csv("KAERU_MCP_VIZ_INITIATIVES"),
        deny: viz_csv("KAERU_MCP_VIZ_DENY"),
        include_local: matches!(
            std::env::var("KAERU_MCP_VIZ_INCLUDE_LOCAL").ok().as_deref(),
            Some("1" | "true" | "yes")
        ),
        allow_origin: std::env::var("KAERU_MCP_VIZ_ALLOW_ORIGIN")
            .ok()
            .filter(|s| !s.is_empty()),
    };
    let runtime = ServerRuntime::new(
        Arc::new(store),
        mcp_config,
        RuntimeOptions {
            access,
            account,
            ui_enabled: web_enabled,
            api_enabled: viz_enabled,
            api_config,
            hygiene_enabled: hygiene_enabled_from(
                std::env::var("KAERU_MCP_HYGIENE_ENABLE").ok().as_deref(),
            ),
            update_check_enabled: true,
        },
    );
    runtime.serve().await?;
    Ok(())
}

/// Whether background hygiene runs, from `KAERU_MCP_HYGIENE_ENABLE`.
///
/// Unset means ON. The variable exists to turn it OFF, which is the inverse of
/// what it meant before 0.7.3 — so it is a named function with tests rather
/// than an inline `matches!`, because silently flipping the sense of an env var
/// a vault already sets is exactly the kind of thing that should fail a test
/// if anyone reverses it again.
fn hygiene_enabled_from(var: Option<&str>) -> bool {
    !matches!(
        var.map(str::trim),
        Some("0") | Some("false") | Some("FALSE") | Some("no") | Some("off")
    )
}

#[cfg(test)]
mod hygiene_flag_tests {
    use super::hygiene_enabled_from;

    /// The change: a daemon nobody configured sweeps. Off meant the fixes that
    /// depend on a pass ever running were silently inert — #75's oversized
    /// `core` never came down, because nothing ever ran to bring it down.
    #[test]
    fn unset_means_on() {
        assert!(hygiene_enabled_from(None));
    }

    /// The variable now exists to opt OUT.
    #[test]
    fn the_documented_off_values_turn_it_off() {
        for v in ["0", "false", "FALSE", "no", "off", " 0 "] {
            assert!(!hygiene_enabled_from(Some(v)), "{v:?} should disable");
        }
    }

    /// A vault that set `=1` under the old opt-in rule keeps working, and says
    /// the same thing it always did.
    #[test]
    fn the_old_opt_in_value_still_means_on() {
        for v in ["1", "true", "TRUE", "yes", ""] {
            assert!(hygiene_enabled_from(Some(v)), "{v:?} should enable");
        }
    }
}
