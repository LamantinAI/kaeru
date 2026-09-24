//! Capture & connect: write thoughts, references, tasks, and typed links.
//!
//! Verb for verb with the daemon (#98), which means two things this file did
//! not use to do. First, `jot` and `episode` are separate verbs — one auto-names
//! a fleeting note, the other takes a deliberate name — instead of one
//! `remember` that guessed from whether a name was passed. Second, every
//! capture can stamp its layer, set a reminder, and go to the cloud in the
//! same call; and `link` / `unlink` / `reweight` mirror the edit to a cloud
//! that already holds both endpoints.

use kaeru_core::{
    EdgeType, EpisodeKind, Layer, Significance, cite_with_layer, complete_task, jot_with_layer,
    link_with_weight, node_brief_by_id, parse_due_to_iso, set_edge_weight, set_reminder, unlink,
    write_episode_with_layer, write_task_with_layer,
};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::cloud::{EdgeChange, propagate_edge, share_node};
use crate::{KaeruMemory, mem_tool_cloud, mem_tool_in, resolve, target_initiative};

/// Parses the optional `layer` argument, defaulting to `warm` — a node is
/// never born layer-less.
fn parse_layer(raw: Option<&str>) -> Result<Layer, String> {
    match raw.map(str::trim).filter(|s| !s.is_empty()) {
        None => Ok(Layer::default()),
        Some(v) => v.parse::<Layer>().map_err(|e| e.to_string()),
    }
}

/// `visibility` → "does the caller want this in the cloud?".
fn wants_shared(raw: Option<&str>) -> Result<bool, String> {
    match raw.map(|v| v.trim().to_lowercase()) {
        None => Ok(false),
        Some(v) => match v.as_str() {
            "" | "local" => Ok(false),
            "shared" => Ok(true),
            other => Err(format!(
                "visibility must be `local` or `shared`, got {other:?}"
            )),
        },
    }
}

/// Applies an `after:` / `for:` reminder to a freshly-written node.
///
/// `for_days` has no default on purpose: a reminder nobody chose a window for
/// either vanishes unseen or never stops. So each argument requires the other,
/// and the mismatch is reported rather than guessed at.
fn apply_reminder(
    store: &kaeru_core::Store,
    id: &str,
    after: Option<&str>,
    for_days: Option<i64>,
) -> Result<Option<String>, String> {
    match (after, for_days) {
        (None, None) => Ok(None),
        (Some(after), Some(days)) => {
            set_reminder(store, &id.to_string(), after, days).map_err(|e| e.to_string())?;
            Ok(Some(format!(
                "set aside until {after}, then surfacing in `kaeru_awake` for {days} day(s)"
            )))
        }
        (Some(_), None) => Err(
            "`after` needs `for_days`: how many days the reminder keeps \
                                appearing once it surfaces. There is no default."
                .to_string(),
        ),
        (None, Some(_)) => Err("`for_days` needs `after`: the date the reminder becomes \
                                relevant. A window has nothing to count from without one."
            .to_string()),
    }
}

/// Pushes a freshly-captured node to the cloud when the caller asked for
/// `visibility=shared`, and says what happened either way.
async fn maybe_share(
    mem: &KaeruMemory,
    id: &str,
    initiative: Option<&str>,
    cloud: Option<&str>,
    want_share: bool,
) -> Option<Value> {
    if !want_share {
        return None;
    }
    let Some(init) = initiative else {
        return Some(json!({
            "shared": false,
            "reason": "no initiative — saved local; a node must name one to leave",
        }));
    };
    let client = match mem.clouds().resolve(cloud) {
        Ok(c) => c.clone(),
        Err(why) => return Some(json!({ "shared": false, "reason": why })),
    };
    match share_node(mem, &client, id.to_string(), init.to_string(), false).await {
        Ok(message) => Some(json!({ "cloud": client.name(), "outcome": message })),
        Err(e) => Some(json!({ "shared": false, "cloud": client.name(), "error": e })),
    }
}

