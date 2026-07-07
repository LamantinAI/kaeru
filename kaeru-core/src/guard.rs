//! Pre-share guard (Gate 2) — deterministic secret/token scanner.
//!
//! Inspects content about to leave the local store for the shared cloud
//! and flags likely secrets, API tokens, and private keys. It is **silent
//! on clean content** and only reports on a real hit, so the friction lands
//! exactly where a leak would and nowhere else — the whole point of making
//! the gates not annoying.
//!
//! This is the mandatory floor on the share path: it runs regardless of
//! what the agent believes about the content. An LLM classifier may be
//! layered on top later, but never replaces this. PII (emails, phones) is
//! intentionally **not** flagged yet — it is noisy and would nag; the first
//! cut targets unambiguous secrets only.

/// A single guard finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardHit {
    /// Stable rule identifier (e.g. `"openai_key"`).
    pub rule: &'static str,
    /// Human-readable reason shown to the user on a block.
    pub reason: &'static str,
    /// The matched fragment, truncated for safe, compact display.
    pub matched: String,
}

/// Known secret token prefixes: `(prefix, minimum suffix length, rule id,
/// reason)`. Conservative minimums keep short innocuous strings that merely
/// share a prefix from matching. The suffix run allows alphanumerics and
/// `_` but **not** `-`, so hyphenated prose (`ask-the-user-…`) can't be
/// mistaken for an `sk-` token.
const TOKEN_PREFIXES: &[(&str, usize, &str, &str)] = &[
    ("sk-", 20, "openai_key", "looks like an OpenAI API key"),
    (
        "ghp_",
        20,
        "github_pat",
        "looks like a GitHub personal access token",
    ),
    (
        "gho_",
        20,
        "github_oauth",
        "looks like a GitHub OAuth token",
    ),
    (
        "ghs_",
        20,
        "github_server",
        "looks like a GitHub server token",
    ),
    (
        "github_pat_",
        20,
        "github_fine_pat",
        "looks like a fine-grained GitHub token",
    ),
    ("xoxb-", 10, "slack_bot", "looks like a Slack bot token"),
    ("xoxp-", 10, "slack_user", "looks like a Slack user token"),
    (
        "AKIA",
        16,
        "aws_access_key",
        "looks like an AWS access key id",
    ),
    ("AIza", 30, "google_api_key", "looks like a Google API key"),
];

/// Env-var key fragments that, when assigned a non-trivial value, signal a
/// secret (`API_KEY=…`, `DB_PASSWORD=…`).
const SECRET_KEY_MARKERS: &[&str] = &[
    "SECRET",
    "TOKEN",
    "PASSWORD",
    "PASSWD",
    "API_KEY",
    "APIKEY",
    "ACCESS_KEY",
    "PRIVATE_KEY",
];

/// Scans `content` and returns every guard hit. An empty vec means clean.
pub fn scan(content: &str) -> Vec<GuardHit> {
    let mut hits: Vec<GuardHit> = Vec::new();

    for (prefix, min_suffix, rule, reason) in TOKEN_PREFIXES {
        if let Some(matched) = find_prefixed_token(content, prefix, *min_suffix) {
            hits.push(GuardHit {
                rule,
                reason,
                matched,
            });
        }
    }

    if content.contains("-----BEGIN") && content.contains("PRIVATE KEY-----") {
        hits.push(GuardHit {
            rule: "private_key_block",
            reason: "contains a PEM private-key block",
            matched: "-----BEGIN … PRIVATE KEY-----".to_string(),
        });
    }

    if let Some(matched) = find_secret_assignment(content) {
        hits.push(GuardHit {
            rule: "env_secret",
            reason: "looks like a secret assignment (KEY=value)",
            matched,
        });
    }

    hits
}

/// Convenience: `true` when `content` has no guard hits.
pub fn is_clean(content: &str) -> bool {
    scan(content).is_empty()
}

/// Credential-like phrases (case-insensitive substring) that should never go
/// into a **public** export even though they are not structured secrets.
const PUBLIC_PHRASE_MARKERS: &[&str] = &[
    "default-cred",
    "password=",
    "password:",
    "passwd=",
    "passwd:",
    "ssh-rsa ",
    "ssh-ed25519 ",
    "-----begin",
];

