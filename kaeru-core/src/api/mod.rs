//! Canonical typed memory contract, shared by MCP, HTTP and framework adapters.

use std::collections::{BTreeMap, BTreeSet};
use std::result::Result;
use std::sync::OnceLock;

use chrono::Utc;
use cozo::{DataValue, JsonData, ScriptMutability};
use jsonschema::Validator;
use jsonschema::draft202012::options;
use schemars::schema_for;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value, from_str, from_value, json, to_string, to_value, to_vec};
use sha2::{Digest, Sha256};

use self::access::{check_node, check_scope, resolve};
use self::read::{dispatch as read_dispatch, entity as read_entity};
use self::surface::DEFINITIONS;
use self::write::{dispatch as write_dispatch, run_hygiene, supplements};
use crate::config::api::API_REPLAY_TTL_SECONDS;
use crate::graph::audit::write_audit;
use crate::{Error, Store, read_node_full};

mod access;
mod graph;
pub(crate) mod read;
mod surface;
mod write;
pub use crate::models::{
    ApiOptions, Artifact, ArtifactSource, Change, CheckEvidence, ContentRange, ContextData,
    Continuation, Coverage, EdgeChange, EdgeMirror, Entity, Evidence, EvidenceResult,
    ExistingEvidence, FailedStep, HistoryData, HistoryEvent, IdRef, InitiativesData, Issue,
    MutationData, NodeData, NodeRef, Omission, RecordData, RelationData, Response, ResponseData,
    RetentionProposal, ScopedRef, SearchData, Section, SlotChange, Source, State, TextSource,
    ToolAnnotations, ToolDefinition, TrailData,
};

type ApiResult<T> = Result<T, Issue>;

/// Version of the memory response contract, independent of the MCP transport
/// version and the crate/release version.
pub use crate::config::API_VERSION;

/// Resolve a reference inside an already scoped store span. Adapters use the
/// same ambiguity and membership checks as local commands before network I/O.
pub fn resolve_node(store: &Store, reference: &NodeRef, options: &ApiOptions) -> ApiResult<String> {
    resolve(store, reference, options, None, false, false)
}

pub fn authorize_arguments(name: &str, arguments: &Value, options: &ApiOptions) -> ApiResult<()> {
    let args = Args(arguments.clone());
    if let Some(scope) = args.text("initiative") {
        check_scope(&scope, options)?;
    }
    if name == "search" {
        for scope in args.parse::<Vec<String>>("initiatives")? {
            check_scope(&scope, options)?;
        }
    }
    if name == "initiative_manage" {
        for key in ["old", "new", "source", "target", "to", "name"] {
            if let Some(scope) = args.text(key) {
                check_scope(&scope, options)?;
            }
        }
    }
    if !options.include_local && !is_read(name, args.text("action").as_deref(), &args) {
        return Err(Issue::new(
            "READ_ONLY_SCOPE",
            "this viewer is not authorized for local mutations",
        ));
    }
    Ok(())
}

pub fn tool_definitions() -> &'static [ToolDefinition] {
    static TOOLS: OnceLock<Vec<ToolDefinition>> = OnceLock::new();
    TOOLS.get_or_init(|| from_str(DEFINITIONS).expect("compiled canonical definitions are valid"))
}

pub fn validate(name: &str, arguments: &Value) -> ApiResult<()> {
    static VALIDATORS: OnceLock<BTreeMap<String, Validator>> = OnceLock::new();
    let validators = VALIDATORS.get_or_init(|| {
        tool_definitions()
            .iter()
            .map(|tool| {
                (
                    tool.name.clone(),
                    options()
                        .should_validate_formats(true)
                        .build(&tool.input_schema)
                        .expect("compiled canonical schemas are valid"),
                )
            })
            .collect()
    });
    let validator = validators
        .get(name)
        .ok_or_else(|| Issue::new("UNKNOWN_TOOL", format!("unknown memory operation {name}")))?;
    validator.validate(arguments).map_err(|error| {
        Issue::new("INVALID_ARGUMENT", error.to_string()).field(error.instance_path.to_string())
    })
}

pub fn output_schema() -> Value {
    to_value(schema_for!(Response)).expect("response schema serializes")
}

