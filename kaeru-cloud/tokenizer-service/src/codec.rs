//! Shared CBOR representation for HTTP, durable operations and cached facts.
use crate::error::{Error, Result};
use serde::{Serialize, de::DeserializeOwned};
use std::io::Cursor;

pub fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    ciborium::into_writer(value, &mut bytes).map_err(|_| Error::Codec)?;
    Ok(bytes)
}

pub fn decode<T: DeserializeOwned>(bytes: &[u8], nesting: usize) -> Result<T> {
    let mut cursor = Cursor::new(bytes);
    let result = ciborium::de::from_reader_with_recursion_limit(&mut cursor, nesting)
        .map_err(|_| Error::Codec)?;
    if cursor.position() != bytes.len() as u64 {
        return Err(Error::Codec);
    }
    Ok(result)
}
