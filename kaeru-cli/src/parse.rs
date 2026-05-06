//! Small CLI-side parsing and resolution helpers.
//!
//! - `resolve_name` : name → NodeId at NOW (errors on miss; saves
//!   every command from repeating the same `recall + ok_or_else`).
//! - `derive_auto_name` : auto-name from free-form text + short id
//!   suffix; used by `claim` / `test` so the agent doesn't have to
//!   invent names for fleeting nodes.
//! - `parse_duration_secs` : `30m` / `3h` / `2d` / raw seconds.
//! - `parse_tier` : `operational` / `archival` (with abbreviations).

use chrono::DateTime;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use kaeru_core::Error;
use kaeru_core::NodeId;
use kaeru_core::Result;
use kaeru_core::Store;
use kaeru_core::Tier;
use kaeru_core::new_node_id;
use kaeru_core::recall_id_by_name;

/// Resolves a node name to its id at NOW (initiative-scoped if a
/// current initiative is set on the store). Returns `Error::NotFound`
/// rather than `None` so call sites can `?`.
pub fn resolve_name(store: &Store, name: &str) -> Result<NodeId> {
    recall_id_by_name(store, name)?
        .ok_or_else(|| Error::NotFound(format!("no node named {name:?} at NOW")))
}

/// Polymorphic resolver — accepts either a UUIDv7 id or a node name.
/// If the input parses as a UUID it's used verbatim; otherwise it's
/// resolved through `recall_id_by_name`. This lets agent-facing
/// commands (`pin`, `unpin`, `summary`, `forget`, …) take whichever
/// form the caller has at hand.
pub fn resolve_name_or_id(store: &Store, input: &str) -> Result<NodeId> {
    if uuid::Uuid::parse_str(input).is_ok() {
        Ok(input.to_string())
    } else {
        resolve_name(store, input)
    }
}

/// Auto-name from a free-form text plus a 6-character suffix from a
/// fresh node id. Two calls with identical text get distinct names.
/// `fallback_prefix` is used when the text yields no usable tokens
/// (e.g. `"   "` → `"<prefix>-<suffix>"`).
pub fn derive_auto_name(text: &str, fallback_prefix: &str) -> String {
    const MAX_WORDS: usize = 5;
    let mut words: Vec<String> = Vec::new();
    for raw in text.split_whitespace() {
        let cleaned: String = raw
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect::<String>()
            .to_lowercase();
        if cleaned.is_empty() {
            continue;
        }
        words.push(cleaned);
        if words.len() >= MAX_WORDS {
            break;
        }
    }
    let id = new_node_id();
    let id_suffix: String = id
        .chars()
        .rev()
        .take(6)
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    if words.is_empty() {
        format!("{fallback_prefix}-{id_suffix}")
    } else {
        format!("{}-{id_suffix}", words.join("-"))
    }
}

/// Parses a duration like `30m`, `3h`, `2d`, or a raw integer (seconds)
/// into seconds. Used by `recent --since`.
pub fn parse_duration_secs(s: &str) -> Result<u64> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(Error::Invalid("empty duration".to_string()));
    }
    if trimmed.chars().all(|c| c.is_ascii_digit()) {
        return trimmed
            .parse::<u64>()
            .map_err(|e| Error::Invalid(format!("bad seconds: {e}")));
    }
    let (num_part, unit) = trimmed.split_at(trimmed.len() - 1);
    let n: u64 = num_part
        .parse()
        .map_err(|e| Error::Invalid(format!("bad duration {trimmed:?}: {e}")))?;
    let multiplier: u64 = match unit {
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
    Ok(n.saturating_mul(multiplier))
}

/// Parses a `--when` argument to a Unix-seconds float, accepting:
///   - pure digits (with optional decimal): treated as Unix seconds.
///   - duration suffix (`30m`, `2h`, `3d`): "that long ago" relative
///     to NOW. So `--when 5m` means "5 minutes ago".
///   - everything else: tried as RFC-3339 datetime
///     (`2026-05-06T12:00:00Z`).
pub fn parse_when(s: &str) -> Result<f64> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Err(Error::Invalid("empty timestamp".to_string()));
    }

    // Pure digits / decimal → unix seconds.
    if trimmed.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return trimmed
            .parse::<f64>()
            .map_err(|e| Error::Invalid(format!("bad seconds {trimmed:?}: {e}")));
    }

    // Duration-suffix forms — interpret as "ago".
    if let Some(last) = trimmed.chars().last() {
        if matches!(last, 's' | 'm' | 'h' | 'd' | 'w')
            && trimmed[..trimmed.len() - 1]
                .chars()
                .all(|c| c.is_ascii_digit())
        {
            let secs = parse_duration_secs(trimmed)?;
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            return Ok(now.saturating_sub(secs) as f64);
        }
    }

    // Fallback — RFC 3339 datetime.
    DateTime::parse_from_rfc3339(trimmed)
        .map(|dt| dt.timestamp() as f64)
        .map_err(|e| Error::Invalid(format!("bad timestamp {trimmed:?}: {e}")))
}

/// Parses `Tier` from `operational` / `archival` (case-insensitive),
/// plus the abbreviations `op` / `ar` for terminal use.
pub fn parse_tier(s: &str) -> Result<Tier> {
    match s.to_lowercase().as_str() {
        "operational" | "op" => Ok(Tier::Operational),
        "archival" | "ar" => Ok(Tier::Archival),
        _ => Err(Error::Invalid(format!(
            "unknown tier {s:?} (expected `operational` or `archival`)"
        ))),
    }
}
