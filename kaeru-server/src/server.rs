//! MCP transport for the shared 35-operation memory API.

use std::borrow::Cow;
use std::sync::Arc;

use kaeru_core::Store;
use rmcp::model::{
    CallToolRequestParams, CallToolResponse, ContentBlock, Implementation, InitializeRequestParams,
    InitializeResult, ListResourceTemplatesResult, ListResourcesResult, ListToolsResult,
    PaginatedRequestParams, ProtocolVersion, ReadResourceRequestParams, ReadResourceResponse,
    ReadResourceResult, ServerCapabilities, ServerConfig, Tool,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData as McpError, RoleServer, ServerHandler};
use serde_json::Value;
use tokio_util::sync::CancellationToken;

use crate::canonical;
use crate::cloud_client::CloudRegistry;
use crate::exports::ExportResources;
use crate::hygiene::HygieneScheduler;
use crate::session;
use crate::settings::{
    MCP_PROTOCOL_VERSION, MCP_SERVER_NAME, MCP_SERVER_TITLE, MCP_SERVER_WEBSITE_URL,
};
use crate::update::UpdateNotice;
use kaeru_core::models::{FailedStep, Issue, Response, ResponseData, State};

#[derive(Clone)]
pub struct KaeruServer {
    store: Arc<Store>,
    clouds: CloudRegistry,
    hygiene: HygieneScheduler,
    update_notice: UpdateNotice,
    exports: ExportResources,
    require_http_session: bool,
}

impl KaeruServer {
    pub(crate) fn tool_definitions() -> Vec<Tool> {
        canonical::definitions()
    }

    #[cfg(test)]
    pub fn new(
        store: Store,
        clouds: CloudRegistry,
        cancel: CancellationToken,
        hygiene_enabled: bool,
    ) -> Self {
        Self::from_shared(Arc::new(store), clouds, cancel, hygiene_enabled)
    }

    pub(crate) fn from_shared(
        store: Arc<Store>,
        clouds: CloudRegistry,
        cancel: CancellationToken,
        hygiene_enabled: bool,
    ) -> Self {
        let hygiene = HygieneScheduler::new(store.clone(), cancel, hygiene_enabled);
        Self {
            store,
            clouds,
            hygiene,
            update_notice: UpdateNotice::default(),
            exports: ExportResources::default(),
            require_http_session: false,
        }
    }
    pub fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
    pub fn hygiene_scheduler(&self) -> &HygieneScheduler {
        &self.hygiene
    }
    pub fn update_notice(&self) -> UpdateNotice {
        self.update_notice.clone()
    }
    pub fn cloud_registry(&self) -> CloudRegistry {
        self.clouds.clone()
    }

    pub fn stateful_http(mut self) -> Self {
        self.require_http_session = true;
        self
    }

    fn session_lease(
        &self,
        extensions: &rmcp::model::Extensions,
    ) -> Result<Option<Arc<session::SessionSlot>>, McpError> {
        if self.require_http_session {
            session::require_lease(extensions).map(Some)
        } else {
            Ok(None)
        }
    }
}

