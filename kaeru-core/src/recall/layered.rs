//! Layer-prioritised recall — the read side of the memory-layer system.
//!
//! The `layer` column (`Core`/`Hot`/`Warm`/`Cold`/`Frozen`) sets
//! context-injection priority. `recall_by_layer` surfaces an initiative's
//! nodes grouped by layer, one bucket per requested layer in order, so an
//! agent re-entering a project loads the whole `Core` first, then `Hot`,
//! then `Warm` — exactly the priority the `Layer` enum was designed for.
//! `awake` builds its `layered` view on top of this.

use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};

use super::support::support_counts;
use super::{NodeBrief, parse_brief};
use crate::errors::Result;
use crate::graph::{Layer, NodeId, Tier};
use crate::store::Store;

/// One layer's worth of recalled nodes, with what it costs to load them.
#[derive(Debug, Clone)]
pub struct LayerBucket {
    pub layer: Layer,
    pub nodes: Vec<NodeBrief>,
    /// Characters the listed nodes occupy — name plus excerpt. Re-entry
    /// spends context, and until this existed the spend was a node count,
    /// which is not the unit anybody is actually billed in (#97). Roughly
    /// four characters to a token.
    pub chars: usize,
    /// Nodes in this layer the budget left out. Named rather than dropped
    /// silently: a window that quietly truncates is indistinguishable from a
    /// project that holds nothing more.
    pub omitted: usize,
}

/// Returns the current initiative's nodes grouped by `layer`, one bucket
/// per requested layer, in the given order. `Core` is **uncapped** — it is
/// "always in context"; every other layer is capped at
/// `config().active_window_size`. Audit-event nodes are excluded, and
/// within a bucket the newest assertions come first.
///
/// Initiative-scoped through `current_initiative`; with no active
/// initiative the buckets are cross-initiative.
pub fn recall_by_layer(store: &Store, layers: &[Layer]) -> Result<Vec<LayerBucket>> {
    recall_by_layer_in_tier(store, layers, None)
}

/// Like [`recall_by_layer`], but optionally restricted to one tier:
/// `Some(Tier::Operational)` for the in-flight working set (hippocampus),
/// `Some(Tier::Archival)` for settled knowledge (cortex), or `None` for both.
/// `awake` reads the two tiers separately so its working view and its cortex
/// view don't shadow each other (a Core archival fact belongs in cortex, not
/// mixed into the operational layers).
pub fn recall_by_layer_in_tier(
    store: &Store,
    layers: &[Layer],
    tier: Option<Tier>,
) -> Result<Vec<LayerBucket>> {
    // One read for the whole graph, shared by every bucket: the window is
    // sorted by how many live nodes point at a node, not by how recently it
    // was written (#97). "The last fifteen" was never "the needed fifteen".
    let support = support_counts(store)?;
    let mut budget = store.config().awake_budget_chars;

    let mut out = Vec::with_capacity(layers.len());
    for &layer in layers {
        let mut nodes = nodes_with_layer(store, layer, tier, &support)?;
        let total = nodes.len();
        let mut chars = 0usize;
        if layer == Layer::Core {
            // `core` promises to load whole, every session. Bounding it here
            // would break that promise quietly; hygiene bounds its SIZE
            // instead, where the bound is visible and reversible (#75).
            chars = nodes.iter().map(cost).sum();
        } else {
            let mut kept = Vec::with_capacity(nodes.len());
            for node in nodes {
                let c = cost(&node);
                if !kept.is_empty() && c > budget {
                    break;
                }
                budget = budget.saturating_sub(c);
                chars += c;
                kept.push(node);
            }
            nodes = kept;
        }
        let omitted = total - nodes.len();
        out.push(LayerBucket {
            layer,
            nodes,
            chars,
            omitted,
        });
    }
    Ok(out)
}

/// What one listed node costs to read: its name and its excerpt. The excerpt
/// is already truncated at `body_excerpt_chars`, so this is bounded per node
/// — what was not bounded is how many of them a re-entry loads.
fn cost(brief: &NodeBrief) -> usize {
    brief.name.chars().count()
        + brief
            .body_excerpt
            .as_deref()
            .map(|b| b.chars().count())
            .unwrap_or(0)
}

