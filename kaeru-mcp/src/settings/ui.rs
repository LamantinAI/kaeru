//! Validated browser mount, independent of service transports and account origin.
use serde::Deserialize;

const MAX_BASE_PATH_BYTES: usize = 256;
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub struct UiMount(String);
impl TryFrom<String> for UiMount {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}
impl UiMount {
    pub fn parse(value: &str) -> Result<Self, String> {
        if value.is_empty() {
            return Ok(Self::default());
        }
        let path = value.strip_suffix('/').unwrap_or(value);
        if path.is_empty()
            || path.len() > MAX_BASE_PATH_BYTES
            || !path.starts_with('/')
            || !path
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'/' | b'-' | b'_' | b'.'))
            || path[1..]
                .split('/')
                .any(|s| s.is_empty() || s == "." || s == "..")
        {
            return Err("UI base path must be an absolute path of nonempty ASCII segments without URL, escapes or dot segments".into());
        }
        let mount = Self(path.into());
        mount.validate_mounts(&["/v1", "/auth", "/admin"])?;
        Ok(mount)
    }
    pub fn validate_mounts(&self, paths: &[&str]) -> Result<(), String> {
        if !self.prefixed() {
            return Ok(());
        }
        for path in paths {
            let other = path.trim_end_matches('/');
            if other.is_empty()
                || self.0 == other
                || self.0.starts_with(&format!("{other}/"))
                || other.starts_with(&format!("{}/", self.0))
            {
                return Err("UI base path overlaps another transport or API mount".into());
            }
        }
        Ok(())
    }
    pub fn prefixed(&self) -> bool {
        !self.0.is_empty()
    }
    pub fn page_root(&self) -> &str {
        if self.prefixed() { &self.0 } else { "/ui" }
    }
    pub fn service_root(&self) -> &str {
        &self.0
    }
    pub fn cookie_path(&self) -> &str {
        if self.prefixed() { &self.0 } else { "/" }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paths_are_explicit_normalized_and_transport_independent() {
        let legacy = UiMount::parse("").unwrap();
        assert_eq!(legacy.page_root(), "/ui");
        assert_eq!(legacy.service_root(), "");
        assert_eq!(legacy.cookie_path(), "/");
        for input in ["/memory", "/memory/", "/tools/memory/"] {
            let p = UiMount::parse(input).unwrap();
            assert_eq!(p.page_root(), input.trim_end_matches('/'));
            assert_eq!(p.cookie_path(), p.page_root());
            assert!(p.prefixed());
        }
        for input in [
            "/",
            "//",
            "//elsewhere",
            "relative",
            "https://example.test",
            "/a//b",
            "/a//",
            "/a/../b",
            "/a/./b",
            "/a?x",
            "/a#x",
            "/a%2fb",
            "/a\\b",
            "/a b",
            "/a\nb",
            "/日本",
            "/v1",
            "/v1/nested",
            "/auth",
            "/admin/settings",
        ] {
            assert!(UiMount::parse(input).is_err(), "{input:?}");
        }
        let p = UiMount::parse("/tools/memory").unwrap();
        assert!(p.validate_mounts(&["/tools"]).is_err());
        assert!(p.validate_mounts(&["/tools/memory/mcp"]).is_err());
        assert!(p.validate_mounts(&["/mcp", "/sse", "/messages"]).is_ok());
    }
}
