//! "You are behind, and here is what it is costing you" (#99).
//!
//! The daemon has always asked its cloud for a version and compared it with
//! its own — and written the answer to `tracing::warn!`, which is a daemon
//! log, which nobody reads. Half a mechanism that has never once caused an
//! upgrade. Detection was never the problem; delivery was, which is the same
//! disease #90 named: a message that fires where nobody looks is
//! indistinguishable from no message.
//!
//! So the check goes where the agent already looks — into `awake`, through
//! the same "deliver once, then clear" slot the hygiene headline uses. No new
//! verb, nothing for an agent to learn.
//!
//! **The wording carries it.** The #79 finding is that a line converts when
//! it names a *debt* and not an opportunity: `open reviews (2)` produced 8
//! calls; "trails exist — read one" produced 0 from 7 deliveries. So the line
//! does not say a new version is available. It says what the version being
//! run is doing wrong, quotes the release's own headline for why, and carries
//! the command for the channel this binary was actually installed through —
//! because the agent is the updater here, not the binary. A self-replacing
//! binary would be right for exactly one of the four ways kaeru is installed.
//!
//! **What leaves the machine:** one HTTPS GET to a public GitHub endpoint,
//! once a day, with no identifiers, no payload and no vault content. It never
//! delays startup and never blocks a verb: failure is silence. Set
//! `KAERU_MCP_UPDATE_CHECK=0` and none of it happens at all — a version check
//! that cannot be turned off is a fair objection to a local-first tool, not a
//! detail.

use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::Value;
use tokio_util::sync::CancellationToken;

/// The public listing the check reads. No auth, no identifiers.
const RELEASES_URL: &str = "https://api.github.com/repos/LamantinAI/kaeru/releases?per_page=20";

/// Long enough that the question is asked roughly once a day per daemon, and
/// nowhere near any hot path.
const CHECK_EVERY: Duration = Duration::from_secs(24 * 60 * 60);

/// A missed check costs nothing, so it waits rather than retrying.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

/// The line waiting to be handed to the next `awake`, if any.
///
/// One slot, not a queue: a second check overwrites the first, because what
/// matters is the current gap, not its history.
pub type UpdateNotice = Arc<Mutex<Option<String>>>;

/// Whether the check runs at all.
pub fn enabled_from(raw: Option<String>) -> bool {
    !matches!(
        raw.unwrap_or_default().trim().to_ascii_lowercase().as_str(),
        "0" | "false" | "no" | "off"
    )
}

/// How this binary got here — which decides what the line tells the agent to
/// run, because the right command is different for each and a wrong one is
/// worse than none.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    /// `contrib/install/install.sh` — the user's own `~/.local/bin`, so the
    /// installer can replace it in place.
    Installer,
    /// An MCPB bundle: the host app put it there and manages it.
    Bundle,
    /// A build tree — `cargo install` from source is the honest instruction.
    Source,
}

impl Channel {
    /// The command an agent can offer to run, or the sentence that explains
    /// why there is no command to run.
    pub fn instruction(self) -> &'static str {
        match self {
            Channel::Installer => {
                "curl -fsSL https://raw.githubusercontent.com/LamantinAI/kaeru/main/contrib/install/install.sh | bash"
            }
            Channel::Bundle => {
                "this binary came from an .mcpb bundle — update it through the app that installed \
                 it; replacing the file by hand will be undone"
            }
            Channel::Source => {
                "cd <kaeru checkout> && git pull && cargo install --path kaeru-mcp --force"
            }
        }
    }
}

/// Reads the channel off the running binary's own path.
pub fn channel_of(exe: &Path) -> Channel {
    let path = exe.to_string_lossy();
    if path.contains(".mcpb")
        || path.contains("/Application Support/")
        || path.contains("\\AppData\\")
    {
        Channel::Bundle
    } else if path.contains("/.local/bin/") || path.contains("/.local/share/kaeru") {
        Channel::Installer
    } else {
        Channel::Source
    }
}

/// A release as the check cares about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    pub tag: String,
    /// The first line of the notes — our releases open with a sentence that
    /// says what the release is for, which is exactly the "why" this line
    /// needs and cannot invent.
    pub headline: String,
}

