//! MCP transport for the shared 35-operation memory API.

use std::sync::Arc;

use kaeru_core::Store;
use rmcp::model::{
    CallToolRequestParams, CallToolResult, Implementation, ListToolsResult, PaginatedRequestParams,
    ProtocolVersion, ServerCapabilities, ServerInfo, Tool,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData as McpError, RoleServer, ServerHandler};
use serde_json::Value;
use tokio_util::sync::CancellationToken;

use crate::canonical;
use crate::cloud_client::CloudRegistry;
use crate::hygiene::HygieneScheduler;
use crate::settings::{MCP_SERVER_NAME, MCP_SERVER_TITLE, MCP_SERVER_WEBSITE_URL};
use crate::update::UpdateNotice;

#[derive(Clone)]
pub struct KaeruServer {
    store: Arc<Store>,
    clouds: CloudRegistry,
    hygiene: HygieneScheduler,
    update_notice: UpdateNotice,
}

impl KaeruServer {
    pub(crate) fn tool_definitions() -> Vec<Tool> {
        canonical::definitions()
    }

    pub fn new(
        store: Store,
        clouds: CloudRegistry,
        cancel: CancellationToken,
        hygiene_enabled: bool,
    ) -> Self {
        let store = Arc::new(store);
        let hygiene = HygieneScheduler::new(store.clone(), cancel, hygiene_enabled);
        Self {
            store,
            clouds,
            hygiene,
            update_notice: UpdateNotice::default(),
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
}

impl ServerHandler for KaeruServer {
    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        Ok(ListToolsResult {
            tools: Self::tool_definitions(),
            ..ListToolsResult::default()
        })
    }
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let args = Value::Object(request.arguments.unwrap_or_default());
        let result = canonical::call(
            self.store.clone(),
            &self.clouds,
            request.name.as_ref(),
            args.clone(),
        )
        .await;
        self.hygiene.consider(args["initiative"].as_str());
        result
    }
    fn get_tool(&self, name: &str) -> Option<Tool> {
        Self::tool_definitions()
            .into_iter()
            .find(|t| t.name == name)
    }
    fn get_info(&self) -> ServerInfo {
        let mut info=ServerInfo::new(
            ServerCapabilities::builder().enable_tools().build(),
        )
        .with_server_info(
            Implementation::new(MCP_SERVER_NAME, env!("CARGO_PKG_VERSION"))
                .with_title(MCP_SERVER_TITLE)
                .with_website_url(MCP_SERVER_WEBSITE_URL),
        )
        .with_protocol_version(ProtocolVersion::LATEST)
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
