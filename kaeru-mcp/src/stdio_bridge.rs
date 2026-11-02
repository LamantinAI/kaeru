//! Frame-preserving stdio relay to the shared HTTP daemon that owns the vault.
//! Each relay owns one MCP session, echoes the negotiated protocol version,
//! and closes its session on EOF or shutdown without stopping the daemon.
//! Method payloads pass through unchanged; HTTP-only failures become RPC errors.

use std::fs::OpenOptions;
use std::io;
use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;

use fs2::FileExt;
use reqwest::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderValue, RETRY_AFTER};
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::time::sleep;

use crate::settings::{
    MCP_PROTOCOL_HEADER, MCP_SESSION_HEADER as SESSION_HEADER, STDIO_SESSION_CLOSE_TIMEOUT,
};

/// How long to wait for a daemon we just started to begin answering.
const START_TIMEOUT: Duration = Duration::from_secs(20);

pub struct Bridge {
    url: String,
    port: u16,
    token: Option<String>,
    http: Client,
    session: Option<String>,
    protocol_version: Option<String>,
    initialize_request: Option<String>,
}

impl Bridge {
    pub fn new(url: String, port: u16, token: Option<String>) -> Self {
        Bridge {
            url,
            port,
            token: token.filter(|t| !t.trim().is_empty()),
            http: Client::new(),
            session: None,
            protocol_version: None,
            initialize_request: None,
        }
    }

    /// A state-file path keyed to the daemon's port, so every relay that would
    /// start the *same* daemon contends on the *same* file. In the system temp
    /// dir because it is process-lifetime scratch, not vault data.
    fn state_path(&self, suffix: &str) -> PathBuf {
        std::env::temp_dir().join(format!("kaeru-mcp-{}.{suffix}", self.port))
    }

