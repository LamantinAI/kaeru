//! Shared utilities used across `tools/<group>.rs`. Two flavours:
//!
//! - **Output builders**: `text`, `render_summary`, `render_briefs`,
//!   `brief_suffix` — turn kaeru-core results into `CallToolResult`
//!   text content.
//! - **Input/state helpers**: `to_mcp`, `with_initiative`,
//!   `resolve_name`, `resolve_name_or_id`, the `parse_*` family —
//!   massage CLI-style strings into typed core arguments.
//!
//! Nothing in here knows about specific MCP tools; it's pure glue.
//! Call sites live in `tools/<group>.rs`.

use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;
use rmcp::model::Content;

use kaeru_core::Error;
use kaeru_core::NodeBrief;
use kaeru_core::NodeId;
use kaeru_core::Store;
use kaeru_core::SummaryView;
use kaeru_core::Tier;

// =========================================================================
// Output builders
// =========================================================================

pub fn text(s: &str) -> CallToolResult {
    CallToolResult::success(vec![Content::text(s)])
}

pub fn to_mcp(e: Error) -> McpError {
    McpError::internal_error(e.to_string(), None)
}

/// Renders ` — <name>` for an id when its brief resolves at NOW;
/// returns `""` when the id is unknown. Used as a suffix in
/// `awake` / `recent` / `lint` outputs to give a human-readable
/// label next to each id.
pub fn brief_suffix(store: &Store, id: &str) -> String {
    match kaeru_core::node_brief_by_id(store, &id.to_string()) {
        Ok(Some(b)) => format!(" — {}", b.name),
        _ => String::new(),
    }
}

pub fn render_summary(view: &SummaryView) -> String {
    let mut out = format!(
        "{} ({}) — {}\n",
        view.root.name, view.root.node_type, view.root.id
    );
    if let Some(e) = &view.root.body_excerpt {
        out.push_str(&format!("  {e}\n"));
    }
    if view.children.is_empty() {
        out.push_str("(no drill-down children)\n");
    } else {
        out.push_str(&format!("children ({}):\n", view.children.len()));
        for c in &view.children {
            out.push_str(&format!("  - {} ({}) — {}\n", c.name, c.node_type, c.id));
            if let Some(e) = &c.body_excerpt {
                out.push_str(&format!("    {e}\n"));
            }
        }
    }
    out
}

pub fn render_briefs(label: &str, briefs: &[NodeBrief]) -> String {
    if briefs.is_empty() {
        return format!("{label} (0): (empty)");
    }
    let mut out = format!("{label} ({}):\n", briefs.len());
    for b in briefs {
        out.push_str(&format!("  - {} ({}) — {}\n", b.name, b.node_type, b.id));
        if let Some(e) = &b.body_excerpt {
            out.push_str(&format!("    {e}\n"));
        }
    }
    out
}

// =========================================================================
// Initiative scoping + name resolution
// =========================================================================

/// Sets the store's current initiative (or clears it), runs `f`, then
/// restores the previous initiative. Stdio MCP processes one tool call
/// at a time at the protocol level, but rmcp dispatches handlers onto
/// tokio tasks — so we restore state per call rather than relying on
/// strict serialization.
pub fn with_initiative<T>(
    store: &Store,
    initiative: Option<&str>,
    f: impl FnOnce() -> Result<T, McpError>,
) -> Result<T, McpError> {
    let prev = store.current_initiative();
    match initiative {
        Some(name) => store.use_initiative(name),
        None => store.clear_initiative(),
    }
    let result = f();
    match prev {
        Some(p) => store.use_initiative(&p),
        None => store.clear_initiative(),
    }
    result
}

pub fn resolve_name(store: &Store, name: &str) -> Result<NodeId, McpError> {
    kaeru_core::recall_id_by_name(store, name)
        .map_err(to_mcp)?
        .ok_or_else(|| {
            to_mcp(Error::NotFound(format!("no node named {name:?} at NOW")))
        })
}

/// UUIDv7 has 36 chars with dashes at fixed positions; cheap heuristic
/// avoids a roundtrip when the caller already has an id.
pub fn resolve_name_or_id(store: &Store, input: &str) -> Result<NodeId, McpError> {
    if input.len() == 36 && input.chars().nth(8) == Some('-') {
        return Ok(input.to_string());
    }
    resolve_name(store, input)
}

// =========================================================================
// Parsing
// =========================================================================

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
pub fn parse_due_to_iso(s: &str) -> Result<String, McpError> {
    let trimmed = s.trim();
    if let Some(last) = trimmed.chars().last() {
        if matches!(last, 's' | 'm' | 'h' | 'd' | 'w')
            && trimmed[..trimmed.len() - 1]
                .chars()
                .all(|c| c.is_ascii_digit())
        {
            let secs = parse_duration_secs(trimmed).map_err(to_mcp)?;
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let future = (now.saturating_add(secs)) as i64;
            return Ok(format_iso_date(future));
        }
    }
    let secs = parse_when(trimmed).map_err(to_mcp)?;
    Ok(format_iso_date(secs as i64))
}

fn format_iso_date(unix_secs: i64) -> String {
    DateTime::<Utc>::from_timestamp(unix_secs, 0)
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| format!("t-{unix_secs}"))
}

pub fn parse_tier(s: &str) -> Result<Tier, Error> {
    match s.to_lowercase().as_str() {
        "operational" | "op" => Ok(Tier::Operational),
        "archival" | "ar" => Ok(Tier::Archival),
        _ => Err(Error::Invalid(format!("unknown tier {s:?}"))),
    }
}

pub fn derive_auto_name(text: &str, fallback: &str) -> String {
    const MAX_WORDS: usize = 5;
    let mut words: Vec<String> = Vec::new();
    for raw in text.split_whitespace() {
        let cleaned: String = raw
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect::<String>()
            .to_lowercase();
        if !cleaned.is_empty() {
            words.push(cleaned);
            if words.len() >= MAX_WORDS {
                break;
            }
        }
    }
    let id = kaeru_core::new_node_id();
    let suffix: String = id
        .chars()
        .rev()
        .take(6)
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    if words.is_empty() {
        format!("{fallback}-{suffix}")
    } else {
        format!("{}-{suffix}", words.join("-"))
    }
}
