//! Canonical tools share their schemas and local command implementation with MCP.

use std::convert::Infallible;
use std::path::PathBuf;

use kaeru_core::api::{
    ExecutionControl, FailedStep, Issue, Response, ResponseData, State, edge_mirror,
    execute_with_control, finish_remote, normalize_arguments, tool_definitions, validate,
};
use kaeru_core::{Visibility, get_visibility};
use rig::agent::{AgentBuilder, NoToolConfig, WithBuilderTools};
use rig::completion::{CompletionModel, ToolDefinition};
use rig::tool::Tool;
use serde_json::{Value, json, to_value};
use tokio::task::spawn_blocking;
use tokio_util::sync::CancellationToken;

use crate::KaeruMemory;
use crate::cloud::{call as core_call, propagate_edge};

const NAMES: [&str; 35] = [
    "kaeru_context",
    "kaeru_search",
    "kaeru_read",
    "kaeru_note",
    "kaeru_reference",
    "kaeru_task",
    "kaeru_task_update",
    "kaeru_claim",
    "kaeru_verdict",
    "kaeru_revise",
    "kaeru_replace",
    "kaeru_relate",
    "kaeru_review",
    "kaeru_trail",
    "kaeru_pin",
    "kaeru_reflect",
    "kaeru_lifecycle",
    "kaeru_synthesise",
    "kaeru_forget",
    "kaeru_history",
    "kaeru_retention",
    "kaeru_initiatives",
    "kaeru_initiative_manage",
    "kaeru_board_config",
    "kaeru_policy",
    "kaeru_export",
    "kaeru_clouds",
    "kaeru_share",
    "kaeru_unshare",
    "kaeru_cloud_initiatives",
    "kaeru_cloud_recall",
    "kaeru_pull",
    "kaeru_link_cloud",
    "kaeru_cloud_links",
    "kaeru_sync_review",
];

/// A distinct static tool type for each canonical operation. The const index
/// only selects the fixed catalogue; models never receive an execute tunnel.
#[derive(Clone)]
pub struct MemoryTool<const I: usize>(KaeruMemory);

