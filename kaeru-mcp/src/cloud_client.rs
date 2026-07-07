//! Thin async HTTP client to the shared `kaeru-cloud` service.
//!
//! The local daemon is the agent's only surface; for the nodes an
//! initiative chooses to share it proxies into the cloud over this client.
//! Methods return `(status_code, body_text)` and leave JSON parsing to the
//! caller (`tools::cloud`), keeping the client dumb. Bearer auth is sent on
//! every request; an empty token still sends `Bearer ` (the cloud treats an
//! empty *expected* token as auth-disabled).

use std::collections::HashMap;
use std::time::Duration;

use serde_json::Value;

/// Hard ceiling on any single cloud request. Without it a dead or
/// black-holed connection blocks the calling MCP tool indefinitely —
/// reqwest sets no total-request timeout by default.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Ceiling on TCP connect alone, so an unreachable host fails fast
/// instead of waiting out the OS connect timeout.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

/// Holds the cloud base URL, the bearer token, and a reusable reqwest
/// client (cheap to clone — it shares a connection pool internally).
#[derive(Clone)]
pub struct CloudClient {
    base_url: String,
    token: String,
    client: reqwest::Client,
}

impl CloudClient {
    pub fn new(base_url: String, token: String) -> Self {
        // `Client::builder()` only fails when TLS/system config is broken;
        // fall back to the default client rather than panicking the daemon —
        // a cloud client without timeouts still beats no daemon at all.
        let client = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .connect_timeout(CONNECT_TIMEOUT)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            token,
            client,
        }
    }

    /// `POST /api/v1/nodes` — push a shared node.
    pub async fn post_node(&self, body: &Value) -> Result<(u16, String), String> {
        let url = format!("{}/api/v1/nodes", self.base_url);
        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.token)
            .json(body)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let code = resp.status().as_u16();
        let text = resp.text().await.map_err(|e| e.to_string())?;
        Ok((code, text))
    }

    /// `GET /api/v1/nodes/{id}` — fetch a node's full record.
    pub async fn get_node(&self, id: &str) -> Result<(u16, String), String> {
        let url = format!("{}/api/v1/nodes/{id}", self.base_url);
        self.get(&url).await
    }

    /// `POST /api/v1/edges` — push an edge between two shared nodes.
    pub async fn post_edge(&self, body: &Value) -> Result<(u16, String), String> {
        let url = format!("{}/api/v1/edges", self.base_url);
        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.token)
            .json(body)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let code = resp.status().as_u16();
        let text = resp.text().await.map_err(|e| e.to_string())?;
        Ok((code, text))
    }

    /// `GET /api/v1/initiatives/{name}/nodes` — list shared briefs.
    pub async fn list_initiative(&self, initiative: &str) -> Result<(u16, String), String> {
        let url = format!("{}/api/v1/initiatives/{initiative}/nodes", self.base_url);
        self.get(&url).await
    }

    /// `GET /api/v1/initiatives/{name}/edges` — list shared edges.
    pub async fn list_edges(&self, initiative: &str) -> Result<(u16, String), String> {
        let url = format!("{}/api/v1/initiatives/{initiative}/edges", self.base_url);
        self.get(&url).await
    }

    /// `POST /api/v1/initiatives/{old}/rename` — rename an initiative
    /// team-wide in the shared cloud.
    pub async fn rename_initiative(&self, old: &str, new: &str) -> Result<(u16, String), String> {
        let url = format!("{}/api/v1/initiatives/{old}/rename", self.base_url);
        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.token)
            .json(&serde_json::json!({ "new": new }))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let code = resp.status().as_u16();
        let text = resp.text().await.map_err(|e| e.to_string())?;
        Ok((code, text))
    }

    /// `DELETE /api/v1/initiatives/{name}` — delete an initiative team-wide
    /// from the shared cloud.
    pub async fn delete_initiative(&self, name: &str) -> Result<(u16, String), String> {
        let url = format!("{}/api/v1/initiatives/{name}", self.base_url);
        let resp = self
            .client
            .delete(&url)
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let code = resp.status().as_u16();
        let text = resp.text().await.map_err(|e| e.to_string())?;
        Ok((code, text))
    }

    async fn get(&self, url: &str) -> Result<(u16, String), String> {
        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let code = resp.status().as_u16();
        let text = resp.text().await.map_err(|e| e.to_string())?;
        Ok((code, text))
    }
}