/// Each advertised result schema contains only the DTOs that operation can return.
pub fn output_schema_for(name: &str) -> Value {
    let mut schema = output_schema();
    let kind = match name {
        "read" => "RecordData",
        "search" => "SearchData",
        "context" | "reflect" => "ContextData",
        "history" => "HistoryData",
        "initiatives" => "InitiativesData",
        _ => "MutationData",
    };
    let data = if name == "retention" {
        json!({"anyOf":[{"$ref":"#/$defs/ContextData"},{"$ref":"#/$defs/MutationData"}]})
    } else {
        json!({"$ref":format!("#/$defs/{kind}")})
    };
    schema["properties"]["data"] = data;
    schema["properties"]["api_version"]["const"] = json!(API_VERSION);
    schema["properties"]["operation"]["const"] = json!(name);
    // Rejected reads use the empty mutation-result DTO to report no committed change.
    if kind != "MutationData" && name != "retention" {
        schema["properties"]["data"] =
            json!({"anyOf":[{"$ref":format!("#/$defs/{kind}")},{"$ref":"#/$defs/MutationData"}]});
    }
    let all = schema["$defs"].as_object().cloned().unwrap_or_default();
    let mut needed = BTreeSet::new();
    fn visit(value: &Value, all: &Map<String, Value>, needed: &mut BTreeSet<String>) {
        match value {
            Value::Object(object) => {
                if let Some(key) = object
                    .get("$ref")
                    .and_then(Value::as_str)
                    .and_then(|s| s.strip_prefix("#/$defs/"))
                    && needed.insert(key.into())
                    && let Some(def) = all.get(key)
                {
                    visit(def, all, needed);
                }
                for (key, value) in object {
                    if key != "$defs" {
                        visit(value, all, needed);
                    }
                }
            }
            Value::Array(array) => {
                for value in array {
                    visit(value, all, needed)
                }
            }
            _ => {}
        }
    }
    visit(&schema, &all, &mut needed);
    schema["$defs"] = Value::Object(
        needed
            .into_iter()
            .filter_map(|key| all.get(&key).cloned().map(|value| (key, value)))
            .collect(),
    );
    schema
}

pub fn execute(store: &Store, name: &str, mut arguments: Value, options: &ApiOptions) -> Response {
    normalize_arguments(name, &mut arguments);
    if let Err(issue) = validate(name, &arguments) {
        return Response::rejected(name, issue);
    }
    if let Err(issue) = authorize_arguments(name, &arguments, options) {
        return Response::rejected(name, issue);
    }
    let args = Args(arguments.clone());
    let initiative = args.text("initiative");
    let action = args.text("action");
    let request_id = args.text("request_id");
    let _request_guard = request_id
        .as_ref()
        .map(|_| store.api_requests.lock().unwrap_or_else(|p| p.into_inner()));
    if name == "retention" && action.as_deref() == Some("run_hygiene") {
        let replay = store.scoped(initiative.as_deref(), |store| {
            cached(store, name, &args, options)
        });
        match replay {
            Ok(Some(mut r)) => {
                r.replayed = true;
                return r;
            }
            Err(e) => return Response::rejected(name, e),
            _ => {}
        }
        let mut r = run_hygiene(store, &args, options)
            .unwrap_or_else(|issue| Response::rejected(name, issue));
        r.request_id = request_id;
        if !matches!(r.state, State::Rejected)
            && let Err(issue) = store.scoped(initiative.as_deref(), |store| {
                save_result(store, name, &args, options, &r)
            })
        {
            r.state = State::Partial;
            r.issues.push(issue);
        }
        return r;
    }
    let mut result = store.scoped(initiative.as_deref(), |store| {
        if let Some(scope) = initiative.as_deref() {
            check_scope(scope, options)?;
        }
        if is_read(name, action.as_deref(), &args) {
            return read_dispatch(store, name, &args, options);
        }
        if name == "export" {
            if let Some(mut response) = cached(store, name, &args, options)? {
                response.replayed = true;
                return Ok(response);
            }
            let mut response = write_dispatch(store, name, &args, options)?;
            response.request_id = request_id.clone();
            if let Err(issue) = save_result(store, name, &args, options, &response) {
                response.state = State::Partial;
                response.issues.push(issue);
            }
            return Ok(response);
        }
        let mut response = store
            .atomic(|store| {
                let attempt: ApiResult<Response> = (|| {
                    if let Some(mut r) = cached(store, name, &args, options)? {
                        r.replayed = true;
                        return Ok(r);
                    }
                    let mut r = write_dispatch(store, name, &args, options)?;
                    r.request_id = request_id.clone();
                    if args.has("related")
                        && let ResponseData::Mutation(d) = &mut r.data
                    {
                        d.failed_steps.push(FailedStep {
                            step: "related".into(),
                            code: "RELATION_PENDING".into(),
                            message: "capture committed; relation supplement has not completed"
                                .into(),
                        });
                        r.state = State::Partial;
                    }
                    save_result(store, name, &args, options, &r)?;
                    Ok(r)
                })();
                match attempt {
                    Ok(response) => Ok(response),
                    Err(issue) => Err(Error::Invalid(to_string(&issue).expect("issue serializes"))),
                }
            })
            .map_err(issue_from_error)?;
        if !response.replayed {
            if let ResponseData::Mutation(d) = &mut response.data {
                d.failed_steps.retain(|s| s.code != "RELATION_PENDING");
            }
            if !matches!(response.state, State::Unchanged) {
                response.state = State::Applied;
            }
            response = supplements(store, name, &args, options, response)?;
            if let ResponseData::Mutation(d) = &response.data
                && !d.failed_steps.is_empty()
            {
                response.state = State::Partial;
            }
            refresh_entities(store, &mut response, options);
            save_result(store, name, &args, options, &response)?;
        }
        Ok(response)
    });
    if let Ok(response) = result.as_mut() {
        response.request_id = request_id;
    }
    result.unwrap_or_else(|issue| Response::rejected(name, issue))
}

