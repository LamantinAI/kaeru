//! Write-side commands an agent uses to capture thoughts and
//! connections: `episode`, `jot`, `link`.

use kaeru_core::EdgeType;
use kaeru_core::EpisodeKind;
use kaeru_core::NodeType;
use kaeru_core::Result;
use kaeru_core::Significance;
use kaeru_core::Store;
use kaeru_core::cite as core_cite;
use kaeru_core::jot as core_jot;
use kaeru_core::link as core_link;
use kaeru_core::node_brief_by_id;
use kaeru_core::supersedes as core_supersedes;
use kaeru_core::unlink as core_unlink;
use kaeru_core::write_episode;

use crate::parse::parse_tier;
use crate::parse::resolve_name;
use crate::parse::resolve_name_or_id;

pub fn episode(store: &Store, name: &str, body: &str) -> Result<()> {
    let id = write_episode(
        store,
        EpisodeKind::Observation,
        Significance::Medium,
        name,
        body,
    )?;
    println!("wrote episode: {id}");
    Ok(())
}

pub fn jot(store: &Store, body: &str) -> Result<()> {
    let id = core_jot(store, body)?;
    let brief = node_brief_by_id(store, &id)?;
    match brief {
        Some(b) => println!("jotted: {} — {id}", b.name),
        None => println!("jotted: {id}"),
    }
    Ok(())
}

pub fn link(store: &Store, from: &str, to: &str, edge_type_str: &str) -> Result<()> {
    let edge_type: EdgeType = edge_type_str.parse()?;
    let from_id = resolve_name(store, from)?;
    let to_id = resolve_name(store, to)?;
    core_link(store, &from_id, &to_id, edge_type)?;
    println!("linked: {from} -[{}]-> {to}", edge_type.as_str());
    Ok(())
}

pub fn unlink(store: &Store, from: &str, to: &str, edge_type_str: &str) -> Result<()> {
    let edge_type: EdgeType = edge_type_str.parse()?;
    let from_id = resolve_name(store, from)?;
    let to_id = resolve_name(store, to)?;
    core_unlink(store, &from_id, &to_id, edge_type)?;
    println!("unlinked: {from} -[{}]-> {to}", edge_type.as_str());
    Ok(())
}

pub fn cite(store: &Store, name: &str, url: Option<&str>, body: &str) -> Result<()> {
    let id = core_cite(store, name, url, body)?;
    match url {
        Some(u) => println!("cited: {name} ({u}) — {id}"),
        None => println!("cited: {name} — {id}"),
    }
    Ok(())
}

pub fn supersede(
    store: &Store,
    old: &str,
    new_type_str: &str,
    new_name: &str,
    new_body: &str,
    tier_override: Option<&str>,
) -> Result<()> {
    let old_id = resolve_name_or_id(store, old)?;
    let new_type: NodeType = new_type_str.parse()?;
    let target_tier = match tier_override {
        Some(t) => parse_tier(t)?,
        None => new_type.default_tier(),
    };
    let new_id = core_supersedes(store, &old_id, new_type, target_tier, new_name, new_body)?;
    println!(
        "superseded: {old} → {new_name} ({} / {}) — {new_id}",
        new_type.as_str(),
        target_tier.as_str()
    );
    Ok(())
}
