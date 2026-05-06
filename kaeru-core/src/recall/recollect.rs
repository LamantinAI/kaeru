//! Archival-tier recollection — `recollect_idea`, `recollect_outcome`,
//! and `recollect_provenance`. Mirrors of operational-side recall on the
//! cortex side.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;

use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

use super::NodeBrief;
use super::parse_brief;

/// Returns archival-tier idea nodes valid at NOW as briefs, ordered
/// newest-first by validity. Mirror of `recent_episodes` on the
/// recollection (cortex) side — the agent's stable long-term ideas.
pub fn recollect_idea(store: &Store) -> Result<Vec<NodeBrief>> {
    recollect_briefs_by_archival_type(store, "idea")
}

/// Returns archival-tier outcome nodes valid at NOW as briefs, ordered
/// newest-first by validity. Outcomes are settled results — what the
/// agent / user has decided "this is what we found".
pub fn recollect_outcome(store: &Store) -> Result<Vec<NodeBrief>> {
    recollect_briefs_by_archival_type(store, "outcome")
}

/// Walks `derived_from` edges from `node_id` back through its ancestors —
/// the synthesise / consolidate provenance chain. Returns ancestor briefs
/// (the seed itself is excluded).
///
/// Useful for the curator-API question "where did this come from?": an
/// Outcome's provenance is the Idea(s) it was synthesised from; an Idea's
/// provenance is the Episodes it was synthesised from.
pub fn recollect_provenance(store: &Store, node_id: &NodeId) -> Result<Vec<NodeBrief>> {
    let max_hops = store.config().provenance_max_hops;
    let excerpt_chars = store.config().body_excerpt_chars;
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("seed".to_string(), DataValue::Str(node_id.clone().into()));

    // Recursive Datalog: hop 0 is the seed; each recursive step extends
    // one `derived_from` edge in the source direction (src → dst). The
    // final projection drops `h = 0` so the seed itself does not appear
    // as its own ancestor. When an initiative is active, the projection
    // also restricts ancestors to that initiative — provenance does not
    // leak across initiatives.
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            format!(
                r#"
                ancestor[id, hops] := id = $seed, hops = 0
                ancestor[id, hops] := ancestor[prev, h],
                                      h < {max_hops},
                                      hops = h + 1,
                                      *edge{{src: prev, dst: id, edge_type @ 'NOW'}},
                                      edge_type = 'derived_from'

                ?[id, type, name, body] := ancestor[id, h], h > 0,
                                            *node{{id, type, name, body @ 'NOW'}},
                                            *node_initiative{{initiative, node_id: id}},
                                            initiative = $init
                "#
            )
        }
        None => format!(
            r#"
            ancestor[id, hops] := id = $seed, hops = 0
            ancestor[id, hops] := ancestor[prev, h],
                                  h < {max_hops},
                                  hops = h + 1,
                                  *edge{{src: prev, dst: id, edge_type @ 'NOW'}},
                                  edge_type = 'derived_from'

            ?[id, type, name, body] := ancestor[id, h], h > 0,
                                        *node{{id, type, name, body @ 'NOW'}}
            "#
        ),
    };
    let rows = store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Immutable)?;

    let briefs: Vec<NodeBrief> = rows
        .rows
        .iter()
        .map(|r| parse_brief(r.as_slice(), excerpt_chars))
        .collect();
    Ok(briefs)
}

fn recollect_briefs_by_archival_type(store: &Store, node_type: &str) -> Result<Vec<NodeBrief>> {
    let excerpt_chars = store.config().body_excerpt_chars;
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("nt".to_string(), DataValue::Str(node_type.into()));

    // `validity` is bound for ordering only; `parse_brief` reads columns
    // 0..=3 and ignores the trailing validity column. When an initiative
    // is active, the read joins `node_initiative` so only nodes
    // attached to that initiative surface.
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
                ?[id, type, name, body, validity] :=
                    *node{id, type, name, body, validity, tier @ 'NOW'},
                    type = $nt,
                    tier = 'archival',
                    *node_initiative{initiative, node_id: id},
                    initiative = $init
                :order validity
            "#
        }
        None => {
            r#"
                ?[id, type, name, body, validity] :=
                    *node{id, type, name, body, validity, tier @ 'NOW'},
                    type = $nt,
                    tier = 'archival'
                :order validity
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let briefs: Vec<NodeBrief> = rows
        .rows
        .iter()
        .map(|r| parse_brief(r.as_slice(), excerpt_chars))
        .collect();
    Ok(briefs)
}
