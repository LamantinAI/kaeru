//! `reflect` — the maintenance work-list for a reflection pass.
//!
//! Where [`lint`](crate::lint) returns raw hygiene issues, `reflect` computes a
//! fuller, actionable picture: orphans and open reviews (from `lint`), chains
//! the graph has made stale, operational nodes that have settled (cortex
//! candidates), and the shared nodes whose rebalancing must be escalated to the
//! user. The store computes *what* to fix; the adapter pairs it with *how*.

use std::collections::{BTreeMap, HashSet};

use cozo::{DataValue, NamedRows, ScriptMutability};

use super::board::effective_statuses;
use super::lint;
use super::open_work::open_tasks;
use crate::errors::Result;
use crate::graph::NodeId;
use crate::graph::temporal::validity_seconds;
use crate::hygiene::HYGIENE_EPISODE_PREFIX;
use crate::mutate::now_validity_seconds;
use crate::recall::path::{scoped_edge_rows, shortest_path_over};
use crate::store::Store;

/// The computed reflection work-list. Each field is a set of node ids to act
/// on; an all-empty report means the store is already tidy.
#[derive(Debug, Clone, Default)]
pub struct ReflectionReport {
    /// Nodes with no edges at NOW — `link` them or `forget` them.
    pub orphans: Vec<NodeId>,
    /// Nodes with an open `contradicts` review — `resolve` or `refute`.
    pub open_reviews: Vec<NodeId>,
    /// Chains whose stored members no longer match the shortest path between
    /// their endpoints (the graph changed underneath, or the endpoints became
    /// unreachable) — `rechain` to refresh.
    pub stale_chains: Vec<NodeId>,
    /// Operational, **referenced** nodes untouched past
    /// `reflect_settle_age_secs` — settled work other nodes still point at,
    /// worth `settle` / `cite` into the archival cortex.
    ///
    /// Deliberately narrower than "operational and old". Anything still open
    /// is excluded (`settle` on a live task would archive it out of the board
    /// and the open-work read-back), and so are structures rather than facts:
    /// what it means to settle a chain or a board is undefined. See
    /// [`archivable`](Self::archivable) for the other half of the old list.
    pub cortex_candidates: Vec<NodeId>,
    /// Operational nodes just as old and just as finished, that **nothing
    /// references any more** — delivered history rather than knowledge.
    ///
    /// These used to sit in `cortex_candidates` beside the rest, under a line
    /// recommending `settle`. Following it moved a finished sprint's whole
    /// journal into the tier `awake` loads whole and uncapped in every
    /// session — one kind of noise traded for a more expensive kind. `layer
    /// <name> cold` is the move: out of `awake`, still reachable through
    /// `surface layers=cold`.
    pub archivable: Vec<NodeId>,
    /// How many nodes the archival tier holds right now, in scope.
    ///
    /// Carried so the report can price its own recommendation. Cortex is
    /// injected whole into every session, so a section proposing to multiply
    /// it has to say by how much rather than print an item count and leave
    /// the reader to find out next session.
    pub cortex_size: usize,
    /// Shared nodes in scope. Touching the cloud (re-share, edge rebalance) is
    /// the user's call — escalate, don't auto-rewrite.
    pub shared: Vec<NodeId>,
    /// Open tasks whose `due:` date has already passed — `done` them, move
    /// them, or push the deadline. A maintenance item like any other: the
    /// graph knows the date passed, so the work-list should say so.
    pub overdue_tasks: Vec<NodeId>,
    /// Hypotheses still tagged `status:open` whose own text announces a
    /// verdict — "REFUTED", "VERDICT: PARTIAL" and friends.
    ///
    /// These exist because for a long time no tool would take a verdict at
    /// capture, so agents wrote it where they could: into the prose. The tag
    /// then says `open` while the body says the opposite, and every read
    /// surface trusts the tag — so the claim shows up forever as awaiting an
    /// answer it already has.
    ///
    /// Surfaced, never auto-corrected. Deciding what a body *means* is
    /// reading, and rewriting someone's text on a keyword match is a guess
    /// with the user's content as the stake. The work-list points; the agent
    /// (or the user) settles it with `confirm` / `refute` / `inconclusive`.
    pub contested_claims: Vec<NodeId>,
}

