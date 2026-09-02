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

use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, NaiveDate, Utc};
use kaeru_core::{
    EdgeType, Error, Layer, Neighbour, NodeBrief, NodeId, Store, SummaryView, Tier,
    count_nodes_in_initiative, edges_of, history,
};
use rmcp::ErrorData as McpError;
use rmcp::model::{CallToolResult, Content};

// =========================================================================
// Output builders
// =========================================================================

pub fn text(s: &str) -> CallToolResult {
    CallToolResult::success(vec![Content::text(s)])
}

/// One-line nudge appended to a capture result *when the node lands as an
/// island* (see [`capture_result`]). It reminds the agent, at the moment of
/// capture, to `link` the node and (when a line of work runs start→decision)
/// save the reasoning trail with `chain`. A hint in the result, not a gate:
/// the substrate is a facilitator, not an enforcer.
pub const CAPTURE_NUDGE: &str = "\n↳ now connect it: `search` related → `link` \
     (strong=true for key edges); when a line of work runs start→decision/outcome, \
     `chain(from, to)` to save the reasoning trail. Don't leave it an island.";

/// Capture result for the knowledge-forming verbs (`episode`, `cite`,
/// `claim`). Appends [`CAPTURE_NUDGE`] only when the new node is actually an
/// island worth connecting: it has no edges yet **and** its initiative already
/// holds other nodes, so there is something to link to. A node that already
/// gained an edge at capture (e.g. `claim` with a target), or the first node
/// in a fresh initiative, gets no nudge — the hint lands only where it teaches.
/// `jot` / `task` never call this: they're operational notes, not graph
/// anchors. Best-effort — any read error simply suppresses the nudge.
pub fn capture_result(
    store: &Store,
    id: &NodeId,
    initiative: Option<&str>,
    msg: &str,
) -> CallToolResult {
    if should_nudge(store, id, initiative) {
        text(&format!("{msg}{CAPTURE_NUDGE}"))
    } else {
        text(msg)
    }
}

/// True when `id` is isolated at NOW and its initiative already holds at
/// least one other node. Needs an explicit `initiative`: the capture's scope
/// is atomic and already gone by the time the result is built, so a capture
/// with no initiative is never nudged.
fn should_nudge(store: &Store, id: &NodeId, initiative: Option<&str>) -> bool {
    let Some(init) = initiative else {
        return false;
    };
    let isolated = edges_of(store, id).map(|e| e.is_empty()).unwrap_or(false);
    isolated
        && count_nodes_in_initiative(store, init)
            .map(|n| n > 1)
            .unwrap_or(false)
}

// ---- Deepen-lane hints -------------------------------------------------
//
// The read verbs (`recall` / `drill` / `search`) surface only an id or a
// truncated excerpt, and agents routinely forget the two verbs that go
// deeper: `at` (whole body, and time-travel) and `history` (the timeline).
// These build the forward edges — one-line pointers appended to a result,
// only when they teach (a body was actually cut, a node actually changed).
// Hints, never gates: the substrate facilitates, it does not enforce.

/// The `…` that `truncate_excerpt` appends is the tell that a body was cut,
/// so `at` would reveal more than the excerpt shown here.
pub fn body_truncated(excerpt: Option<&str>) -> bool {
    excerpt.is_some_and(|e| e.ends_with('…'))
}

/// True when the node carries more than its birth assertion — a real
/// revision/retraction history worth pointing `history` at. One indexed read;
/// only called from single-node verbs. Best-effort: any error suppresses the
/// hint rather than failing the caller.
pub fn was_revised(store: &Store, id: &NodeId) -> bool {
    history(store, id).map(|h| h.len() > 1).unwrap_or(false)
}

/// `↳ full text: at <name>` — the forward edge a truncating single-node read
/// owes to `at`. Names the node so the hint is a ready-to-run call.
pub fn at_fulltext_hint(name: &str) -> String {
    format!("\n↳ excerpt only — full text: `at {name}`.")
}

/// Generic form for multi-result reads (`search`), where no single name fits.
pub const AT_FULLTEXT_HINT_MANY: &str = "\n↳ excerpts only — read one in full with `at <name>`.";

/// `recall` returns just an opaque id; this reminds the agent it can now read
/// the node — in full via `at`, or with its drill-down children via `drill`.
pub fn recall_read_hint(name: &str) -> String {
    format!("\n↳ that's the id — read it: `at {name}` (full) or `drill {name}` (＋children).")
}

