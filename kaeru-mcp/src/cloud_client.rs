//! Thin async HTTP client to the shared `kaeru-cloud` service.
//!
//! The local daemon is the agent's only surface; for the nodes an
//! initiative chooses to share it proxies into the cloud over this client.
//! Methods return `(status_code, body_text)` and leave JSON parsing to the
//! caller (`tools::cloud`), keeping the client dumb. Bearer auth is sent on
//! every request; an empty token still sends `Bearer ` (the cloud treats an
//! empty *expected* token as auth-disabled).

use serde_json::Value;

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
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            token,
            client: reqwest::Client::new(),
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

    /// `GET /api/v1/initiatives/{name}/nodes` — list shared briefs.
    pub async fn list_initiative(&self, initiative: &str) -> Result<(u16, String), String> {
        let url = format!("{}/api/v1/initiatives/{initiative}/nodes", self.base_url);
        self.get(&url).await
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