impl<const I: usize> Tool for MemoryTool<I> {
    const NAME: &'static str = NAMES[I];
    type Args = Value;
    type Output = Value;
    type Error = Infallible;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let d = &tool_definitions()[I];
        ToolDefinition {
            name: Self::NAME.into(),
            description: d.description.clone(),
            parameters: d.input_schema.clone(),
        }
    }

    async fn call(&self, mut args: Value) -> Result<Value, Infallible> {
        let definition = &tool_definitions()[I];
        let name = definition.name.clone();
        if !args.is_object() {
            return Ok(to_value(Response::rejected(
                &name,
                Issue::new("INVALID_ARGUMENT", "arguments must be an object"),
            ))
            .unwrap());
        }
        if let Some(scope) = self.0.initiative.as_deref() {
            if name == "search" {
                if args.get("initiatives").is_none() {
                    args["initiatives"] = json!([scope]);
                }
            } else if args.get("initiative").is_none()
                && !(name == "context" && args.get("view").and_then(Value::as_str) == Some("graph"))
            {
                let schema = &definition.input_schema;
                let accepts = schema.pointer("/properties/initiative").is_some()
                    || schema
                        .get("oneOf")
                        .and_then(Value::as_array)
                        .is_some_and(|variants| {
                            variants.iter().any(|variant| {
                                variant.pointer("/properties/initiative").is_some()
                                    && variant.pointer("/properties/action/const").is_none_or(
                                        |action| {
                                            args.get("action")
                                                .is_none_or(|provided| provided == action)
                                        },
                                    )
                            })
                        });
                if accepts {
                    args["initiative"] = json!(scope);
                }
            }
        }
        normalize_arguments(&name, &mut args);
        if definition.priority == "cloud" {
            if let Err(issue) = validate(&name, &args) {
                return Ok(to_value(Response::rejected(&name, issue)).unwrap());
            }
            let result = core_call(&self.0, &name, args).await;
            return Ok(result);
        }
        let opts = self.0.api_options.clone();
        let worker_opts = opts.clone();
        let store = self.0.store.clone();
        let operation = name.clone();
        let input = args.clone();
        let cancel = CancellationToken::new();
        let _cancel_on_drop = cancel.clone().drop_guard();
        let control = ExecutionControl::new(move || cancel.is_cancelled());
        let response = spawn_blocking(move || {
            execute_with_control(&store, &operation, input, &worker_opts, &control)
        })
        .await;
        let mut response = match response {
            Ok(r) => r,
            Err(e) => Response::rejected(&name, Issue::new("EXECUTION_FAILED", e.to_string())),
        };
        if !matches!(response.state, State::Rejected) && !response.replayed {
            if args.pointer("/sharing/visibility").and_then(Value::as_str) == Some("shared") {
                let id = match &response.data {
                    ResponseData::Mutation(d) => d.primary.as_ref().map(|e| e.id.clone()),
                    _ => None,
                };
                if let Some(id) = id {
                    let shared=core_call(&self.0,"share",json!({"name":id,"initiative":args["initiative"],"cloud":args.pointer("/sharing/cloud").cloned().unwrap_or(Value::Null)})).await;
                    let store = self.0.store.clone();
                    let lookup = id;
                    let accepted = spawn_blocking(move || {
                        get_visibility(&store, &lookup).is_ok_and(|v| v == Visibility::Shared)
                    })
                    .await
                    .unwrap_or(false);
                    if let ResponseData::Mutation(d) = &mut response.data {
                        d.failed_steps.retain(|s| s.step != "share");
                        if !accepted {
                            d.failed_steps.push(FailedStep {
                                step: "share".into(),
                                code: "REMOTE_FAILED".into(),
                                message: shared.to_string(),
                            });
                            response
                                .issues
                                .push(Issue::new("REMOTE_FAILED", shared.to_string()));
                        }
                    }
                }
            }
            if let Some(edge) = edge_mirror(&response)
                && let Some(outcome) = propagate_edge(
                    &self.0,
                    args["cloud"].as_str(),
                    &edge.from,
                    &edge.to,
                    edge.edge_type,
                    edge.change,
                    args["initiative"].as_str(),
                )
                .await
                && outcome["mirrored"] != true
            {
                let message = outcome.to_string();
                if let ResponseData::Mutation(d) = &mut response.data {
                    d.failed_steps.push(FailedStep {
                        step: "mirror_edge".into(),
                        code: "REMOTE_FAILED".into(),
                        message: message.clone(),
                    });
                }
                response.issues.push(Issue::new("REMOTE_FAILED", message));
            }

            if name == "initiative_manage" && args.get("cloud").is_some() {
                let outcome = match self.0.cloud_registry().resolve(args["cloud"].as_str()) {
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
                if let ResponseData::Mutation(d) = &mut response.data {
                    d.failed_steps.retain(|s| s.step != "namespace_cloud");
                    if let Err(message) = outcome.and_then(|(status, body)| {
                        if (200..300).contains(&status) {
                            Ok(())
                        } else {
                            Err(format!("cloud returned {status}: {body}"))
                        }
                    }) {
                        d.failed_steps.push(FailedStep {
                            step: "namespace_cloud".into(),
                            code: "REMOTE_FAILED".into(),
                            message: message.clone(),
                        });
                        response.issues.push(Issue::new("REMOTE_FAILED", message));
                    }
                }
            }
            if let ResponseData::Mutation(d) = &response.data
                && !matches!(response.state, State::Unchanged)
            {
                response.state = if d.failed_steps.is_empty() {
                    State::Applied
                } else {
                    State::Partial
                };
            }
            let store = self.0.store.clone();
            let operation = name;
            let input = args.clone();
            let record = response.clone();
            response =
                spawn_blocking(move || finish_remote(&store, &operation, input, &opts, &record))
                    .await
                    .unwrap_or(response);
        }
        self.0.after_tool(args["initiative"].as_str());
        Ok(to_value(response).unwrap())
    }
}

