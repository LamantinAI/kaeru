//! Actual MCP HTTP sessions export and retrieve archives without filesystem tools.
use std::fs;
use std::io::{Cursor, Read};
use std::net::TcpListener;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

use base64::{Engine, engine::general_purpose::STANDARD};
use kaeru_core::config::amg::AmgLimits;
use kaeru_core::{
    KaeruConfig, Store, amg,
    api::{self, ApiOptions},
};
use reqwest::Client;
use rmcp::model::ProtocolVersion;
use serde_json::{Value, json};

struct Process(Child);
impl Drop for Process {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
fn parse(text: &str) -> Value {
    serde_json::from_str(text).unwrap_or_else(|_| {
        text.lines()
            .filter_map(|line| {
                line.strip_prefix("data: ")
                    .or_else(|| line.strip_prefix("data:"))
            })
            .filter_map(|data| serde_json::from_str::<Value>(data).ok())
            .find(|data| data.get("id").is_some())
            .unwrap_or_else(|| panic!("missing JSON-RPC response: {text}"))
    })
}
async fn rpc(
    client: &Client,
    url: &str,
    session: &str,
    id: u32,
    method: &str,
    params: Value,
) -> Value {
    let response = client
        .post(url)
        .header("accept", "application/json, text/event-stream")
        .header("mcp-session-id", session)
        .header("mcp-protocol-version", ProtocolVersion::LATEST.to_string())
        .json(&json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}))
        .send()
        .await
        .unwrap();
    assert!(
        response.status().is_success(),
        "{method}: {}",
        response.text().await.unwrap()
    );
    parse(&response.text().await.unwrap())
}
#[tokio::test]
async fn mcp_delivers_both_export_formats_as_resources_and_http_uses_the_same_contract() {
    let dir = tempfile::tempdir().unwrap();
    let vault = dir.path().join("vault");
    let artifacts = dir.path().join("exports");
    let exact = "Полный исходник.\n```\n<parameter name=\"body\">literal source bytes.";
    {
        let store = Store::open_with_config(KaeruConfig {
            vault_path: vault.clone(),
            ..Default::default()
        })
        .unwrap();
        let result = api::execute(
            &store,
            "reference",
            json!({"initiative":"alpha","name":"Original","source":{"text":exact}}),
            &ApiOptions::local(),
        );
        assert!(!result.is_error());
    }
    let port = {
        let socket = TcpListener::bind("127.0.0.1:0").unwrap();
        socket.local_addr().unwrap().port()
    };
    let log = fs::File::create(dir.path().join("daemon.log")).unwrap();
    let mut command = Command::new(env!("CARGO_BIN_EXE_kaeru-mcp"));
    for (key, _) in std::env::vars().filter(|(key, _)| key.starts_with("KAERU_")) {
        command.env_remove(key);
    }
    let child = command
        .env("KAERU_VAULT_PATH", &vault)
        .env("KAERU_MCP_LISTEN_ADDRESS", "127.0.0.1")
        .env("KAERU_MCP_LISTEN_PORT", port.to_string())
        .env("KAERU_MCP_WEB_CONFIG_FILE", dir.path().join("web.toml"))
        .env("KAERU_MCP_CLOUDS_FILE", dir.path().join("clouds.toml"))
        .env("KAERU_MCP_ARTIFACT_ROOT", &artifacts)
        .env("KAERU_MCP_VIZ_INITIATIVES", "alpha")
        .env("KAERU_MCP_VIZ_INCLUDE_LOCAL", "1")
        .env("KAERU_MCP_HYGIENE_ENABLE", "0")
        .env("KAERU_MCP_UPDATE_CHECK", "0")
        .env("KAERU_READ_TRACE_ENABLED", "false")
        .stdout(Stdio::null())
        .stderr(log)
        .spawn()
        .unwrap();
    let _process = Process(child);
    let origin = format!("http://127.0.0.1:{port}");
    let url = format!("{origin}/mcp");
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();
    let mut ready = false;
    for _ in 0..100 {
        if client
            .get(format!("{origin}/auth/session"))
            .send()
            .await
            .is_ok()
        {
            ready = true;
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert!(
        ready,
        "{}",
        fs::read_to_string(dir.path().join("daemon.log")).unwrap()
    );
    let initialized=client.post(&url).header("accept","application/json, text/event-stream").json(&json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":ProtocolVersion::LATEST,"capabilities":{},"clientInfo":{"name":"export-test","version":"1"}}})).send().await.unwrap();
    assert!(initialized.status().is_success());
    let session = initialized
        .headers()
        .get("mcp-session-id")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let init = parse(&initialized.text().await.unwrap());
    assert!(init["result"]["capabilities"].get("resources").is_some());
    let notification = client
        .post(&url)
        .header("accept", "application/json, text/event-stream")
        .header("mcp-session-id", &session)
        .json(&json!({"jsonrpc":"2.0","method":"notifications/initialized"}))
        .send()
        .await
        .unwrap();
    assert!(notification.status().is_success());
    let tools = rpc(&client, &url, &session, 2, "tools/list", json!({})).await;
    assert_eq!(tools["result"]["tools"].as_array().unwrap().len(), 35);
    for (index, format) in ["amg", "markdown"].into_iter().enumerate() {
        let exported=rpc(&client,&url,&session,10+index as u32,"tools/call",json!({"name":"export","arguments":{"format":format,"initiative":"alpha","destination":format,"request_id":format!("export-{index}")}})).await;
        assert!(exported.get("error").is_none(), "{exported}");
        let result = &exported["result"];
        assert_ne!(result["isError"], true, "{result}");
        let structured = &result["structuredContent"];
        assert_eq!(structured["data"]["artifacts"][0]["format"], format);
        let text: Value =
            serde_json::from_str(result["content"][0]["text"].as_str().unwrap()).unwrap();
        assert_eq!(&text, structured);
        let link = result["content"]
            .as_array()
            .unwrap()
            .iter()
            .find(|item| item["type"] == "resource_link")
            .unwrap();
        let uri = link["uri"].as_str().unwrap();
        assert!(uri.starts_with("kaeru://exports/"));
        assert!(
            result["content"]
                .as_array()
                .unwrap()
                .iter()
                .all(|item| item.get("blob").is_none())
        );
        let download = rpc(
            &client,
            &url,
            &session,
            20 + index as u32,
            "resources/read",
            json!({"uri":uri}),
        )
        .await;
        let bytes = STANDARD
            .decode(download["result"]["contents"][0]["blob"].as_str().unwrap())
            .unwrap();
        if format == "amg" {
            let doc = amg::decode(bytes.as_slice(), &AmgLimits::default()).unwrap();
            assert_eq!(doc.data.nodes[0].body.as_deref(), Some(exact));
        } else {
            let mut zip = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
            let mut text = String::new();
            for i in 0..zip.len() {
                let mut entry = zip.by_index(i).unwrap();
                assert!(entry.name().ends_with(".md"));
                entry.read_to_string(&mut text).unwrap();
            }
            assert!(text.contains(exact));
        }
        let replayed=rpc(&client,&url,&session,30+index as u32,"tools/call",json!({"name":"export","arguments":{"format":format,"initiative":"alpha","destination":format,"request_id":format!("export-{index}")}})).await;
        assert_eq!(replayed["result"]["structuredContent"]["replayed"], true);
        assert!(
            replayed["result"]["content"]
                .as_array()
                .unwrap()
                .iter()
                .any(|item| item["uri"] == uri)
        );
    }
    let missing = rpc(
        &client,
        &url,
        &session,
        50,
        "resources/read",
        json!({"uri":"file:///etc/passwd"}),
    )
    .await;
    assert!(missing.get("error").is_some());
    let http = client
        .post(format!("{origin}/v1/export"))
        .json(&json!({"format":"amg","initiative":"alpha","destination":"http.amg"}))
        .send()
        .await
        .unwrap();
    assert!(http.status().is_success());
    let response: Value = http.json().await.unwrap();
    let file = response["data"]["artifacts"][0]["path"].as_str().unwrap();
    let doc = amg::decode(fs::File::open(file).unwrap(), &AmgLimits::default()).unwrap();
    assert_eq!(doc.data.nodes[0].body.as_deref(), Some(exact));
    let denied = client
        .post(format!("{origin}/v1/export"))
        .json(&json!({"format":"amg","all":true,"destination":"denied.amg"}))
        .send()
        .await
        .unwrap();
    assert!(!denied.status().is_success());
    assert!(!artifacts.join("denied.amg").exists());
}