/// Single-node reads point here once the node has actually changed.
pub fn history_hint(name: &str) -> String {
    format!("\n↳ this node has changed — timeline: `history {name}`.")
}

/// `history` lists timestamps; this closes the loop back to `at`'s time-travel
/// so the agent reads a specific past version instead of stopping at the list.
pub fn history_read_version_hint(name: &str) -> String {
    format!("\n↳ read any version in full: `at {name} when=<t>` (`5m`/`2h` ago, or unix secs).")
}

// ---- Read-side nudges --------------------------------------------------
//
// The nudge economy used to run one way: every capture verb pushed the agent
// to write more (`CAPTURE_NUDGE`), and nothing ever pushed it to read. A hint
// in a tool's output is the strongest discovery channel this product has —
// it made `link` the most-used verb in the whole graph — so the read verbs
// that had no such channel simply stayed unused. These are the return edges:
// a pointer in the output of a verb the agent already calls, naming the read
// verb that goes further. Hints, never gates.

/// Footer for a `search` hit list: the two ways to go deeper on a result.
/// Names the top hit so both are ready-to-run calls.
pub fn search_deepen_hint(top: &str) -> String {
    format!(
        "\n↳ go deeper on a hit: `drill {top}` (its neighbours) · `trace {top}` (where it came from)."
    )
}

/// Footer for a `search` that matched nothing. A miss is the moment the agent
/// is most likely to conclude "memory is empty" and stop — so it gets the
/// three widenings, cheapest first, instead of a bare `(no matches)`.
pub fn search_empty_hint(query: &str) -> String {
    format!(
        "\n↳ widen it: `search \"{query}*\"` (prefix match) · `tagged topic:<theme>` \
         (browse by tag) · `recent 7d` (what's fresh)."
    )
}

/// `↳ part of a saved trail: …` — the return edge from a single-node read to
/// `why`. A chain is authored for a future session and then read by nobody;
/// this is how the node itself says it belongs to one. Empty when the node is
/// in no chain, so the hint only ever appears where it teaches. Best-effort:
/// a read error suppresses it rather than failing the caller.
pub fn chain_membership_hint(store: &Store, id: &NodeId) -> String {
    let Ok(memberships) = kaeru_core::chain_membership(store, id) else {
        return String::new();
    };
    let steps: Vec<String> = memberships
        .iter()
        .map(|m| format!("`{}` (step {}/{})", m.chain_name, m.position, m.length))
        .collect();
    match steps.len() {
        0 => String::new(),
        1 => format!(
            "\n↳ part of a saved trail: {} — read it: `why {}`.",
            steps[0], memberships[0].chain_name
        ),
        n => format!(
            "\n↳ part of {n} saved trails: {} — read one: `why <name>`.",
            steps.join(", ")
        ),
    }
}

/// `↳ …` for a demotion into an archived layer. `cold` / `frozen` are exactly
/// the layers `awake` does not load, so a node moved there leaves the
/// re-entry view — the agent should know the one verb that reaches it again.
pub fn archived_layer_hint(layer: Layer) -> &'static str {
    match layer {
        Layer::Cold | Layer::Frozen => {
            "\n↳ out of the re-entry view now — `surface layers=cold,frozen` reads it back."
        }
        _ => "",
    }
}

/// `↳ when the verdict lands: …` — a claim is a promise to settle it later,
/// and the settling verbs live nowhere else in the agent's path. Also names
/// how to list the claims still waiting.
pub fn claim_verdict_hint(name: &str) -> String {
    format!(
        "\n↳ when the verdict lands: `confirm {name} --by <evidence>` or `refute {name} --by <evidence>`; \
         still-open ones show in `awake`, or `tagged \"status:open\"`."
    )
}

