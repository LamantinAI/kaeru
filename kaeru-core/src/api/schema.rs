//! Equivalent schema composition with explicit object shapes for tool clients.

use serde_json::{Map, Value, json};

/// Some clients render a composition without its sibling object properties.
/// Repeat the implied shape inside the composition, retaining every original
/// guard. This changes presentation, not the set of accepted JSON values.
pub(super) fn expose_object_shapes(schema: &mut Value) {
    let Some(object) = schema.as_object_mut() else {
        return;
    };
    // Only schema positions are traversed: const/default/enum may contain
    // ordinary JSON objects whose keys happen to resemble schema keywords.
    for key in [
        "$defs",
        "definitions",
        "properties",
        "patternProperties",
        "dependentSchemas",
    ] {
        if let Some(children) = object.get_mut(key).and_then(Value::as_object_mut) {
            for child in children.values_mut() {
                expose_object_shapes(child);
            }
        }
    }
    for key in [
        "items",
        "additionalProperties",
        "unevaluatedProperties",
        "unevaluatedItems",
        "not",
        "if",
        "then",
        "else",
        "contains",
        "propertyNames",
        "contentSchema",
    ] {
        if let Some(child) = object.get_mut(key) {
            expose_object_shapes(child);
        }
    }
    for key in ["allOf", "anyOf", "oneOf", "prefixItems"] {
        if let Some(children) = object.get_mut(key).and_then(Value::as_array_mut) {
            for child in children {
                expose_object_shapes(child);
            }
        }
    }

    if object.get("type").and_then(Value::as_str) != Some("object")
        || !object.get("properties").is_some_and(Value::is_object)
    {
        return;
    }
    let shape = object_shape(object, None);
    // Preserve the original union branches, including exclusivity and guards.
    // Each extra shape is implied by the parent and that branch together.
    for key in ["anyOf", "oneOf"] {
        if let Some(branches) = object.get_mut(key).and_then(Value::as_array_mut) {
            for branch in branches {
                let original = branch.take();
                let explicit = object_shape(shape.as_object().unwrap(), original.as_object());
                *branch = json!({"allOf": [explicit, original]});
            }
        }
    }
    if let Some(conjuncts) = object.get_mut("allOf").and_then(Value::as_array_mut) {
        conjuncts.insert(0, shape);
    }
}

fn object_shape(parent: &Map<String, Value>, branch: Option<&Map<String, Value>>) -> Value {
    let mut shape = Map::new();
    for key in ["type", "properties", "required", "additionalProperties"] {
        if let Some(value) = parent.get(key) {
            shape.insert(key.into(), value.clone());
        }
    }
    if let Some(branch) = branch {
        if let Some(properties) = branch.get("properties").and_then(Value::as_object) {
            let inherited = shape
                .get_mut("properties")
                .unwrap()
                .as_object_mut()
                .unwrap();
            inherited.extend(properties.clone());
        }
        if let Some(required) = branch.get("required").and_then(Value::as_array) {
            let inherited = shape.entry("required").or_insert_with(|| json!([]));
            let inherited = inherited.as_array_mut().unwrap();
            for field in required {
                if !inherited.contains(field) {
                    inherited.push(field.clone());
                }
            }
        }
    }
    Value::Object(shape)
}

#[cfg(test)]
mod tests {
    use jsonschema::validator_for;
    use serde_json::json;

    use super::expose_object_shapes;

    #[test]
    fn schema_keywords_inside_literal_data_are_not_transformed() {
        let literal = json!({"type":"object","properties":{"a":{"type":"string"}},"allOf":[{"required":["a"]}]});
        let original = json!({
            "type":"object",
            "properties":{"literal":{"const":literal,"default":literal,"enum":[literal]}},
            "additionalProperties":false,
            "allOf":[{"minProperties":1}]
        });
        let mut advertised = original.clone();
        expose_object_shapes(&mut advertised);
        assert_eq!(advertised["properties"], original["properties"]);
        let value = json!({"literal":literal});
        assert!(validator_for(&original).unwrap().is_valid(&value));
        assert!(validator_for(&advertised).unwrap().is_valid(&value));
    }
}
