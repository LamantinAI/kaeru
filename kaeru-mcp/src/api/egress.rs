//! Operator-owned scope configuration and response redaction. Scope and
//! visibility authorization run in the shared core API before content reads.

use axum::http::header::{ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue};
use axum::response::Response;
use kaeru_core::guard::scan_public;
use serde_json::Value;

#[derive(Clone, Default)]
pub struct ApiConfig {
    /// Empty permits no initiative; request scopes only narrow this ceiling.
    pub allow: Vec<String>,
    pub deny: Vec<String>,
    /// Local nodes stay private unless the operator explicitly permits them.
    pub include_local: bool,
    pub allow_origin: Option<String>,
}

impl ApiConfig {
    pub fn finish(&self, response: &mut Response) {
        if let Some(origin) = self.allow_origin.as_deref()
            && let Ok(value) = origin.parse::<HeaderValue>()
        {
            response
                .headers_mut()
                .insert(ACCESS_CONTROL_ALLOW_ORIGIN, value);
        }
    }
    pub fn redact_value(&self, value: &mut Value) {
        redact(value);
    }
}

fn redact(value: &mut Value) {
    match value {
        Value::String(text) => {
            if !scan_public(text).is_empty() {
                *text = "[redacted]".into();
            }
        }
        Value::Array(array) => {
            for value in array {
                redact(value);
            }
        }
        Value::Object(object) => {
            for value in object.values_mut() {
                redact(value);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::ApiConfig;
    use serde_json::json;
    #[test]
    fn secrets_are_redacted_in_nested_response_fields() {
        let secret = format!("sk-{}", "a".repeat(40));
        let mut value = json!({"data":{"sections":[{"records":[{"body":secret}]}]},"safe":"A normal observation."});
        ApiConfig::default().redact_value(&mut value);
        assert_eq!(
            value["data"]["sections"][0]["records"][0]["body"],
            "[redacted]"
        );
        assert_eq!(value["safe"], "A normal observation.");
    }
}
