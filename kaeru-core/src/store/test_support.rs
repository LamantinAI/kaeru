//! Test-only capture of actual API queries; absent from production builds.
use std::cell::RefCell;
use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};

pub(crate) type ReadQuery = (String, BTreeMap<String, DataValue>);
thread_local! {static QUERIES:RefCell<Option<Vec<ReadQuery>>>=const{RefCell::new(None)};}
pub(super) fn record(
    script: &str,
    params: &BTreeMap<String, DataValue>,
    mutability: ScriptMutability,
) {
    if matches!(mutability, ScriptMutability::Immutable) {
        QUERIES.with_borrow_mut(|queries| {
            if let Some(queries) = queries {
                queries.push((script.to_string(), params.clone()));
            }
        });
    }
}
pub(crate) fn capture<T>(f: impl FnOnce() -> T) -> (T, Vec<ReadQuery>) {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            QUERIES.with_borrow_mut(|q| *q = None);
        }
    }
    QUERIES.with_borrow_mut(|q| {
        assert!(q.is_none());
        *q = Some(vec![]);
    });
    let _reset = Reset;
    let result = f();
    let queries = QUERIES.with_borrow_mut(|q| q.take().unwrap());
    (result, queries)
}
