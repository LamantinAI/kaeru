//! Hypothesis-experiment cycle: `claim`, `evidence`, `confirm`, `refute`,
//! `inconclusive`.
//!
//! The cycle was designed prospectively — state a guess, run an experiment,
//! record a verdict — and agents do not work that way. By the time an agent
//! writes to memory the check has already happened in-session, so all three
//! stages land in one moment. The old shape asked for three calls at three
//! different times, and got: `test` never called once in 76k calls, fifteen
//! of twenty-one hypotheses open forever, and seven of them carrying the
//! verdict in their prose ("REFUTED", "VERDICT: PARTIAL") while the tag still
//! said `open`. Thirteen of twenty-three claims were written by one-shot
//! subagents that die immediately — for them the three-step cycle was not
//! slow, it was impossible.
//!
//! So the retrospective shape is now first-class: `claim <text> --verdict
//! refuted --by <evidence>` writes a settled hypothesis in one call.
//! Prospective still works unchanged — a `claim` with no verdict is an open
//! question.
//!
//! What does **not** change is that evidence stays a *node* joined by a typed
//! edge, never a string field on the claim. A verdict with its reasoning
//! inlined would be a log line; the point of the graph is that the next agent
//! can walk from the claim to what convinced you.

use kaeru_core::{EdgeType, HypothesisStatus, Store, Visibility, get_visibility};
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::utils::{
    arc_closed_hint, capture_result, claim_verdict_hint, derive_auto_name, parse_layer,
    resolve_name, resolve_name_or_id, text, to_mcp, with_initiative,
};

/// `↳ …` for a verdict recorded with nothing to point at.
///
/// A verdict with no evidence node is still better than an `open` tag beside
/// a body shouting REFUTED — the tag is what every read surface trusts. But
/// it is a claim without a citation, so the result says so and names the two
/// ways to supply one.
const NO_EVIDENCE_HINT: &str = "\n↳ no evidence node attached — write what convinced you \
     (`evidence <claim> --method \"…\"`, or an `episode`), then re-run the verdict with `--by` \
     so the trail leads somewhere.";

pub fn claim(
    store: &Store,
    text_arg: &str,
    about: Option<&str>,
    verdict: Option<&str>,
    by: Option<&str>,
    layer: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let auto_name = derive_auto_name(text_arg, "claim");
        let layer = parse_layer(layer)?;
        let status = match verdict {
            Some(v) => v.parse::<HypothesisStatus>().map_err(to_mcp)?,
            None => HypothesisStatus::Open,
        };

        // The status is stamped at creation rather than set by a follow-up
        // call: validities are whole seconds, and a create-then-update inside
        // one second leaves an assert and a retract that cannot be ordered.
        let id = kaeru_core::formulate_hypothesis_with_status(
            store, &auto_name, text_arg, layer, status,
        )
        .map_err(to_mcp)?;

        if let Some(a) = about {
            let target = resolve_name(store, a)?;
            kaeru_core::link(store, &id, &target, EdgeType::RefersTo).map_err(to_mcp)?;
        }

        // A verdict at creation still earns its verdict edge, so the evidence
        // is reachable from the claim exactly as it would be after `confirm`.
        let by_id = match by {
            Some(b) => Some(resolve_name_or_id(store, b)?),
            None => None,
        };
        if status.is_verdict() {
            kaeru_core::update_hypothesis_status(store, &id, status, by_id.as_ref())
                .map_err(to_mcp)?;
        }

        let msg = match status {
            HypothesisStatus::Open => format!("claimed: {auto_name} — {id}"),
            settled => format!("claimed {}: {auto_name} — {id}", settled.as_str()),
        };
        if status.is_verdict() {
            let tail = if by_id.is_none() {
                NO_EVIDENCE_HINT
            } else {
                ""
            };
            return Ok(text(&format!("{msg}{tail}")));
        }
        Ok(capture_result(
            store,
            &id,
            initiative,
            &format!("{msg}{}", claim_verdict_hint(&auto_name)),
        ))
    })
}