impl KaeruMemory {
    pub fn with_api_artifact_root(mut self, root: impl Into<PathBuf>) -> Self {
        self.api_options.artifact_root = Some(root.into());
        self
    }

    pub async fn local_tool_definitions(&self) -> Vec<ToolDefinition> {
        tool_definitions()
            .iter()
            .enumerate()
            .filter(|(_, d)| d.priority != "cloud")
            .map(|(i, d)| ToolDefinition {
                name: NAMES[i].into(),
                description: d.description.clone(),
                parameters: d.input_schema.clone(),
            })
            .collect()
    }
    pub async fn cloud_tool_definitions(&self) -> Vec<ToolDefinition> {
        tool_definitions()
            .iter()
            .enumerate()
            .filter(|(_, d)| d.priority == "cloud")
            .map(|(i, d)| ToolDefinition {
                name: NAMES[i].into(),
                description: d.description.clone(),
                parameters: d.input_schema.clone(),
            })
            .collect()
    }
    pub fn context(&self) -> MemoryTool<0> {
        MemoryTool(self.clone())
    }
    pub fn search(&self) -> MemoryTool<1> {
        MemoryTool(self.clone())
    }
    pub fn read(&self) -> MemoryTool<2> {
        MemoryTool(self.clone())
    }
    pub fn note(&self) -> MemoryTool<3> {
        MemoryTool(self.clone())
    }
    pub fn reference(&self) -> MemoryTool<4> {
        MemoryTool(self.clone())
    }
    pub fn task(&self) -> MemoryTool<5> {
        MemoryTool(self.clone())
    }
    pub fn task_update(&self) -> MemoryTool<6> {
        MemoryTool(self.clone())
    }
    pub fn claim(&self) -> MemoryTool<7> {
        MemoryTool(self.clone())
    }
    pub fn verdict(&self) -> MemoryTool<8> {
        MemoryTool(self.clone())
    }
    pub fn revise(&self) -> MemoryTool<9> {
        MemoryTool(self.clone())
    }
    pub fn replace(&self) -> MemoryTool<10> {
        MemoryTool(self.clone())
    }
    pub fn relate(&self) -> MemoryTool<11> {
        MemoryTool(self.clone())
    }
    pub fn review(&self) -> MemoryTool<12> {
        MemoryTool(self.clone())
    }
    pub fn trail(&self) -> MemoryTool<13> {
        MemoryTool(self.clone())
    }
    pub fn pin(&self) -> MemoryTool<14> {
        MemoryTool(self.clone())
    }
    pub fn reflect(&self) -> MemoryTool<15> {
        MemoryTool(self.clone())
    }
    pub fn lifecycle(&self) -> MemoryTool<16> {
        MemoryTool(self.clone())
    }
    pub fn synthesise(&self) -> MemoryTool<17> {
        MemoryTool(self.clone())
    }
    pub fn forget(&self) -> MemoryTool<18> {
        MemoryTool(self.clone())
    }
    pub fn history(&self) -> MemoryTool<19> {
        MemoryTool(self.clone())
    }
    pub fn retention(&self) -> MemoryTool<20> {
        MemoryTool(self.clone())
    }
    pub fn initiatives(&self) -> MemoryTool<21> {
        MemoryTool(self.clone())
    }
    pub fn initiative_manage(&self) -> MemoryTool<22> {
        MemoryTool(self.clone())
    }
    pub fn board_config(&self) -> MemoryTool<23> {
        MemoryTool(self.clone())
    }
    pub fn policy(&self) -> MemoryTool<24> {
        MemoryTool(self.clone())
    }
    pub fn export(&self) -> MemoryTool<25> {
        MemoryTool(self.clone())
    }
    pub fn clouds(&self) -> MemoryTool<26> {
        MemoryTool(self.clone())
    }
    pub fn share(&self) -> MemoryTool<27> {
        MemoryTool(self.clone())
    }
    pub fn unshare(&self) -> MemoryTool<28> {
        MemoryTool(self.clone())
    }
    pub fn cloud_initiatives(&self) -> MemoryTool<29> {
        MemoryTool(self.clone())
    }
    pub fn cloud_recall(&self) -> MemoryTool<30> {
        MemoryTool(self.clone())
    }
    pub fn pull(&self) -> MemoryTool<31> {
        MemoryTool(self.clone())
    }
    pub fn link_cloud(&self) -> MemoryTool<32> {
        MemoryTool(self.clone())
    }
    pub fn cloud_links(&self) -> MemoryTool<33> {
        MemoryTool(self.clone())
    }
    pub fn sync_review(&self) -> MemoryTool<34> {
        MemoryTool(self.clone())
    }

