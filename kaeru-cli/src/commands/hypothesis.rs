//! Hypothesis-experiment cycle: `claim`, `test`, `confirm`, `refute`.

use kaeru_core::EdgeType;
use kaeru_core::HypothesisStatus;
use kaeru_core::Result;
use kaeru_core::Store;
use kaeru_core::formulate_hypothesis;
use kaeru_core::link;
use kaeru_core::run_experiment;
use kaeru_core::update_hypothesis_status;

use crate::parse::derive_auto_name;
use crate::parse::resolve_name;

pub fn claim(store: &Store, text: &str, about: Option<&str>) -> Result<()> {
    let auto_name = derive_auto_name(text, "claim");
    let id = formulate_hypothesis(store, &auto_name, text)?;
    if let Some(about_name) = about {
        let target = resolve_name(store, about_name)?;
        link(store, &id, &target, EdgeType::RefersTo)?;
    }
    println!("claimed: {auto_name} — {id}");
    Ok(())
}

pub fn test(store: &Store, hypothesis: &str, method: &str) -> Result<()> {
    let hyp_id = resolve_name(store, hypothesis)?;
    let auto_name = derive_auto_name(method, "experiment");
    let exp_id = run_experiment(store, &hyp_id, &auto_name, method)?;
    println!("experiment: {auto_name} — {exp_id}");
    Ok(())
}

pub fn confirm(store: &Store, hypothesis: &str, by: &str) -> Result<()> {
    let hyp_id = resolve_name(store, hypothesis)?;
    let by_id = resolve_name(store, by)?;
    update_hypothesis_status(store, &hyp_id, HypothesisStatus::Supported, &by_id)?;
    println!("confirmed: {hypothesis}");
    Ok(())
}

pub fn refute(store: &Store, hypothesis: &str, by: &str) -> Result<()> {
    let hyp_id = resolve_name(store, hypothesis)?;
    let by_id = resolve_name(store, by)?;
    update_hypothesis_status(store, &hyp_id, HypothesisStatus::Refuted, &by_id)?;
    println!("refuted: {hypothesis}");
    Ok(())
}
