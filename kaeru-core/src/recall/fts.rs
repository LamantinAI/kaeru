//! Full-text search across `name` and `body` via Cozo's BM25-ish FTS
//! indexes (`node:fts_name`, `node:fts_body`). Fallback for cold queries
//! where the agent doesn't remember an exact name.
//!
//! Hits are unioned across both indexes, anchored at NOW (so retracted
//! rows don't surface), filtered by `current_initiative` when set, and
//! deduplicated by node id. The score that wins for a duplicate id is
//! the larger of the per-index scores.

use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};

use cozo::{DataValue, ScriptMutability};

use super::{NodeBrief, parse_brief};
use crate::errors::{Error, Result};
use crate::store::Store;

/// Maximum results [`fuzzy_recall`] may return per call. Mirrors the
/// pattern used elsewhere — bound the working set so the agent's
/// attention budget stays small.
pub const FUZZY_RECALL_LIMIT_CAP: usize = 50;

/// Searches the substrate for nodes whose `name` or `body` matches
/// `query`. Returns at most `limit` briefs ordered by descending FTS
/// score. `limit` is clamped to [`FUZZY_RECALL_LIMIT_CAP`].
///
/// `query` is the Cozo FTS expression — single tokens, `AND` / `OR` /
/// `NOT`, or quoted phrases. See Cozo docs for the full grammar. A trailing
/// `*` is a prefix match on the word it is written on, in any letter case.
pub fn fuzzy_recall(store: &Store, query: &str, limit: usize) -> Result<Vec<NodeBrief>> {
    // A bare `*` is a common guess for "show me everything" — the grammar has
    // no such wildcard, and its parse error explains nothing. Name the two
    // verbs that actually answer the question instead.
    if query.trim().chars().all(|c| c == '*') && !query.trim().is_empty() {
        return Err(Error::Invalid(
            "`*` alone matches nothing — FTS needs a term. Append `*` to a word \
             for prefix matching (`token*`), or list nodes with `recent` / `overview`."
                .to_string(),
        ));
    }
    // `topic:figma` / `kind:task` is field syntax the FTS grammar has no notion
    // of — but the fields exist, just behind another verb. Name it (#49).
    if let Some((field, _)) = query.trim().split_once(':')
        && matches!(
            field,
            "topic" | "kind" | "sig" | "role" | "lang" | "status" | "due"
        )
    {
        return Err(Error::Invalid(format!(
            "`{}` is a tag, not FTS syntax — read tags with `tagged \"{}\"`. \
             `search` matches words in names and bodies.",
            query.trim(),
            query.trim()
        )));
    }
    let query = split_prefix_terms(query);
    let query = query.as_ref();
    match run_fts(store, query, limit) {
        Ok(hits) => Ok(hits),
        // The FTS grammar rejects ordinary punctuation inside a token, and
        // kaeru's own naming convention is hyphenated slugs — so looking a node
        // up by the very name kaeru gave it was guaranteed to fail. Retry once
        // with the query rewritten into quoted phrases, which the grammar does
        // accept. Only after a parse failure, so a query that already works
        // keeps its exact meaning (prefix `*`, AND/OR/NOT, phrases).
        Err(e) => {
            let safe = quote_unparseable_tokens(query);
            if safe == query {
                return Err(e);
            }
            // Report the ORIGINAL error if the rewrite fails too: it describes
            // what the caller actually typed.
            run_fts(store, &safe, limit).map_err(|_| e)
        }
    }
}

/// Rewrites a query the FTS grammar would reject into one it accepts: any token
/// carrying punctuation is wrapped in quotes, which turns it into a phrase over
/// the same tokens (the Simple tokenizer splits on non-alphanumerics anyway, so
/// `"pilot-finalize"` matches the node named `pilot-finalize`).
///
/// Left untouched: the boolean operators, already-quoted phrases, and plain
/// tokens — including a trailing `*`, the documented inflection idiom. A
/// trailing `*` on a punctuated token is dropped, since prefix search and
/// phrases are mutually exclusive in the grammar; matching the phrase beats
/// failing to parse.
fn quote_unparseable_tokens(query: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut chars = query.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }
        // A quoted phrase is one token however many spaces it holds — splitting
        // on whitespace first would shred `"already a phrase"` into three.
        if c == '"' {
            chars.next();
            let mut phrase = String::new();
            for c in chars.by_ref() {
                if c == '"' {
                    break;
                }
                phrase.push(c);
            }
            if !phrase.trim().is_empty() {
                out.push(format!("\"{phrase}\""));
            }
            continue;
        }
        let mut tok = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                break;
            }
            tok.push(c);
            chars.next();
        }
        let is_operator = matches!(tok.as_str(), "AND" | "OR" | "NOT");
        let core = tok.strip_suffix('*').unwrap_or(&tok);
        let is_plain = !core.is_empty() && core.chars().all(|c| c.is_alphanumeric() || c == '_');
        if is_operator || is_plain {
            out.push(tok);
        } else if !core.is_empty() {
            out.push(format!("\"{}\"", core.replace('"', "")));
        }
    }
    if out.is_empty() {
        return query.to_string();
    }
    out.join(" ")
}

