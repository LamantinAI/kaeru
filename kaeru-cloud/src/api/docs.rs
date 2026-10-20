//! The OpenAPI document for the cloud's REST surface (#70).
//!
//! The cloud used to publish no schema at all: `/openapi.json` and `/docs` were
//! both 404, so the only way to learn a payload was to send something and read
//! the 422. That method can only ever reveal *required* fields — validation
//! names one missing field at a time, so an optional field is invisible to the
//! whole procedure. It produced a concrete wrong conclusion in the field: an
//! agent decided `POST /api/v1/nodes` had no `initiative` field, posted a node
//! without one, and the node was accepted and invisible to `cloud_recall`.
//!
//! The document is **derived from the handlers**, not written beside them. Each
//! route is registered through `utoipa_axum::routes!`, so the path the router
//! serves and the path the spec describes are the same attribute — they cannot
//! disagree. A hand-maintained schema would have been a second description of
//! the payload, and this issue exists because a payload was described wrongly.
//!
//! Served always and unauthenticated. There is nothing to hide in the shape of
//! the API, and the schema is only useful to a caller before it has a token
//! working.

use serde_json::Value;
use utoipa::OpenApi;
use utoipa::openapi::schema::Schema;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::openapi::{OpenApi as OpenApiDocument, RefOr};

use crate::api::router::edges::EDGES_TAG;
use crate::api::router::health::HEALTH_TAG;
use crate::api::router::initiatives::INITIATIVES_TAG;
use crate::api::router::nodes::NODES_TAG;
pub use crate::models::ErrorBody;
use kaeru_core::{EdgeType, Layer, NodeType, Tier};

/// Name of the security scheme every authenticated operation references as
/// `security(("bearer" = []))` — the macro takes a literal, so the two must be
/// kept equal by hand, and a test checks they are.
pub const BEARER: &str = "bearer";

#[derive(OpenApi)]
#[openapi(
    info(
        title = "kaeru-cloud",
        description = "The shared cloud tier of kaeru: a team's shared nodes and the edges between \
                       them, behind one bearer token. Local daemons push with `share`, read with \
                       `pull` and `cloud_recall`, and retract with `unshare`."
    ),
    tags(
        (name = HEALTH_TAG, description = "Liveness, and the running kaeru-core version a daemon checks for skew."),
        (name = NODES_TAG, description = "Shared nodes: ingest (an upsert under the preserved id), read, retract."),
        (name = EDGES_TAG, description = "Edges between shared nodes: ingest (an upsert) and retract."),
        (name = INITIATIVES_TAG, description = "Discovery: what the cloud holds, per initiative, and team-wide rename / delete."),
    )
)]
pub struct CloudApiDoc;

/// Fields whose accepted values are a closed vocabulary owned by kaeru-core.
///
/// Listed in the schema as enums, taken from the core's own `VALID` lists. The
/// failure this closes is the same one #70 is about: a value the API accepts
/// but does not publish gets found by guessing — "related_to and friends are
/// not edge types" is a lesson agents kept learning from 400s. Writing the lists
/// into attributes would have been a second copy; this reads the first one.
const CLOSED_VOCABULARIES: [(&str, &str, &[&str], bool); 5] = [
    ("NodeIngestReq", "node_type", &NodeType::VALID, false),
    ("NodeIngestReq", "tier", &Tier::VALID, false),
    ("NodeIngestReq", "layer", &Layer::VALID, true),
    ("EdgeIngestReq", "edge_type", &EdgeType::VALID, false),
    ("EdgeRetractReq", "edge_type", &EdgeType::VALID, false),
];

/// Completes the document once the router has added its paths and schemas.
///
/// This cannot be a `utoipa::Modify`: modifiers run on the base document when
/// `CloudApiDoc::openapi()` is called, *before* `routes!` registers the
/// handlers — so at that point there are no request schemas to annotate, and a
/// modifier that looked for `NodeIngestReq` would find nothing and change
/// nothing, silently. Run after `split_for_parts`, everything is in place.
pub fn finalise(spec: &mut OpenApiDocument) {
    let components = spec.components.get_or_insert_with(Default::default);
    components.add_security_scheme(
        BEARER,
        SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
    );

    for (schema, field, values, nullable) in CLOSED_VOCABULARIES {
        if let Some(RefOr::T(Schema::Object(object))) = components.schemas.get_mut(schema)
            && let Some(RefOr::T(Schema::Object(property))) = object.properties.get_mut(field)
        {
            let mut allowed: Vec<Value> = values.iter().map(|v| Value::from(*v)).collect();
            // An optional field still accepts `null`; an enum without it would
            // tell a client that omitting the value by null is invalid.
            if nullable {
                allowed.push(Value::Null);
            }
            property.enum_values = Some(allowed);
        }
    }
}
