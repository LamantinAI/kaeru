//! Human time strings — `2h`, `2026-05-06`, a raw unix second — parsed once.
//!
//! Every adapter takes these: `at --when`, `recent --since`, `task --due`.
//! They used to be parsed in `kaeru-mcp`, which meant the rig adapter either
//! copied them or (as it did) took a bare number instead — so the same verb
//! answered to different arguments depending on how the agent reached kaeru
//! (#98). The vocabulary is part of the API, so it belongs in the core.

use chrono::{DateTime, NaiveDate, Utc};

use crate::errors::Error;

pub fn parse_duration_secs(s: &str) -> Result<u64, Error> {
    let trimmed = s.trim();
    if trimmed.chars().all(|c| c.is_ascii_digit()) {
        return trimmed
            .parse::<u64>()
            .map_err(|e| Error::Invalid(format!("bad seconds: {e}")));
    }
    if trimmed.is_empty() {
        return Err(Error::Invalid("empty duration".to_string()));
    }
    let (num, unit) = trimmed.split_at(trimmed.len() - 1);
    let n: u64 = num
        .parse()
        .map_err(|e| Error::Invalid(format!("bad duration: {e}")))?;
    let mult: u64 = match unit {
        "s" => 1,
        "m" => 60,
        "h" => 3600,
        "d" => 86_400,
        "w" => 7 * 86_400,
        other => {
            return Err(Error::Invalid(format!(
                "unknown unit {other:?} (use s/m/h/d/w)"
            )));
        }
    };
    Ok(n.saturating_mul(mult))
}

/// Parses a `--when` argument to a Unix-seconds float, accepting:
///   - pure digits (with optional decimal): treated as Unix seconds.
///   - duration suffix (`5m`, `2h`, `3d`): "that long ago" relative to NOW.
///   - bare ISO date (`YYYY-MM-DD`): treated as UTC midnight.
///   - RFC-3339 datetime (`2026-05-06T12:00:00Z`).
pub fn parse_when(s: &str) -> Result<f64, Error> {
    let trimmed = s.trim();
    if trimmed.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return trimmed
            .parse::<f64>()
            .map_err(|e| Error::Invalid(format!("bad seconds: {e}")));
    }
    if let Some(last) = trimmed.chars().last() {
        if matches!(last, 's' | 'm' | 'h' | 'd' | 'w')
            && trimmed[..trimmed.len() - 1]
                .chars()
                .all(|c| c.is_ascii_digit())
        {
            let secs = parse_duration_secs(trimmed)?;
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            return Ok(now.saturating_sub(secs) as f64);
        }
    }
    if let Ok(date) = NaiveDate::parse_from_str(trimmed, "%Y-%m-%d") {
        let dt = date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| Error::Invalid(format!("bad date {trimmed:?}")))?
            .and_utc();
        return Ok(dt.timestamp() as f64);
    }
    DateTime::parse_from_rfc3339(trimmed)
        .map(|dt| dt.timestamp() as f64)
        .map_err(|e| Error::Invalid(format!("bad timestamp {trimmed:?}: {e}")))
}

/// Converts a user-friendly `--due` string into ISO `YYYY-MM-DD`.
/// Duration suffixes (`3d`/`2w`) are interpreted as **future** from
/// now (opposite of `--when` for `at`). Bare dates and RFC-3339
/// datetimes pass through `parse_when`.
pub fn parse_due_to_iso(s: &str) -> Result<String, Error> {
    let trimmed = s.trim();
    if let Some(last) = trimmed.chars().last() {
        if matches!(last, 's' | 'm' | 'h' | 'd' | 'w')
            && trimmed[..trimmed.len() - 1]
                .chars()
                .all(|c| c.is_ascii_digit())
        {
            let secs = parse_duration_secs(trimmed)?;
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let future = (now.saturating_add(secs)) as i64;
            return Ok(format_iso_date(future));
        }
    }
    let secs = parse_when(trimmed)?;
    Ok(format_iso_date(secs as i64))
}

fn format_iso_date(unix_secs: i64) -> String {
    DateTime::<Utc>::from_timestamp(unix_secs, 0)
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| format!("t-{unix_secs}"))
}