/// Registers what was actually checked — renamed from `test`, which was never
/// called once.
///
/// Two faults, both fixed by the name. `test` reads as `cargo test` / `pytest`
/// to an agent that spends its day running suites; and its own documentation
/// disagreed about tense — the description said "Run an experiment" (future)
/// while the parameter said "How the experiment was conducted" (past). The
/// past tense is the true one: this records a check that already happened.
///
/// `node` registers an existing node as the evidence instead of writing a new
/// one, which is what the single session that honestly lived the cycle did —
/// it wrote ordinary episodes and pointed the verdict at them.
pub fn evidence(
    store: &Store,
    hypothesis: &str,
    method: Option<&str>,
    node: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let hyp_id = resolve_name_or_id(store, hypothesis)?;
        match (node, method) {
            (Some(existing), _) => {
                let node_id = resolve_name_or_id(store, existing)?;
                kaeru_core::link(store, &node_id, &hyp_id, EdgeType::Targets).map_err(to_mcp)?;
                Ok(text(&format!(
                    "evidence: {existing} now targets {hypothesis}\n↳ record the verdict: \
                     `confirm`/`refute`/`inconclusive` {hypothesis} --by {existing}."
                )))
            }
            (None, Some(m)) => {
                let auto_name = derive_auto_name(m, "evidence");
                let exp_id =
                    kaeru_core::run_experiment(store, &hyp_id, &auto_name, m).map_err(to_mcp)?;
                Ok(text(&format!(
                    "evidence: {auto_name} — {exp_id}\n↳ record the verdict: \
                     `confirm`/`refute`/`inconclusive` {hypothesis} --by {auto_name}."
                )))
            }
            (None, None) => Err(to_mcp(kaeru_core::Error::Invalid(
                "pass `method` to record what you did, or `node` to point at an existing result"
                    .to_string(),
            ))),
        }
    })
}

/// The three verdicts share one implementation — they differ only in the
/// status they land on, and `inconclusive` exists because the status did
/// (`HypothesisStatus::Inconclusive` has always been in core) while no tool
/// would set it. Agents typed "PARTIAL" into the body instead.
fn verdict(
    store: &Store,
    hypothesis: &str,
    by: Option<&str>,
    status: HypothesisStatus,
    label: &str,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    with_initiative(store, initiative, || {
        let hyp_id = resolve_name_or_id(store, hypothesis)?;
        let by_id = match by {
            Some(b) => Some(resolve_name_or_id(store, b)?),
            None => None,
        };
        kaeru_core::update_hypothesis_status(store, &hyp_id, status, by_id.as_ref())
            .map_err(to_mcp)?;

        let mut msg = format!("{label}: {hypothesis}");
        msg.push_str(&arc_closed_hint(store, &hyp_id));
        // Inconclusive writes no verdict edge by design, so it is not missing
        // one — only supported/refuted can be left uncited.
        if by_id.is_none() && status != HypothesisStatus::Inconclusive {
            msg.push_str(NO_EVIDENCE_HINT);
        }
        if get_visibility(store, &hyp_id).map_err(to_mcp)? == Visibility::Shared {
            msg.push_str(
                "\n⚠ cloud copy is stale — run `share` on this node to push the new version.",
            );
        }
        Ok(text(&msg))
    })
}

pub fn confirm(
    store: &Store,
    hypothesis: &str,
    by: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    verdict(
        store,
        hypothesis,
        by,
        HypothesisStatus::Supported,
        "confirmed",
        initiative,
    )
}

pub fn refute(
    store: &Store,
    hypothesis: &str,
    by: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    verdict(
        store,
        hypothesis,
        by,
        HypothesisStatus::Refuted,
        "refuted",
        initiative,
    )
}

