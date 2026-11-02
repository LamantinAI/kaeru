use super::*;

fn database() -> Database {
    let db = Database::new(DbInstance::new("mem", "", "").unwrap());
    db.run_script(
        ":create worker_test {id: Int}",
        BTreeMap::new(),
        ScriptMutability::Mutable,
    )
    .unwrap();
    db
}

fn insert(db: &Database, id: i64) -> Result<()> {
    db.atomic(|| {
        db.run_script(
            "?[id] <- [[$id]] :put worker_test {id}",
            BTreeMap::from([("id".into(), DataValue::from(id))]),
            ScriptMutability::Mutable,
        )
        .map(|_| ())
        .map_err(Error::from)
    })
}

fn id(db: &Database) -> ThreadId {
    db.worker.lock().unwrap().as_ref().unwrap().id()
}

#[test]
fn separate_transactions_reuse_one_worker_and_keep_independent_commits() {
    let db = database();
    insert(&db, 0).unwrap();
    let worker = id(&db);
    let initial = db.generation();
    for n in 1..=20 {
        insert(&db, n).unwrap();
        assert_eq!(id(&db), worker);
    }
    assert_eq!(db.generation(), initial + 20);
    let count = db
        .read_snapshot(|| {
            db.run_script(
                "?[count(id)] := *worker_test{id}",
                BTreeMap::new(),
                ScriptMutability::Immutable,
            )
            .map_err(Error::from)
        })
        .unwrap();
    assert_eq!(count.rows[0][0].get_int(), Some(21));
    assert_eq!(db.generation(), initial + 20, "a read changed generation");
}

#[test]
fn cross_database_nesting_uses_independent_workers_without_deadlocking() {
    let first = database();
    let second = database();
    let (completed, result) = bounded(1);
    let task = thread::spawn(move || {
        first
            .atomic(|| {
                insert(&first, 1)?;
                second.atomic(|| insert(&second, 2))?;
                assert_ne!(id(&first), id(&second));
                Ok(())
            })
            .unwrap();
        completed.send(()).unwrap();
    });
    result
        .recv_timeout(std::time::Duration::from_secs(5))
        .unwrap();
    task.join().unwrap();
}

#[test]
fn rollback_read_only_rejection_and_panic_preserve_worker_and_generation() {
    let db = database();
    insert(&db, 0).unwrap();
    let generation = db.generation();
    let worker = id(&db);
    let result: Result<()> = db.atomic(|| {
        insert(&db, 1)?;
        Err(Error::Invalid("rollback".into()))
    });
    assert!(result.is_err());
    assert_eq!(db.generation(), generation);
    assert!(db.read_snapshot(|| insert(&db, 2)).is_err());
    assert_eq!(db.generation(), generation);
    let panic = catch_unwind(AssertUnwindSafe(|| {
        let _ = db.atomic::<()>(|| {
            insert(&db, 3)?;
            panic!("abort caller");
        });
    }));
    assert!(panic.is_err());
    assert_eq!(db.generation(), generation);
    insert(&db, 4).unwrap();
    assert_eq!(id(&db), worker);
    let rows = db
        .run_script(
            "?[id] := *worker_test{id}",
            BTreeMap::new(),
            ScriptMutability::Immutable,
        )
        .unwrap();
    assert_eq!(
        rows.rows
            .iter()
            .map(|r| r[0].get_int().unwrap())
            .collect::<Vec<_>>(),
        vec![0, 4]
    );
}

#[test]
fn a_stopped_worker_is_replaced_for_the_next_transaction_without_replaying_prior_writes() {
    let db = database();
    insert(&db, 0).unwrap();
    let old = id(&db);
    let generation = db.generation();
    db.worker.lock().unwrap().as_mut().unwrap().shutdown();
    insert(&db, 1).unwrap();
    assert_ne!(id(&db), old);
    assert_eq!(db.generation(), generation + 1);
    let count = db
        .run_script(
            "?[count(id)] := *worker_test{id}",
            BTreeMap::new(),
            ScriptMutability::Immutable,
        )
        .unwrap();
    assert_eq!(count.rows[0][0].get_int(), Some(2));
}

#[test]
fn dropping_store_joins_worker_and_releases_the_disk_vault_immediately() {
    let directory = tempfile::tempdir().unwrap();
    {
        let store = crate::Store::open(directory.path()).unwrap();
        store.run(":create worker_test {id: Int}").unwrap();
        insert(store.db_ref(), 1).unwrap();
    }
    let store = crate::Store::open(directory.path()).unwrap();
    assert_eq!(
        store.run_read("?[id] := *worker_test{id}").unwrap().rows[0][0].get_int(),
        Some(1)
    );
}