/// Makes the edge a capture asked for, and returns what to say about it.
///
/// Never fails the capture: the thought is stored either way, and refusing
/// to keep it because a name was mistyped is worse than an island. Every
/// refusal names itself — an edge silently not made is how a vault goes
/// flat (#102).
fn link_at_capture(
    store: &kaeru_core::Store,
    id: &str,
    to: Option<&str>,
    edge_type: Option<&str>,
    weight: Option<f64>,
) -> Option<Value> {
    if to.is_none() && weight.is_none() && edge_type.is_none() {
        return None;
    }
    let Some(target) = to.map(str::trim).filter(|t| !t.is_empty()) else {
        return Some(json!({
            "linked": false,
            "reason": "`edge_type` / `weight` without `link_to` — name the other end",
        }));
    };
    let Some(weight) = weight else {
        return Some(json!({
            "linked": false, "to": target,
            "reason": "`weight` (0..1) is required — it is what knowledge chains route on,                        and there is no default on purpose",
        }));
    };
    let parsed = match edge_type.unwrap_or("refers_to").parse::<EdgeType>() {
        Ok(e) => e,
        Err(e) => return Some(json!({ "linked": false, "to": target, "error": e.to_string() })),
    };
    let target_id = resolve(store, target);
    match link_with_weight(store, &id.to_string(), &target_id, parsed, weight) {
        Ok(()) => Some(json!({
            "linked": true, "to": target, "edge_type": parsed.as_str(),
            "weight": weight.clamp(0.0, 1.0),
        })),
        Err(e) => Some(json!({ "linked": false, "to": target, "error": e.to_string() })),
    }
}

