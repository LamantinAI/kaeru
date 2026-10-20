//! Browser-account configuration and limits; no active default credentials.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub const SESSION_COOKIE: &str = "kaeru_session";
pub const CSRF_HEADER: &str = "x-kaeru-csrf";
pub const SESSION_ISSUER: &str = "kaeru-mcp";
pub const SESSION_AUDIENCE: &str = "kaeru-web";
pub const DEFAULT_TTL_SECONDS: u64 = 3600;
pub const MIN_TTL_SECONDS: u64 = 300;
pub const MAX_TTL_SECONDS: u64 = 86400;
pub const MAX_PASSWORD_BYTES: usize = 1024;
pub const MAX_LOGIN_BYTES: usize = 128;
pub const LOGIN_ATTEMPTS: usize = 5;
pub const LOGIN_WINDOW_SECONDS: u64 = 60;
pub const PASSWORD_WORKERS: usize = 2;
pub const MAX_REQUEST_BYTES: usize = 8192;

#[derive(Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct WebConfig {
    pub enabled: bool,
    pub mcp_token: String,
    pub username: String,
    pub password_hash: String,
    pub jwt_secret: String,
    #[serde(default)]
    pub public_origin: String,
    pub ttl_seconds: u64,
    pub issuer: String,
    pub audience: String,
    pub revision: u64,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mcp_token: String::new(),
            username: String::new(),
            password_hash: String::new(),
            jwt_secret: String::new(),
            public_origin: String::new(),
            ttl_seconds: DEFAULT_TTL_SECONDS,
            issuer: SESSION_ISSUER.into(),
            audience: SESSION_AUDIENCE.into(),
            revision: 1,
        }
    }
}

pub struct AccessMode {
    pub enabled: bool,
    pub mcp_token: String,
}

pub fn access_mode(config: Option<&WebConfig>, token: &str) -> AccessMode {
    let enabled = config.map_or(!token.trim().is_empty(), |cfg| cfg.enabled);
    let token = if !enabled {
        ""
    } else if !token.trim().is_empty() {
        token.trim()
    } else {
        config.map_or("", |cfg| cfg.mcp_token.as_str())
    };
    AccessMode {
        enabled,
        mcp_token: token.into(),
    }
}

impl WebConfig {
    pub fn validate(&self) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }
        if self.username.trim().is_empty() || self.username.len() > MAX_LOGIN_BYTES {
            return Err("Username must be between 1 and 128 bytes".into());
        }
        if !(MIN_TTL_SECONDS..=MAX_TTL_SECONDS).contains(&self.ttl_seconds) {
            return Err("Session lifetime must be between 300 and 86400 seconds".into());
        }
        if self.jwt_secret.len() < 64 || !self.jwt_secret.bytes().all(|c| c.is_ascii_hexdigit()) {
            return Err(
                "The signing key must contain at least 32 random bytes encoded as hex".into(),
            );
        }
        let hash =
            argon2::PasswordHash::new(&self.password_hash).map_err(|_| "Invalid password hash")?;
        if hash.algorithm.as_str() != "argon2id" || hash.salt.is_none() || hash.hash.is_none() {
            return Err("An Argon2id password hash with salt is required".into());
        }
        if self.issuer.is_empty() || self.audience.is_empty() {
            return Err("JWT issuer and audience are required".into());
        }
        if !self.public_origin.is_empty() {
            let url =
                reqwest::Url::parse(&self.public_origin).map_err(|_| "Invalid public origin")?;
            if !matches!(url.scheme(), "http" | "https")
                || url.origin().ascii_serialization() != self.public_origin
            {
                return Err("Use an exact HTTP or HTTPS origin, without a path".into());
            }
        }
        Ok(())
    }

    pub fn secure_cookie(&self) -> bool {
        self.public_origin.starts_with("https://")
    }
}