fn nodes_with_layer(
    store: &Store,
    layer: Layer,
    tier: Option<Tier>,
    support: &BTreeMap<NodeId, usize>,
) -> Result<Vec<NodeBrief>> {
    let excerpt = store.config().body_excerpt_chars;
    // Core is "always in context" → no cap. Other layers read a generous
    // multiple of the window and then sort by support: the scan stays bounded
    // while the choice of what to keep is made on the right key rather than
    // on whatever the query happened to return first.
    let limit_clause = match layer {
        Layer::Core => String::new(),
        _ => format!(":limit {}", store.config().active_window_size * 4),
    };

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("layer".to_string(), DataValue::Str(layer.as_str().into()));

    // Optional tier filter: bind `tier` in the node pattern and constrain it.
    let (tier_field, tier_cond) = match tier {
        Some(t) => {
            params.insert("tier".to_string(), DataValue::Str(t.as_str().into()));
            (", tier", ", tier = $tier")
        }
        None => ("", ""),
    };

    // `:order validity` yields newest-first: Cozo wraps the validity
    // timestamp in `Reverse<>`, so ascending order on the stored key is
    // descending in wall-clock time (same idiom as `recall_id_by_name`).
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            format!(
                r#"
                ?[id, type, name, body, validity] :=
                    *node_initiative{{initiative, node_id: id}}, initiative = $init,
                    *node{{id, type, name, body, layer{tier_field}, validity @ 'NOW'}}, layer = $layer{tier_cond},
                    type != 'audit_event'
                :order validity
                {limit_clause}
                "#
            )
        }
        None => format!(
            r#"
            ?[id, type, name, body, validity] :=
                *node{{id, type, name, body, layer{tier_field}, validity @ 'NOW'}}, layer = $layer{tier_cond},
                type != 'audit_event'
            :order validity
            {limit_clause}
            "#
        ),
    };

    let rows = store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Immutable)?;
    let mut briefs: Vec<NodeBrief> = rows
        .rows
        .iter()
        .map(|r| parse_brief(r.as_slice(), excerpt))
        .collect();
    // Most-supported first, and newest-first among equals — the query already
    // returned them newest-first, and a stable sort keeps that.
    briefs.sort_by_key(|b| std::cmp::Reverse(support.get(&b.id).copied().unwrap_or(0)));
    Ok(briefs)
}

#[cfg(test)]
mod tests {
    use crate::store::Store;
    use crate::{
        EpisodeKind, Layer, Significance, awake, cite_with_layer, write_episode,
        write_episode_with_layer,
    };

    /// `awake` returns Core → Hot → Warm in order, with the right node in
    /// each bucket, plus every initiative the substrate knows.
    #[test]
    fn awake_layers_core_then_hot_then_warm_and_lists_initiatives() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("proj");

