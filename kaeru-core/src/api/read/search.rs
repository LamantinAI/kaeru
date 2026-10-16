//! Bounded retrieval with currentness and stored reasoning on the ordinary path.

use std::collections::BTreeSet;

use serde_json::Value;

use super::{node, scope_nodes};
use crate::Store;
use crate::api::access::check_scope;
use crate::api::{ApiOptions, ApiResult, Args, issue_from_error};
use crate::config::api::API_SEARCH_CANDIDATE_LIMIT;
use crate::models::{Issue, NodeData, Omission, Response, ResponseData, SearchData};
use crate::recall::literal_ids;

pub(in crate::api) fn search(
    store: &Store,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    let scopes: Vec<String> = args.parse("initiatives")?;
    for scope in &scopes {
        check_scope(scope, options)?;
    }
    let query = args.text("text").map(|text| {
        let prefix = args.value("prefix").as_bool().unwrap_or(false);
        let tokens: Vec<_> = text
            .split(|c: char| !c.is_alphanumeric())
            .filter(|t| !t.is_empty())
            .map(|s| {
                if prefix {
                    format!("{}*", s.to_lowercase())
                } else {
                    format!("\"{}\"", s.to_lowercase())
                }
            })
            .collect();
        tokens.join(if args.text("match").as_deref() == Some("all") {
            " AND "
        } else {
            " OR "
        })
    });
    if query.as_ref().is_some_and(String::is_empty) {
        return Err(Issue::new(
            "INVALID_ARGUMENT",
            "text must contain a lexical term",
        ));
    }
    let mut seen = BTreeSet::new();
    let mut hits = vec![];
    let mut truncated = false;
    for scope in &scopes {
        store.use_initiative(scope);
        let nodes = if let Some(query) = &query {
            let candidates =
                literal_ids(store, query, API_SEARCH_CANDIDATE_LIMIT).map_err(issue_from_error)?;
            truncated |= candidates.len() >= API_SEARCH_CANDIDATE_LIMIT;
            candidates
                .into_iter()
                .filter_map(|n| node(store, &n, options, false).ok())
                .collect()
        } else {
            scope_nodes(store, scope, options)?
        };
        for n in nodes {
            if !matches_filters(&n, args.value("filters")) || !seen.insert(n.id.clone()) {
                continue;
            }
            // A stale lexical match carries its accessible current successor too.
            if n.currentness == "superseded"
                && args.value("filters").get("current").and_then(Value::as_str) != Some("all")
            {
                for successor in &n.successors {
                    if seen.insert(successor.clone())
                        && let Ok(s) = node(store, successor, options, false)
                    {
                        hits.push(s);
                    }
                }
            }
            hits.push(n);
        }
    }
    store.clear_initiative();
    let total = hits.len();
    let mut response = Response::read(
        "search",
        ResponseData::Search(SearchData { hits, total }),
        scopes,
    );
    if truncated {
        let c = response.coverage.as_mut().unwrap();
        c.complete = false;
        c.omitted.push(Omission {
            section: "candidate_scan".into(),
            count: 1,
            reason: format!("{API_SEARCH_CANDIDATE_LIMIT}_candidate_cap"),
        });
    }
    Ok(response)
}

pub(in crate::api) fn matches_filters(n: &NodeData, filters: &Value) -> bool {
    for (key, value) in [
        ("types", n.node_type.as_str()),
        ("tiers", n.tier.as_str()),
        ("layers", n.layer.as_str()),
        ("statuses", n.status.as_deref().unwrap_or("")),
    ] {
        if let Some(a) = filters.get(key).and_then(Value::as_array)
            && !a.iter().any(|v| v.as_str() == Some(value))
        {
            return false;
        }
    }
    if let Some(a) = filters.get("tags").and_then(Value::as_array)
        && !a
            .iter()
            .any(|v| v.as_str().is_some_and(|v| n.tags.iter().any(|t| t == v)))
    {
        return false;
    }
    true
}
