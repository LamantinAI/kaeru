//! Thin async HTTP client to the shared `kaeru-cloud` service.
//!
//! The local daemon is the agent's only surface; for the nodes an
//! initiative chooses to share it proxies into the cloud over this client.
//! Methods return `(status_code, body_text)` and leave JSON parsing to the
//! caller (`tools::cloud`), keeping the client dumb. Bearer auth is sent on
//! every request; an empty token still sends `Bearer ` (the cloud treats an
//! empty *expected* token as auth-disabled).

use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::Value;

/// Hard ceiling on any single cloud request. Without it a dead or
/// black-holed connection blocks the calling MCP tool indefinitely —
/// reqwest sets no total-request timeout by default.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Ceiling on TCP connect alone, so an unreachable host fails fast
/// instead of waiting out the OS connect timeout.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

/// Holds the cloud's configured name, its base URL, the bearer token, and a
/// reusable reqwest client (cheap to clone — it shares a connection pool
/// internally).
///
/// `Debug` prints the name and endpoint but never the bearer token — a client
/// ends up in error text and log lines, and a credential should not ride
/// along.
///
/// The **name** is carried here rather than passed alongside because every
/// result and error has to be able to say which cloud it touched, and a
/// parameter threaded through a dozen signatures is a parameter some call
/// site will forget. In a multi-cloud setup an answer that does not name its
/// cloud is indistinguishable from an answer about a different one — that is
/// the whole of #65.
#[derive(Clone)]
pub struct CloudClient {
    name: String,
    base_url: String,
    token: String,
    client: reqwest::Client,
}

impl std::fmt::Debug for CloudClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CloudClient")
            .field("name", &self.name)
            .field("base_url", &self.base_url)
            .finish_non_exhaustive()
    }
}

impl CloudClient {
    pub fn new(name: String, base_url: String, token: String) -> Self {
        // `Client::builder()` only fails when TLS/system config is broken;
        // fall back to the default client rather than panicking the daemon —
        // a cloud client without timeouts still beats no daemon at all.
        let client = reqwest::Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .connect_timeout(CONNECT_TIMEOUT)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self {
            name,
            base_url: base_url.trim_end_matches('/').to_string(),
            token,
            client,
        }
    }