/// `↳ that closes an arc — …` for the moment a piece of work actually ends.
///
/// kaeru has an entry ritual and no exit one. A session stops rather than
/// finishes, so "this is done, let me tidy up" never arrives: in the audit,
/// ~97% of all hygiene traffic came from a handful of threads where a user
/// asked for a cleanup in so many words, and not one spontaneous consolidation
/// was found anywhere. A five-episode arc ending in an episode that literally
/// reported production verification left all five nodes operational forever.
///
/// The instruction "move things to archival when they stop changing" points at
/// something unobservable — "stopped changing" is only visible *between*
/// sessions. But the terminal verbs are observable, and they are the moment:
/// `done`, `close_review`, a verdict. This asks the question that belongs
/// there — what did the work conclude? — and names the two verbs that answer
/// it.
///
/// Fires only past two operational neighbours: one is a detail, several are an
/// arc. Best-effort; a read error suppresses it rather than failing a verb
/// that has already done its work.
pub fn arc_closed_hint(store: &Store, id: &NodeId) -> String {
    const MIN_ARC: usize = 2;
    const NAMED: usize = 3;

    let Ok(near) = kaeru_core::operational_neighbours(store, id) else {
        return String::new();
    };
    if near.len() < MIN_ARC {
        return String::new();
    }
    let names: Vec<String> = near
        .iter()
        .take(NAMED)
        .map(|b| format!("`{}`", b.name))
        .collect();
    let more = near.len().saturating_sub(NAMED);
    let tail = if more > 0 {
        format!(", +{more} more")
    } else {
        String::new()
    };
    format!(
        "\n↳ that closes an arc — {} operational nodes still converge here ({}{tail}). \
         What did the work conclude? `settle <name>` promotes one in place; \
         `synthesise from=a,b,c` converges several into one outcome.",
        near.len(),
        names.join(", ")
    )
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

/// Formats a Unix-seconds timestamp as `2026-06-23 11:21 · 2d ago` — an
/// absolute UTC stamp for unambiguous ordering plus a relative "ago" for
/// quick freshness. Gives every brief / snapshot a chronological anchor so
/// the agent can follow the order of work and thought.
pub fn fmt_ts(secs: f64) -> String {
    let abs = DateTime::<Utc>::from_timestamp(secs as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| "?".to_string());
    format!("{abs} · {}", humanize_ago(secs))
}

/// How long ago `secs` (Unix seconds) was, relative to now: `just now`,
/// `5m ago`, `3h ago`, `2d ago`, `4w ago`. Future stamps (clock skew) read
/// as `just now`.
fn humanize_ago(secs: f64) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(secs);
    let delta = now - secs;
    if delta < 60.0 {
        "just now".to_string()
    } else if delta < 3_600.0 {
        format!("{}m ago", (delta / 60.0) as i64)
    } else if delta < 86_400.0 {
        format!("{}h ago", (delta / 3_600.0) as i64)
    } else if delta < 604_800.0 {
        format!("{}d ago", (delta / 86_400.0) as i64)
    } else {
        format!("{}w ago", (delta / 604_800.0) as i64)
    }
}

/// ` · <abs · rel>` suffix for a brief's timestamp, or `""` when the node
/// carried no parseable validity.
pub fn ts_suffix(ts: Option<f64>) -> String {
    ts.map(|s| format!(" · {}", fmt_ts(s))).unwrap_or_default()
}

