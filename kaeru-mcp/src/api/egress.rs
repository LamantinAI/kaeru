//! What may leave the process, and in what shape.
//!
//! Every response the API returns passes through here. That is the whole point
//! of the module: redaction and scope are policy, and policy scattered across
//! handlers is policy nobody can audit. One place to read means one place to
//! get wrong, and one place to fix.
//!
//! Two rules, both operator-driven — there are **no** vault-specific names in
//! this source:
//!
//! - **Scope.** [`ApiConfig::allow`] is the authoritative ceiling. Empty (the
//!   default) exports nothing. A request may *narrow* within it; it can never
//!   widen it, so a caller cannot reach an initiative the operator did not opt
//!   into. [`ApiConfig::deny`] is always applied on top, and a request may add
//!   to it.
//! - **Redaction.** Every node passes the public secret/credential guard, and
//!   `local` nodes stay on the machine unless the operator says otherwise.
//!
//! Cross-origin reads are opt-in for the same reason: the dev vite proxy and a
//! baked snapshot are both same-origin, so a daemon that answers anyone by
//! default would be answering a question nobody asked it.

use axum::http::header;
use axum::response::Response;
use kaeru_core::ExportOpts;
use kaeru_core::guard::scan_public;

/// Endpoint configuration, all operator-controlled.
#[derive(Clone, Default)]
pub struct ApiConfig {
    /// Authoritative allow-list ceiling (`KAERU_MCP_VIZ_INITIATIVES`). **Empty =
    /// export nothing** — the operator must opt in. Requests can only narrow it.
    pub allow: Vec<String>,
    /// Always-applied deny-list (`KAERU_MCP_VIZ_DENY`).
    pub deny: Vec<String>,
    /// Export `local` nodes too (`KAERU_MCP_VIZ_INCLUDE_LOCAL`). Default false —
    /// only `shared` nodes leave the daemon.
    pub include_local: bool,
    /// `Access-Control-Allow-Origin` value (`KAERU_MCP_VIZ_ALLOW_ORIGIN`).
    /// `None` (default) sends no CORS header — same-origin only.
    pub allow_origin: Option<String>,
}

/// What a request asked to narrow to. Both fields are CSV of names or `*`
/// globs, exactly as they arrive on the query string.
#[derive(Debug, Default)]
pub struct Narrowing {
    /// Names / globs to narrow *within* the configured allow-list.
    pub initiatives: Option<String>,
    /// Names / globs to deny on top of the configured deny-list.
    pub deny: Option<String>,
}

impl ApiConfig {
    /// Turns operator policy plus a request's narrowing into export options.
    ///
    /// `allow_initiatives` is always `Some`, never `None`: `None` would mean
    /// "no ceiling", and a misconfigured daemon must export nothing rather
    /// than everything.
    pub fn export_opts(&self, narrow: &Narrowing, include_bodies: bool) -> ExportOpts {
        let mut deny = self.deny.clone();
        if let Some(csv) = narrow.deny.as_deref() {
            deny.extend(csv_to_vec(csv));
        }
        ExportOpts {
            allow_initiatives: Some(self.allow.clone()),
            restrict_initiatives: narrow.initiatives.as_deref().map(csv_to_vec),
            deny_initiatives: deny,
            shared_only: !self.include_local,
            include_bodies,
            redact: true,
        }
    }

    /// Stamps the response with whatever cross-origin permission the operator
    /// granted. Called on the way out of every handler; when no origin is
    /// configured this does nothing, which is the default.
    pub fn finish(&self, resp: &mut Response) {
        if let Some(origin) = self.allow_origin.as_deref()
            && let Ok(val) = origin.parse()
        {
            resp.headers_mut()
                .insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, val);
        }
    }
}

impl ApiConfig {
    /// Whether the operator opted this initiative into the surface at all.
    ///
    /// The whole-graph export gets the same decision applied per node inside
    /// `export_graph_json`; a per-initiative verb has to make it here, before
    /// it reads anything, because "which initiative" is the request itself.
    /// A caller outside the ceiling is answered `404`, not `403` — telling
    /// someone an initiative exists but is off-limits is telling them it
    /// exists.
    pub fn reaches(&self, initiative: &str) -> bool {
        self.allow.iter().any(|p| pat_match(initiative, p))
            && !self.deny.iter().any(|p| pat_match(initiative, p))
    }
}

impl ApiConfig {
    /// Whether a node may leave, judged by the initiatives it belongs to.
    ///
    /// The whole-graph export decides this per node inside `export_graph_json`
    /// and a board decides it from the name in the request; a node-addressed
    /// verb has neither, so it has to ask the junction. One reachable
    /// initiative is enough — the same rule the export applies, where a node
    /// survives if any of its initiatives clears the filter.
    ///
    /// A node attached to nothing is unreachable. That is deliberate: an
    /// unfiled node has no initiative to have been opted into, and the ceiling
    /// is written in initiatives.
    pub fn reaches_node(&self, initiatives: &[String]) -> bool {
        initiatives.iter().any(|i| self.reaches(i))
    }
}