fn default_config_path(
    windows: bool,
    mut variable: impl FnMut(&str) -> Option<std::ffi::OsString>,
) -> Option<PathBuf> {
    let mut value = |name| {
        variable(name)
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
    };
    let base = if windows {
        value("APPDATA").or_else(|| value("USERPROFILE").map(|home| home.join("AppData/Roaming")))
    } else {
        value("XDG_CONFIG_HOME").or_else(|| value("HOME").map(|home| home.join(".config")))
    }?;
    Some(base.join("kaeru/web.toml"))
}

pub fn optional_config_path() -> Result<Option<PathBuf>, String> {
    if let Some(value) = std::env::var_os("KAERU_MCP_WEB_CONFIG_FILE").filter(|v| !v.is_empty()) {
        let path = PathBuf::from(value);
        return if path.is_absolute() {
            Ok(Some(path))
        } else {
            std::env::current_dir()
                .map(|cwd| Some(cwd.join(path)))
                .map_err(|_| "Cannot resolve configuration path".into())
        };
    }
    Ok(default_config_path(cfg!(windows), |name| {
        std::env::var_os(name)
    }))
}

pub fn config_path() -> Result<PathBuf, String> {
    optional_config_path()?.ok_or_else(|| {
        "Cannot resolve configuration directory; set KAERU_MCP_WEB_CONFIG_FILE".into()
    })
}

#[cfg(test)]
mod tests {
    use super::default_config_path;
    use std::{ffi::OsString, path::PathBuf};

    fn resolve(windows: bool, values: &[(&str, &str)]) -> Option<PathBuf> {
        default_config_path(windows, |name| {
            values
                .iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| OsString::from(value))
        })
    }

    #[test]
    fn explicit_mode_wins_over_old_tokens_and_missing_credentials_fail_closed() {
        use super::{WebConfig, access_mode};
        let disabled: WebConfig = toml::from_str("enabled = false").unwrap();
        assert!(disabled.validate().is_ok());
        assert!(!access_mode(Some(&disabled), "leftover-token").enabled);
        assert!(
            access_mode(Some(&disabled), "leftover-token")
                .mcp_token
                .is_empty()
        );
        let mut enabled = WebConfig {
            mcp_token: "saved-token".into(),
            ..Default::default()
        };
        assert!(enabled.validate().is_err());
        assert!(access_mode(Some(&enabled), "").enabled);
        assert_eq!(access_mode(Some(&enabled), "").mcp_token, "saved-token");
        assert_eq!(
            access_mode(Some(&enabled), "override-token").mcp_token,
            "override-token"
        );
        enabled.mcp_token.clear();
        assert!(access_mode(Some(&enabled), "").mcp_token.is_empty());
        assert!(access_mode(Some(&enabled), "").enabled);
        assert!(!access_mode(None, "").enabled);
        assert!(access_mode(None, "legacy-token").enabled);
        assert!(toml::from_str::<WebConfig>("enabled = 'false'").is_err());
        let legacy: WebConfig = toml::from_str("username = 'admin'").unwrap();
        assert!(
            legacy.enabled,
            "Existing files without a mode stay protected"
        );
    }

    #[test]
    fn defaults_follow_platform_configuration_directories() {
        assert_eq!(
            resolve(
                false,
                &[("XDG_CONFIG_HOME", "/xdg"), ("HOME", "/home/test")]
            ),
            Some(PathBuf::from("/xdg/kaeru/web.toml"))
        );
        assert_eq!(
            resolve(false, &[("HOME", "/home/test")]),
            Some(PathBuf::from("/home/test/.config/kaeru/web.toml"))
        );
        assert_eq!(
            resolve(
                true,
                &[("APPDATA", "/roaming"), ("USERPROFILE", "/users/test")]
            ),
            Some(PathBuf::from("/roaming/kaeru/web.toml"))
        );
        assert_eq!(
            resolve(true, &[("USERPROFILE", "/users/test")]),
            Some(PathBuf::from("/users/test/AppData/Roaming/kaeru/web.toml"))
        );
        assert!(resolve(false, &[]).is_none());
        assert!(resolve(true, &[]).is_none());
        assert!(resolve(false, &[("XDG_CONFIG_HOME", ""), ("HOME", "")]).is_none());
    }
}
