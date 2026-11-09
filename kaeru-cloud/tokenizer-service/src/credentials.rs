use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use contracts::SecretToken;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use uuid::Uuid;

use crate::error::{Error, Result};

const DOMAIN: &[u8] = b"kaeru.credential.v1\0";
const SECRET_LEN: usize = 32;

pub fn mint(id: Uuid) -> Result<(SecretToken, [u8; 32])> {
    let mut secret = [0; SECRET_LEN];
    getrandom::fill(&mut secret).map_err(|_| Error::Entropy)?;
    let verifier = fingerprint(id, &secret);
    let value = SecretToken::new(format!("krt1.{id}.{}", URL_SAFE_NO_PAD.encode(secret)));
    Ok((value, verifier))
}

pub fn parse(token: &SecretToken) -> Result<(Uuid, [u8; 32])> {
    let mut parts = token.expose().split('.');
    if token.expose().len() > 128 || parts.next() != Some("krt1") {
        return Err(Error::Unauthorized);
    }
    let id = parts
        .next()
        .and_then(|p| Uuid::parse_str(p).ok())
        .ok_or(Error::Unauthorized)?;
    let encoded = parts.next().ok_or(Error::Unauthorized)?;
    if parts.next().is_some() {
        return Err(Error::Unauthorized);
    }
    let bytes = URL_SAFE_NO_PAD
        .decode(encoded)
        .map_err(|_| Error::Unauthorized)?;
    let secret: [u8; SECRET_LEN] = bytes.try_into().map_err(|_| Error::Unauthorized)?;
    if URL_SAFE_NO_PAD.encode(secret) != encoded {
        return Err(Error::Unauthorized);
    }
    Ok((id, fingerprint(id, &secret)))
}

fn fingerprint(id: Uuid, secret: &[u8; SECRET_LEN]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(DOMAIN);
    hasher.update(id.as_bytes());
    hasher.update(secret);
    hasher.finalize().into()
}

pub fn matches(expected: &[u8], actual: &[u8; 32]) -> bool {
    expected.len() == 32 && bool::from(expected.ct_eq(actual.as_slice()))
}

#[cfg(test)]
mod tests {
    use super::{matches, mint, parse};
    use contracts::SecretToken;
    use uuid::Uuid;

    #[test]
    fn selector_and_secret_are_bound_and_debug_is_redacted() {
        let id = Uuid::now_v7();
        let (token, verifier) = mint(id).unwrap();
        let (selector, hash) = parse(&token).unwrap();
        assert_eq!(id, selector);
        assert!(matches(&verifier, &hash));
        assert_eq!(format!("{token:?}"), "[REDACTED]");
        let changed = SecretToken::new(
            token
                .expose()
                .replace(&id.to_string(), &Uuid::now_v7().to_string()),
        );
        assert!(!matches(&verifier, &parse(&changed).unwrap().1));
    }
}
