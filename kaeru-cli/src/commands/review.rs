//! Review-flow commands: `flag` (mark a node for review) and
//! `resolve` (close an open question).

use kaeru_core::Result;
use kaeru_core::Store;
use kaeru_core::mark_resolved;
use kaeru_core::mark_under_review;

use crate::parse::resolve_name;

pub fn flag(store: &Store, target: &str, reason: &str) -> Result<()> {
    let target_id = resolve_name(store, target)?;
    let review_id = mark_under_review(store, &target_id, reason)?;
    println!("flagged: {target} (review id: {review_id})");
    Ok(())
}

pub fn resolve(store: &Store, question: &str, by: &str) -> Result<()> {
    let question_id = resolve_name(store, question)?;
    let by_id = resolve_name(store, by)?;
    mark_resolved(store, &question_id, &by_id)?;
    println!("resolved: {question} ← {by}");
    Ok(())
}
