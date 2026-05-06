//! Hygiene mutations: `forget` (bi-temporal retract of node + edges)
//! and `revise` (rewrite name and/or body in place).

use kaeru_core::Error;
use kaeru_core::Result;
use kaeru_core::Store;
use kaeru_core::forget as core_forget;
use kaeru_core::improve;
use kaeru_core::node_brief_by_id;
use kaeru_core::summary_view;

use crate::parse::resolve_name_or_id;

pub fn forget(store: &Store, name: &str) -> Result<()> {
    let id = resolve_name_or_id(store, name)?;
    core_forget(store, &id)?;
    println!("forgot: {name}");
    Ok(())
}

pub fn revise(
    store: &Store,
    name: &str,
    body: Option<&str>,
    rename: Option<&str>,
) -> Result<()> {
    let id = resolve_name_or_id(store, name)?;

    // Read current name + body so omitted args fall back to the
    // existing values rather than nullifying them.
    let brief = node_brief_by_id(store, &id)?
        .ok_or_else(|| Error::NotFound(format!("node {name:?} not found at NOW")))?;
    let new_name = rename.unwrap_or(&brief.name);

    // The brief excerpt is truncated; for revise we want the full
    // body, so re-read via summary_view's root path.
    let preserved_body: String = match summary_view(store, &id)?.root.body_excerpt {
        Some(b) => b,
        None => String::new(),
    };
    let new_body = body.unwrap_or(&preserved_body);

    improve(store, &id, new_name, new_body)?;
    println!("revised: {name} → {new_name}");
    Ok(())
}
