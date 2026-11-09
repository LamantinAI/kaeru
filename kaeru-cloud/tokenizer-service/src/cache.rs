use std::time::Duration;

use contracts::ProjectionSnapshot;
use redis::{AsyncCommands, aio::ConnectionManager};

// Compatibility exports; serialization is independent of optional caching.
pub use crate::codec::{decode, encode};

/// Optional derived-facts cache. No credentials or final allow decisions.
#[derive(Clone)]
pub struct Cache {
    connection: ConnectionManager,
    ttl: Duration,
    timeout: Duration,
    max_bytes: usize,
}

impl Cache {
    pub async fn connect(
        url: &str,
        ttl: Duration,
        timeout: Duration,
        max_bytes: usize,
    ) -> Option<Self> {
        let client = redis::Client::open(url).ok()?;
        let connection = tokio::time::timeout(timeout, ConnectionManager::new(client))
            .await
            .ok()?
            .ok()?;
        Some(Self {
            connection,
            ttl,
            timeout,
            max_bytes,
        })
    }

    pub async fn get(&self, key: &str) -> Option<ProjectionSnapshot> {
        let mut connection = self.connection.clone();
        let bytes: Vec<u8> = tokio::time::timeout(self.timeout, connection.get(key))
            .await
            .ok()?
            .ok()?;
        if bytes.len() > self.max_bytes {
            return None;
        }
        decode(&bytes, 32).ok()
    }

    pub async fn put(&self, key: &str, value: &ProjectionSnapshot) {
        let Ok(bytes) = encode(value) else {
            return;
        };
        if bytes.len() > self.max_bytes {
            return;
        }
        let mut connection = self.connection.clone();
        let _: Option<redis::RedisResult<()>> = tokio::time::timeout(
            self.timeout,
            connection.set_ex(key, bytes, self.ttl.as_secs().max(1)),
        )
        .await
        .ok();
    }
}