#[derive(Debug, Deserialize)]
pub struct JotArgs {
    pub body: String,
    #[serde(default)]
    pub layer: Option<String>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub cloud: Option<String>,
    #[serde(default)]
    pub after: Option<String>,
    #[serde(default)]
    pub for_days: Option<i64>,
    /// The node this one connects to, by name or id — the edge is made in
    /// THIS call, while both ends are still in mind (#102).
    #[serde(default)]
    pub link_to: Option<String>,
    #[serde(default)]
    pub edge_type: Option<String>,
    /// Required with `link_to`, and no default: the capture lands without
    /// it, the edge does not.
    #[serde(default)]
    pub weight: Option<f64>,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_cloud!(
    /// `kaeru_jot` — a fleeting note, auto-named.
    Jot,
    "kaeru_jot",
    "Save a quick note to long-term memory without stopping to name it — the name is derived \
     from the first words. For a decision or a load-bearing fact use `kaeru_episode`, which takes \
     a deliberate name you can recall it by. Pass `initiative` to file it under a specific \
     project; omit for your default.",
    JotArgs,
    { "type": "object", "properties": {
        "body": { "type": "string", "description": "the thought to save" },
        "layer": { "type": "string", "description": "memory layer at creation: core / hot / warm (default) / cold / frozen" },
        "visibility": { "type": "string", "description": "`shared` pushes it to the cloud in this same call; default `local`" },
        "cloud": { "type": "string", "description": "which cloud to share into, when several are configured" },
        "after": { "type": "string", "description": "not relevant until this date (YYYY-MM-DD); needs `for_days`" },
        "for_days": { "type": "integer", "description": "how many days it keeps surfacing once the date arrives; needs `after`" },
        "link_to": { "type": "string", "description": "node (name or id) to link this capture to, in the same call" },
        "edge_type": { "type": "string", "description": "edge type for `link_to` (default refers_to)" },
        "weight": { "type": "number", "description": "how load-bearing the `link_to` edge is, 0..1 — required with `link_to`" },
        "initiative": { "type": "string", "description": "optional initiative (project) to file this under; omit for your default" }
    }, "required": ["body"] },
    |mem, a| {
        let init = target_initiative(mem, &a.initiative);
        let (body, layer_raw) = (a.body.clone(), a.layer.clone());
        let (after, for_days) = (a.after.clone(), a.for_days);
        let (link_to, edge_type, weight) = (a.link_to.clone(), a.edge_type.clone(), a.weight);
        let written = mem
            .run_in(init.clone(), move |store| {
                let layer = match parse_layer(layer_raw.as_deref()) {
                    Ok(l) => l,
                    Err(e) => return json!({ "saved": false, "error": e }),
                };
                match jot_with_layer(store, &body, layer) {
                    Ok(id) => {
                        let name = node_brief_by_id(store, &id)
                            .ok()
                            .flatten()
                            .map(|b| b.name)
                            .unwrap_or_default();
                        let edge = link_at_capture(
                            store, &id, link_to.as_deref(), edge_type.as_deref(), weight,
                        );
                        match apply_reminder(store, &id, after.as_deref(), for_days) {
                            Ok(note) => json!({
                                "saved": true, "id": id, "name": name,
                                "reminder": note, "link": edge
                            }),
                            Err(e) => json!({
                                "saved": true, "id": id, "name": name,
                                "reminder_error": e, "link": edge
                            }),
                        }
                    }
                    Err(e) => json!({ "saved": false, "error": e.to_string() }),
                }
            })
            .await;
        finish_capture(mem, written, &a.visibility, init.as_deref(), a.cloud.as_deref()).await
    }
);

#[derive(Debug, Deserialize)]
pub struct EpisodeArgs {
    pub name: String,
    pub body: String,
    #[serde(default)]
    pub layer: Option<String>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub cloud: Option<String>,
    #[serde(default)]
    pub after: Option<String>,
    #[serde(default)]
    pub for_days: Option<i64>,
    /// The node this one connects to, by name or id — the edge is made in
    /// THIS call, while both ends are still in mind (#102).
    #[serde(default)]
    pub link_to: Option<String>,
    #[serde(default)]
    pub edge_type: Option<String>,
    /// Required with `link_to`, and no default: the capture lands without
    /// it, the edge does not.
    #[serde(default)]
    pub weight: Option<f64>,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_cloud!(
    /// `kaeru_episode` — an observation tied to the work, under a chosen name.
    Episode,
    "kaeru_episode",
    "Record an observation tied to what you are doing now, under a deliberate `name` you can \
     recall it by later — a decision, a load-bearing fact, something that happened. For a \
     fleeting note use `kaeru_jot`; for a settled document kept verbatim use `kaeru_cite`. Pass \
     `initiative` to file it under a specific project; omit for your default.",
    EpisodeArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "deliberate name to recall it by" },
        "body": { "type": "string", "description": "what happened / what was decided" },
        "layer": { "type": "string", "description": "memory layer at creation: core / hot / warm (default) / cold / frozen" },
        "visibility": { "type": "string", "description": "`shared` pushes it to the cloud in this same call; default `local`" },
        "cloud": { "type": "string", "description": "which cloud to share into, when several are configured" },
        "after": { "type": "string", "description": "not relevant until this date (YYYY-MM-DD); needs `for_days`" },
        "for_days": { "type": "integer", "description": "how many days it keeps surfacing once the date arrives; needs `after`" },
        "link_to": { "type": "string", "description": "node (name or id) to link this capture to, in the same call" },
        "edge_type": { "type": "string", "description": "edge type for `link_to` (default refers_to)" },
        "weight": { "type": "number", "description": "how load-bearing the `link_to` edge is, 0..1 — required with `link_to`" },
        "initiative": { "type": "string", "description": "optional initiative (project) to file this under; omit for your default" }
    }, "required": ["name", "body"] },
    |mem, a| {
        let init = target_initiative(mem, &a.initiative);
        let (name, body, layer_raw) = (a.name.clone(), a.body.clone(), a.layer.clone());
        let (after, for_days) = (a.after.clone(), a.for_days);
        let (link_to, edge_type, weight) = (a.link_to.clone(), a.edge_type.clone(), a.weight);
        let written = mem
            .run_in(init.clone(), move |store| {
                let layer = match parse_layer(layer_raw.as_deref()) {
                    Ok(l) => l,
                    Err(e) => return json!({ "saved": false, "error": e }),
                };
                match write_episode_with_layer(
                    store,
                    EpisodeKind::Observation,
                    Significance::Medium,
                    &name,
                    &body,
                    layer,
                ) {
                    Ok(id) => {
                        let edge = link_at_capture(
                            store, &id, link_to.as_deref(), edge_type.as_deref(), weight,
                        );
                        match apply_reminder(store, &id, after.as_deref(), for_days) {
                            Ok(note) => json!({
                                "saved": true, "id": id, "name": name,
                                "reminder": note, "link": edge
                            }),
                            Err(e) => json!({
                                "saved": true, "id": id, "name": name,
                                "reminder_error": e, "link": edge
                            }),
                        }
                    }
                    Err(e) => json!({ "saved": false, "error": e.to_string() }),
                }
            })
            .await;
        finish_capture(mem, written, &a.visibility, init.as_deref(), a.cloud.as_deref()).await
    }
);

