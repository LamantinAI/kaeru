//! Reusable runtime with explicit storage, policy and account inputs.

use std::{future::IntoFuture, net::SocketAddr, sync::Arc, time::Duration};

use axum::{
    Router,
    extract::{Request, State},
    http::{HeaderValue, header::ACCEPT},
    middleware::Next,
    response::Response,
};
use kaeru_core::Store;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};
use tokio::{net::TcpListener, signal};
use tokio_util::sync::CancellationToken;

use crate::{
    api, auth, diagnostics,
    server::KaeruServer,
    session,
    settings::{
        KaeruMcpConfig, RuntimeOptions,
        runtime::{SHUTDOWN_GRACE, STORE_RELEASE_POLL},
    },
    sse, ui, update,
};

mod clouds;

pub struct ServerRuntime {
    config: KaeruMcpConfig,
    store: Arc<Store>,
    server: Option<KaeruServer>,
    router: Option<Router>,
    cancel: CancellationToken,
}

impl ServerRuntime {
    /// Compose transports around an already opened Store. No local files,
    /// environment variables or storage defaults are discovered here.
    /// Call inside the host Tokio runtime to start bounded background tasks.
    pub fn new(store: Arc<Store>, mcp_config: KaeruMcpConfig, mut options: RuntimeOptions) -> Self {
        let cancel = CancellationToken::new();
        let clouds = clouds::registry(&mcp_config, options.update_check_enabled);
        let server = KaeruServer::from_shared(
            store.clone(),
            clouds,
            cancel.child_token(),
            options.hygiene_enabled,
        );
        server.hygiene_scheduler().spawn_sweeper();
        let has_account = options.account.is_some();
        let web_session = auth::session::WebSession::new(options.account.take());
        let browser = ui::BrowserAuth {
            auth_enabled: options.access.enabled,
            session: web_session,
            static_token: options.access.mcp_token.clone(),
            api: options.api_config.clone(),
            clouds: server
                .cloud_registry()
                .names()
                .into_iter()
                .map(str::to_owned)
                .collect(),
        };
        let api_router = options.api_enabled.then(|| {
            api::router_with_clouds(
                server.store(),
                options.api_config.clone(),
                server.cloud_registry(),
            )
            .layer(axum::middleware::from_fn_with_state(
                browser.clone(),
                ui::api_access,
            ))
        });

        let mut session_manager = LocalSessionManager::default();
        // The SDK timer observes transport events rather than active handlers.
        // Managed expiry below waits for application work and response delivery.
        session_manager.session_config.keep_alive = None;

        // Asks once a day whether this binary is behind, and leaves the answer
        // where initialization will hand it to an agent. Spawned: a slow or unreachable
        // GitHub must never delay startup, and a failure is silence (#99).
        if options.update_check_enabled {
            update::spawn(server.update_notice(), cancel.child_token());
        }

        let sse_router = sse::router(
            server.clone(),
            cancel.child_token(),
            &mcp_config.sse_path,
            &mcp_config.messages_path,
        );
        // rmcp's Streamable HTTP transport rejects any request whose `Host`
        // header isn't in `allowed_hosts` (default: loopback only) as a
        // DNS-rebinding guard — answering `403 Forbidden: Host header is not
        // allowed`, which clients like Claude Code mislabel as "Needs
        // authentication". When exposed on a routable address the operator
        // must whitelist the host(s) clients use; we keep the loopback
        // defaults so localhost sessions keep working regardless.
        let default_config = StreamableHttpServerConfig::default();
        let mut allowed_hosts = default_config.allowed_hosts.clone();
        allowed_hosts.extend(
            mcp_config
                .allowed_hosts
                .split(',')
                .map(str::trim)
                .filter(|h| !h.is_empty())
                .map(str::to_string),
        );
        tracing::info!(?allowed_hosts, "host allow-list for inbound MCP requests");

        let session_manager = Arc::new(session::BoundedSessionManager::new(
            session_manager,
            mcp_config.max_sessions,
        ));
        session_manager.spawn_idle_reaper(
            Duration::from_secs(mcp_config.keep_alive_secs),
            cancel.child_token(),
        );
        let session_server = server.clone();
        let service = StreamableHttpService::new(
            // Each MCP session reuses the same KaeruServer (and therefore
            // the same Arc<Store> / RocksDB lock); cloning the server is
            // cheap and shares state across sessions.
            move || Ok(session_server.clone().stateful_http()),
            session_manager.clone(),
            default_config
                .with_allowed_hosts(allowed_hosts)
                .with_legacy_session_mode(true)
                .with_max_request_body_bytes(if mcp_config.max_request_body_bytes == 0 {
                    usize::MAX
                } else {
                    mcp_config.max_request_body_bytes
                })
                .with_cancellation_token(cancel.child_token()),
        );

        let router = axum::Router::new()
            .nest_service(&mcp_config.mount_path, service)
            .layer(axum::middleware::from_fn_with_state(
                session_manager.clone(),
                session::admit,
            ))
            .merge(sse_router);

        // Authentication is one installation-level choice for MCP, HTTP and UI.
        // A required mode without a token fails closed instead of reopening MCP.
        let auth_enabled = options.access.enabled;
        let router = if auth_enabled {
            tracing::info!("Authentication enabled for inbound MCP requests");
            router.layer(axum::middleware::from_fn_with_state(
                Arc::<str>::from(options.access.mcp_token.as_str()),
                auth::require_bearer,
            ))
        } else {
            tracing::info!("Authentication disabled by explicit host policy");
            router
        };

        let mut router = router;
        if let Some(api_router) = api_router {
            router = router.merge(api_router);
        }
        if options.ui_enabled {
            router = router.merge(ui::router(browser));
        } else if auth_enabled && has_account {
            router = router.merge(ui::auth_routes(browser));
        }

        // rmcp's Streamable HTTP transport returns `406 Not Acceptable` to any
        // request whose `Accept` header doesn't list BOTH `application/json` and
        // `text/event-stream`. Several MCP clients send only `application/json` —
        // Claude Code's HTTP transport and the `claude mcp list` health check among
        // them — and would never connect. Normalize `Accept` on the MCP mount path
        // so those clients are accepted. Outermost layer: runs before auth and the
        // transport. Other paths (`/sse`, `/graph.json`) are untouched.
        let mount: Arc<str> = Arc::from(mcp_config.mount_path.as_str());
        let router = router.layer(axum::middleware::from_fn_with_state(
            mount,
            normalize_mcp_accept,
        ));
        let router = if mcp_config.diagnostics_interval_secs == 0 {
            router
        } else {
            diagnostics::attach(
                router,
                session_manager,
                Duration::from_secs(mcp_config.diagnostics_interval_secs),
                cancel.child_token(),
            )
        };

        Self {
            config: mcp_config,
            store,
            server: Some(server),
            router: Some(router),
            cancel,
        }
    }