        let core = write_episode_with_layer(
            &store,
            EpisodeKind::Decision,
            Significance::High,
            "core-fact",
            "the one fact everything hinges on",
            Layer::Core,
        )
        .unwrap();
        let hot = write_episode_with_layer(
            &store,
            EpisodeKind::Observation,
            Significance::Medium,
            "hot-task",
            "active work in progress",
            Layer::Hot,
        )
        .unwrap();
        // Default layer is Warm.
        let warm = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "warm-note",
            "useful reference",
        )
        .unwrap();

        // A node in another initiative must not leak into proj's buckets,
        // but its initiative must show up in `all_initiatives`.
        store.use_initiative("other");
        write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "x",
            "y",
        )
        .unwrap();

        store.use_initiative("proj");
        let ctx = awake(&store).expect("awake");

        assert_eq!(ctx.layered.len(), 3, "core/hot/warm buckets");
        assert_eq!(ctx.layered[0].layer, Layer::Core);
        assert_eq!(ctx.layered[1].layer, Layer::Hot);
        assert_eq!(ctx.layered[2].layer, Layer::Warm);

        assert!(
            ctx.layered[0].nodes.iter().any(|b| b.id == core),
            "core bucket has core-fact"
        );
        assert!(
            ctx.layered[1].nodes.iter().any(|b| b.id == hot),
            "hot bucket has hot-task"
        );
        assert!(
            ctx.layered[2].nodes.iter().any(|b| b.id == warm),
            "warm bucket has warm-note"
        );

        // No cross-bucket leakage.
        assert!(
            !ctx.layered[0]
                .nodes
                .iter()
                .any(|b| b.id == hot || b.id == warm)
        );

        // all_initiatives spans every initiative, not just the active one.
        assert!(ctx.all_initiatives.iter().any(|n| n == "proj"));
        assert!(ctx.all_initiatives.iter().any(|n| n == "other"));
    }

    /// `awake` splits the operational working set (`layered`) from the
    /// archival cortex (`cortex`): an in-flight episode lands in `layered`,
    /// a settled citation pinned to Core lands in `cortex` — and neither
    /// bleeds into the other.
    #[test]
    fn awake_splits_operational_layers_from_archival_cortex() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("proj");

        let wip = write_episode_with_layer(
            &store,
            EpisodeKind::Observation,
            Significance::Medium,
            "wip",
            "in flight",
            Layer::Hot,
        )
        .unwrap();
        // A settled fact, pinned to Core — standing knowledge that should
        // always re-enter via cortex.
        let fact = cite_with_layer(
            &store,
            "house-style",
            None,
            "always 4-space indent",
            Layer::Core,
        )
        .unwrap();

        let ctx = awake(&store).expect("awake");

        let in_layered = |id: &str| {
            ctx.layered
                .iter()
                .any(|b| b.nodes.iter().any(|n| n.id == id))
        };
        let in_cortex = |id: &str| ctx.cortex.iter().any(|n| n.id == id);

        assert!(in_layered(&wip), "operational episode in the working set");
        assert!(!in_cortex(&wip), "operational episode not in cortex");
        assert!(in_cortex(&fact), "settled citation surfaces in cortex");
        assert!(
            !in_layered(&fact),
            "archival fact not mixed into the layers"
        );
    }

    /// The window used to be "the newest fifteen by write time", which is a
    /// FIFO over writes and not over need (#97). The node the project keeps
    /// pointing at now leads its layer, however many newer notes exist.
    #[test]
    fn the_window_leads_with_what_the_graph_points_at() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("proj");

        let anchor = crate::jot(&store, "the decision everything rests on").expect("jot");
        for i in 0..3 {
            let referrer = crate::jot(&store, &format!("a note about it {i}")).expect("jot");
            crate::link(&store, &referrer, &anchor, crate::EdgeType::RefersTo).expect("link");
        }
        // Written last, referred to by nobody: recency alone would put it first.
        crate::jot(&store, "a passing thought nobody points at").expect("jot");

        let warm = crate::recall_by_layer(&store, &[Layer::Warm]).expect("recall");
        let first = &warm[0].nodes[0];
        assert_eq!(first.id, anchor, "most-supported first, not most-recent");
    }

    /// Re-entry spends context, and the spend is now a stated number rather
    /// than a node count — with anything the budget left out named, because
    /// a window that truncates silently reads like a project holding nothing
    /// more.
    #[test]
    fn a_bucket_states_its_cost_and_names_what_it_left_out() {
        let mut cfg = crate::config::KaeruConfig::defaults();
        // Room for roughly two of the notes below.
        cfg.awake_budget_chars = 120;
        let store = Store::open_in_memory_with(cfg).expect("open");
        store.use_initiative("proj");
        for i in 0..6 {
            crate::jot(
                &store,
                &format!("note {i} with a body long enough to cost something"),
            )
            .expect("jot");
        }

        let warm = crate::recall_by_layer(&store, &[Layer::Warm]).expect("recall");
        let bucket = &warm[0];
        assert!(
            bucket.nodes.len() < 6,
            "the budget bit: {} kept",
            bucket.nodes.len()
        );
        assert_eq!(
            bucket.omitted,
            6 - bucket.nodes.len(),
            "and what it left out is counted, not dropped quietly"
        );
        assert!(bucket.chars > 0, "the cost is stated");
    }

    /// `core` promises to load whole in every session. A budget that quietly
    /// bounded it would break that promise where nobody could see it —
    /// hygiene bounds core's SIZE instead, visibly and reversibly (#75).
    #[test]
    fn core_is_not_billed_against_the_budget() {
        let mut cfg = crate::config::KaeruConfig::defaults();
        cfg.awake_budget_chars = 1;
        let store = Store::open_in_memory_with(cfg).expect("open");
        store.use_initiative("proj");
        for i in 0..4 {
            let id = crate::jot(&store, &format!("standing fact {i}")).expect("jot");
            crate::set_layer(&store, &id, Layer::Core).expect("core");
        }

        let core = crate::recall_by_layer(&store, &[Layer::Core]).expect("recall");
        assert_eq!(core[0].nodes.len(), 4, "every core node loads");
        assert_eq!(core[0].omitted, 0);
        assert!(core[0].chars > 0, "its cost is still stated");
    }
}