pub fn normalize_arguments(name: &str, arguments: &mut Value) {
    fn walk(v: &mut Value) {
        match v {
            Value::Object(map) => {
                for (k, v) in map {
                    if k == "initiative"
                        && let Some(s) = v.as_str()
                    {
                        *v = json!(s.trim());
                    } else {
                        walk(v);
                    }
                }
            }
            Value::Array(a) => {
                for v in a {
                    walk(v);
                }
            }
            Value::Number(n) if n.is_f64() => {
                if let Some(f) = n.as_f64()
                    && f.fract() == 0.0
                    && f >= i64::MIN as f64
                    && f < i64::MAX as f64
                {
                    *v = json!(f as i64);
                }
            }
            _ => {}
        }
    }
    walk(arguments);
    if name == "search"
        && let Some(a) = arguments
            .get_mut("initiatives")
            .and_then(Value::as_array_mut)
    {
        for v in a {
            if let Some(s) = v.as_str() {
                *v = json!(s.trim());
            }
        }
    }
    if name == "initiative_manage" {
        for key in ["old", "new", "source", "target", "name", "to"] {
            if let Some(v) = arguments.get_mut(key)
                && let Some(s) = v.as_str()
            {
                *v = json!(s.trim());
            }
        }
    }
}

fn request_hash(name: &str, args: &Args) -> String {
    let mut value = args.0.clone();
    value.as_object_mut().unwrap().remove("request_id");
    match name {
        "note" => {
            value
                .as_object_mut()
                .unwrap()
                .entry("kind")
                .or_insert(json!("observation"));
        }
        "claim" => {
            value
                .as_object_mut()
                .unwrap()
                .entry("initial_status")
                .or_insert(json!("open"));
        }
        "verdict" => {
            value
                .as_object_mut()
                .unwrap()
                .entry("action")
                .or_insert(json!("decide"));
        }
        "trail" => {
            value
                .as_object_mut()
                .unwrap()
                .entry("action")
                .or_insert(json!("save"));
        }
        _ => {}
    }
    hash(&value)
}