pub fn inconclusive(
    store: &Store,
    hypothesis: &str,
    by: Option<&str>,
    initiative: Option<&str>,
) -> Result<CallToolResult, McpError> {
    verdict(
        store,
        hypothesis,
        by,
        HypothesisStatus::Inconclusive,
        "inconclusive",
        initiative,
    )
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;

    use kaeru_core::{EpisodeKind, Significance, Store};
    use rmcp::model::CallToolResult;

    use super::{claim, evidence, inconclusive, refute};

    fn text_of(r: CallToolResult) -> String {
        r.content
            .iter()
            .filter_map(|c| c.as_text().map(|t| t.text.clone()))
            .collect::<Vec<_>>()
            .join("")
    }

    fn store_t() -> Store {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        store
    }

    fn tags_of(store: &Store, name: &str) -> Vec<String> {
        let id = kaeru_core::recall_id_by_name(store, name)
            .expect("resolve")
            .expect("exists");
        kaeru_core::read_node_full(store, &id)
            .expect("read")
            .expect("exists")
            .tags
    }

    /// The shape #54 is about: the check already ran, so the claim and its
    /// verdict are one call. The status has to land on the TAG — the body is
    /// where verdicts used to go to die, invisible to every read surface.
    #[test]
    fn a_claim_can_be_born_settled() {
        let store = store_t();
        let out = text_of(
            claim(
                &store,
                "the cache pays for itself",
                None,
                Some("refuted"),
                None,
                None,
                Some("t"),
            )
            .unwrap(),
        );
        assert!(out.starts_with("claimed refuted:"), "{out}");

        let name = out
            .split(": ")
            .nth(1)
            .and_then(|r| r.split(" — ").next())
            .unwrap()
            .to_string();
        let tags = tags_of(&store, &name);
        assert!(tags.iter().any(|t| t == "status:refuted"), "{tags:?}");
        assert!(
            !tags.iter().any(|t| t == "status:open"),
            "and never both: {tags:?}"
        );
    }

    /// A verdict with nothing to point at is still recorded — an `open` tag
    /// beside a body shouting REFUTED is the worse outcome — but the result
    /// says the citation is missing.
    #[test]
    fn a_verdict_without_evidence_is_recorded_and_flagged() {
        let store = store_t();
        let out = text_of(
            claim(
                &store,
                "it is faster",
                None,
                Some("supported"),
                None,
                None,
                Some("t"),
            )
            .unwrap(),
        );
        assert!(out.contains("no evidence node attached"), "{out}");
        assert!(
            out.contains("`evidence <claim>"),
            "and names the fix: {out}"
        );
    }

    /// `inconclusive` writes no verdict edge by design, so it must not nag
    /// about a missing one.
    #[test]
    fn inconclusive_is_a_verdict_not_a_missing_citation() {
        let store = store_t();
        claim(&store, "it is faster", None, None, None, None, Some("t")).unwrap();
        let name = kaeru_core::tagged(&store, "status:open").unwrap()[0]
            .name
            .clone();
        sleep(Duration::from_millis(1100));

        let out = text_of(inconclusive(&store, &name, None, Some("t")).unwrap());
        assert!(out.starts_with("inconclusive:"), "{out}");
        assert!(
            !out.contains("no evidence node"),
            "it never wanted one: {out}"
        );
        let tags = tags_of(&store, &name);
        assert!(tags.iter().any(|t| t == "status:inconclusive"), "{tags:?}");
    }

    /// The prospective path still works end to end, and `by` still links.
    #[test]
    fn the_prospective_path_still_works() {
        let store = store_t();
        claim(&store, "the index helps", None, None, None, None, Some("t")).unwrap();
        let name = kaeru_core::tagged(&store, "status:open").unwrap()[0]
            .name
            .clone();
        kaeru_core::write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "the-measurement",
            "it did not help",
        )
        .unwrap();
        sleep(Duration::from_millis(1100));

        let out = text_of(refute(&store, &name, Some("the-measurement"), Some("t")).unwrap());
        assert!(out.starts_with("refuted:"), "{out}");
        assert!(!out.contains("no evidence node"), "it was cited: {out}");
        assert!(tags_of(&store, &name).iter().any(|t| t == "status:refuted"));
    }

    /// `evidence` registers something already captured rather than demanding
    /// a fresh write-up — what the one session that lived the cycle actually
    /// did.
    #[test]
    fn evidence_can_point_at_a_node_you_already_wrote() {
        let store = store_t();
        claim(&store, "the index helps", None, None, None, None, Some("t")).unwrap();
        let name = kaeru_core::tagged(&store, "status:open").unwrap()[0]
            .name
            .clone();
        kaeru_core::write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "the-measurement",
            "numbers here",
        )
        .unwrap();

        let out =
            text_of(evidence(&store, &name, None, Some("the-measurement"), Some("t")).unwrap());
        assert!(out.contains("now targets"), "{out}");
        assert!(
            out.contains("--by the-measurement"),
            "closes the loop: {out}"
        );
    }

    /// Neither `method` nor `node` is not a silent no-op.
    #[test]
    fn evidence_with_nothing_to_record_says_so() {
        let store = store_t();
        claim(&store, "the index helps", None, None, None, None, Some("t")).unwrap();
        let name = kaeru_core::tagged(&store, "status:open").unwrap()[0]
            .name
            .clone();
        let err = evidence(&store, &name, None, None, Some("t")).unwrap_err();
        assert!(
            err.to_string().contains("pass `method`"),
            "{}",
            err.to_string()
        );
    }
}