/// Builds the reflection work-list for the active initiative (cross-initiative
/// when none is selected). Read-only — it computes, never mutates.
pub fn reflect(store: &Store) -> Result<ReflectionReport> {
    let hygiene = lint(store)?;
    let settleable = cortex_candidates(store)?;
    Ok(ReflectionReport {
        orphans: hygiene.orphans,
        open_reviews: hygiene.unresolved_reviews,
        stale_chains: stale_chains(store)?,
        cortex_candidates: settleable.referenced,
        archivable: settleable.inert,
        cortex_size: cortex_size(store)?,
        shared: shared_nodes(store)?,
        overdue_tasks: open_tasks(store)?
            .into_iter()
            .filter(|t| t.overdue)
            .map(|t| t.id)
            .collect(),
        contested_claims: contested_claims(store)?,
    })
}

fn first_col_ids(rows: &NamedRows) -> Vec<NodeId> {
    rows.rows
        .iter()
        .filter_map(|r| r.first().and_then(|v| v.get_str()).map(String::from))
        .collect()
}

/// Chains whose materialised path is out of date.
fn stale_chains(store: &Store) -> Result<Vec<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
            ?[id] := *node{id, type @ 'NOW'}, type = 'chain',
                     *node_initiative{initiative, node_id: id}, initiative = $init
            "#
        }
        None => r#"?[id] := *node{id, type @ 'NOW'}, type = 'chain'"#,
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    let chain_ids = first_col_ids(&rows);
    if chain_ids.is_empty() {
        return Ok(Vec::new());
    }

    // Scan the edge set once and reuse it for every chain, instead of
    // re-scanning `*edge` inside a per-chain `shortest_path` (O(chains × edges)).
    let edges = scoped_edge_rows(store)?;
    let mut stale = Vec::new();
    for cid in chain_ids {
        let members = chain_members(store, &cid)?;
        if members.len() < 2 {
            continue;
        }
        let recomputed =
            shortest_path_over(store, &edges, &members[0], &members[members.len() - 1])?;
        if recomputed != members {
            stale.push(cid);
        }
    }
    Ok(stale)
}

/// Ordered member ids of a chain, raw from the junction.
fn chain_members(store: &Store, chain_id: &NodeId) -> Result<Vec<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("cid".to_string(), DataValue::Str(chain_id.clone().into()));
    let rows = store.db_ref().run_script(
        r#"
        ?[position, node_id] := *chain_member{chain_id, position, node_id}, chain_id = $cid
        :order position
        "#,
        params,
        ScriptMutability::Immutable,
    )?;
    Ok(rows
        .rows
        .iter()
        .filter_map(|r| r.get(1).and_then(|v| v.get_str()).map(String::from))
        .collect())
}

/// Node types that are never "settled knowledge", whatever their age.
///
/// `chain` and `board` are structures over nodes rather than facts that
/// stopped changing — settling one is undefined, and `settle` would retract
/// the structure and write an `outcome` in its place. `audit_event` was
/// already excluded; these belong out for the same reason.
const NOT_SETTLEABLE_TYPES: [&str; 3] = ["chain", "board", "audit_event"];

/// The two halves of the old candidate list, split by whether anything still
/// points at the node.
struct Settleable {
    referenced: Vec<NodeId>,
    inert: Vec<NodeId>,
}

