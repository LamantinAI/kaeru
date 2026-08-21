//! Input hygiene for the capture boundary.
//!
//! kaeru is an MCP server: its callers are arbitrary LLMs, and a malformed
//! tool call can spill the invocation envelope — the parameter open/close
//! tags, an invoke wrapper — into a string argument (a node `name` or
//! `body`). Stored verbatim, that garbage pollutes the graph and every
//! downstream read. The server cannot control how a model formats a call,
//! so it defends at the write boundary instead of trusting the input.
//!
//! [`strip_tool_call_markup`] removes that leaked envelope. It is
//! deliberately **narrow**: it keys off the specific tool-call markers,
//! never angle-bracket markup in general, so legitimate content — code
//! (`Vec<u8>`), an XML/HTML snippet, even a literal `</body>` inside prose —
//! passes through untouched. High precision, so it only fires on an actual
//! leak, never on a body that merely contains `<...>`.

/// Substrings unique to the tool-call invocation envelope. Chosen to match
/// both the plain and the `antml:`-namespaced spellings via a single token
/// (e.g. `parameter>` matches both `</parameter>` and its namespaced form),
/// and kept specific (`parameter name=`, not a bare `parameter`) so ordinary
/// prose about "a parameter" can't trip them. None of these appears in
/// legitimate captured content.
const ENVELOPE_MARKERS: &[&str] = &[
    "parameter name=",
    "parameter>",
    "invoke name=",
    "invoke>",
    "function_calls>",
    "function_results>",
];

/// Strips a leaked tool-call envelope from `input`, returning the cleaned
/// string and `true` when anything was removed. Input with no envelope
/// marker comes back unchanged (`false`).
///
/// The leak is always a *tail*: real content, then the spilled envelope
/// (`…real text.</body>\n<parameter name="initiative">rack`). The cut is made
/// at the tag that opens the earliest envelope marker, after also dropping an
/// immediately preceding orphan closing tag (`</body>` / `</name>` — the
/// close of the parameter the content belonged to) that would otherwise be
/// left dangling. A standalone `</body>` in genuine prose is untouched: it is
/// only removed when it directly abuts the envelope.
pub fn strip_tool_call_markup(input: &str) -> (String, bool) {
    let Some(marker_at) = ENVELOPE_MARKERS.iter().filter_map(|m| input.find(m)).min() else {
        return (input.to_string(), false);
    };

    // Back up to the `<` that opens the marker's tag, so the whole tag
    // (plain or namespaced) is dropped, not just its inner token.
    let tag_start = input[..marker_at].rfind('<').unwrap_or(marker_at);
    let mut head = input[..tag_start].trim_end();

    // Drop a single orphan closing tag left from the leaked parameter, but
    // only when it directly abuts the envelope — a `</body>` elsewhere in
    // real prose never reaches this point.
    if head.ends_with('>') {
        if let Some(open) = head.rfind("</") {
            let inner = &head[open + 2..head.len() - 1];
            let simple_tag = !inner.is_empty()
                && inner
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | ':'));
            if simple_tag {
                head = head[..open].trim_end();
            }
        }
    }

    (head.to_string(), true)
}

#[cfg(test)]
mod tests {
    use super::strip_tool_call_markup;

    /// The real-world leak: a spilled parameter envelope tail is removed,
    /// including the orphan `</body>` that closed the leaked param.
    #[test]
    fn strips_the_observed_leak() {
        let dirty = "…max savings need a custom board, not a firmware toggle.</body>\n<parameter name=\"initiative\">rack";
        let (clean, stripped) = strip_tool_call_markup(dirty);
        assert!(stripped);
        assert_eq!(
            clean,
            "…max savings need a custom board, not a firmware toggle."
        );
    }

    /// A bare closing param tag with no orphan open tag before it.
    #[test]
    fn strips_a_trailing_close_tag() {
        let (clean, stripped) = strip_tool_call_markup("the answer</parameter>");
        assert!(stripped);
        assert_eq!(clean, "the answer");
    }

    /// The namespaced spelling is caught by the same `parameter>` token —
    /// assembled here so this file never emits the literal close tag.
    #[test]
    fn strips_the_namespaced_form() {
        let dirty = concat!("keep this</antml", ":parameter>");
        let (clean, stripped) = strip_tool_call_markup(dirty);
        assert!(stripped);
        assert_eq!(clean, "keep this");
    }

    /// Legitimate content with angle brackets is preserved — code, a
    /// comparison, an HTML-ish tag — none of it is envelope wire-format.
    #[test]
    fn preserves_legitimate_markup() {
        for s in [
            "let v: Vec<u8> = go(); if a < b && c > d { ok }",
            "the <body> element and its </body> close a page",
            "config: <threshold>0.7</threshold> stays as-is",
            "no markup here at all",
        ] {
            let (clean, stripped) = strip_tool_call_markup(s);
            assert!(!stripped, "should not fire on: {s}");
            assert_eq!(clean, s);
        }
    }

    /// A body that is nothing but envelope collapses to empty, still flagged.
    #[test]
    fn wholly_envelope_becomes_empty() {
        let (clean, stripped) = strip_tool_call_markup("<parameter name=\"body\">");
        assert!(stripped);
        assert_eq!(clean, "");
    }
}
