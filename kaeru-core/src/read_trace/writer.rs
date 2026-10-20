//! Owned JSONL segments and age-based gzip retention. No substrate access.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};

use chrono::Utc;
use flate2::{Compression, write::GzEncoder};
use fs2::FileExt;
use uuid::Uuid;

use crate::config::KaeruConfig;
use crate::config::read_trace::MAX_SEGMENT_BYTES;
use crate::models::ReadTraceEvent;

struct Active {
    file: File,
    path: PathBuf,
    first: i64,
    last: i64,
    nonce: String,
    bytes: u64,
}

pub(super) struct Writer {
    directory: PathBuf,
    _ownership: File,
    config: KaeruConfig,
    active: Option<Active>,
}

fn io_error(message: &str) -> io::Error {
    io::Error::other(message)
}

fn resolved(path: &Path) -> io::Result<PathBuf> {
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        return Err(io_error("parent traversal is not allowed in trace paths"));
    }
    let full = if path.is_absolute() {
        path.to_owned()
    } else {
        std::env::current_dir()?.join(path)
    };
    let mut ancestor = full.as_path();
    let mut tail = Vec::new();
    while !ancestor.try_exists()? {
        tail.push(
            ancestor
                .file_name()
                .ok_or_else(|| io_error("invalid path"))?
                .to_owned(),
        );
        ancestor = ancestor
            .parent()
            .ok_or_else(|| io_error("missing ancestor"))?;
    }
    let mut absolute = ancestor.canonicalize()?;
    for item in tail.into_iter().rev() {
        absolute.push(item);
    }
    Ok(absolute)
}

fn create_file(path: &Path, exclusive: bool) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.read(true).write(true);
    if exclusive {
        options.create_new(true);
    } else {
        options.create(true).truncate(false);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options.open(path)
}

fn filename(first: i64, last: i64, nonce: &str, extension: &str) -> String {
    format!("read-{first:020}-{last:020}-{nonce}.{extension}")
}

fn segment(path: &Path) -> Option<(i64, i64, String, String)> {
    let name = path.file_name()?.to_str()?.strip_prefix("read-")?;
    let (stem, extension) = name.split_once('.')?;
    if !matches!(extension, "active" | "jsonl" | "jsonl.gz" | "jsonl.gz.tmp") {
        return None;
    }
    let mut parts = stem.splitn(3, '-');
    let first = parts.next()?.parse::<i64>().ok()?;
    let last = parts.next()?.parse::<i64>().ok()?;
    let nonce = parts.next()?;
    Uuid::parse_str(nonce).ok()?;
    (first >= 0 && last >= first).then(|| (first, last, nonce.into(), extension.into()))
}

impl Writer {
    pub(super) fn open(config: &KaeruConfig) -> io::Result<Self> {
        Self::open_at(config, Utc::now().timestamp_millis())
    }

    pub(super) fn open_at(config: &KaeruConfig, now: i64) -> io::Result<Self> {
        let directory = resolved(&config.read_trace_dir)?;
        if directory.starts_with(resolved(&config.vault_path)?) {
            return Err(io_error("read traces must be outside the vault"));
        }
        let mut builder = fs::DirBuilder::new();
        builder.recursive(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }
        builder.create(&directory)?;
        let lock_path = directory.join(".read-trace.lock");
        if fs::symlink_metadata(&lock_path).is_ok_and(|m| !m.file_type().is_file()) {
            return Err(io_error("trace lock is not a regular file"));
        }
        let ownership = create_file(&lock_path, false)?;
        FileExt::try_lock_exclusive(&ownership)?;
        let mut writer = Self {
            directory,
            _ownership: ownership,
            config: config.clone(),
            active: None,
        };
        writer.recover()?;
        let _ = writer.maintain(now);
        Ok(writer)
    }

    pub(super) fn append(&mut self, event: &ReadTraceEvent) -> io::Result<()> {
        let mut bytes = serde_json::to_vec(event)?;
        bytes.push(b'\n');
        // An oversized event is dropped whole, never misrepresented as a partial read.
        if bytes.len() as u64 > self.config.read_trace_segment_bytes {
            return Ok(());
        }
        let time = event.timestamp.timestamp_millis();
        if time < 0 {
            return Ok(());
        }
        let span = self
            .config
            .read_trace_segment_secs
            .saturating_mul(1000)
            .min(i64::MAX as u64) as i64;
        if self.active.as_ref().is_some_and(|a| {
            a.bytes + bytes.len() as u64 > self.config.read_trace_segment_bytes
                || time < a.first
                || time.saturating_sub(a.first) >= span
        }) {
            self.close()?;
        }
        if self.active.is_none() {
            let nonce = Uuid::now_v7().simple().to_string();
            let path = self.directory.join(filename(time, time, &nonce, "active"));
            let file = create_file(&path, true)?;
            self.active = Some(Active {
                file,
                path,
                first: time,
                last: time,
                nonce,
                bytes: 0,
            });
        }
        let active = self.active.as_mut().unwrap();
        active.file.write_all(&bytes)?;
        active.bytes += bytes.len() as u64;
        active.last = active.last.max(time);
        Ok(())
    }