    pub fn router(&self) -> Router {
        self.router
            .as_ref()
            .expect("runtime router exists before serve")
            .clone()
    }
    pub fn store(&self) -> Arc<Store> {
        self.store.clone()
    }
    pub fn cancellation_token(&self) -> CancellationToken {
        self.cancel.clone()
    }
    pub fn merge(mut self, router: Router) -> Self {
        self.router = Some(
            self.router
                .take()
                .expect("runtime router exists before serve")
                .merge(router),
        );
        self
    }

    pub async fn serve(mut self) -> std::io::Result<()> {
        let address = SocketAddr::new(self.config.listen_address, self.config.listen_port);
        let listener = TcpListener::bind(address).await?;
        tracing::info!(%address,mcp_mount=%self.config.mount_path,"memory server listening");
        let router = self.router.take().expect("serve called once");
        {
            let graceful = axum::serve(listener, router)
                .with_graceful_shutdown(shutdown(self.cancel.clone()))
                .into_future();
            tokio::pin!(graceful);
            tokio::select! {
                result=&mut graceful=>{result?;}
                ()=grace_deadline(self.cancel.clone(),SHUTDOWN_GRACE)=>{
                    tracing::warn!("shutdown grace elapsed; closing remaining connections");
                }
            }
        }
        self.cancel.cancel();
        drop(self.server.take());
        // Give cancelled background owners a bounded window to release the
        // Store. This permits a normal TiKV shutdown to release its lease.
        let deadline = tokio::time::Instant::now() + SHUTDOWN_GRACE;
        while Arc::strong_count(&self.store) > 1 && tokio::time::Instant::now() < deadline {
            tokio::time::sleep(STORE_RELEASE_POLL).await;
        }
        Ok(())
    }
}

impl Drop for ServerRuntime {
    fn drop(&mut self) {
        self.cancel.cancel();
    }
}

/// Waits for either Ctrl-C or SIGTERM, then cancels the rmcp service
/// token so in-flight sessions wind down cleanly. SIGTERM coverage is
/// what makes the daemon usable under systemd / launchd.
pub async fn shutdown(cancel: CancellationToken) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to register Ctrl-C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = cancel.cancelled() => {},
        _ = ctrl_c => tracing::info!("Ctrl-C received"),
        _ = terminate => tracing::info!("SIGTERM received"),
    }
    cancel.cancel();
}

/// Resolves once shutdown has been requested (`cancel` fired) **and** the
/// grace window has elapsed — the backstop that bounds how long the daemon
/// waits for stubborn long-lived connections to drain.
async fn grace_deadline(cancel: CancellationToken, grace: Duration) {
    cancel.cancelled().await;
    tokio::time::sleep(grace).await;
}

/// Rewrites the `Accept` header to `application/json, text/event-stream` on the
/// MCP mount path. rmcp's Streamable HTTP transport answers `406 Not
/// Acceptable` unless the client accepts *both* JSON and an SSE stream; some
/// clients (Claude Code's HTTP transport, the `claude mcp list` health check)
/// send only `application/json` and would never connect. Scoped to `mount` so
/// `/sse` and `/graph.json` pass through unchanged.
async fn normalize_mcp_accept(
    State(mount): State<Arc<str>>,
    mut request: Request,
    next: Next,
) -> Response {
    if request.uri().path().starts_with(mount.as_ref()) {
        request.headers_mut().insert(
            ACCEPT,
            HeaderValue::from_static("application/json, text/event-stream"),
        );
    }
    next.run(request).await
}
