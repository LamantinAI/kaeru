//! Offline operator access; the daemon's RocksDB lock is never bypassed.

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::{Args, Subcommand};
use kaeru_core::config::amg::AmgLimits;
use kaeru_core::models::amg::AmgSelection;
use kaeru_core::{KaeruConfig, Store, amg};
use serde_json::json;

#[derive(Args)]
pub(super) struct Options {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Read metadata and validate a document without opening a vault.
    Inspect { file: PathBuf },
    /// Write a portable snapshot; requires --all or repeated --initiative NAME.
    Export {
        #[arg(long)]
        vault: Option<PathBuf>,
        #[arg(
            long,
            required_unless_present = "initiative",
            conflicts_with = "initiative"
        )]
        all: bool,
        #[arg(long = "initiative")]
        initiative: Vec<String>,
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Atomically merge a document; force accepts new current node/edge versions.
    Import {
        file: PathBuf,
        #[arg(long)]
        vault: Option<PathBuf>,
        #[arg(long)]
        force: bool,
        /// Validate the destination merge without applying knowledge changes.
        #[arg(long)]
        dry_run: bool,
    },
}

fn store(path: Option<PathBuf>) -> Result<Store, Box<dyn std::error::Error>> {
    let mut config = KaeruConfig::from_env()?;
    if let Some(path) = path {
        config.vault_path = path;
    }
    Ok(Store::open_with_config(config).map_err(|error|format!("Cannot open AMG vault: {error}. Stop the daemon before offline export/import; do not remove its lock."))?)
}

fn atomic_file(path: &Path, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    let parent = path.parent().ok_or("Output needs a parent directory")?;
    std::fs::create_dir_all(parent)?;
    let mut file = tempfile::NamedTempFile::new_in(parent)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        file.as_file()
            .set_permissions(std::fs::Permissions::from_mode(0o600))?;
    }
    file.write_all(bytes)?;
    file.as_file_mut().sync_all()?;
    file.persist_noclobber(&path).map_err(|error| error.error)?;
    #[cfg(unix)]
    if let Err(error) = File::open(parent).and_then(|dir| dir.sync_all()) {
        eprintln!("AMG file was written, but directory sync failed: {error}");
    }
    Ok(())
}

pub(super) fn run(options: Options) -> Result<(), Box<dyn std::error::Error>> {
    let limits = AmgLimits::default();
    match options.command {
        Command::Inspect { file } => {
            let doc = amg::decode(File::open(file)?, &limits)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({"metadata":doc.metadata,
                "nodes":doc.data.nodes.len(),"edges":doc.data.edges.len(),"trails":doc.data.trails.len(),
                "slots":doc.data.slots.len(),"decisions":doc.data.decisions.len()}))?
            );
        }
        Command::Export {
            vault,
            all,
            initiative,
            output,
        } => {
            let selection = if all {
                AmgSelection::All
            } else {
                AmgSelection::Initiatives(initiative)
            };
            let store = store(vault)?;
            let doc = amg::snapshot(&store, &selection, &limits)?;
            let bytes = amg::encode(&doc, &limits)?;
            atomic_file(&output, &bytes)?;
            println!(
                "{}",
                serde_json::to_string_pretty(&json!({"file":output,"metadata":doc.metadata,
                "nodes":doc.data.nodes.len(),"edges":doc.data.edges.len(),"bytes":bytes.len()}))?
            );
        }
        Command::Import {
            file,
            vault,
            force,
            dry_run,
        } => {
            // Bad framing or unsupported schema must fail before opening (and
            // potentially bootstrapping/migrating) the destination substrate.
            let doc = amg::decode(File::open(file)?, &limits)?;
            let store = store(vault)?;
            let result = amg::import(&store, &doc, force, dry_run, &limits)?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    }
    Ok(())
}