/// Parses the GitHub listing into releases, newest first, drafts skipped.
pub fn parse_releases(body: &str) -> Vec<Release> {
    serde_json::from_str::<Value>(body)
        .ok()
        .and_then(|v| v.as_array().cloned())
        .unwrap_or_default()
        .iter()
        .filter(|r| r.get("draft").and_then(Value::as_bool) != Some(true))
        .filter_map(|r| {
            let tag = r.get("tag_name").and_then(Value::as_str)?.to_string();
            let headline = r
                .get("body")
                .and_then(Value::as_str)
                .unwrap_or("")
                .lines()
                .map(str::trim)
                .find(|l| !l.is_empty() && !l.starts_with('#'))
                .unwrap_or("")
                .to_string();
            Some(Release { tag, headline })
        })
        .collect()
}

/// `v0.7.3` / `0.7.3` → `[0, 7, 3]`. Anything unparseable sorts as oldest,
/// so a tag we do not understand can never look newer than what is running.
fn parts(tag: &str) -> Vec<u64> {
    tag.trim_start_matches('v')
        .split(['.', '-', '+'])
        .map(|p| p.parse::<u64>().unwrap_or(0))
        .collect()
}

fn newer(tag: &str, than: &str) -> bool {
    parts(tag) > parts(than)
}

/// The line to hand to `awake`, or `None` when there is nothing owed.
///
/// Nothing is owed when the running version is current — and equally when
/// every newer release is one this reader cannot be hurt by, which the check
/// cannot know, so the headline is quoted rather than summarised: a release
/// that fixes nothing you are hitting should read as obviously skippable.
pub fn compose(current: &str, releases: &[Release], channel: Channel) -> Option<String> {
    let behind: Vec<&Release> = releases.iter().filter(|r| newer(&r.tag, current)).collect();
    let latest = behind.first()?;
    let count = behind.len();
    let plural = if count == 1 { "release" } else { "releases" };
    let mut line = format!(
        "⚠ running kaeru {current}, {count} {plural} behind ({}).",
        latest.tag
    );
    if !latest.headline.is_empty() {
        line.push(' ');
        line.push_str(&latest.headline);
    }
    line.push_str("\n↳ ");
    line.push_str(channel.instruction());
    if channel != Channel::Bundle {
        line.push_str(" — ask the user before running it.");
    }
    Some(line)
}

/// Asks GitHub once. `None` on any failure: no network, no answer, no line.
async fn fetch(client: &reqwest::Client) -> Option<Vec<Release>> {
    let resp = client
        .get(RELEASES_URL)
        // GitHub requires a User-Agent. It carries no version and no id —
        // the point is to ask a public question, not to be counted.
        .header("User-Agent", "kaeru")
        .header("Accept", "application/vnd.github+json")
        .timeout(REQUEST_TIMEOUT)
        .send()
        .await
        .ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let body = resp.text().await.ok()?;
    let releases = parse_releases(&body);
    (!releases.is_empty()).then_some(releases)
}

/// Starts the daily check. Returns immediately; the first ask happens on the
/// spawned task, so a slow or unreachable GitHub cannot delay startup.
pub fn spawn(notice: UpdateNotice, cancel: CancellationToken) {
    if !enabled_from(std::env::var("KAERU_MCP_UPDATE_CHECK").ok()) {
        tracing::debug!("update check disabled by KAERU_MCP_UPDATE_CHECK");
        return;
    }
    let channel = std::env::current_exe()
        .map(|exe| channel_of(&exe))
        .unwrap_or(Channel::Source);
    tokio::spawn(async move {
        let client = reqwest::Client::new();
        loop {
            if let Some(releases) = fetch(&client).await
                && let Some(line) = compose(kaeru_core::version(), &releases, channel)
            {
                // Overwrites rather than queues: the current gap is the only
                // interesting one.
                if let Ok(mut slot) = notice.lock() {
                    *slot = Some(line);
                }
            }
            tokio::select! {
                _ = cancel.cancelled() => break,
                _ = tokio::time::sleep(CHECK_EVERY) => {}
            }
        }
    });
}