/// Operational nodes that have sat untouched long enough to look settled, and
/// that it is actually safe to settle — split by whether the graph still
/// refers to them.
///
/// Three things are excluded that the first version of this query let through,
/// each of which made the printed recommendation wrong to follow (#76):
///
/// - **Open work.** `status:open` is what the board and the claim verbs write,
///   and `settle` on one archives a live task out of the board and out of
///   `awake`'s open-work read-back. A task is settleable once it reaches its
///   board's terminal column, not once it goes quiet for a fortnight.
/// - **Structures.** See [`NOT_SETTLEABLE_TYPES`].
/// - **The tools' own bookkeeping.** The hygiene pass writes a durable
///   episode per run; recommending that the agent settle the sweeper's diary
///   is noise the agent cannot distinguish from real work.
///
/// The split is on **inbound** edges. A finished thing other nodes point at is
/// knowledge worth loading; a finished thing nothing points at is delivered
/// history, and belongs in `cold` rather than in the tier that loads whole
/// every session.
fn cortex_candidates(store: &Store) -> Result<Settleable> {
    let cutoff = now_validity_seconds() as f64 - store.config().reflect_settle_age_secs as f64;
    let terminal = terminal_status(store)?;

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
            inbound[id] := *edge{src, dst: id, edge_type @ 'NOW'}
            connected[id] := *edge{src: id, dst, edge_type @ 'NOW'}
            connected[id] := *edge{src, dst: id, edge_type @ 'NOW'}
            ?[id, type, name, tags, validity] :=
                *node{id, type, tier, name, tags, validity @ 'NOW'},
                tier = 'operational', type != 'audit_event',
                connected[id],
                *node_initiative{initiative, node_id: id}, initiative = $init
            :order validity
            "#
        }
        None => {
            r#"
            connected[id] := *edge{src: id, dst, edge_type @ 'NOW'}
            connected[id] := *edge{src, dst: id, edge_type @ 'NOW'}
            ?[id, type, name, tags, validity] :=
                *node{id, type, tier, name, tags, validity @ 'NOW'},
                tier = 'operational', type != 'audit_event',
                connected[id]
            :order validity
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;

    let inbound = inbound_ids(store)?;
    let mut out = Settleable {
        referenced: Vec::new(),
        inert: Vec::new(),
    };
    for r in &rows.rows {
        if !validity_seconds(r.get(4)).is_some_and(|ts| ts < cutoff) {
            continue;
        }
        let Some(id) = r.first().and_then(|v| v.get_str()) else {
            continue;
        };
        let node_type = r.get(1).and_then(|v| v.get_str()).unwrap_or_default();
        if NOT_SETTLEABLE_TYPES.contains(&node_type) {
            continue;
        }
        let name = r.get(2).and_then(|v| v.get_str()).unwrap_or_default();
        if name.starts_with(HYGIENE_EPISODE_PREFIX) {
            continue;
        }
        let tags = tag_list(r.get(3));
        if !is_finished(&tags, node_type, terminal.as_deref()) {
            continue;
        }
        if inbound.contains(id) {
            out.referenced.push(id.to_string());
        } else {
            out.inert.push(id.to_string());
        }
    }
    Ok(out)
}

/// Reads a node's `tags` column into plain strings.
fn tag_list(value: Option<&DataValue>) -> Vec<String> {
    match value {
        Some(DataValue::List(items)) => items
            .iter()
            .filter_map(|v| v.get_str().map(String::from))
            .collect(),
        _ => Vec::new(),
    }
}

/// Whether a node's own tags say its work is over.
///
/// A `status:` tag is the node saying so itself, and it outranks the clock:
/// "nobody has touched it in a fortnight" is not a verdict, it is a silence.
/// A node carrying no status tag at all was never a work item, and age is all
/// there is to go on.
fn is_finished(tags: &[String], node_type: &str, terminal: Option<&str>) -> bool {
    let status = tags
        .iter()
        .find_map(|t| t.strip_prefix("status:"))
        .map(str::to_string);
    match status {
        None => true,
        Some(s) if s == "open" => false,
        // A task belongs to its board, so its board decides when it is done:
        // the terminal column, whatever the initiative renamed it to. Any
        // column before that is work in flight, however quiet.
        Some(s) if node_type == "task" => terminal.is_some_and(|t| s == t),
        // Everything else (a claim's verdict, a review's resolution) is a
        // status that is not `open`, which is the whole test.
        Some(_) => true,
    }
}

/// The key of the last column of the initiative's board — reaching it is what
/// makes a task done. `None` when reflect is running cross-initiative, where
/// there is no one board to ask — and with no board to say otherwise, no task
/// is treated as settled. Recommending nothing is the cheap mistake here;
/// recommending that a live task be archived is not.
fn terminal_status(store: &Store) -> Result<Option<String>> {
    let Some(init) = store.current_initiative() else {
        return Ok(None);
    };
    Ok(effective_statuses(store, &init)?
        .last()
        .map(|s| s.key.clone()))
}

