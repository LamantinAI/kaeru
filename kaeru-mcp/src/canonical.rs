//! MCP definitions and dispatch for the shared memory API.

use std::collections::BTreeMap;
use std::env::var_os;
use std::path::PathBuf;
use std::sync::Arc;

use kaeru_core::api::{
    ApiOptions, FailedStep, Issue, Response, ResponseData, State, authorize_arguments, edge_mirror,
    execute, finish_remote, normalize_arguments, output_schema_for, tool_definitions, validate,
};
use kaeru_core::{Store, Visibility, get_visibility};
use rmcp::ErrorData as McpError;
use rmcp::model::{CallToolResult, Tool, ToolAnnotations};
use serde::de::DeserializeOwned;
use serde_json::{Value, from_value, json, to_value};
use tokio::task::spawn_blocking;

use crate::cloud::{
    cloud_initiatives, cloud_links, cloud_recall, link_cloud, propagate_edge, pull, push_to_cloud,
    share, sync_review, unshare,
};
use crate::cloud_client::CloudRegistry;
use crate::models::{
    CloudLinksParams, CloudRecallParams, CloudScopeParams, LinkCloudParams, PullParams,
    ShareParams, SyncReviewParams, UnshareParams,
};
#[cfg(test)]
use serde_json::from_str;

pub(crate) const CLOUD_TOOLS: &[&str] = &[
    "clouds",
    "share",
    "unshare",
    "cloud_initiatives",
    "cloud_recall",
    "pull",
    "link_cloud",
    "cloud_links",
    "sync_review",
];

pub(crate) fn definitions() -> Vec<Tool> {
    tool_definitions()
        .iter()
        .map(|d| {
            let mut tool = Tool::new(
                d.name.clone(),
                d.description.clone(),
                Arc::new(d.input_schema.as_object().unwrap().clone()),
            );
            tool.annotations =
                Some(ToolAnnotations::default().read_only(d.annotations.read_only_hint));
            if !CLOUD_TOOLS.contains(&d.name.as_str()) {
                tool.output_schema = Some(Arc::new(
                    output_schema_for(&d.name).as_object().unwrap().clone(),
                ));
            }
            tool
        })
        .collect()
}

pub(crate) fn options() -> ApiOptions {
    ApiOptions {
        artifact_root: var_os("KAERU_MCP_ARTIFACT_ROOT").map(PathBuf::from),
        ..ApiOptions::local()
    }
}

pub(crate) async fn call(
    store: Arc<Store>,
    clouds: &CloudRegistry,
    name: &str,
    args: Value,
) -> Result<CallToolResult, McpError> {
    call_with_options(store, clouds, name, args, options()).await
}

