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
use crate::graph::Layer;
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
    let mut out = Vec::with_capacity(layers.len());
    for &layer in layers {
        let nodes = nodes_with_layer(store, layer)?;
        out.push(LayerBucket { layer, nodes });
    }
    Ok(out)
}

fn nodes_with_layer(store: &Store, layer: Layer) -> Result<Vec<NodeBrief>> {
    let excerpt = store.config().body_excerpt_chars;
    // Core is "always in context" → no cap; other layers are bounded so a
    // large Warm tier can't flood the re-entry context.
    let limit_clause = match layer {
        Layer::Core => String::new(),
        _ => format!(":limit {}", store.config().active_window_size),
    };

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("layer".to_string(), DataValue::Str(layer.as_str().into()));

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
                    *node{{id, type, name, body, layer, validity @ 'NOW'}}, layer = $layer,
                    type != 'audit_event'
                :order validity
                {limit_clause}
                "#
            )
        }
        None => format!(
            r#"
            ?[id, type, name, body, validity] :=
                *node{{id, type, name, body, layer, validity @ 'NOW'}}, layer = $layer,
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
    use crate::{EpisodeKind, Layer, Significance, awake, write_episode, write_episode_with_layer};

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
}