#[derive(Debug, Deserialize)]
pub struct CiteArgs {
    pub name: String,
    #[serde(default)]
    pub url: Option<String>,
    pub body: String,
    #[serde(default)]
    pub layer: Option<String>,
    #[serde(default)]
    pub visibility: Option<String>,
    #[serde(default)]
    pub cloud: Option<String>,
    #[serde(default)]
    pub after: Option<String>,
    #[serde(default)]
    pub for_days: Option<i64>,
    /// The node this one connects to, by name or id — the edge is made in
    /// THIS call, while both ends are still in mind (#102).
    #[serde(default)]
    pub link_to: Option<String>,
    #[serde(default)]
    pub edge_type: Option<String>,
    /// Required with `link_to`, and no default: the capture lands without
    /// it, the edge does not.
    #[serde(default)]
    pub weight: Option<f64>,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_cloud!(
    /// `kaeru_cite` — record an external source or a persona/entity.
    Cite,
    "kaeru_cite",
    "Record a long-term reference kept verbatim: an external source (pass `url` for a paper / \
     gist / dashboard), a settled document of your own (an ADR, a spec, a glossary), or a \
     persona / entity (skip `url` for a person, place, or book). Lands in the archival tier. \
     Pass `initiative` to file it under a specific project; omit for your default.",
    CiteArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "name of the source / document / entity" },
        "url": { "type": "string", "description": "canonical URL (omit for a persona/entity or your own document)" },
        "body": { "type": "string", "description": "the content, or what it is and why it matters" },
        "layer": { "type": "string", "description": "memory layer at creation: core / hot / warm (default) / cold / frozen" },
        "visibility": { "type": "string", "description": "`shared` pushes it to the cloud in this same call; default `local`" },
        "cloud": { "type": "string", "description": "which cloud to share into, when several are configured" },
        "after": { "type": "string", "description": "not relevant until this date (YYYY-MM-DD); needs `for_days`" },
        "for_days": { "type": "integer", "description": "how many days it keeps surfacing once the date arrives; needs `after`" },
        "link_to": { "type": "string", "description": "node (name or id) to link this capture to, in the same call" },
        "edge_type": { "type": "string", "description": "edge type for `link_to` (default refers_to)" },
        "weight": { "type": "number", "description": "how load-bearing the `link_to` edge is, 0..1 — required with `link_to`" },
        "initiative": { "type": "string", "description": "optional initiative (project) to file this under; omit for your default" }
    }, "required": ["name", "body"] },
    |mem, a| {
        let init = target_initiative(mem, &a.initiative);
        let (name, url, body, layer_raw) =
            (a.name.clone(), a.url.clone(), a.body.clone(), a.layer.clone());
        let (after, for_days) = (a.after.clone(), a.for_days);
        let (link_to, edge_type, weight) = (a.link_to.clone(), a.edge_type.clone(), a.weight);
        let written = mem
            .run_in(init.clone(), move |store| {
                let layer = match parse_layer(layer_raw.as_deref()) {
                    Ok(l) => l,
                    Err(e) => return json!({ "saved": false, "error": e }),
                };
                match cite_with_layer(store, &name, url.as_deref(), &body, layer) {
                    Ok(id) => {
                        let edge = link_at_capture(
                            store, &id, link_to.as_deref(), edge_type.as_deref(), weight,
                        );
                        match apply_reminder(store, &id, after.as_deref(), for_days) {
                            Ok(note) => json!({
                                "saved": true, "id": id, "name": name,
                                "reminder": note, "link": edge
                            }),
                            Err(e) => json!({
                                "saved": true, "id": id, "name": name,
                                "reminder_error": e, "link": edge
                            }),
                        }
                    }
                    Err(e) => json!({ "saved": false, "error": e.to_string() }),
                }
            })
            .await;
        finish_capture(mem, written, &a.visibility, init.as_deref(), a.cloud.as_deref()).await
    }
);