pub(crate) async fn call_with_options(
    store: Arc<Store>,
    clouds: &CloudRegistry,
    name: &str,
    mut args: Value,
    opts: ApiOptions,
) -> Result<CallToolResult, McpError> {
    normalize_arguments(name, &mut args);
    if let Err(issue) = validate(name, &args) {
        return Ok(render(Response::rejected(name, issue)));
    }
    if let Err(issue) = authorize_arguments(name, &args, &opts) {
        return Ok(render(Response::rejected(name, issue)));
    }
    if CLOUD_TOOLS.contains(&name) {
        if opts.allowed_initiatives.is_some()
            && !(opts.include_local
                && opts
                    .allowed_initiatives
                    .as_ref()
                    .is_some_and(|s| s.iter().any(|s| s == "*"))
                && opts.denied_initiatives.is_empty())
        {
            return Ok(render(Response::rejected(
                name,
                Issue::new("NOT_FOUND", "cloud operation requires full operator scope"),
            )));
        }
        return cloud_call(&store, clouds, name, args).await;
    }
    let worker_opts = opts.clone();
    let worker_store = store.clone();
    let operation = name.to_string();
    let input = args.clone();
    let mut response =
        spawn_blocking(move || execute(&worker_store, &operation, input, &worker_opts))
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
    if !matches!(response.state, State::Rejected) && !response.replayed {
        if args.pointer("/sharing/visibility").and_then(Value::as_str) == Some("shared") {
            let id = match &response.data {
                ResponseData::Mutation(d) => d.primary.as_ref().map(|e| e.id.clone()),
                _ => None,
            };
            if let Some(id) = id {
                let cloud_name = args.pointer("/sharing/cloud").and_then(Value::as_str);
                let scope = args
                    .get("initiative")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .trim();
                let outcome = match clouds.resolve(cloud_name) {
                    Ok(cloud) => push_to_cloud(&store, cloud, &id, scope, false)
                        .await
                        .map_err(|e| e.to_string()),
                    Err(e) => Err(e),
                };
                let shared = get_visibility(&store, &id).is_ok_and(|v| v == Visibility::Shared);
                if let ResponseData::Mutation(data) = &mut response.data {
                    data.failed_steps.retain(|s| s.step != "share");
                    if !shared {
                        let message = match outcome {
                            Ok(msg) => msg,
                            Err(msg) => msg,
                        };
                        data.failed_steps.push(FailedStep {
                            step: "share".into(),
                            code: "REMOTE_FAILED".into(),
                            message: message.clone(),
                        });
                        response.issues.push(Issue::new("REMOTE_FAILED", message));
                    }
                }
            }
        }
        if let Some(edge) = edge_mirror(&response)
            && let Err(message) = propagate_edge(
                &store,
                clouds,
                args["cloud"].as_str(),
                &edge,
                args["initiative"].as_str().unwrap(),
            )
            .await
        {
            if let ResponseData::Mutation(data) = &mut response.data {
                data.failed_steps.push(FailedStep {
                    step: "mirror_edge".into(),
                    code: "REMOTE_FAILED".into(),
                    message: message.clone(),
                });
            }
            response.issues.push(Issue::new("REMOTE_FAILED", message));
        }

        if name == "initiative_manage" && args.get("cloud").is_some() {
            let outcome = match clouds.resolve(args["cloud"].as_str()) {
                Ok(cloud) => match args["action"].as_str() {
                    Some("rename") => {
                        cloud
                            .rename_initiative(
                                args["old"].as_str().unwrap(),
                                args["new"].as_str().unwrap(),
                            )
                            .await
                    }
                    Some("delete") => {
                        cloud
                            .delete_initiative(args["name"].as_str().unwrap())
                            .await
                    }
                    _ => Ok((
                        400,
                        "cloud operation is not supported for this action".into(),
                    )),
                },
                Err(e) => Err(e),
            };
            if let ResponseData::Mutation(data) = &mut response.data {
                data.failed_steps.retain(|s| s.step != "namespace_cloud");
                if let Err(message) = outcome.and_then(|(status, body)| {
                    if (200..300).contains(&status) {
                        Ok(())
                    } else {
                        Err(format!("cloud returned {status}: {body}"))
                    }
                }) {
                    data.failed_steps.push(FailedStep {
                        step: "namespace_cloud".into(),
                        code: "REMOTE_FAILED".into(),
                        message: message.clone(),
                    });
                    response.issues.push(Issue::new("REMOTE_FAILED", message));
                }
            }
        }
        if let ResponseData::Mutation(data) = &response.data
            && !matches!(response.state, State::Unchanged)
        {
            response.state = if data.failed_steps.is_empty() {
                State::Applied
            } else {
                State::Partial
            };
        }
        let worker = store.clone();
        let operation = name.to_string();
        let input = args;
        let record = response.clone();
        let options = opts;
        response =
            spawn_blocking(move || finish_remote(&worker, &operation, input, &options, &record))
                .await
                .map_err(|e| McpError::internal_error(e.to_string(), None))?;
    }
    Ok(render(response))
}