/// Ids that at least one live edge points **at**.
fn inbound_ids(store: &Store) -> Result<HashSet<NodeId>> {
    let rows = store.db_ref().run_script(
        r#"?[dst] := *edge{src, dst, edge_type @ 'NOW'}"#,
        BTreeMap::new(),
        ScriptMutability::Immutable,
    )?;
    Ok(rows
        .rows
        .iter()
        .filter_map(|r| r.first().and_then(|v| v.get_str()).map(String::from))
        .collect())
}

/// How many nodes the archival tier holds in scope, so a recommendation to
/// grow it can be priced.
fn cortex_size(store: &Store) -> Result<usize> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
            ?[id] := *node{id, tier, type @ 'NOW'}, tier = 'archival', type != 'audit_event',
                     *node_initiative{initiative, node_id: id}, initiative = $init
            "#
        }
        None => {
            r#"?[id] := *node{id, tier, type @ 'NOW'}, tier = 'archival', type != 'audit_event'"#
        }
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    Ok(rows.rows.len())
}

/// Shared nodes in scope — any cloud-touching rebalance is the user's call.
fn shared_nodes(store: &Store) -> Result<Vec<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
            ?[id] := *node{id, visibility @ 'NOW'}, visibility = 'shared',
                     *node_initiative{initiative, node_id: id}, initiative = $init
            "#
        }
        None => r#"?[id] := *node{id, visibility @ 'NOW'}, visibility = 'shared'"#,
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    Ok(first_col_ids(&rows))
}

/// Verdict words as they actually appear in a claim that never got one
/// through the API: shouted in caps, or introduced as a label. Deliberately
/// narrow — a body that merely *discusses* refutation in lowercase prose is
/// not making a claim about its own status, and flagging it would train the
/// agent to ignore the section.
const VERDICT_MARKERS: [&str; 8] = [
    "REFUTED",
    "CONFIRMED",
    "SUPPORTED",
    "INCONCLUSIVE",
    "FALSIFIED",
    "VERDICT:",
    "verdict:",
    "PARTIAL",
];

/// Open hypotheses whose name or body announces a verdict the tag doesn't
/// carry. See [`ReflectionReport::contested_claims`].
fn contested_claims(store: &Store) -> Result<Vec<NodeId>> {
    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            r#"
            ?[id, name, body] :=
                *node_initiative{initiative, node_id: id}, initiative = $init,
                *node{id, type, name, body, tags @ 'NOW'},
                type = 'hypothesis',
                !is_null(tags),
                is_in('status:open', tags)
            "#
        }
        None => {
            r#"
            ?[id, name, body] :=
                *node{id, type, name, body, tags @ 'NOW'},
                type = 'hypothesis',
                !is_null(tags),
                is_in('status:open', tags)
            "#
        }
    };
    let rows = store
        .db_ref()
        .run_script(script, params, ScriptMutability::Immutable)?;
    Ok(rows
        .rows
        .iter()
        .filter(|r| {
            let text = [r.get(1), r.get(2)]
                .iter()
                .filter_map(|c| c.and_then(|v| v.get_str()))
                .collect::<Vec<_>>()
                .join(" ");
            VERDICT_MARKERS.iter().any(|m| text.contains(m))
        })
        .filter_map(|r| r.first().and_then(|v| v.get_str()).map(String::from))
        .collect())
}

#[cfg(test)]
mod contested_tests {
    use super::reflect;
    use crate::store::Store;
    use crate::{HypothesisStatus, Layer, formulate_hypothesis_with_status};

    /// The exact shape found in the wild: an open claim whose own body
    /// announces the verdict. The tag says `open`, so every read surface
    /// keeps asking for an answer the node already has.
    #[test]
    fn a_claim_shouting_its_verdict_is_surfaced() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        formulate_hypothesis_with_status(
            &store,
            "the-cache-claim",
            "REFUTED — the cache cost more than it saved",
            Layer::default(),
            HypothesisStatus::Open,
        )
        .expect("claim");
        formulate_hypothesis_with_status(
            &store,
            "an-honest-open-one",
            "the index may help under load",
            Layer::default(),
            HypothesisStatus::Open,
        )
        .expect("claim");

