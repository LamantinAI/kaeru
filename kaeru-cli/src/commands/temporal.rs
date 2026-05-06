//! Bi-temporal handle: `at <name> --when <ts>` resolves a name and
//! prints what the node looked like at that moment; `history <name>`
//! prints every assertion / retraction row of the node.
//!
//! These verbs are the visible payoff for kaeru's `Validity`-based
//! substrate — without them the time-travel column is hidden.

use chrono::DateTime;
use chrono::Utc;

use kaeru_core::Result;
use kaeru_core::Store;
use kaeru_core::at as core_at;
use kaeru_core::history as core_history;

use crate::parse::parse_when;
use crate::parse::resolve_name;

pub fn at(store: &Store, name: &str, when: &str) -> Result<()> {
    let id = resolve_name(store, name)?;
    let secs = parse_when(when)?;
    match core_at(store, &id, secs)? {
        Some(snap) => {
            println!("{} @ {}", snap.name, format_ts(secs));
            if let Some(body) = &snap.body {
                println!("{body}");
            } else {
                println!("(no body at this moment)");
            }
        }
        None => {
            println!(
                "(no row valid for {name:?} at {} — node may have been retracted or not yet asserted)",
                format_ts(secs)
            );
        }
    }
    Ok(())
}

pub fn history(store: &Store, name: &str) -> Result<()> {
    let id = resolve_name(store, name)?;
    let revisions = core_history(store, &id)?;
    if revisions.is_empty() {
        println!("(no history — node was never asserted)");
        return Ok(());
    }
    println!("history ({}):", revisions.len());
    for rev in &revisions {
        let mark = if rev.asserted { "+" } else { "-" };
        let ts = format_ts(rev.seconds);
        // Names can change across revisions (improve / supersedes) —
        // include the name that was current at this row.
        println!(
            "  [{mark}] {ts}  {name}",
            mark = mark,
            ts = ts,
            name = if rev.name.is_empty() {
                "(placeholder)"
            } else {
                rev.name.as_str()
            }
        );
        if rev.asserted {
            if let Some(body) = &rev.body {
                let preview = body
                    .lines()
                    .next()
                    .unwrap_or("")
                    .chars()
                    .take(120)
                    .collect::<String>();
                if !preview.is_empty() {
                    println!("       {preview}");
                }
            }
        }
    }
    Ok(())
}

fn format_ts(secs: f64) -> String {
    DateTime::<Utc>::from_timestamp(secs as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| format!("t={secs}"))
}
