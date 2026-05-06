//! Read-side commands: `recall`, `drill`, `trace`, `search`,
//! `summary`, `ideas`, `outcomes`.

use kaeru_core::Result;
use kaeru_core::Store;
use kaeru_core::between as core_between;
use kaeru_core::fuzzy_recall;
use kaeru_core::recall_id_by_name;
use kaeru_core::recollect_idea;
use kaeru_core::recollect_outcome;
use kaeru_core::recollect_provenance;
use kaeru_core::summary_view;
use kaeru_core::tagged as core_tagged;

use crate::format::print_brief_section;
use crate::parse::resolve_name;
use crate::parse::resolve_name_or_id;

pub fn recall(store: &Store, name: &str) -> Result<()> {
    match recall_id_by_name(store, name)? {
        Some(id) => println!("{id}"),
        None => println!("(not found)"),
    }
    Ok(())
}

pub fn search(store: &Store, query: &str, limit: usize) -> Result<()> {
    let hits = fuzzy_recall(store, query, limit)?;
    if hits.is_empty() {
        println!("(no matches)");
    } else {
        println!("matches ({}):", hits.len());
        for brief in &hits {
            println!("  - {} ({}) — {}", brief.name, brief.node_type, brief.id);
            if let Some(excerpt) = &brief.body_excerpt {
                println!("    {excerpt}");
            }
        }
    }
    Ok(())
}

pub fn summary(store: &Store, name_or_id: &str) -> Result<()> {
    let id = resolve_name_or_id(store, name_or_id)?;
    let view = summary_view(store, &id)?;
    render_summary(&view);
    Ok(())
}

pub fn drill(store: &Store, name: &str) -> Result<()> {
    let id = resolve_name(store, name)?;
    let view = summary_view(store, &id)?;
    render_summary(&view);
    Ok(())
}

fn render_summary(view: &kaeru_core::SummaryView) {
    println!(
        "{} ({}) — {}",
        view.root.name, view.root.node_type, view.root.id
    );
    if let Some(excerpt) = &view.root.body_excerpt {
        println!("  {excerpt}");
    }
    if view.children.is_empty() {
        println!("(no drill-down children)");
    } else {
        println!("children ({}):", view.children.len());
        for child in &view.children {
            println!("  - {} ({}) — {}", child.name, child.node_type, child.id);
            if let Some(excerpt) = &child.body_excerpt {
                println!("    {excerpt}");
            }
        }
    }
}

pub fn trace(store: &Store, name: &str) -> Result<()> {
    let id = resolve_name(store, name)?;
    let ancestors = recollect_provenance(store, &id)?;
    if ancestors.is_empty() {
        println!("(no provenance — node has no derived_from ancestors)");
    } else {
        print_brief_section("provenance", &ancestors);
    }
    Ok(())
}

pub fn ideas(store: &Store) -> Result<()> {
    let briefs = recollect_idea(store)?;
    print_brief_section("ideas", &briefs);
    Ok(())
}

pub fn outcomes(store: &Store) -> Result<()> {
    let briefs = recollect_outcome(store)?;
    print_brief_section("outcomes", &briefs);
    Ok(())
}

pub fn tagged(store: &Store, tag: &str) -> Result<()> {
    let briefs = core_tagged(store, tag)?;
    print_brief_section(&format!("tagged `{tag}`"), &briefs);
    Ok(())
}

pub fn between(store: &Store, a: &str, b: &str) -> Result<()> {
    let a_id = resolve_name(store, a)?;
    let b_id = resolve_name(store, b)?;
    let edges = core_between(store, &a_id, &b_id)?;
    if edges.is_empty() {
        println!("(no edges between {a:?} and {b:?} at NOW)");
        return Ok(());
    }
    println!("edges between {a} and {b} ({}):", edges.len());
    for e in &edges {
        if e.a_to_b {
            println!("  {a} —[{}]→ {b}", e.edge_type);
        } else {
            println!("  {a} ←[{}]— {b}", e.edge_type);
        }
    }
    Ok(())
}
