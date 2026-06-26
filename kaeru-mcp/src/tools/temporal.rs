//! Bi-temporal handle: `at`, `history`.

use std::time::{SystemTime, UNIX_EPOCH};

use kaeru_core::{NodeSnapshot, Store};
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{fmt_ts, parse_when, resolve_name, text, to_mcp, with_initiative};

/// Reads a node **in full** — every field plus the complete, untruncated
/// body. `when` is optional: omit it to read the node as it is now, or pass
/// a moment to time-travel to that point. The full-read counterpart to the
/// truncating `drill` / `search` / `recall`.
pub fn at(
    store: &Store,
    name: &str,
    when: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name(store, name)?;
        let secs = match when {
            Some(w) if !w.trim().is_empty() => parse_when(w).map_err(to_mcp)?,
            _ => now_seconds(),
        };
        match kaeru_core::at(store, &id, secs).map_err(to_mcp)? {
            Some(snap) => Ok(text(&render(&snap, when))),
            None => Ok(text("(no version valid at that moment)")),
        }
    })
}

pub fn history(
    store: &Store,
    name: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name(store, name)?;
        let revs = kaeru_core::history(store, &id).map_err(to_mcp)?;
        if revs.is_empty() {
            return Ok(text("(no history)"));
        }
        let mut out = format!("history ({}):\n", revs.len());
        for r in &revs {
            let mark = if r.asserted { "+" } else { "-" };
            out.push_str(&format!("  [{mark}] t={:.0}  {}\n", r.seconds, r.name));
        }
        Ok(text(&out))
    })
}

/// Renders a full node snapshot: header (name, tier/type), layer +
/// visibility, tags, then the complete body. Prefixes an `[as of …]` line
/// when the read time-travelled.
fn render(s: &NodeSnapshot, when: Option<&str>) -> String {
    let mut out = String::new();
    if let Some(w) = when {
        let w = w.trim();
        if !w.is_empty() {
            out.push_str(&format!("[as of {w}]\n"));
        }
    }
    out.push_str(&format!("{} ({} / {})\n", s.name, s.tier, s.node_type));
    out.push_str(&format!(
        "layer: {}   visibility: {}\n",
        s.layer, s.visibility
    ));
    if let Some(ts) = s.ts {
        out.push_str(&format!("recorded: {}\n", fmt_ts(ts)));
    }
    if !s.tags.is_empty() {
        out.push_str(&format!("tags: {}\n", s.tags.join(", ")));
    }
    out.push('\n');
    out.push_str(s.body.as_deref().unwrap_or("(no body)"));
    out
}

fn now_seconds() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as f64)
        .unwrap_or(0.0)
}