/// Takes the pending line, if there is one — delivered once, like the
/// hygiene headline it rides beside.
pub fn take(notice: &UpdateNotice) -> Option<String> {
    notice.lock().ok().and_then(|mut slot| slot.take())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{Channel, Release, channel_of, compose, enabled_from, parse_releases, take};

    fn releases() -> Vec<Release> {
        vec![
            Release {
                tag: "v0.7.4".into(),
                headline: "Six fixes in the substrate and the passes that read it.".into(),
            },
            Release {
                tag: "v0.7.3".into(),
                headline: "Mostly one thing: the Linux binary opens your vault again.".into(),
            },
            Release {
                tag: "v0.7.2".into(),
                headline: "A modest release by intent.".into(),
            },
        ]
    }

    #[test]
    fn a_current_version_owes_nothing() {
        assert_eq!(compose("0.7.4", &releases(), Channel::Installer), None);
        // And a build ahead of every published release is not "behind".
        assert_eq!(compose("0.8.0", &releases(), Channel::Installer), None);
    }

    /// The wording is the feature: a debt, what it is costing, and the
    /// command for THIS install — not "a new version is available" (#79).
    #[test]
    fn the_line_names_the_debt_the_reason_and_the_command() {
        let line = compose("0.7.2", &releases(), Channel::Installer).expect("behind");
        assert!(line.contains("running kaeru 0.7.2"), "{line}");
        assert!(line.contains("2 releases behind (v0.7.4)"), "{line}");
        assert!(
            line.contains("Six fixes in the substrate"),
            "it quotes the release's own reason: {line}"
        );
        assert!(line.contains("install.sh | bash"), "{line}");
        assert!(
            line.contains("ask the user before running it"),
            "the agent is the updater, and it asks first: {line}"
        );
    }

    #[test]
    fn one_release_behind_reads_as_one() {
        let line = compose("0.7.3", &releases(), Channel::Source).expect("behind");
        assert!(line.contains("1 release behind"), "{line}");
        assert!(line.contains("cargo install --path kaeru-mcp"), "{line}");
    }

    /// A bundle is not the user's to replace, so the line says so instead of
    /// handing over a command that the host app would undo.
    #[test]
    fn a_bundle_is_told_where_updates_come_from() {
        let line = compose("0.7.2", &releases(), Channel::Bundle).expect("behind");
        assert!(line.contains(".mcpb bundle"), "{line}");
        assert!(
            !line.contains("ask the user before running it"),
            "there is no command to run: {line}"
        );
    }

    #[test]
    fn the_channel_is_read_off_the_binarys_own_path() {
        assert_eq!(
            channel_of(&PathBuf::from("/home/x/.local/bin/kaeru-mcp")),
            Channel::Installer
        );
        assert_eq!(
            channel_of(&PathBuf::from(
                "/Users/x/Library/Application Support/Claude/kaeru-mcp"
            )),
            Channel::Bundle
        );
        assert_eq!(
            channel_of(&PathBuf::from("/home/x/code/kaeru/target/debug/kaeru-mcp")),
            Channel::Source
        );
    }

    #[test]
    fn a_tag_it_cannot_parse_never_looks_newer() {
        let odd = vec![Release {
            tag: "nightly".into(),
            headline: String::new(),
        }];
        assert_eq!(compose("0.7.4", &odd, Channel::Source), None);
    }

    #[test]
    fn drafts_and_missing_bodies_survive_parsing() {
        // Built rather than pasted: the notes carry newlines, and a literal
        // holding them is harder to read than the thing it is testing.
        let body = serde_json::json!([
            {"tag_name": "v0.9.0", "draft": true, "body": "unreleased"},
            {"tag_name": "v0.8.0", "body": "# kaeru 0.8.0\n\nThe one about verbs."},
            {"tag_name": "v0.7.9"},
        ])
        .to_string();
        let parsed = parse_releases(&body);
        assert_eq!(parsed.len(), 2, "the draft is skipped: {parsed:?}");
        assert_eq!(parsed[0].tag, "v0.8.0");
        assert_eq!(parsed[0].headline, "The one about verbs.");
        assert_eq!(parsed[1].headline, "", "a release with no notes is fine");
    }

    #[test]
    fn the_switch_is_off_only_when_it_says_so() {
        for off in ["0", "false", "no", "off", "OFF"] {
            assert!(!enabled_from(Some(off.into())), "{off}");
        }
        for on in ["", "1", "yes", "anything"] {
            assert!(enabled_from(Some(on.into())), "{on}");
        }
        assert!(enabled_from(None), "on unless asked otherwise");
    }

    #[test]
    fn the_line_is_delivered_once() {
        let notice = super::UpdateNotice::default();
        *notice.lock().unwrap() = Some("⚠ behind".into());
        assert_eq!(take(&notice).as_deref(), Some("⚠ behind"));
        assert_eq!(take(&notice), None, "delivered once, like the hygiene cue");
    }
}
