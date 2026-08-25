//! The hypothesis cycle and the review flow.

use kaeru_core::{
    EdgeType, HypothesisStatus, Layer, formulate_hypothesis_with_status, link, mark_resolved,
    mark_under_review, resolve_review, run_experiment, update_hypothesis_status,
};
use serde::Deserialize;
use serde_json::json;

use crate::{mem_tool, resolve};

#[derive(Debug, Deserialize)]
pub struct ClaimArgs {
    pub name: String,
    pub claim: String,
    #[serde(default)]
    pub verdict: Option<String>,
    #[serde(default)]
    pub by: Option<String>,
}

mem_tool!(
    /// `kaeru_claim` — formulate a falsifiable hypothesis.
    Claim,
    "kaeru_claim",
    "Record a falsifiable hypothesis. If you ALREADY know how it turned out — the usual case, \
     since you reach memory after the check has run — pass `verdict` (supported/refuted/\
     inconclusive) and `by` (the evidence node) and it lands settled in this one call. Without a \
     verdict it is an open question that keeps surfacing in `kaeru_awake` until one arrives.",
    ClaimArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "short name for the hypothesis" },
        "claim": { "type": "string", "description": "the claim, stated so it can be falsified" },
        "verdict": { "type": "string", "description": "optional: supported | refuted | inconclusive, when the answer is already known" },
        "by": { "type": "string", "description": "optional evidence node (name or id) the verdict rests on" }
    }, "required": ["name", "claim"] },
    |store, args| {
        let status = match args.verdict.as_deref() {
            Some(v) => match v.parse::<HypothesisStatus>() {
                Ok(s) => s,
                Err(e) => return json!({ "created": false, "error": e.to_string() }),
            },
            None => HypothesisStatus::Open,
        };
        // Stamped at creation, not by a follow-up call: validities are whole
        // seconds, and create-then-update inside one second leaves an assert
        // and a retract that cannot be ordered.
        match formulate_hypothesis_with_status(
            store, &args.name, &args.claim, Layer::default(), status,
        ) {
            Ok(id) => {
                let by = args.by.as_deref().map(|b| resolve(store, b));
                if status.is_verdict()
                    && let Err(e) = update_hypothesis_status(store, &id, status, by.as_ref())
                {
                    return json!({ "created": false, "error": e.to_string() });
                }
                let mut out = json!({
                    "created": true, "id": id, "status": status.as_str()
                });
                out["hint"] = json!(if status.is_verdict() && by.is_none() {
                    "recorded without an evidence node — write what convinced you \
                     (kaeru_evidence, or an episode) and re-run the verdict with `by`"
                        .to_string()
                } else if status.is_verdict() {
                    "settled in one call — the evidence is linked to the claim".to_string()
                } else {
                    format!(
                        "when the verdict lands: kaeru_confirm / kaeru_refute / \
                         kaeru_inconclusive \"{}\"; still-open claims surface in kaeru_awake.",
                        args.name
                    )
                });
                out
            }
            Err(e) => json!({ "created": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct EvidenceArgs {
    pub hypothesis: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub node: Option<String>,
}

mem_tool!(
    /// `kaeru_evidence` — record what was actually checked, against a hypothesis.
    Evidence,
    "kaeru_evidence",
    "Record what you actually checked and attach it to a hypothesis. Past tense — this documents \
     a check that already ran, it does not schedule one (and it is not `cargo test`). Pass \
     `method` to write the result up as a new experiment node, or `node` to point at something \
     you already captured.",
    EvidenceArgs,
    { "type": "object", "properties": {
        "hypothesis": { "type": "string", "description": "hypothesis name or id" },
        "name": { "type": "string", "description": "short name for the experiment node (with `method`)" },
        "method": { "type": "string", "description": "what you did and what came out of it" },
        "node": { "type": "string", "description": "an existing node to register as the evidence instead" }
    }, "required": ["hypothesis"] },
    |store, args| {
        let hyp = resolve(store, &args.hypothesis);
        match (args.node.as_deref(), args.method.as_deref()) {
            (Some(existing), _) => {
                let node = resolve(store, existing);
                match link(store, &node, &hyp, EdgeType::Targets) {
                    Ok(_) => json!({ "linked": true, "id": node }),
                    Err(e) => json!({ "linked": false, "error": e.to_string() }),
                }
            }
            (None, Some(method)) => {
                let name = args.name.as_deref().unwrap_or("evidence");
                match run_experiment(store, &hyp, name, method) {
                    Ok(id) => json!({ "created": true, "id": id }),
                    Err(e) => json!({ "created": false, "error": e.to_string() }),
                }
            }
            (None, None) => json!({
                "created": false,
                "error": "pass `method` to record what you did, or `node` to point at an existing result"
            }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct VerdictArgs {
    pub hypothesis: String,
    /// Optional: record the verdict even with nothing to cite yet, rather
    /// than leaving the claim open with the answer buried in its prose.
    #[serde(default)]
    pub by: Option<String>,
}

mem_tool!(
    /// `kaeru_confirm` — mark a hypothesis supported by evidence.
    Confirm,
    "kaeru_confirm",
    "Mark a hypothesis Supported. `by` (the verifying evidence node) is optional — record the \
     verdict even with nothing to cite yet rather than leaving the claim open with the answer \
     buried in its text. Adds a `verifies` edge when given.",
    VerdictArgs,
    { "type": "object", "properties": {
        "hypothesis": { "type": "string", "description": "hypothesis name or id" },
        "by": { "type": "string", "description": "optional evidence node name or id" }
    }, "required": ["hypothesis"] },
    |store, args| {
        let hyp = resolve(store, &args.hypothesis);
        let by = args.by.as_deref().map(|b| resolve(store, b));
        match update_hypothesis_status(store, &hyp, HypothesisStatus::Supported, by.as_ref()) {
            Ok(()) => json!({ "updated": true, "status": "supported" }),
            Err(e) => json!({ "updated": false, "error": e.to_string() }),
        }
    }
);

mem_tool!(
    /// `kaeru_refute` — mark a hypothesis refuted by a counterexample.
    Refute,
    "kaeru_refute",
    "Mark a hypothesis Refuted. `by` (the falsifying counterexample) is optional, same as for \
     `kaeru_confirm`. Adds a `falsifies` edge when given.",
    VerdictArgs,
    { "type": "object", "properties": {
        "hypothesis": { "type": "string", "description": "hypothesis name or id" },
        "by": { "type": "string", "description": "optional counterexample node name or id" }
    }, "required": ["hypothesis"] },
    |store, args| {
        let hyp = resolve(store, &args.hypothesis);
        let by = args.by.as_deref().map(|b| resolve(store, b));
        match update_hypothesis_status(store, &hyp, HypothesisStatus::Refuted, by.as_ref()) {
            Ok(()) => json!({ "updated": true, "status": "refuted" }),
            Err(e) => json!({ "updated": false, "error": e.to_string() }),
        }
    }
);

mem_tool!(
    /// `kaeru_inconclusive` — the check ran and did not decide.
    Inconclusive,
    "kaeru_inconclusive",
    "Mark a hypothesis Inconclusive — the check ran and did not decide. A real third verdict, \
     not a failure to answer: it closes the claim out of the open queue while recording that the \
     question stayed open on the merits. Writes no verdict edge, so `by` is not needed.",
    VerdictArgs,
    { "type": "object", "properties": {
        "hypothesis": { "type": "string", "description": "hypothesis name or id" },
        "by": { "type": "string", "description": "unused — inconclusive writes no verdict edge" }
    }, "required": ["hypothesis"] },
    |store, args| {
        let hyp = resolve(store, &args.hypothesis);
        match update_hypothesis_status(store, &hyp, HypothesisStatus::Inconclusive, None) {
            Ok(()) => json!({ "updated": true, "status": "inconclusive" }),
            Err(e) => json!({ "updated": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct FlagArgs {
    pub target: String,
    pub reason: String,
}

mem_tool!(
    /// `kaeru_flag` — flag a node for review (non-destructive).
    Flag,
    "kaeru_flag",
    "Flag a memory you doubt for review — non-destructive, attaches a `contradicts` edge with \
     your reason. Surfaces in `kaeru_awake`'s under-review list.",
    FlagArgs,
    { "type": "object", "properties": {
        "target": { "type": "string", "description": "node name or id to flag" },
        "reason": { "type": "string", "description": "why it needs a second look" }
    }, "required": ["target", "reason"] },
    |store, args| {
        let target = resolve(store, &args.target);
        match mark_under_review(store, &target, &args.reason) {
            Ok(id) => json!({ "flagged": true, "review_id": id }),
            Err(e) => json!({ "flagged": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct ResolveArgs {
    pub question: String,
    pub by: String,
}

mem_tool!(
    /// `kaeru_resolve` — close an open question with the answer.
    Resolve,
    "kaeru_resolve",
    "Close an open / under-review question by recording the answer node (`by`).",
    ResolveArgs,
    { "type": "object", "properties": {
        "question": { "type": "string", "description": "question node name or id" },
        "by": { "type": "string", "description": "answer node name or id" }
    }, "required": ["question", "by"] },
    |store, args| {
        let q = resolve(store, &args.question);
        let by = resolve(store, &args.by);
        match mark_resolved(store, &q, &by) {
            Ok(()) => json!({ "resolved": true }),
            Err(e) => json!({ "resolved": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct CloseReviewArgs {
    pub target: String,
    #[serde(default)]
    pub resolution: Option<String>,
}

mem_tool!(
    /// `kaeru_close_review` — close an open review non-destructively.
    CloseReview,
    "kaeru_close_review",
    "Close an open review on a node — the counterpart to `kaeru_flag`. Retracts its `contradicts` \
     edge(s) so it leaves `kaeru_awake`'s under-review list, while the doubt stays in history. \
     Pass an optional `resolution` note to record how it was settled as provenance.",
    CloseReviewArgs,
    { "type": "object", "properties": {
        "target": { "type": "string", "description": "node name or id whose review to close" },
        "resolution": { "type": "string", "description": "optional note on how it was settled" }
    }, "required": ["target"] },
    |store, args| {
        let target = resolve(store, &args.target);
        match resolve_review(store, &target, args.resolution.as_deref()) {
            Ok(closed) => json!({ "closed": closed.len(), "reviews": closed }),
            Err(e) => json!({ "closed": 0, "error": e.to_string() }),
        }
    }
);
