//! Browser authentication and administration exchange contracts.

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SessionClaims {
    pub sub: String,
    pub exp: u64,
    pub iat: u64,
    pub nbf: u64,
    pub iss: String,
    pub aud: String,
    pub revision: u64,
    pub csrf: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountInput {
    pub current_password: String,
    pub username: String,
    pub new_password: Option<String>,
    pub revision: u64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionSettingsInput {
    pub current_password: String,
    pub ttl_seconds: u64,
    pub revision: u64,
}

#[derive(Serialize)]
pub struct SessionView {
    pub configured: bool,
    pub authenticated: bool,
    pub username: Option<String>,
    pub expires_at: Option<u64>,
    pub csrf_token: Option<String>,
}

impl SessionView {
    pub fn signed_out(configured: bool) -> Self {
        Self {
            configured,
            authenticated: false,
            username: None,
            expires_at: None,
            csrf_token: None,
        }
    }

    pub fn authenticated(claims: &SessionClaims) -> Self {
        Self {
            configured: true,
            authenticated: true,
            username: Some(claims.sub.clone()),
            expires_at: Some(claims.exp),
            csrf_token: Some(claims.csrf.clone()),
        }
    }
}