pub fn render_summary(view: &SummaryView) -> String {
    let mut out = format!(
        "{} ({}) — {}{}\n",
        view.root.name,
        view.root.node_type,
        view.root.id,
        ts_suffix(view.root.ts)
    );
    if let Some(e) = &view.root.body_excerpt {
        out.push_str(&format!("  {e}\n"));
    }
    if view.children.is_empty() {
        out.push_str("(no drill-down children)\n");
    } else {
        out.push_str(&format!("children ({}):\n", view.children.len()));
        for c in &view.children {
            out.push_str(&format!(
                "  - {} ({}) — {}{}\n",
                c.name,
                c.node_type,
                c.id,
                ts_suffix(c.ts)
            ));
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
        out.push_str(&format!(
            "  - {} ({}) — {}{}\n",
            b.name,
            b.node_type,
            b.id,
            ts_suffix(b.ts)
        ));
        if let Some(e) = &b.body_excerpt {
            out.push_str(&format!("    {e}\n"));
        }
    }
    out
}

/// Renders a node's neighbours: each on its own line with the edge TYPE and
/// direction (`—[type]→` outgoing, `←[type]—` incoming), then the node's brief
/// and excerpt. The direction arrows mirror `between`, so the two verbs read
/// alike — a typed graph should never render as an untyped list.
pub fn render_neighbours(seed: &str, ns: &[Neighbour]) -> String {
    if ns.is_empty() {
        return format!("neighbours of {seed} (0): (none)");
    }
    let mut out = format!("neighbours of {seed} ({}):\n", ns.len());
    for n in ns {
        let arrow = if n.outgoing {
            format!("—[{}]→", n.edge_type)
        } else {
            format!("←[{}]—", n.edge_type)
        };
        out.push_str(&format!(
            "  {arrow} {} ({}) — {}{}\n",
            n.brief.name,
            n.brief.node_type,
            n.brief.id,
            ts_suffix(n.brief.ts)
        ));
        if let Some(e) = &n.brief.body_excerpt {
            out.push_str(&format!("    {e}\n"));
        }
    }
    out
}

/// Parses an optional comma-separated edge-type filter into `EdgeType`s. `None`
/// or an all-blank spec yields an empty vec, which `neighbours` reads as "all
/// types". An unknown type errors with the closed vocabulary, exactly as `link`
/// does — the agent gets one message, not a silent empty result.
pub fn parse_edge_types(spec: Option<&str>) -> Result<Vec<EdgeType>, McpError> {
    let Some(spec) = spec else {
        return Ok(Vec::new());
    };
    spec.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| EdgeType::from_str(s).map_err(to_mcp))
        .collect()
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
    // Scope + run go through `Store::scoped`, which serializes scope sessions
    // on the shared `Store`. The daemon hands the same `Arc<Store>` to every
    // concurrent MCP session, so a bare set-scope-then-run would let two
    // sessions interleave each other's initiative across threads (the same
    // race fixed in kaeru-rig). The guard makes each session's scope atomic;
    // no explicit restore is needed — no other scoped caller observes the
    // intermediate state.
    store.scoped(initiative, |_store| f())
}

pub fn resolve_name(store: &Store, name: &str) -> Result<NodeId, McpError> {
    match kaeru_core::recall_id_by_name(store, name).map_err(to_mcp)? {
        Some(id) => Ok(id),
        None => Err(name_not_found(store, name)),
    }
}

/// The not-found an agent can act on.
///
/// One message for every miss taught nothing: `no node named "x" at NOW` is
/// the same answer whether the node is in the next initiative over, spelled
/// differently, or genuinely absent — three situations with three different
/// next moves. The audit caught one misremembered name being re-tried across
/// three separate sessions, a false memory that nothing ever corrected.
///
/// The cases are checked cheapest-first, and each names its own way out:
///
/// 1. **Elsewhere.** The name resolves outside the active scope — the agent
///    had it right all along and only the scope was wrong.
/// 2. **Nearly.** Close names exist here; say which.
/// 3. **Nowhere.** Nothing to offer, so point at the verb that searches text
///    rather than names, and admit it may simply not be captured yet.
/// The four-branch "not found" message: the same name in another initiative, a
/// node that belongs to no initiative (#81), a near spelling, or nowhere at all.
/// Returned as a plain string so both the error path (`name_not_found`, used by
/// `resolve_name`) and the text path (`recall`'s miss) carry the same recovery
/// guidance — a bare "(not found)" was the single biggest source of recall
/// retry-loops in the usage audit (#84).
pub(crate) fn name_not_found_message(store: &Store, name: &str) -> String {
    // 1 — the same name, under another initiative.
    if let Ok(Some(id)) = kaeru_core::recall_id_by_name_global(store, name) {
        let elsewhere = kaeru_core::initiatives_of_node(store, &id).unwrap_or_default();
        if let Some(first) = elsewhere.first() {
            let named = elsewhere
                .iter()
                .map(|i| format!("`{i}`"))
                .collect::<Vec<_>>()
                .join(", ");
            return format!(
                "no node named {name:?} in this initiative — it lives in {named}. \
                 Pass `initiative={first}` to work there, or `attach {name} <this initiative>` \
                 to file it here as well."
            );
        }
        // 1b — exists, but attached to NO initiative (#81). Every real read is
        // scoped, so a bare "not found" is a lie: the node is there, it just
        // loads in no session (a `core` node here is silently dead). Name the
        // fix rather than sending the agent looking for something it wrote.
        return format!(
            "{name:?} exists but belongs to no initiative, so no scoped read or session reaches \
             it — `attach {name} <initiative>` to file it, then it loads."
        );
    }

    // 2 — something close, here.
    let near = kaeru_core::suggest_node_name(store, name).unwrap_or_default();
    if !near.is_empty() {
        let listed = near
            .iter()
            .map(|n| format!("`{n}`"))
            .collect::<Vec<_>>()
            .join(", ");
        return format!("no node named {name:?} at NOW — did you mean {listed}?");
    }

    // 3 — nowhere.
    format!(
        "no node named {name:?} anywhere at NOW — `search {name}*` looks by text instead of by \
         name, and `initiatives` shows what else exists. If it was never captured, capture it."
    )
}

