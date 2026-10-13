use std::panic::{AssertUnwindSafe, catch_unwind};

use kaeru_core::{Error, Store, fuzzy_recall, jot, list_initiatives, nodes_in_initiative};

#[test]
fn a_failed_command_rolls_back_content_membership_audit_and_fts() {
    let store = Store::open_in_memory().unwrap();
    store.scoped(Some("transaction-test"), |store| {
        let result: Result<(), Error> = store.atomic(|store| {
            let id = jot(store, "quasar rollback probe")?;
            assert!(kaeru_core::read_node_full(store, &id)?.is_some());
            assert_eq!(fuzzy_recall(store, "quasar", 10)?.len(), 1);
            Err(Error::Invalid("injected command failure".into()))
        });
        assert!(result.is_err());
        assert!(
            nodes_in_initiative(store, "transaction-test")
                .unwrap()
                .is_empty()
        );
        assert!(list_initiatives(store).unwrap().is_empty());
        assert!(fuzzy_recall(store, "quasar", 10).unwrap().is_empty());
        let count = store.run_read("?[count(id)] := *node{id}").unwrap();
        assert_eq!(count.rows[0][0].get_int(), Some(0));
    });
}

#[test]
fn a_panic_aborts_and_does_not_poison_subsequent_commands() {
    let store = Store::open_in_memory().unwrap();
    let panicked = catch_unwind(AssertUnwindSafe(|| {
        let _: Result<(), Error> = store.atomic(|store| {
            jot(store, "must disappear")?;
            panic!("injected panic");
        });
    }));
    assert!(panicked.is_err());
    let id = store
        .atomic(|store| jot(store, "survives committed command"))
        .unwrap();
    assert!(kaeru_core::read_node_full(&store, &id).unwrap().is_some());
    let count = store
        .run_read("?[count(id)] := *node{id}, type = 'episode', *node{id, type @ 'NOW'}")
        .unwrap();
    assert_eq!(count.rows[0][0].get_int(), Some(1));
}

#[test]
fn swallowing_a_nested_failure_cannot_commit_partial_work() {
    let store = Store::open_in_memory().unwrap();
    let result = store.atomic(|store| {
        jot(store, "outer draft")?;
        let nested: Result<(), Error> = store.atomic(|store| {
            jot(store, "inner draft")?;
            Err(Error::Invalid("failed inner command".into()))
        });
        assert!(nested.is_err());
        Ok(())
    });
    assert!(result.is_err());
    let count = store.run_read("?[count(id)] := *node{id}").unwrap();
    assert_eq!(count.rows[0][0].get_int(), Some(0));
}
