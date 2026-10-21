//! Portable schema and semantic closure checks, performed before store writes.

use std::collections::{BTreeMap, BTreeSet};

use chrono::DateTime;
use sha2::{Digest, Sha256};

use super::codec::json_digest;
use crate::config::amg::{AmgLimits, MAX_ORIGINS, PROFILE, SCHEMA_VERSION};
use crate::models::amg::{AmgDocument, AmgMetadata, AmgOrigin, AmgScope};
use crate::{EdgeType, Error, HypothesisStatus, Layer, NodeType, Result, Tier};

pub(super) fn invalid(message: impl Into<String>) -> Error {
    Error::Invalid(format!("AMG: {}", message.into()))
}
fn text(value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(invalid("empty identifier or initiative"));
    }
    Ok(())
}
fn timestamp(value: &str) -> Result<()> {
    DateTime::parse_from_rfc3339(value).map_err(|_| invalid("invalid RFC3339 timestamp"))?;
    Ok(())
}
fn unique<'a>(values: impl IntoIterator<Item = &'a str>) -> Result<BTreeSet<&'a str>> {
    let mut out = BTreeSet::new();
    for value in values {
        text(value)?;
        if !out.insert(value) {
            return Err(invalid("duplicate identity or membership"));
        }
    }
    Ok(out)
}

pub(super) fn metadata(meta: &AmgMetadata) -> Result<()> {
    if meta.schema_version != SCHEMA_VERSION {
        return Err(invalid(format!(
            "schema {} is unsupported; supported AMG schema is {SCHEMA_VERSION}",
            meta.schema_version
        )));
    }
    if meta.profile != PROFILE {
        return Err(invalid("unsupported knowledge profile"));
    }
    timestamp(&meta.exported_at)?;
    text(&meta.software.name)?;
    text(&meta.software.version)?;
    unique(meta.initiatives.iter().map(String::as_str))?;
    if meta.scope == AmgScope::Initiatives && meta.initiatives.is_empty() {
        return Err(invalid("selected initiative set cannot be empty"));
    }
    for omission in &meta.omissions {
        text(&omission.reason)?;
    }
    Ok(())
}

fn origins(origins: &[AmgOrigin]) -> Result<()> {
    if origins.len() > MAX_ORIGINS {
        return Err(invalid("provenance chain exceeds limit"));
    }
    for origin in origins {
        if origin.document_sha256.len() != 64
            || !origin
                .document_sha256
                .bytes()
                .all(|b| b.is_ascii_hexdigit())
        {
            return Err(invalid("invalid provenance digest"));
        }
        timestamp(&origin.exported_at)?;
        for value in [&origin.recorded_at, &origin.created_at]
            .into_iter()
            .flatten()
        {
            timestamp(value)?;
        }
        if origin
            .source_visibility
            .as_ref()
            .is_some_and(|v| v != "local" && v != "shared")
        {
            return Err(invalid("invalid source visibility"));
        }
        text(&origin.software.name)?;
        text(&origin.software.version)?;
    }
    Ok(())
}

