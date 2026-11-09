pub mod defaults;

use std::env;
use std::net::SocketAddr;
use std::time::Duration;

use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

use crate::error::{Error, Result};

use defaults::{
    CACHE_MAX_BYTES, CACHE_TIMEOUT, CACHE_TTL, LISTEN, MAX_BODY_BYTES, MAX_CONNECTIONS,
    MAX_PROJECTION_RECORDS,
};

#[derive(Clone)]
pub struct ServiceKeys {
    control: [u8; 32],
    memory: [u8; 32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Caller {
    Control,
    Memory,
}

impl ServiceKeys {
    pub fn new(control: &str, memory: &str) -> Result<Self> {
        if control.len() < 32 || memory.len() < 32 || control == memory {
            return Err(Error::Config("service_keys"));
        }
        Ok(Self {
            control: Sha256::digest(control.as_bytes()).into(),
            memory: Sha256::digest(memory.as_bytes()).into(),
        })
    }
    pub fn identify(&self, value: &str) -> Option<Caller> {
        let hash: [u8; 32] = Sha256::digest(value.as_bytes()).into();
        let control = bool::from(hash.ct_eq(&self.control));
        let memory = bool::from(hash.ct_eq(&self.memory));
        if control {
            Some(Caller::Control)
        } else if memory {
            Some(Caller::Memory)
        } else {
            None
        }
    }
}

/// Does not implement Debug: connection strings and service keys are sensitive.
pub struct Settings {
    pub listen: SocketAddr,
    pub database_url: String,
    pub valkey_url: Option<String>,
    pub cache_namespace: String,
    pub keys: ServiceKeys,
    pub max_connections: u32,
    pub max_body_bytes: usize,
    pub max_projection_records: usize,
    pub cache_ttl: Duration,
    pub cache_timeout: Duration,
    pub cache_max_bytes: usize,
}

fn required(name: &'static str) -> Result<String> {
    env::var(name)
        .ok()
        .filter(|v| !v.is_empty())
        .ok_or(Error::Config(name))
}

impl Settings {
    pub fn from_env() -> Result<Self> {
        let listen = env::var("TOKENIZER_LISTEN")
            .unwrap_or_else(|_| LISTEN.to_string())
            .parse()
            .map_err(|_| Error::Config("TOKENIZER_LISTEN"))?;
        let namespace =
            env::var("TOKENIZER_CACHE_NAMESPACE").unwrap_or_else(|_| "kaeru:tokenizer:v1".into());
        if namespace.is_empty()
            || namespace.len() > 128
            || !namespace
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b':' | b'-' | b'_'))
        {
            return Err(Error::Config("TOKENIZER_CACHE_NAMESPACE"));
        }
        Ok(Self {
            listen,
            database_url: required("TOKENIZER_DATABASE_URL")?,
            valkey_url: env::var("TOKENIZER_VALKEY_URL")
                .ok()
                .filter(|s| !s.is_empty()),
            cache_namespace: namespace,
            keys: ServiceKeys::new(
                &required("TOKENIZER_CONTROL_KEY")?,
                &required("TOKENIZER_MEMORY_KEY")?,
            )?,
            max_connections: MAX_CONNECTIONS,
            max_body_bytes: MAX_BODY_BYTES,
            max_projection_records: MAX_PROJECTION_RECORDS,
            cache_ttl: CACHE_TTL,
            cache_timeout: CACHE_TIMEOUT,
            cache_max_bytes: CACHE_MAX_BYTES,
        })
    }
}
