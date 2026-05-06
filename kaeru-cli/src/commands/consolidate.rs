//! Tier-promotion / many-to-one consolidation:
//! `settle` (operational → archival), `reopen` (archival → operational),
//! `synthesise` (many seeds → one new node).

use kaeru_core::Error;
use kaeru_core::NodeId;
use kaeru_core::NodeType;
use kaeru_core::Result;
use kaeru_core::Store;
use kaeru_core::consolidate_in;
use kaeru_core::consolidate_out;
use kaeru_core::synthesise as core_synthesise;

use crate::parse::parse_tier;
use crate::parse::resolve_name;

pub fn settle(
    store: &Store,
    draft: &str,
    new_type_str: &str,
    new_name: &str,
    new_body: &str,
) -> Result<()> {
    let draft_id = resolve_name(store, draft)?;
    let new_type: NodeType = new_type_str.parse()?;
    let new_id = consolidate_out(store, &draft_id, new_type, new_name, new_body)?;
    println!(
        "settled: {draft} → {new_name} ({}) — {new_id}",
        new_type.as_str()
    );
    Ok(())
}

pub fn reopen(
    store: &Store,
    archival: &str,
    new_type_str: &str,
    new_name: &str,
    new_body: &str,
) -> Result<()> {
    let archival_id = resolve_name(store, archival)?;
    let new_type: NodeType = new_type_str.parse()?;
    let new_id = consolidate_in(store, &archival_id, new_type, new_name, new_body)?;
    println!(
        "reopened: {archival} → {new_name} ({}) — {new_id}",
        new_type.as_str()
    );
    Ok(())
}

pub fn synthesise(
    store: &Store,
    from_names: &[String],
    new_type_str: &str,
    new_name: &str,
    new_body: &str,
    tier_override: Option<&str>,
) -> Result<()> {
    if from_names.is_empty() {
        return Err(Error::Invalid(
            "--from must list at least one seed".to_string(),
        ));
    }
    let new_type: NodeType = new_type_str.parse()?;
    let target_tier = match tier_override {
        Some(t) => parse_tier(t)?,
        None => new_type.default_tier(),
    };
    let mut seed_ids: Vec<NodeId> = Vec::with_capacity(from_names.len());
    for seed_name in from_names {
        seed_ids.push(resolve_name(store, seed_name)?);
    }
    let new_id = core_synthesise(store, &seed_ids, new_type, target_tier, new_name, new_body)?;
    println!(
        "synthesised: {new_name} ({} / {}) — {new_id}",
        new_type.as_str(),
        target_tier.as_str()
    );
    Ok(())
}
