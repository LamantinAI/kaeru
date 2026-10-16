//! Bounded, best-effort read telemetry. Only the worker touches the filesystem.

use std::thread;
use std::time::{Duration, Instant};

use chrono::Utc;
use crossbeam_channel::{Sender, bounded};

use crate::config::KaeruConfig;
use crate::config::read_trace::{MAX_QUEUE_CAPACITY, MAX_SEGMENT_BYTES};
use crate::models::ReadTraceEvent;

mod writer;

enum Message {
    Event(ReadTraceEvent),
    Flush(Sender<bool>),
    Shutdown(Sender<bool>),
}

#[derive(Default)]
pub(crate) struct ReadTrace {
    sender: Option<Sender<Message>>,
}

impl ReadTrace {
    pub(crate) fn enabled(&self) -> bool {
        self.sender.is_some()
    }
    pub(crate) fn start(config: &KaeruConfig) -> Self {
        if !config.read_trace_enabled
            || config.read_trace_dir.as_os_str().is_empty()
            || config.read_trace_queue_capacity == 0
            || config.read_trace_queue_capacity > MAX_QUEUE_CAPACITY
            || config.read_trace_segment_bytes > MAX_SEGMENT_BYTES
            || config.read_trace_segment_secs == 0
            || config.read_trace_segment_bytes == 0
            || config.read_trace_maintenance_interval_secs == 0
            || config.read_trace_raw_ttl_secs > config.read_trace_retention_secs
        {
            return Self::default();
        }
        let (sender, receiver) = bounded(config.read_trace_queue_capacity);
        let config = config.clone();
        let spawned = thread::Builder::new()
            .name("kaeru-read-trace".into())
            .spawn(move || {
                let Ok(mut writer) = writer::Writer::open(&config) else {
                    return;
                };
                let interval = Duration::from_secs(config.read_trace_maintenance_interval_secs);
                let mut maintenance = Instant::now();
                loop {
                    if maintenance.elapsed() >= interval {
                        let _ = writer.maintain(Utc::now().timestamp_millis());
                        maintenance = Instant::now();
                    }
                    match receiver.recv_timeout(interval.saturating_sub(maintenance.elapsed())) {
                        Ok(Message::Event(event)) => {
                            // Leave a possibly partial active segment for recovery.
                            if writer.append(&event).is_err() {
                                return;
                            }
                        }
                        Ok(Message::Flush(ack)) => {
                            let _ = ack.send(writer.flush().is_ok());
                        }
                        Ok(Message::Shutdown(ack)) => {
                            let ok = writer.close().is_ok();
                            drop(writer); // Release ownership before acknowledging shutdown.
                            let _ = ack.send(ok);
                            return;
                        }
                        Err(crossbeam_channel::RecvTimeoutError::Timeout) => {}
                        Err(crossbeam_channel::RecvTimeoutError::Disconnected) => break,
                    }
                }
                let _ = writer.close();
            });
        if spawned.is_err() {
            return Self::default();
        }
        // Dropping the join handle detaches: a last Store drop cannot block a read.
        Self {
            sender: Some(sender),
        }
    }

    pub(crate) fn record(&self, mut event: ReadTraceEvent) {
        if let Some(sender) = &self.sender {
            event.node_ids.sort_unstable();
            event.node_ids.dedup();
            event.initiatives.sort_unstable();
            event.initiatives.dedup();
            let _ = sender.try_send(Message::Event(event));
        }
    }

    pub(crate) fn flush(&self, timeout: Duration, shutdown: bool) -> bool {
        let Some(sender) = &self.sender else {
            return true;
        };
        let started = Instant::now();
        let (ack, result) = bounded(1);
        let message = if shutdown {
            Message::Shutdown(ack)
        } else {
            Message::Flush(ack)
        };
        sender.send_timeout(message, timeout).is_ok()
            && result
                .recv_timeout(timeout.saturating_sub(started.elapsed()))
                .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests;
