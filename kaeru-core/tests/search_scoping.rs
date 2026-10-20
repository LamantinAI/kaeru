use kaeru_core::{
    EpisodeKind, Significance, Store,
    api::{self, ApiOptions, ResponseData},
    config::api::API_SEARCH_CANDIDATE_LIMIT,
    fuzzy_recall, write_episode,
};
use serde_json::json;

fn competing_initiatives(strong_matches: usize) -> (Store, String) {
    let store = Store::open_in_memory().unwrap();
    store.use_initiative("busy");
    store
        .atomic(|store| {
            for i in 0..strong_matches {
                write_episode(
                    store,
                    EpisodeKind::Observation,
                    Significance::Low,
                    &format!("alpha-note-{i}"),
                    "alpha alpha alpha alpha alpha",
                )?;
            }
            Ok(())
        })
        .unwrap();
    store.use_initiative("quiet");
    let expected = write_episode(
        &store,
        EpisodeKind::Observation,
        Significance::Low,
        "quiet-design",
        "A long design note mentions alpha once among many words about storage layers edges and metadata.",
    )
    .unwrap();
    (store, expected)
}

#[test]
fn scoped_fuzzy_recall_finds_a_weaker_match_beyond_the_output_limit() {
    let (store, expected) = competing_initiatives(8);
    let hits = fuzzy_recall(&store, "alpha", 3).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].id, expected);
}

#[test]
fn scoped_search_finds_a_weaker_match_beyond_the_global_candidate_limit() {
    let (store, expected) = competing_initiatives(API_SEARCH_CANDIDATE_LIMIT + 1);
    let response = api::execute(
        &store,
        "search",
        json!({"initiatives":["quiet"],"text":"alpha","limit":3}),
        &ApiOptions::local(),
    );
    assert!(!response.is_error(), "{response:?}");
    let ResponseData::Search(data) = response.data else {
        panic!("expected search data");
    };
    assert_eq!(data.hits.len(), 1);
    assert_eq!(data.hits[0].id, expected);
    assert!(response.coverage.unwrap().complete);
}
