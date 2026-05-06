//! kaeru-core
//!
//! Cognitive memory layer for LLM agents — typed graph over CozoDB embedded
//! with bi-temporal `Validity` native, two-tier (operational / archival) with
//! explicit `consolidate_out`, per-initiative subgraph via junction-relation
//! pattern.
//!
//! Design philosophy: facilitator, not enforcer. Cognitive primitives are
//! available tools; agent and user choose when to invoke.

pub mod config;
pub mod errors;
pub mod export;
pub mod graph;
pub mod mutate;
pub mod recall;
pub mod session;
pub mod store;

pub use config::KaeruConfig;
pub use errors::Error;
pub use errors::Result;
pub use export::ExportSummary;
pub use export::export_vault;
pub use graph::EdgeType;
pub use graph::EpisodeKind;
pub use graph::HypothesisStatus;
pub use graph::NodeId;
pub use graph::NodeSnapshot;
pub use graph::NodeType;
pub use graph::Revision;
pub use graph::Significance;
pub use graph::Tier;
pub use graph::at;
pub use graph::history;
pub use graph::new_node_id;
pub use mutate::cite;
pub use mutate::consolidate_in;
pub use mutate::consolidate_out;
pub use mutate::forget;
pub use mutate::formulate_hypothesis;
pub use mutate::improve;
pub use mutate::jot;
pub use mutate::link;
pub use mutate::mark_resolved;
pub use mutate::mark_under_review;
pub use mutate::run_experiment;
pub use mutate::supersedes;
pub use mutate::synthesise;
pub use mutate::unlink;
pub use mutate::update_hypothesis_status;
pub use mutate::write_episode;
pub use recall::LintReport;
pub use recall::NodeBrief;
pub use recall::SummaryView;
pub use recall::FUZZY_RECALL_LIMIT_CAP;
pub use recall::EdgeRow;
pub use recall::between;
pub use recall::count_by_type;
pub use recall::fuzzy_recall;
pub use recall::lint;
pub use recall::list_initiatives;
pub use recall::overview;
pub use recall::node_brief_by_id;
pub use recall::recall_id_by_name;
pub use recall::recent_episodes;
pub use recall::recollect_idea;
pub use recall::recollect_outcome;
pub use recall::recollect_provenance;
pub use recall::summary_view;
pub use recall::tagged;
pub use recall::under_review_pinned;
pub use recall::walk;
pub use session::AwakenedContext;
pub use session::active_window;
pub use session::awake;
pub use session::pin;
pub use session::unpin;
pub use store::Store;