        let r = reflect(&store).expect("reflect");
        assert_eq!(r.contested_claims.len(), 1, "{:?}", r.contested_claims);
    }

    /// A claim already carrying a real verdict is not contested — it is done.
    #[test]
    fn a_settled_claim_is_not_contested() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        formulate_hypothesis_with_status(
            &store,
            "settled",
            "REFUTED — and the tag agrees",
            Layer::default(),
            HypothesisStatus::Refuted,
        )
        .expect("claim");
        assert!(
            reflect(&store)
                .expect("reflect")
                .contested_claims
                .is_empty()
        );
    }

    /// Lowercase prose that merely discusses refutation is not a claim about
    /// the node's own status. Flagging it would train the agent to skip the
    /// section.
    #[test]
    fn ordinary_prose_about_refuting_things_is_left_alone() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        formulate_hypothesis_with_status(
            &store,
            "quiet",
            "we should try to refute this by measuring the cold path",
            Layer::default(),
            HypothesisStatus::Open,
        )
        .expect("claim");
        assert!(
            reflect(&store)
                .expect("reflect")
                .contested_claims
                .is_empty()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::reflect;
    use crate::config::KaeruConfig;
    use crate::graph::EdgeType;
    use crate::graph::NodeId;
    use crate::store::Store;
    use crate::{EpisodeKind, Significance, link_with_weight, mark_under_review, write_episode};

    #[test]
    fn reflect_flags_orphans_and_open_reviews() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("p");
        let orphan = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "lonely",
            "no edges",
        )
        .unwrap();
        let a = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "a",
            "A",
        )
        .unwrap();
        let b = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "b",
            "B",
        )
        .unwrap();
        link_with_weight(&store, &a, &b, EdgeType::RefersTo, 0.9).unwrap();
        mark_under_review(&store, &a, "needs another look").unwrap();

        let r = reflect(&store).unwrap();
        assert!(r.orphans.contains(&orphan), "orphan flagged");
        assert!(!r.orphans.contains(&a), "linked node not an orphan");
        assert!(r.open_reviews.contains(&a), "open review flagged");
    }

    #[test]
    fn reflect_flags_stale_chain_after_graph_change() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("p");
        let a = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "a",
            "A",
        )
        .unwrap();
        let b = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "b",
            "B",
        )
        .unwrap();
        let c = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "c",
            "C",
        )
        .unwrap();
        link_with_weight(&store, &a, &b, EdgeType::RefersTo, 0.9).unwrap();
        link_with_weight(&store, &b, &c, EdgeType::RefersTo, 0.9).unwrap();
        let chain = crate::create_chain(&store, &a, &c, None, None)
            .unwrap()
            .unwrap()
            .id;

        assert!(
            reflect(&store).unwrap().stale_chains.is_empty(),
            "fresh chain not stale"
        );

        // A strong direct edge changes the shortest path → chain is stale.
        link_with_weight(&store, &a, &c, EdgeType::RefersTo, 1.0).unwrap();
        assert!(
            reflect(&store).unwrap().stale_chains.contains(&chain),
            "chain flagged stale after the graph changed"
        );
    }

    #[test]
    fn reflect_flags_settled_node_as_cortex_candidate() {
        let mut cfg = KaeruConfig::defaults();
        cfg.reflect_settle_age_secs = 0; // everything linked counts as settled
        let store = Store::open_in_memory_with(cfg).expect("open");
        store.use_initiative("p");
        let a = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "a",
            "A",
        )
        .unwrap();
        let b = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "b",
            "B",
        )
        .unwrap();
        link_with_weight(&store, &a, &b, EdgeType::RefersTo, 0.5).unwrap();

        // Cross the whole-second boundary so the assertion is strictly older
        // than the (now) cutoff.
        std::thread::sleep(std::time::Duration::from_millis(1100));
        let r = reflect(&store).unwrap();
        // `a` points at `b` and nothing points at `a`: `b` is knowledge
        // something still refers to, `a` is delivered history. Both are
        // settled; they earn different advice.
        assert!(
            r.cortex_candidates.contains(&b),
            "a referenced settled node is a cortex candidate"
        );
        assert!(
            r.archivable.contains(&a),
            "one nothing points at is delivered work for `layer cold`"
        );
    }

    /// The reported list: two open tasks, an unverdicted claim and four chains
    /// under a line that said "settled — `settle`/`cite` into cortex".
    /// Following it as printed would have archived live work out of the board
    /// and the open-claims read-back, and settled four structures for which
    /// settling is undefined.
    #[test]
    fn unfinished_work_is_not_a_cortex_candidate() {
        use crate::mutate::{create_chain, formulate_hypothesis, write_task};

        let mut cfg = KaeruConfig::defaults();
        cfg.reflect_settle_age_secs = 0;
        let store = Store::open_in_memory_with(cfg).expect("open");
        store.use_initiative("p");

        let task = write_task(&store, "ship the thing", None).unwrap();
        let claim = formulate_hypothesis(&store, "it-is-faster", "measured once").unwrap();
        let anchor = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "anchor",
            "A",
        )
        .unwrap();
        // Every one of them linked, so the old query's only filter passes.
        link_with_weight(&store, &anchor, &task, EdgeType::RefersTo, 0.5).unwrap();
        link_with_weight(&store, &anchor, &claim, EdgeType::RefersTo, 0.5).unwrap();
        let step = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "step",
            "S",
        )
        .unwrap();
        link_with_weight(&store, &step, &anchor, EdgeType::DerivedFrom, 1.0).unwrap();
        let chain = create_chain(&store, &step, &anchor, Some("trail"), None)
            .unwrap()
            .expect("path");

        std::thread::sleep(std::time::Duration::from_millis(1100));
        let r = reflect(&store).unwrap();
        let listed = |id: &NodeId| r.cortex_candidates.contains(id) || r.archivable.contains(id);

        assert!(!listed(&task), "an open task is not settled work");
        assert!(!listed(&claim), "a claim awaiting a verdict is not settled");
        assert!(
            !listed(&chain.id),
            "a chain is a structure over nodes — settling one is undefined"
        );
        assert!(listed(&anchor), "ordinary settled work is still listed");
    }

    /// A done task is settleable — the exclusion is about open work, not about
    /// tasks. The board's own terminal column decides, so an initiative that
    /// renamed its columns still gets this right.
    #[test]
    fn a_finished_task_is_settleable_again() {
        use crate::mutate::{complete_task, write_task};

        let mut cfg = KaeruConfig::defaults();
        cfg.reflect_settle_age_secs = 0;
        let store = Store::open_in_memory_with(cfg).expect("open");
        store.use_initiative("p");

        let task = write_task(&store, "ship the thing", None).unwrap();
        let anchor = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "anchor",
            "A",
        )
        .unwrap();
        link_with_weight(&store, &anchor, &task, EdgeType::RefersTo, 0.5).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1100));
        complete_task(&store, &task).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1100));

        let r = reflect(&store).unwrap();
        let id = crate::recall_id_by_name(&store, "ship-the-thing")
            .unwrap()
            .unwrap_or(task);
        assert!(
            r.cortex_candidates.contains(&id) || r.archivable.contains(&id),
            "a done task is settled work like any other"
        );
    }

    /// The pass's own durable episode was showing up in *orphans* with the
    /// advice to link or forget it — bookkeeping the agent cannot tell apart
    /// from a note it dropped.
    #[test]
    fn the_hygiene_passes_own_diary_is_not_a_finding() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("p");
        let diary = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "hygiene-p-1750000000",
            "3 archived",
        )
        .unwrap();
        let dropped = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "a-real-loose-note",
            "…",
        )
        .unwrap();

        let r = reflect(&store).unwrap();
        assert!(
            !r.orphans.contains(&diary),
            "the sweeper's diary is not work"
        );
        assert!(r.orphans.contains(&dropped), "a real orphan still is");
    }

    /// The report carries what cortex costs, so the renderer can price its own
    /// recommendation instead of printing an item count.
    #[test]
    fn the_report_knows_how_big_cortex_already_is() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("p");
        crate::cite(&store, "the-adr", None, "a settled decision").unwrap();
        crate::cite(&store, "the-other-adr", None, "another").unwrap();

        let r = reflect(&store).unwrap();
        assert_eq!(r.cortex_size, 2, "both references are archival");
    }
}
