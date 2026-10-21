use std::collections::BTreeMap;

use cozo::{DataValue, ScriptMutability};
use uuid::Uuid;

use super::read::session::{check as cached_check, memberships, visibility};
use super::{ApiOptions, ApiResult, Issue, NodeRef, issue_from_error};
use crate::api::control::checkpoint;
use crate::export_json::pat_match;
use crate::{Error, Store};

pub(super) fn check_scope(scope: &str, options: &ApiOptions) -> ApiResult<()> {
    if scope.trim().is_empty() {
        return Err(
            Issue::new("INVALID_ARGUMENT", "initiative must not be blank").field("initiative"),
        );
    }
    if options
        .allowed_initiatives
        .as_ref()
        .is_some_and(|p| !p.iter().any(|p| pat_match(scope, p)))
        || options
            .denied_initiatives
            .iter()
            .any(|p| pat_match(scope, p))
    {
        return Err(Issue::new("NOT_FOUND", "initiative is not accessible"));
    }
    Ok(())
}

pub(super) fn check_node(
    store: &Store,
    id: &str,
    scope: Option<&str>,
    options: &ApiOptions,
    historical: bool,
) -> ApiResult<()> {
    checkpoint()?;
    cached_check(store, id, scope, options, historical, || {
        check_node_inner(store, id, scope, options, historical)
    })
}

fn check_node_inner(
    store: &Store,
    id: &str,
    scope: Option<&str>,
    options: &ApiOptions,
    historical: bool,
) -> ApiResult<()> {
    let memberships = memberships(store, id).map_err(issue_from_error)?;
    if let Some(scope) = scope {
        check_scope(scope, options)?;
        let recovered_retired = historical
            && memberships.is_empty()
            && options.include_local
            && options.allowed_initiatives.is_none()
            && visibility(store, id, false)?.is_none();
        if !memberships.iter().any(|m| m == scope) && !recovered_retired {
            return Err(Issue::new(
                "NOT_FOUND",
                "node is not accessible in this initiative",
            ));
        }
    } else if options.allowed_initiatives.is_some()
        && !memberships.iter().any(|m| check_scope(m, options).is_ok())
    {
        return Err(Issue::new("NOT_FOUND", "node is not accessible"));
    }
    if let Some(visibility) = visibility(store, id, false)? {
        if !options.include_local && visibility != "shared" {
            return Err(Issue::new("NOT_FOUND", "node is not accessible"));
        }
    } else if historical {
        let visibility = visibility(store, id, true)?
            .ok_or_else(|| Issue::new("NOT_FOUND", "node does not exist"))?;
        if !options.include_local && visibility != "shared" {
            return Err(Issue::new("NOT_FOUND", "node is not accessible"));
        }
    } else {
        return Err(Issue::new(
            "RETIRED_TARGET",
            "node is not live; inspect its history",
        ));
    }
    Ok(())
}

pub(super) fn resolve(
    store: &Store,
    node: &NodeRef,
    options: &ApiOptions,
    when: Option<f64>,
    allow_retired: bool,
    foreign: bool,
) -> ApiResult<String> {
    let active = store.current_initiative();
    let (id, name, scope) = match node {
        NodeRef::Id(n) => (
            Some(n.id.clone()),
            None,
            if foreign { None } else { active.clone() },
        ),
        NodeRef::NameOrId(n) if Uuid::parse_str(n).is_ok() => (
            Some(n.clone()),
            None,
            if foreign { None } else { active.clone() },
        ),
        NodeRef::NameOrId(n)
            if check_node(
                store,
                n,
                if foreign { None } else { active.as_deref() },
                options,
                when.is_some() || allow_retired,
            )
            .is_ok() =>
        {
            (
                Some(n.clone()),
                None,
                if foreign { None } else { active.clone() },
            )
        }
        NodeRef::NameOrId(n) => (None, Some(n.clone()), active.clone()),
        NodeRef::Scoped(n) => {
            if !foreign && active.as_deref() != Some(n.initiative.trim()) {
                return Err(Issue::new(
                    "INVALID_ARGUMENT",
                    "primary node must be in the call initiative",
                ));
            }
            (None, Some(n.name.clone()), Some(n.initiative.trim().into()))
        }
    };
    if let Some(id) = id {
        check_node(
            store,
            &id,
            scope.as_deref(),
            options,
            when.is_some() || allow_retired,
        )?;
        return Ok(id);
    }
    let mut params = BTreeMap::from([("name".into(), DataValue::Str(name.unwrap().into()))]);
    let scope_clause = if let Some(scope) = scope.as_deref() {
        check_scope(scope, options)?;
        params.insert("scope".into(), DataValue::Str(scope.into()));
        ", initiative=$scope, *node_initiative{initiative, node_id:id}"
    } else {
        ""
    };
    let at = when
        .map(|n| format!(" @ {}", n.floor()))
        .unwrap_or_else(|| " @ 'NOW'".into());
    let script = format!("?[id] := name=$name, *node{{id, name{at}}}{scope_clause}");
    let mut rows = store
        .db_ref()
        .run_script(&script, params.clone(), ScriptMutability::Immutable)
        .map_err(Error::from)
        .map_err(issue_from_error)?;
    if rows.rows.is_empty() && allow_retired && when.is_none() {
        let script = format!("?[id] := name=$name, *node{{id, name}}{scope_clause}");
        rows = store
            .db_ref()
            .run_script(&script, params, ScriptMutability::Immutable)
            .map_err(Error::from)
            .map_err(issue_from_error)?;
    }
    let candidates: Vec<String> = rows
        .rows
        .iter()
        .filter_map(|r| r.first()?.get_str().map(str::to_string))
        .filter(|id| {
            check_node(
                store,
                id,
                scope.as_deref(),
                options,
                when.is_some() || allow_retired,
            )
            .is_ok()
        })
        .collect();
    match candidates.as_slice() {
        [id] => Ok(id.clone()),
        [] => Err(Issue::new(
            if when.is_some() {
                "NOT_FOUND_AT_TIME"
            } else {
                "NOT_FOUND"
            },
            "no accessible node matches that reference",
        )),
        _ => Err(Issue::new(
            "AMBIGUOUS_NODE",
            format!(
                "use a returned node ID; candidates: {}",
                candidates.join(", ")
            ),
        )),
    }
}
