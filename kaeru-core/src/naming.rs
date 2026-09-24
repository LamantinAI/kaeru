//! The name a capture gives itself.
//!
//! A verb that takes text and not a name still has to store one, because a
//! name is how a node is recalled by hand. The rule lives here rather than in
//! an adapter so `claim` invents the same name over MCP and in process (#98).

use crate::graph::new_node_id;

/// The name a text-only capture is stored under: its first five alphanumeric
/// words, plus six characters of a fresh id so two claims that open the same
/// way are still distinct. `fallback` names the node when the text has no
/// usable words at all.
pub fn derive_auto_name(text: &str, fallback: &str) -> String {
    const MAX_WORDS: usize = 5;
    let mut words: Vec<String> = Vec::new();
    for raw in text.split_whitespace() {
        let cleaned: String = raw
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect::<String>()
            .to_lowercase();
        if !cleaned.is_empty() {
            words.push(cleaned);
            if words.len() >= MAX_WORDS {
                break;
            }
        }
    }
    let id = new_node_id();
    let suffix: String = id
        .chars()
        .rev()
        .take(6)
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    if words.is_empty() {
        format!("{fallback}-{suffix}")
    } else {
        format!("{}-{suffix}", words.join("-"))
    }
}
