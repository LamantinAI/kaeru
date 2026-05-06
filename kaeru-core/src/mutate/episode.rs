//! `write_episode` — the canonical operational-tier write.

use cozo::DataValue;
use cozo::ScriptMutability;
use std::collections::BTreeMap;

use crate::errors::Result;
use crate::graph::EpisodeKind;
use crate::graph::NodeId;
use crate::graph::Significance;
use crate::graph::audit::write_audit;
use crate::graph::new_node_id;
use crate::store::Store;

use super::attach_node_to_initiative;
use super::now_validity_seconds;

/// Writes an episode node and an audit_event for the operation.
/// Returns the new episode node id.
pub fn write_episode(
    store: &Store,
    kind: EpisodeKind,
    significance: Significance,
    name: &str,
    body: &str,
) -> Result<NodeId> {
    let id = new_node_id();

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));
    params.insert("name".to_string(), DataValue::Str(name.into()));
    params.insert("body".to_string(), DataValue::Str(body.into()));

    // Encode kind + significance into `tags` — keeps the schema uniform.
    // Richer per-episode typing (separate columns / properties JSON
    // schema) is a follow-up.
    // Tags and validity are inlined into the script: cozo's `<-` literal
    // rule needs concrete values for List and Validity columns; passing
    // them as `DataValue` parameters trips `eval::not_constant`.
    let kind_tag = kind.as_str();
    let sig_tag = significance.as_str();
    let now_secs = now_validity_seconds();
    let script = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{now_secs}.0, true], 'episode', 'operational', $name, $body, ['kind:{kind_tag}', 'sig:{sig_tag}'], null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Mutable)?;

    attach_node_to_initiative(store, &id)?;
    write_audit(store.db_ref(), "write_episode", "system", &[id.clone()])?;
    Ok(id)
}

/// Low-friction episode write — derives a name from the body's first
/// words plus a short id suffix and defaults `kind = Observation`,
/// `significance = Low`. Returns the new episode id.
///
/// Use this for fleeting thoughts you don't want to slow down to name.
/// For load-bearing episodes pick a deliberate name and call
/// [`write_episode`] instead.
pub fn jot(store: &Store, body: &str) -> Result<NodeId> {
    let id = new_node_id();
    let name = derive_jot_name(body, &id);

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("id".to_string(), DataValue::Str(id.clone().into()));
    params.insert("name".to_string(), DataValue::Str(name.into()));
    params.insert("body".to_string(), DataValue::Str(body.into()));

    let now_secs = now_validity_seconds();
    let script = format!(
        r#"
        ?[id, validity, type, tier, name, body, tags, initiatives, properties] <-
            [[$id, [{now_secs}.0, true], 'episode', 'operational', $name, $body, ['kind:observation', 'sig:low', 'role:jot'], null, null]]
        :put node {{id, validity => type, tier, name, body, tags, initiatives, properties}}
        "#
    );
    store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Mutable)?;

    attach_node_to_initiative(store, &id)?;
    write_audit(store.db_ref(), "jot", "system", &[id.clone()])?;
    Ok(id)
}

/// Builds an auto-name from `body`'s first ~5 alphanumeric words plus
/// a 6-character suffix from the node id, so two jots with the same
/// preface still get distinct names. Falls back to `jot-<suffix>` when
/// the body has no usable tokens.
fn derive_jot_name(body: &str, id: &NodeId) -> String {
    const MAX_WORDS: usize = 5;

    let mut words: Vec<String> = Vec::new();
    for raw in body.split_whitespace() {
        let cleaned: String = raw
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect::<String>()
            .to_lowercase();
        if cleaned.is_empty() {
            continue;
        }
        words.push(cleaned);
        if words.len() >= MAX_WORDS {
            break;
        }
    }

    let id_suffix: String = id.chars().rev().take(6).collect::<String>().chars().rev().collect();
    if words.is_empty() {
        format!("jot-{id_suffix}")
    } else {
        format!("{}-{id_suffix}", words.join("-"))
    }
}
