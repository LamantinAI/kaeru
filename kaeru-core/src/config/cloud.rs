//! Defaults shared by cloud transports and discovery listings.

use std::time::Duration;

pub const CLOUD_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
pub const CLOUD_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
pub const CLOUD_DEFAULT_PAGE_SIZE: usize = 50;
pub const CLOUD_MAX_PAGE_SIZE: usize = 500;
