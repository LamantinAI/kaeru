//! The hypothesis cycle and the review flow.

use kaeru_core::{
    HypothesisStatus, formulate_hypothesis, mark_resolved, mark_under_review, resolve_review,
    run_experiment, update_hypothesis_status,
};
use serde::Deserialize;
use serde_json::json;

use crate::{mem_tool, resolve};

#[derive(Debug, Deserialize)]
pub struct ClaimArgs {
    pub name: String,
    pub claim: String,
}

mem_tool!(
    /// `kaeru_claim` — formulate a falsifiable hypothesis.
    Claim,
    "kaeru_claim",
    "Record a falsifiable hypothesis you want to test. Later `kaeru_test` it, then `kaeru_confirm` \
     or `kaeru_refute` with evidence.",
    ClaimArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "short name for the hypothesis" },
        "claim": { "type": "string", "description": "the claim, stated so it can be falsified" }
    }, "required": ["name", "claim"] },
    |store, args| match formulate_hypothesis(store, &args.name, &args.claim) {
        Ok(id) => json!({ "created": true, "id": id }),
        Err(e) => json!({ "created": false, "error": e.to_string() }),
    }
);

#[derive(Debug, Deserialize)]
pub struct TestArgs {
    pub hypothesis: String,
    pub name: String,
    pub method: String,
}

mem_tool!(
    /// `kaeru_test` — record an experiment targeting a hypothesis.
    Test,
    "kaeru_test",
    "Record an experiment that tests a hypothesis (creates an experiment node with a `targets` \
     edge). Describe how you'll test it in `method`.",
    TestArgs,
    { "type": "object", "properties": {
        "hypothesis": { "type": "string", "description": "hypothesis name or id" },
        "name": { "type": "string", "description": "short name for the experiment" },
        "method": { "type": "string", "description": "how the hypothesis is tested" }
    }, "required": ["hypothesis", "name", "method"] },
    |store, args| {
        let hyp = resolve(store, &args.hypothesis);
        match run_experiment(store, &hyp, &args.name, &args.method) {
            Ok(id) => json!({ "created": true, "id": id }),
            Err(e) => json!({ "created": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct VerdictArgs {
    pub hypothesis: String,
    pub by: String,
}

mem_tool!(
    /// `kaeru_confirm` — mark a hypothesis supported by evidence.
    Confirm,
    "kaeru_confirm",
    "Mark a hypothesis Supported, citing the evidence node (`by`). Adds a `verifies` edge.",
    VerdictArgs,
    { "type": "object", "properties": {
        "hypothesis": { "type": "string", "description": "hypothesis name or id" },
        "by": { "type": "string", "description": "evidence node name or id" }
    }, "required": ["hypothesis", "by"] },
    |store, args| {
        let hyp = resolve(store, &args.hypothesis);
        let by = resolve(store, &args.by);
        match update_hypothesis_status(store, &hyp, HypothesisStatus::Supported, &by) {
            Ok(()) => json!({ "updated": true, "status": "supported" }),
            Err(e) => json!({ "updated": false, "error": e.to_string() }),
        }
    }
);

mem_tool!(
    /// `kaeru_refute` — mark a hypothesis refuted by a counterexample.
    Refute,
    "kaeru_refute",
    "Mark a hypothesis Refuted, citing the counterexample node (`by`). Adds a `falsifies` edge.",
    VerdictArgs,
    { "type": "object", "properties": {
        "hypothesis": { "type": "string", "description": "hypothesis name or id" },
        "by": { "type": "string", "description": "counterexample node name or id" }
    }, "required": ["hypothesis", "by"] },
    |store, args| {
        let hyp = resolve(store, &args.hypothesis);
        let by = resolve(store, &args.by);
        match update_hypothesis_status(store, &hyp, HypothesisStatus::Refuted, &by) {
            Ok(()) => json!({ "updated": true, "status": "refuted" }),
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
