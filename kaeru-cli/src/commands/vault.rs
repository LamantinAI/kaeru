//! Snapshot the substrate as an Obsidian-friendly markdown vault.

use std::path::Path;

use kaeru_core::Result;
use kaeru_core::Store;
use kaeru_core::export_vault;

pub fn export(store: &Store, output_dir: &Path) -> Result<()> {
    let summary = export_vault(store, output_dir)?;
    let scope = summary
        .initiative
        .as_deref()
        .map(|s| format!("initiative '{s}'"))
        .unwrap_or_else(|| "all initiatives".to_string());
    println!(
        "exported {} node(s), {} edge(s) of {} → {}",
        summary.nodes_exported,
        summary.edges_exported,
        scope,
        summary.root.display(),
    );
    Ok(())
}