fn name_not_found(store: &Store, name: &str) -> McpError {
    to_mcp(Error::NotFound(name_not_found_message(store, name)))
}

/// Whether `input` has the shape of a UUID, letting a resolver skip the name
/// lookup when the caller already holds an id.
///
/// Checks the whole shape — `8-4-4-4-12` hex groups — rather than a length and
/// one hyphen. The cheap version compared `len()` (**bytes**) against
/// `chars().nth(8)` (**characters**), which agree only for ASCII. A name in any
/// two-byte script could be 36 bytes while being far shorter in characters, and
/// kaeru's own slug convention puts a hyphen at character 8 often enough that
/// the two conditions met by accident: the name was then passed into the query
/// verbatim as an id, matched nothing, and the caller was told the node does
/// not exist (#74).
///
/// That is worse than a failed read. `recall` and `search` resolve such a name
/// correctly while `at`, `drill` and `history` deny it exists, so the node
/// looks like a ghost left behind by a broken retraction — and the reasonable
/// response to a ghost is to `forget` or `unlink` something perfectly healthy.
///
/// Strictness is free here: ids come from tool output and names come from
/// people, so nothing legitimate is on the boundary.
pub fn looks_like_id(input: &str) -> bool {
    let mut groups = input.split('-');
    let lengths = [8, 4, 4, 4, 12];
    for len in lengths {
        match groups.next() {
            Some(g) if g.len() == len && g.bytes().all(|b| b.is_ascii_hexdigit()) => {}
            _ => return false,
        }
    }
    groups.next().is_none()
}

/// Resolves a name at NOW, or passes a raw id straight through.
pub fn resolve_name_or_id(store: &Store, input: &str) -> Result<NodeId, McpError> {
    if looks_like_id(input) {
        return Ok(input.to_string());
    }
    resolve_name(store, input)
}

/// Builds a one-line note when any capture field carried leaked tool-call
/// wire-format that the core write path scrubbed. `fields` pairs a label with
/// the raw input; the note names only the fields that were actually dirty, so
/// a clean call gets `None` and no noise. The stripping itself happens in
/// `kaeru-core`; this only mirrors the detection to tell the caller.
pub fn markup_strip_note(fields: &[(&str, &str)]) -> Option<String> {
    let dirty: Vec<&str> = fields
        .iter()
        .filter(|(_, raw)| kaeru_core::strip_tool_call_markup(raw).1)
        .map(|(label, _)| *label)
        .collect();
    if dirty.is_empty() {
        return None;
    }
    Some(format!(
        "\n(note: stripped leaked tool-call markup from {})",
        dirty.join(" and ")
    ))
}

/// Resolves an edge endpoint for `link` / `unlink` / `reweight`: accepts a raw
/// id, else resolves the name in the active initiative first and falls back to
/// a cross-initiative lookup. An edge often joins nodes living under different
/// initiatives — the scoped resolvers used by the read verbs would miss an
/// endpoint outside the active scope, which is exactly the friction that made
/// a cross-initiative `link` fail until the scope was dropped. Mirrors the
/// global resolution `attach` already relies on. On a name collision the newest
/// node wins, and a scoped match is preferred over the global fallback.
pub fn resolve_link_endpoint(store: &Store, input: &str) -> Result<NodeId, McpError> {
    if looks_like_id(input) {
        return Ok(input.to_string());
    }
    if let Some(id) = kaeru_core::recall_id_by_name(store, input).map_err(to_mcp)? {
        return Ok(id);
    }
    kaeru_core::recall_id_by_name_global(store, input)
        .map_err(to_mcp)?
        .ok_or_else(|| {
            to_mcp(Error::NotFound(format!(
                "no node named {input:?} at NOW (searched the active initiative, then all initiatives)"
            )))
        })
}