fn cached(
    store: &Store,
    name: &str,
    args: &Args,
    options: &ApiOptions,
) -> ApiResult<Option<Response>> {
    let Some(key) = args.text("request_id") else {
        return Ok(None);
    };
    let rows=store.db_ref().run_script("?[tool,arguments_hash,result,created_at] := *api_operation{caller,request_id,tool,arguments_hash,result,created_at},caller=$caller,request_id=$key",BTreeMap::from([("caller".into(),DataValue::Str(options.caller.clone().into())),("key".into(),DataValue::Str(key.into()))]),ScriptMutability::Immutable).map_err(Error::from).map_err(issue_from_error)?;
    let Some(row) = rows.rows.first() else {
        return Ok(None);
    };
    if row[0].get_str() != Some(name) || row[1].get_str() != Some(request_hash(name, args).as_str())
    {
        return Err(Issue::new(
            "IDEMPOTENCY_KEY_REUSED",
            "request_id was used with another tool or input",
        ));
    }
    let created = row[3].get_float().unwrap_or(0.0);
    if Utc::now().timestamp() as f64 - created > API_REPLAY_TTL_SECONDS as f64 {
        return Err(Issue::new(
            "IDEMPOTENCY_EXPIRED",
            "replay window expired; inspect current state before a new operation",
        ));
    }
    let response: Response = from_value(Value::from(row[2].clone()))
        .map_err(|e| Issue::new("SUBSTRATE_ERROR", e.to_string()))?;
    if let ResponseData::Mutation(data) = &response.data {
        for id in data
            .primary
            .iter()
            .filter(|_| !matches!(name, "relate" | "board_config" | "policy"))
            .chain(data.created.iter())
            .chain(data.affected.iter())
            .map(|e| e.id.as_str())
            .chain(data.evidence.iter().map(|e| e.id.as_str()))
            .chain(
                data.slots
                    .iter()
                    .flat_map(|s| s.before.iter().chain(s.after.iter()).map(String::as_str)),
            )
        {
            check_node(store, id, None, options, true)?;
        }
        for change in &data.changes {
            if change.field == "edge" {
                for value in [&change.before, &change.after] {
                    if let Ok(edge) = from_value::<RelationData>(value.clone()) {
                        check_node(store, &edge.from, None, options, true)?;
                        if edge.dst_store == "local" {
                            check_node(store, &edge.to, None, options, true)?;
                        }
                    }
                }
            }
        }
    }
    Ok(Some(response))
}

fn save_result(
    store: &Store,
    name: &str,
    args: &Args,
    options: &ApiOptions,
    response: &Response,
) -> ApiResult<()> {
    let Some(key) = args.text("request_id") else {
        return Ok(());
    };
    let params = BTreeMap::from([
        (
            "caller".into(),
            DataValue::Str(options.caller.clone().into()),
        ),
        ("key".into(), DataValue::Str(key.into())),
        ("tool".into(), DataValue::Str(name.into())),
        (
            "hash".into(),
            DataValue::Str(request_hash(name, args).into()),
        ),
        (
            "result".into(),
            DataValue::Json(JsonData(to_value(response).unwrap())),
        ),
    ]);
    store.db_ref().run_script("?[caller,request_id,tool,arguments_hash,result] <- [[$caller,$key,$tool,$hash,$result]] :put api_operation {caller,request_id=>tool,arguments_hash,result}",params,ScriptMutability::Mutable).map_err(Error::from).map_err(issue_from_error)?;
    Ok(())
}

/// Adapters record completion of an optional remote step without repeating the
/// local semantic command. The original request remains bound to its arguments.
pub fn finish_remote(
    store: &Store,
    name: &str,
    arguments: Value,
    options: &ApiOptions,
    response: &Response,
) -> Response {
    let mut arguments = arguments;
    normalize_arguments(name, &mut arguments);
    let args = Args(arguments);
    let mut response = response.clone();
    let key = args.text("request_id");
    let _guard = key
        .as_ref()
        .map(|_| store.api_requests.lock().unwrap_or_else(|p| p.into_inner()));
    let recorded = store.scoped(args.text("initiative").as_deref(), |store| {
        if key.is_some() {
            cached(store, name, &args, options)?
                .ok_or_else(|| Issue::new("NOT_FOUND", "local operation has no replay record"))?;
        }
        refresh_entities(store, &mut response, options);
        save_result(store, name, &args, options, &response)
    });
    if let Err(issue) = recorded {
        response.issues.push(issue);
    }
    response
}

fn is_read(name: &str, action: Option<&str>, args: &Args) -> bool {
    matches!(
        name,
        "context" | "search" | "read" | "reflect" | "history" | "initiatives"
    ) || name == "retention" && action == Some("preview_hygiene")
        || name == "policy" && !args.has("policy") && !args.has("clouds")
}

