//! Terminal-output helpers shared across command handlers.
//!
//! Plain text by design — structured (JSON) output is a future
//! addition layered on top, not a replacement.

use kaeru_core::NodeBrief;
use kaeru_core::Result;
use kaeru_core::Store;
use kaeru_core::node_brief_by_id;

/// Prints `<label> (count):` followed by `  - <id> — <name>` per row,
/// or `(empty)` when the section has no entries. Used by `awake`,
/// `recent`, etc. Always finishes with a blank line so consecutive
/// sections in the same command are visually separated.
pub fn print_id_section(store: &Store, label: &str, ids: &[String]) -> Result<()> {
    if ids.is_empty() {
        println!("{label} (0): (empty)");
        println!();
        return Ok(());
    }
    println!("{label} ({}):", ids.len());
    for id in ids {
        print_id_with_name(store, id)?;
    }
    println!();
    Ok(())
}

/// Prints `  - <id> — <name>` for a single id, falling back to
/// `(unknown)` if the brief lookup returns None.
pub fn print_id_with_name(store: &Store, id: &str) -> Result<()> {
    match node_brief_by_id(store, &id.to_string())? {
        Some(brief) => println!("  - {id} — {}", brief.name),
        None => println!("  - {id} — (unknown)"),
    }
    Ok(())
}

/// Renders a `Vec<NodeBrief>` as a short bulleted section. Used by
/// `ideas`, `outcomes`, `trace`, etc. Always finishes with a blank line.
pub fn print_brief_section(label: &str, briefs: &[NodeBrief]) {
    if briefs.is_empty() {
        println!("{label} (0): (empty)");
        println!();
        return;
    }
    println!("{label} ({}):", briefs.len());
    for brief in briefs {
        println!("  - {} ({}) — {}", brief.name, brief.node_type, brief.id);
        if let Some(excerpt) = &brief.body_excerpt {
            println!("    {excerpt}");
        }
    }
    println!();
}
