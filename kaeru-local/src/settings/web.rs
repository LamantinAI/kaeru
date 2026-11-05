//! Local installation account discovery; never used by memory-service.
pub use kaeru_server::settings::web::{WebConfig, access_mode};
use std::path::PathBuf;

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