    /// The cloud's configured name — what every message about this client
    /// should call it.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The endpoint, for a diagnostic that needs to be unambiguous about
    /// *where* rather than only *which*.
    pub fn base_url(&self) -> &str {
        &self.base_url
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

    /// `GET /health` — the cloud's reported `kaeru_core` version, or `None`
    /// when the field is absent. Unauthenticated; used at startup to warn on a
    /// mcp <-> cloud version skew.
    pub async fn fetch_core_version(&self) -> Result<Option<String>, String> {
        let url = format!("{}/health", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let json: Value = resp.json().await.map_err(|e| e.to_string())?;
        Ok(json
            .get("core_version")
            .and_then(|v| v.as_str())
            .map(String::from))
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
    /// `DELETE /api/v1/nodes/{id}` — retract a node from the cloud.
    ///
    /// Bi-temporal on the far side: the node leaves every read at NOW while
    /// its history stays intact. Idempotent, so a retry after a dropped
    /// connection is not reported as a failure.
    pub async fn delete_node(&self, id: &str) -> Result<(u16, String), String> {
        let url = format!("{}/api/v1/nodes/{id}", self.base_url);
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

    /// `GET /api/v1/initiatives` — every initiative the cloud knows, with its
    /// node counts. Used to tell "this initiative is empty here" apart from
    /// "this cloud has never heard of it".
    pub async fn list_initiatives(&self) -> Result<(u16, String), String> {
        let url = format!("{}/api/v1/initiatives", self.base_url);
        self.get(&url).await
    }

    /// One page of an initiative's shared briefs. The listing is bounded
    /// server-side — an unpaged one meant 188 KB arriving in a single response
    /// that did not fit in the caller's context (#67).
    pub async fn list_initiative(
        &self,
        initiative: &str,
        limit: usize,
        offset: usize,
    ) -> Result<(u16, String), String> {
        let url = format!(
            "{}/api/v1/initiatives/{initiative}/nodes?limit={limit}&offset={offset}",
            self.base_url
        );
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
    /// Unix seconds when this registry was built — i.e. when `clouds.toml`
    /// was last read. `None` for a registry assembled in a test.
    ///
    /// Carried because the file is read once, at daemon startup, and a client
    /// reconnect does not respawn the process: an edited TOML keeps producing
    /// errors about the old configuration with nothing to suggest the file was
    /// never re-read. Half an hour went into exactly that. Saying when the
    /// config was loaded turns a baffling error into an obvious one.
    loaded_at: Option<f64>,
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
        let loaded_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .ok()
            .map(|d| d.as_secs() as f64);
        Self {
            clients,
            default,
            loaded_at,
        }
    }

    /// When `clouds.toml` was read, as unix seconds — see [`Self::loaded_at`].
    pub fn loaded_at(&self) -> Option<f64> {
        self.loaded_at
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
    ///
    /// Prefer [`Self::resolve`] for anything that acts on a cloud — this one
    /// silently picks the default, which is the behaviour #65 is about.
    pub fn get(&self, name: Option<&str>) -> Option<&CloudClient> {
        match name {
            Some(n) => self.clients.get(n),
            None => self.default.as_ref().and_then(|d| self.clients.get(d)),
        }
    }

    /// Resolves the cloud an operation should act on, refusing to guess when
    /// a guess could be wrong.
    ///
    /// With one cloud configured there is nothing to disambiguate, so an
    /// unnamed call resolves silently — that is the overwhelmingly common
    /// setup and adding ceremony to it would buy nothing. With **several**,
    /// an unnamed call is refused rather than routed to the default.
    ///
    /// The reason is that the default was invisible in both directions: a read
    /// answered by one cloud while the nodes lived in another is
    /// indistinguishable from an empty answer, and the same silence sits under
    /// `delete_initiative`, whose own description says "team-wide, removes it
    /// for everyone". A refusal costs one retry; a misrouted destructive verb
    /// costs a team's initiative, and the cloud has no undo.
    ///
    /// The error is a finished sentence rather than a code, because every
    /// caller would otherwise write the same one slightly differently.
    pub fn resolve(&self, name: Option<&str>) -> Result<&CloudClient, String> {
        if self.clients.is_empty() {
            return Err(
                "no cloud is configured — add one to `clouds.toml` (the daemon reads it at \
                 startup) and restart the daemon."
                    .to_string(),
            );
        }
        match name {
            Some(n) => self.clients.get(n).ok_or_else(|| {
                format!(
                    "no cloud named `{n}` is configured. Available: {}. (`clouds.toml` is read \
                     once at daemon startup — if you just edited it, restart the daemon; a client \
                     reconnect does not respawn it.)",
                    self.names().join(", ")
                )
            }),
            None => match (self.clients.len(), self.default.as_ref()) {
                // One cloud: nothing to disambiguate.
                (1, _) => Ok(self.clients.values().next().expect("len == 1")),
                (_, Some(d)) => Err(format!(
                    "several clouds are configured ({}) — name the one you mean with `cloud`. \
                     Not defaulting to `{d}`: an operation that reaches the wrong cloud cannot \
                     be undone there.",
                    self.names().join(", ")
                )),
                (_, None) => Err(format!(
                    "several clouds are configured ({}) and none is the default — name the one \
                     you mean with `cloud`.",
                    self.names().join(", ")
                )),
            },
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
                    CloudClient::new(n.to_string(), format!("http://{n}.test"), String::new()),
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

    /// One cloud means no ambiguity — an unnamed call must not acquire
    /// ceremony it cannot benefit from.
    #[test]
    fn a_single_cloud_resolves_without_being_named() {
        let r = reg(&["only"], None);
        assert!(r.resolve(None).is_ok());
        assert_eq!(r.resolve(None).unwrap().name(), "only");
    }

    /// The case #65 is about: with several clouds an unnamed call is refused
    /// rather than sent to the default, and the refusal lists the choices.
    #[test]
    fn several_clouds_refuse_to_be_guessed() {
        let r = reg(&["alpha", "beta"], Some("alpha"));
        let err = r.resolve(None).unwrap_err();
        assert!(err.contains("alpha, beta"), "{err}");
        assert!(err.contains("cannot"), "says why it refuses: {err}");
        // Naming one still works.
        assert_eq!(r.resolve(Some("beta")).unwrap().name(), "beta");
    }

    /// An unknown name is a different mistake from an unnamed call, and gets
    /// a different answer.
    #[test]
    fn an_unknown_name_lists_what_exists() {
        let r = reg(&["alpha", "beta"], Some("alpha"));
        let err = r.resolve(Some("gamma")).unwrap_err();
        assert!(err.contains("`gamma`"), "{err}");
        assert!(err.contains("alpha, beta"), "{err}");
    }

    /// No clouds at all is a configuration problem, and says so.
    #[test]
    fn no_clouds_says_it_is_unconfigured() {
        let r = reg(&[], None);
        assert!(
            r.resolve(None)
                .unwrap_err()
                .contains("no cloud is configured")
        );
    }

    /// A client knows its own name, which is what lets every message say
    /// which cloud it touched without threading a parameter everywhere.
    #[test]
    fn a_client_carries_its_name() {
        let r = reg(&["work"], None);
        assert_eq!(r.get(None).unwrap().name(), "work");
    }
}
