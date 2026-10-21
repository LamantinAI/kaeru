//! Operator commands are parsed before any vault, listener or background task.

use clap::{ArgGroup, Parser, Subcommand};

mod amg;
mod auth;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Startup {
    Serve,
    Stdio,
    Handled,
}

#[derive(Parser)]
#[command(
    version,
    about = "Kaeru cognitive memory for agents",
    args_conflicts_with_subcommands = true,
    group(ArgGroup::new("auth_action").multiple(false).args(["init_web_auth", "reset_web_password", "auth", "auth_status"]))
)]
struct Options {
    #[command(flatten)]
    auth: auth::AuthOptions,
    /// Bridge standard input/output to the running MCP service.
    #[arg(long, conflicts_with = "auth_action")]
    stdio: bool,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Inspect, export and import portable current-knowledge graph documents.
    Amg(amg::Options),
}

pub(crate) fn run() -> Result<Startup, Box<dyn std::error::Error>> {
    let options = Options::parse();
    if let Some(Command::Amg(args)) = options.command {
        amg::run(args)?;
        return Ok(Startup::Handled);
    }
    if options.auth.init_web_auth
        || options.auth.reset_web_password
        || options.auth.auth.is_some()
        || options.auth.auth_status
    {
        auth::handle(options.auth)?;
        return Ok(Startup::Handled);
    }
    Ok(if options.stdio {
        Startup::Stdio
    } else {
        Startup::Serve
    })
}
