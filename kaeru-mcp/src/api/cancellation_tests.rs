use std::fs;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::Request;
use axum::middleware::{Next, from_fn};
use kaeru_core::{KaeruConfig, Store, new_node_id};
use serde_json::json;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Notify, oneshot};
use tokio::time::timeout;

use super::{ApiConfig, router};

struct Dropped(Arc<Notify>);
impl Drop for Dropped {
    fn drop(&mut self) {
        self.0.notify_one();
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn disconnected_http_read_does_not_run_after_the_scope_queue_releases() {
    let directory = std::env::temp_dir().join(format!("kaeru-http-cancel-{}", new_node_id()));
    let config = KaeruConfig {
        read_trace_enabled: true,
        read_trace_dir: directory.join("trace"),
        ..KaeruConfig::default()
    };
    let store = Arc::new(Store::open_in_memory_with(config).unwrap());
    let request_started = Arc::new(Notify::new());
    let request_dropped = Arc::new(Notify::new());
    let started = request_started.clone();
    let dropped = request_dropped.clone();
    let app = router(
        store.clone(),
        ApiConfig {
            allow: vec!["*".into()],
            include_local: true,
            ..ApiConfig::default()
        },
    )
    .layer(from_fn(move |request: Request, next: Next| {
        let started = started.clone();
        let dropped = dropped.clone();
        async move {
            let _drop = Dropped(dropped);
            started.notify_one();
            next.run(request).await
        }
    }));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    let (held_tx, held_rx) = oneshot::channel();
    let (release_tx, release_rx) = std::sync::mpsc::channel();
    let worker = store.clone();
    let holder = std::thread::spawn(move || {
        worker.scoped(Some("holder"), |_| {
            let _ = held_tx.send(());
            let _ = release_rx.recv();
        })
    });
    held_rx.await.unwrap();
    let mut socket = TcpStream::connect(address).await.unwrap();
    let body = json!({"initiative":"perf","view":"map"}).to_string();
    let request = format!(
        "POST /v1/context HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );
    socket.write_all(request.as_bytes()).await.unwrap();
    timeout(Duration::from_secs(3), request_started.notified())
        .await
        .unwrap();
    drop(socket);
    let disconnected = timeout(Duration::from_secs(3), request_dropped.notified()).await;
    // Always release the blocking holder, including on assertion failure.
    let _ = release_tx.send(());
    holder.join().unwrap();
    assert!(
        disconnected.is_ok(),
        "HTTP disconnect must drop the handler future"
    );
    // Confirm the scope is available and observe the delivery journal after
    // disconnect. The core cancellation test separately joins its worker;
    // another mutex waiter is not a join or a FIFO-order guarantee.
    let worker = store.clone();
    tokio::task::spawn_blocking(move || worker.scoped(None, |_| ()))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert!(store.flush_read_trace(Duration::from_secs(2)));
    if let Ok(files) = fs::read_dir(directory.join("trace")) {
        for file in files.flatten() {
            if file.path().extension().is_some_and(|s| s == "jsonl") {
                assert!(fs::read_to_string(file.path()).unwrap().trim().is_empty());
            }
        }
    }
    server.abort();
    store.shutdown_read_trace(Duration::from_secs(2));
    let _ = fs::remove_dir_all(directory);
}
