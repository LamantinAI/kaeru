//! `summary_view` — PageIndex-style hierarchical navigation: seed brief
//! plus 1-hop drill-down children via `derived_from` (outgoing) and
//! `part_of` (incoming).

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;

use crate::errors::Error;
use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

use super::NodeBrief;
use super::parse_brief;

/// Hierarchical summary handle returned by [`summary_view`]. The agent
/// reads `root`, scans `children`, and decides which child to recurse
/// into by calling `summary_view` again with that child's id —
/// PageIndex-style reasoning-based navigation.
#[derive(Debug, Clone)]
pub struct SummaryView {
    pub root: NodeBrief,
    pub children: Vec<NodeBrief>,
}

/// Returns a one-level summary view of `seed`: the seed's own brief plus
/// up to `config().summary_view_children_cap` one-hop "drill-down"
/// neighbours.
///
/// Drill-down direction:
///   - outgoing `derived_from` — sources the seed was built from
///     (synthesise / consolidate provenance);
///   - incoming `part_of` — children that compose the seed
///     (checklist / roadmap structure).
///
/// Ordering inside `children` is arbitrary (Datalog set semantics); a
/// caller that wants stable ordering should sort by `name` or `id`.
pub fn summary_view(store: &Store, seed: &NodeId) -> Result<SummaryView> {
    let initiative = store.current_initiative();

    // Step 1 — root brief. Errors if the seed isn't asserted at NOW or
    // (when an initiative is active) isn't attached to that initiative.
    let mut p_root: BTreeMap<String, DataValue> = BTreeMap::new();
    p_root.insert("seed".to_string(), DataValue::Str(seed.clone().into()));
    let root_script = match &initiative {
        Some(init) => {
            p_root.insert("init".to_string(), DataValue::Str(init.clone().into()));
            r#"
                ?[id, type, name, body] := *node{id, type, name, body @ 'NOW'},
                                            id = $seed,
                                            *node_initiative{initiative, node_id: id},
                                            initiative = $init
            "#
        }
        None => {
            r#"
                ?[id, type, name, body] := *node{id, type, name, body @ 'NOW'}, id = $seed
            "#
        }
    };
    let root_rows = store
        .db_ref()
        .run_script(root_script, p_root, ScriptMutability::Immutable)?;
    let root_row = root_rows
        .rows
        .first()
        .ok_or_else(|| Error::NotFound(format!("node {seed} not found at NOW")))?;
    let excerpt_chars = store.config().body_excerpt_chars;
    let root = parse_brief(root_row.as_slice(), excerpt_chars);

    // Step 2 — one-hop drill-down children. Two Datalog rules unioned
    // into `candidates`, then joined to *node for the brief fields.
    // Edge and node both anchored at NOW so retracted rows are skipped.
    // When an initiative is active, a third clause filters children to
    // that initiative through `node_initiative`.
    let mut p_children: BTreeMap<String, DataValue> = BTreeMap::new();
    p_children.insert("seed".to_string(), DataValue::Str(seed.clone().into()));
    let children_script = match &initiative {
        Some(init) => {
            p_children.insert("init".to_string(), DataValue::Str(init.clone().into()));
            r#"
                candidates[child] := *edge{src, dst: child, edge_type @ 'NOW'},
                                     src = $seed,
                                     edge_type = 'derived_from'
                candidates[child] := *edge{src: child, dst, edge_type @ 'NOW'},
                                     dst = $seed,
                                     edge_type = 'part_of'
                ?[id, type, name, body] := candidates[id],
                                            *node{id, type, name, body @ 'NOW'},
                                            *node_initiative{initiative, node_id: id},
                                            initiative = $init
            "#
        }
        None => {
            r#"
                candidates[child] := *edge{src, dst: child, edge_type @ 'NOW'},
                                     src = $seed,
                                     edge_type = 'derived_from'
                candidates[child] := *edge{src: child, dst, edge_type @ 'NOW'},
                                     dst = $seed,
                                     edge_type = 'part_of'
                ?[id, type, name, body] := candidates[id], *node{id, type, name, body @ 'NOW'}
            "#
        }
    };
    let child_rows = store
        .db_ref()
        .run_script(children_script, p_children, ScriptMutability::Immutable)?;

    let children_cap = store.config().summary_view_children_cap;
    let children: Vec<NodeBrief> = child_rows
        .rows
        .iter()
        .take(children_cap)
        .map(|row| parse_brief(row.as_slice(), excerpt_chars))
        .collect();

    Ok(SummaryView { root, children })
}