    /// Ensures a daemon is answering, starting one if it is not.
    ///
    /// Starting it here is what makes the bundle work with nothing installed:
    /// a client that has only ever been handed a binary gets a working vault
    /// without the user having to know a daemon exists. If one is already up —
    /// launchd, systemd, or another session that got here first — this finds it
    /// and adds nothing.
    pub async fn ensure_daemon(&self) -> io::Result<()> {
        if self.ping().await {
            tracing::debug!(url = %self.url, "daemon already answering");
            return Ok(());
        }

        // Single-flight. Without this, several relays starting at once each see
        // no daemon and each spawn one; all but the process that wins the port
        // and the RocksDB LOCK then die — silently, for a race no user asked
        // for. An advisory lock lets exactly one relay through the spawn at a
        // time. It releases when this process exits (or the file closes), so a
        // relay that crashes mid-start never wedges the next one.
        let lock_path = self.state_path("start.lock");
        let _lock = tokio::task::spawn_blocking(move || -> io::Result<std::fs::File> {
            // The file is only ever a lock handle — its bytes are never read or
            // written — so truncate(false): do not disturb it, just hold it.
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(false)
                .open(&lock_path)?;
            file.lock_exclusive()?;
            Ok(file)
        })
        .await
        .map_err(io::Error::other)??;

        // Whoever held the lock before us may already have started the daemon —
        // the common case when a burst of clients launches together. Now that we
        // are the one allowed to spawn, re-check, and add nothing if it is up.
        if self.ping().await {
            return Ok(());
        }

        let exe = std::env::current_exe()?;
        // The daemon outlives this relay, so its output cannot go to a pipe that
        // closes when we exit — but discarding it means a failed start (a LOCK
        // it lost, a bad config) leaves no trace at all. Send it to a known log
        // file instead, truncated per start so it stays bounded, and name the
        // path so it can be found.
        let log_path = self.state_path("startup.log");
        let log = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&log_path)?;
        tracing::info!(
            url = %self.url, ?exe, log = %log_path.display(),
            "no daemon answering — starting one; its output goes to the log path"
        );
        // Detached: this bridge exits with its client, and taking the vault's
        // owner down with one session would strand every other session.
        Command::new(exe)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::from(log))
            .spawn()?;

        let deadline = tokio::time::Instant::now() + START_TIMEOUT;
        while tokio::time::Instant::now() < deadline {
            sleep(Duration::from_millis(250)).await;
            if self.ping().await {
                return Ok(());
            }
        }
        Err(io::Error::new(
            io::ErrorKind::TimedOut,
            format!(
                "started a daemon but {} never answered — see {}",
                self.url,
                log_path.display()
            ),
        ))
    }

    /// Whether something is listening and speaking HTTP at the mount path. A
    /// bare GET is enough — any status at all proves a daemon is there, and
    /// the transport answers a GET differently from a POST by design.
    async fn ping(&self) -> bool {
        self.http
            .get(&self.url)
            .timeout(Duration::from_millis(800))
            .send()
            .await
            .is_ok()
    }

    /// Reads newline-delimited JSON-RPC from stdin, forwards each message to
    /// the daemon, and writes whatever comes back to stdout.
    pub async fn run(mut self) -> io::Result<()> {
        self.run_io(
            BufReader::new(tokio::io::stdin()),
            tokio::io::stdout(),
            crate::shutdown(tokio_util::sync::CancellationToken::new()),
        )
        .await
    }

    async fn run_io<I: AsyncBufRead + Unpin, O: AsyncWrite + Unpin>(
        &mut self,
        input: I,
        output: O,
        stop: impl Future<Output = ()>,
    ) -> io::Result<()> {
        let result = tokio::select! {
            result = self.relay(input, output) => result,
            () = stop => Ok(()),
        };
        self.close_session().await;
        result
    }

    async fn relay<I: AsyncBufRead + Unpin, O: AsyncWrite + Unpin>(
        &mut self,
        input: I,
        mut out: O,
    ) -> io::Result<()> {
        let mut lines = input.lines();

        while let Some(line) = lines.next_line().await? {
            if line.trim().is_empty() {
                continue;
            }
            match self.forward(&line).await {
                Ok(frames) => {
                    for frame in frames {
                        out.write_all(frame.as_bytes()).await?;
                        out.write_all(b"\n").await?;
                    }
                    out.flush().await?;
                }
                // A transport failure is not a protocol answer, so there is
                // nothing meaningful to write back to a client that is waiting
                // on one. Log it and keep the pipe open — the next request may
                // well succeed, and closing would kill the session outright.
                Err(e) => tracing::warn!(error = %e, "forwarding to the daemon failed"),
            }
        }
        Ok(())
    }

    async fn forward(&mut self, body: &str) -> Result<Vec<String>, reqwest::Error> {
        let method = serde_json::from_str::<serde_json::Value>(body)
            .ok()
            .and_then(|value| value["method"].as_str().map(str::to_owned));
        if self.session.is_none()
            && self.initialize_request.is_some()
            && !matches!(
                method.as_deref(),
                Some("initialize" | "notifications/initialized" | "notifications/cancelled")
            )
            && let Some(error) = self.restore_session(body).await?
        {
            return Ok(error);
        }
        let response = self.send_http(body).await?;
        if response.status() == reqwest::StatusCode::NOT_FOUND
            && self.session.is_some()
            && self.initialize_request.is_some()
            && method.as_deref() != Some("initialize")
        {
            // Only this transport rejection proves the operation never ran.
            // A timeout, RPC error or unrelated HTTP 404 must not replay a write.
            let text = response.text().await?;
            if text.trim() == "Not Found: Session not found" {
                if method.as_deref() == Some("notifications/cancelled") {
                    return Ok(Vec::new());
                }
                self.session = None;
                self.protocol_version = None;
                if let Some(error) = self.restore_session(body).await? {
                    return Ok(error);
                }
                let response = self.send_http(body).await?;
                return self.finish_response(body, response).await;
            }
            return Ok(Self::http_error(body, 404, None, text));
        }
        self.finish_response(body, response).await
    }

    async fn restore_session(
        &mut self,
        original: &str,
    ) -> Result<Option<Vec<String>>, reqwest::Error> {
        let Some(initialize) = self.initialize_request.clone() else {
            return Ok(None);
        };
        self.protocol_version = None;
        let response = self.send_http(&initialize).await?;
        let frames = self.finish_response(&initialize, response).await?;
        if self.session.is_none() || self.protocol_version.is_none() {
            let id = serde_json::from_str::<serde_json::Value>(original)
                .ok()
                .and_then(|value| value.get("id").cloned());
            let mut errors: Vec<_> = frames
                .into_iter()
                .filter_map(|frame| {
                    let mut value: serde_json::Value = serde_json::from_str(&frame).ok()?;
                    if !value["error"].is_object() {
                        return None;
                    }
                    value["id"] = id.clone()?;
                    Some(value.to_string())
                })
                .collect();
            if errors.is_empty()
                && let Some(id) = id
            {
                let error =
                    rmcp::ErrorData::internal_error("MCP session reinitialization failed", None);
                errors.push(serde_json::json!({"jsonrpc":"2.0","id":id,"error":error}).to_string());
            }
            self.close_session().await;
            return Ok(Some(errors));
        }
        let initialized = r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;
        let response = self.send_http(initialized).await?;
        if !response.status().is_success() {
            let errors = self.finish_response(original, response).await?;
            self.close_session().await;
            return Ok(Some(errors));
        }
        let _ = response.bytes().await?;
        Ok(None)
    }

    async fn send_http(&self, body: &str) -> Result<reqwest::Response, reqwest::Error> {
        let mut req = self
            .http
            .post(&self.url)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            // The transport refuses a request that does not accept both.
            .header(
                ACCEPT,
                HeaderValue::from_static("application/json, text/event-stream"),
            )
            .body(body.to_string());
        if let Some(s) = &self.session {
            req = req.header(SESSION_HEADER, s);
        }
        if let Some(version) = &self.protocol_version {
            req = req.header(MCP_PROTOCOL_HEADER, version);
        }
        if let Some(t) = &self.token {
            req = req.bearer_auth(t);
        }

        req.send().await
    }

    async fn finish_response(
        &mut self,
        body: &str,
        resp: reqwest::Response,
    ) -> Result<Vec<String>, reqwest::Error> {
        if !resp.status().is_success() {
            // A stdio client cannot see the HTTP status. Return a correlated
            // RPC error instead of forwarding a plain-text 503 as JSON.
            let retry = resp
                .headers()
                .get(RETRY_AFTER)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok());
            let status = resp.status().as_u16();
            let text = resp.text().await?;
            return Ok(Self::http_error(body, status, retry, text));
        }
        // The session id arrives on the initialize response and must be echoed
        // on everything after it.
        if self.session.is_none()
            && let Some(v) = resp.headers().get(SESSION_HEADER)
            && let Ok(v) = v.to_str()
        {
            self.session = Some(v.to_string());
        }
        let content_type = resp
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        let text = resp.text().await?;

        let frames = if content_type.starts_with("text/event-stream") {
            parse_sse(&text)
        } else if text.trim().is_empty() {
            // 202 to a notification: nothing to relay, and writing an empty
            // line would look like a malformed frame to the client.
            Vec::new()
        } else {
            vec![text]
        };
        if self.protocol_version.is_none()
            && serde_json::from_str::<serde_json::Value>(body)
                .ok()
                .is_some_and(|value| value["method"] == "initialize")
        {
            self.protocol_version = frames.iter().find_map(|frame| {
                serde_json::from_str::<serde_json::Value>(frame)
                    .ok()?
                    .pointer("/result/protocolVersion")?
                    .as_str()
                    .map(str::to_owned)
            });
            if self.session.is_some() && self.protocol_version.is_some() {
                self.initialize_request = Some(body.to_owned());
            }
        }
        Ok(frames)
    }

    fn http_error(body: &str, status: u16, retry: Option<u64>, text: String) -> Vec<String> {
        if serde_json::from_str::<serde_json::Value>(&text)
            .ok()
            .is_some_and(|value| value["jsonrpc"] == "2.0" && value["error"].is_object())
        {
            return vec![text];
        }
        let id = serde_json::from_str::<serde_json::Value>(body)
            .ok()
            .and_then(|value| value.get("id").cloned());
        let error = rmcp::ErrorData::internal_error(
            format!("MCP HTTP backend returned {status}"),
            Some(serde_json::json!({"http_status":status,"retry_after_seconds":retry})),
        );
        id.map(|id| vec![serde_json::json!({"jsonrpc":"2.0","id":id,"error":error}).to_string()])
            .unwrap_or_default()
    }

    async fn close_session(&mut self) {
        let Some(id) = self.session.take() else {
            return;
        };
        let mut request = self
            .http
            .delete(&self.url)
            .header(SESSION_HEADER, id)
            .timeout(STDIO_SESSION_CLOSE_TIMEOUT);
        if let Some(version) = &self.protocol_version {
            request = request.header(MCP_PROTOCOL_HEADER, version);
        }
        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }
        match request.send().await {
            Ok(response)
                if response.status().is_success()
                    || response.status() == reqwest::StatusCode::NOT_FOUND => {}
            Ok(response) => tracing::warn!(status = %response.status(), "MCP session close failed"),
            Err(error) => tracing::warn!(%error, "MCP session close failed"),
        }
    }
}

/// Pulls the JSON payloads out of an SSE body.
///
/// Only `data:` carries protocol; `event:`, `id:` and comments are transport
/// bookkeeping the stdio client neither sees nor needs.
fn parse_sse(body: &str) -> Vec<String> {
    body.lines()
        .filter_map(|l| l.strip_prefix("data:"))
        .map(|d| d.trim().to_string())
        .filter(|d| !d.is_empty())
        .collect()
}

#[cfg(test)]
mod tests;
