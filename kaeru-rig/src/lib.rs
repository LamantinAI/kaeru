//! Rig tools for Kaeru's shared memory API. `install` exposes 26 local
//! operations; `install_with_cloud` adds the nine network operations. Every
//! store operation runs on a blocking thread with an isolated initiative scope.

use std::sync::Arc;

use hygiene::HygieneScheduler;
use kaeru_core::Store;
use kaeru_core::api::{ApiOptions, NodeRef, resolve_node};
use rig::agent::{AgentBuilder, NoToolConfig, WithBuilderTools};
use rig::completion::CompletionModel;
use tokio::task::spawn_blocking;

mod cloud;
mod cloud_client;
mod hygiene;
mod tools;

pub use cloud_client::{CloudClient, CloudRegistry};
pub use tools::MemoryTool;
/// Shared handle to the kaeru substrate plus the initiative an agent works in.
/// Clone it freely — every clone shares the same `Arc<Store>`. The `mem.*()`
/// methods build the individual tools.
#[derive(Clone)]
pub struct KaeruMemory {
    store: Arc<Store>,
    initiative: Option<Arc<str>>,
    clouds: CloudRegistry,
    api_options: ApiOptions,
    /// Decides when a hygiene pass is due and runs it off the reactor. Off
    /// unless the host asked for it — see [`with_hygiene`](Self::with_hygiene).
    hygiene: HygieneScheduler,
}

impl KaeruMemory {
    /// Cross-initiative memory (no active initiative; reads span every project).
    pub fn new(store: Arc<Store>) -> Self {
        let hygiene = HygieneScheduler::new(Arc::clone(&store), false);
        Self {
            store,
            initiative: None,
            clouds: CloudRegistry::default(),
            hygiene,
            api_options: ApiOptions::local(),
        }
    }

    /// Memory scoped to one initiative — captures attach to it and reads
    /// default-filter to it, mirroring `kaeru --initiative <name>`.
    pub fn with_initiative(store: Arc<Store>, initiative: impl Into<String>) -> Self {
        let hygiene = HygieneScheduler::new(Arc::clone(&store), false);
        Self {
            store,
            initiative: Some(Arc::from(initiative.into())),
            clouds: CloudRegistry::default(),
            hygiene,
            api_options: ApiOptions::local(),
        }
    }

    /// Memory scoped to one initiative **and** wired to one or more clouds — the
    /// cloud tools (`share` / `pull` / `cloud_recall` / …) become usable, each
    /// targeting a named cloud or the registry's default. The caller (the host
    /// app) owns the config: it builds the [`CloudRegistry`] from its own
    /// endpoints/tokens and hands it in; kaeru just holds and uses it.
    pub fn with_clouds(
        store: Arc<Store>,
        initiative: impl Into<String>,
        clouds: CloudRegistry,
    ) -> Self {
        let hygiene = HygieneScheduler::new(Arc::clone(&store), false);
        Self {
            store,
            initiative: Some(Arc::from(initiative.into())),
            clouds,
            hygiene,
            api_options: ApiOptions::local(),
        }
    }

    /// Turns on the background **hygiene** pass for this memory: old
    /// unreferenced journal entries drift to `cold`, untouched unreferenced
    /// `core` nodes drop one step, heavily-referenced nodes rise one step. Only
    /// layers move — nothing is deleted or rewritten, and every move reverses
    /// with one `retention` call.
    ///
    /// Opt-in on purpose. An embedded vault belongs to the host application,
    /// and its first sweep re-layers the whole graph in one go; kaeru does not
    /// do that uninvited. Look before you leap: `kaeru_retention(action="preview_hygiene")` shows
    /// exactly what a pass would move without running one.
    ///
    /// Passes trigger on accumulation (writes since the last pass, `core`
    /// growth, elapsed time) and run on the blocking pool, so a tool call is
    /// never held up. This also starts a sweep ticker, which needs a tokio
    /// reactor — **call it from inside the host's runtime**, and call
    /// [`shutdown_hygiene`](Self::shutdown_hygiene) when the host winds down.
    ///
    /// ```ignore
    /// let mem = KaeruMemory::with_initiative(store, "albert").with_hygiene();
    /// ```
    #[must_use]
    pub fn with_hygiene(mut self) -> Self {
        self.hygiene = HygieneScheduler::new(Arc::clone(&self.store), true);
        self.hygiene.spawn_sweeper();
        self
    }

    /// Stops the hygiene sweeper; a pass already running stops at its next
    /// batch boundary. Every further trigger is a no-op.
    ///
    /// Explicit rather than `Drop`: a `KaeruMemory` is `Clone` and shared, so
    /// dropping one clone must not silently stop the memory's housekeeping.
    pub fn shutdown_hygiene(&self) {
        self.hygiene.shutdown();
    }

    /// Nudge opted-in hygiene after a scoped call. Pending reports are delivered
    /// by the shared context reader, within its ordinary response budget.
    pub(crate) fn after_tool(&self, initiative: Option<&str>) {
        self.hygiene
            .consider(initiative.or(self.initiative.as_deref()));
    }

    /// The full cloud registry — for tools that inspect it (name validation,
    /// soft-link routing, discovery).
    pub(crate) fn cloud_registry(&self) -> &CloudRegistry {
        &self.clouds
    }
}

impl KaeruMemory {
    pub(crate) async fn blocking<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&Store) -> T + Send + 'static,
        T: Send + 'static,
    {
        self.blocking_in(None, f).await
    }
    pub(crate) async fn blocking_in<T, F>(&self, initiative: Option<String>, f: F) -> T
    where
        F: FnOnce(&Store) -> T + Send + 'static,
        T: Send + 'static,
    {
        let store = self.store.clone();
        let init: Option<String> =
            initiative.or_else(|| self.initiative.as_deref().map(str::to_owned));
        spawn_blocking(move || store.scoped(init.as_deref(), f))
            .await
            .expect("kaeru store task panicked")
    }
}

impl KaeruMemory {
    /// Install the 26 local memory operations.
    pub fn install<M: CompletionModel>(
        &self,
        b: AgentBuilder<M, (), NoToolConfig>,
    ) -> AgentBuilder<M, (), WithBuilderTools> {
        self.chain_local(b)
    }
    /// Install the full 35-operation memory API.
    pub fn install_with_cloud<M: CompletionModel>(
        &self,
        b: AgentBuilder<M, (), NoToolConfig>,
    ) -> AgentBuilder<M, (), WithBuilderTools> {
        self.chain_cloud(self.chain_local(b))
    }
}

pub(crate) fn resolve(store: &Store, reference: &str) -> Result<String, String> {
    resolve_node(
        store,
        &NodeRef::NameOrId(reference.into()),
        &ApiOptions::local(),
    )
    .map_err(|e| e.message)
}
