//! Local installation configuration discovery, preserving existing ENV and files.

use config::{Config, ConfigError, Environment, File, FileFormat};
use std::path::PathBuf;

#[cfg(test)]
pub use kaeru_server::settings::MCP_PROTOCOL_VERSION;
pub use kaeru_server::settings::{
    KaeruMcpConfig, MCP_PROTOCOL_HEADER, MCP_SESSION_HEADER, STDIO_SESSION_CLOSE_TIMEOUT,
};
pub mod web;

pub fn load() -> Result<KaeruMcpConfig, ConfigError> {
    load_sources(
        clouds_file_path(),
        Environment::with_prefix("KAERU_MCP").try_parsing(true),
    )
}

fn load_sources(
    path: Option<PathBuf>,
    environment: Environment,
) -> Result<KaeruMcpConfig, ConfigError> {
    let mut builder = Config::builder();
    if let Some(path) = path {
        builder = builder.add_source(File::from(path).format(FileFormat::Toml).required(false));
    }
    let settings: KaeruMcpConfig = builder.add_source(environment).build()?.try_deserialize()?;
    settings.validate("KAERU_MCP")?;
    Ok(settings)
}

fn clouds_file_path() -> Option<PathBuf> {
    if let Ok(value) = std::env::var("KAERU_MCP_CLOUDS_FILE")
        && !value.is_empty()
    {
        return Some(value.into());
    }
    let base = std::env::var("XDG_CONFIG_HOME")
        .ok()
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var("HOME")
                .ok()
                .filter(|s| !s.is_empty())
                .map(|s| PathBuf::from(s).join(".config"))
        })?;
    Some(base.join("kaeru/clouds.toml"))
}

#[cfg(test)]
mod tests {
    use super::load_sources;
    use config::Environment;
    use std::collections::HashMap;

    #[test]
    fn explicit_cloud_file_and_environment_keep_local_precedence() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("clouds.toml");
        std::fs::write(
            &path,
            "default='team'\n[clouds.team]\nurl='https://team.example'\ntoken='fixture'\n",
        )
        .unwrap();
        let config = load_sources(
            Some(path),
            Environment::with_prefix("KAERU_MCP")
                .try_parsing(true)
                .source(Some(HashMap::from([(
                    "KAERU_MCP_LISTEN_PORT".into(),
                    "19001".into(),
                )]))),
        )
        .unwrap();
        assert_eq!(config.default_cloud, "team");
        assert_eq!(config.clouds["team"].token, "fixture");
        assert_eq!(config.listen_port, 19001);
    }
}
