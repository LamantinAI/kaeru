//! Session-restoration & vault-meta tools: `awake`, `overview`,
//! `initiatives`, `recent`, `pin`, `unpin`, `config`.

use std::str::FromStr;

use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use kaeru_core::{Layer, Store};

use crate::utils::{
    brief_suffix, parse_duration_secs, resolve_name_or_id, text, to_mcp, with_initiative,
};

pub fn awake(store: &Store, initiative: Option<&str>) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let ctx = kaeru_core::awake(store).map_err(to_mcp)?;
        let mut out = String::new();
        out.push_str(&format!(
            "initiative: {}\n",
            ctx.initiative.as_deref().unwrap_or("(none)")
        ));
        out.push_str(&format!(
            "available initiatives ({}): {}\n\n",
            ctx.all_initiatives.len(),
            if ctx.all_initiatives.is_empty() {
                "(none)".to_string()
            } else {
                ctx.all_initiatives.join(", ")
            }
        ));

        // Layer-prioritised re-entry context: whole Core first, then Hot,
        // then Warm — load these into working context in this order.
        for bucket in &ctx.layered {
            out.push_str(&format!(
                "{} layer ({}):\n",
                bucket.layer.as_str(),
                bucket.nodes.len()
            ));
            for b in &bucket.nodes {
                out.push_str(&format!("  - {} ({}) — {}\n", b.name, b.node_type, b.id));
            }
        }
        out.push('\n');

        out.push_str(&format!("pinned ({}):\n", ctx.pinned.len()));
        for id in &ctx.pinned {
            out.push_str(&format!("  - {id}{}\n", brief_suffix(store, id)));
        }
        out.push('\n');
        out.push_str(&format!("recent ({}):\n", ctx.recent.len()));
        for id in &ctx.recent {
            out.push_str(&format!("  - {id}{}\n", brief_suffix(store, id)));
        }
        out.push('\n');
        out.push_str(&format!("under review ({}):\n", ctx.under_review.len()));
        for id in &ctx.under_review {
            out.push_str(&format!("  - {id}{}\n", brief_suffix(store, id)));
        }
        Ok(text(&out))
    })
}

pub fn overview(store: &Store, initiative: Option<&str>) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let report = kaeru_core::overview(store).map_err(to_mcp)?;
        Ok(text(&report))
    })
}

pub fn initiatives(store: &Store) -> Result<CallToolResult, McpError> {
    let names = kaeru_core::list_initiatives(store).map_err(to_mcp)?;
    if names.is_empty() {
        return Ok(text(
            "(no initiatives yet — pass `initiative` on a mutation to register one)",
        ));
    }
    let mut out = format!("initiatives ({}):\n", names.len());
    for n in &names {
        out.push_str(&format!("  - {n}\n"));
    }
    Ok(text(&out))
}

pub fn recent(
    store: &Store,
    since: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let window = parse_duration_secs(since).map_err(to_mcp)?;
        let ids = kaeru_core::recent_episodes(store, window).map_err(to_mcp)?;
        let mut out = format!("recent ({}):\n", ids.len());
        for id in &ids {
            out.push_str(&format!("  - {id}{}\n", brief_suffix(store, id)));
        }
        Ok(text(&out))
    })
}

pub fn pin(
    store: &Store,
    name_or_id: &str,
    reason: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name_or_id)?;
        kaeru_core::pin(store, &id, reason).map_err(to_mcp)?;
        Ok(text(&format!("pinned: {name_or_id} ({id})")))
    })
}

pub fn unpin(
    store: &Store,
    name_or_id: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let id = resolve_name_or_id(store, name_or_id)?;
        kaeru_core::unpin(store, &id).map_err(to_mcp)?;
        Ok(text(&format!("unpinned: {name_or_id} ({id})")))
    })
}

pub fn config(store: &Store) -> Result<CallToolResult, McpError> {
    let c = store.config();
    let out = format!(
        "kaeru {}\nvault_path           = {}\nactive_window_size   = {}\nrecent_episodes_cap  = {}\nawake_window_secs    = {}\nsummary_children_cap = {}\nbody_excerpt_chars   = {}\nprovenance_max_hops  = {}\ndefault_max_hops     = {}\nmax_hops_cap         = {}\n",
        kaeru_core::version(),
        c.vault_path.display(),
        c.active_window_size,
        c.recent_episodes_cap,
        c.awake_default_window_secs,
        c.summary_view_children_cap,
        c.body_excerpt_chars,
        c.provenance_max_hops,
        c.default_max_hops,
        c.max_hops_cap,
    );
    Ok(text(&out))
}

