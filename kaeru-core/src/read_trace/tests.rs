use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Duration;

use chrono::DateTime;
use crossbeam_channel::bounded;
use flate2::read::GzDecoder;
use uuid::Uuid;

use super::{Message, ReadTrace, writer::Writer};
use crate::config::KaeruConfig;
use crate::models::ReadTraceEvent;

struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("kaeru-trace-test-{}", Uuid::now_v7()));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    fn config(&self) -> KaeruConfig {
        KaeruConfig {
            vault_path: self.0.join("vault"),
            read_trace_enabled: true,
            read_trace_dir: self.0.join("trace"),
            ..KaeruConfig::defaults()
        }
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn event(time: i64) -> ReadTraceEvent {
    ReadTraceEvent {
        operation: "read".into(),
        node_ids: vec!["node".into()],
        initiatives: vec!["project".into()],
        timestamp: DateTime::from_timestamp_millis(time).unwrap(),
    }
}
fn files(config: &KaeruConfig, suffix: &str) -> Vec<PathBuf> {
    fs::read_dir(&config.read_trace_dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.to_string_lossy().ends_with(suffix))
        .collect()
}

#[test]
fn retention_uses_event_age_at_exact_raw_and_total_boundaries() {
    let scratch = Scratch::new();
    let config = scratch.config();
    let start = 1_700_000_000_000;
    let mut writer = Writer::open_at(&config, start).unwrap();
    writer.append(&event(start)).unwrap();
    writer.append(&event(start + 120_000)).unwrap();
    writer.close().unwrap();
    fs::write(config.read_trace_dir.join("keep-me.jsonl"), "unrelated").unwrap();
    let end = start + 120_000;
    writer
        .maintain(end + config.read_trace_raw_ttl_secs as i64 * 1000 - 1)
        .unwrap();
    assert_eq!(files(&config, ".jsonl.gz").len(), 0);
    writer
        .maintain(end + config.read_trace_raw_ttl_secs as i64 * 1000)
        .unwrap();
    let archives = files(&config, ".jsonl.gz");
    assert_eq!(archives.len(), 1);
    let mut content = String::new();
    GzDecoder::new(fs::File::open(&archives[0]).unwrap())
        .read_to_string(&mut content)
        .unwrap();
    let restored: Vec<ReadTraceEvent> = content
        .lines()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    assert_eq!(restored, vec![event(start), event(end)]);
    writer
        .maintain(end + config.read_trace_retention_secs as i64 * 1000 - 1)
        .unwrap();
    assert_eq!(files(&config, ".jsonl.gz").len(), 1);
    writer
        .maintain(end + config.read_trace_retention_secs as i64 * 1000)
        .unwrap();
    assert!(files(&config, ".jsonl.gz").is_empty());
    assert_eq!(
        fs::read_to_string(config.read_trace_dir.join("keep-me.jsonl")).unwrap(),
        "unrelated"
    );
}

#[test]
fn idle_maintenance_rotates_and_compresses_active_data() {
    let scratch = Scratch::new();
    let config = scratch.config();
    let start = 1_700_000_000_000;
    let mut writer = Writer::open_at(&config, start).unwrap();
    writer.append(&event(start)).unwrap();
    assert_eq!(files(&config, ".active").len(), 1);
    writer
        .maintain(start + config.read_trace_raw_ttl_secs as i64 * 1000)
        .unwrap();
    assert!(files(&config, ".active").is_empty());
    assert_eq!(files(&config, ".jsonl.gz").len(), 1);
}

#[test]
fn failed_compression_preserves_the_raw_segment() {
    let scratch = Scratch::new();
    let config = scratch.config();
    let start = 1_700_000_000_000;
    let mut writer = Writer::open_at(&config, start).unwrap();
    writer.append(&event(start)).unwrap();
    writer.close().unwrap();
    let raw = files(&config, ".jsonl").pop().unwrap();
    let before = fs::read(&raw).unwrap();
    fs::create_dir(raw.with_extension("jsonl.gz.tmp")).unwrap();
    writer
        .maintain(start + config.read_trace_raw_ttl_secs as i64 * 1000)
        .unwrap();
    assert_eq!(fs::read(&raw).unwrap(), before);
}

#[test]
fn size_rotation_is_bounded_and_oversized_events_are_dropped_whole() {
    let scratch = Scratch::new();
    let mut config = scratch.config();
    let start = 1_700_000_000_000;
    config.read_trace_segment_bytes = serde_json::to_vec(&event(start)).unwrap().len() as u64 + 1;
    let mut writer = Writer::open_at(&config, start).unwrap();
    writer.append(&event(start)).unwrap();
    writer.append(&event(start + 1000)).unwrap();
    let mut oversized = event(start);
    oversized.node_ids.push("x".repeat(1000));
    writer.append(&oversized).unwrap();
    writer.close().unwrap();
    let raw = files(&config, ".jsonl");
    assert_eq!(raw.len(), 2);
    for path in raw {
        assert!(fs::metadata(path).unwrap().len() <= config.read_trace_segment_bytes);
    }
}

#[test]
fn a_raw_archive_pair_left_by_a_crash_is_recompressed_safely() {
    let scratch = Scratch::new();
    let config = scratch.config();
    let start = 1_700_000_000_000;
    let mut writer = Writer::open_at(&config, start).unwrap();
    writer.append(&event(start)).unwrap();
    writer.close().unwrap();
    let raw = files(&config, ".jsonl").pop().unwrap();
    fs::write(raw.with_extension("jsonl.gz"), b"bad old archive").unwrap();
    writer
        .maintain(start + config.read_trace_raw_ttl_secs as i64 * 1000)
        .unwrap();
    assert!(!raw.exists());
    let mut content = String::new();
    GzDecoder::new(fs::File::open(raw.with_extension("jsonl.gz")).unwrap())
        .read_to_string(&mut content)
        .unwrap();
    assert_eq!(
        serde_json::from_str::<ReadTraceEvent>(content.trim()).unwrap(),
        event(start)
    );
}

#[test]
fn restart_recovers_complete_lines_and_cleans_owned_temporary_files() {
    let scratch = Scratch::new();
    let config = scratch.config();
    let start = 1_700_000_000_000;
    let mut writer = Writer::open_at(&config, start).unwrap();
    writer.append(&event(start)).unwrap();
    drop(writer);
    let active = files(&config, ".active").pop().unwrap();
    OpenOptions::new()
        .append(true)
        .open(&active)
        .unwrap()
        .write_all(b"{\"partial")
        .unwrap();
    fs::write(active.with_extension("jsonl.gz.tmp"), b"interrupted gzip").unwrap();
    let _writer = Writer::open_at(&config, start).unwrap();
    assert!(files(&config, ".active").is_empty());
    assert!(files(&config, ".tmp").is_empty());
    let raw = files(&config, ".jsonl").pop().unwrap();
    let recovered: ReadTraceEvent =
        serde_json::from_str(fs::read_to_string(raw).unwrap().trim()).unwrap();
    assert_eq!(recovered, event(start));
}

#[test]
fn reducing_rotation_size_does_not_expire_a_recovered_segment_early() {
    let scratch = Scratch::new();
    let mut config = scratch.config();
    let start = 1_700_000_000_000;
    let mut writer = Writer::open_at(&config, start).unwrap();
    writer.append(&event(start)).unwrap();
    writer.append(&event(start + 120_000)).unwrap();
    drop(writer);
    config.read_trace_segment_bytes = serde_json::to_vec(&event(start)).unwrap().len() as u64 + 1;
    let now = start + config.read_trace_retention_secs as i64 * 1000;
    let _writer = Writer::open_at(&config, now).unwrap();
    assert_eq!(
        files(&config, ".jsonl.gz").len(),
        1,
        "youngest event has not expired"
    );
}

#[test]
fn unrecoverable_partial_active_segment_expires_on_startup() {
    let scratch = Scratch::new();
    let config = scratch.config();
    let start = 1_700_000_000_000;
    let mut writer = Writer::open_at(&config, start).unwrap();
    writer.append(&event(start)).unwrap();
    drop(writer);
    let active = files(&config, ".active").pop().unwrap();
    fs::write(&active, b"{incomplete").unwrap();
    let _writer = Writer::open_at(
        &config,
        start + config.read_trace_retention_secs as i64 * 1000,
    )
    .unwrap();
    assert!(!active.exists());
}

#[test]
fn only_one_writer_owns_a_directory_and_shutdown_releases_it() {
    let scratch = Scratch::new();
    let config = scratch.config();
    let writer = Writer::open(&config).unwrap();
    assert!(Writer::open(&config).is_err());
    drop(writer);
    let trace = ReadTrace::start(&config);
    trace.record(event(1_700_000_000_000));
    assert!(trace.flush(Duration::from_secs(2), true));
    assert!(Writer::open(&config).is_ok());
}

#[test]
fn disabled_invalid_and_vault_nested_paths_do_not_create_a_journal() {
    let scratch = Scratch::new();
    let mut config = scratch.config();
    config.read_trace_enabled = false;
    let trace = ReadTrace::start(&config);
    trace.record(event(0));
    assert!(!config.read_trace_dir.exists());
    config.read_trace_enabled = true;
    config.read_trace_dir = config.vault_path.join("logs");
    assert!(Writer::open(&config).is_err());
    assert!(!config.read_trace_dir.exists());
    config.read_trace_queue_capacity = 0;
    assert!(!ReadTrace::start(&config).enabled());
    config.read_trace_queue_capacity = usize::MAX;
    assert!(!ReadTrace::start(&config).enabled());
    config = scratch.config();
    config.read_trace_dir = PathBuf::new();
    assert!(!ReadTrace::start(&config).enabled());
}

#[cfg(unix)]
#[test]
fn symlink_ancestry_cannot_place_journal_data_in_the_vault() {
    let scratch = Scratch::new();
    let mut config = scratch.config();
    fs::create_dir_all(&config.vault_path).unwrap();
    let alias = scratch.0.join("alias");
    std::os::unix::fs::symlink(&config.vault_path, &alias).unwrap();
    config.read_trace_dir = alias.join("logs");
    assert!(Writer::open(&config).is_err());
    assert!(!config.vault_path.join("logs").exists());
}

#[test]
fn full_or_disconnected_queue_drops_events_without_waiting() {
    let (sender, receiver) = bounded(1);
    let trace = ReadTrace {
        sender: Some(sender),
    };
    let mut data = event(0);
    data.node_ids = vec!["b".into(), "a".into(), "a".into()];
    trace.record(data);
    trace.record(event(1));
    assert_eq!(receiver.len(), 1);
    let Message::Event(first) = receiver.recv().unwrap() else {
        panic!("event expected")
    };
    assert_eq!(first.node_ids, vec!["a", "b"]);
    drop(receiver);
    trace.record(event(2));
    assert!(!trace.flush(Duration::ZERO, false));
}
