//! Explicit host policy for the shared transports.

use super::web::AccessMode;
use crate::{api::ApiConfig, auth::store::AccountStore};

pub struct RuntimeOptions {
    pub access: AccessMode,
    pub account: Option<AccountStore>,
    pub ui_enabled: bool,
    pub api_enabled: bool,
    pub api_config: ApiConfig,
    pub hygiene_enabled: bool,
    pub update_check_enabled: bool,
}

impl Default for RuntimeOptions {
    fn default() -> Self {
        Self {
            access: AccessMode {
                enabled: false,
                mcp_token: String::new(),
            },
            account: None,
            ui_enabled: false,
            api_enabled: false,
            api_config: ApiConfig::default(),
            hygiene_enabled: true,
            update_check_enabled: false,
        }
    }
}

pub(crate) const SHUTDOWN_GRACE: std::time::Duration = std::time::Duration::from_secs(3);
pub(crate) const STORE_RELEASE_POLL: std::time::Duration = std::time::Duration::from_millis(10);