impl ServerHandler for KaeruServer {
    fn supported_protocol_versions(&self) -> Cow<'static, [ProtocolVersion]> {
        Cow::Borrowed(ProtocolVersion::known_up_to(&MCP_PROTOCOL_VERSION))
    }
    async fn initialize(
        &self,
        request: InitializeRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        let _session = self.session_lease(&context.extensions)?;
        context.peer.set_peer_info(request.clone());
        self.negotiate_initialize(&request)
    }

    async fn ping(&self, context: RequestContext<RoleServer>) -> Result<(), McpError> {
        let _session = self.session_lease(&context.extensions)?;
        Ok(())
    }
    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let _session = self.session_lease(&context.extensions)?;
        Ok(ListToolsResult {
            tools: Self::tool_definitions(),
            ..ListToolsResult::default()
        })
    }
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, McpError> {
        let _session = self.session_lease(&context.extensions)?;
        let args = Value::Object(request.arguments.unwrap_or_default());
        let options = canonical::options();
        let root = options.artifact_root.clone();
        let is_export = request.name.as_ref() == "export";
        let mut result = canonical::call_with_cancellation(
            self.store.clone(),
            &self.clouds,
            request.name.as_ref(),
            args.clone(),
            options,
            context.ct,
        )
        .await;
        if is_export
            && let (Ok(outcome), Some(root)) = (&mut result, root)
            && let Some(value) = outcome.structured_content.clone()
            && let Ok(mut response) = serde_json::from_value::<Response>(value)
            && !matches!(response.state, State::Rejected)
        {
            let artifacts = match &response.data {
                ResponseData::Mutation(data) => data.artifacts.clone(),
                _ => vec![],
            };
            let exports = self.exports.clone();
            let links = tokio::task::spawn_blocking(move || {
                artifacts
                    .iter()
                    .map(|a| exports.register(a, &root))
                    .collect::<Vec<_>>()
            })
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
            let mut content = vec![];
            for link in links {
                match link {
                    Ok(resource) => content.push(ContentBlock::resource_link(resource)),
                    Err(message) => {
                        response.state = State::Partial;
                        response
                            .issues
                            .push(Issue::new("ARTIFACT_DOWNLOAD_UNAVAILABLE", &message));
                        if let ResponseData::Mutation(data) = &mut response.data {
                            data.failed_steps.push(FailedStep {
                                step: "mcp_download".into(),
                                code: "ARTIFACT_DOWNLOAD_UNAVAILABLE".into(),
                                message,
                            });
                        }
                    }
                }
            }
            *outcome = canonical::render(response);
            outcome.content.extend(content);
        }
        self.hygiene.consider(args["initiative"].as_str());
        result.map(Into::into)
    }
    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResponse, McpError> {
        let _session = self.session_lease(&context.extensions)?;
        let exports = self.exports.clone();
        let content = tokio::task::spawn_blocking(move || exports.read(&request.uri))
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        content
            .map(|content| ReadResourceResult::new(vec![content]).into())
            .ok_or_else(|| McpError::resource_not_found("unknown or expired export resource", None))
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        let _session = self.session_lease(&context.extensions)?;
        Ok(ListResourcesResult::default())
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, McpError> {
        let _session = self.session_lease(&context.extensions)?;
        Ok(ListResourceTemplatesResult::default())
    }
    fn get_tool(&self, name: &str) -> Option<Tool> {
        Self::tool_definitions()
            .into_iter()
            .find(|t| t.name == name)
    }
    fn get_info(&self) -> ServerConfig {
        let mut info=ServerConfig::new(
            ServerCapabilities::builder().enable_tools().enable_resources().build(),
        )
        .with_server_info(
            Implementation::new(MCP_SERVER_NAME, env!("CARGO_PKG_VERSION"))
                .with_title(MCP_SERVER_TITLE)
                .with_website_url(MCP_SERVER_WEBSITE_URL),
        )
        .with_protocol_version(MCP_PROTOCOL_VERSION)
        .with_instructions(
            "Kaeru is your memory of record. Pass initiative on project calls. \
             Start with context to recover rules, current records and unfinished work; \
             search finds text or explicit filters, read returns full content with currentness \
             and stored reasoning. History and read(recorded_at) show previous versions.\n\n\
             Capture meaning: note for observations, decisions, constraints, plans and outcomes; \
             reference preserves exact supplied sources; task creates work; claim can include an \
             already reached initial_status and evidence. Verdict requires supported, refuted or \
             inconclusive, and can record the completed check in the same call.\n\n\
             Revise edits the same identity. Replace explicitly supersedes an old fact and \
             reports dependents. A generic relation is not an atomic replacement. Tier \
             (operational/archival) differs from recall layer (core/hot/warm/cold/frozen). \
             Lifecycle preserves full content and reports identity changes.\n\n\
             Ordinary reads deliver material conflicts and saved trail summaries. Expand only \
             when needed; follow continuation when coverage is incomplete. Search prefix=true \
             helps inflections; text is lexical, not a query expression. Preserve capture language.\n\n\
             Mutation results report actual committed IDs and state. Partial means the local \
             record may exist even if a relation or remote step failed. Reuse a host request_id \
             to recover an uncertain outcome without duplicate capture. Replayed results describe \
             the original operation. Cloud reads remain explicit when local coverage is insufficient."
        );
        if let Some(notice) = self
            .update_notice
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .as_deref()
        {
            info.instructions = Some(format!(
                "{}\n\n{notice}",
                info.instructions.as_deref().unwrap_or("")
            ));
        }
        info
    }
}

#[cfg(test)]
mod tests {
    use kaeru_core::Store;
    use rmcp::ServerHandler;
    use tokio_util::sync::CancellationToken;

    use super::KaeruServer;
    use crate::cloud_client::CloudRegistry;

    #[test]
    fn server_advertises_its_product_identity() {
        let server = KaeruServer::new(
            Store::open_in_memory().unwrap(),
            CloudRegistry::default(),
            CancellationToken::new(),
            false,
        );
        let info = server.get_info().server_info;
        assert_eq!(info.name, "kaeru");
        assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(info.title.as_deref(), Some("kaeru memory"));
        assert_eq!(
            info.website_url.as_deref(),
            Some("https://lamantin-ai.com/products/kaeru/")
        );
    }

    #[test]
    fn instructions_fit_the_client_truncation_budget() {
        let server = KaeruServer::new(
            Store::open_in_memory().unwrap(),
            CloudRegistry::default(),
            CancellationToken::new(),
            false,
        );
        let instructions = server.get_info().instructions.unwrap();
        assert!(instructions.len() <= 2048);
        for name in ["context", "search", "read", "verdict", "history"] {
            assert!(instructions.to_lowercase().contains(name));
        }
    }
    #[test]
    fn removed_names_are_not_registered() {
        let server = KaeruServer::new(
            Store::open_in_memory().unwrap(),
            CloudRegistry::default(),
            CancellationToken::new(),
            false,
        );
        for name in [
            "awake", "jot", "episode", "cite", "why", "confirm", "refute", "slot", "unsettle",
        ] {
            assert!(server.get_tool(name).is_none());
        }
        assert_eq!(KaeruServer::tool_definitions().len(), 35);
    }
}