/// A query token, for the rewrites that only touch plain queries.
enum Term<'a> {
    Operator(&'a str),
    /// A quoted phrase, quotes included.
    Phrase(&'a str),
    Word {
        stem: &'a str,
        prefix: bool,
    },
}

/// Splits `query` into terms, or `None` if it uses anything beyond bare words,
/// quoted phrases and `AND` / `OR` / `NOT` — grouping, `NEAR`, boosters,
/// `,` / `;`, punctuation inside a word. Such queries are passed through as is.
fn terms(query: &str) -> Option<Vec<Term<'_>>> {
    let mut out = Vec::new();
    let mut rest = query.trim_start();
    while !rest.is_empty() {
        if let Some(after) = rest.strip_prefix('"') {
            let end = after.find('"')?;
            let (phrase, tail) = rest.split_at(end + 2);
            out.push(Term::Phrase(phrase));
            rest = tail.trim_start();
            continue;
        }
        let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
        let (tok, tail) = rest.split_at(end);
        rest = tail.trim_start();
        if matches!(tok, "AND" | "OR" | "NOT") {
            out.push(Term::Operator(tok));
            continue;
        }
        let (stem, prefix) = match tok.strip_suffix('*') {
            Some(stem) => (stem, true),
            None => (tok, false),
        };
        if stem.is_empty()
            || stem == "NEAR"
            || !stem.chars().all(|c| c.is_alphanumeric() || c == '_')
        {
            return None;
        }
        out.push(Term::Word { stem, prefix });
    }
    Some(out)
}

/// Keeps a trailing `*` on the one word it is written on.
///
/// The grammar folds consecutive bare words into a single phrase, and a
/// trailing `*` applies to the whole of it. A prefix literal also bypasses the
/// analyzer — it is neither split into tokens nor lowercased — so
/// `workshop analyst*` looks for an index key starting with `workshop analyst`,
/// and `Workshop*` for a capitalised token the `Lowercase` filter never stored.
/// Both return nothing, without an error.
///
/// A prefix word is separated from a bare word before it by `AND`, which is
/// what the bare words mean already (a phrase group without `*` is tokenized
/// into an AND of its words), and its stem is lowercased.
fn split_prefix_terms(query: &str) -> Cow<'_, str> {
    let Some(terms) = terms(query) else {
        return Cow::Borrowed(query);
    };
    let mut out = Vec::with_capacity(terms.len());
    let mut changed = false;
    let mut after_bare_word = false;
    for term in &terms {
        match *term {
            Term::Word { stem, prefix: true } => {
                if after_bare_word {
                    out.push("AND".to_string());
                    changed = true;
                }
                let lower = stem.to_lowercase();
                changed |= lower != stem;
                out.push(format!("{lower}*"));
                after_bare_word = false;
            }
            Term::Word {
                stem,
                prefix: false,
            } => {
                out.push(stem.to_string());
                after_bare_word = true;
            }
            Term::Operator(s) | Term::Phrase(s) => {
                out.push(s.to_string());
                after_bare_word = false;
            }
        }
    }
    if changed {
        Cow::Owned(out.join(" "))
    } else {
        Cow::Borrowed(query)
    }
}

/// The prefix-match widening of a query that found nothing: every bare word
/// gets a trailing `*`. `None` if there is nothing to widen — each word already
/// ends in `*`, or the query uses syntax [`terms`] does not rewrite.
pub fn prefix_widening(query: &str) -> Option<String> {
    let terms = terms(query)?;
    if !terms
        .iter()
        .any(|t| matches!(t, Term::Word { prefix: false, .. }))
    {
        return None;
    }
    let widened: Vec<String> = terms
        .iter()
        .map(|t| match *t {
            Term::Word { stem, .. } => format!("{stem}*"),
            Term::Operator(s) | Term::Phrase(s) => s.to_string(),
        })
        .collect();
    Some(widened.join(" "))
}

