//! Single-account credential persistence, serialized and atomically replaced.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use fs2::FileExt;

use crate::settings::web::WebConfig;

pub struct AccountStore {
    path: PathBuf,
    value: Mutex<WebConfig>,
    _writer_lock: File,
}

fn lock(path: &Path) -> io::Result<File> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut options = OpenOptions::new();
    options.create(true).read(true).write(true).truncate(false);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let file = options.open(path.with_extension("toml.lock"))?;
    file.try_lock_exclusive()
        .map_err(|_| io::Error::other("Browser account configuration is already in use"))?;
    Ok(file)
}

fn save(path: &Path, value: &WebConfig) -> io::Result<()> {
    value.validate().map_err(io::Error::other)?;
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::other("Configuration path needs a directory"))?;
    let mut temp = tempfile::NamedTempFile::new_in(parent)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        temp.as_file()
            .set_permissions(fs::Permissions::from_mode(0o600))?;
    }
    temp.write_all(
        b"# basic-creds: admin:admin\n# Example only; no account is enabled by this comment.\n\n",
    )?;
    let body = toml::to_string_pretty(value).map_err(io::Error::other)?;
    temp.write_all(body.as_bytes())?;
    temp.as_file_mut().sync_all()?;
    temp.persist(path).map_err(|error| error.error)?;
    #[cfg(unix)]
    {
        if File::open(parent).and_then(|dir| dir.sync_all()).is_err() {
            tracing::warn!("Account saved, but configuration directory sync failed");
        }
    }
    Ok(())
}

impl AccountStore {
    pub fn open(path: PathBuf) -> io::Result<Option<Self>> {
        if !path.exists() {
            return Ok(None);
        }
        if fs::symlink_metadata(&path)?.file_type().is_symlink() {
            return Err(io::Error::other(
                "Browser account configuration must be a regular file",
            ));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if fs::metadata(&path)?.permissions().mode() & 0o077 != 0 {
                return Err(io::Error::other(
                    "Browser account configuration must be owner-only (0600)",
                ));
            }
        }
        let writer_lock = lock(&path)?;
        let value: WebConfig = toml::from_str(&fs::read_to_string(&path)?)
            .map_err(|_| io::Error::other("Invalid browser account configuration"))?;
        value.validate().map_err(io::Error::other)?;
        Ok(Some(Self {
            path,
            value: Mutex::new(value),
            _writer_lock: writer_lock,
        }))
    }

    pub fn initialize(path: PathBuf, value: WebConfig) -> io::Result<()> {
        let _lock = lock(&path)?;
        if path.exists() {
            return Err(io::Error::other(
                "Account already configured; use --reset-web-password",
            ));
        }
        save(&path, &value)
    }

    pub fn snapshot(&self) -> WebConfig {
        self.value.lock().unwrap_or_else(|p| p.into_inner()).clone()
    }

    pub async fn update_async(
        self: &std::sync::Arc<Self>,
        expected: u64,
        change: impl FnOnce(&mut WebConfig) + Send + 'static,
    ) -> io::Result<()> {
        let account = self.clone();
        tokio::task::spawn_blocking(move || account.update(expected, change))
            .await
            .map_err(|_| io::Error::other("Configuration update task failed"))?
    }

    pub fn update(&self, expected: u64, change: impl FnOnce(&mut WebConfig)) -> io::Result<()> {
        let mut current = self.value.lock().unwrap_or_else(|p| p.into_inner());
        if current.revision != expected {
            return Err(io::Error::other("Account changed; reload settings"));
        }
        let mut next = current.clone();
        change(&mut next);
        next.revision = next
            .revision
            .checked_add(1)
            .ok_or_else(|| io::Error::other("Account revision exhausted"))?;
        save(&self.path, &next)?;
        *current = next;
        Ok(())
    }
}
