//! `lint` — diagnostic snapshot: orphan nodes and the open-review
//! queue.

use kaeru_core::Result;
use kaeru_core::Store;
use kaeru_core::lint as core_lint;

use crate::format::print_id_with_name;

pub fn lint(store: &Store) -> Result<()> {
    let report = core_lint(store)?;
    println!("orphans ({}):", report.orphans.len());
    for id in &report.orphans {
        print_id_with_name(store, id)?;
    }
    println!();
    println!(
        "unresolved reviews ({}):",
        report.unresolved_reviews.len()
    );
    for id in &report.unresolved_reviews {
        print_id_with_name(store, id)?;
    }
    println!();
    Ok(())
}