/// One FTS attempt with `query` exactly as given.
fn run_fts(store: &Store, query: &str, limit: usize) -> Result<Vec<NodeBrief>> {
    let limit = limit.min(FUZZY_RECALL_LIMIT_CAP);
    if limit == 0 {
        return Ok(Vec::new());
    }
    let excerpt_chars = store.config().body_excerpt_chars;

    let mut params: BTreeMap<String, DataValue> = BTreeMap::new();
    params.insert("q".to_string(), DataValue::Str(query.into()));

    // The FTS atom requires a literal `k` and `query` parameters; we
    // inline `k` into the script and pass `q` as a Datalog parameter so
    // user input never reaches the script source.
    //
    // Initiative-scoped: an extra `*node_initiative` join trims hits.
    // Cross-initiative: just a NOW anchor on the base relation.
    let script = match store.current_initiative() {
        Some(init) => {
            params.insert("init".to_string(), DataValue::Str(init.into()));
            format!(
                r#"
                hits[id, score] := ~node:fts_name{{id | query: $q, k: {limit}, bind_score: score}}
                hits[id, score] := ~node:fts_body{{id | query: $q, k: {limit}, bind_score: score}}

                ?[id, type, name, body, score, validity] :=
                    hits[id, score],
                    *node{{id, type, name, body, validity @ 'NOW'}},
                    *node_initiative{{initiative, node_id: id}},
                    initiative = $init
                :order -score, validity
                "#
            )
        }
        None => format!(
            r#"
            hits[id, score] := ~node:fts_name{{id | query: $q, k: {limit}, bind_score: score}}
            hits[id, score] := ~node:fts_body{{id | query: $q, k: {limit}, bind_score: score}}

            ?[id, type, name, body, score, validity] :=
                hits[id, score],
                *node{{id, type, name, body, validity @ 'NOW'}}
            :order -score, validity
            "#
        ),
    };

    let rows = store
        .db_ref()
        .run_script(&script, params, ScriptMutability::Immutable)?;

    // Multiple FTS hits per id (one from name, one from body) come back
    // as separate rows; keep the highest-score row per id, preserving
    // overall descending-score ordering.
    let mut best: HashMap<String, (f64, NodeBrief)> = HashMap::new();
    let mut order: Vec<String> = Vec::new();
    for row in &rows.rows {
        let Some(id) = row.first().and_then(|v| v.get_str()).map(String::from) else {
            continue;
        };
        let score = row.get(4).and_then(|v| v.get_float()).unwrap_or(0.0);
        let brief = parse_brief(row.as_slice(), excerpt_chars);
        match best.get(&id) {
            Some((prev, _)) if *prev >= score => continue,
            None => order.push(id.clone()),
            _ => {}
        }
        best.insert(id, (score, brief));
    }

    let mut out: Vec<(f64, NodeBrief)> = order
        .into_iter()
        .filter_map(|id| best.remove(&id))
        .collect();
    // Re-sort because dedup might have stuck a higher-score body hit
    // behind a lower-score name hit in `order`.
    out.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
    Ok(out.into_iter().take(limit).map(|(_, b)| b).collect())
}

#[cfg(test)]
mod tests {
    use super::{fuzzy_recall, prefix_widening, quote_unparseable_tokens, split_prefix_terms};
    use crate::store::Store;
    use crate::{EpisodeKind, Significance, write_episode};

    fn two_workshop_notes() -> Store {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        for (name, body) in [
            ("workshop-plan", "Провести воркшоп с аналитиками 30.09"),
            (
                "workshop-agreed",
                "Договорились с аналитиками провести воркшоп 30.09",
            ),
        ] {
            write_episode(
                &store,
                EpisodeKind::Observation,
                Significance::Low,
                name,
                body,
            )
            .expect("write");
        }
        store
    }

    /// A prefix word after a bare word matched nothing: the grammar read
    /// `воркшоп аналитик*` as one prefix literal, `воркшоп аналитик`.
    #[test]
    fn a_prefix_word_after_a_bare_word_matches() {
        let store = two_workshop_notes();
        for q in [
            "воркшоп аналитик*",
            "провести воркшоп*",
            "воркшоп аналитиками",
        ] {
            let hits = fuzzy_recall(&store, q, 5).unwrap_or_else(|e| panic!("`{q}`: {e}"));
            assert_eq!(hits.len(), 2, "`{q}` finds both notes");
        }
    }

    /// A prefix literal skips the `Lowercase` filter, so a capital letter in
    /// it could never match the index.
    #[test]
    fn a_capitalised_prefix_matches() {
        let store = two_workshop_notes();
        let hits = fuzzy_recall(&store, "Воркшоп*", 5).expect("parses");
        assert_eq!(hits.len(), 2);
    }

    #[test]
    fn only_a_prefix_after_a_bare_word_is_split() {
        assert_eq!(split_prefix_terms("a b*"), "a AND b*");
        assert_eq!(split_prefix_terms("a b c*"), "a b AND c*");
        assert_eq!(split_prefix_terms("Ab*"), "ab*");
        for q in [
            "a b",
            "a* b",
            "a* b*",
            "a OR b*",
            "\"a b\" c*",
            "(a b*)",
            "a b*^2",
        ] {
            assert_eq!(split_prefix_terms(q), q, "`{q}` is left as is");
        }
    }

