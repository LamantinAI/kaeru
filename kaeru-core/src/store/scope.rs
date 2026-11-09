//! Synchronous request scopes never mutate another caller's default initiative.

use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use super::Store;

struct Frame {
    store: usize,
    initiative: Option<String>,
}
thread_local! {static SCOPES:RefCell<Vec<Frame>>=const {RefCell::new(Vec::new())};}

pub(super) struct Guard<'a> {
    _store: &'a Store,
    _thread_bound: PhantomData<Rc<()>>,
}
impl Drop for Guard<'_> {
    fn drop(&mut self) {
        SCOPES.with_borrow_mut(|frames| {
            frames.pop();
        });
    }
}

pub(super) fn enter<'a>(store: &'a Store, initiative: Option<&str>) -> Guard<'a> {
    SCOPES.with_borrow_mut(|frames| {
        frames.push(Frame {
            store: store as *const Store as usize,
            initiative: initiative.map(|v| v.trim().to_owned()),
        })
    });
    Guard {
        _store: store,
        _thread_bound: PhantomData,
    }
}
pub(super) fn current(store: &Store) -> Option<Option<String>> {
    SCOPES.with_borrow(|frames| {
        frames
            .iter()
            .rfind(|f| f.store == store as *const Store as usize)
            .map(|f| f.initiative.clone())
    })
}
pub(super) fn set(store: &Store, value: Option<String>) -> bool {
    SCOPES.with_borrow_mut(|frames| {
        if let Some(frame) = frames
            .iter_mut()
            .rfind(|f| f.store == store as *const Store as usize)
        {
            frame.initiative = value;
            true
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::Store;
    use crate::api::{ApiOptions, execute};
    use crate::{Layer, NodeType, Tier, Visibility, upsert_node};
    use serde_json::json;

    #[test]
    fn same_named_records_never_cross_request_scope_or_ceiling() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        for (scope, id) in [("alpha", "alpha-record"), ("beta", "beta-record")] {
            upsert_node(
                &store,
                &id.into(),
                NodeType::Episode,
                Tier::Operational,
                "same-name",
                Some(scope),
                &[],
                Some(scope),
                Visibility::Shared,
                Layer::Warm,
                None,
            )
            .unwrap();
        }
        let tasks = ["alpha", "beta"].map(|scope| {
            let store = store.clone();
            thread::spawn(move || {
                let options = ApiOptions {
                    allowed_initiatives: Some(vec![scope.into()]),
                    include_local: false,
                    ..ApiOptions::local()
                };
                for _ in 0..20 {
                    let response = execute(
                        &store,
                        "read",
                        json!({"initiative":scope,"node":"same-name","budget_chars":64000}),
                        &options,
                    );
                    assert!(!response.is_error(), "{response:?}");
                    let value = serde_json::to_value(response).unwrap();
                    assert_eq!(
                        value.pointer("/data/node/body").and_then(|v| v.as_str()),
                        Some(scope)
                    );
                    let foreign = if scope == "alpha" {
                        "beta-record"
                    } else {
                        "alpha-record"
                    };
                    let denied = execute(
                        &store,
                        "read",
                        json!({"initiative":scope,"node":{"id":foreign}}),
                        &options,
                    );
                    assert!(denied.is_error());
                }
            })
        });
        for task in tasks {
            task.join().unwrap();
        }
    }
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::sync::{Arc, mpsc::channel};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn concurrent_scopes_overlap_without_changing_the_default() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        store.use_initiative("default");
        let (ready, events) = channel();
        let mut release = Vec::new();
        let mut threads = Vec::new();
        for name in ["first", "second"] {
            let store = store.clone();
            let ready = ready.clone();
            let (go, wait) = channel();
            release.push(go);
            threads.push(thread::spawn(move || {
                store.scoped(Some(name), |store| {
                    ready.send(()).unwrap();
                    wait.recv_timeout(Duration::from_secs(5)).unwrap();
                    assert_eq!(store.current_initiative().as_deref(), Some(name));
                    store.clear_initiative();
                    assert_eq!(store.current_initiative(), None);
                    store.use_initiative(name);
                    assert_eq!(store.current_initiative().as_deref(), Some(name));
                })
            }));
        }
        events.recv_timeout(Duration::from_secs(5)).unwrap();
        events.recv_timeout(Duration::from_secs(5)).unwrap();
        assert_eq!(store.current_initiative().as_deref(), Some("default"));
        for go in release {
            go.send(()).unwrap();
        }
        for task in threads {
            task.join().unwrap();
        }
        assert_eq!(store.current_initiative().as_deref(), Some("default"));
    }

    #[test]
    fn nested_none_cross_store_and_unwind_restore_the_outer_frame() {
        let first = Store::open_in_memory().unwrap();
        let second = Store::open_in_memory().unwrap();
        first.use_initiative("default-a");
        second.use_initiative("default-b");
        first.scoped(Some(" outer "), |first| {
            first.scoped(None, |first| assert_eq!(first.current_initiative(), None));
            second.scoped(Some("other"), |second| {
                assert_eq!(first.current_initiative().as_deref(), Some("outer"));
                assert_eq!(second.current_initiative().as_deref(), Some("other"));
                first.scoped(Some("nested"), |first| {
                    assert_eq!(first.current_initiative().as_deref(), Some("nested"))
                });
            });
            let panic = catch_unwind(AssertUnwindSafe(|| {
                first.scoped(Some("panic"), |_| panic!("scope unwind"))
            }));
            assert!(panic.is_err());
            assert_eq!(first.current_initiative().as_deref(), Some("outer"));
        });
        assert_eq!(first.current_initiative().as_deref(), Some("default-a"));
        assert_eq!(second.current_initiative().as_deref(), Some("default-b"));
    }
}