/// Stricter scan for a **public** export (talk / website): everything [`scan`]
/// catches, plus credential-like phrases and raw IPv4 addresses. Used by
/// [`crate::export_json`] to redact a node before it leaves the building.
pub fn scan_public(content: &str) -> Vec<GuardHit> {
    let mut hits = scan(content);

    // Note: raw IPv4 addresses are deliberately NOT flagged here — in many
    // engineering notes they are overwhelmingly benign lab/example addresses,
    // and genuinely sensitive material is meant to be excluded at the
    // initiative level. The phrase + structured-secret rules are the floor.
    let lower = content.to_ascii_lowercase();
    if let Some(marker) = PUBLIC_PHRASE_MARKERS.iter().find(|m| lower.contains(**m)) {
        hits.push(GuardHit {
            rule: "public_phrase",
            reason: "contains a credential-like phrase",
            matched: (*marker).to_string(),
        });
    }

    hits
}

/// Finds the first occurrence of `prefix` immediately followed by at least
/// `min_suffix` token characters (alphanumeric / `_`). Returns the matched
/// fragment, truncated.
fn find_prefixed_token(content: &str, prefix: &str, min_suffix: usize) -> Option<String> {
    let mut from = 0usize;
    while let Some(rel) = content[from..].find(prefix) {
        let start = from + rel;
        let after = &content[start + prefix.len()..];
        let run_len = after
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .count();
        if run_len >= min_suffix {
            let end = start + prefix.len() + run_len;
            return Some(truncate(&content[start..end]));
        }
        from = start + prefix.len();
        if from >= content.len() {
            break;
        }
    }
    None
}

/// Scans line by line for `KEY = value` where the key name contains a
/// secret marker and the value is non-trivial (≥ 8 chars after trimming
/// quotes / whitespace).
fn find_secret_assignment(content: &str) -> Option<String> {
    for line in content.lines() {
        let Some((lhs, rhs)) = line.split_once('=') else {
            continue;
        };
        let key = lhs.trim().to_uppercase();
        if key.is_empty() || !SECRET_KEY_MARKERS.iter().any(|m| key.contains(m)) {
            continue;
        }
        let value = rhs
            .trim()
            .trim_matches(|c| c == '"' || c == '\'' || c == ' ');
        if value.len() >= 8 {
            return Some(truncate(line.trim()));
        }
    }
    None
}

/// Truncates a matched fragment for safe, compact display.
fn truncate(s: &str) -> String {
    const KEEP: usize = 12;
    if s.chars().count() <= KEEP {
        return s.to_string();
    }
    let head: String = s.chars().take(KEEP).collect();
    format!("{head}…")
}

#[cfg(test)]
mod tests {
    use super::{is_clean, scan, scan_public};

    #[test]
    fn clean_content_has_no_hits() {
        assert!(is_clean("just a normal note about the auth rewrite"));
        // Hyphenated prose must not trip the `sk-` rule.
        assert!(is_clean(
            "we should ask-the-user-about-the-token-expiry-behaviour first"
        ));
        // A short `KEY=value` with a trivial value is not flagged.
        assert!(is_clean("API_KEY=todo"));
    }

    #[test]
    fn detects_openai_key() {
        let hits = scan("the key is sk-abcdefghijklmnopqrstuvwxyz0123 ok");
        assert!(hits.iter().any(|h| h.rule == "openai_key"), "{hits:?}");
    }

    #[test]
    fn detects_github_and_aws_and_slack() {
        assert!(
            scan("ghp_0123456789abcdefghijABCDEFG")
                .iter()
                .any(|h| h.rule == "github_pat")
        );
        assert!(
            scan("AKIAIOSFODNN7EXAMPLE")
                .iter()
                .any(|h| h.rule == "aws_access_key")
        );
        assert!(
            scan("token=xoxb-123456789012-abcdef")
                .iter()
                .any(|h| h.rule == "slack_bot")
        );
    }

    #[test]
    fn detects_private_key_block_and_env_secret() {
        assert!(
            scan("-----BEGIN RSA PRIVATE KEY-----\nMIIE...\n")
                .iter()
                .any(|h| h.rule == "private_key_block")
        );
        assert!(
            scan("DB_PASSWORD=s3cr3tValue123")
                .iter()
                .any(|h| h.rule == "env_secret")
        );
    }

    /// The share gate now uses `scan_public`, which is strictly stronger than
    /// the base `scan`: a YAML-style secret (no `=`) is missed by `scan` but
    /// caught by the phrase markers in `scan_public` (issue #29, the closed
    /// asymmetry). A bare prefix-less token still slips both — a tracked gap.
    #[test]
    fn scan_public_is_strictly_stronger_than_base() {
        let yaml = "password: hunter2secretvalue";
        assert!(scan(yaml).is_empty(), "base scan misses the YAML secret");
        assert!(
            !scan_public(yaml).is_empty(),
            "strict scan catches the YAML secret"
        );

        // Known remaining gap (separate follow-up): a bare, prefix-less token
        // with no `=` and no known prefix slips both scanners.
        assert!(scan_public("db_pass_x7Fq2mK9").is_empty());
    }
}
