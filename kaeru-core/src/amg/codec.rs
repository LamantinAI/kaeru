//! Length-delimited container. Never scan binary payloads for ':' delimiters.

use std::io::{Cursor, Read, Write};

use ciborium::value::Value;
use serde::{Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};

use super::{
    validate::{metadata as validate_metadata, validate},
    wire::preflight,
};
use crate::config::amg::{AmgLimits, MAGIC};
use crate::models::amg::{AmgDocument, AmgGraph, AmgMetadata};
use crate::{Error, Result};

pub(super) fn canonical<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    fn sort(value: &mut Value) {
        match value {
            Value::Map(pairs) => {
                for (_, value) in pairs.iter_mut() {
                    sort(value);
                }
                pairs.sort_by(|(a, _), (b, _)| {
                    let (a, b) = (a.as_text().unwrap_or(""), b.as_text().unwrap_or(""));
                    a.len().cmp(&b.len()).then(a.as_bytes().cmp(b.as_bytes()))
                });
            }
            Value::Array(values) => {
                for value in values {
                    sort(value);
                }
            }
            _ => {}
        }
    }
    let mut value =
        Value::serialized(value).map_err(|e| Error::Invalid(format!("AMG encode: {e}")))?;
    sort(&mut value);
    let mut bytes = vec![];
    ciborium::ser::into_writer(&value, &mut bytes)
        .map_err(|e| Error::Invalid(format!("AMG encode: {e}")))?;
    Ok(bytes)
}

fn parse<T: DeserializeOwned>(bytes: &[u8], limits: &AmgLimits) -> Result<T> {
    preflight(bytes, limits)?;
    ciborium::de::from_reader_with_recursion_limit(Cursor::new(bytes), limits.nesting + 8)
        .map_err(|e| Error::Invalid(format!("AMG decode: {e}")))
}

pub fn encode(document: &AmgDocument, limits: &AmgLimits) -> Result<Vec<u8>> {
    validate(document, limits)?;
    struct Counter {
        size: usize,
        maximum: usize,
    }
    impl Write for Counter {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.size = self
                .size
                .checked_add(bytes.len())
                .filter(|size| *size <= self.maximum)
                .ok_or_else(|| std::io::Error::other("AMG exceeds serialization budget"))?;
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    // JSON escaping can cost six bytes for one CBOR text byte. Bound traversal
    // before constructing the intermediate CBOR value, without copying bodies.
    let mut counter = Counter {
        size: 0,
        maximum: limits.metadata_bytes.saturating_mul(6),
    };
    serde_json::to_writer(&mut counter, &document.metadata)
        .map_err(|e| Error::Invalid(e.to_string()))?;
    counter.maximum = limits.file_bytes.saturating_mul(6);
    serde_json::to_writer(&mut counter, &document.data)
        .map_err(|e| Error::Invalid(e.to_string()))?;
    let metadata = canonical(&document.metadata)?;
    let data = canonical(&document.data)?;
    let size = metadata
        .len()
        .checked_add(data.len())
        .and_then(|s| s.checked_add(17))
        .ok_or_else(|| Error::Invalid("AMG size overflow".into()))?;
    if metadata.len() > limits.metadata_bytes || size > limits.file_bytes {
        return Err(Error::Invalid(
            "AMG document exceeds configured byte limits".into(),
        ));
    }
    let length = u32::try_from(metadata.len())
        .map_err(|_| Error::Invalid("AMG metadata too large".into()))?;
    let mut out = Vec::with_capacity(size);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&length.to_be_bytes());
    out.extend_from_slice(&metadata);
    out.push(b':');
    out.extend_from_slice(&(data.len() as u64).to_be_bytes());
    out.extend_from_slice(&data);
    Ok(out)
}

pub fn decode(mut reader: impl Read, limits: &AmgLimits) -> Result<AmgDocument> {
    let mut magic = [0; 4];
    reader.read_exact(&mut magic)?;
    if &magic != MAGIC {
        return Err(Error::Invalid("Not an AMG document".into()));
    }
    let mut small = [0; 4];
    reader.read_exact(&mut small)?;
    let metadata_len = u32::from_be_bytes(small) as usize;
    if metadata_len > limits.metadata_bytes || metadata_len > limits.file_bytes.saturating_sub(17) {
        return Err(Error::Invalid("AMG metadata exceeds byte limit".into()));
    }
    let mut bytes = vec![0; metadata_len];
    reader.read_exact(&mut bytes)?;
    let metadata: AmgMetadata = parse(&bytes, limits)?;
    validate_metadata(&metadata)?;
    let mut colon = [0];
    reader.read_exact(&mut colon)?;
    if colon != [b':'] {
        return Err(Error::Invalid("AMG missing section separator".into()));
    }
    let mut large = [0; 8];
    reader.read_exact(&mut large)?;
    let data_len = usize::try_from(u64::from_be_bytes(large))
        .map_err(|_| Error::Invalid("AMG size overflow".into()))?;
    if data_len > limits.file_bytes.saturating_sub(metadata_len + 17) {
        return Err(Error::Invalid("AMG graph exceeds byte limit".into()));
    }
    let mut bytes = vec![0; data_len];
    reader.read_exact(&mut bytes)?;
    let data: AmgGraph = parse(&bytes, limits)?;
    if reader.read(&mut colon)? != 0 {
        return Err(Error::Invalid("AMG trailing data".into()));
    }
    let document = AmgDocument { metadata, data };
    validate(&document, limits)?;
    Ok(document)
}

