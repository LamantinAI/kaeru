//! Independent recorded-state oracle: no database or production-code imports.

use std::collections::BTreeMap;

pub struct RecordedReference<K, V> {
    rows: BTreeMap<(K, u64, bool), V>,
}

impl<K: Ord + Clone, V> RecordedReference<K, V> {
    pub fn new() -> Self {
        Self {
            rows: BTreeMap::new(),
        }
    }

    pub fn put(&mut self, key: K, seconds: u64, asserted: bool, value: V) {
        self.rows.insert((key, seconds, asserted), value);
    }

    pub fn at(&self, key: &K, seconds: u64) -> Option<&V> {
        let ((_, _, asserted), value) = self
            .rows
            .iter()
            .filter(|((candidate, time, _), _)| candidate == key && *time <= seconds)
            .max_by_key(|((_, time, asserted), _)| (*time, *asserted))?;
        asserted.then_some(value)
    }
}