    #[test]
    fn widening_stars_each_bare_word() {
        assert_eq!(
            prefix_widening("воркшоп аналитик*").as_deref(),
            Some("воркшоп* аналитик*")
        );
        assert_eq!(prefix_widening("a OR b").as_deref(), Some("a* OR b*"));
        assert_eq!(prefix_widening("\"a b\" c").as_deref(), Some("\"a b\" c*"));
        assert_eq!(prefix_widening("a* b*"), None);
        assert_eq!(prefix_widening("pilot-finalize"), None);
    }

    fn seeded() -> Store {
        let store = Store::open_in_memory().expect("open");
        store.use_initiative("t");
        write_episode(
            &store,
            EpisodeKind::Observation,
            Significance::Low,
            "pilot-finalize-6of6-2026-06-02",
            "the pilot run at 185.130.225.215 for a master's degree",
        )
        .expect("write");
        store
    }

    /// The headline case from #49: kaeru names its own nodes with hyphenated
    /// slugs, and the FTS grammar rejects a hyphen inside a token — so looking
    /// a node up by the name kaeru itself gave it was guaranteed to fail.
    #[test]
    fn a_node_is_findable_by_its_own_slug_name() {
        let store = seeded();
        let hits = fuzzy_recall(&store, "pilot-finalize", 5).expect("no parse error");
        assert_eq!(hits.len(), 1, "the hyphenated slug finds its node");
    }

    /// The rest of the punctuation family: dots in an IP, an apostrophe.
    #[test]
    fn punctuation_no_longer_breaks_the_parser() {
        let store = seeded();
        for q in ["185.130.225.215", "master's", "pilot-finalize-6of6"] {
            let hits = fuzzy_recall(&store, q, 5)
                .unwrap_or_else(|e| panic!("`{q}` should parse, got: {e}"));
            assert!(!hits.is_empty(), "`{q}` finds the node");
        }
    }

    /// The rewrite is a fallback, not a filter: queries the grammar already
    /// accepts must keep their exact meaning — especially the documented `*`
    /// inflection idiom and the boolean operators.
    #[test]
    fn working_queries_are_left_untouched() {
        assert_eq!(quote_unparseable_tokens("pilot*"), "pilot*");
        assert_eq!(
            quote_unparseable_tokens("pilot OR finalize"),
            "pilot OR finalize"
        );
        assert_eq!(
            quote_unparseable_tokens("\"already a phrase\""),
            "\"already a phrase\""
        );
        // …and a punctuated token becomes a phrase over the same tokens.
        assert_eq!(
            quote_unparseable_tokens("pilot-finalize"),
            "\"pilot-finalize\""
        );
        // A trailing `*` on a punctuated token is dropped — prefix and phrase
        // can't combine, and matching beats failing to parse.
        assert_eq!(
            quote_unparseable_tokens("v2-humanizer*"),
            "\"v2-humanizer\""
        );
    }

    /// A bare `*` gets an actionable message instead of a parser dump.
    #[test]
    fn a_bare_star_explains_itself() {
        let store = seeded();
        let err = fuzzy_recall(&store, "*", 5).expect_err("bare star is not a query");
        let msg = err.to_string();
        assert!(
            msg.contains("matches nothing"),
            "explains the problem: {msg}"
        );
        assert!(msg.contains("token*"), "names the working idiom: {msg}");
    }
}

#[cfg(test)]
mod field_syntax_tests {
    use super::fuzzy_recall;
    use crate::store::Store;

    /// Tag syntax typed into `search` used to be a parse error, then (after the
    /// phrase fallback) a silent zero-result — both leave the agent guessing.
    /// The fields are real, they just live behind `tagged` (#49).
    #[test]
    fn tag_syntax_points_at_tagged() {
        let store = Store::open_in_memory().expect("open");
        for q in ["topic:figma", "kind:task", "status:open"] {
            let err = fuzzy_recall(&store, q, 5).expect_err("tag syntax is not an FTS query");
            let msg = err.to_string();
            assert!(msg.contains("tagged"), "`{q}` names the right verb: {msg}");
        }
    }

    /// A colon inside ordinary text is not field syntax and must still search.
    #[test]
    fn an_ordinary_colon_is_not_field_syntax() {
        let store = Store::open_in_memory().expect("open");
        assert!(
            fuzzy_recall(&store, "note: shipped", 5).is_ok(),
            "prose with a colon still searches"
        );
    }
}
