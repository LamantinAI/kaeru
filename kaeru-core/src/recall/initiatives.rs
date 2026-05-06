//! Initiative discovery — `list_initiatives` returns the distinct set
//! of initiative names the substrate has seen at least one node attached
//! to. Mutations populate `node_initiative` automatically when the
//! `Store` has a `current_initiative` set.

use crate::errors::Result;
use crate::store::Store;

/// Returns every initiative name that has at least one node attached
/// through the `node_initiative` junction. Sorted alphabetically.
///
/// Datalog rule-head deduplication produces distinct names; ordering is
/// applied at projection time so CLI output is stable.
pub fn list_initiatives(store: &Store) -> Result<Vec<String>> {
    let script = r#"
        ?[initiative] := *node_initiative{initiative, node_id}
        :order initiative
    "#;
    let rows = store.run_read(script)?;
    let names: Vec<String> = rows
        .rows
        .iter()
        .filter_map(|row| row.first().and_then(|v| v.get_str()).map(String::from))
        .collect();
    Ok(names)
}