/// Explicit layered recall — surfaces nodes from the requested memory
/// layers, on demand. `awake` only loads Core/Hot/Warm; this is how you
/// reach `cold` / `frozen` (archived / not-surfaced-by-default) when you
/// know you need them. `layers` is a comma/space list (e.g. `cold,frozen`);
/// defaults to `cold,frozen`. Scoped to the active initiative when given.
pub fn surface(
    store: &Store,
    layers: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let spec = layers.unwrap_or("cold,frozen");
        let mut parsed: Vec<Layer> = Vec::new();
        for tok in spec.split([',', ' ']).map(str::trim).filter(|s| !s.is_empty()) {
            parsed.push(Layer::from_str(tok).map_err(to_mcp)?);
        }
        if parsed.is_empty() {
            parsed = vec![Layer::Cold, Layer::Frozen];
        }

        let buckets = kaeru_core::recall_by_layer(store, &parsed).map_err(to_mcp)?;
        let mut out = String::new();
        for bucket in &buckets {
            out.push_str(&format!(
                "{} layer ({}):\n",
                bucket.layer.as_str(),
                bucket.nodes.len()
            ));
            for b in &bucket.nodes {
                out.push_str(&format!("  - {} ({}) — {}\n", b.name, b.node_type, b.id));
            }
        }
        if out.is_empty() {
            out.push_str("(no nodes in the requested layers)");
        }
        Ok(text(&out))
    })
}

/// Static how-to-import guide. Returned verbatim so an agent about to
/// bulk-load knowledge (e.g. from a markdown export) does the right
/// thing without guessing: which verb matches which epistemic status,
/// how to stamp the memory layer at creation, and to link after writing.
pub fn import_guide() -> Result<CallToolResult, McpError> {
    let guide = r#"# kaeru import guide

Goal: load knowledge so a future agent recalls the right things first.

## 1. Scope every call
Pick/confirm the initiative first: `initiatives` -> `awake(initiative)` ->
`overview(initiative)`. Pass `initiative` on EVERY create/link call —
untagged writes are invisible to a scoped `overview`.

## 2. Choose the verb by epistemic status (not by length)
- `cite <name> --body ... [--url ...]`  -> settled facts, specs, decisions,
  references, persona/entity records. Lands in archival/reference.
- `episode <name> --body ...`           -> a named observation tied to work.
- `jot --body ...`                      -> a fleeting note (auto-named).
- `claim --text ... [--about X]`        -> a hypothesis under test
  (then `test` -> `confirm`/`refute`).
- `task --body ... [--due ...]` / `done`-> actionable todos.
- `synthesise` -> `settle`              -> promote converged operational
  seeds into one durable archival insight.

## 3. Stamp the layer AT creation (importance => recall priority)
Every create verb takes an optional `layer`. Injection order is
core -> hot -> warm -> cold -> frozen. Set it now; don't rely on a
later `layer` call.
- core   : foundational truth, always in context (architecture, current status,
           the one fact everything hinges on). Keep this set small.
- hot    : active work and recent changes; open blocking tasks; live hypotheses.
- warm   : default; useful reference, contacts, access points.
- cold   : passed stages, completed tasks, superseded notes, old probes.
- frozen : keep-for-the-record, do not surface.
A wrong-but-present layer beats a missing one; refine later if needed.

## 4. Link AFTER capturing
A node with no edges is easy to lose. After writing, `search` for related
nodes and `link from to --edge_type ...`
(`refers_to` default; also `derived_from`, `supersedes`, `causal`,
`part_of`, `blocks`, `contradicts`).

## 5. Bulk import from a markdown export
For each page: recreate it with the verb matching its tier/type and a
layer from its importance; then recreate the `## Outgoing` / `## Incoming`
edges with `link`. Don't import mechanically — drop stale operational
noise, keep settled knowledge and active work.
"#;
    Ok(text(guide))
}