/// Returns the package version as declared in Cargo.toml.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::EdgeType;
    use super::EpisodeKind;
    use super::Error;
    use super::HypothesisStatus;
    use super::NodeType;
    use super::Significance;
    use super::Store;
    use super::Tier;
    use super::active_window;
    use super::at;
    use super::awake;
    use super::export_vault;
    use super::fuzzy_recall;
    use super::list_initiatives;
    use super::overview;
    use super::consolidate_in;
    use super::consolidate_out;
    use super::count_by_type;
    use super::forget;
    use super::formulate_hypothesis;
    use super::improve;
    use super::jot;
    use super::lint;
    use super::history;
    use super::link;
    use super::mark_resolved;
    use super::mark_under_review;
    use super::pin;
    use super::recall_id_by_name;
    use super::recent_episodes;
    use super::recollect_idea;
    use super::recollect_outcome;
    use super::recollect_provenance;
    use super::run_experiment;
    use super::summary_view;
    use super::supersedes;
    use super::synthesise;
    use super::under_review_pinned;
    use super::unlink;
    use super::unpin;
    use super::update_hypothesis_status;
    use super::version;
    use super::walk;
    use super::write_episode;

    #[test]
    fn smoke_version() {
        assert!(!version().is_empty());
    }

    /// Initiative auto-attachment isolates writes per `use_initiative`
    /// scope: an episode written under `alpha` does not surface in
    /// `recall_id_by_name` / `recent_episodes` / `awake` while `beta` is
    /// the active initiative. `list_initiatives` returns both names.
    #[test]
    fn initiative_auto_attach_filters_reads() {
        let store = Store::open_in_memory().expect("open");

        // Write under alpha.
        store.use_initiative("alpha");
        let alpha_id = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "alpha-thought",
            "noticed under alpha",
        )
        .unwrap();

        // Write under beta.
        store.use_initiative("beta");
        let beta_id = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "beta-thought",
            "noticed under beta",
        )
        .unwrap();

        // While beta is current, recall_id_by_name should NOT find the
        // alpha-thought; recent_episodes should not contain alpha_id.
        let recalled = recall_id_by_name(&store, "alpha-thought").unwrap();
        assert!(recalled.is_none(), "alpha-thought hidden under beta scope");
        let recent = recent_episodes(&store, 3600).unwrap();
        assert!(recent.contains(&beta_id));
        assert!(!recent.contains(&alpha_id), "alpha hidden under beta scope");

        // Cross-initiative read (no current initiative): both visible.
        store.clear_initiative();
        let cross = recent_episodes(&store, 3600).unwrap();
        assert!(cross.contains(&alpha_id));
        assert!(cross.contains(&beta_id));

        let initiatives = list_initiatives(&store).unwrap();
        assert!(initiatives.iter().any(|n| n == "alpha"));
        assert!(initiatives.iter().any(|n| n == "beta"));
    }

    /// `export_vault` writes a hierarchical markdown snapshot. Verify
    /// frontmatter, body, and `## Outgoing`/`### derived_from` rendering
    /// with `[[wikilink]]` references for a synthesised structure plus
    /// a flagged target so the open-questions section also exercises.
    #[test]
    fn export_vault_renders_frontmatter_and_wikilinks() {
        use std::env;
        use std::fs;

        let store = Store::open_in_memory().expect("open");
        store.use_initiative("research");

        let seed = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "raw-seed",
            "raw observation",
        )
        .unwrap();
        let _idea = synthesise(
            &store,
            &[seed.clone()],
            NodeType::Idea,
            Tier::Archival,
            "settled-idea",
            "stable form",
        )
        .unwrap();
        // Flag the seed with a contradicts edge so the open-questions
        // section in INDEX.md gets exercised.
        mark_under_review(&store, &seed, "second look needed").unwrap();

        let out_root = env::temp_dir().join(format!(
            "kaeru-test-export-{}",
            super::new_node_id()
        ));

        let summary = export_vault(&store, &out_root).expect("export");
        assert_eq!(summary.initiative.as_deref(), Some("research"));
        // seed + idea + review-episode that mark_under_review created.
        assert_eq!(summary.nodes_exported, 3);
        // derived_from (idea→seed) + contradicts (review→seed) = 2.
        assert_eq!(summary.edges_exported, 2);

        let idea_path = out_root.join("archival/idea/settled-idea.md");
        let seed_path = out_root.join("operational/episode/raw-seed.md");
        assert!(idea_path.exists(), "idea file at expected hierarchical path");
        assert!(seed_path.exists(), "seed file at expected hierarchical path");

        let idea_md = fs::read_to_string(&idea_path).unwrap();
        assert!(idea_md.contains("type: idea"));
        assert!(idea_md.contains("tier: archival"));
        assert!(idea_md.contains("- research"), "initiatives frontmatter present");
        assert!(
            idea_md.contains("## Outgoing\n\n### derived_from\n\n- [[raw-seed]]"),
            "outgoing derived_from wikilink to seed; got:\n{idea_md}"
        );

        let seed_md = fs::read_to_string(&seed_path).unwrap();
        assert!(seed_md.contains("## Incoming"));
        assert!(seed_md.contains("### derived_from"));
        assert!(
            seed_md.contains("[[settled-idea]]"),
            "settled-idea wikilink in seed's incoming; got:\n{seed_md}"
        );
        assert!(seed_md.contains("### contradicts"));

        // llm-wiki framing: README, INDEX, LOG must exist with
        // expected content shape.
        let readme = fs::read_to_string(out_root.join("README.md")).unwrap();
        assert!(readme.contains("# kaeru vault"));
        assert!(readme.contains("Scope: initiative `research`"));
        assert!(readme.contains("Nodes: 3"));
        assert!(readme.contains("[[INDEX]]") && readme.contains("[[LOG]]"));

        let index = fs::read_to_string(out_root.join("INDEX.md")).unwrap();
        assert!(index.contains("# Index"));
        assert!(index.contains("## operational"));
        assert!(index.contains("### episode"));
        assert!(index.contains("[[raw-seed]]"));
        assert!(index.contains("## archival"));
        assert!(index.contains("### idea"));
        assert!(index.contains("[[settled-idea]]"));

        // Graph-structure sections: provenance forest from settled-idea
        // points down to raw-seed; edge stats include both edge types;
        // open questions surfaces raw-seed (contradicts target).
        assert!(index.contains("## Provenance forests"));
        assert!(
            index.contains("[[settled-idea]] (idea)"),
            "archival root rendered with type tag; got:\n{index}"
        );
        assert!(
            index.contains("- [[raw-seed]]"),
            "derived_from child rendered as nested wikilink; got:\n{index}"
        );
        assert!(index.contains("## Open questions"));
        assert!(
            index.contains("[[raw-seed]]"),
            "open-question target rendered; got:\n{index}"
        );
        assert!(index.contains("## Edge stats"));
        assert!(index.contains("`derived_from`: 1"));
        assert!(index.contains("`contradicts`: 1"));

        let log = fs::read_to_string(out_root.join("LOG.md")).unwrap();
        assert!(log.contains("# Log"));
        assert!(log.contains("write_episode"));
        assert!(log.contains("synthesise"));
        assert!(log.contains("[[raw-seed]]"));
        assert!(log.contains("[[settled-idea]]"));

        let _ = fs::remove_dir_all(&out_root);
    }

    /// `overview` renders a terminal-readable map of the substrate:
    /// counts by tier/type, provenance forests, open questions, and
    /// edge stats. Initiative-scoped.
    #[test]
    fn overview_renders_all_sections() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("research");

        let seed = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "raw-seed",
            "raw observation",
        )
        .unwrap();
        let _idea = synthesise(
            &store,
            &[seed.clone()],
            NodeType::Idea,
            Tier::Archival,
            "settled-idea",
            "stable form",
        )
        .unwrap();
        mark_under_review(&store, &seed, "second look needed").unwrap();

        let report = overview(&store).expect("overview");
        assert!(report.contains("initiative `research`"));
        assert!(report.contains("operational:"));
        assert!(report.contains("archival:"));
        assert!(report.contains("episode ("), "episode count line; got:\n{report}");
        assert!(report.contains("idea ("));
        assert!(report.contains("provenance:"));
        assert!(report.contains("settled-idea"));
        assert!(report.contains("- raw-seed"), "derived_from child rendered; got:\n{report}");
        assert!(report.contains("open questions:"));
        assert!(report.contains("edge stats:"));
        assert!(report.contains("derived_from: 1"));
        assert!(report.contains("contradicts: 1"));

        // Empty initiative — clean degenerate output.
        store.use_initiative("empty");
        let empty = overview(&store).expect("overview empty");
        assert!(empty.contains("0 node(s), 0 edge(s)"));
        assert!(empty.contains("(no nodes)"));
    }

    /// `jot` writes a low-friction episode with an auto-derived name
    /// from the body's first words plus a unique 6-char id suffix.
    /// Two jots with identical bodies must still get distinct names.
    #[test]
    fn jot_auto_names_episode_uniquely() {
        let store = Store::open_in_memory().expect("open");

        let id_a = jot(&store, "auth token expiry seems off").unwrap();
        let id_b = jot(&store, "auth token expiry seems off").unwrap();
        assert_ne!(id_a, id_b);

        let brief_a = super::node_brief_by_id(&store, &id_a).unwrap().unwrap();
        let brief_b = super::node_brief_by_id(&store, &id_b).unwrap().unwrap();
        assert_ne!(brief_a.name, brief_b.name, "auto-names must differ");
        assert!(brief_a.name.starts_with("auth-token-expiry-seems-off-"));
        assert!(brief_b.name.starts_with("auth-token-expiry-seems-off-"));

        // Empty / non-alphanumeric body falls back to `jot-<suffix>`.
        let id_empty = jot(&store, "   ").unwrap();
        let brief_empty = super::node_brief_by_id(&store, &id_empty).unwrap().unwrap();
        assert!(brief_empty.name.starts_with("jot-"));

        // Tagged with `role:jot` so a future filter can find them.
        let count = count_by_type(&store, "episode").unwrap();
        assert_eq!(count, 3, "three episodes (jots are episodes)");
    }

    /// `fuzzy_recall` finds nodes by content via Cozo FTS. Matches in
    /// either `name` or `body` count; results are deduplicated by id
    /// and ordered by score. Initiative scope, when set, restricts
    /// hits. The Simple tokenizer splits on non-alphanumeric and does
    /// not stem, so the test uses exact-form words.
    #[test]
    fn fuzzy_recall_matches_name_and_body_dedup_by_id() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("auth");

        let _e1 = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "expiry-bug",
            "auth token expired on the device",
        )
        .unwrap();
        let _e2 = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "tenant-leak",
            "the session leaks across tenants",
        )
        .unwrap();
        let _e3 = write_episode(
            &store,
            EpisodeKind::Decision,
            Significance::High,
            "rotation",
            "rotate token hourly going forward",
        )
        .unwrap();

        // 'token' is in two bodies (expiry-bug body, rotation body) —
        // both surface, deduped one row per id.
        let hits = fuzzy_recall(&store, "token", 10).expect("fuzzy");
        let names: Vec<&String> = hits.iter().map(|b| &b.name).collect();
        assert!(names.iter().any(|n| n.as_str() == "expiry-bug"), "got {names:?}");
        assert!(names.iter().any(|n| n.as_str() == "rotation"), "got {names:?}");
        assert!(
            !names.iter().any(|n| n.as_str() == "tenant-leak"),
            "no 'token' reference; got {names:?}"
        );
        // Each id should appear once, even though name+body indexes
        // could both have hit.
        let mut ids: Vec<&String> = hits.iter().map(|b| &b.id).collect();
        ids.sort();
        let unique_count = {
            let mut copy = ids.clone();
            copy.dedup();
            copy.len()
        };
        assert_eq!(ids.len(), unique_count, "results must be deduped by id");

        // Body-only match: 'tenants' in tenant-leak's body.
        let hits = fuzzy_recall(&store, "tenants", 10).expect("fuzzy tenants");
        assert!(hits.iter().any(|b| b.name == "tenant-leak"));

        // Name match on a single segment after tokenization: 'tenant' is
        // in tenant-leak's name (split on '-' produces ['tenant', 'leak']).
        let hits = fuzzy_recall(&store, "tenant", 10).expect("fuzzy tenant");
        assert!(hits.iter().any(|b| b.name == "tenant-leak"));

        // Other initiative — no hits.
        store.use_initiative("other");
        let hits = fuzzy_recall(&store, "token", 10).expect("fuzzy other scope");
        assert!(hits.is_empty(), "auth nodes hidden under other scope");
    }

    /// Phase-2 initiative filtering: `walk`, `summary_view`,
    /// `recollect_provenance`, and `lint` honour `current_initiative`.
    /// A graph chain in `alpha` is fully invisible from `beta`'s view
    /// even though it lives in the same substrate.
    #[test]
    fn initiative_filter_walk_summary_provenance_lint() {
        let store = Store::open_in_memory().expect("open");

        // Build a chain in alpha: a → b (causal), then a synthesised
        // summary derived from a + b.
        store.use_initiative("alpha");
        let a = write_episode(&store, EpisodeKind::Observation, Significance::Low, "alpha-a", "A").unwrap();
        let b = write_episode(&store, EpisodeKind::Observation, Significance::Low, "alpha-b", "B").unwrap();
        link(&store, &a, &b, EdgeType::Causal).unwrap();
        let summary = synthesise(
            &store,
            &[a.clone(), b.clone()],
            NodeType::Idea,
            Tier::Operational,
            "alpha-summary",
            "summary in alpha",
        )
        .unwrap();

        // A solitary node in beta — it's an orphan within beta's view.
        store.use_initiative("beta");
        let solo = write_episode(&store, EpisodeKind::Observation, Significance::Low, "beta-solo", "S").unwrap();

        // Switch to beta and verify alpha's chain is invisible.
        // walk from a (alpha seed) under beta scope returns empty.
        let walked = walk(&store, &a, &[EdgeType::Causal], 2).unwrap();
        assert!(walked.is_empty(), "alpha chain hidden from beta walk");

        // summary_view of an alpha node from beta → NotFound.
        let summary_attempt = summary_view(&store, &summary);
        assert!(matches!(summary_attempt, Err(Error::NotFound(_))));

        // recollect_provenance of an alpha summary under beta → empty.
        let prov = recollect_provenance(&store, &summary).unwrap();
        assert!(prov.is_empty(), "alpha provenance not leaking to beta");

        // lint under beta surfaces only beta's orphan, not alpha's nodes.
        let report = lint(&store).unwrap();
        assert!(report.orphans.contains(&solo));
        assert!(!report.orphans.contains(&a));
        assert!(!report.orphans.contains(&b));

        // Cross-initiative: everything visible.
        store.clear_initiative();
        let cross_walk = walk(&store, &a, &[EdgeType::Causal], 2).unwrap();
        assert!(cross_walk.contains(&a) && cross_walk.contains(&b));
        let cross_summary = summary_view(&store, &summary).unwrap();
        assert_eq!(cross_summary.root.id, summary);
        assert_eq!(cross_summary.children.len(), 2, "summary derived from 2 seeds");
    }

    /// Disk-mode persistence: open a fresh RocksDB-backed `Store`, write
    /// an episode, drop the store, reopen on the same path, recall the
    /// episode by name. Verifies that `Store::open` actually flushes
    /// state through the embedded RocksDB engine.
    #[test]
    fn disk_store_persists_across_reopen() {
        use std::env;
        use std::fs;

        let path = env::temp_dir().join(format!(
            "kaeru-test-disk-{}",
            super::new_node_id()
        ));

        let written_id = {
            let store = Store::open(&path).expect("open disk store");
            write_episode(
                &store,
                EpisodeKind::Observation,
                Significance::Medium,
                "persist-me",
                "should survive reopen",
            )
            .expect("write episode")
        };

        let recalled = {
            let store = Store::open(&path).expect("reopen disk store");
            recall_id_by_name(&store, "persist-me")
                .expect("recall after reopen")
                .expect("episode present after reopen")
        };

        assert_eq!(written_id, recalled, "id survives across reopen");

        // Best-effort cleanup; failures here are not test failures.
        let _ = fs::remove_dir_all(&path);
    }

    /// Two episodes, a causal link between them, recall by name, and a
    /// count of automatically-written audit events. Exercises the basic
    /// `write_episode` / `link` / `recall_id_by_name` / `count_by_type`
    /// surface in one place.
    #[test]
    fn write_recall_link_audit_chain() {
        let store = Store::open_in_memory().expect("open store");

        let a = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Medium,
            "first-observation",
            "I noticed X.",
        )
        .expect("write episode A");
        let b = write_episode(
            &store,
            EpisodeKind::Decision,
            Significance::High,
            "first-decision",
            "Therefore I do Y.",
        )
        .expect("write episode B");

        link(&store, &a, &b, EdgeType::Causal).expect("link A→B");

        let recalled = recall_id_by_name(&store, "first-observation")
            .expect("recall by name")
            .expect("episode A present by name");
        assert_eq!(recalled, a, "recall returns the right id");

        let episodes = count_by_type(&store, "episode").expect("count episodes");
        assert_eq!(episodes, 2, "two episodes were written");
        let audits = count_by_type(&store, "audit_event").expect("count audit_events");
        assert_eq!(audits, 3, "three audit events: 2 writes + 1 link");
    }

    /// Build a chain a → b → c with `causal` edges, walk from a with
    /// max_hops=2 → expect a, b, c.
    #[test]
    fn walk_two_hops_along_typed_chain() {
        let store = Store::open_in_memory().expect("open store");

        let a = write_episode(&store, EpisodeKind::Observation, Significance::Low, "a", "A").unwrap();
        let b = write_episode(&store, EpisodeKind::Observation, Significance::Low, "b", "B").unwrap();
        let c = write_episode(&store, EpisodeKind::Observation, Significance::Low, "c", "C").unwrap();

        link(&store, &a, &b, EdgeType::Causal).unwrap();
        link(&store, &b, &c, EdgeType::Causal).unwrap();

        let mut reached = walk(&store, &a, &[EdgeType::Causal], 2).expect("walk");
        reached.sort();
        let mut expected = vec![a.clone(), b.clone(), c.clone()];
        expected.sort();
        assert_eq!(reached, expected, "all three nodes reached within 2 hops");
    }

    /// max_hops = 1 stops before reaching c.
    #[test]
    fn walk_respects_hop_limit() {
        let store = Store::open_in_memory().expect("open");

        let a = write_episode(&store, EpisodeKind::Observation, Significance::Low, "a", "A").unwrap();
        let b = write_episode(&store, EpisodeKind::Observation, Significance::Low, "b", "B").unwrap();
        let c = write_episode(&store, EpisodeKind::Observation, Significance::Low, "c", "C").unwrap();

        link(&store, &a, &b, EdgeType::Causal).unwrap();
        link(&store, &b, &c, EdgeType::Causal).unwrap();

        let reached = walk(&store, &a, &[EdgeType::Causal], 1).expect("walk");
        assert!(reached.contains(&a));
        assert!(reached.contains(&b));
        assert!(!reached.contains(&c), "c is two hops away");
    }

    /// Edge type filter: walk over `causal` does not follow `refers_to`.
    #[test]
    fn walk_filters_by_edge_type() {
        let store = Store::open_in_memory().expect("open");

        let a = write_episode(&store, EpisodeKind::Observation, Significance::Low, "a", "A").unwrap();
        let b = write_episode(&store, EpisodeKind::Observation, Significance::Low, "b", "B").unwrap();

        link(&store, &a, &b, EdgeType::RefersTo).unwrap();

        let reached = walk(&store, &a, &[EdgeType::Causal], 2).expect("walk");
        assert_eq!(reached, vec![a], "only seed reached, refers_to not followed");
    }

    /// max_hops over the cap → Error::Invalid.
    #[test]
    fn walk_rejects_excessive_max_hops() {
        let store = Store::open_in_memory().expect("open");
        let a = write_episode(&store, EpisodeKind::Observation, Significance::Low, "a", "A").unwrap();
        let result = walk(&store, &a, &[EdgeType::Causal], 99);
        assert!(matches!(result, Err(Error::Invalid(_))));
    }

    /// Bi-temporal point-in-time read: insert at t1=1000 with body="first";
    /// retract + re-assert at t2=2000 with body="second". `at(1500)` resolves
    /// "first"; `at(2500)` resolves "second"; `history` returns three rows.
    #[test]
    fn temporal_at_and_history() {
        let store = Store::open_in_memory().expect("open");

        let s1 = r#"
            ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
                [['c1', [1000.0, true], 'concept', 'archival', 'TestConcept', 'first', null, null, null]]
            :put node {id, validity => type, tier, name, body, tags, initiatives, properties}
        "#;
        store.run(s1).expect("insert at t1");

        let s2 = r#"
            ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
                [['c1', [2000.0, false], 'concept', 'archival', 'TestConcept', null, null, null, null],
                 ['c1', [2000.0, true],  'concept', 'archival', 'TestConcept', 'second', null, null, null]]
            :put node {id, validity => type, tier, name, body, tags, initiatives, properties}
        "#;
        store.run(s2).expect("retract + reassert at t2");

        let id = "c1".to_string();

        let snap_1500 = at(&store, &id, 1500.0).expect("at 1500").expect("present");
        assert_eq!(snap_1500.body.as_deref(), Some("first"));

        let snap_2500 = at(&store, &id, 2500.0).expect("at 2500").expect("present");
        assert_eq!(snap_2500.body.as_deref(), Some("second"));

        let hist = history(&store, &id).expect("history");
        assert_eq!(hist.len(), 3, "three rows: assert@1000, retract@2000, assert@2000");
        assert!(hist.iter().any(|r| r.asserted && r.body.as_deref() == Some("first")));
        assert!(hist.iter().any(|r| !r.asserted));
        assert!(hist.iter().any(|r| r.asserted && r.body.as_deref() == Some("second")));
    }

    /// Three episodes synthesised into one summary. The summary is reachable
    /// from each seed by walking `derived_from` one hop in reverse, and the
    /// audit log records exactly one synthesise event.
    #[test]
    fn synthesise_many_to_one_with_derived_from_edges() {
        let store = Store::open_in_memory().expect("open");

        let a = write_episode(&store, EpisodeKind::Observation, Significance::Low, "obs-a", "A").unwrap();
        let b = write_episode(&store, EpisodeKind::Observation, Significance::Low, "obs-b", "B").unwrap();
        let c = write_episode(&store, EpisodeKind::Observation, Significance::Low, "obs-c", "C").unwrap();

        let summary_id = synthesise(
            &store,
            &[a.clone(), b.clone(), c.clone()],
            NodeType::Summary,
            Tier::Operational,
            "obs-summary",
            "A + B + C consolidated",
        )
        .expect("synthesise");

        // walk(summary, derived_from, 1) reaches all three seeds.
        let mut reached = walk(&store, &summary_id, &[EdgeType::DerivedFrom], 1).expect("walk");
        reached.sort();
        let mut expected = vec![summary_id.clone(), a.clone(), b.clone(), c.clone()];
        expected.sort();
        assert_eq!(reached, expected, "summary + 3 seeds via derived_from");

        // Audit count: 3 write_episodes + 1 synthesise = 4.
        let audits = count_by_type(&store, "audit_event").expect("count audits");
        assert_eq!(audits, 4, "3 episode writes + 1 synthesise event");
    }

    /// Empty seed list is a usage error.
    #[test]
    fn synthesise_rejects_empty_seeds() {
        let store = Store::open_in_memory().expect("open");
        let result = synthesise(
            &store,
            &[],
            NodeType::Summary,
            Tier::Operational,
            "empty",
            "—",
        );
        assert!(matches!(result, Err(Error::Invalid(_))));
    }

    /// pin → active_window contains; unpin → no longer contains.
    #[test]
    fn pin_unpin_active_window_round_trip() {
        let store = Store::open_in_memory().expect("open");

        let a = write_episode(&store, EpisodeKind::Observation, Significance::Low, "node-a", "A").unwrap();
        let b = write_episode(&store, EpisodeKind::Observation, Significance::Low, "node-b", "B").unwrap();

        pin(&store, &a, "currently investigating A").expect("pin a");
        pin(&store, &b, "follow-up on A").expect("pin b");

        let window = active_window(&store).expect("window");
        assert!(window.contains(&a));
        assert!(window.contains(&b));
        assert_eq!(window.len(), 2);

        unpin(&store, &a).expect("unpin a");
        let window2 = active_window(&store).expect("window");
        assert!(!window2.contains(&a), "a is no longer pinned");
        assert!(window2.contains(&b), "b stays pinned");
        assert_eq!(window2.len(), 1);
    }

    /// `recent_episodes` returns episodes within the window, capped and ordered
    /// newest-first; `under_review_pinned` surfaces nodes with inbound
    /// `contradicts` edges (the open-review queue).
    #[test]
    fn recent_episodes_and_under_review_pinned() {
        let store = Store::open_in_memory().expect("open");

        // An "old" episode, then a 2.1s gap, then two "new" ones. 2.1s clears
        // the whole-second resolution boundary so the old episode's validity
        // is at least 2 seconds before the new ones.
        let old = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "ep-old",
            "OLD",
        )
        .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(2100));

        let new_a = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "ep-a",
            "A",
        )
        .unwrap();
        let new_b = write_episode(
            &store,
            EpisodeKind::Decision,
            Significance::Medium,
            "ep-b",
            "B",
        )
        .unwrap();

        // Wide window catches all three.
        let all = recent_episodes(&store, 3600).expect("recent wide");
        assert!(all.contains(&old));
        assert!(all.contains(&new_a));
        assert!(all.contains(&new_b));

        // Tight 1-second window excludes the old episode.
        let tight = recent_episodes(&store, 1).expect("recent tight");
        assert!(!tight.contains(&old), "old episode is older than 1s window");
        assert!(tight.contains(&new_a));
        assert!(tight.contains(&new_b));

        // mark_under_review on `new_a` puts it in the open-review queue.
        mark_under_review(&store, &new_a, "questioning A").unwrap();

        let queue = under_review_pinned(&store).expect("under_review_pinned");
        assert!(queue.contains(&new_a), "new_a is the contradicts target");
        assert!(!queue.contains(&new_b), "new_b has no inbound contradicts");
        assert!(!queue.contains(&old), "old has no inbound contradicts");
    }

    /// Custom `KaeruConfig` overrides actually take effect on primitives.
    /// Verifies that `Store::open_in_memory_with` is the test-friendly
    /// path the design promised — caps tweak behaviour deterministically.
    #[test]
    fn custom_config_overrides_caps() {
        use super::KaeruConfig;

        let mut config = KaeruConfig::defaults();
        config.recent_episodes_cap = 2;
        config.max_hops_cap = 1;

        let store = Store::open_in_memory_with(config).expect("open");

        // 3 episodes, but cap of 2 → at most 2 returned.
        write_episode(&store, EpisodeKind::Observation, Significance::Low, "e1", "1").unwrap();
        write_episode(&store, EpisodeKind::Observation, Significance::Low, "e2", "2").unwrap();
        write_episode(&store, EpisodeKind::Observation, Significance::Low, "e3", "3").unwrap();
        let recent = super::recent_episodes(&store, 3600).expect("recent");
        assert!(recent.len() <= 2, "cap of 2 honoured, got {}", recent.len());

        // walk with max_hops=2 must now fail (cap is 1).
        let seed = recall_id_by_name(&store, "e1").unwrap().unwrap();
        let result = walk(&store, &seed, &[EdgeType::Causal], 2);
        assert!(matches!(result, Err(Error::Invalid(_))), "max_hops=2 exceeds custom cap=1");
    }

    /// `forget` retracts a node and every edge connected to it at NOW.
    /// Reads at NOW skip the node; `at(t)` for `t` before forget still
    /// resolves it.
    #[test]
    fn forget_retracts_node_and_connected_edges() {
        let store = Store::open_in_memory().expect("open");

        let a = write_episode(&store, EpisodeKind::Observation, Significance::Low, "a", "A").unwrap();
        let b = write_episode(&store, EpisodeKind::Observation, Significance::Low, "b", "B").unwrap();
        let c = write_episode(&store, EpisodeKind::Observation, Significance::Low, "c", "C").unwrap();

        link(&store, &a, &b, EdgeType::Causal).unwrap();
        link(&store, &b, &c, EdgeType::Causal).unwrap();

        // Pre-forget: walk a → causal → reaches b and c.
        let pre = walk(&store, &a, &[EdgeType::Causal], 2).unwrap();
        assert!(pre.contains(&b));
        assert!(pre.contains(&c));

        std::thread::sleep(std::time::Duration::from_millis(1100));

        forget(&store, &b).expect("forget b");

        // recall_id_by_name on b returns None at NOW.
        let recalled = recall_id_by_name(&store, "b").expect("recall");
        assert!(recalled.is_none(), "b is no longer asserted at NOW");

        // walk a → causal no longer reaches b or c (b's edges retracted).
        let post = walk(&store, &a, &[EdgeType::Causal], 2).unwrap();
        assert!(!post.contains(&b), "b retracted");
        assert!(!post.contains(&c), "c unreachable: edge b→c retracted");
    }

    /// `improve` rewrites name + body of a node, preserving id and type.
    /// `at(t)` before the call shows the old content; reads at NOW show
    /// the new content.
    #[test]
    fn improve_rewrites_name_and_body_preserving_id() {
        let store = Store::open_in_memory().expect("open");

        let id = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "draft",
            "first take",
        )
        .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(1100));

        improve(&store, &id, "draft-revised", "second take after revision").expect("improve");

        // recall_id_by_name on the new name returns the same id.
        let recalled = recall_id_by_name(&store, "draft-revised")
            .expect("recall")
            .expect("present");
        assert_eq!(recalled, id, "id is preserved across improve");

        // Old name no longer resolves at NOW.
        let stale = recall_id_by_name(&store, "draft").expect("recall stale");
        assert!(stale.is_none());

        // Audits: 1 write_episode + 1 improve.
        let audits = count_by_type(&store, "audit_event").expect("count");
        assert_eq!(audits, 2);
    }

    /// `lint` reports orphan nodes (no edges) and unresolved reviews.
    #[test]
    fn lint_reports_orphans_and_unresolved_reviews() {
        let store = Store::open_in_memory().expect("open");

        // Linked pair — neither is an orphan.
        let a = write_episode(&store, EpisodeKind::Observation, Significance::Low, "a", "A").unwrap();
        let b = write_episode(&store, EpisodeKind::Observation, Significance::Low, "b", "B").unwrap();
        link(&store, &a, &b, EdgeType::Causal).unwrap();

        // Solitary episode — orphan.
        let solo = write_episode(&store, EpisodeKind::Observation, Significance::Low, "solo", "S").unwrap();

        // Reviewed episode — `b` becomes a contradicts target.
        mark_under_review(&store, &b, "questioning B").unwrap();

        let report = lint(&store).expect("lint");

        assert!(report.orphans.contains(&solo), "solo node is orphan");
        assert!(!report.orphans.contains(&a), "a is linked");
        assert!(!report.orphans.contains(&b), "b is linked");
        assert!(report.unresolved_reviews.contains(&b));
    }

    /// `consolidate_out` promotes an operational draft to an archival
    /// counterpart and replicates its `derived_from` edges so the
    /// provenance chain is reachable from the new archival node.
    #[test]
    fn consolidate_out_preserves_provenance_across_tier() {
        let store = Store::open_in_memory().expect("open");

        let ep_a = write_episode(&store, EpisodeKind::Observation, Significance::Low, "ep-a", "A").unwrap();
        let ep_b = write_episode(&store, EpisodeKind::Observation, Significance::Low, "ep-b", "B").unwrap();

        // Operational draft synthesised from two episodes.
        let draft = synthesise(
            &store,
            &[ep_a.clone(), ep_b.clone()],
            NodeType::Idea,
            Tier::Operational,
            "operational-draft",
            "An idea taking shape.",
        )
        .unwrap();

        // 1.1s sleep so retract / reassert distinguish in whole-second
        // validity resolution.
        std::thread::sleep(std::time::Duration::from_millis(1100));

        let archived = consolidate_out(
            &store,
            &draft,
            NodeType::Idea,
            "settled-idea",
            "Stable form of the idea.",
        )
        .expect("consolidate_out");
        assert_ne!(archived, draft, "consolidation produces a fresh id");

        // Provenance from the archival node walks back to the original episodes.
        let provenance = recollect_provenance(&store, &archived).expect("provenance");
        let ids: Vec<&String> = provenance.iter().map(|b| &b.id).collect();
        assert!(ids.contains(&&ep_a), "ep_a still reachable from archival");
        assert!(ids.contains(&&ep_b), "ep_b still reachable from archival");

        // Walking `consolidated_to` from the old draft reaches the new archival node.
        let reached = walk(&store, &draft, &[EdgeType::ConsolidatedTo], 1).unwrap();
        assert!(reached.contains(&archived), "draft → consolidated_to → archived");
    }

    /// `consolidate_in` is the reverse direction — archival back to
    /// operational. Edge still goes old → new, marking the consolidation
    /// event itself.
    #[test]
    fn consolidate_in_demotes_archival_to_operational() {
        let store = Store::open_in_memory().expect("open");

        let seed = write_episode(&store, EpisodeKind::Observation, Significance::Low, "seed", "S").unwrap();
        let archival = synthesise(
            &store,
            &[seed.clone()],
            NodeType::Idea,
            Tier::Archival,
            "stable",
            "Currently stable idea.",
        )
        .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(1100));

        let operational = consolidate_in(
            &store,
            &archival,
            NodeType::Idea,
            "draft-revision",
            "Reopening for revision.",
        )
        .expect("consolidate_in");

        // Provenance still points back to seed via replicated derived_from.
        let provenance = recollect_provenance(&store, &operational).expect("provenance");
        let ids: Vec<&String> = provenance.iter().map(|b| &b.id).collect();
        assert!(ids.contains(&&seed));

        // consolidated_to: archival → operational.
        let reached = walk(&store, &archival, &[EdgeType::ConsolidatedTo], 1).unwrap();
        assert!(reached.contains(&operational));
    }

    /// `recollect_provenance` walks `derived_from` ancestors from a node.
    /// A two-level synthesise chain (episodes → idea → outcome) yields the
    /// idea + episodes when starting from the outcome.
    #[test]
    fn recollect_provenance_walks_derived_from_chain() {
        let store = Store::open_in_memory().expect("open");

        let ep_a = write_episode(&store, EpisodeKind::Observation, Significance::Low, "ep-a", "A").unwrap();
        let ep_b = write_episode(&store, EpisodeKind::Observation, Significance::Low, "ep-b", "B").unwrap();

        // Level 1: idea synthesised from two episodes.
        let idea = synthesise(
            &store,
            &[ep_a.clone(), ep_b.clone()],
            NodeType::Idea,
            Tier::Archival,
            "the-idea",
            "Pattern noticed across two observations.",
        )
        .unwrap();

        // Level 2: outcome synthesised from the idea.
        let outcome = synthesise(
            &store,
            &[idea.clone()],
            NodeType::Outcome,
            Tier::Archival,
            "the-outcome",
            "Settled conclusion based on the idea.",
        )
        .unwrap();

        let provenance = recollect_provenance(&store, &outcome).expect("provenance");
        let ids: Vec<&String> = provenance.iter().map(|b| &b.id).collect();

        assert!(ids.contains(&&idea), "idea is direct ancestor of outcome");
        assert!(ids.contains(&&ep_a), "ep_a reached transitively through idea");
        assert!(ids.contains(&&ep_b), "ep_b reached transitively through idea");
        assert!(!ids.contains(&&outcome), "seed itself excluded from its own provenance");

        // A seed with no derived_from edges → empty provenance.
        let lonely = write_episode(&store, EpisodeKind::Observation, Significance::Low, "lonely", "no provenance").unwrap();
        let lonely_prov = recollect_provenance(&store, &lonely).expect("lonely provenance");
        assert!(lonely_prov.is_empty());
    }

    /// `recollect_idea` / `recollect_outcome` list archival-tier Ideas and
    /// Outcomes. Operational nodes of the same type, and archival nodes of
    /// other types, are excluded.
    #[test]
    fn recollect_idea_and_outcome_filter_by_tier_and_type() {
        let store = Store::open_in_memory().expect("open");

        // Seed an operational episode to use as provenance.
        let seed = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "seed",
            "raw observation",
        )
        .unwrap();

        // Two archival ideas, one archival outcome, one operational summary
        // (which should NOT show up in either recollect call).
        let idea_a = synthesise(
            &store,
            &[seed.clone()],
            NodeType::Idea,
            Tier::Archival,
            "idea-a",
            "A long-term idea.",
        )
        .unwrap();
        let idea_b = synthesise(
            &store,
            &[seed.clone()],
            NodeType::Idea,
            Tier::Archival,
            "idea-b",
            "Another long-term idea.",
        )
        .unwrap();
        let outcome = synthesise(
            &store,
            &[seed.clone()],
            NodeType::Outcome,
            Tier::Archival,
            "outcome-1",
            "A settled outcome.",
        )
        .unwrap();
        let op_summary = synthesise(
            &store,
            &[seed.clone()],
            NodeType::Summary,
            Tier::Operational,
            "op-summary",
            "Operational summary, not archival.",
        )
        .unwrap();

        let ideas = recollect_idea(&store).expect("recollect_idea");
        let idea_ids: Vec<&String> = ideas.iter().map(|b| &b.id).collect();
        assert!(idea_ids.contains(&&idea_a));
        assert!(idea_ids.contains(&&idea_b));
        assert!(!idea_ids.contains(&&outcome), "outcomes filtered out of ideas");
        assert!(!idea_ids.contains(&&op_summary), "operational excluded");
        for brief in &ideas {
            assert_eq!(brief.node_type, "idea");
        }

        let outcomes = recollect_outcome(&store).expect("recollect_outcome");
        let outcome_ids: Vec<&String> = outcomes.iter().map(|b| &b.id).collect();
        assert!(outcome_ids.contains(&&outcome));
        assert!(!outcome_ids.contains(&&idea_a), "ideas filtered out of outcomes");
        for brief in &outcomes {
            assert_eq!(brief.node_type, "outcome");
        }
    }

    /// `summary_view` returns the seed brief plus 1-hop drill-down children
    /// — sources via outgoing `derived_from` and parts via incoming
    /// `part_of`. PageIndex-style hierarchical navigation surface.
    #[test]
    fn summary_view_returns_root_and_drilldown_children() {
        let store = Store::open_in_memory().expect("open");

        // Three observation seeds + one synthesised summary above them.
        // The summary's outgoing derived_from edges point to each seed.
        let s1 = write_episode(&store, EpisodeKind::Observation, Significance::Low, "seed-1", "first observation").unwrap();
        let s2 = write_episode(&store, EpisodeKind::Observation, Significance::Low, "seed-2", "second observation").unwrap();
        let s3 = write_episode(&store, EpisodeKind::Observation, Significance::Low, "seed-3", "third observation").unwrap();
        let summary = synthesise(
            &store,
            &[s1.clone(), s2.clone(), s3.clone()],
            NodeType::Summary,
            Tier::Operational,
            "obs-summary",
            "Combined view of three observations.",
        )
        .unwrap();

        let view = summary_view(&store, &summary).expect("summary_view");

        assert_eq!(view.root.id, summary);
        assert_eq!(view.root.name, "obs-summary");
        assert_eq!(view.root.node_type, "summary");
        assert!(view.root.body_excerpt.as_deref().is_some());

        // All three sources surface as children (via outgoing derived_from).
        let child_ids: Vec<&String> = view.children.iter().map(|c| &c.id).collect();
        assert!(child_ids.contains(&&s1));
        assert!(child_ids.contains(&&s2));
        assert!(child_ids.contains(&&s3));

        // A node with no derived_from / part_of relationships → empty children.
        let lonely = write_episode(&store, EpisodeKind::Observation, Significance::Low, "lonely", "Solo node").unwrap();
        let lonely_view = summary_view(&store, &lonely).expect("summary_view lonely");
        assert!(lonely_view.children.is_empty(), "isolated node has no drill-down targets");

        // Missing seed → NotFound.
        let missing = summary_view(&store, &"non-existent".to_string());
        assert!(matches!(missing, Err(Error::NotFound(_))));
    }

    /// `awake` bundles initiative, pinned set, recent episodes, and the
    /// open-review queue into one struct — single call for session
    /// restoration.
    #[test]
    fn awake_bundles_session_restoration_state() {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("kaeru");

        let a = write_episode(&store, EpisodeKind::Observation, Significance::Low, "a", "A").unwrap();
        let b = write_episode(&store, EpisodeKind::Decision, Significance::High, "b", "B").unwrap();
        pin(&store, &a, "actively investigating").expect("pin a");
        mark_under_review(&store, &b, "inconsistent with prior decision").unwrap();

        let ctx = awake(&store).expect("awake");

        assert_eq!(ctx.initiative.as_deref(), Some("kaeru"));
        assert!(ctx.pinned.contains(&a), "pinned set has a");
        assert!(ctx.recent.contains(&a), "recent set has a (just written)");
        assert!(ctx.recent.contains(&b), "recent set has b (just written)");
        assert!(ctx.under_review.contains(&b), "b is in the open-review queue");
        assert!(!ctx.under_review.contains(&a), "a was not flagged");
    }

    /// `unlink` retracts an edge through bi-temporal substrate. Walk after
    /// unlink no longer reaches the previously-linked node.
    #[test]
    fn unlink_retracts_edge_then_walk_does_not_reach() {
        let store = Store::open_in_memory().expect("open");

        let a = write_episode(&store, EpisodeKind::Observation, Significance::Low, "a", "A").unwrap();
        let b = write_episode(&store, EpisodeKind::Observation, Significance::Low, "b", "B").unwrap();

        link(&store, &a, &b, EdgeType::Causal).unwrap();
        let before = walk(&store, &a, &[EdgeType::Causal], 1).unwrap();
        assert!(before.contains(&b), "before unlink: b is reachable");

        // Whole-second resolution: distinct timestamps for assert and retract.
        std::thread::sleep(std::time::Duration::from_millis(1100));

        unlink(&store, &a, &b, EdgeType::Causal).unwrap();
        let after = walk(&store, &a, &[EdgeType::Causal], 1).unwrap();
        assert!(!after.contains(&b), "after unlink: b is no longer reachable");
        assert!(after.contains(&a), "seed always present");
    }

    /// Hypothesis-experiment cycle: formulate, run experiment, mark supported.
    /// Verifies the targets edge from experiment → hypothesis exists, the
    /// verifies edge from experiment → hypothesis is created on Supported,
    /// and the new hypothesis row carries `status:supported` after the update.
    #[test]
    fn hypothesis_experiment_cycle_supported() {
        let store = Store::open_in_memory().expect("open");

        let h = formulate_hypothesis(&store, "h-claim", "Cozo Validity composes with [String]?")
            .expect("formulate");

        let exp = run_experiment(
            &store,
            &h,
            "exp-validity-list",
            "Insert at t1 with list, retract+reassert at t2 with different list, query is_in @ t1 and t2.",
        )
        .expect("run_experiment");

        // experiment → targets → hypothesis
        let reached_targets = walk(&store, &exp, &[EdgeType::Targets], 1).unwrap();
        assert!(reached_targets.contains(&h));

        std::thread::sleep(std::time::Duration::from_millis(1100));

        update_hypothesis_status(&store, &h, HypothesisStatus::Supported, &exp)
            .expect("update_hypothesis_status");

        // experiment → verifies → hypothesis
        let reached_verifies = walk(&store, &exp, &[EdgeType::Verifies], 1).unwrap();
        assert!(reached_verifies.contains(&h));

        // The hypothesis row at NOW should carry the supported status tag.
        // (No public tag-read primitive yet; we read directly via Cozo.)
        let rows = store
            .run_read(&format!(
                "?[tags] := *node{{id, tags @ 'NOW'}}, id = '{h}'"
            ))
            .expect("read tags");
        let tags_str = format!("{:?}", rows.rows);
        assert!(
            tags_str.contains("status:supported"),
            "hypothesis tags must include status:supported, got: {tags_str}"
        );

        // Audit count: formulate + run_experiment + update_hypothesis_status = 3.
        let audits = count_by_type(&store, "audit_event").expect("count audits");
        assert_eq!(audits, 3);
    }

    /// Refuted hypothesis: a falsifies edge appears instead of verifies.
    #[test]
    fn hypothesis_refuted_writes_falsifies_edge() {
        let store = Store::open_in_memory().expect("open");

        let h = formulate_hypothesis(&store, "h-bad", "X holds always").expect("formulate");
        let counterexample = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::High,
            "counterexample",
            "X fails when condition C is met.",
        )
        .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(1100));

        update_hypothesis_status(&store, &h, HypothesisStatus::Refuted, &counterexample)
            .expect("refute");

        let reached = walk(&store, &counterexample, &[EdgeType::Falsifies], 1).unwrap();
        assert!(reached.contains(&h));
    }

    /// `mark_resolved` adds a supersedes edge from the resolution to the
    /// question and writes a `mark_resolved` audit event.
    #[test]
    fn mark_resolved_links_resolution_supersedes_question() {
        let store = Store::open_in_memory().expect("open");

        let question = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Medium,
            "open-question",
            "Is X true?",
        )
        .unwrap();
        let resolution = write_episode(
            &store,
            EpisodeKind::Decision,
            Significance::High,
            "resolution",
            "Yes, X holds because of …",
        )
        .unwrap();

        mark_resolved(&store, &question, &resolution).expect("mark_resolved");

        // From resolution we walk supersedes 1 hop and reach the question.
        let reached = walk(&store, &resolution, &[EdgeType::Supersedes], 1).unwrap();
        assert!(reached.contains(&question), "resolution → supersedes → question");

        // Audits: 2 write_episode + 1 mark_resolved.
        let audits = count_by_type(&store, "audit_event").expect("count audits");
        assert_eq!(audits, 3);
    }

    /// `mark_under_review` creates a high-significance review episode and
    /// attaches it to the target via a `contradicts` edge. The target itself
    /// is untouched.
    #[test]
    fn mark_under_review_creates_review_episode_and_contradicts_edge() {
        let store = Store::open_in_memory().expect("open");

        let target = write_episode(
            &store,
            EpisodeKind::Decision,
            Significance::Medium,
            "claim-x",
            "X is true under condition C.",
        )
        .unwrap();

        let review = mark_under_review(&store, &target, "Counter-example: when C is false, X fails.")
            .expect("mark_under_review");

        // From the review we should be able to walk contradicts 1 hop to the target.
        let reached = walk(&store, &review, &[EdgeType::Contradicts], 1).unwrap();
        assert!(reached.contains(&target), "review → contradicts → target");

        // Two episodes were written: the original target and the review.
        let episodes = count_by_type(&store, "episode").expect("count episodes");
        assert_eq!(episodes, 2);

        // Audits: 1 write_episode + 1 mark_under_review.
        let audits = count_by_type(&store, "audit_event").expect("count audits");
        assert_eq!(audits, 2);
    }

    /// `supersedes` retracts the old node, asserts a new one with new id,
    /// connects them with a `supersedes` edge, and writes one audit event.
    #[test]
    fn supersedes_creates_new_retracts_old_with_edge() {
        let store = Store::open_in_memory().expect("open");

        let old = write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "draft-v1",
            "first take",
        )
        .unwrap();

        // Whole-second validity resolution: the old assertion's timestamp
        // must strictly precede the retract / new-assert timestamps so the
        // bi-temporal substrate distinguishes them. 1100ms guarantees a
        // boundary crossing.
        std::thread::sleep(std::time::Duration::from_millis(1100));

        let new = supersedes(
            &store,
            &old,
            NodeType::Concept,
            Tier::Operational,
            "draft-v2",
            "second take",
        )
        .expect("supersedes");
        assert_ne!(new, old, "new id is fresh");

        // walk along supersedes edge: from old reaches new.
        let reached = walk(&store, &old, &[EdgeType::Supersedes], 1).expect("walk");
        assert!(reached.contains(&new), "supersedes edge connects old → new");

        // History of old has assertion + retraction.
        let hist_old = history(&store, &old).expect("history old");
        assert!(hist_old.iter().any(|r| r.asserted));
        assert!(hist_old.iter().any(|r| !r.asserted));
    }
}