    pub(super) fn flush(&mut self) -> io::Result<()> {
        if let Some(active) = self.active.as_mut() {
            active.file.flush()?;
        }
        Ok(())
    }

    pub(super) fn close(&mut self) -> io::Result<()> {
        if let Some(mut active) = self.active.take() {
            active.file.flush()?;
            drop(active.file);
            let target =
                self.directory
                    .join(filename(active.first, active.last, &active.nonce, "jsonl"));
            if target.try_exists()? {
                return Err(io_error("segment destination exists"));
            }
            fs::rename(active.path, target)?;
        }
        Ok(())
    }

    fn recover(&mut self) -> io::Result<()> {
        for entry in fs::read_dir(&self.directory)? {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                continue;
            }
            let path = entry.path();
            let Some((first, _, nonce, extension)) = segment(&path) else {
                continue;
            };
            if extension == "jsonl.gz.tmp" {
                fs::remove_file(path)?;
                continue;
            }
            if extension != "active" {
                continue;
            }
            let file = File::open(&path)?;
            // A newly reduced rotation size must not hide valid older segments.
            if file.metadata()?.len() > MAX_SEGMENT_BYTES {
                continue;
            }
            let mut content = Vec::new();
            file.take(MAX_SEGMENT_BYTES.saturating_add(1))
                .read_to_end(&mut content)?;
            let mut valid = 0;
            let mut last = first;
            for line in content.split_inclusive(|b| *b == b'\n') {
                if line.last() != Some(&b'\n') {
                    break;
                }
                let Ok(event) = serde_json::from_slice::<ReadTraceEvent>(line) else {
                    break;
                };
                if event.timestamp.timestamp_millis() < first {
                    break;
                }
                last = last.max(event.timestamp.timestamp_millis());
                valid += line.len();
            }
            if valid == 0 {
                continue;
            }
            let target = self.directory.join(filename(first, last, &nonce, "jsonl"));
            if !target.try_exists()? {
                OpenOptions::new()
                    .write(true)
                    .open(&path)?
                    .set_len(valid as u64)?;
                fs::rename(path, target)?;
            }
        }
        Ok(())
    }

    pub(super) fn maintain(&mut self, now: i64) -> io::Result<()> {
        let ms = |seconds: u64| seconds.saturating_mul(1000).min(i64::MAX as u64) as i64;
        if self
            .active
            .as_ref()
            .is_some_and(|a| now.saturating_sub(a.first) >= ms(self.config.read_trace_segment_secs))
        {
            self.close()?;
        }
        for entry in fs::read_dir(&self.directory)? {
            let entry = entry?;
            if !entry.file_type()?.is_file() {
                continue;
            }
            let path = entry.path();
            let Some((_, last, _, extension)) = segment(&path) else {
                continue;
            };
            if extension == "jsonl.gz.tmp" {
                continue;
            }
            if self
                .active
                .as_ref()
                .is_some_and(|active| active.path == path)
            {
                continue;
            }
            let age = now.saturating_sub(last);
            if age >= ms(self.config.read_trace_retention_secs) {
                fs::remove_file(path)?;
            } else if extension == "jsonl" && age >= ms(self.config.read_trace_raw_ttl_secs) {
                // Maintenance failure keeps the raw segment for a later sweep.
                let _ = self.compress(&path);
            }
        }
        Ok(())
    }

    fn compress(&self, raw: &Path) -> io::Result<()> {
        let target = raw.with_extension("jsonl.gz");
        // A crash between publication and unlink leaves both. Rebuilding the
        // archive from raw also repairs a pre-existing partial/corrupt archive.
        let temporary = raw.with_extension("jsonl.gz.tmp");
        let output = create_file(&temporary, true)?;
        let result = (|| {
            let mut encoder = GzEncoder::new(output, Compression::default());
            io::copy(&mut File::open(raw)?, &mut encoder)?;
            let archive = encoder.finish()?;
            archive.sync_all()?;
            drop(archive);
            #[cfg(windows)]
            if target.try_exists()? {
                fs::remove_file(&target)?;
            }
            fs::rename(&temporary, &target)?;
            #[cfg(unix)]
            File::open(&self.directory)?.sync_all()?;
            fs::remove_file(raw)
        })();
        if result.is_err() {
            let _ = fs::remove_file(temporary);
        }
        result
    }
}