/// The shared tail of every capture: if the caller asked for `shared`, push
/// it and fold the outcome into the result.
async fn finish_capture(
    mem: &KaeruMemory,
    mut written: Value,
    visibility: &Option<String>,
    initiative: Option<&str>,
    cloud: Option<&str>,
) -> Value {
    let want_share = match wants_shared(visibility.as_deref()) {
        Ok(v) => v,
        Err(e) => {
            written["visibility_error"] = json!(e);
            return written;
        }
    };
    if written.get("saved").and_then(Value::as_bool) != Some(true) {
        return written;
    }
    let id = written
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    if let Some(outcome) = maybe_share(mem, &id, initiative, cloud, want_share).await {
        written["share"] = outcome;
    }
    written
}

#[derive(Debug, Deserialize)]
pub struct LinkArgs {
    pub from: String,
    pub to: String,
    #[serde(default = "default_edge_type")]
    pub edge_type: String,
    /// Required, and deliberately without a default: it is the signal chains
    /// route on, and in a 6,003-call audit every single `link` omitted it, so
    /// every edge sat at one value and every weighted path ranked on nothing.
    pub weight: f64,
    #[serde(default)]
    pub cloud: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

fn default_edge_type() -> String {
    "refers_to".to_string()
}

mem_tool_cloud!(
    /// `kaeru_link` — connect two memories with a typed, weighted edge.
    Link,
    "kaeru_link",
    "Connect two memories with a typed link so later recall can follow the reasoning trail \
     between them. `weight` (0..1) is REQUIRED and says how load-bearing the connection is — it \
     is the only signal knowledge chains route on, and there is no default because an unweighted \
     graph makes every chain rank on noise. State it by the scale: 0.9–1.0 load-bearing (a cause, \
     a source a conclusion rests on, a supersession); 0.6–0.8 supporting; 0.3–0.5 associative. \
     Types: refers_to (default), causal, derived_from, contradicts, part_of, blocks, targets, \
     supersedes, verifies, falsifies, temporal. `supersedes` runs new → old: link the replacement \
     TO what it replaces.",
    LinkArgs,
    { "type": "object", "properties": {
        "from": { "type": "string", "description": "source node name or id" },
        "to": { "type": "string", "description": "destination node name or id" },
        "edge_type": { "type": "string", "description": "link type (default refers_to)" },
        "weight": { "type": "number", "description": "how load-bearing the link is, 0..1 — required" },
        "cloud": { "type": "string", "description": "which cloud to mirror the edge to, when several are configured" },
        "initiative": { "type": "string", "description": "optional initiative (project) to resolve within; omit for your default" }
    }, "required": ["from", "to", "weight"] },
    |mem, a| {
        let init = target_initiative(mem, &a.initiative);
        let (from, to, type_raw, weight) =
            (a.from.clone(), a.to.clone(), a.edge_type.clone(), a.weight);
        let local = mem
            .blocking_in(init.clone(), move |store| {
                let edge: EdgeType = match type_raw.parse() {
                    Ok(e) => e,
                    Err(e) => return Err(e.to_string()),
                };
                let from_id = resolve(store, &from);
                let to_id = resolve(store, &to);
                link_with_weight(store, &from_id, &to_id, edge, weight)
                    .map(|()| (edge, from_id, to_id))
                    .map_err(|e| e.to_string())
            })
            .await;
        let (edge, from_id, to_id) = match local {
            Ok(v) => v,
            Err(e) => return json!({ "linked": false, "error": e }),
        };
        let mut out = json!({
            "linked": true, "from": from_id, "to": to_id,
            "edge_type": edge.as_str(), "weight": weight.clamp(0.0, 1.0)
        });
        if let Some(note) = propagate_edge(
            mem,
            a.cloud.as_deref(),
            &from_id,
            &to_id,
            edge,
            EdgeChange::Upsert(weight.clamp(0.0, 1.0)),
            init.as_deref(),
        )
        .await
        {
            out["cloud"] = note;
        }
        out
    }
);

#[derive(Debug, Deserialize)]
pub struct ReweightArgs {
    pub from: String,
    pub to: String,
    #[serde(default = "default_edge_type")]
    pub edge_type: String,
    pub weight: f64,
    #[serde(default)]
    pub cloud: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_cloud!(
    /// `kaeru_reweight` — adjust an existing link's connection strength.
    Reweight,
    "kaeru_reweight",
    "Adjust the strength (`weight` 0..1) of an existing link in place — stronger links make \
     shorter knowledge chains. Use to tune which connections matter after the fact. Edge types \
     match `kaeru_link` (default refers_to).",
    ReweightArgs,
    { "type": "object", "properties": {
        "from": { "type": "string", "description": "source node name or id" },
        "to": { "type": "string", "description": "destination node name or id" },
        "edge_type": { "type": "string", "description": "link type (default refers_to)" },
        "weight": { "type": "number", "description": "new strength 0..1" },
        "cloud": { "type": "string", "description": "which cloud to mirror the change to, when several are configured" },
        "initiative": { "type": "string", "description": "optional initiative (project) to resolve within; omit for your default" }
    }, "required": ["from", "to", "weight"] },
    |mem, a| {
        let init = target_initiative(mem, &a.initiative);
        let (from, to, type_raw, weight) =
            (a.from.clone(), a.to.clone(), a.edge_type.clone(), a.weight);
        let local = mem
            .blocking_in(init.clone(), move |store| {
                let edge: EdgeType = match type_raw.parse() {
                    Ok(e) => e,
                    Err(e) => return Err(e.to_string()),
                };
                let from_id = resolve(store, &from);
                let to_id = resolve(store, &to);
                set_edge_weight(store, &from_id, &to_id, edge, weight.clamp(0.0, 1.0))
                    .map(|()| (edge, from_id, to_id))
                    .map_err(|e| e.to_string())
            })
            .await;
        let (edge, from_id, to_id) = match local {
            Ok(v) => v,
            Err(e) => return json!({ "reweighted": false, "error": e }),
        };
        let mut out = json!({
            "reweighted": true, "from": from_id, "to": to_id,
            "edge_type": edge.as_str(), "weight": weight.clamp(0.0, 1.0)
        });
        if let Some(note) = propagate_edge(
            mem,
            a.cloud.as_deref(),
            &from_id,
            &to_id,
            edge,
            EdgeChange::Upsert(weight.clamp(0.0, 1.0)),
            init.as_deref(),
        )
        .await
        {
            out["cloud"] = note;
        }
        out
    }
);

#[derive(Debug, Deserialize)]
pub struct UnlinkArgs {
    pub from: String,
    pub to: String,
    #[serde(default = "default_edge_type")]
    pub edge_type: String,
    #[serde(default)]
    pub cloud: Option<String>,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_cloud!(
    /// `kaeru_unlink` — retract an edge (bi-temporal; history kept).
    Unlink,
    "kaeru_unlink",
    "Retract a previously-asserted edge between two nodes. Bi-temporal — historical reads still \
     see it; only reads at NOW skip it. Takes no weight: there is nothing to weigh about an edge \
     being removed.",
    UnlinkArgs,
    { "type": "object", "properties": {
        "from": { "type": "string", "description": "source node name or id" },
        "to": { "type": "string", "description": "destination node name or id" },
        "edge_type": { "type": "string", "description": "link type (default refers_to)" },
        "cloud": { "type": "string", "description": "which cloud to mirror the retraction to, when several are configured" },
        "initiative": { "type": "string", "description": "optional initiative (project) to resolve within; omit for your default" }
    }, "required": ["from", "to"] },
    |mem, a| {
        let init = target_initiative(mem, &a.initiative);
        let (from, to, type_raw) = (a.from.clone(), a.to.clone(), a.edge_type.clone());
        let local = mem
            .blocking_in(init.clone(), move |store| {
                let edge: EdgeType = match type_raw.parse() {
                    Ok(e) => e,
                    Err(e) => return Err(e.to_string()),
                };
                let from_id = resolve(store, &from);
                let to_id = resolve(store, &to);
                unlink(store, &from_id, &to_id, edge)
                    .map(|()| (edge, from_id, to_id))
                    .map_err(|e| e.to_string())
            })
            .await;
        let (edge, from_id, to_id) = match local {
            Ok(v) => v,
            Err(e) => return json!({ "unlinked": false, "error": e }),
        };
        let mut out = json!({
            "unlinked": true, "from": from_id, "to": to_id, "edge_type": edge.as_str()
        });
        if let Some(note) = propagate_edge(
            mem,
            a.cloud.as_deref(),
            &from_id,
            &to_id,
            edge,
            EdgeChange::Retract,
            init.as_deref(),
        )
        .await
        {
            out["cloud"] = note;
        }
        out
    }
);

#[derive(Debug, Deserialize)]
pub struct TaskArgs {
    pub body: String,
    #[serde(default)]
    pub due: Option<String>,
    #[serde(default)]
    pub layer: Option<String>,
    /// The node this one connects to, by name or id — the edge is made in
    /// THIS call, while both ends are still in mind (#102).
    #[serde(default)]
    pub link_to: Option<String>,
    #[serde(default)]
    pub edge_type: Option<String>,
    /// Required with `link_to`, and no default: the capture lands without
    /// it, the edge does not.
    #[serde(default)]
    pub weight: Option<f64>,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_task` — record a todo with an optional deadline.
    Task,
    "kaeru_task",
    "Record a task / todo that should survive into the next session. `due` takes a date \
     (`2026-07-01`), a datetime, or a distance into the future (`3d`, `2w`). Open tasks resurface \
     via `kaeru_awake`, overdue ones first. Pass `initiative` to file it under a specific \
     project; omit for your default.",
    TaskArgs,
    { "type": "object", "properties": {
        "body": { "type": "string", "description": "what needs doing" },
        "due": { "type": "string", "description": "deadline: `2026-07-01`, an RFC-3339 datetime, or `3d` / `2w` from now" },
        "layer": { "type": "string", "description": "memory layer at creation: core / hot / warm (default) / cold / frozen" },
        "link_to": { "type": "string", "description": "node (name or id) to link this capture to, in the same call" },
        "edge_type": { "type": "string", "description": "edge type for `link_to` (default refers_to)" },
        "weight": { "type": "number", "description": "how load-bearing the `link_to` edge is, 0..1 — required with `link_to`" },
        "initiative": { "type": "string", "description": "optional initiative (project) to file this under; omit for your default" }
    }, "required": ["body"] },
    |store, args| {
        let layer = match parse_layer(args.layer.as_deref()) {
            Ok(l) => l,
            Err(e) => return json!({ "created": false, "error": e }),
        };
        let due = match args.due.as_deref() {
            Some(raw) => match parse_due_to_iso(raw) {
                Ok(iso) => Some(iso),
                Err(e) => return json!({ "created": false, "error": e.to_string() }),
            },
            None => None,
        };
        match write_task_with_layer(store, &args.body, due.as_deref(), layer) {
            Ok(id) => {
                let edge = link_at_capture(
                    store,
                    &id,
                    args.link_to.as_deref(),
                    args.edge_type.as_deref(),
                    args.weight,
                );
                json!({ "created": true, "id": id, "due": due, "link": edge })
            }
            Err(e) => json!({ "created": false, "error": e.to_string() }),
        }
    }
);

#[derive(Debug, Deserialize)]
pub struct DoneArgs {
    pub name: String,
    #[serde(default)]
    pub initiative: Option<String>,
}

mem_tool_in!(
    /// `kaeru_done` — mark a task complete.
    Done,
    "kaeru_done",
    "Mark a task complete (sets its status to done). Pass the task's name or id.",
    DoneArgs,
    { "type": "object", "properties": {
        "name": { "type": "string", "description": "task name or id" },
        "initiative": { "type": "string", "description": "optional initiative (project) to resolve within; omit for your default" }
    }, "required": ["name"] },
    |store, args| {
        let id = resolve(store, &args.name);
        match complete_task(store, &id) {
            Ok(()) => json!({ "done": true, "id": id }),
            Err(e) => json!({ "done": false, "error": e.to_string() }),
        }
    }
);
