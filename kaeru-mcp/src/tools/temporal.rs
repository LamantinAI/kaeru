//! Bi-temporal handle: `at`, `history`.

use std::time::{SystemTime, UNIX_EPOCH};

use kaeru_core::{NodeSnapshot, Store};
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{
    fmt_ts, history_hint, history_read_version_hint, parse_when, resolve_name,
    resolve_name_or_id_at, text, to_mcp, was_revised, with_initiative,
};

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
        // Resolve the target AT the requested moment, not NOW — otherwise a
        // node retracted since `when` fails resolution before the historical
        // read is ever attempted. A raw id passes straight through.
        let secs = match when {
            Some(w) if !w.trim().is_empty() => parse_when(w).map_err(to_mcp)?,
            _ => now_seconds(),
        };
        let id = resolve_name_or_id_at(store, name, secs)?;
        match kaeru_core::at(store, &id, secs).map_err(to_mcp)? {
            Some(snap) => {
                let mut out = render(&snap, when);
                // Full text is in hand; if the node changed, point at the timeline.
                if was_revised(store, &id) {
                    out.push_str(&history_hint(&snap.name));
                }
                Ok(text(&out))
            }
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
        // More than one version means there's a past worth time-travelling to.
        if revs.len() > 1 {
            out.push_str(&history_read_version_hint(name));
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

#[cfg(test)]
mod tests {
    use kaeru_core::{EpisodeKind, Significance, Store};
    use rmcp::model::CallToolResult;

    use super::{at, history};

    fn store_t() -> Store {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        store
    }

    fn text_of(r: CallToolResult) -> String {
        r.content
            .iter()
            .filter_map(|c| c.as_text().map(|t| t.text.clone()))
            .collect::<Vec<_>>()
            .join("")
    }

    /// A node with two versions: `at` points forward to the timeline, and
    /// `history` closes the loop back to `at`'s time-travel.
    #[test]
    fn revised_node_wires_at_and_history_together() {
        let store = store_t();
        let id = kaeru_core::write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "d1",
            "one",
        )
        .expect("write");
        std::thread::sleep(std::time::Duration::from_millis(1100)); // cross validity second
        kaeru_core::improve(&store, &id, "d2", "two").expect("improve");

        let at_out = text_of(at(&store, "d2", None, Some("t")).unwrap());
        assert!(
            at_out.contains("timeline: `history d2`"),
            "at on a revised node points at history:\n{at_out}"
        );

        let hist_out = text_of(history(&store, "d2", Some("t")).unwrap());
        assert!(
            hist_out.contains("read any version in full: `at d2 when="),
            "history points back to at's time-travel:\n{hist_out}"
        );
    }

    /// A never-revised node: `at` stays quiet (single version, nothing to
    /// time-travel to).
    #[test]
    fn unrevised_node_gets_no_history_hint_from_at() {
        let store = store_t();
        kaeru_core::write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "solo",
            "x",
        )
        .expect("write");
        let out = text_of(at(&store, "solo", None, Some("t")).unwrap());
        assert!(
            !out.contains("timeline: `history"),
            "no history-hint for one version:\n{out}"
        );
    }
}
