//! `kaeru-mcp --stdio` — a relay from a client that speaks stdio to the daemon
//! that owns the vault.
//!
//! kaeru is deliberately not a stdio server: the substrate is a single-writer
//! RocksDB, so a subprocess per agent session would race for the LOCK and the
//! second one would lose. That decision is right and this module does not
//! revisit it — but it left every stdio-only MCP client unable to reach kaeru
//! at all, which is most of them.
//!
//! So: the client spawns *this*, and this forwards to the one daemon. Each
//! session gets a process of its own, as stdio clients expect, and the vault
//! still has exactly one writer.
//!
//! **It relays frames rather than modelling them.** A proxy built out of an
//! MCP client and server would have to understand every method to pass it on,
//! and would silently drop whatever it had not been taught — a new method, a
//! notification, a capability added upstream. This forwards the JSON-RPC that
//! arrives and returns what comes back, so it stays correct as the protocol
//! grows.

use std::fs::OpenOptions;
use std::io;
use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;

use fs2::FileExt;
use reqwest::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderValue};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::time::sleep;

/// Header the streamable-HTTP transport uses to bind a client to its session.
const SESSION_HEADER: &str = "mcp-session-id";

/// How long to wait for a daemon we just started to begin answering.
const START_TIMEOUT: Duration = Duration::from_secs(20);

pub struct Bridge {
    url: String,
    port: u16,
    token: Option<String>,
    http: Client,
    session: Option<String>,
}

impl Bridge {
    pub fn new(url: String, port: u16, token: Option<String>) -> Self {
        Bridge {
            url,
            port,
            token: token.filter(|t| !t.trim().is_empty()),
            http: Client::new(),
            session: None,
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
        let mut lines = BufReader::new(tokio::io::stdin()).lines();
        let mut out = tokio::io::stdout();

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
        if let Some(t) = &self.token {
            req = req.bearer_auth(t);
        }

        let resp = req.send().await?;
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

        Ok(if content_type.starts_with("text/event-stream") {
            parse_sse(&text)
        } else if text.trim().is_empty() {
            // 202 to a notification: nothing to relay, and writing an empty
            // line would look like a malformed frame to the client.
            Vec::new()
        } else {
            vec![text]
        })
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
mod tests {
    use super::parse_sse;

    #[test]
    fn only_data_lines_survive_an_sse_frame() {
        let body = ": keep-alive\nevent: message\nid: 3\ndata: {\"jsonrpc\":\"2.0\",\"id\":1}\n\n";
        assert_eq!(parse_sse(body), vec![r#"{"jsonrpc":"2.0","id":1}"#]);
    }

    #[test]
    fn a_stream_carrying_several_messages_relays_all_of_them() {
        let body = "data: {\"id\":1}\n\ndata: {\"id\":2}\n\n";
        assert_eq!(parse_sse(body), vec![r#"{"id":1}"#, r#"{"id":2}"#]);
    }

    #[test]
    fn a_keep_alive_only_frame_relays_nothing() {
        assert!(parse_sse(": ping\n\n").is_empty());
    }
}