pub(super) struct Args(pub Value);
impl Args {
    fn text(&self, key: &str) -> Option<String> {
        self.0.get(key).and_then(Value::as_str).map(str::to_string)
    }
    fn required_text(&self, key: &str) -> ApiResult<String> {
        self.text(key).ok_or_else(|| {
            Issue::new("INVALID_ARGUMENT", format!("{key} must be a string")).field(key)
        })
    }
    fn has(&self, key: &str) -> bool {
        self.0.get(key).is_some()
    }
    fn value(&self, key: &str) -> &Value {
        self.0.get(key).unwrap_or(&Value::Null)
    }
    fn parse<T: DeserializeOwned>(&self, key: &str) -> ApiResult<T> {
        from_value(self.value(key).clone())
            .map_err(|e| Issue::new("INVALID_ARGUMENT", e.to_string()).field(key))
    }
    fn optional<T: DeserializeOwned>(&self, key: &str) -> ApiResult<Option<T>> {
        if self.has(key) {
            self.parse(key).map(Some)
        } else {
            Ok(None)
        }
    }
    fn action(&self, default: &str) -> String {
        self.text("action").unwrap_or_else(|| default.into())
    }
}

pub(super) fn issue_from_error(error: Error) -> Issue {
    match error {
        Error::Invalid(message) => {
            from_str(&message).unwrap_or_else(|_| Issue::new("INVALID_ARGUMENT", message))
        }
        Error::NotFound(message) => Issue::new("NOT_FOUND", message),
        other => Issue::new("SUBSTRATE_ERROR", other.to_string()),
    }
}

pub(super) fn hash(value: &Value) -> String {
    format!(
        "{:x}",
        Sha256::digest(to_vec(value).expect("JSON serializes"))
    )
}

pub(super) fn properties(store: &Store, id: &str) -> ApiResult<Value> {
    let node = read_node_full(store, &id.to_string())
        .map_err(issue_from_error)?
        .ok_or_else(|| Issue::new("NOT_FOUND", format!("node {id} is not live")))?;
    Ok(node.properties.unwrap_or_else(|| json!({})))
}

pub(super) fn put_properties(store: &Store, id: &str, value: Value) -> ApiResult<()> {
    let params = BTreeMap::from([
        ("id".into(), DataValue::Str(id.into())),
        ("properties".into(), DataValue::Json(JsonData(value))),
    ]);
    store.db_ref().run_script(
        "?[id, validity, type, tier, name, body, tags, initiatives, properties, visibility, layer] := \
         *node{id, validity, type, tier, name, body, tags, initiatives, visibility, layer @ 'NOW'}, \
         id = $id, properties = $properties \
         :put node {id, validity => type, tier, name, body, tags, initiatives, properties, visibility, layer}",
        params,ScriptMutability::Mutable).map_err(Error::from).map_err(issue_from_error)?;
    write_audit(
        store.db_ref(),
        "metadata_update",
        "system",
        &[id.to_string()],
    )
    .map_err(issue_from_error)?;
    Ok(())
}

/// Obtain resolved IDs from the committed result rather than resolving a
/// concurrently renamed node again before network I/O.
pub fn edge_mirror(response: &Response) -> Option<EdgeMirror> {
    if response.operation != "relate"
        || matches!(response.state, State::Rejected | State::Unchanged)
    {
        return None;
    }
    let ResponseData::Mutation(data) = &response.data else {
        return None;
    };
    let edge = data.changes.iter().find(|c| c.field == "edge")?;
    let value = if edge.after.is_null() {
        &edge.before
    } else {
        &edge.after
    };
    let relation: RelationData = from_value(value.clone()).ok()?;
    if relation.dst_store != "local" {
        return None;
    }
    Some(EdgeMirror {
        from: relation.from,
        to: relation.to,
        edge_type: relation.edge_type.parse().ok()?,
        change: if edge.after.is_null() {
            EdgeChange::Retract
        } else {
            EdgeChange::Upsert(relation.weight)
        },
    })
}

fn refresh_entities(store: &Store, response: &mut Response, options: &ApiOptions) {
    if let ResponseData::Mutation(data) = &mut response.data {
        for entity in data
            .primary
            .iter_mut()
            .filter(|_| {
                !matches!(
                    response.operation.as_str(),
                    "relate" | "board_config" | "policy"
                )
            })
            .chain(data.created.iter_mut())
            .chain(data.affected.iter_mut())
        {
            if let Ok(actual) = read_entity(store, &entity.id, options) {
                *entity = actual;
            }
        }
    }
}