pub(super) fn digest(document: &AmgDocument, limits: &AmgLimits) -> Result<String> {
    Ok(format!("{:x}", Sha256::digest(encode(document, limits)?)))
}

pub(super) fn json_digest(value: &serde_json::Value) -> String {
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(value).expect("JSON serializes"))
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::amg::{PROFILE, SCHEMA_VERSION};
    use crate::models::amg::{AmgScope, AmgSoftware};
    fn document() -> AmgDocument {
        AmgDocument {
            metadata: AmgMetadata {
                schema_version: SCHEMA_VERSION,
                exported_at: "2026-10-20T00:00:00Z".into(),
                software: AmgSoftware {
                    name: "kaeru".into(),
                    version: "0.8.0".into(),
                },
                profile: PROFILE.into(),
                scope: AmgScope::All,
                initiatives: vec![],
                omissions: vec![],
            },
            data: AmgGraph::default(),
        }
    }
    fn frame(meta: &[u8], data: &[u8]) -> Vec<u8> {
        let mut bytes = b"amg:".to_vec();
        bytes.extend_from_slice(&(meta.len() as u32).to_be_bytes());
        bytes.extend_from_slice(meta);
        bytes.push(b':');
        bytes.extend_from_slice(&(data.len() as u64).to_be_bytes());
        bytes.extend_from_slice(data);
        bytes
    }
    #[test]
    fn matches_independently_encoded_golden_document() {
        let expected = include_bytes!("../../tests/fixtures/amg-v1-empty.amg");
        let limits = AmgLimits::default();
        assert_eq!(encode(&document(), &limits).unwrap(), expected.as_slice());
        assert_eq!(decode(expected.as_slice(), &limits).unwrap(), document());
    }
    #[test]
    fn truncations_and_trailing_data_fail() {
        let limits = AmgLimits::default();
        let bytes = encode(&document(), &limits).unwrap();
        for end in 0..bytes.len() {
            assert!(
                decode(&bytes[..end], &limits).is_err(),
                "accepted prefix {end}"
            );
        }
        let mut trailing = bytes.clone();
        trailing.push(0);
        assert!(decode(trailing.as_slice(), &limits).is_err());
        let mut magic = bytes;
        magic[0] = b'x';
        assert!(decode(magic.as_slice(), &limits).is_err());
    }
    #[test]
    fn unsupported_schema_is_rejected_before_graph_is_read() {
        let mut meta = document().metadata;
        meta.schema_version = SCHEMA_VERSION + 1;
        let meta = canonical(&meta).unwrap();
        let mut bytes = b"amg:".to_vec();
        bytes.extend_from_slice(&(meta.len() as u32).to_be_bytes());
        bytes.extend_from_slice(&meta);
        let error = decode(bytes.as_slice(), &AmgLimits::default())
            .unwrap_err()
            .to_string();
        assert!(error.contains("unsupported"), "{error}");
    }
    #[test]
    fn malformed_collections_duplicates_tags_and_nonfinite_values_fail() {
        let limits = AmgLimits::default();
        let meta = canonical(&document().metadata).unwrap();
        for data in [
            vec![0x9b, 255, 255, 255, 255, 255, 255, 255, 255],
            vec![0xbf, 0xff],
            vec![0xc0, 0xf6],
            vec![0xfb, 0x7f, 0xf0, 0, 0, 0, 0, 0, 0],
            vec![0x41, 0],
        ] {
            assert!(decode(frame(&meta, &data).as_slice(), &limits).is_err());
        }
        let duplicate = Value::Map(vec![
            (Value::Text("key".into()), Value::Null),
            (Value::Text("key".into()), Value::Null),
        ]);
        let mut encoded = vec![];
        ciborium::ser::into_writer(&duplicate, &mut encoded).unwrap();
        assert!(
            decode(frame(&meta, &encoded).as_slice(), &limits)
                .unwrap_err()
                .to_string()
                .contains("duplicate")
        );
        let mut deep = vec![0x81; limits.nesting + 2];
        deep.push(0xf6);
        assert!(decode(frame(&meta, &deep).as_slice(), &limits).is_err());
    }
    #[test]
    fn byte_limits_are_checked_before_allocations() {
        let mut bytes = b"amg:".to_vec();
        bytes.extend_from_slice(&u32::MAX.to_be_bytes());
        assert!(decode(bytes.as_slice(), &AmgLimits::default()).is_err());
        let limits = AmgLimits {
            file_bytes: 32,
            ..Default::default()
        };
        assert!(encode(&document(), &limits).is_err());
    }
}