pub fn validate(document: &AmgDocument, limits: &AmgLimits) -> Result<()> {
    metadata(&document.metadata)?;
    let graph = &document.data;
    if graph.nodes.len() > limits.nodes
        || graph.edges.len() > limits.edges
        || graph
            .trails
            .len()
            .saturating_add(graph.slots.len())
            .saturating_add(graph.decisions.len())
            > limits.auxiliary_records
    {
        return Err(invalid("record count exceeds limit"));
    }
    let scope = document
        .metadata
        .initiatives
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let ids = unique(graph.nodes.iter().map(|node| node.id.as_str()))?;
    let nodes = graph
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect::<BTreeMap<_, _>>();
    let membership = |names: &[String]| -> Result<()> {
        let names = unique(names.iter().map(String::as_str))?;
        if !names.is_subset(&scope) {
            return Err(invalid("membership outside declared scope"));
        }
        Ok(())
    };
    let mut boards = BTreeSet::new();
    for node in &graph.nodes {
        if !NodeType::VALID.contains(&node.node_type.as_str())
            || node.node_type == "audit_event"
            || !Tier::VALID.contains(&node.tier.as_str())
            || !Layer::VALID.contains(&node.layer.as_str())
            || !["local", "shared"].contains(&node.visibility.as_str())
        {
            return Err(invalid("unknown node type, tier, layer or visibility"));
        }
        json_structure(&node.properties, limits)?;
        membership(&node.initiatives)?;
        if document.metadata.scope == AmgScope::Initiatives && node.initiatives.is_empty() {
            return Err(invalid("unscoped node in selected export"));
        }
        if node.layer == "core" && node.initiatives.is_empty() {
            return Err(invalid("core node has no initiative"));
        }
        unique(node.tags.iter().map(String::as_str))?;
        if node.tags.iter().any(|tag| tag.starts_with("seen:")) {
            return Err(invalid("delivery bookkeeping is not portable knowledge"));
        }
        origins(&node.origins)?;
        for value in [&node.recorded_at, &node.created_at].into_iter().flatten() {
            timestamp(value)?;
        }
        if let Some(task) = node.properties.pointer("/applicability/task_id") {
            let task = task
                .as_str()
                .ok_or_else(|| invalid("invalid task applicability reference"))?;
            if nodes.get(task).is_none_or(|node| node.node_type != "task") {
                return Err(invalid(format!(
                    "task applicability is not closed inside document for {}",
                    node.id
                )));
            }
        }
        if let Some(source) = node
            .properties
            .pointer("/source/original")
            .filter(|_| node.node_type == "reference")
        {
            let original = source
                .as_str()
                .ok_or_else(|| invalid("source original must be text"))?;
            let hash = node
                .properties
                .pointer("/source/sha256")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| invalid("source digest missing"))?;
            let expected = format!("{:x}", Sha256::digest(original.as_bytes()));
            if node.body.as_deref() != Some(original) || !hash.eq_ignore_ascii_case(&expected) {
                return Err(invalid("immutable source body/digest mismatch"));
            }
        }
        if node.node_type == "board" {
            for name in &node.initiatives {
                if !boards.insert(name) {
                    return Err(invalid("multiple board registries in one initiative"));
                }
            }
        }
    }
    let mut edges = BTreeSet::new();
    let mut legacy_keys = BTreeSet::new();
    for edge in &graph.edges {
        if !ids.contains(edge.src.as_str()) || !ids.contains(edge.dst.as_str()) {
            return Err(invalid("edge endpoint outside document"));
        }
        if !EdgeType::VALID.contains(&edge.edge_type.as_str())
            || !edge.weight.is_finite()
            || !(0.0..=1.0).contains(&edge.weight)
        {
            return Err(invalid("unknown edge type or invalid weight"));
        }
        if !edges.insert((&edge.src, &edge.dst, &edge.edge_type)) {
            return Err(invalid("duplicate edge"));
        }
        if !legacy_keys.insert(format!("{}|{}|{}", edge.src, edge.dst, edge.edge_type)) {
            return Err(invalid("ambiguous legacy edge membership key"));
        }
        json_structure(&edge.properties, limits)?;
        membership(&edge.initiatives)?;
        origins(&edge.origins)?;
        for value in [&edge.recorded_at, &edge.created_at].into_iter().flatten() {
            timestamp(value)?;
        }
    }
    let mut trails = BTreeSet::new();
    for trail in &graph.trails {
        if !trails.insert(&trail.id)
            || nodes
                .get(trail.id.as_str())
                .is_none_or(|n| n.node_type != "chain")
        {
            return Err(invalid("invalid or duplicate trail identity"));
        }
        unique(trail.members.iter().map(String::as_str))?;
        if trail.members.iter().any(|id| !ids.contains(id.as_str())) {
            return Err(invalid("trail is not closed"));
        }
        if trail.stale.is_some() != trail.fingerprints.is_some() {
            return Err(invalid("incomplete trail summary metadata"));
        }
        if let Some(fingerprints) = &trail.fingerprints {
            if fingerprints.keys().any(|key| !ids.contains(key.as_str())) {
                return Err(invalid("trail fingerprint outside document"));
            }
            for hash in fingerprints.values() {
                if hash.len() != 64
                    || !hash
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
                {
                    return Err(invalid("invalid trail fingerprint"));
                }
            }
            if trail.stale == Some(false)
                && (fingerprints.len() != trail.members.len()
                    || trail.members.iter().any(|id| {
                        let node = nodes[id.as_str()];
                        fingerprints.get(id)
                            != Some(&json_digest(&serde_json::json!([node.name, node.body])))
                    }))
            {
                return Err(invalid(
                    "fresh trail summary does not match its current members",
                ));
            }
        }
    }
    if graph
        .nodes
        .iter()
        .any(|node| node.node_type == "chain" && !trails.contains(&node.id))
    {
        return Err(invalid("chain node has no ordered trail"));
    }
    let mut slots = BTreeSet::new();
    for slot in &graph.slots {
        text(&slot.slot)?;
        if !scope.contains(slot.initiative.as_str())
            || !slots.insert((&slot.initiative, &slot.slot))
            || nodes
                .get(slot.node.as_str())
                .is_none_or(|node| !node.initiatives.contains(&slot.initiative))
        {
            return Err(invalid(
                "slot occupant outside initiative or duplicate slot",
            ));
        }
    }
    let mut decisions = BTreeSet::new();
    for decision in &graph.decisions {
        if !decisions.insert(&decision.node)
            || nodes
                .get(decision.node.as_str())
                .is_none_or(|n| n.node_type != "hypothesis")
            || !HypothesisStatus::VALID.contains(&decision.status.as_str())
        {
            return Err(invalid("invalid hypothesis decision"));
        }
        unique(decision.evidence.iter().map(String::as_str))?;
        if decision
            .evidence
            .iter()
            .any(|id| !ids.contains(id.as_str()))
        {
            return Err(invalid("evidence outside document"));
        }
        let node = nodes[decision.node.as_str()];
        if !node.tags.contains(&format!("status:{}", decision.status)) {
            return Err(invalid("decision and hypothesis status disagree"));
        }
    }
    Ok(())
}

fn json_structure(value: &serde_json::Value, limits: &AmgLimits) -> Result<()> {
    let mut stack = vec![(value, 4usize)];
    let mut items = 0usize;
    while let Some((value, depth)) = stack.pop() {
        items += 1;
        if depth > limits.nesting || items > limits.cbor_items {
            return Err(invalid("property structure exceeds bounds"));
        }
        match value {
            serde_json::Value::Array(values) => stack.extend(values.iter().map(|v| (v, depth + 1))),
            serde_json::Value::Object(values) => {
                stack.extend(values.values().map(|v| (v, depth + 1)))
            }
            _ => {}
        }
    }
    Ok(())
}