    pub(crate) fn chain_local<M: CompletionModel>(
        &self,
        b: AgentBuilder<M, (), NoToolConfig>,
    ) -> AgentBuilder<M, (), WithBuilderTools> {
        b.tool(self.context())
            .tool(self.search())
            .tool(self.read())
            .tool(self.note())
            .tool(self.reference())
            .tool(self.task())
            .tool(self.task_update())
            .tool(self.claim())
            .tool(self.verdict())
            .tool(self.revise())
            .tool(self.replace())
            .tool(self.relate())
            .tool(self.review())
            .tool(self.trail())
            .tool(self.pin())
            .tool(self.reflect())
            .tool(self.lifecycle())
            .tool(self.synthesise())
            .tool(self.forget())
            .tool(self.history())
            .tool(self.retention())
            .tool(self.initiatives())
            .tool(self.initiative_manage())
            .tool(self.board_config())
            .tool(self.policy())
            .tool(self.export())
    }
    pub(crate) fn chain_cloud<M: CompletionModel>(
        &self,
        b: AgentBuilder<M, (), WithBuilderTools>,
    ) -> AgentBuilder<M, (), WithBuilderTools> {
        b.tool(self.clouds())
            .tool(self.share())
            .tool(self.unshare())
            .tool(self.cloud_initiatives())
            .tool(self.cloud_recall())
            .tool(self.pull())
            .tool(self.link_cloud())
            .tool(self.cloud_links())
            .tool(self.sync_review())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use kaeru_core::Store;

    use super::*;

    #[tokio::test]
    async fn rig_calls_share_the_same_state_and_replay_contract() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let memory = KaeruMemory::with_initiative(store, "rig-test");
        let args = json!({"body":"A real observation.","request_id":"rig-1"});
        let first = memory.note().call(args.clone()).await.unwrap();
        assert_eq!(first["api_version"], 1);
        assert_eq!(first["state"], "applied");
        let second = memory.note().call(args).await.unwrap();
        assert_eq!(second["replayed"], true);
        assert_eq!(
            first["data"]["primary"]["id"],
            second["data"]["primary"]["id"]
        );
        assert_eq!(memory.local_tool_definitions().await.len(), 26);
        assert_eq!(memory.cloud_tool_definitions().await.len(), 9);
    }
    #[tokio::test]
    async fn scoped_administrative_actions_receive_only_their_applicable_default() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let mem = KaeruMemory::with_initiative(store.clone(), "rig-test");
        let captured = mem
            .note()
            .call(json!({"body":"Current handoff."}))
            .await
            .unwrap();
        let id = captured["data"]["primary"]["id"].as_str().unwrap();
        let set = mem
            .initiative_manage()
            .call(json!({"action":"set_slot","slot":"handoff","node":id}))
            .await
            .unwrap();
        assert_eq!(set["state"], "applied");
        let rename = mem
            .initiative_manage()
            .call(json!({"action":"rename","old":"rig-test","new":"renamed"}))
            .await
            .unwrap();
        assert_eq!(rename["state"], "applied");
        assert_eq!(
            kaeru_core::slot_holder(&store, "renamed", "handoff")
                .unwrap()
                .as_deref(),
            Some(id)
        );
    }

    #[tokio::test]
    async fn capture_and_share_without_a_cloud_retains_and_replays_the_local_capture() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let mem = KaeruMemory::with_initiative(store.clone(), "rig-test");
        let args = json!({"body":"Shareable result.","sharing":{"visibility":"shared"},"request_id":"share-unavailable"});
        let first = mem.note().call(args.clone()).await.unwrap();
        assert_eq!(first["state"], "partial");
        let id = first["data"]["primary"]["id"].as_str().unwrap();
        assert_eq!(
            get_visibility(&store, &id.to_string()).unwrap(),
            Visibility::Local
        );
        let replay = mem.note().call(args).await.unwrap();
        assert_eq!(replay["replayed"], true);
        assert_eq!(first["data"], replay["data"]);
    }

    #[tokio::test]
    async fn graph_scope_selection_does_not_receive_an_implicit_singular_scope() {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let first = KaeruMemory::with_initiative(store.clone(), "first");
        let second = KaeruMemory::with_initiative(store, "second");
        first
            .note()
            .call(json!({"name":"From first","body":"A"}))
            .await
            .unwrap();
        second
            .note()
            .call(json!({"name":"From second","body":"B"}))
            .await
            .unwrap();
        let all = first
            .context()
            .call(json!({"view":"graph","budget_chars":64000}))
            .await
            .unwrap();
        let names = all["data"]["graph"]["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|n| n["name"].as_str())
            .collect::<Vec<_>>();
        assert!(names.contains(&"From first") && names.contains(&"From second"));
        let selected = first
            .context()
            .call(json!({"view":"graph","initiatives":["second"],"budget_chars":64000}))
            .await
            .unwrap();
        assert_ne!(selected["state"], "rejected", "{selected}");
        assert!(
            selected["data"]["graph"]["nodes"]
                .as_array()
                .unwrap()
                .iter()
                .all(|n| n["initiatives"] == json!(["second"]))
        );
    }

    #[tokio::test]
    async fn an_opted_in_hygiene_report_is_consumed_by_context_once() {
        let mut cfg = kaeru_core::KaeruConfig::defaults();
        cfg.hygiene_writes_trigger = 2;
        cfg.hygiene_batch_pause_ms = 0;
        let store = Arc::new(Store::open_in_memory_with(cfg).unwrap());
        let plain = KaeruMemory::with_initiative(store.clone(), "rig-test");
        assert!(!plain.hygiene.is_enabled());
        plain
            .note()
            .call(json!({"body":"First observation."}))
            .await
            .unwrap();
        plain
            .note()
            .call(json!({"body":"Second observation."}))
            .await
            .unwrap();
        assert_eq!(plain.hygiene.passes_started(), 0);
        let enabled = plain.with_hygiene();
        enabled.after_tool(Some("rig-test"));
        for _ in 0..100 {
            if enabled.hygiene.passes_started() > 0 {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        assert!(enabled.hygiene.passes_started() > 0);
        enabled.shutdown_hygiene();
        // Inject a known completed headline to test delivery independently of
        // whether this young store had any nodes eligible for cooling.
        store.run("?[initiative,last_run_at,nodes_at_last_run,pending_report] <- [['rig-test',1.0,2,'Layers changed']] :put initiative_hygiene {initiative=>last_run_at,nodes_at_last_run,pending_report}").unwrap();
        let first = enabled.context().call(json!({})).await.unwrap();
        assert!(first.to_string().contains("Layers changed"));
        let second = enabled.context().call(json!({})).await.unwrap();
        assert!(!second.to_string().contains("Layers changed"));
    }
}