pub(crate) fn render(response: Response) -> CallToolResult {
    let is_error = response.is_error();
    let value = to_value(response).expect("typed API response serializes");
    let mut result = CallToolResult::structured(value);
    result.is_error = Some(is_error);
    result
}

async fn cloud_call(
    store: &Store,
    clouds: &CloudRegistry,
    name: &str,
    args: Value,
) -> Result<CallToolResult, McpError> {
    fn decode<T: DeserializeOwned>(args: Value) -> Result<T, McpError> {
        from_value(args).map_err(|e| McpError::invalid_params(e.to_string(), None))
    }
    let client = |name: Option<&str>| {
        clouds
            .resolve(name)
            .map_err(|e| McpError::invalid_params(e, None))
    };
    match name {
        "clouds" => Ok(CallToolResult::structured(
            json!({"clouds":clouds.names(),"endpoints":clouds.names().into_iter().filter_map(|name|clouds.resolve(Some(name)).ok().map(|client|(name,client.base_url().to_string()))).collect::<BTreeMap<_,_>>(),"loaded_at":clouds.loaded_at()}),
        )),
        "share" => {
            let p: ShareParams = decode(args)?;
            share(
                store,
                Some(client(p.cloud.as_deref())?),
                &p.name,
                &p.initiative,
                p.force,
            )
            .await
        }
        "unshare" => {
            let p: UnshareParams = decode(args)?;
            unshare(
                store,
                Some(client(p.cloud.as_deref())?),
                &p.name,
                &p.initiative,
            )
            .await
        }
        "cloud_initiatives" => {
            let p: CloudScopeParams = decode(args)?;
            cloud_initiatives(Some(client(p.cloud.as_deref())?)).await
        }
        "cloud_recall" => {
            let p: CloudRecallParams = decode(args)?;
            cloud_recall(
                Some(client(p.cloud.as_deref())?),
                &p.initiative,
                p.query.as_deref(),
                p.limit,
                p.offset,
            )
            .await
        }
        "pull" => {
            let p: PullParams = decode(args)?;
            pull(
                store,
                Some(client(p.cloud.as_deref())?),
                &p.id,
                &p.initiative,
            )
            .await
        }
        "link_cloud" => {
            let p: LinkCloudParams = decode(args)?;
            link_cloud(
                store,
                clouds,
                &p.name,
                &p.cloud_id,
                p.edge_type.as_deref().unwrap_or("refers_to"),
                p.cloud.as_deref(),
                &p.initiative,
            )
        }
        "cloud_links" => {
            let p: CloudLinksParams = decode(args)?;
            cloud_links(store, clouds, &p.name, &p.initiative).await
        }
        "sync_review" => {
            let p: SyncReviewParams = decode(args)?;
            sync_review(store, &p.initiative)
        }
        _ => Err(McpError::invalid_params("unknown memory operation", None)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_definitions_expose_exactly_35_tools() {
        let defs = definitions();
        assert_eq!(defs.len(), 35);
        assert!(!defs.iter().any(|d| matches!(
            d.name.as_ref(),
            "awake" | "why" | "evidence" | "slot" | "role"
        )));
        assert!(defs.iter().any(|d| d.name == "history"));
    }

    #[tokio::test]
    async fn canonical_call_returns_committed_state_in_both_content_channels() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let clouds = CloudRegistry::default();
        let args =
            json!({"initiative":"wire-test","body":"A real observation.","request_id":"wire-1"});
        let result = call(store.clone(), &clouds, "note", args.clone())
            .await
            .unwrap();
        assert_eq!(result.is_error, Some(false));
        let data = result.structured_content.clone().unwrap();
        assert_eq!(data["api_version"], 1);
        assert_eq!(data["replayed"], false);
        assert_eq!(
            from_str::<Value>(&result.content[0].as_text().unwrap().text).unwrap(),
            data
        );
        let replay = call(store, &clouds, "note", args).await.unwrap();
        assert_eq!(replay.structured_content.unwrap()["replayed"], true);
    }
}
