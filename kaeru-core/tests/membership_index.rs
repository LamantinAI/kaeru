//! Fresh, ordinary legacy and already-indexed vaults converge without rewriting history.
use std::fs;

use kaeru_core::{KaeruConfig, Store, history, improve, initiatives_of_node, jot, new_node_id};

fn has_index(store: &Store) -> bool {
    store
        .run_read("::indices node_initiative")
        .unwrap()
        .rows
        .iter()
        .any(|row| {
            row.first()
                .and_then(|v| v.get_str())
                .is_some_and(|name| name == "by_node" || name == "node_initiative:by_node")
        })
}
#[test]
fn fresh_schema_has_membership_reverse_index() {
    let store = Store::open_in_memory_with(KaeruConfig::default()).unwrap();
    assert!(has_index(&store));
}
#[test]
fn ordinary_existing_vaults_gain_the_index_and_already_indexed_vaults_reopen_safely() {
    for remove_index in [true, false] {
        let path = std::env::temp_dir().join(format!("kaeru-membership-index-{}", new_node_id()));
        let id;
        let before;
        {
            let store = Store::open(&path).unwrap();
            store.use_initiative("existing");
            id = jot(&store, "First version").unwrap();
            improve(&store, &id, "Second", "Second version").unwrap();
            before = history(&store, &id).unwrap();
            if remove_index {
                store.run("::index drop node_initiative:by_node").unwrap();
                assert!(!has_index(&store));
            }
            // 0011 is already applied and the table exists: this was the missed case.
            store
                .run("?[name] <- [['0013_node_initiative_by_node']] :rm migration_journal {name}")
                .unwrap();
            assert_eq!(
                store
                    .run_read(
                        "?[name] := *migration_journal{name},name='0011_scope_legacy_unscoped'"
                    )
                    .unwrap()
                    .rows
                    .len(),
                1
            );
        }
        for _ in 0..2 {
            let store = Store::open(&path).unwrap();
            assert!(has_index(&store));
            let after = history(&store, &id).unwrap();
            assert_eq!(
                after
                    .iter()
                    .map(|r| (&r.name, &r.body, r.seconds, r.asserted))
                    .collect::<Vec<_>>(),
                before
                    .iter()
                    .map(|r| (&r.name, &r.body, r.seconds, r.asserted))
                    .collect::<Vec<_>>()
            );
            assert_eq!(initiatives_of_node(&store, &id).unwrap(), vec!["existing"]);
            assert_eq!(
                store
                    .run_read(
                        "?[name] := *migration_journal{name},name='0013_node_initiative_by_node'"
                    )
                    .unwrap()
                    .rows
                    .len(),
                1
            );
        }
        fs::remove_dir_all(path).unwrap();
    }
}
