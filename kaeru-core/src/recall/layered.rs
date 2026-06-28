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

use super::{NodeBrief, parse_brief};
use crate::errors::Result;
use crate::graph::{Layer, Tier};
use crate::store::Store;

/// One layer's worth of recalled nodes.
#[derive(Debug, Clone)]
pub struct LayerBucket {
    pub layer: Layer,
    pub nodes: Vec<NodeBrief>,
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
    let mut out = Vec::with_capacity(layers.len());
    for &layer in layers {
        let nodes = nodes_with_layer(store, layer, tier)?;
        out.push(LayerBucket { layer, nodes });
    }
    Ok(out)
}

fn nodes_with_layer(store: &Store, layer: Layer, tier: Option<Tier>) -> Result<Vec<NodeBrief>> {
    let excerpt = store.config().body_excerpt_chars;
    // Core is "always in context" → no cap; other layers are bounded so a
    // large Warm tier can't flood the re-entry context.
    let limit_clause = match layer {
        Layer::Core => String::new(),
        _ => format!(":limit {}", store.config().active_window_size),
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
    let briefs = rows
        .rows
        .iter()
        .map(|r| parse_brief(r.as_slice(), excerpt))
        .collect();
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
}
