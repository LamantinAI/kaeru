use std::process::Command;

struct Daemon(std::process::Child);
impl Drop for Daemon {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn rpc_frame(body: &str) -> serde_json::Value {
    serde_json::from_str(body).unwrap_or_else(|_| {
        body.lines()
            .filter_map(|line| line.strip_prefix("data:"))
            .find_map(|line| serde_json::from_str(line.trim()).ok())
            .expect("RPC response")
    })
}

async fn rpc(
    client: &reqwest::Client,
    url: &str,
    sid: Option<&str>,
    body: serde_json::Value,
) -> reqwest::Response {
    let mut request = client
        .post(url)
        .header("accept", "application/json, text/event-stream")
        .header("mcp-protocol-version", "2025-06-18")
        .json(&body);
    if let Some(sid) = sid {
        request = request.header("mcp-session-id", sid);
    }
    request.send().await.unwrap()
}

#[tokio::test]
async fn real_daemon_expires_transport_sessions_without_removing_graph_records() {
    use serde_json::json;
    use std::time::Duration;
    let directory = tempfile::tempdir().unwrap();
    let config = directory.path().join("web.toml");
    std::fs::write(&config, "enabled = false\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&config, std::fs::Permissions::from_mode(0o600)).unwrap();
    }
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let mut command = Command::new(env!("CARGO_BIN_EXE_kaeru-mcp"));
    for (key, _) in std::env::vars().filter(|(key, _)| key.starts_with("KAERU_")) {
        command.env_remove(key);
    }
    let log = std::fs::File::create(directory.path().join("daemon.log")).unwrap();
    let mut daemon = Daemon(
        command
            .env("KAERU_VAULT_PATH", directory.path().join("vault"))
            .env(
                "KAERU_MCP_CLOUDS_FILE",
                directory.path().join("clouds.toml"),
            )
            .env("KAERU_MCP_WEB_CONFIG_FILE", &config)
            .env("KAERU_MCP_LISTEN_PORT", port.to_string())
            .env("KAERU_MCP_LISTEN_ADDRESS", "127.0.0.1")
            .env("KAERU_MCP_KEEP_ALIVE_SECS", "1")
            .env("KAERU_MCP_MAX_SESSIONS", "1")
            .env("KAERU_MCP_UPDATE_CHECK", "0")
            .env("KAERU_MCP_HYGIENE_ENABLE", "0")
            .env("KAERU_READ_TRACE_ENABLED", "false")
            .stdout(std::process::Stdio::null())
            .stderr(log)
            .spawn()
            .unwrap(),
    );
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
    let url = format!("http://127.0.0.1:{port}/mcp");
    let ready = tokio::time::timeout(Duration::from_secs(15), async {
        loop {
            assert!(
                daemon.0.try_wait().unwrap().is_none(),
                "{}",
                std::fs::read_to_string(directory.path().join("daemon.log")).unwrap()
            );
            if client
                .get(format!("http://127.0.0.1:{port}/auth/session"))
                .send()
                .await
                .is_ok()
            {
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await;
    assert!(
        ready.is_ok(),
        "{}",
        std::fs::read_to_string(directory.path().join("daemon.log")).unwrap()
    );
    let initialize = json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"expiry-test","version":"1"}}});
    let response = rpc(&client, &url, None, initialize.clone()).await;
    assert_eq!(response.status(), 200);
    let old = response.headers()["mcp-session-id"]
        .to_str()
        .unwrap()
        .to_owned();
    let _ = response.text().await.unwrap();
    let _ = rpc(
        &client,
        &url,
        Some(&old),
        json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
    )
    .await;
    let response=rpc(&client,&url,Some(&old),json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"task","arguments":{"initiative":"expiry-test","body":"Knowledge survives MCP session expiry"}}})).await;
    let value = rpc_frame(&response.text().await.unwrap());
    let id = value["result"]["structuredContent"]["data"]["primary"]["id"]
        .as_str()
        .expect("task committed")
        .to_owned();
    tokio::time::sleep(Duration::from_secs(3)).await;
    let response = rpc(
        &client,
        &url,
        Some(&old),
        json!({"jsonrpc":"2.0","id":3,"method":"ping"}),
    )
    .await;
    assert_eq!(response.status(), 404);
    let response = rpc(&client, &url, None, initialize).await;
    assert_eq!(
        response.status(),
        200,
        "expired session did not free the only capacity slot"
    );
    let new = response.headers()["mcp-session-id"]
        .to_str()
        .unwrap()
        .to_owned();
    let _ = response.text().await.unwrap();
    assert_ne!(old, new);
    let _ = rpc(
        &client,
        &url,
        Some(&new),
        json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
    )
    .await;
    let response=rpc(&client,&url,Some(&new),json!({"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"read","arguments":{"initiative":"expiry-test","node":{"id":id}}}})).await;
    let body = response.text().await.unwrap();
    assert!(
        body.contains("Knowledge survives MCP session expiry"),
        "{body}"
    );
}

#[test]
fn invalid_session_capacity_fails_before_opening_the_vault() {
    let oversized = (tokio::sync::Semaphore::MAX_PERMITS + 1).to_string();
    for capacity in ["0", oversized.as_str()] {
        let directory = tempfile::tempdir().unwrap();
        let vault = directory.path().join("must-not-be-created");
        let mut command = Command::new(env!("CARGO_BIN_EXE_kaeru-mcp"));
        for (key, _) in std::env::vars().filter(|(key, _)| key.starts_with("KAERU_")) {
            command.env_remove(key);
        }
        let result = command
            .env("KAERU_VAULT_PATH", &vault)
            .env(
                "KAERU_MCP_CLOUDS_FILE",
                directory.path().join("clouds.toml"),
            )
            .env(
                "KAERU_MCP_WEB_CONFIG_FILE",
                directory.path().join("web.toml"),
            )
            .env("KAERU_MCP_MAX_SESSIONS", capacity)
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("KAERU_MCP_MAX_SESSIONS"));
        assert!(!vault.exists(), "invalid capacity opened the vault");
    }
}
