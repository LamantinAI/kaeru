//! Which nodes the graph itself says are no longer current (#92).
//!
//! Some edge types point at a node to **cancel** it, not to support it. Every
//! rule that weighs a node by how many live nodes point at it has to know the
//! difference, or the edge that says "this is obsolete" becomes the edge that
//! keeps it loaded — which is exactly what happened to three `core` nodes of
//! one live initiative, superseded weeks earlier and injected into every
//! session since.
//!
//! This module is the one place that knows which types those are, so the
//! hygiene pass and `awake` cannot drift apart about it.

use std::collections::BTreeMap;

use cozo::ScriptMutability;

use crate::errors::Result;
use crate::graph::NodeId;
use crate::store::Store;

/// Edge types that cancel what they point at instead of supporting it.
///
/// `consolidated_to` is deliberately absent: it runs old → new, so the
/// inbound one sits on the *summary*, and treating it as a cancellation
/// would penalise the node that survived rather than the one consolidated
/// away.
pub const CANCELLING: [&str; 3] = ["supersedes", "contradicts", "falsifies"];

/// The cancellations that are a **verdict** — something replaced this node,
/// or refuted it — as opposed to `contradicts`, which is a doubt raised by
/// `flag` and not yet settled.
pub const VERDICT: [&str; 2] = ["supersedes", "falsifies"];

/// A live node's standing objection: what was written against it, and by
/// whom.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    /// `supersedes` or `falsifies` — see [`VERDICT`].
    pub edge_type: String,
    /// The name of the node that carries the verdict.
    pub peer: String,
}

impl Verdict {
    /// How a reader should be told, e.g. ``superseded by `state-evening` ``.
    pub fn phrase(&self) -> String {
        let verb = match self.edge_type.as_str() {
            "supersedes" => "superseded",
            "falsifies" => "refuted",
            other => other,
        };
        format!("{verb} by `{}`", self.peer)
    }
}

/// Every node that a **live** node has passed a verdict on, at NOW.
///
/// Cross-initiative on purpose: the node that replaces a fact is often
/// captured in the scope the work moved to, and a supersession that only
/// counts inside one initiative is a supersession an agent can walk around.
/// A verdict held by a retracted node does not count — the same rule the
/// inbound-edge count has always used.
pub fn verdicts_against(store: &Store) -> Result<BTreeMap<NodeId, Verdict>> {
    let types = format!(
        "[{}]",
        VERDICT
            .iter()
            .map(|t| format!("'{t}'"))
            .collect::<Vec<_>>()
            .join(", ")
    );
    let rows = store.db_ref().run_script(
        &format!(
            r#"
            ?[dst, edge_type, name] := *edge{{src, dst, edge_type @ 'NOW'}},
                                       is_in(edge_type, {types}),
                                       *node{{id: src, name @ 'NOW'}}
            "#
        ),
        BTreeMap::new(),
        ScriptMutability::Immutable,
    )?;

    let mut out: BTreeMap<NodeId, Verdict> = BTreeMap::new();
    for r in &rows.rows {
        let (Some(dst), Some(edge_type), Some(name)) = (
            r.first().and_then(|v| v.get_str()),
            r.get(1).and_then(|v| v.get_str()),
            r.get(2).and_then(|v| v.get_str()),
        ) else {
            continue;
        };
        out.entry(dst.to_string()).or_insert_with(|| Verdict {
            edge_type: edge_type.to_string(),
            peer: name.to_string(),
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::verdicts_against;
    use crate::graph::{EdgeType, NodeId, Significance};
    use crate::store::Store;
    use crate::{EpisodeKind, write_episode};

    fn episode(store: &Store, name: &str) -> NodeId {
        write_episode(
            store,
            EpisodeKind::Observation,
            Significance::Medium,
            name,
            "body",
        )
        .expect("write")
    }

    #[test]
    fn a_verdict_names_what_carries_it_and_a_doubt_is_not_one() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("proj");
        let stale = episode(&store, "the-old-direction");
        let successor = episode(&store, "the-new-direction");
        crate::link(&store, &successor, &stale, EdgeType::Supersedes).expect("link");

        let doubted = episode(&store, "the-doubted-fact");
        let doubt = episode(&store, "why-i-doubt-it");
        crate::link(&store, &doubt, &doubted, EdgeType::Contradicts).expect("link");

        let verdicts = verdicts_against(&store).expect("read");
        assert_eq!(
            verdicts.get(&stale).map(|v| v.phrase()),
            Some("superseded by `the-new-direction`".to_string())
        );
        assert!(
            !verdicts.contains_key(&doubted),
            "a flag is an open question, not a verdict"
        );
    }
}
