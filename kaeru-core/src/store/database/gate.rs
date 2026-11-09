//! Reentrant operation leases; upgrading a shared lease is forbidden.
use crate::errors::{Error, Result};
use std::cell::RefCell;
use std::sync::RwLock;

thread_local! {static LEASES:RefCell<Vec<(usize,bool)>>=const {RefCell::new(Vec::new())};}
#[derive(Default)]
pub(super) struct Gate {
    lock: RwLock<()>,
}
struct Frame;
impl Drop for Frame {
    fn drop(&mut self) {
        LEASES.with_borrow_mut(|v| {
            v.pop();
        });
    }
}

impl Gate {
    pub fn exclusive(&self) -> bool {
        let key = self as *const Self as usize;
        LEASES.with_borrow(|v| v.iter().rfind(|v| v.0 == key).is_some_and(|v| v.1))
    }
    pub fn run<T>(&self, write: bool, f: impl FnOnce() -> Result<T>) -> Result<T> {
        let key = self as *const Self as usize;
        let current = LEASES.with_borrow(|v| v.iter().rfind(|v| v.0 == key).map(|v| v.1));
        if let Some(exclusive) = current {
            if write && !exclusive {
                return Err(Error::Invalid(
                    "write operation inside shared read lease".into(),
                ));
            }
            return f();
        }
        if write {
            let _lock = self.lock.write().unwrap_or_else(|p| p.into_inner());
            LEASES.with_borrow_mut(|v| v.push((key, true)));
            let _frame = Frame;
            f()
        } else {
            let _lock = self.lock.read().unwrap_or_else(|p| p.into_inner());
            LEASES.with_borrow_mut(|v| v.push((key, false)));
            let _frame = Frame;
            f()
        }
    }
}