/// Named clouds this daemon can reach, plus which one is the default.
///
/// Multi-cloud support: one local daemon may proxy into several
/// `kaeru-cloud` endpoints (e.g. `family`, `work`). Cloud tools resolve a
/// client by explicit `--cloud <name>`, falling back to the default (or the
/// sole configured cloud). Soft links remember their cloud by name
/// (`dst_store = cloud:<name>`); [`Self::get`] with that parsed name routes
/// resolution back to the right endpoint.
#[derive(Clone, Default)]
pub struct CloudRegistry {
    clients: HashMap<String, CloudClient>,
    default: Option<String>,
}

impl CloudRegistry {
    /// Builds a registry from named clients and an optional default name.
    /// If `default` is unset but exactly one client exists, that one becomes
    /// the implicit default.
    pub fn new(clients: HashMap<String, CloudClient>, default: Option<String>) -> Self {
        let default = default.filter(|d| clients.contains_key(d)).or_else(|| {
            if clients.len() == 1 {
                clients.keys().next().cloned()
            } else {
                None
            }
        });
        Self { clients, default }
    }

    /// No clouds configured at all — cloud tools should report "not configured".
    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }

    /// Whether a cloud of this exact name is configured.
    pub fn contains(&self, name: &str) -> bool {
        self.clients.contains_key(name)
    }

    /// The default cloud's name, if one is resolvable.
    pub fn default_name(&self) -> Option<&str> {
        self.default.as_deref()
    }

    /// Sorted list of configured cloud names — for error messages / discovery.
    pub fn names(&self) -> Vec<&str> {
        let mut ns: Vec<&str> = self.clients.keys().map(String::as_str).collect();
        ns.sort_unstable();
        ns
    }

    /// Resolves a client by explicit name, or the default when `name` is
    /// `None` (the common single-cloud / "just use my default" case).
    /// Returns `None` when the name is unknown or no default resolves.
    pub fn get(&self, name: Option<&str>) -> Option<&CloudClient> {
        match name {
            Some(n) => self.clients.get(n),
            None => self.default.as_ref().and_then(|d| self.clients.get(d)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{CloudClient, CloudRegistry};

    fn reg(names: &[&str], default: Option<&str>) -> CloudRegistry {
        let clients = names
            .iter()
            .map(|n| {
                (
                    n.to_string(),
                    CloudClient::new(format!("http://{n}.test"), String::new()),
                )
            })
            .collect::<HashMap<_, _>>();
        CloudRegistry::new(clients, default.map(String::from))
    }

    #[test]
    fn empty_registry_resolves_nothing() {
        let r = reg(&[], None);
        assert!(r.is_empty());
        assert!(r.get(None).is_none());
        assert!(r.get(Some("family")).is_none());
        assert!(r.default_name().is_none());
    }

    #[test]
    fn single_cloud_is_implicit_default() {
        let r = reg(&["family"], None);
        assert_eq!(r.default_name(), Some("family"));
        assert!(r.get(None).is_some(), "None resolves to the sole cloud");
        assert!(r.get(Some("family")).is_some());
        assert!(r.get(Some("work")).is_none(), "unknown name → None");
    }

    #[test]
    fn explicit_default_among_many() {
        let r = reg(&["family", "work"], Some("work"));
        assert_eq!(r.default_name(), Some("work"));
        assert!(r.get(None).is_some(), "None → the named default");
        assert_eq!(r.names(), vec!["family", "work"], "names sorted");
    }

    #[test]
    fn no_default_among_many_means_none_unresolvable() {
        // Ambiguous: two clouds, no default declared → `get(None)` can't pick.
        let r = reg(&["family", "work"], None);
        assert!(r.default_name().is_none());
        assert!(r.get(None).is_none(), "ambiguous default does not guess");
        assert!(r.get(Some("family")).is_some(), "explicit still works");
    }

    #[test]
    fn bogus_default_falls_back_to_unset() {
        // A `default` naming a cloud that isn't configured is ignored.
        let r = reg(&["family", "work"], Some("ghost"));
        assert!(r.default_name().is_none(), "unknown default dropped");
    }
}