/// Like [`resolve_name_or_id`] but resolves a **name** as of `at_seconds`
/// rather than NOW, so a time-travel read (`at`) can target a node that has
/// since been retracted. A raw id passes through untouched (the historical read
/// happens in `kaeru_core::at`).
pub fn resolve_name_or_id_at(
    store: &Store,
    input: &str,
    at_seconds: f64,
) -> Result<NodeId, McpError> {
    if looks_like_id(input) {
        return Ok(input.to_string());
    }
    if let Some(id) = kaeru_core::recall_id_by_name_at(store, input, at_seconds).map_err(to_mcp)? {
        return Ok(id);
    }
    // A time-travel read that misses can miss in two different ways, and the
    // fix differs: the name may be wrong, or the moment may be. Saying which
    // costs one extra lookup on a path that has already failed.
    if kaeru_core::recall_id_by_name(store, input)
        .map_err(to_mcp)?
        .is_some()
    {
        return Err(to_mcp(Error::NotFound(format!(
            "`{input}` exists now but not at that moment — it was written later, \
             or carried a different name then. `history {input}` shows when it changed."
        ))));
    }
    Err(name_not_found(store, input))
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

/// Parses an optional layer string into a [`Layer`], defaulting to
/// `Layer::default()` (warm) when absent or blank. Lets capture verbs
/// stamp a memory layer at creation so a node is never born layer-less.
pub fn parse_layer(s: Option<&str>) -> Result<Layer, McpError> {
    match s {
        Some(v) if !v.trim().is_empty() => Layer::from_str(v.trim()).map_err(to_mcp),
        _ => Ok(Layer::default()),
    }
}

/// Parses an optional visibility string into "wants shared?". `shared` →
/// true; absent / `local` → false; anything else is an error. Used by the
/// capture verbs to decide whether to push a freshly-created node to the
/// cloud in the same call.
pub fn parse_wants_shared(s: Option<&str>) -> Result<bool, McpError> {
    match s {
        None => Ok(false),
        Some(v) => match v.trim().to_lowercase().as_str() {
            "" | "local" => Ok(false),
            "shared" => Ok(true),
            other => Err(to_mcp(Error::Invalid(format!(
                "visibility must be `local` or `shared`, got {other:?}"
            )))),
        },
    }
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
#[cfg(test)]
mod id_shape_tests {
    use super::looks_like_id;

    /// Real ids still pass — this is the only thing the check exists for.
    #[test]
    fn a_uuid_is_recognised() {
        assert!(looks_like_id("01a03a4f-4107-7ab2-a599-1f9f92ce7393"));
        assert!(looks_like_id("00000000-0000-0000-0000-000000000000"));
        assert!(
            looks_like_id("01A03A4F-4107-7AB2-A599-1F9F92CE7393"),
            "hex is hex in either case"
        );
    }

    /// The bug, exactly as reported: 36 UTF-8 BYTES, far fewer characters, and
    /// a hyphen at character 8 because that is what kaeru's slugs look like.
    /// The old check read this as an id and the node became unreadable by name.
    #[test]
    fn a_36_byte_non_ascii_name_is_not_an_id() {
        let name = "änderung-prüfung-fehleranalyse2026";
        assert_eq!(
            name.len(),
            36,
            "36 bytes, which is what fooled the old check"
        );
        assert!(name.chars().count() < 36, "but fewer characters");
        assert!(!looks_like_id(name), "and it is a name, not an id");
    }

    /// Names that merely resemble the shape are still names.
    #[test]
    fn near_misses_are_names() {
        for name in [
            "not-a-uuid",
            "01a03a4f-4107-7ab2-a599",                // too few groups
            "01a03a4f-4107-7ab2-a599-1f9f92ce7393-x", // too many
            "01a03a4f-4107-7ab2-a599-1f9f92ce739",    // last group short
            "zzzzzzzz-4107-7ab2-a599-1f9f92ce7393",   // not hex
            "01a03a4f_4107_7ab2_a599_1f9f92ce7393",   // underscores
            "",
        ] {
            assert!(!looks_like_id(name), "{name:?} is a name");
        }
    }

    /// An ASCII slug of exactly 36 characters with a hyphen at index 8 — the
    /// shape the old check accepted — is a perfectly ordinary name.
    #[test]
    fn an_ascii_slug_of_the_old_shape_is_a_name() {
        let name = "abcdefgh-ijklmnop-qrstuvwx-yzabcdef0";
        assert_eq!(name.len(), 36);
        assert_eq!(name.chars().nth(8), Some('-'));
        assert!(!looks_like_id(name));
    }
}
