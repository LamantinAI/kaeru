//! Session-restoration & vault meta commands: `awake`, `overview`,
//! `initiatives`, `recent`, `pin`, `unpin`, `config`.

use kaeru_core::Result;
use kaeru_core::Store;
use kaeru_core::active_window;
use kaeru_core::awake as core_awake;
use kaeru_core::list_initiatives;
use kaeru_core::overview as core_overview;
use kaeru_core::pin as core_pin;
use kaeru_core::recent_episodes;
use kaeru_core::unpin as core_unpin;
use kaeru_core::version;

use crate::format::print_id_section;
use crate::parse::parse_duration_secs;
use crate::parse::resolve_name_or_id;

pub fn awake(store: &Store) -> Result<()> {
    let ctx = core_awake(store)?;
    println!(
        "initiative: {}",
        ctx.initiative.as_deref().unwrap_or("(none)")
    );
    println!();
    print_id_section(store, "pinned", &ctx.pinned)?;
    print_id_section(store, "recent", &ctx.recent)?;
    print_id_section(store, "under review", &ctx.under_review)?;
    Ok(())
}

pub fn overview(store: &Store) -> Result<()> {
    let report = core_overview(store)?;
    print!("{report}");
    Ok(())
}

pub fn initiatives(store: &Store) -> Result<()> {
    let names = list_initiatives(store)?;
    if names.is_empty() {
        println!("(no initiatives yet — run a mutation with `--initiative <name>`)");
    } else {
        println!("initiatives ({}):", names.len());
        for name in &names {
            println!("  - {name}");
        }
    }
    Ok(())
}

pub fn recent(store: &Store, since: &str) -> Result<()> {
    let window = parse_duration_secs(since)?;
    let ids = recent_episodes(store, window)?;
    print_id_section(store, "recent", &ids)?;
    Ok(())
}

pub fn pin(store: &Store, name_or_id: &str, reason: &str) -> Result<()> {
    let id = resolve_name_or_id(store, name_or_id)?;
    core_pin(store, &id, reason)?;
    let pinned = active_window(store)?;
    println!("pinned: {name_or_id} ({id})");
    println!("active window now {} node(s)", pinned.len());
    Ok(())
}

pub fn unpin(store: &Store, name_or_id: &str) -> Result<()> {
    let id = resolve_name_or_id(store, name_or_id)?;
    core_unpin(store, &id)?;
    println!("unpinned: {name_or_id} ({id})");
    Ok(())
}

pub fn config(store: &Store) -> Result<()> {
    let c = store.config();
    println!("kaeru {}", version());
    println!("vault_path           = {}", c.vault_path.display());
    println!("active_window_size   = {}", c.active_window_size);
    println!("recent_episodes_cap  = {}", c.recent_episodes_cap);
    println!("awake_window_secs    = {}", c.awake_default_window_secs);
    println!("summary_children_cap = {}", c.summary_view_children_cap);
    println!("body_excerpt_chars   = {}", c.body_excerpt_chars);
    println!("provenance_max_hops  = {}", c.provenance_max_hops);
    println!("default_max_hops     = {}", c.default_max_hops);
    println!("max_hops_cap         = {}", c.max_hops_cap);
    Ok(())
}
