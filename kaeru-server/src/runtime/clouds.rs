use crate::{
    cloud_client::{CloudClient, CloudRegistry},
    settings::KaeruMcpConfig,
};
use std::collections::HashMap;

pub(super) fn registry(mcp_config: &KaeruMcpConfig, check_versions: bool) -> CloudRegistry {
    // Bridge(s) to the shared kaeru-cloud service. Named clouds come from the
    // clouds TOML; the legacy single `cloud_url`/`cloud_token` pair, if set,
    // is folded in as an extra named cloud. Empty registry → the cloud tools
    // (share / pull / cloud_recall) report the cloud is not configured.
    let mut clients: HashMap<String, CloudClient> = mcp_config
        .clouds
        .iter()
        .filter(|(_, ep)| !ep.url.trim().is_empty())
        .map(|(name, ep)| {
            (
                name.clone(),
                CloudClient::new(name.clone(), ep.url.clone(), ep.token.clone()),
            )
        })
        .collect();

    if !mcp_config.cloud_url.trim().is_empty() {
        // Fold the legacy single-cloud config in under the default name (or
        // "default" when none is set), without clobbering a same-named TOML
        // entry.
        let legacy_name = if mcp_config.default_cloud.trim().is_empty() {
            "default".to_string()
        } else {
            mcp_config.default_cloud.clone()
        };
        let folded = legacy_name.clone();
        clients.entry(legacy_name).or_insert_with(|| {
            CloudClient::new(
                folded,
                mcp_config.cloud_url.clone(),
                mcp_config.cloud_token.clone(),
            )
        });
    }

    // Best-effort mcp <-> cloud version-skew warning. Spawned so a slow or
    // unreachable cloud never delays startup; a version mismatch only warns
    // (the schema is forward-additive), and an unreachable cloud stays quiet
    // here — the first real cloud op surfaces a hard error with more context.
    for (name, client) in clients.iter().filter(|_| check_versions) {
        let (name, client) = (name.clone(), client.clone());
        tokio::spawn(async move {
            match client.fetch_core_version().await {
                Ok(Some(remote)) if remote != kaeru_core::version() => tracing::warn!(
                    cloud = %name,
                    mcp_version = kaeru_core::version(),
                    cloud_version = %remote,
                    "local host and memory-service run different kaeru-core versions — usually fine \
                     (schema is forward-additive), but watch for skew"
                ),
                Ok(_) => {}
                Err(e) => {
                    tracing::debug!(cloud = %name, error = %e, "couldn't check cloud version at startup")
                }
            }
        });
    }

    let default_name =
        (!mcp_config.default_cloud.trim().is_empty()).then(|| mcp_config.default_cloud.clone());
    let clouds = CloudRegistry::new(clients, default_name);

    if clouds.is_empty() {
        tracing::info!("cloud sharing disabled (no clouds configured)");
    } else {
        tracing::info!(
            clouds = ?clouds.names(),
            default = ?clouds.default_name(),
            "cloud sharing enabled"
        );
    }

    clouds
}
