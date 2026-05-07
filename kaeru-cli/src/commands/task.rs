//! Personal-life capture: `task` (todo with optional due date) and
//! `done` (mark a task complete).

use chrono::DateTime;
use chrono::Utc;

use kaeru_core::Error;
use kaeru_core::Result;
use kaeru_core::Store;
use kaeru_core::complete_task;
use kaeru_core::node_brief_by_id;
use kaeru_core::write_task;

use crate::parse::parse_when;
use crate::parse::resolve_name_or_id;

pub fn task(store: &Store, body: &str, due: Option<&str>) -> Result<()> {
    let due_iso = match due {
        Some(d) => Some(parse_due_to_iso(d)?),
        None => None,
    };
    let id = write_task(store, body, due_iso.as_deref())?;
    let brief = node_brief_by_id(store, &id)?;
    match (brief, due_iso.as_deref()) {
        (Some(b), Some(d)) => println!("task: {} (due {d}) — {id}", b.name),
        (Some(b), None) => println!("task: {} — {id}", b.name),
        (None, Some(d)) => println!("task: {id} (due {d})"),
        (None, None) => println!("task: {id}"),
    }
    Ok(())
}

pub fn done(store: &Store, name_or_id: &str) -> Result<()> {
    let id = resolve_name_or_id(store, name_or_id)?;
    complete_task(store, &id)?;
    println!("done: {name_or_id}");
    Ok(())
}

/// Converts a user-friendly `--due` string into an ISO `YYYY-MM-DD`
/// tag value. Accepts the same forms as `parse_when`: unix seconds,
/// RFC-3339 datetime, or a duration suffix interpreted as **future**
/// from now (`3d` = "in 3 days", not "3 days ago"). Errors with
/// `Error::Invalid` on bad input.
fn parse_due_to_iso(s: &str) -> Result<String> {
    let trimmed = s.trim();
    // Duration suffix → future. parse_when treats it as past, so for
    // `--due` we negate: 3d ago + 6d into future = 3d future. Easier
    // to do manually: detect duration form, add to now.
    if let Some(last) = trimmed.chars().last() {
        if matches!(last, 's' | 'm' | 'h' | 'd' | 'w')
            && trimmed[..trimmed.len() - 1]
                .chars()
                .all(|c| c.is_ascii_digit())
        {
            let secs = duration_secs(trimmed)?;
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let future = now.saturating_add(secs);
            return Ok(format_iso_date(future as i64));
        }
    }
    // Otherwise reuse parse_when (handles unix seconds + RFC-3339).
    let secs = parse_when(trimmed)?;
    Ok(format_iso_date(secs as i64))
}

fn duration_secs(s: &str) -> Result<u64> {
    let (num, unit) = s.split_at(s.len() - 1);
    let n: u64 = num
        .parse()
        .map_err(|e| Error::Invalid(format!("bad duration {s:?}: {e}")))?;
    let mult: u64 = match unit {
        "s" => 1,
        "m" => 60,
        "h" => 3600,
        "d" => 86_400,
        "w" => 7 * 86_400,
        other => {
            return Err(Error::Invalid(format!(
                "unknown duration unit {other:?} (use s/m/h/d/w)"
            )));
        }
    };
    Ok(n.saturating_mul(mult))
}

fn format_iso_date(unix_secs: i64) -> String {
    DateTime::<Utc>::from_timestamp(unix_secs, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| format!("t-{unix_secs}"))
}
