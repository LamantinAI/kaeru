//! Bounded retrieval with currentness and stored reasoning on the ordinary path.

use serde_json::{Value, json};

use crate::api::access::check_scope;
use crate::api::types::{ContextData, Response, ResponseData, Section};
use crate::api::{ApiOptions, ApiResult, Args, issue_from_error};
use crate::guard::scan_public;
use crate::{Store, get_share_policy, list_initiatives};

use super::{context, relations, scope_nodes, trails};

pub(in crate::api) fn reflect(
    store: &Store,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Response> {
    let scope = args.required_text("initiative")?;
    let include: Vec<String> = args.optional("include")?.unwrap_or_default();
    let wants = |section: &str| include.is_empty() || include.iter().any(|s| s == section);
    let nodes = scope_nodes(store, &scope, options)?;
    let mut response = if wants("work") {
        context(store, args, options)?
    } else {
        Response::read(
            "reflect",
            ResponseData::Context(ContextData {
                proposals: vec![],
                sections: vec![],
                slots: vec![],
            }),
            vec![scope.clone()],
        )
    };
    response.operation = "reflect".into();
    if let ResponseData::Context(data) = &mut response.data {
        if wants("integrity") {
            let mut orphans = vec![];
            let mut conflicts = vec![];
            for n in &nodes {
                let edges = relations(store, &n.id, options)?;
                if edges.is_empty() && !matches!(n.node_type.as_str(), "audit_event" | "board") {
                    orphans.push(n.clone());
                }
                if n.successors.len() > 1 {
                    conflicts.push(n.clone());
                }
            }
            data.sections.push(Section{name:"integrity".into(),records:orphans,details:json!({"supersession_conflicts":conflicts.iter().map(|n|&n.id).collect::<Vec<_>>(),"scope":"accessible_graph","dangling_edge_validation":"run owner diagnostics for substrate-wide checks"})});
        }
        if wants("staleness") {
            let mut stale = vec![];
            for n in &nodes {
                if n.currentness == "superseded"
                    || trails(store, &n.id, options, false)?
                        .iter()
                        .any(|t| t.stale == Some(true))
                {
                    stale.push(n.clone());
                }
            }
            data.sections.push(Section {
                name: "staleness".into(),
                records: stale,
                details: Value::Null,
            });
        }
        if wants("scope") {
            let initiatives = list_initiatives(store)
                .map_err(issue_from_error)?
                .into_iter()
                .filter(|s| check_scope(s, options).is_ok())
                .collect::<Vec<_>>();
            let normalized = |name: &str| {
                name.to_lowercase()
                    .chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
            };
            let similar = initiatives
                .iter()
                .filter(|s| *s != &scope && normalized(s) == normalized(&scope))
                .collect::<Vec<_>>();
            data.sections.push(Section{name:"scope".into(),records:vec![],details:json!({"initiative":scope,"permitted_initiatives":initiatives,"possible_duplicates":similar})});
        }
        if wants("sharing") {
            let permitted = get_share_policy(store, &scope)
                .map_err(issue_from_error)?
                .permits_share();
            let candidates = if permitted {
                nodes
                    .iter()
                    .filter(|n| {
                        n.visibility == "local"
                            && !matches!(n.node_type.as_str(), "audit_event" | "board")
                    })
                    .filter(|n| {
                        scan_public(&format!("{}\n{}", n.name, n.body.as_deref().unwrap_or("")))
                            .is_empty()
                    })
                    .cloned()
                    .collect()
            } else {
                vec![]
            };
            data.sections.push(Section{name:"sharing".into(),records:candidates,details:json!({"policy_permits_share":permitted,"proposal_only":true,"full_content_guard_required_on_share":true})});
        }
    }
    Ok(response)
}