/// Exact match, or prefix match when `pattern` ends with `*`.
///
/// Deliberately a second copy of the rule `export_json` applies — the same
/// deliberate duplication the cloud clients carry, for the same reason: the
/// alternative is widening `kaeru-core`'s public surface to share four lines.
/// The tests below pin the behaviour so the two cannot drift silently.
fn pat_match(name: &str, pattern: &str) -> bool {
    match pattern.strip_suffix('*') {
        Some(prefix) => name.starts_with(prefix),
        None => name == pattern,
    }
}

/// A name that tripped the public guard, replaced by a marker.
///
/// Redaction never removes the *card* — a task the operator cannot be shown
/// is still a task, and silently dropping it would make the board's counts
/// lie. Only the text goes.
pub fn redact_name(name: &str, kind: &str) -> (String, bool) {
    if scan_public(name).is_empty() {
        (name.to_string(), false)
    } else {
        (format!("\u{27e8}redacted {kind}\u{27e9}"), true)
    }
}

/// An excerpt that tripped the public guard, dropped entirely.
pub fn redact_excerpt(excerpt: Option<&str>) -> (Option<String>, bool) {
    match excerpt {
        Some(text) if !scan_public(text).is_empty() => (None, true),
        other => (other.map(str::to_string), false),
    }
}

fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> ApiConfig {
        ApiConfig {
            allow: vec!["alpha".into(), "beta".into()],
            deny: vec!["secret".into()],
            include_local: false,
            allow_origin: None,
        }
    }

    #[test]
    fn the_configured_allow_list_is_always_the_ceiling() {
        // A request naming an initiative outside the ceiling narrows to it;
        // the ceiling is still handed to the exporter, so the intersection —
        // not the request — decides what is reachable.
        let narrow = Narrowing {
            initiatives: Some("gamma".into()),
            deny: None,
        };
        let opts = cfg().export_opts(&narrow, false);
        assert_eq!(
            opts.allow_initiatives,
            Some(vec!["alpha".to_string(), "beta".to_string()])
        );
        assert_eq!(opts.restrict_initiatives, Some(vec!["gamma".to_string()]));
    }

    #[test]
    fn an_empty_allow_list_stays_empty_rather_than_becoming_no_ceiling() {
        let opts = ApiConfig::default().export_opts(&Narrowing::default(), false);
        assert_eq!(opts.allow_initiatives, Some(Vec::new()));
    }

    #[test]
    fn a_request_adds_to_the_deny_list_and_never_replaces_it() {
        let narrow = Narrowing {
            initiatives: None,
            deny: Some(" extra , , more ".into()),
        };
        let opts = cfg().export_opts(&narrow, false);
        assert_eq!(opts.deny_initiatives, vec!["secret", "extra", "more"]);
    }

    #[test]
    fn the_ceiling_gates_a_named_initiative_the_same_way_it_gates_a_node() {
        let c = cfg();
        assert!(c.reaches("alpha"));
        assert!(!c.reaches("gamma"), "outside the allow-list");
        assert!(!c.reaches("secret"), "denied even if it were allowed");
    }

    #[test]
    fn a_trailing_star_is_a_prefix_and_nothing_else_is() {
        let c = ApiConfig {
            allow: vec!["proj-*".into()],
            ..ApiConfig::default()
        };
        assert!(c.reaches("proj-one"));
        assert!(c.reaches("proj-"));
        assert!(
            !c.reaches("otherproj-one"),
            "* is a suffix glob, not a substring match"
        );
    }

    #[test]
    fn an_unconfigured_daemon_reaches_nothing() {
        assert!(!ApiConfig::default().reaches("alpha"));
    }

    #[test]
    fn a_node_leaves_if_any_of_its_initiatives_is_reachable() {
        let c = cfg();
        assert!(
            c.reaches_node(&["gamma".into(), "alpha".into()]),
            "one is enough"
        );
        assert!(!c.reaches_node(&["gamma".into()]));
        assert!(
            !c.reaches_node(&[]),
            "an unfiled node has no permission to inherit"
        );
        // deny is judged per initiative, so being in a denied one alongside a
        // reachable one does not withdraw the reachable one's permission
        assert!(c.reaches_node(&["alpha".into(), "secret".into()]));
    }

    #[test]
    fn redaction_replaces_a_name_and_drops_an_excerpt() {
        let secret = "token AKIAIOSFODNN7EXAMPLE";
        let (name, hit) = redact_name(secret, "task");
        assert!(hit && name.contains("redacted task"));
        let (excerpt, hit) = redact_excerpt(Some(secret));
        assert!(hit && excerpt.is_none());
        let (clean, hit) = redact_name("a plain task", "task");
        assert!(!hit && clean == "a plain task");
    }

    #[test]
    fn redaction_is_not_optional() {
        let opts = ApiConfig::default().export_opts(&Narrowing::default(), true);
        assert!(opts.redact);
        assert!(opts.shared_only);
    }
}
