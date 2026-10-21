//! Traversal only follows endpoints admitted by the caller's current access policy.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque};

use cozo::{DataValue, ScriptMutability};

use super::access::check_node;
use super::read::relations;
use super::{ApiOptions, ApiResult, Issue, issue_from_error};
use crate::api::control::checkpoint;
use crate::config::api::API_TRAVERSAL_NODE_LIMIT;
use crate::{Error, Store};

pub(super) fn provenance(
    store: &Store,
    start: &str,
    options: &ApiOptions,
) -> ApiResult<(Vec<String>, bool)> {
    let mut truncated = false;
    let mut queue = VecDeque::from([(start.to_string(), 0usize)]);
    let mut seen = BTreeSet::from([start.to_string()]);
    let mut result = vec![];
    while let Some((id, depth)) = queue.pop_front() {
        checkpoint()?;
        if depth >= usize::from(store.config().provenance_max_hops) {
            truncated |= relations(store, &id, options)?.iter().any(|e| {
                e.from == id
                    && e.edge_type == "derived_from"
                    && e.dst_store == "local"
                    && !seen.contains(&e.to)
            });
            continue;
        }
        for edge in relations(store, &id, options)? {
            if edge.from != id
                || edge.edge_type != "derived_from"
                || edge.dst_store != "local"
                || !seen.insert(edge.to.clone())
            {
                continue;
            }
            check_node(store, &edge.to, None, options, false)?;
            result.push(edge.to.clone());
            queue.push_back((edge.to, depth + 1));
        }
        if seen.len() > API_TRAVERSAL_NODE_LIMIT {
            return Err(Issue::new("TRAVERSAL_LIMIT", "narrow this provenance view"));
        }
    }
    Ok((result, truncated))
}

#[derive(Clone)]
struct Route {
    cost: f64,
    path: Vec<String>,
}
impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.cost.to_bits() == other.cost.to_bits() && self.path == other.path
    }
}
impl Eq for Route {}
impl Ord for Route {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .total_cmp(&self.cost)
            .then_with(|| other.path.len().cmp(&self.path.len()))
            .then_with(|| other.path.cmp(&self.path))
    }
}
impl PartialOrd for Route {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub(super) fn path(
    store: &Store,
    start: &str,
    end: &str,
    options: &ApiOptions,
) -> ApiResult<Vec<String>> {
    let mut queue = BinaryHeap::from([Route {
        cost: 0.0,
        path: vec![start.into()],
    }]);
    let mut best = BTreeMap::new();
    let mut visited = 0;
    while let Some(route) = queue.pop() {
        checkpoint()?;
        let current = route.path.last().unwrap();
        if current == end {
            return Ok(route.path);
        }
        if route.path.len() > store.config().chain_max_hops {
            continue;
        }
        let key = (current.clone(), route.path.len());
        if best.get(&key).is_some_and(|cost: &f64| *cost < route.cost) {
            continue;
        }
        best.insert(key, route.cost);
        visited += 1;
        if visited > API_TRAVERSAL_NODE_LIMIT {
            return Err(Issue::new("TRAVERSAL_LIMIT", "narrow this graph route"));
        }
        for edge in relations(store, current, options)? {
            if edge.from != *current
                || edge.dst_store != "local"
                || route.path.contains(&edge.to)
                || !edge.weight.is_finite()
            {
                continue;
            }
            check_node(store, &edge.to, None, options, false)?;
            let mut path = route.path.clone();
            path.push(edge.to);
            queue.push(Route {
                cost: route.cost + 1.0 - edge.weight.clamp(0.0, 1.0),
                path,
            });
        }
    }
    Ok(vec![])
}

/// Supersession acyclicity is global and independent of the recall hop cap.
/// This probe returns no inaccessible identity or content.
pub(super) fn supersession_reaches(store: &Store, start: &str, end: &str) -> ApiResult<bool> {
    let mut queue = VecDeque::from([start.to_string()]);
    let mut seen = BTreeSet::new();
    while let Some(id) = queue.pop_front() {
        checkpoint()?;
        if id == end {
            return Ok(true);
        }
        if !seen.insert(id.clone()) {
            continue;
        }
        if seen.len() > API_TRAVERSAL_NODE_LIMIT {
            return Err(Issue::new(
                "TRAVERSAL_LIMIT",
                "cannot establish supersession acyclicity",
            ));
        }
        let rows=store.db_ref().run_script("?[dst] := *edge{src,dst,edge_type,dst_store @ 'NOW'},src=$id,edge_type='supersedes',dst_store='local'",BTreeMap::from([("id".into(),DataValue::Str(id.into()))]),ScriptMutability::Immutable).map_err(Error::from).map_err(issue_from_error)?;
        for row in rows.rows {
            if let Some(dst) = row.first().and_then(DataValue::get_str) {
                queue.push_back(dst.into());
            }
        }
    }
    Ok(false)
}
